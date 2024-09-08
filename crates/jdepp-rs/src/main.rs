use libc::c_char;
use libc::c_uint;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

#[repr(C)]
struct Jdepp;

#[repr(C)]
struct Sentence;

extern "C" {
    fn jdepp_create(model_path: *const c_char) -> *mut Jdepp;
    fn jdepp_destroy(instance: *mut Jdepp);
    // fn jdepp_load_model(instance: *mut Jdepp, model_path: *const c_char) -> c_bool;
    fn jdepp_model_loaded(instance: *const Jdepp) -> bool;

    fn parse_from_postagged(
        instance: *mut Jdepp,
        input_postagged: *const c_char,
        len: c_uint,
    ) -> *mut Sentence;
    fn sentence_str(instance: *const Sentence) -> *const c_char;

}

fn main() {
    // モデルのパスをCStringで作成
    let model_path = CString::new("./knbc").unwrap();

    unsafe {
        // Jdeppインスタンスの作成
        let jdepp = jdepp_create(model_path.as_ptr());
        //  if jdepp.is_null() {
        //      eprintln!("Failed to create Jdepp instance.");
        //      return;
        //  }
        //
        //  // モデルのロード
        //  if jdepp_load_model(jdepp, model_path.as_ptr()) {
        //      println!("Model loaded successfully.");
        //  } else {
        //      println!("Failed to load model.");
        //  }

        // モデルがロードされたか確認
        if jdepp_model_loaded(jdepp) {
            println!("Model is loaded.");
        } else {
            println!("Model is not loaded.");
        }

        let s = r#"頭	名詞,普通名詞,*,*,頭,あたま,*
EOS
"#;

        let len = s.len();
        let c_string = CString::new(s).unwrap();
        let p = c_string.as_ptr();
        let s = parse_from_postagged(jdepp, p, len as u32);
        let c_str = sentence_str(s);
        let result = CStr::from_ptr(c_str).to_string_lossy().into_owned();
        dbg!(result);

        // let c_str = sentence_str(s);
        // let result = CStr::from_ptr(c_str).to_string_lossy().into_owned();
        // println!("Result from C++: {}", result);

        // let c_str = sentence_str(s);
        // let str = CStr::from_ptr(c_str).to_str().unwrap();
        // println!("Result from C++: {}", str);
        // dbg!(sentence_str(s));
        // Jdeppインスタンスの破棄
        // jdepp_destroy(jdepp);
    }
}
