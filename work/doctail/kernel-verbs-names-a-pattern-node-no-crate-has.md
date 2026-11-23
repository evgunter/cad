---
id: kernel-verbs-names-a-pattern-node-no-crate-has
kind: issue
title: docs/KERNEL-VERBS.md names Node::Pattern and PlacedUnion as shipped, and no crate has either
status: open
opened: 2026-11-22
priority: P4
cost: E
---


Filed by the cut of `work/repeat/a-loop-cannot-repeat-inside-a-profile.md`.
`docs/KERNEL-VERBS.md` has no owning program. DOCTAIL holds the
document model's tail, and body repetition is the document's index, so
the row is filed here.

## What

`docs/KERNEL-VERBS.md`'s **patterns (linear/circular) + mirror** row
(line 65) says "the patterns half is SHIPPED (`Node::Pattern`
linear/circular/explicit + `PlacedUnion`, `Instance(i)` naming …)". The
**hole features** row (line 64) says "`PlacedUnion` × `Subtract` spells
a counterbore today". Neither type exists in any crate. The only
`PlacedUnion` left is a string in
`crates/editor-core/tests/unreadable_by_this_build.rs`, which pins that
an old document predating the word is unreadable. Body repetition is
`docs/DESIGN.md` `[repetition]`: an index `k = index(N)` and one
placement reading it (`crates/pncad/src/pattern.rs`; the viewer's
`PatternTool`). Since PR 7263, a profile also repeats with `each k`.

## Fix

Re-word both rows to say what exists. The fix is written here.
