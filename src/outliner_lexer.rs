use crate::outliner::{State, TokenKind};

use crate::outliner_actions::Ctx;
#[cfg(debug_assertions)]
use colored::*;
use lazy_static::lazy_static;
use regex::Regex;
use rustemo::{log, Context, Input, Lexer, Location, Position, Token};

pub type InputType = str;

lazy_static! {
    static ref RE_WS: Regex = Regex::new(r"^\s+").unwrap();
    static ref RE_ID: Regex = Regex::new(r"^[^\d\W]\w*\b").unwrap();
    static ref RE_NAME: Regex =
        Regex::new(r#"^("(\\"|[^"])*")|^('(\\'|[^'])*')|^(\w|\+|-)+"#).unwrap();
    static ref RE_STRING: Regex = Regex::new(r#"^(("(\\"|[^"])*")|('(\\'|[^'])*'))"#).unwrap();
    static ref RE_MODEL_PROPERTY: Regex = Regex::new(r#"^model\s+="#).unwrap();
    static ref RE_CONFIGURATION_PROPERTY: Regex = Regex::new(r#"^configuration\s+="#).unwrap();
}

pub struct OutlinerLexer();

impl OutlinerLexer {
    pub fn new() -> Self {
        Self()
    }

    fn skip<'i>(&self, context: &mut Ctx<'i>, input: &'i InputType) {
        let skipped_len: usize = input[context.position()..]
            .chars()
            .take_while(|x| x.is_whitespace())
            .map(|c| c.len_utf8())
            .sum();
        let skipped = &input[context.position()..context.position() + skipped_len];
        log!("{}: {}", "Skipped ws".green(), skipped_len);
        if skipped_len > 0 {
            context.set_layout_ahead(Some(skipped));
            context.set_position(context.position() + skipped_len);
        } else {
            context.set_layout_ahead(None);
        }
        context.set_location(skipped.location_after(context.location()));
    }
}

impl<'i> Lexer<'i, Ctx<'i>, State, TokenKind> for OutlinerLexer {
    type Input = InputType;

    fn next_tokens(
        &self,
        c: &mut Ctx<'i>,
        i: &'i Self::Input,
        token_kinds: Vec<(TokenKind, bool)>,
    ) -> Box<dyn Iterator<Item = Token<'i, Self::Input, TokenKind>> + 'i> {
        self.skip(c, i);

        fn is_word_char(c: char) -> bool {
            c.is_alphabetic() || c == '-' || c == '_'
        }
        fn is_id(s: &str) -> bool {
            s.chars().all(is_word_char)
        }
        /// Match on a word boundaries for ID-like strings
        fn is_match(s: &str, input: &str, position: usize) -> bool {
            match input.get(position..position + s.len()) {
                Some(subs) => {
                    subs == s
                        && (!is_id(s)
                            || (!is_word_char(
                                input[..position].chars().next_back().unwrap_or(' '),
                            ) && !is_word_char(
                                input[position + s.len()..].chars().next().unwrap_or(' '),
                            )))
                }
                None => false,
            }
        }

        fn str_recognize<'i>(input: &'i str, context: &Ctx, s: &str) -> Option<&'i str> {
            if is_match(s, input, context.position()) {
                Some(&input[context.position()..(context.position() + s.len())])
            } else {
                None
            }
        }

        /// Consumes and returns a slice of string until one of until_words is
        /// matched (not including until_word).
        fn consume_until<'i>(
            input: &'i str,
            context: &Ctx,
            until_words: &[&str],
        ) -> Option<&'i str> {
            let position: usize = input[context.position()..]
                .char_indices()
                .take_while(|(idx, _)| {
                    !until_words
                        .iter()
                        .any(|&w| is_match(w, input, context.position() + idx))
                })
                .map(|(_, c)| c.len_utf8())
                .sum();

            if position > 0 {
                Some(&input[context.position()..(context.position() + position)])
            } else {
                None
            }
        }

        Box::new(
            token_kinds
                .iter()
                .filter_map(|(token_kind, _finish)| -> Option<(TokenKind, &str)> {
                    log!("{} {:?}", "Recognizing".green(), token_kind);
                    match token_kind {
                        TokenKind::STOP => {
                            if c.position() >= i.len() {
                                Some((TokenKind::STOP, ""))
                            } else {
                                None
                            }
                        }
                        TokenKind::OBrace => {
                            str_recognize(i, c, "{").map(|value| (TokenKind::OBrace, value))
                        }
                        TokenKind::CBrace => {
                            str_recognize(i, c, "}").map(|value| (TokenKind::CBrace, value))
                        }
                        TokenKind::ComponentKW => str_recognize(i, c, "component")
                            .map(|value| (TokenKind::ComponentKW, value)),
                        TokenKind::ConfigurationKW => str_recognize(i, c, "configuration")
                            .map(|value| (TokenKind::ConfigurationKW, value)),
                        TokenKind::CodeKW => {
                            str_recognize(i, c, "CODE").map(|value| (TokenKind::CodeKW, value))
                        }
                        TokenKind::EndCodeKW => str_recognize(i, c, "ENDCODE")
                            .map(|value| (TokenKind::EndCodeKW, value)),
                        TokenKind::TillEndCodeKW => consume_until(i, c, &["ENDCODE"])
                            .map(|value| (TokenKind::TillEndCodeKW, value)),
                        TokenKind::ModelKW => {
                            str_recognize(i, c, "model").map(|value| (TokenKind::ModelKW, value))
                        }
                        TokenKind::LibraryKW => str_recognize(i, c, "library")
                            .map(|value| (TokenKind::LibraryKW, value)),
                        TokenKind::ID => RE_ID
                            .find(&i[c.position()..])
                            .map(|m| (TokenKind::ID, m.as_str())),
                        TokenKind::Name => RE_NAME
                            .find(&i[c.position()..])
                            .map(|m| (TokenKind::Name, m.as_str())),
                        TokenKind::CommentLine => {
                            let mut got_true = false;
                            if i[c.position()..].starts_with("//") {
                                let skipped_len: usize = i[c.position()..]
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
                                    &i[c.position()..(c.position() + skipped_len)],
                                ))
                            } else {
                                None
                            }
                        }
                        TokenKind::CommentName => RE_ID
                            .find(&i[c.position()..])
                            .and_then(|name| {
                                let name = name.as_str();
                                if name == "START" {
                                    None
                                } else {
                                    Some(name)
                                }
                            })
                            .map(|m| (TokenKind::CommentName, m)),
                        TokenKind::OComment => {
                            str_recognize(i, c, "/*").map(|value| (TokenKind::OComment, value))
                        }
                        TokenKind::CComment => {
                            str_recognize(i, c, "*/").map(|value| (TokenKind::CComment, value))
                        }
                        TokenKind::NotComment => consume_until(i, c, &["//", "/*", "*/"])
                            .map(|value| (TokenKind::NotComment, value)),
                        TokenKind::ModelProperty => RE_MODEL_PROPERTY
                            .find(&i[c.position()..])
                            .map(|m| (TokenKind::ModelProperty, m.as_str())),
                        TokenKind::ConfigurationProperty => RE_CONFIGURATION_PROPERTY
                            .find(&i[c.position()..])
                            .map(|m| (TokenKind::ConfigurationProperty, m.as_str())),
                        TokenKind::WS => RE_WS
                            .find(&i[c.position()..])
                            .map(|m| (TokenKind::WS, m.as_str())),
                        TokenKind::Anything => consume_until(
                            i,
                            c,
                            &[
                                "//",
                                "/*",
                                "{",
                                "}",
                                r#"""#,
                                r#"'"#,
                                "component",
                                "model",
                                "library",
                                //"configuration",
                                "CODE",
                                "ENDCODE",
                                "comment",
                                "START",
                                "ENDCOMMENT",
                            ],
                        )
                        .map(|value| (TokenKind::Anything, value)),
                        TokenKind::String => RE_STRING
                            .find(&i[c.position()..])
                            .map(|m| (TokenKind::String, m.as_str())),
                        TokenKind::CommentKW => str_recognize(i, c, "comment")
                            .map(|value| (TokenKind::CommentKW, value)),
                        TokenKind::StartCommentKW => str_recognize(i, c, "START")
                            .map(|value| (TokenKind::StartCommentKW, value)),
                        TokenKind::EndCommentKW => str_recognize(i, c, "ENDCOMMENT")
                            .map(|value| (TokenKind::EndCommentKW, value)),
                        TokenKind::TillEndCommentKW => consume_until(i, c, &["ENDCOMMENT"])
                            .map(|value| (TokenKind::TillEndCommentKW, value)),
                    }
                })
                .map(|(kind, value)| {
                    log!("    --- {}", "Match!".bold().green());
                    Token {
                        kind,
                        value,
                        location: Location {
                            start: Position::Position(c.position()),
                            end: Some(Position::Position(c.position() + value.len())),
                        },
                    }
                })
                .collect::<Vec<_>>()
                .into_iter(),
        )
    }
}
