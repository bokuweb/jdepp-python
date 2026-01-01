use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let model_dir = match args.next() {
        Some(v) => v,
        None => {
            eprintln!("usage: cargo run -p jdepp-rs --example parse_structured -- /path/to/model/knbc");
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

    if !jdepp.model_loaded() {
        eprintln!("model is not loaded. please pass a valid model dir (e.g. contains dic.utf8): {model_dir}");
        return ExitCode::from(1);
    }

    // NOTE: MeCab 互換: surface + TAB + feature + '\n'、最後に "EOS\n"
    let input = "吾輩\t名詞,普通名詞,*,*,吾輩,わがはい,代表表記:我が輩/わがはい カテゴリ:人\n\
は\t助詞,副助詞,*,*,は,は,*\n\
猫\t名詞,普通名詞,*,*,猫,ねこ,*\n\
である\t判定詞,*,判定詞,デアル列基本形,だ,である,*\n\
。\t特殊,句点,*,*,。,。,*\n\
名前\t名詞,普通名詞,*,*,名前,なまえ,*\n\
は\t助詞,副助詞,*,*,は,は,*\n\
まだ\t副詞,*,*,*,まだ,まだ,*\n\
ない\t形容詞,*,イ形容詞アウオ段,基本形,ない,ない,*\n\
。\t特殊,句点,*,*,。,。,*\n\
EOS\n";

    let sent = match jdepp.parse_from_postagged_parsed(input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("parse failed: {e}");
            return ExitCode::from(1);
        }
    };

    for c in &sent.chunks {
        let surfaces: String = c.tokens.iter().map(|t| t.surface.as_str()).collect();
        println!("chunk {} -> head {} ({:?}) : {}", c.id, c.head, c.dep_type, surfaces);
    }

    ExitCode::SUCCESS
}


