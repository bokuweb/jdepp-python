use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let model_dir = match args.next() {
        Some(v) => v,
        None => {
            eprintln!("usage: cargo run -p jdepp-rs --example parse_from_postagged -- /path/to/model/knbc");
            return ExitCode::from(2);
        }
    };

    let jdepp = match jdepp_rs::Jdepp::new(&model_dir) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to create Jdepp: {e}");
            return ExitCode::from(1);
        }
    };

    eprintln!("model_loaded: {}", jdepp.model_loaded());

    // NOTE: MeCab 互換: surface + TAB + feature(comma separated 7 fields) + '\n'、最後に "EOS\n"
    let input = "吾輩\t名詞,普通名詞,*,*,吾輩,わがはい,代表表記:我が輩/わがはい カテゴリ:人\n\
は\t助詞,副助詞,*,*,は,は,*\n\
猫\t名詞,普通名詞,*,*,猫,ねこ,*\n\
である\t判定詞,*,判定詞,デアル列基本形,だ,である,*\n\
。\t特殊,句点,*,*,。,。,*\n\
EOS\n";

    match jdepp.parse_from_postagged(input) {
        Ok(out) => {
            print!("{out}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("parse_from_postagged failed: {e}");
            ExitCode::from(1)
        }
    }
}


