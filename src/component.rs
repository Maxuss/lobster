#![allow(clippy::field_reassign_with_default)]
//!
//! Main module containing all the component related things
//!

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::str::FromStr;
use uuid::Uuid;

/// This trait allows you to convert an object into a component
/// by passing it as reference
pub trait AsComponent {
    /// Converts this object reference to a component
    fn as_component(&self) -> Component;
}

impl<S> AsComponent for S
where
    S: Into<Component> + Clone,
{
    fn as_component(&self) -> Component {
        let cmp: Component = self.clone().into();
        cmp
    }
}

impl From<&str> for Component {
    fn from(str: &str) -> Self {
        Component::text(str)
    }
}

impl From<String> for Component {
    fn from(value: String) -> Self {
        Component::text(value)
    }
}

/// A container for item data to be displayed
/// See [wiki.vg](https://wiki.vg/Chat#Schema) for more info.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
#[skip_serializing_none]
pub struct DisplayItemData {
    /// Namespaced ID of this item. Stored in format of
    /// `namespace:identifier`
    pub id: String,
    /// Count of items to be displayed
    pub count: Option<i32>,
    /// Extra [SNBT](https://minecraft.fandom.com/wiki/NBT_format) tag containing item information, like
    /// the `display`, `Enchantments`, etc.
    pub tag: Option<String>,
}

/// A container for entity data to be displayed
/// See [wiki.vg](https://wiki.vg/Chat#Schema) for more info.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[skip_serializing_none]
pub struct DisplayEntityData {
    /// Optional display name of entity
    pub name: Option<Component>,
    /// Namespaced ID of this entity's type.
    /// Stored in format of `namespace:identifier`
    #[serde(rename = "type")]
    pub entity_type: String,
    /// Unique ID of this entity. Can usually be random, unless
    /// you want to display a specific entity.
    pub id: Uuid,
}

/// Container for component hover events.
/// See [wiki.vg](https://wiki.vg/Chat#Schema) for more info.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum HoverEvent {
    /// Shows text on hover
    ShowText {
        /// The component to be displayed.
        /// Boxed to avoid possible recursion problems.
        contents: Box<Component>,
    },
    /// Displays an item's tooltip to player
    ShowItem {
        /// Item data to be displayed.
        contents: DisplayItemData,
    },
    /// Displays entity data to player
    ShowEntity {
        /// The entity to be displayed.
        /// Boxed to avoid possible recursion problems.
        contents: Box<DisplayEntityData>,
    },
}

impl HoverEvent {
    /// Shows provided component on hover
    pub fn show_text(text: Component) -> HoverEvent {
        HoverEvent::ShowText {
            contents: Box::new(text),
        }
    }

    /// Shows provided item data on hover
    pub fn show_item(item_data: DisplayItemData) -> HoverEvent {
        HoverEvent::ShowItem {
            contents: item_data,
        }
    }

    /// Shows provided entity data on hover
    pub fn show_entity(entity_data: DisplayEntityData) -> HoverEvent {
        HoverEvent::ShowEntity {
            contents: Box::new(entity_data),
        }
    }
}

/// Container for click events
/// See [wiki.vg](https://wiki.vg/Chat#Schema) for more info.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action", content = "value")]
pub enum ClickEvent {
    /// Opens the provided URL on click
    OpenUrl(String),
    /// Runs the provided command on click
    RunCommand(String),
    /// Suggests (puts in the chat box) the provided command on click
    SuggestCommand(String),
    /// Changes current open book's page
    ChangePage(String),
    /// Copies provided data to clipboard
    CopyToClipboard(String),
}

impl ClickEvent {
    /// Opens the provided URL on click
    pub fn open_url<S: Into<String>>(url: S) -> Self {
        Self::OpenUrl(url.into())
    }

    /// Runs the provided command on click
    pub fn run_command<S: Into<String>>(cmd: S) -> Self {
        Self::RunCommand(cmd.into())
    }

    /// Suggests (puts in the chat box) the provided command on click
    pub fn suggest_command<S: Into<String>>(cmd: S) -> Self {
        Self::SuggestCommand(cmd.into())
    }

    /// Changes current open book's page7
    pub fn change_page(page: i32) -> Self {
        Self::ChangePage(page.to_string())
    }

