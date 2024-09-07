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
}
