use libc::{c_char, size_t};
use std::ffi::{CStr, CString, NulError};
use std::path::Path;
use std::ptr::NonNull;
use std::{fmt, result};

#[repr(C)]
struct JdeppOpaque {
    _private: [u8; 0],
}

#[repr(C)]
struct SentenceOpaque {
    _private: [u8; 0],
}

extern "C" {
    fn jdepp_create(model_path: *const c_char) -> *mut JdeppOpaque;
    fn jdepp_destroy(instance: *mut JdeppOpaque);
    fn jdepp_model_loaded(instance: *const JdeppOpaque) -> bool;

    fn jdepp_parse_from_postagged(
        instance: *const JdeppOpaque,
        input_postagged: *const c_char,
        len: size_t,
    ) -> *mut SentenceOpaque;

    fn sentence_str(instance: *const SentenceOpaque) -> *const c_char;
    fn sentence_destroy(instance: *mut SentenceOpaque);
}

#[derive(Debug)]
pub enum Error {
    NullInstance,
    Nul(NulError),
    SentenceNull,
    SentenceStrNull,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NullInstance => write!(f, "failed to create Jdepp instance (null)"),
            Error::Nul(e) => write!(f, "nul byte in string: {e}"),
            Error::SentenceNull => write!(f, "jdepp returned null Sentence"),
            Error::SentenceStrNull => write!(f, "sentence_str returned null"),
        }
    }
}

impl std::error::Error for Error {}

impl From<NulError> for Error {
    fn from(value: NulError) -> Self {
        Error::Nul(value)
    }
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub surface: String,
    pub feature: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub id: usize,
    /// 係り先 chunk id。root の場合は `-1`
    pub head: isize,
    /// 係り種別（例: 'D'）。不明な場合は `None`
    pub dep_type: Option<char>,
    pub tokens: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedSentence {
    pub chunks: Vec<Chunk>,
}

#[derive(Debug)]
pub enum ParseError {
    Jdepp(Error),
    NoChunks,
    UnexpectedTokenLine { line: String },
    InvalidChunkHeader { line: String },
    InvalidHead { line: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Jdepp(e) => write!(f, "jdepp error: {e}"),
            ParseError::NoChunks => write!(f, "no chunks found"),
            ParseError::UnexpectedTokenLine { line } => write!(f, "token line before any chunk: {line}"),
            ParseError::InvalidChunkHeader { line } => write!(f, "invalid chunk header: {line}"),
            ParseError::InvalidHead { line } => write!(f, "invalid head in chunk header: {line}"),
        }
    }
}

impl std::error::Error for ParseError {}

pub struct Jdepp {
    raw: NonNull<JdeppOpaque>,
}

impl Jdepp {
    pub fn new(model_path: impl AsRef<Path>) -> Result<Self> {
        let model_path = model_path.as_ref().to_string_lossy();
        let c_model_path = CString::new(model_path.as_bytes())?;
        let raw = unsafe { jdepp_create(c_model_path.as_ptr()) };
        let raw = NonNull::new(raw).ok_or(Error::NullInstance)?;
        Ok(Self { raw })
    }

    pub fn model_loaded(&self) -> bool {
        unsafe { jdepp_model_loaded(self.raw.as_ptr()) }
    }

    /// POS-tagged 入力を渡して、J.DepP の出力文字列（`EOS\n` 含む）を返します。
    pub fn parse_from_postagged(&self, input_postagged: &str) -> Result<String> {
        let c_input = CString::new(input_postagged)?;
        let sent = unsafe {
            jdepp_parse_from_postagged(
                self.raw.as_ptr(),
                c_input.as_ptr(),
                input_postagged.len() as size_t,
            )
        };
        let sent = NonNull::new(sent).ok_or(Error::SentenceNull)?;

        // `Sentence` が生きている間だけ有効なので即コピーしてから破棄する
        let c_str = unsafe { sentence_str(sent.as_ptr()) };
        if c_str.is_null() {
            unsafe { sentence_destroy(sent.as_ptr()) };
            return Err(Error::SentenceStrNull);
        }
        let out = unsafe { CStr::from_ptr(c_str) }.to_string_lossy().into_owned();
        unsafe { sentence_destroy(sent.as_ptr()) };
        Ok(out)
    }

    /// `parse_from_postagged` の出力（CaboCha 互換: `* <id> <head>D` + 形態素行 + `EOS`）を
    /// Rust の構造体に parse して返します。
    pub fn parse_from_postagged_parsed(&self, input_postagged: &str) -> result::Result<ParsedSentence, ParseError> {
        let out = self.parse_from_postagged(input_postagged).map_err(ParseError::Jdepp)?;
        parse_cabocha_like(&out)
    }
}

impl Drop for Jdepp {
    fn drop(&mut self) {
        unsafe { jdepp_destroy(self.raw.as_ptr()) };
    }
}

/// J.DepP の CaboCha 互換出力を parse します。
///
/// 例:
/// - `* 1 4D` : chunk 1 は chunk 4 に係る（D=dependency）
/// - `* 4 -1D`: chunk 4 は root（係り先なし）
pub fn parse_cabocha_like(output: &str) -> result::Result<ParsedSentence, ParseError> {
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut current: Option<Chunk> = None;

    for raw_line in output.lines() {
        let line = raw_line.trim_end();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }
        if line == "EOS" {
            break;
        }

        if let Some(rest) = line.strip_prefix("*") {
            if let Some(c) = current.take() {
                chunks.push(c);
            }
            let fields: Vec<&str> = rest.split_whitespace().collect();
            if fields.len() < 2 {
                return Err(ParseError::InvalidChunkHeader {
                    line: line.to_string(),
                });
            }

            // 互換性重視: 末尾フィールドを "auto" として扱う（to_tree.py 互換）
            // "* 1 4D" -> auto="4D"
            // "* 1 4D@0.98" -> auto="4D@0.98"
            let id = fields[0]
                .parse::<usize>()
                .map_err(|_| ParseError::InvalidChunkHeader {
                    line: line.to_string(),
                })?;
            let mut auto = *fields.last().unwrap();

            // auto の "@prob" は無視（必要なら後で拡張）
            if let Some((left, _prob)) = auto.split_once('@') {
                auto = left;
            }

            if auto.len() < 2 {
                return Err(ParseError::InvalidHead {
                    line: line.to_string(),
                });
            }
            let dep_type = auto.chars().last();
            let head_str = &auto[..auto.len() - dep_type.map(|c| c.len_utf8()).unwrap_or(0)];
            let head = head_str
                .parse::<isize>()
                .map_err(|_| ParseError::InvalidHead {
                    line: line.to_string(),
                })?;

            current = Some(Chunk {
                id,
                head,
                dep_type,
                tokens: Vec::new(),
            });
            continue;
        }

        // token line
        let Some(ref mut c) = current else {
            return Err(ParseError::UnexpectedTokenLine {
                line: line.to_string(),
            });
        };

        let (surface, feature) = match line.find(|ch: char| ch == '\t' || ch.is_whitespace()) {
            Some(pos) => {
                let s = line[..pos].to_string();
                let f = line[pos..].trim().to_string();
                (s, f)
            }
            None => (line.to_string(), String::new()),
        };
        c.tokens.push(Token { surface, feature });
    }

    if let Some(c) = current.take() {
        chunks.push(c);
    }

    if chunks.is_empty() {
        return Err(ParseError::NoChunks);
    }

    Ok(ParsedSentence { chunks })
}


