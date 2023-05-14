use crate::outliner::OutlinerParser;
use crate::outliner_lexer::OutlinerLexer;
use rustemo_compiler::output_cmp;

#[test]
fn test_model_parse() {
    let result = OutlinerParser::new(OutlinerLexer::new())
        .parse_file("models/battery generic server and client.tse");
    output_cmp!(
        "src/model.ast",
        format!("{:#?}", result)
    );
}
