#[cfg(feature = "minimessage")]
use message::tokens::{MessageToken, Parser};

pub mod component;
#[cfg(feature = "minimessage")]
pub mod message;
#[cfg(feature = "minimessage")]
pub use message::{lobster, placeholder_lobster};


#[cfg(test)]
mod tests {
    #![allow(soft_unstable)]

    use logos::Lexer;
    #[cfg(feature = "minimessage")]
    use crate::message::tokens::{MessageToken, Parser};
    #[cfg(feature = "minimessage")]
    use crate::{Component, lobster, placeholder_lobster};
    use crate::component::Component;
    #[cfg(feature = "minimessage")]
    use crate::message::{lobster, placeholder_lobster};

    #[test]
    #[cfg(feature = "minimessage")]
    fn test_lexer() {
        let mut lexer: Lexer<MessageToken> = MessageToken
            ::lexer("<#AABBCC>Hex text<reset>Stop hex text");

        while let Some(tk) = lexer.next() {
            println!("{:?}", tk)
        }
    }

    #[test]
    #[cfg(feature = "minimessage")]
    fn test_parser() {
        let lexer: Lexer<MessageToken> = MessageToken::lexer("<red>Red text");
        let mut parser = Parser::new(lexer);

        while let Ok(_) = parser.advance() {
            // no-op
        }
        let out = parser.finish();
        println!("{}", serde_json::to_string(&out).unwrap());
    }

    #[test]
    #[cfg(feature = "minimessage")]
    fn test_placeholders() {
        let lobster = placeholder_lobster("Before placeholder, <replace_me> Stuff after placeholder. <another>", [
            ("replace_me", lobster("<aqua>This is a <dark_aqua>placeholder!<reset>")),
            ("another", lobster("<gold><bold>Another placeholder!"))
        ]);

        println!("{}", serde_json::to_string(&lobster).unwrap());
    }

    #[test]
    #[cfg(feature = "minimessage")]
    fn test_flattening() {
        let mut message = lobster("<red>Some message<blue> Even more message <green>Green message ").append(Component::translate::<&str, Component>("some.message.translate", None));

        println!("{}", message.flatten())
    }

    // #[bench]
    // #[cfg(feature = "minimessage")]
    // fn benchmark_lobster(bencher: &mut Bencher) {
    //     bencher.iter(|| {
    //         test::black_box(lobster("<red>Red text <green>Green text <italic><yellow>Yellow italic text. <bold>BOLD. <red>Red text"))
    //     })
    // }
}