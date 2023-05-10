#[cfg(test)]
mod tests;
mod outliner;
mod outliner_actions;
mod outliner_lexer;

use outliner_lexer::OutlinerLexer;

fn main() {

    let result = outliner::OutlinerParser::new(OutlinerLexer::new()).parse_file("pmsm iron losses.tse");
    dbg!(result);

}
