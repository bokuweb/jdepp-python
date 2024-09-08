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
}

impl Drop for Jdepp {
    fn drop(&mut self) {
        unsafe { jdepp_destroy(self.raw.as_ptr()) };
    }
}


