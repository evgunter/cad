---
id: a-faces-loops-are-walked-inline-outside-topo
kind: issue
title: Crates outside topo walk a face's loops inline beside Face::loop_keys
status: open
opened: 2026-11-03
priority: P2
cost: E
---

Found by the second sweep of `a-faces-loops-are-walked-inline-beside-face-loops-linked`,
which gave `topo::Face` the one public spelling of a face's loop order,
`Face::loop_keys` (`crates/topo/src/entity.rs`), and a source pin in
topo (`live::tests::a_faces_loops_are_walked_in_one_place`) that holds
`crates/topo/src` to it.

## What

The same inline walk, `core::iter::once(face.outer).chain(face.rings…)`,
is spelled outside topo at about 75 sites (sweep pattern
`once\(\(?&?[\w.()]*\.outer\b`), which the topo pin does not read.
Production ones: `crates/sweep/src/blend/reach.rs` (five),
`crates/sweep/src/blend/surgery.rs` (two), `crates/sweep/src/blend/admit.rs`,
`crates/sweep/src/blend/consume.rs`, `crates/sweep/src/blend/battery.rs`,
`crates/sweep/src/swept.rs`, `crates/mesh/src/memo.rs`,
`crates/mesh/src/planar.rs`, `crates/editor-core/src/coincide/replay.rs`,
`crates/editor-core/src/eval/measure.rs`,
`crates/editor-core/src/names/emit.rs`,
`crates/editor-core/src/names/plane_pieces.rs`, and `demos/tour/src/skinned.rs`;
the rest are tests and test fixtures (sweep's above all).

## Direction

Each reads `face.loop_keys()`. The resolving walk,
`Body::face_loops_linked`, is `pub(crate)`, so a crate outside topo
resolves each loop through a public door of its own; whether topo
should export a resolving walk too is the question for the lane that
takes this, beside `face-to-surface-walks-outside-topo-predate-body-face-surface`.
A pin like topo's, over every crate root, is the cheap gate.
