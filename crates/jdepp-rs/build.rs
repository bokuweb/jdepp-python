fn main() {
    let dst = cmake::build("../../");
    println!("cargo:rustc-link-search=native={}", dst.display());
    // println!("cargo:rustc-link-lib=static=cmake");
    // staticライブラリとして他に利用するライブラリはなし
    println!("cargo:rustc-link-lib=static=jdepp");
    // staticライブラリとして他に利用するライブラリはなし

    // C++ソースコードの場合は必ずこれを追加すること
    // C++ソースコードの場合は必ずこれを追加すること
    // println!("cargo:rustc-link-lib=dylib=stdc++");
    //println!("cargo:rustc-link-search=native={}", dst.display());
    //
    //// staticライブラリとして他に利用するライブラリはなし
    //println!("cargo:rustc-link-lib=static=");
    //
    //// C++ソースコードの場合は必ずこれを追加すること
    println!("cargo:rustc-link-lib=c++");
    //println!("cargo:rustc-link-lib=dylib=stdc++");
    //
    //// CMakeLists.txt内の記述とは別に、その他のライブラリは必要なものを全て記述する必要あり
    //println!("cargo:rustc-link-lib=dylib=EGL");
    //println!("cargo:rustc-link-lib=dylib=GLESv2");
    //println!("cargo:rustc-link-lib=dylib=X11");
    //
    // pkg-configでヒットしないライブラリは以下のように直接パス指定が可能
    // let soil_lib_dir = "/usr/lib";
    // println!("cargo:rustc-link-search={}", soil_lib_dir);
    // println!("cargo:rustc-link-lib=dylib=SOIL");

    // let bindings = bindgen::Builder::default()
    //     .header("../../jdepp/pdep.h")
    //     .clang_arg("-I/usr/lib/clang/<version>/include")
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     .generate()
    //     .expect("Unable to generate bindings");
    //
    // let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");

    // cxx_build::bridge("src/main.rs")
    //     .file("src/bindings.cpp") // Replace with the actual path to your C++ source file
    //     .flag_if_supported("-std=c++17") // Ensure C++17 or above is supported
    //     .compile("jdepp");
    println!("cargo:rerun-if-changed=src/bindings.cpp");
    println!("cargo:rerun-if-changed=jdepp/pdepp.cc");
}
