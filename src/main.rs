mod outliner;
mod outliner_actions;
mod outliner_lexer;
#[cfg(test)]
mod tests;

use clap::Parser;
use outliner_lexer::OutlinerLexer;
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
    model: PathBuf,
}

fn main() {
    let args = Args::parse();

    match outliner::OutlinerParser::new(OutlinerLexer::new()).parse_file(args.model) {
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
