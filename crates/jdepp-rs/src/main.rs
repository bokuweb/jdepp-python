use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

#[repr(C)]
struct Jdepp;

extern "C" {
    #[no_mangle]
    fn jdepp_create(model_path: *const c_char) -> *mut Jdepp;
    #[no_mangle]
    fn jdepp_destroy(instance: *mut Jdepp);
    // fn jdepp_load_model(instance: *mut Jdepp, model_path: *const c_char) -> c_bool;
    #[no_mangle]
    fn jdepp_model_loaded(instance: *const Jdepp) -> bool;
}

fn main() {
    // モデルのパスをCStringで作成
    let model_path = CString::new("path/to/model").unwrap();

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

        // Jdeppインスタンスの破棄
        // jdepp_destroy(jdepp);
    }
}
