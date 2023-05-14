mod outliner;
mod outliner_actions;
mod outliner_lexer;
#[cfg(test)]
mod tests;

use std::{env, path::Path};

use outliner_lexer::OutlinerLexer;

fn main() -> ! {
    match &env::args().collect::<Vec<_>>()[..] {
        [_, model] => {
            println!("Parsing: {model}");
            match outliner::OutlinerParser::new(OutlinerLexer::new()).parse_file(model) {
                Ok(_) => {
                    println!("Success!");
                    std::process::exit(0);
                }
                Err(e) => {
                    println!("Failure!");
                    println!("{e:#?}");
                    std::process::exit(1);
                },
            }
        }
        [exec, ..] => {
            println!(
                "Usage: {} <model>",
                Path::new(exec)
                    .file_name()
                    .expect("Unable to get file name.")
                    .to_string_lossy()
            );
            std::process::exit(1);
        }
        _ => panic!("This should not happen!"),
    }
}
