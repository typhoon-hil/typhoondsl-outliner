use crate::outliner::TokenRecognizer;

use super::outliner::TokenKind;
use regex::Regex;
use rustemo::{
    lexer::{self, Context, Lexer, Token},
    location::{Location, Position},
    log,
};
use colored::*;

pub type Input = str;

pub struct OutlinerLexer();

impl OutlinerLexer {
    pub fn new() -> Self {
        Self()
    }

    fn skip(&self, context: &mut Context<Input>) {
        let skipped_len = context.input[context.position..]
            .chars()
            .take_while(|x| x.is_whitespace())
            .count();
        let skipped = &context.input[context.position..context.position + skipped_len];
        log!("Skipped ws: {}", skipped.len());
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
        let str_recognize = |s: &'i str| -> Option<&'i str> {
            if context.input[context.position..].starts_with(s) {
                Some(&context.input[context.position..(context.position + s.len())])
            } else {
                None
            }
        };

        let consume_until = |until_words: &[&str]| -> Option<&'i str> {
            log!(
                "Current position: {}",
                &context.input[context.position..]
                    .chars()
                    .take(10)
                    .collect::<String>()
            );
            let is_word_char = |c: char| -> bool { c.is_alphabetic() || c == '-' || c == '_' };
            let is_id = |s: &str| -> bool { s.chars().all(is_word_char) };
            // To keep track of word constituent chars, and thus detect word
            // boundaries, start with the last char.
            let mut word_constituent = is_word_char(
                context.input[..context.position]
                    .chars()
                    .rev()
                    .next()
                    .unwrap_or(' '),
            );
            let position = context.input[context.position..]
                .char_indices()
                .find_map(|(idx, c)| {
                    // Check the word boundaries by keeping track of whether the
                    // previous char was word constituent. This is relevant only
                    // if the word for matching is ID.
                    let last_word_constituent = word_constituent;
                    word_constituent = is_word_char(c);
                    until_words.iter().find_map(|&w| {
                        if (!last_word_constituent || !is_id(w))
                            && context.input[(context.position + idx)..].starts_with(w)
                            && (!is_id(w)
                                || !is_word_char(
                                    context.input[context.position + idx + w.len()..]
                                        .chars()
                                        .next()
                                        .unwrap_or(' '),
                                ))
                        {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                });

            if let Some(position) = position {
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
                    TokenKind::NotKW => {
                        consume_until(&["component", "model", "configuration", "CODE", "ENDCODE"])
                            .map(|value| (TokenKind::NotKW, value))
                    }
                    TokenKind::ID => Regex::new(r"^[^\d\W]\w*\b")
                        .unwrap()
                        .find(&context.input[context.position..])
                        .map(|m| (TokenKind::ID, m.as_str())),
                    TokenKind::Name => Regex::new(r#"^("(\\"|[^"])*")|('(\\'|[^'])*')|(\w|\+|-)+"#)
                        .unwrap()
                        .find(&context.input[context.position..])
                        .map(|m| (TokenKind::Name, m.as_str())),
                    TokenKind::CommentLine => {
                        if context.input[context.position..].starts_with("//") {
                            let skipped_len = context.input[context.position..]
                                .chars()
                                .take_while(|x| *x != '\n')
                                .count();
                            Some((
                                TokenKind::CommentLine,
                                &context.input
                                    [context.position..(context.position + skipped_len + 1)],
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
                    TokenKind::Anything => consume_until(&[
                        "//",
                        "/*",
                        "{",
                        "}",
                        "component",
                        "model",
                        "configuration",
                        "CODE",
                        "ENDCODE",
                    ])
                    .and_then(|value| {
                        if !value.is_empty() {
                            Some((TokenKind::Anything, value))
                        } else {
                            None
                        }
                    }),
                }
            })
            .map(|(kind, value)| {
                log!("{}", "Match!".bold().green());
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
