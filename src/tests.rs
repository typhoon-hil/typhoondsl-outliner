use crate::outliner::OutlinerParser;
use crate::outliner_lexer::OutlinerLexer;
use rustemo_compiler::output_cmp;

#[test]
fn test_model_parse() {
    let result = OutlinerParser::new(OutlinerLexer::new())
        .parse_file("models/battery generic server and client.tse")
        .unwrap();

    output_cmp!("src/model.ast", format!("{:#?}", result));
}

#[test]
fn test_model_serialize_json() {
    let result = OutlinerParser::new(OutlinerLexer::new())
        .parse_file("models/battery generic server and client.tse")
        .unwrap();

    let result_json = serde_json::to_string_pretty(&result).unwrap();

    output_cmp!("src/model.json", format!("{}", result_json));
}
