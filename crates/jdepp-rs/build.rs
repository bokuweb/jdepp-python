fn main() {
    // `cmake` コマンドに依存せず、`cc` crate で直接 jdepp の C++ をビルドする。
    // (CI/ローカル環境で cmake 未インストールでも `cargo build` だけで完結させるため)

    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.join("../..");
    let jdepp_dir = repo_root.join("jdepp");

    let sources = [
        // C++ bridge (C ABI)
        repo_root.join("crates/jdepp-rs/src/bindings.cpp"),
        // jdepp core
        jdepp_dir.join("classify.cc"),
        jdepp_dir.join("kernel.cc"),
        jdepp_dir.join("linear.cc"),
        jdepp_dir.join("pdep.cc"),
        jdepp_dir.join("io-util.cc"),
    ];

    for s in &sources {
        println!("cargo:rerun-if-changed={}", s.display());
    }
    println!("cargo:rerun-if-changed={}", jdepp_dir.join("pdep.h").display());
    println!(
        "cargo:rerun-if-changed={}",
        jdepp_dir.join("io-util.hh").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        jdepp_dir.join("optparse.h").display()
    );

    let mut build = cc::Build::new();
    build.cpp(true);
    build.flag_if_supported("-std=c++11");
    build.include(&repo_root);
    build.include(&jdepp_dir);

    for s in &sources {
        build.file(s);
    }

    // `libjdepp.a` を生成（crate 側から自動リンクされる）
    build.compile("jdepp");

    // macOS の clang++ では通常自動だが、明示しておく
    println!("cargo:rustc-link-lib=c++");
}
