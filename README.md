# Lobster

A Minecraft [Chat Component](https://wiki.vg/Chat) and[Adventure MiniMessage](https://docs.adventure.kyori.net/minimessage/index.html) implementation in Rust.

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