---
id: a-module-named-for-its-spine-type-is-unfalsifiable
kind: issue
title: the naming rule the split ratified cannot fail for a large enough type, and closes the original finding by re-description
status: open
opened: 2026-09-06
refs: [2083]
---


Found by the style review of #2083, which ratifies the rule in
`pick-and-pickindex-are-named-against-their-contents`'s closing prose
and repeats it in `crates/viewer/src/pickcache.rs:28-31`. (#2083's fix
pass rewrote that header passage — it is `pickcache.rs:28-34` now and
names this item rather than asserting the rule, and the closure was
revised the same way. The rule itself is still what the rename was
argued from, so this item stands.)

## The rule as stated

*"A module named for the type whose inherent `impl` is its spine is
named correctly."*

## Why it cannot fail

The rule is satisfied by construction for any module built around one
type, however much that type does. `impl PickIndex` runs
`pickindex.rs:720-1605` — 885 lines, forty-odd inherent methods —
and the rule certifies the module's name no matter what those methods
decide. Applied to `session.rs` before the 1c split it would have
certified `DocSession`'s god-module; applied to `app.rs` it would have
certified that one too. A naming rule that no module can fail is not
evidence about this module's name.

## What it closes without changing

The original finding was *"it is the index plus every decision taken
over it"*, and the closing prose concedes it survives *word for word*:
`op_for`, `op_under`, the miss rule and `hovered_for`'s priority rule
are untouched. What changed is the description, not the tree. The
substance has a live structural symptom the closure does not mention:
`op_for`/`op_under` return `SessionOp`, so `pickindex` imports
`crate::session` (`pickindex.rs:81`), and that import is one leg of the
ring `pickcache → pickindex → session → pickcache` that
`crates/viewer/README.md:787-797` documents as held open on purpose. A
module that were only "the index" would not name `session` at all. The
finding is closed on the ground that a type may have methods; the thing
it named is the ring, and the ring is still there.

This is not an argument that the rename was wrong — `pick → pickcache`
is right on its own terms, and the swap argument against it is sound.
It is an argument that
`pickindex-holds-the-frames-marks-as-well-as-the-index`'s sibling
finding was closed by re-description and its residue was not given a
file.

## Also: the closing enumeration is not exhaustive

The same prose enumerates everything left in `pickindex.rs` as
*"`PickIndex`, one of its keys, its construction machinery, its errors,
its answers, the two constants its queries take
(`EDGE_PICK_RADIUS_PX`, `PickKinds`), or a private helper"*. `PickKinds`
(`pickindex.rs:155`) is a `pub enum`, not a constant — it is an input
vocabulary with two inherent methods — and the file holds three
constants, not two (`EDGE_PICK_RADIUS_PX:213`,
`OCCLUSION_SLACK_REL:1586`, `PARALLEL_REL:1596`). An enumeration used
as the warrant for a naming decision should be exhaustive and correctly
typed.

## Confidence

`likely` on the rule being unfalsifiable and on the closure being a
re-description. `sure` on the enumeration errors.
