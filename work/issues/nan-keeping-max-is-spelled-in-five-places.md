---
id: nan-keeping-max-is-spelled-in-five-places
kind: issue
title: the NaN-propagating max/min fold is hand-spelled in several crates (geom_core::interval::max_bound / Real::max, bvh aabb pmin/pmax, geom curves/boxes pfold), each with its own comment saying it matches the others
status: open
opened: 2026-10-01
priority: P3
cost: M
---


(SSI orchestrator, 2026-10-01, from the review of PR 3694. This is a class finding.)

A fold that has to propagate a refused (NaN) operand is written by hand at
each site that needs one, because the inherent `f64::max`/`min` drop NaN
and some certification files may not name `Real`. The spellings counted:

- `geom_core::interval::max_bound` (PR 3685), the gate-legal door over
  `<f64 as Real>::max`;
- `<f64 as Real>::max`/`min` itself (`geom-core/src/real.rs`);
- `crates/bvh/src/aabb.rs` `pmin`/`pmax` (~60-75);
- `crates/geom/src/curves/boxes.rs` `pfold` (~103);
- SSI's `max_keeping_nan` (PR 3694). Its fix pass deletes it in favour of `max_bound`.

This is filed here rather than on one program's slate because the sites
belong to several programs: `bvh` and `geom` each have their own owners.
The question is whether one home serves all of them, given the
certification-doors gate's rule on naming `Real`, or whether each copy
earns its place. A related shape that a min/max sweep cannot see: NaN
manufactured by a quotient of finite operands (SSI log, PR 3646).

## A dropped NaN that cost a bound (PR 3916 review, 2026-10-03)

**Reproduced.** `Centred::of` and `Centred::of_affine`
(`crates/geom-brep/src/ssi/enclose.rs`), the `f64` screen of the plane ×
NURBS chart readings, folded their radii with `fold(0.0, f64::max)`.
An offset spanning the whole finite range overflows its radius to `+∞`,
a term that does not read it (`g` exactly zero) then forms `0·∞ = NaN`,
and `f64::max` dropped it, so the radius collapsed to the smallest
normal and the screen skipped the pair that held the maximum (1.0 read
against a true 10). Both sites now fold with `max_bound` and read a
zero factor as an exact zero; the row is
`an_unbounded_radius_screens_nothing_out`. That is one more reason the
inherent `f64::max` should not be the spelling anywhere a refusal can
arrive.

