use std::path::PathBuf;

use lindera::tokenizer::{Tokenizer, TokenizerConfig};
use lindera_core::viterbi::Mode;
use lindera_core::LinderaResult;

fn main() -> LinderaResult<()> {
    // create tokenizer
    let config = TokenizerConfig {
        user_dict_path: Some(PathBuf::from("./resources/userdic.csv")),
        mode: Mode::Normal,
        ..TokenizerConfig::default()
    };
    let mut tokenizer = Tokenizer::with_config(config)?;

    // tokenize the text
    let tokens = tokenizer.tokenize("東京スカイツリーの最寄り駅はとうきょうスカイツリー駅です")?;

    // output the tokens
    for token in tokens {
        println!("{}", token.text);
    }

    Ok(())
}
