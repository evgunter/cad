---
id: a-mate-row-does-not-say-whether-it-placed-its-child
kind: issue
title: a mate row does not say whether it placed its child or only declares
status: open
opened: 2026-09-30
priority: P1
cost: E
blocked_on: [materole-has-no-display]
---


Found by AUTH-7's sweep for evaluated values the viewer never shows.
A `Mate` node evaluates to `ValuePayload::Mate(MateRole)`
(`crates/editor-core/src/mate/solve.rs` ~:51). `Determining` means a
tree mate placed its child. `Declaring` means a non-tree mate solved
nothing and only declares contact. Nothing under `crates/viewer/src`
reads the role: `tree::reading_of` answers `None` for it, and
`matetool.rs` never names it. So a person who adds a mate and sees
nothing move cannot learn from the GUI that the mate was redundant.
Both rows read `Mate` with no badge.

The fix is the shape of AUTH-7's: one more `tree::Reading` arm,
drawn beside the row. The only open choice is the word. `MateRole`
has no `Display`, and a kernel word has to come first so the viewer
does not mint one: `work/msolve/materole-has-no-display`.
