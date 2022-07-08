use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::str::FromStr;
use anyhow::bail;
use logos::{Lexer, Logos};
use crate::component::{AsComponent, Component, Formatting, NamedColor};

fn grab_placeholder(lex: &mut Lexer<MessageToken>) -> Option<String> {
    let slice: &str = lex.slice();
    // skipping begin tags
    return Some(slice[1..slice.len() - 1].to_string());
}

fn grab_named_color(lex: &mut Lexer<MessageToken>) -> Option<NamedColor> {
    let slice: &str = lex.slice();
    let inner = &slice[1..slice.len() - 1];
    Some(NamedColor::from_str(inner).ok()?)
}

fn grab_formatting(lex: &mut Lexer<MessageToken>) -> Option<(Formatting, bool)> {
    let slice: &str = lex.slice();
    let inner = &slice[1..slice.len() - 1];
    return if inner.starts_with("/") {
        Some((Formatting::from_str(&inner[1..]).ok()?, false))
    } else {
        Some((Formatting::from_str(inner).ok()?, true))
    }
}

fn grab_string(lex: &mut Lexer<MessageToken>) -> Option<String> {
    let slice: &str = lex.slice();
    Some(slice.into())
}

fn grab_hex(lex: &mut Lexer<MessageToken>) -> Option<u32> {
    let slice: &str = lex.slice();
    let inner = &slice[2..slice.len() - 1];
    Some(u32::from_str_radix(inner, 16).ok()?)
}

#[derive(Debug, Clone, Logos)]
pub enum MessageToken {
    #[regex("<#[\\da-fA-F]+>", grab_hex)]
    HexColor(u32),

    #[regex("<(dark_red|red|gold|yellow|dark_green|green|aqua|dark_aqua|dark_blue|blue|light_purple|dark_purple|white|gray|dark_gray|black)>", grab_named_color)]
    NamedColor(NamedColor),

    #[regex("</?(obfuscated|bold|strikethrough|underline|italic|reset)>", grab_formatting)]
    Formatting((Formatting, bool)),

    #[regex("<[^\\\\/\\s^<>#]+>", grab_placeholder)]
    PlaceholderTag(String),

    #[regex("[^<>]+", grab_string)]
    Contents(String),

    #[error]
    Error
}

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    tokens: Lexer<'a, MessageToken>,
    stack: VecDeque<MessageToken>,
    placeholders: HashMap<String, Component>,
    current: Component
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a, MessageToken>) -> Self {
        Self {
            tokens: lexer,
            stack: VecDeque::new(),
            placeholders: HashMap::default(),
            current: Component::default()
        }
    }

    pub fn placeholder<S: Into<String>, P: AsComponent>(&mut self, name: S, placeholder: P) {
        self.placeholders.insert(name.into(), placeholder.as_component());
    }

    pub fn parse(mut self) -> Component {
        while let Ok(()) = self.advance() {
            // no-op
        };
        self.finish()
    }

    pub fn advance(&mut self) -> anyhow::Result<()> {
        if let Some(tk) = self.tokens.next() {
            return match tk {
                MessageToken::PlaceholderTag(placeholder) => {
                    if !self.placeholders.contains_key(&placeholder) {
                        bail!("Undefined placeholder: '{}'!", placeholder)
                    }
                    let ph = self.placeholders.get(&placeholder).unwrap();
                    self.current = self.current.append(ph.clone());
                    Ok(())
                }
                MessageToken::Contents(contents) => {
                    let mut text = Component::text(&contents);
                    while let Some(stacked) = self.stack.pop_front() {
                        match stacked {
                            MessageToken::HexColor(hex) => {
                                text = text.hex_color(hex);
                            }
                            MessageToken::NamedColor(color) => {
                                text = text.color(color);
                            }
                            MessageToken::Formatting((fmt, enable)) => {
                                text = text.formatted(fmt, Some(enable));
                            }
                            invalid => {
                                bail!("Invalid token found in stack: {:?}!", invalid)
                            }
                        }
                    }
                    self.current = self.current.append_to_last_child(text);
                    Ok(())
                }
                MessageToken::Error => {
                    bail!("Unexpected parsing error!")
                }
                other => {
                    self.stack.push_back(other);
                    Ok(())
                }
            }
        } else {
            bail!("EOF Reached!")
        }
    }

    pub fn finish(self) -> Component {
        self.current
    }
}