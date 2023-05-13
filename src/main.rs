mod outliner;
mod outliner_actions;
mod outliner_lexer;
#[cfg(test)]
mod tests;

use std::{env, path::Path};

use outliner_lexer::OutlinerLexer;

fn main() {
    match &env::args().collect::<Vec<_>>()[..] {
        [_, model] => {
            let result = outliner::OutlinerParser::new(OutlinerLexer::new()).parse_file(model);
            println!("{result:#?}")
        }
        [exec, ..] => {
            println!(
                "Usage: {} <model>",
                Path::new(exec)
                    .file_name()
                    .expect("Unable to get file name.")
                    .to_string_lossy()
            );
        }
        _ => panic!("This should not happen!"),
    }
}
