//! This module contains minimessage implementation and related function

use crate::{
    component::{AsComponent, Component},
    message::tokens::Parser,
};
use logos::Lexer;

pub(crate) mod tokens;

/// Constructs a component from the provided minimessage string
/// See [Adventure MiniMessage](https://docs.adventure.kyori.net/minimessage/index.html) for more info
pub fn lobster<S: Into<String>>(msg: S) -> Component {
    use logos::Logos;
    let st = msg.into();
    let lexer: Lexer<tokens::MessageToken> = tokens::MessageToken::lexer(&st);
    let parser = Parser::new(lexer);

    parser.parse()
}

/// Constructs a component from the provided minimessage string and placeholders
/// See [Adventure MiniMessage](https://docs.adventure.kyori.net/minimessage/index.html) for more info
pub fn placeholder_lobster<S: Into<String>, C: AsComponent + Sized, const N: usize>(
    msg: S,
    placeholders: [(S, C); N],
) -> Component {
    use logos::Logos;
    let st = msg.into();
    let lexer: Lexer<tokens::MessageToken> = tokens::MessageToken::lexer(&st);
    let mut parser = Parser::new(lexer);
    for (k, v) in placeholders {
        parser.placeholder(k, v)
    }

    parser.parse()
}
