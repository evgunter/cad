---
id: a-point-built-on-a-frame-of-normalized-directions-does-not-fold-onto-its-wall
kind: issue
title: A point built on a cylinder or sphere whose frame is made of normalized f64 directions is not proven on it over a box on the radius: the gap's numerator does not fold where the frame's axes are unit only to rounding
status: open
opened: 2026-11-22
priority: P2
cost: M
---

Found by the review of PR 7370 (`boxfinish/radial-readers`), measured
again by the lane. The rationalized gap (`geom_core::radius_gap`)
reads a point built at `c + r·x` as the zero form over a box on `r`
because its numerator `w·w − r²` is `r²·(x·x) − r²` and `x·x` is `1`
in the tier. That holds where the chart's frame is exactly orthonormal
in the form: an axis-aligned frame, the tilt `(0.6, 0, 0.8)`,
`(0, 1, 0)` (measured: proven Zero at every box width, for the chart's
own `eval`, the sphere's, and `c + r·u_ref + h·axis`). It does not hold
on a frame made of `normalize()`d `f64` directions, which is how a
hole on a tilted face arrives: its axes are unit only to rounding, the
numerator keeps a residue of the radius's box, and the reading is that
box's enclosure, not a theorem.

**Measured** (`crates/geom-brep/src/pcurve_cache/radial_reader_rows.rs`
`a_point_built_on_a_normalized_tilt_is_not_proven_on_it_over_a_box`,
which pins it; frame from `(0.1, −0.2, 1)` and `(1, 0, −0.1)`
normalized, the cylinder's `eval(0.7, 0.4)`, radius boxed about `0.3`):

| box half-width | the gap | the direct `‖w‖ − R` |
|---|---|---|
| 0.1 | undecided, `[0, 0.318]` | undecided, `[0, 0.207]` |
| 1e-3 | undecided, `[0, 2.068e-3]` | undecided, `[0, 2.061e-3]` |
| 1e-6 … 5e-10 | undecided, same to four digits | undecided |
| 2.5e-10 | Zero, unproven | Zero, unproven |

So neither spelling proves it, both enter the band at the same box
width, and the gap's enclosure is the wider one only over a wide box
(a second-order term in the box's width). The reviewer's probe
(`origin/boxfinish/review-radial-readers`, P6) found the same on the
circle carrier's `eval` and on `c + r·(axis × u_ref) + h·axis`, the
last on the exact tilt too (`axis × u_ref` of two exact literals is
not unit in the form).

**Where the cure probably lives:** ONEHOME's frame work
(`docs/DESIGN-FORK-LOG.md` rows 274 and 295: a carrier's frame held as
a rigid-motion witness), which would let a frame state its
orthonormality as a fact of the form and not of its rounding.

**The tests that say it is closed.** The pinned row above turns: a
point built on a cylinder or a sphere whose frame is a rigid motion of
normalized directions reads a proven Zero at `Sym<Interval>` over a
box on the radius.
