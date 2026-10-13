---
id: an-arc-stored-past-a-full-turn-builds-as-a-line
kind: issue
title: An arc stored with a sweep past a full turn whose quarter-tangent rounds to a sub-eps sagitta builds as a line, so arc_sweep_range never refuses it
status: open
opened: 2026-10-13
priority: P1
cost: E
---

Found by a review of #5032 (`arc-arc-secant-arm-misses-crossings-at-unequal-radii`);
off that PR's target and on `main` before it.

## The defect

`crates/profile/src/seg.rs` `build_seg` decides `segment_straightness`
first, on the sagitta `(L/2)·tan(Δθ/4)`, and reads an arc whose
sagitta is within the band as its chord, a line. Only an arc that
stays an arc reaches `check_carrier`, whose `arc_sweep_range` refuses a
sweep outside `0 < |Δθ| < 2π`. `tan(Δθ/4)` is periodic in `4π`, so a
stored sweep of `4π + δ` has the quarter-tangent of `δ`: where that
short arc's sagitta is within ε, the stored arc is read as a short line
and its range is never asked. That breaks `Arc2`'s documented
`0 < |Δθ| ≤ 2π` (`crates/geom-core/src/arc.rs` `Arc2::sweep`) at the one
door that exists to refuse it.

## Reproduction

`build_seg((1, 0), (cos Δθ, sin Δθ), Segment::Arc(Arc2 { centre: (0, 0),
radius: 1, sweep: Δθ }), Consistency::Decide, band)`, measured at
ε = 1e-9 and 1e-12 on this branch's head:

| sweep | read |
|---|---|
| `4π + 1e-7` | `Ok`, `SegKind::Line` |
| `−4π − 1e-7` | `Ok`, `SegKind::Line` |
| `2π + 1e-7` | `Err(arc_sweep_range)` |
| `1e-7` | `Ok`, `SegKind::Line` (correct: a sub-ε sagitta) |

At `2π + δ` the quarter-tangent is `−cot(δ/4)`, far from zero, so the
arc stays an arc and the range check refuses it; only the `4π`-periodic
neighbours slip through.

## Where to look

Ask the range before the straightness read, or fold `|Δθ| ≤ 2π` into
it, so a stored sweep outside the range refuses whatever its sagitta.
Whether a profile carrying such an arc then lowers the stored two-turn
arc downstream was not traced; the band is for the validator accepting
input its contract refuses.
