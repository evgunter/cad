---
id: the-wheels-modifier-decisions-are-outside-the-binding-table
kind: issue
title: viewer: what a wheel does under a modifier is decided in the toolkit adapter, so the controls guide cannot list it
status: open
opened: 2026-11-22
priority: P2
cost: M
---

Found by the review of PR 7336 (the controls guide), which makes `InputMap::bindings` (`crates/viewer/src/input.rs`) the one table the mapper and the guide read.

Three decisions about the wheel are made before that table is asked, in `crates/viewer/src/pane/viewport.rs`:

- `configure_input` turns the toolkit's wheel-zoom routing off, so a wheel turned with ctrl or ⌘ held reaches `scroll_event` as a plain wheel.
- An alt+wheel arrives on the `y` axis by the toolkit's own folding (`vertical_scroll_modifier`), so it is a plain wheel too.
- A shift+wheel is folded by the toolkit onto the `x` axis (`horizontal_scroll_modifier`), and `scroll_event` drops `x`, so it does nothing.

The guide's one row, "Scroll | zoom", says none of this. The first two are consistent with it under the table's own rule (a key a row does not name is not asked about). The third is not: shift+wheel does not zoom, and no row says so. The tests that hold these three are in `pane/viewport.rs`'s `mod tests` (`a wheel with … held zooms as a plain wheel does`, `the wheel did arrive, on the axis the viewer has no gesture for`), and `crates/viewer/README.md`'s "Mouse bindings" section states them in prose that names where they are decided.

Why PR 7336 did not make them rows (`Binding::Scroll { held }`):

- **`Held` has no ctrl/⌘.** `ViewportEvent::Drag` carries `shift` and `alt` and `viewer_modifiers` discards the other three toolkit fields on purpose. A ctrl+wheel row needs a third key in `Held`, in `ViewportEvent::Scroll` and, so that a ctrl row on a drag is not a row the adapter cannot deliver, in `ViewportEvent::Drag` and every suite that builds one.
- **shift+wheel cannot be told from a sideways scroll.** By the time the adapter sees it, the toolkit has moved it to `x`, where a trackpad's sideways swipe and a tilt wheel also arrive. A row for it means either undoing the toolkit's folding (changing `InputOptions::horizontal_scroll_modifier`, which also changes how shift+wheel scrolls every panel) or adding a horizontal-scroll gesture to the event vocabulary and deciding what a real sideways swipe then does. `scroll_event`'s doc records that decision as deliberate: a sideways swipe that zooms is worse than one that does nothing.

So it is a change to the event vocabulary and to a toolkit option that reaches the whole window, in VGEOM's adapter, not a row added to a table.

What a fix looks like: `ViewportEvent::Scroll` carries the keys held and the axis; `Held` names ctrl/⌘; the table holds the scroll rows; `scroll_event` passes on what it now drops; and `tests/controls_guide.rs`'s rows then cover the wheel's modifiers as they cover a drag's.
