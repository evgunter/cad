---
id: profile-embed-lift-has-two-homes-anchor-and-loft
kind: issue
title: Profile<f64> -> Profile<T> is written twice, editor-core's anchor::embed_profile and sweep's loft::end_profile, and the home is a lift on the profile types
status: open
opened: 2026-09-08
refs: [2139, D385]
---

(EVAL orchestrator) From EVAL-1's style review (PR 2139, S1/S4/S8).
`crates/editor-core/src/eval/anchor.rs` `embed_profile` and
`crates/sweep/src/loft.rs:225`–`:245` `end_profile` are the same
function — `Profile<f64> → Profile<T>`: per loop, per vertex `pos` +
`bulge` through `from_f64`, `with_tangent_joints(..to_vec())`,
`Profile::new(plane, loops)` — in two crates. The home is a lift on
the profile types themselves (`ProfileVertex`/`ProfileLoop`/`Profile`,
the `map_scalar` door `work/tcost/D385.md` names for the test crates),
in `crates/profile`. Two placement facts ride along: `embed_profile`
sits in a module whose doc is "program-anchored profile naming" and
is neither, and it is the file's one `pub` (re-exported at
`eval/mod.rs`) with one caller (`wire.rs`); the door's arrival is the
moment it leaves. Where else to look: `rg 'ProfileVertex::new\(' crates/*/src`.

`crates/profile` is S-BOOL's by the paths lattice and not by its
charter, and the consumers are EVAL's and BLEND's; the owner is
undecided, which is what `work/issues/` is for. Citations accurate at
`bd2fe4289`.
