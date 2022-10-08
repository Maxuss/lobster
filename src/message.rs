use crate::component::{AsComponent, Component};
use crate::{MessageToken, Parser};
use logos::Lexer;

pub mod tokens;

pub fn lobster<S: Into<String>>(msg: S) -> Component {
    use logos::Logos;
    let st = msg.into();
    let lexer: Lexer<MessageToken> = MessageToken::lexer(&st);
    let parser = Parser::new(lexer);

    parser.parse()
}

pub fn placeholder_lobster<S: Into<String>, C: AsComponent + Sized, const N: usize>(
    msg: S,
    placeholders: [(S, C); N],
) -> Component {
    use logos::Logos;
    let st = msg.into();
    let lexer: Lexer<MessageToken> = MessageToken::lexer(&st);
    let mut parser = Parser::new(lexer);
    for (k, v) in placeholders {
        parser.placeholder(k, v)
    }

    parser.parse()
}
