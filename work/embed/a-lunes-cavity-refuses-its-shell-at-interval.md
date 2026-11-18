---
id: a-lunes-cavity-refuses-its-shell-at-interval
kind: issue
title: A lune's cavity refuses its shell at Interval, at Departure, where f64 holds
status: open
opened: 2026-11-18
priority: P2
cost: M
---


## Finding

From EMBED's lane `embed/lune` (2026-11-18), making the two-arc lune's
fixture generic (`two_arc_lune::<T>` in
`crates/sweep/tests/self_intersection_proves_injectivity.rs`).

At `f64` the two-arc lune hollowed by 0.05 holds (2,844 cells
discharged), its cavity's sphere halves read beside its caps' circles in
the caps' poses. At `Interval` the same build never reaches the
certificate as a row: `topo::shell` refuses it at its own cavity check,
`CavityUncertified { Uncertified { a: 1v1, b: 3v1, shared: Edge(3v1),
arm: Departure } }`, pinned by
`a_two_arc_lunes_hollow_is_refused_by_its_shell_at_interval`. The solid
lune holds at both scalars (`a_two_arc_lune_holds_at_both_scalars`), so
the hollow's rows read `f64` alone, against the suite's rule that every
row reads both scalars in one class.

**Where it stops, as far as the lane traced it:** a cap's corner of its
line and its arc is read through `Reading::Conic`, whose `one_conic`
(`crates/topo/src/injective.rs:4439`) compares carriers by `one_value`,
and "at `Interval` this admits only carriers stored as points"
(`injective.rs:4437`). The moved clone's carriers are brackets at
`Interval`, so the corner is unread and the cell `Structural`. Not
re-traced past the pose extent's landing; the first step is to trace the
refused cell at `Interval` and confirm the reading it lacks.

**Cost, not soundness:** a refusal. A lune's hollow at the certifying
scalar the shell gate and the document door ask at refuses where `f64`
holds.
