---
id: profile-embed-lift-has-two-homes-anchor-and-loft
kind: issue
title: Profile<f64> -> Profile<T> is written twice, editor-core's anchor::embed_profile and sweep's loft::end_profile, and the home is a lift on the profile types
status: open
opened: 2026-09-08
refs: [2139, 2186, D385]
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

## Narrowed (EVAL-8, PR 2186)

`anchor::embed_profile` is retired: the pinned arm lifts the pre-pass's
VALIDATED form instead (`ValidatedProfile`'s lift door in
`crates/profile/src/validate.rs`), so the production copy is gone.
What remains of the class, at PR 2186's review head:

- `crates/sweep/src/loft.rs:225`–`:245` `end_profile` — the same
  `Profile<f64> → Profile<T>` loop, AND it then re-`validate`s the
  exact lift at `T`, which is exactly the re-decision EVAL-8 removed
  from the pinned arm (EVAL-8's correctness review); the validated
  lift door now exists to replace both halves.
- Two test copies of the raw lift: `crates/editor-core/tests/pinned_lift_validates_once.rs`
  `embed` (born in the same PR that retired the production one) and
  `crates/profile/tests/common/mod.rs` `lift`.

The home is unchanged: a `map`/lift on `Profile`/`ProfileLoop`
themselves in `crates/profile` (D385's `map_scalar`), which the
validated door would then be built over.
