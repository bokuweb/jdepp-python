# jdepp-rs

`jdepp-rs` は、このリポジトリに同梱されている J.DepP(C++) を **Rust から呼ぶための bindings** です。

## できること

- POS 付与済み（MeCab 互換）入力 (`EOS\n` 終端) を渡して、J.DepP の解析結果文字列を取得

## ビルド

この crate は **`cmake` コマンドに依存しません**。`cargo build` だけで、`jdepp/*.cc` と `crates/jdepp-rs/src/bindings.cpp` を `cc` crate でコンパイルして静的リンクします。

```bash
cd /path/to/jdepp-python
cargo build -p jdepp-rs
```

## `cargo add` で依存に追加（ライブラリとして利用）

### ローカル（path 依存）

別プロジェクトから、ワークスペース内の crate を直接参照する場合:

```bash
cargo add jdepp-rs --path /path/to/jdepp-python/crates/jdepp-rs
```

コード側では `jdepp_rs` として import します（`-` は `_` に置き換わります）:

```rust
use jdepp_rs::Jdepp;
```

### Git 依存（このリポジトリをそのまま参照）

```bash
cargo add jdepp-rs --git https://github.com/lighttransport/jdepp-python
```

## 使い方（例）

モデル（辞書）ディレクトリを用意して、example を実行します。

```bash
cargo run -p jdepp-rs --example parse_from_postagged -- /path/to/model/knbc
```

`/path/to/model/knbc` は `dic.utf8` などが入っているディレクトリを指定してください。

## API

- `jdepp_rs::Jdepp::new(model_path)`
- `jdepp_rs::Jdepp::model_loaded() -> bool`
- `jdepp_rs::Jdepp::parse_from_postagged(input: &str) -> Result<String>`


