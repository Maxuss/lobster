# LobsterChat

A Minecraft [Chat Component](https://wiki.vg/Chat) and [Adventure MiniMessage](https://docs.adventure.kyori.net/minimessage/index.html) implementation in Rust.

## Examples

### Without minimessage

```rust
use lobsterchat::component::*;

let text_component: Component = Component::text("Hello, World! ")
                    .color(0xFFAAFF)
                    .click_event(ClickEvent::open_url("https://github.com/Maxuss/lobster"))
                    .append(
                        Component::translatable("my.translation.key")
                        .color(NamedColor::Gold)
                        .hover_event(HoverEvent::ShowText(Component::text("Click for surprise!")))
                        .insert_text("I love lobsterchat!")
                    )
                    .append(
                        Component::keybind("key.sprint")
                        .bold(true)
                        .italic(false)
                    );

println!("{}", text_component.to_string());
```

### With minimessage

```rust
use lobsterchat::message::*;
use lobsterchat::component::{Component, Colored, NamedColor};

let component: Component = lobster("<gold><bold>This is some message!</bold> <blue>Some blue text <#AAFFAA>Some hex text!");
let placeholdered: Component = placeholder_lobster(
    "Some normal text. <first> And then <gold><second>.",
    [
        (
            "first",
            Component::text("Some replacement.").color(NamedColor::Gold)
        ),
        (
            "second",
            Component::translatable("translated.text.key")
        )
    ])
```

Enable minimessage with the `minimessage` crate feature

### Speed:

```
running 1 test
test tests::benchmark_lobster ... bench:       6,335 ns/iter (+/- 147)
```

So around 6mcs to convert message into a component.

### Features


- #### Components:
- [x] Component types (literal, translatable, etc.)
- [x] Formatting and colors
- [x] Click / Hover events in components

- #### MiniMessage
- [x] Named color tags (e.g. `<red>, <blue>`)
- [x] Hex color tags (e.g. `<#AAFFAA>`)
- [x] Formatting tags (e.g. `<bold>, <reset>`)
- [x] Placeholder tags
- [ ] Hover / Click Events
- [ ] Advanced formatting tags (e.g. `<rainbow>, <gradient>`)