---
id: split-bisector-side-in-band-off-a-corner-at-millimetre-scale
kind: issue
title: A cut 1e-7 off a corner of a millimetre body refuses SliverSector at split_bisector_side
status: open
opened: 2026-10-06
priority: P2
cost: M
---


Found by `reach/split-gate-sphere-azimuth` (`split-gate-zone-ignores-the-azimuth-window`),
once the gate let the cut through. Before that the same cut refused at
the gate, so this refusal was hidden behind it.

## What

The capped cylinder of `crates/sweep/tests/reach_split_gate_azimuth.rs`
(unit cylinder `y ∈ [0, 1]` under the cap of the sphere of radius 5/4
about `(0, 1/4)`, revolved about `y` through 2.0 rad) at scale `1e-3`,
cut by the plane `n·p = d` with
`n = (0.0896, −0.9422, 0.3228)` and `d = −0.85251·s`, in all three
poses of that suite, at ε 1e-9:

`SplitReduceError::SliverSector { vertex: 8v1, face: 2v1 }`, predicate
`split_bisector_side`, margin `−9.51e-9` in the band `(1e-9, 1e-8)`,
raised in `splitting/neighborhood.rs` `classify_neighborhood` (the
wide-sector bisector arm).

The cut is `10⁻⁴·s = 10⁻⁷` beyond the cap's greatest support along `n`,
which the cap reaches at its rim vertex `(1, 1)·s` at azimuth 0, so it
clears the sphere face and crosses the cylinder `1.1·10⁻⁷` below that
corner. The same cut at scales 1 and `1e3` splits, and both halves
match the slice integral at ε 1e-9. At ε 1e-6 the same cut refuses
the same way at scale 1 (margin `−9.51e-6`): what matters is the body's
size in ε (`10⁶`), with the cut `10²` ε off the corner.

Unmeasured: which vertex `8v1` is (the corner, or the crossing vertex
the split inserted), and so whether the bisector decision should have
been asked there at all. The margin is `bisector·n` levered by the
sector's arm, and an arm of a sub-micrometre chord would put an
ordinary angle in band at this scale; that is the hypothesis to measure
first.

The row stands this refusal down loudly, with a floor of one per pose
(`every_cut_clear_of_a_partial_turn_sphere_face_splits`).


**A second witness** (REACH review of PR 4123, the reviewer's split
probe, in `probes/` on `analysis/reach-review/4123`): a two-rim sphere zone through
`Θ = 1.5` rad, cut 1e-3 clear of the face by the plane
`n = (−0.320, 0.0435, 0.946)`, `d = −0.2407`. It refuses
`SliverSector` at `split_bisector_side` on vertex `8v1` in both poses,
at `s = 1e-3` under ε 1e-9 (margin 5.44e-9) and at `s = 1` under
ε 1e-6 (margin 5.44e-6). This is the same `10⁶` body-to-ε ratio as the
cap witness above, on a body with no cap.
