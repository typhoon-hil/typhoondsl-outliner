use crate::outliner::TokenRecognizer;

use super::outliner::TokenKind;
#[cfg(debug_assertions)]
use colored::*;
use lazy_static::lazy_static;
use regex::Regex;
use rustemo::{
    lexer::{self, Context, Lexer, Token},
    location::{Location, Position},
    log,
};

pub type Input = str;

lazy_static! {
    static ref RE_ID: Regex = Regex::new(r"^[^\d\W]\w*\b").unwrap();
    static ref RE_NAME: Regex =
        Regex::new(r#"^("(\\"|[^"])*")|^('(\\'|[^'])*')|^(\w|\+|-)+"#).unwrap();
    static ref RE_STRING: Regex = Regex::new(r#"^"(\\"|[^"])*""#).unwrap();
    static ref RE_MODEL_PROPERTY: Regex = Regex::new(r#"^model\s+="#).unwrap();
    static ref RE_CONFIGURATION_PROPERTY: Regex = Regex::new(r#"^configuration\s+="#).unwrap();
}

pub struct OutlinerLexer();

impl OutlinerLexer {
    pub fn new() -> Self {
        Self()
    }

    fn skip(&self, context: &mut Context<Input>) {
        let skipped_len: usize = context.input[context.position..]
            .chars()
            .take_while(|x| x.is_whitespace())
            .map(|c| c.len_utf8())
            .sum();
        let skipped = &context.input[context.position..context.position + skipped_len];
        log!("{}: {}", "Skipped ws".green(), skipped_len);
        if skipped_len > 0 {
            context.layout_ahead = Some(skipped);
            context.position += skipped_len;
        } else {
            context.layout_ahead = None;
        }
        context.location = <str as lexer::Input>::location_after(skipped, context.location);
    }
}

impl Lexer<Input, TokenRecognizer> for OutlinerLexer {
    fn next_token<'i>(
        &self,
        context: &mut Context<'i, Input>,
        token_recognizers: &[&TokenRecognizer],
    ) -> Option<Token<'i, Input, TokenKind>> {
        // 1. Skip WS
        self.skip(context);
        // 2. For each token expect check if it is at the current position
        //    Remember to take into account flaky matching, i.e. tokens
        //    which should match only if no other expected tokens are here.
        //    Flaky tokens should be of low priority.
        let is_word_char = |c: char| -> bool { c.is_alphabetic() || c == '-' || c == '_' };
        let is_id = |s: &str| -> bool { s.chars().all(is_word_char) };

        // Match on a word boundaries for ID-like strings
        let is_match = |s: &str, position: usize| -> bool {
            match context.input.get(position..position + s.len()) {
                Some(subs) => {
                    subs == s
                        && (!is_id(s)
                            || (!is_word_char(
                                context.input[..position]
                                    .chars()
                                    .rev()
                                    .next()
                                    .unwrap_or(' '),
                            ) && !is_word_char(
                                context.input[position + s.len()..]
                                    .chars()
                                    .next()
                                    .unwrap_or(' '),
                            )))
                }
                None => false,
            }
        };

        let str_recognize = |s: &'i str| -> Option<&'i str> {
            if is_match(s, context.position) {
                Some(&context.input[context.position..(context.position + s.len())])
            } else {
                None
            }
        };

        let consume_until = |until_words: &[&str]| -> Option<&'i str> {
            let position: usize =
                context.input[context.position..]
                    .char_indices()
                    .take_while(|(idx, _)| {
                        !until_words
                            .iter()
                            .any(|&w| is_match(w, context.position + idx))
                    }).map(|(_, c)| c.len_utf8()).sum();

            if position > 0 {
                Some(&context.input[context.position..(context.position + position)])
            } else {
                None
            }
        };

        token_recognizers
            .iter()
            .find_map(|token_rec| -> Option<(TokenKind, &str)> {
                log!("{} {:?}", "Recognizing".green(), token_rec.token_kind);
                match token_rec.token_kind {
                    TokenKind::STOP => {
                        if context.position >= context.input.len() {
                            Some((TokenKind::STOP, ""))
                        } else {
                            None
                        }
                    }
                    TokenKind::OBrace => str_recognize("{").map(|value| (TokenKind::OBrace, value)),
                    TokenKind::CBrace => str_recognize("}").map(|value| (TokenKind::CBrace, value)),
                    TokenKind::ComponentKW => {
                        str_recognize("component").map(|value| (TokenKind::ComponentKW, value))
                    }
                    TokenKind::ConfigurationKW => str_recognize("configuration")
                        .map(|value| (TokenKind::ConfigurationKW, value)),
                    TokenKind::CodeKW => {
                        str_recognize("CODE").map(|value| (TokenKind::CodeKW, value))
                    }
                    TokenKind::EndCodeKW => {
                        str_recognize("ENDCODE").map(|value| (TokenKind::EndCodeKW, value))
                    }
                    TokenKind::TillEndCodeKW => {
                        consume_until(&["ENDCODE"]).map(|value| (TokenKind::TillEndCodeKW, value))
                    }
                    TokenKind::ModelKW => {
                        str_recognize("model").map(|value| (TokenKind::ModelKW, value))
                    }
                    TokenKind::LibraryKW => {
                        str_recognize("library").map(|value| (TokenKind::LibraryKW, value))
                    }
                    TokenKind::ID => RE_ID
                        .find(&context.input[context.position..])
                        .map(|m| (TokenKind::ID, m.as_str())),
                    TokenKind::Name => RE_NAME
                        .find(&context.input[context.position..])
                        .map(|m| (TokenKind::Name, m.as_str())),
                    TokenKind::CommentLine => {
                        let mut got_true = false;
                        if context.input[context.position..].starts_with("//") {
                            let skipped_len: usize = context.input[context.position..]
                                .chars()
                                .take_while(|x| {
                                    // Need to get the whole line including the newline.
                                    if got_true {
                                        return false;
                                    } else if *x == '\n' {
                                        got_true = true;
                                    }
                                    true
                                })
                                .map(|c| c.len_utf8())
                                .sum();
                            Some((
                                TokenKind::CommentLine,
                                &context.input[context.position..(context.position + skipped_len)],
                            ))
                        } else {
                            None
                        }
                    }
                    TokenKind::OComment => {
                        str_recognize("/*").map(|value| (TokenKind::OComment, value))
                    }
                    TokenKind::CComment => {
                        str_recognize("*/").map(|value| (TokenKind::CComment, value))
                    }
                    TokenKind::NotComment => consume_until(&["//", "/*", "*/"])
                        .map(|value| (TokenKind::NotComment, value)),
                    TokenKind::ModelProperty => RE_MODEL_PROPERTY
                        .find(&context.input[context.position..])
                        .map(|m| (TokenKind::ModelProperty, m.as_str())),
                    TokenKind::ConfigurationProperty => RE_CONFIGURATION_PROPERTY
                        .find(&context.input[context.position..])
                        .map(|m| (TokenKind::ConfigurationProperty, m.as_str())),
                    TokenKind::Anything => consume_until(&[
                        "//",
                        "/*",
                        "{",
                        "}",
                        r#"""#,
                        "component",
                        "model",
                        "library",
                        "configuration",
                        "CODE",
                        "ENDCODE",
                        "comment",
                        "START",
                        "ENDCOMMENT",
                    ])
                    .map(|value| (TokenKind::Anything, value)),
                    TokenKind::String => RE_STRING
                        .find(&context.input[context.position..])
                        .map(|m| (TokenKind::String, m.as_str())),
                    TokenKind::CommentKW => {
                        str_recognize("comment").map(|value| (TokenKind::CommentKW, value))
                    }
                    TokenKind::StartCommentKW => {
                        str_recognize("START").map(|value| (TokenKind::StartCommentKW, value))
                    }
                    TokenKind::EndCommentKW => {
                        str_recognize("ENDCOMMENT").map(|value| (TokenKind::EndCommentKW, value))
                    }
                    TokenKind::TillEndCommentKW => consume_until(&["ENDCOMMENT"])
                        .map(|value| (TokenKind::TillEndCommentKW, value)),
                }
            })
            .map(|(kind, value)| {
                log!("--- {}", "Match!".bold().green());
                Token {
                    kind,
                    value,
                    location: Location {
                        start: Position::Position(context.position),
                        end: Some(Position::Position(context.position + value.len())),
                    },
                }
            })
    }
}
