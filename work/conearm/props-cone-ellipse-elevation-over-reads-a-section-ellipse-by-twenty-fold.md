---
id: props-cone-ellipse-elevation-over-reads-a-section-ellipse-by-twenty-fold
kind: issue
title: props_cone_ellipse_on_cone bounds a plane × cone section ellipse's elevation 9–21× above its measured miss, so a half of a scale-100 frustum split refuses tier 3 at ε = 1e-12
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [a-steep-frustum-cut-at-a-shallow-tilt-refuses-at-the-endpoint-gate-at-1e-12]
---

Found by GERM while closing
`a-steep-frustum-cut-at-a-shallow-tilt-refuses-at-the-endpoint-gate-at-1e-12`.

## Measured

A frustum of half-angle `α`, height `S = 100`, top radius `S/2`,
revolved about `y`, in a generic rigid pose, split by the plane through
`(0, S/2, 0)` whose normal stands `φ = f·(π/2 − α)` off the axis. At
ε = 1e-12 the split succeeds and both halves pass tiers 1 and 2, but
`validate_geometric` refuses a half:
`VolumeUncomputable { Face { Escalated { predicate: "props_cone_ellipse_on_cone", margin: Enclosure [0, hi] } } }`
from `require_cone_ellipse` (`crates/geom-brep/src/props/curved.rs`).

The carrier miss is the plane × cone arm's ellipse's double-double distance from
both surfaces over the arcs the face consumes
(`crates/sweep/examples/conic_past_band_probe.rs`, `ROT=1`, same pose).

| α | f | A | `hi` (the bound) | carrier miss | ratio |
|---|---|---|---|---|---|
| 1.2 | 0.5 | 237 | 1.06e-12 | 5.8e-14 | 18 |
| 1.2 | 0.7 | 346 | 1.14e-12 | 7.6e-14 | 15 |
| 1.3 | 0.3 | 253 | 1.01e-12 | 4.8e-14 | 21 |
| 1.3 | 0.5 | 306 | 1.24e-12 | 7.3e-14 | 17 |
| 1.3 | 0.7 | 449 | 1.13e-12 | 1.25e-13 | 9 |

`α = 1.2, f = 0.3` reads below the band and measures. Every row
measures at ε = 1e-9 and 1e-6, and the volumes sum to the closed form.
At `S = 1` the same cuts measure at 1e-12.

The bound is a few `u·A` per term (`u·A` is 2.6–5.0e-14 here) summed over
five terms, divided by a lower bound on the second factor, and
maximised over 128 pieces: its doc comment names the over-read (the
spread of `ρ` and `|h|` across a piece, the triangle inequality over
the five terms). Which of those carries the factor of twenty is not
measured.

## What closes it

Measure the five terms and the divisor at the arg-max piece of one row
against the true elevation there, and tighten whichever over-reads; or
show the bound is already tight to the rounding it must cover at this
extent and say so in the row's doc.

Pinned skip: `sweep/tests/reach_cone_split.rs`
`steep_frusta_in_a_generic_pose_split_at_a_shallow_tilt`, the
`(1.3, 100)` row, reads tier 3 only from ε = 1e-9.

**Note from CLEAVE (2026-10-14).** Since PR 5100, the split's own at-rest
gate reads tier 3, so at ε = 1e-12 the `(α 1.3, S 100, f 0.5)` split no
longer returns the halves. It refuses as
`SplitFinishError::ResultNotAtRest`, and every finding is this row's
`VolumeUncomputable`. `steep_frusta_in_a_generic_pose_split_at_a_shallow_tilt`
now pins that refusal below its `measured_from` floor, so fixing this row
turns the split's refusal into halves and reddens that arm. Lift the
floor then.

## Note from CLEAVE (2026-10-19): the operand fixture refuses too

On main's nightly for `07a57312bf` (2026-10-19) and in PR 5357's CI at
1e-12, `reach_cone_split::steep_frusta_in_a_generic_pose_split_at_a_shallow_tilt`
now panics while it builds the *operand*, before any split: "the frustum
is not a finished body: [VolumeUncomputable { … Escalated { … predicate:
Some(\"props_meridian_on_surface\") … }]", from the fixture's at-rest
gate (`crates/topo/src/lib.rs:392`, the `finished` helper). This row
read the refusal at the split's halves; the frustum itself now refuses
at that ε as well. The predicate is `props_meridian_on_surface`, not the
ellipse elevation this row names, so the two may be separate readings.
