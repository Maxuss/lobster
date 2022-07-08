use std::str::FromStr;
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

pub trait AsComponent {
    fn as_component(&self) -> Component;
}

impl<S> AsComponent for S where S: Into<Component> + Clone {
    fn as_component(&self) -> Component {
        self.clone().into()
    }
}

impl From<&str> for Component {
    fn from(str: &str) -> Self {
        str.as_component()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct DisplayItemData {
    pub id: String,
    pub count: Option<i32>,
    pub tag: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct DisplayEntityData {
    pub name: Option<Component> ,
    #[serde(rename = "type")]
    pub entity_type: String,
    pub id: Uuid
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverEvent {
    ShowText {
        action: String,
        contents: Box<Component>
    },
    ShowItem {
        action: String,
        contents: DisplayItemData
    },
    ShowEntity {
        action: String,
        contents: Box<DisplayEntityData>
    }
}

impl HoverEvent {
    pub fn show_text(text: Component) -> HoverEvent {
        HoverEvent::ShowText {
            action: "show_text".to_string(),
            contents: Box::new(text)
        }
    }

    pub fn show_item(item_data: DisplayItemData) -> HoverEvent {
        HoverEvent::ShowItem {
            action: "show_item".to_string(),
            contents: item_data
        }
    }

    pub fn show_entity(entity_data: DisplayEntityData) -> HoverEvent {
        HoverEvent::ShowEntity {
            action: "show_entity".to_string(),
            contents: Box::new(entity_data)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickEvent {
    action: ClickAction,
    value: String
}

impl ClickEvent {
    pub fn open_url<S: Into<String>>(url: S) -> Self {
        Self {
            action: ClickAction::OpenUrl,
            value: url.into()
        }
    }

    pub fn run_command<S: Into<String>>(cmd: S) -> Self {
        Self {
            action: ClickAction::RunCommand,
            value: cmd.into()
        }
    }

    pub fn suggest_command<S: Into<String>>(cmd: S) -> Self {
        Self {
            action: ClickAction::SuggestCommand,
            value: cmd.into()
        }
    }

    pub fn change_page(page: i32) -> Self {
        Self {
            action: ClickAction::ChangePage,
            value: page.to_string()
        }
    }

    pub fn copy_to_clipboard<S: Into<String>>(msg: S) -> Self {
        Self {
            action: ClickAction::CopyToClipboard,
            value: msg.into()
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ClickAction {
    OpenUrl,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    extra: Option<Vec<Component>>,
    bold: Option<bool>,
    italic: Option<bool>,
    obfuscated: Option<bool>,
    strikethrough: Option<bool>,
    underlined: Option<bool>,
    reset: Option<bool>,
    color: Option<TextColor>,
    #[serde(flatten)]
    contents: MessageContents,
    insertion: Option<String>,
    #[serde(rename = "clickEvent")]
    click_event: Option<ClickEvent>,
    #[serde(rename = "hoverEvent")]
    hover_event: Option<HoverEvent>
}

impl ToString for Component {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

macro_rules! _fmt_impl {
    ($($n:ident),* $(,)*) => {
        $(
        pub fn $n(&mut self, $n: bool) -> Self {
            self.$n = Some($n);
            self.clone()
        }
        )*
    }
}

impl Component {
    pub fn text<S>(msg: S) -> Self
        where
            S: Into<String>,
    {
        let mut df = Self::default();
        df.contents = MessageContents::Plain { text: msg.into() };
        df.clone()
    }

    pub fn translate<S, C>(msg: S, placeholders: Option<Vec<C>>) -> Self
        where
            S: Into<String>,
            C: AsComponent,
    {
        let mut df = Self::default();
        df.contents = MessageContents::Translate(TranslatedMessage {
            translate: msg.into(),
            with: placeholders.map(|it| {
                it.iter()
                    .map(|e| e.as_component())
                    .collect::<Vec<Component>>()
            }),
        });
        df.clone()
    }

    pub fn score<S>(name: S, objective: S, placeholder: Option<S>) -> Self
        where
            S: Into<String>,
    {
        let mut df = Self::default();
        df.contents = MessageContents::Score {
            score: ScoreboardMessage {
                name: name.into(),
                objective: objective.into(),
                value: placeholder.map(|it| it.into()),
            },
        };
        df.clone()
    }

    pub fn entity<S, C>(selector: S, separator: Option<C>) -> Self
        where
            S: Into<String>,
            C: AsComponent,
    {
        let mut df = Self::default();
        df.contents = MessageContents::Entity(Box::from(EntityMessage {
            selector: selector.into(),
            separator: separator.map(|it| it.as_component()),
        }));
        df.clone()
    }

    pub fn keybind<S: Into<String>>(key: S) -> Self {
        let mut df = Self::default();
        df.contents = MessageContents::Keybind(KeyMessage { keybind: key.into() });
        df.clone()
    }

    pub fn entity_nbt<S, C>(
        path: S,
        selector: S,
        interpret: Option<bool>,
        separator: Option<C>,
    ) -> Self
        where
            S: Into<String>,
            C: AsComponent,
    {
        let mut df = Self::default();
        df.contents = MessageContents::Nbt(Box::from(NbtMessage {
            nbt: path.into(),
            interpret,
            separator: separator.map(|it| it.as_component()),
            block: None,
            entity: Some(selector.into()),
            storage: None,
        }));
        df.clone()
    }

    pub fn storage_nbt<S, C>(
        path: S,
        storage: S,
        interpret: Option<bool>,
        separator: Option<C>,
    ) -> Self
        where
            S: Into<String>,
            C: AsComponent,
    {
        let mut df = Self::default();
        df.contents = MessageContents::Nbt(Box::from(NbtMessage {
            nbt: path.into(),
            interpret,
            separator: separator.map(|it| it.as_component()),
            block: None,
            entity: None,
            storage: Some(storage.into()),
        }));
        df.clone()
    }

    pub fn insert_text<S: Into<String>>(&mut self, text: S) -> Self {
        self.insertion = Some(text.into());
        self.clone()
    }

    pub fn click_event(&mut self, e: ClickEvent) -> Self {
        self.click_event = Some(e);
        self.clone()
    }

    pub fn hover_event(&mut self, e: HoverEvent) -> Self {
        self.hover_event = Some(e);
        self.clone()
    }

    pub fn append<C>(&mut self, comp: C) -> Self
        where
            C: Into<Component>,
    {
        if let Some(vec) = &mut self.extra {
            vec.push(comp.into());
            self.extra = Some(vec.to_owned())
        } else {
            self.extra = Some(vec![comp.into()])
        }
        self.clone()
    }

    pub fn append_to_last_child(&mut self, comp: Component) -> Self {
        return if let Some(vec) = &mut self.extra {
            let mut last = vec.pop().unwrap();
            last = last.append_to_last_child(comp);
            vec.push(last);
            self.clone()
        } else {
            self.extra = Some(vec![comp]);
            self.clone()
        }
    }

    pub fn hex_color(&mut self, color: u32) -> Self {
        let str = format!("#{:2X}", color);
        self.color = Some(TextColor::Hex(str));
        self.clone()
    }

    pub fn color(&mut self, color: NamedColor) -> Self {
        self.color = Some(TextColor::Named(color));
        self.clone()
    }

    _fmt_impl! {
        bold, italic, obfuscated, strikethrough, underlined, reset,
    }

    pub fn formatted(&mut self, format: Formatting, enable: Option<bool>) -> Component {
        let do_enable = enable.unwrap_or(true);

        match format {
            Formatting::Obfuscated => self.obfuscated(do_enable),
            Formatting::Bold => self.bold(do_enable),
            Formatting::Strikethrough => self.strikethrough(do_enable),
            Formatting::Underline => self.underlined(do_enable),
            Formatting::Italic => self.italic(do_enable),
            Formatting::Reset => self.reset(do_enable)
        }
    }

    pub fn flatten(&mut self) -> String {
        let mut buf = self.contents.flatten();

        if let Some(children) = &mut self.extra {
            for child in children.iter_mut() {
                buf.push_str(&child.flatten())
            }
        }

        return buf
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Formatting {
    Obfuscated,
    Bold,
    Strikethrough,
    Underline,
    Italic,
    Reset
}

impl FromStr for Formatting {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Formatting::*;

        Ok(match s {
            "obfuscated" => Obfuscated,
            "bold" => Bold,
            "strikethrough" => Strikethrough,
            "underline" => Underline,
            "italic" => Italic,
            "reset" => Reset,
            _ => return Err(())
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContents {
    Plain { text: String },
    Translate(TranslatedMessage),
    Score { score: ScoreboardMessage },
    Entity(Box<EntityMessage>),
    Keybind(KeyMessage),
    Nbt(Box<NbtMessage>),
}

impl Default for MessageContents {
    fn default() -> Self {
        MessageContents::Plain {
            text: "".to_string(),
        }
    }
}

impl MessageContents {
    pub fn flatten(&self) -> String {
        match self {
            MessageContents::Plain { text } => text.clone(),
            MessageContents::Translate(translated) => translated.translate.clone(),
            MessageContents::Score { .. } => "<scoreboard message>".into(),
            MessageContents::Entity(_) => "<entity message>".into(),
            MessageContents::Keybind(key) => key.keybind.clone(),
            MessageContents::Nbt(_) => "<nbt message>".into()
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NbtMessage {
    nbt: String,
    interpret: Option<bool>,
    separator: Option<Component>,
    block: Option<String>,
    entity: Option<String>,
    storage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMessage {
    keybind: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMessage {
    selector: String,
    separator: Option<Component>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslatedMessage {
    translate: String,
    with: Option<Vec<Component>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreboardMessage {
    name: String,
    objective: String,
    value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextColor {
    Named(NamedColor),
    Hex(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamedColor {
    DarkRed,
    Red,
    Gold,
    Yellow,
    DarkGreen,
    Green,
    Aqua,
    DarkAqua,
    DarkBlue,
    Blue,
    LightPurple,
    DarkPurple,
    White,
    Gray,
    DarkGray,
    Black,
}

impl FromStr for NamedColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use NamedColor::*;
        Ok(match s {
            "dark_red" => DarkRed,
            "red" => Red,
            "gold" => Gold,
            "yellow" => Yellow,
            "green" => Green,
            "dark_green" => DarkGreen,
            "aqua" => Aqua,
            "dark_aqua" => DarkAqua,
            "dark_blue" => DarkBlue,
            "blue" => Blue,
            "light_purple" => LightPurple,
            "dark_purple" => DarkPurple,
            "white" => White,
            "gray" =>  Gray,
            "dark_gray" => DarkGray,
            "black" => Black,
            _ => return Err(())
        })
    }
}