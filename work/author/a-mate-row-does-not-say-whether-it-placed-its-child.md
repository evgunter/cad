---
id: a-mate-row-does-not-say-whether-it-placed-its-child
kind: issue
title: a mate row does not say whether it placed its child or only declares
status: closed
opened: 2026-09-30
priority: P1
cost: E
blocked_on: [materole-has-no-display]
branch: author/mate-row-role
closed: 2026-10-02
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

## Closed 2026-10-02 — AUTH-16, branch `author/mate-row-role`

`tree::Readout` has a `Role(MateRole)` arm, which `readout_of`
produces from `ValuePayload::Mate`. `pane::features::lines_under`
draws it under the mate's row with `MateRole`'s own `Display`, quietly,
before the row's standing note — under the row rather than beside it,
where `Readout::Unavailable`'s sentence goes: the kernel's words are a
sentence, and the row's own line does not wrap. A refused mate is a `Failed` node, so
`Refused` never reaches the row as an `Ok` value. Pinned by
`tree_badges::a_mate_row_reads_whether_it_placed_its_child` (a loop of
two shelves on two posts; every mate's row carries the solve's role,
three `Determining` and one `Declaring`) and
`pane::features::tests::a_mate_rows_role_paints_the_kernels_sentence_under_it`.
