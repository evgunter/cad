---
id: the-reflex-probes-run-b-minus-a-under-declarations-keyed-for-a-b
kind: issue
title: The reflex probes run b ∖ a under declarations keyed for (a, b), so every declared S_ba pose refuses at the declaration door
status: open
opened: 2026-10-06
priority: P3
cost: E
---


Found by ZIP's `zip-reflex` lane (branch `zip/rest-admission`),
measured on main at `3f1e3b0d03`, release build.

## What

`crates/sweep/tests/common/differential.rs` `reflex_pose` builds `d`
with `flush_declarations(&a, &b, tol)` (its doc: "for the `(a, b)`
order"), and `reflex_run` passes that same `d` to every op, including
`"S_ba"` = `topo::subtract_with(&p.b, &p.a, &p.d, tol)`. Where the pose
has a flush pair, the declaration names an A face in `b`'s arena, and
the op refuses at the declaration door before anything is measured.

## Measured

- `join1_r1_probes` `join1_r1_reflex_battery`: 192 of 288 `S_ba` runs
  refuse, `InvalidDeclaration { operand: A, what: "declared face key
  does not resolve" }`, `ContactContradicted` or
  `ContinuationContradicted` (144, 24, 24). The other 96 are profiles
  with no flush pair, which build `SOUND`.
- `join_rc_probes` `rc_wide_battery`: 960 `S_ba` runs refuse the same
  three ways, all at the unturned poses (`rot = 0`), the only turn where
  `b`'s walls are flush.

So the batteries' "no wrong body" reading says nothing about `b ∖ a`
at a flush-declared pose. The repair is `flush_declarations(&b, &a, tol)`
for that op (a second field on `ReflexPose`, or the op building its own).
