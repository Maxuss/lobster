#![feature(test)]

extern crate test;

use logos::Lexer;
use crate::component::Component;
use crate::tokens::{MessageToken, Parser};

pub mod tokens;
pub mod component;


#[cfg(test)]
mod tests {
    #![allow(soft_unstable)]

    use test::Bencher;
    use crate::tokens::{MessageToken, Parser};
    use logos::{Logos, Lexer};
    use crate::lobster;

    #[test]
    fn test_lexer() {
        let mut lexer: Lexer<MessageToken> = MessageToken
            ::lexer("<#AABBCC>Hex text<reset>Stop hex text");

        while let Some(tk) = lexer.next() {
            println!("{:?}", tk)
        }
    }

    #[test]
    fn test_parser() {
        let lexer: Lexer<MessageToken> = MessageToken::lexer("<red>Red text");
        let mut parser = Parser::new(lexer);

        while let Ok(_) = parser.advance() {
            // no-op
        }
        let out = parser.finish();
        println!("{}", serde_json::to_string(&out).unwrap());
    }

    #[bench]
    fn benchmark_lobster(bencher: &mut Bencher) {
        bencher.iter(|| {
            test::black_box(lobster("<red>Red text <green>Green text <italic><yellow>Yellow italic text. <bold>BOLD. <red>Red text"))
        })
    }
}

#[inline]
pub fn lobster<S: Into<String>>(msg: S) -> Component {
    use logos::Logos;
    let st = msg.into();
    let lexer: Lexer<MessageToken> = MessageToken::lexer(&st);
    let mut parser = Parser::new(lexer);

    parser.parse()
}