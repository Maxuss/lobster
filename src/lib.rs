// # LobsterChat
//!
//! A Minecraft [Chat Component](https://wiki.vg/Chat) and [Adventure MiniMessage](https://docs.adventure.kyori.net/minimessage/index.html) implementation in Rust.
//!
//! ## Examples
//!
//! ### Without minimessage
//!
//! ```rust
//! use lobsterchat::component::*;
//!
//! let text_component: Component = Component::text("Hello, World! ")
//!                     .color(0xFFAAFF)
//!                     .click_event(ClickEvent::open_url("https://github.com/Maxuss/lobster"))
//!                     .append(
//!                         Component::translatable("my.translation.key")
//!                         .color(NamedColor::Gold)
//!                         .hover_event(HoverEvent::ShowText(Component::text("Click for surprise!")))
//!                         .insert_text("I love lobsterchat!")
//!                     )
//!                     .append(
//!                         Component::keybind("key.sprint")
//!                         .bold(true)
//!                         .italic(false)
//!                     );
//!
//! println!("{}", text_component.to_string());
//! ```
//!
//! ### With minimessage
//!
//! ```rust
//! use lobsterchat::message::*;
//! use lobsterchat::component::{Component, Colored, NamedColor};
//!
//! let component: Component = lobster("<gold><bold>This is some message!</bold> <blue>Some blue text <#AAFFAA>Some hex text!");
//! let placeholdered: Component = placeholder_lobster(
//!     "Some normal text. <first> And then <gold><second>.",
//!     [
//!         (
//!             "first",
//!             Component::text("Some replacement.").color(NamedColor::Gold)
//!         ),
//!         (
//!             "second",
//!             Component::translatable("translated.text.key")
//!         )
//!     ])
//! ```
//!
//! Enable minimessage with the `minimessage` crate feature
//!
//! ### Speed:
//!
//! ```text
//! running 1 test
//! test tests::benchmark_lobster ... bench:       6,335 ns/iter (+/- 147)
//! ```
//!
//! So around 6mcs to convert message into a component.
//!
// ### Features
//!
//! - #### Components:
//! - [x] Component types (literal, translatable, etc.)
//! - [x] Formatting and colors
//! - [x] Click / Hover events in components
//!
//! - #### MiniMessage
//! - [x] Named color tags (e.g. `<red>, <blue>`)
//! - [x] Hex color tags (e.g. `<#AAFFAA>`)
//! - [x] Formatting tags (e.g. `<bold>, <reset>`)
//! - [x] Placeholder tags
//! - [ ] Hover / Click Events
//! - [ ] Advanced formatting tags (e.g. `<rainbow>, <gradient>`)

#![warn(missing_docs)]

pub mod component;
#[cfg(feature = "minimessage")]
pub mod message;
#[cfg(feature = "minimessage")]
pub use message::{lobster, placeholder_lobster};

#[cfg(test)]
#[cfg(feature = "minimessage")]
mod tests {
    #![allow(soft_unstable)]

    use crate::component::{AsComponent, ClickEvent, Component, HoverEvent};
    use crate::message::tokens::{MessageToken, Parser};
    use crate::{lobster, placeholder_lobster};
    use logos::Lexer;
    use logos::Logos;

    #[test]
    fn test_components() {
        let cmp = Component::text("Text component")
            .insert_text("Some text")
            .hover_event(HoverEvent::show_text("Some text".as_component()))
            .click_event(ClickEvent::open_url("https://github.com/Maxuss/lobster"));
        println!("{}", cmp.to_string())
    }

    #[test]
    #[cfg(feature = "minimessage")]
    fn test_lexer() {
        let mut lexer: Lexer<MessageToken> =
            MessageToken::lexer("<#AABBCC>Hex text<reset>Stop hex text");

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
        let lobster = placeholder_lobster(
            "Before placeholder, <replace_me> Stuff after placeholder. <another>",
            [
                (
                    "replace_me",
                    lobster("<aqua>This is a <dark_aqua>placeholder!<reset>"),
                ),
                ("another", lobster("<gold><bold>Another placeholder!")),
            ],
        );

        println!("{}", serde_json::to_string(&lobster).unwrap());
    }

    #[test]
    #[cfg(feature = "minimessage")]
    fn test_flattening() {
        let mut message =
            lobster("<red>Some message<blue> Even more message <green>Green message ").append(
                Component::translate::<&str, Component>("some.message.translate", None),
            );

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
