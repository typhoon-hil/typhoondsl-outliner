#[allow(clippy::unit_arg)]
mod outliner;
#[allow(clippy::too_many_arguments)]
mod outliner_actions;
mod outliner_lexer;
#[cfg(test)]
mod tests;

use clap::Parser;
use outliner_lexer::OutlinerLexer;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Should JSON be generated
    #[arg(short, long)]
    json: bool,

    /// Should JSON be pretty printed
    #[arg(short, long)]
    pretty: bool,

    #[clap(value_parser, value_name="MODEL", value_hint = clap::ValueHint::FilePath)]
    model: Option<PathBuf>,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let model_str = match args.model {
        Some(model) => std::fs::read_to_string(model)?,
        None => {
            let mut piped = String::new();
            io::stdin().read_to_string(&mut piped)?;
            piped
        }
    };

    let result = outliner::OutlinerParser::new(OutlinerLexer::new()).parse(&model_str);

    match result {
        Ok(model) => {
            if args.json {
                if args.pretty {
                    println!("{}", serde_json::to_string_pretty(&model).unwrap());
                } else {
                    println!("{}", serde_json::to_string(&model).unwrap());
                }
            } else {
                println!("Success!");
            }
            std::process::exit(0);
        }
        Err(e) => {
            if args.json {
                println!(r#"{{ "error": true }}"#);
            } else {
                println!("Failure!");
                println!("{e:#?}");
            }
            std::process::exit(1);
        }
    }
}