    /// Copies provided data to clipboard
    pub fn copy_to_clipboard<S: Into<String>>(msg: S) -> Self {
        Self::CopyToClipboard(msg.into())
    }
}

/// The JSON chat component container
/// Note that the components are *immutable*, an they are cloned
/// each time they are modified.
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
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
    hover_event: Option<HoverEvent>,
}

impl ToString for Component {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

macro_rules! _fmt_impl {
    ($($n:ident $sn:expr => $gn:ident),* $(,)*) => {
        $(
            /// Gives or removes the `
            #[doc = $sn]
            ///` effect from this component
            pub fn $n(&mut self, $n: bool) -> Self {
                self.$n = Some($n);
                self.clone()
            }

            /// Returns whether the `
            #[doc = $sn]
            ///` effect is enabled in this component
            pub fn $gn(&self) -> bool {
                if let Some(val) = self.$n {
                    val
                } else {
                    false
                }
            }
            )*
    };

    ($($n:ident ($gn:ident)),* $(,)*) => {
        _fmt_impl!($($n stringify!($n) => $gn),*);
    }
}

/// A trait to color the components more easily
pub trait Colored<C> {
    /// Adds colored to the object
    fn color(&mut self, color: C) -> Self;
}

impl Colored<u32> for Component {
    fn color(&mut self, color: u32) -> Self {
        let str = format!("#{:2X}", color);
        self.color = Some(TextColor::Hex(str));
        self.clone()
    }
}

impl Colored<NamedColor> for Component {
    fn color(&mut self, color: NamedColor) -> Self {
        self.color = Some(TextColor::Named(color));
        self.clone()
    }
}

impl Colored<TextColor> for Component {
    fn color(&mut self, color: TextColor) -> Self {
        self.color = Some(color);
        self.clone()
    }
}

impl Component {
    /// Constructs a new literal text component.
    pub fn text<S>(msg: S) -> Self
    where
        S: Into<String>,
    {
        let mut df = Self::default();
        df.contents = MessageContents::Plain { text: msg.into() };
        df
    }

    /// Constructs a new translatable component.
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
        df
    }

    /// Constructs a new scoreboard component.
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
        df
    }

    /// Constructs a new entity-mentioning component
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
        df
    }

    /// Constructs a new keybind component
    pub fn keybind<S: Into<String>>(key: S) -> Self {
        let mut df = Self::default();
        df.contents = MessageContents::Keybind(KeyMessage {
            keybind: key.into(),
        });
        df
    }

    /// Constructs an entity nbt data based component
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
        df
    }

    /// Constructs a new storage nbt data based component
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
        df
    }

    /// Adds text that is inserted each time you click this component.
    /// Not connected to [ClickEvent]
    pub fn insert_text<S: Into<String>>(&mut self, text: S) -> Self {
        self.insertion = Some(text.into());
        self.clone()
    }

    /// Adds a click event handler to this component
    pub fn click_event(&mut self, e: ClickEvent) -> Self {
        self.click_event = Some(e);
        self.clone()
    }

    /// Adds a hover event handler to this component
    pub fn hover_event(&mut self, e: HoverEvent) -> Self {
        self.hover_event = Some(e);
        self.clone()
    }

    /// Appends another component to this one.
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

    /// Appends another component to the last child component.
    ///
    /// Imagine this structure:
    /// ```text
    /// --- component foo:
    ///     --- component bar
    ///     --- component baz
    /// ```
    /// When you normally [`Self::append()`] a component it gets pushed further, giving this structure:
    /// ```text
    /// --- component foo:
    ///     --- component bar
    ///     --- component baz
    ///     --- component quz
    /// ```
    /// But [`Self::append_to_last_child()`] instead pushes it to the last component in children stack:
    /// ```text
    /// --- component foo:
    ///     --- component bar
    ///     --- component baz:
    ///         --- component quz
    /// ```
    pub fn append_to_last_child(&mut self, comp: Component) -> Self {
        if let Some(vec) = &mut self.extra {
            let mut last = vec.pop().unwrap();
            last = last.append_to_last_child(comp);
            vec.push(last);
            self.clone()
        } else {
            self.extra = Some(vec![comp]);
            self.clone()
        }
    }

    /// Gets the current color of this component, or white if it is not assigned.
    pub fn get_color(&mut self) -> TextColor {
        match &self.color {
            None => TextColor::Named(NamedColor::White),
            Some(color) => color.to_owned(),
        }
    }

    /// Attempts to get text contents of this component.
    /// Returns [None] if this component is not a Literal Text Component
    pub fn get_text_content(&mut self) -> Option<String> {
        match &self.contents {
            MessageContents::Plain { text } => Some(text.clone()),
            _ => None,
        }
    }

    _fmt_impl! {
        bold(get_bold), italic(get_italic), obfuscated(get_obfuscated), strikethrough(get_strikethrough), underlined(get_underlined), reset(get_reset),
    }

    /// Adds a formatting to this component. Passing [None] to the `enable` argument is equivalent of passing `Some(true)`
    pub fn formatted(&mut self, format: Formatting, enable: Option<bool>) -> Component {
        let do_enable = enable.unwrap_or(true);

        match format {
            Formatting::Obfuscated => self.obfuscated(do_enable),
            Formatting::Bold => self.bold(do_enable),
            Formatting::Strikethrough => self.strikethrough(do_enable),
            Formatting::Underline => self.underlined(do_enable),
            Formatting::Italic => self.italic(do_enable),
            Formatting::Reset => self.reset(do_enable),
        }
    }

    /// Gets whether the specific formatting is enabled in this component
    pub fn get_formatting(&self, format: Formatting) -> bool {
        match format {
            Formatting::Obfuscated => self.get_obfuscated(),
            Formatting::Bold => self.get_bold(),
            Formatting::Strikethrough => self.get_strikethrough(),
            Formatting::Underline => self.get_underlined(),
            Formatting::Italic => self.get_italic(),
            Formatting::Reset => self.get_reset(),
        }
    }

    /// Flattens this component, getting the *approximate* contents of it
    pub fn flatten(&mut self) -> String {
        let mut buf = self.contents.flatten();

        if let Some(children) = &mut self.extra {
            for child in children.iter_mut() {
                buf.push_str(&child.flatten())
            }
        }

        buf
    }
}

/// Type of formatting for component
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Formatting {
    /// Renders component as obfuscated
    Obfuscated,
    /// Renders component in **bold**
    Bold,
    /// Renders component as ~strikethrough~
    Strikethrough,
    /// Renders component underlined
    Underline,
    /// Renders component in *italic*
    Italic,
    /// Resets the current component formatting
    Reset,
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
            _ => return Err(()),
        })
    }
}

/// Container for inner contents of a component
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum MessageContents {
    /// Literal text
    Plain {
        /// Text to be displayed
        text: String,
    },
    /// Translatable component
    Translate(TranslatedMessage),
    /// Scoreboard component
    Score {
        /// Scoreboard message to be displayed
        score: ScoreboardMessage,
    },
    /// Entity component
    Entity(Box<EntityMessage>),
    /// Keybind component
    Keybind(KeyMessage),
    /// NBT component
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
    /// Flattens this component, acquiring it's inner data
    pub fn flatten(&self) -> String {
        match self {
            MessageContents::Plain { text } => text.clone(),
            MessageContents::Translate(translated) => translated.translate.clone(),
            MessageContents::Score { .. } => "<scoreboard message>".into(),
            MessageContents::Entity(_) => "<entity message>".into(),
            MessageContents::Keybind(key) => key.keybind.clone(),
            MessageContents::Nbt(_) => "<nbt message>".into(),
        }
    }
}

/// NBT based message
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct NbtMessage {
    nbt: String,
    interpret: Option<bool>,
    separator: Option<Component>,
    block: Option<String>,
    entity: Option<String>,
    storage: Option<String>,
}

/// Keybind based message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct KeyMessage {
    keybind: String,
}

/// Entity based message
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct EntityMessage {
    selector: String,
    separator: Option<Component>,
}

/// Translatable message
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct TranslatedMessage {
    translate: String,
    with: Option<Vec<Component>>,
}

/// Scoreboard message
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct ScoreboardMessage {
    name: String,
    objective: String,
    value: Option<String>,
}

/// A text color formatting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum TextColor {
    /// A named color
    Named(NamedColor),
    /// A hex string color
    Hex(String),
}

/// A named color
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
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
            "gray" => Gray,
            "dark_gray" => DarkGray,
            "black" => Black,
            _ => return Err(()),
        })
    }
}
