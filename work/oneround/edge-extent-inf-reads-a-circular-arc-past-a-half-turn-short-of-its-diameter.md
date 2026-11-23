---
id: edge-extent-inf-reads-a-circular-arc-past-a-half-turn-short-of-its-diameter
kind: issue
title: edge_extent_inf reads a circular arc past a half turn short of its diameter, so sweep's link extent keeps its own schedule and its pin
status: open
opened: 2026-11-20
priority: P2
cost: M
---

Found by `oneround/lever-sites-brep` (2026-11-20), converting
`sweep::blend::battery::extent_of` to the extent home's door.

## What

`edge_extent_inf` reads a circle arc of span `Δ` as
`max(chord, r·(1 − cos(Δ/2)))` (`extent_inf` in
`crates/geom-brep/src/extent.rs`). Past a half turn the arc holds a
diametral pair, so its diameter is `2r`, and the door reads less:
`max(2·sin(Δ/2), 1 − cos(Δ/2))·r`, least at `tan(Δ/4) = 2`
(`Δ ≈ 254°`), where it is `1.6r`. It is a bound from below, so it is
sound; it is not tight, and the circle is the one kind whose arm has no
sampled pairs.

`edge_extent_sup`'s circle arm is the diameter exactly,
`2r·sin(min(|Δ|, π)/2)`, so on a circle the two bounds differ where
they need not.

## What it costs today

`sweep::blend::battery::extent_of` reads the farthest pair of its own
nine samples (`points_diameter`, pinned by
`scripts/gates/lever-doors.sh`), which on such an arc is at least
`2r·cos(Δ/32) ≥ 1.96r`. Handing the site `edge_extent_inf` was measured
over the workspace suite at ε = 1e-9 and 1e-12:

- 73 of 50 190 `f64` hits and 1 of 1 386 `Interval` hits read shorter,
  by a factor down to 0.858; every other hit read the same bits but 2
  `Interval` hits, one ulp longer (ε = 1e-9; 1e-12 is the same);
- `sweep::all review_verbs_rim_lever_probes::open_arcs_approaching_a_full_turn_do_not_collapse`
  goes red at both ε (`θ = 4.2`: lever `1.726r` against its `1.9r`
  floor). No decision count moves in any other test that reaches the
  site (same binaries and fuzz seed, the site's old and new value).

So the site keeps its own schedule and its pin, and the row
`lever-doors-pinned-sites-await-a-geometry-read` keeps it listed.

## The fix

Read a circle arc's extent from below as its diameter,
`2r·sin(min(|Δ|, π)/2)`, the form `edge_extent_sup` has. That moves
every reader of `edge_extent_inf` on a circular arc past a half turn
(certification's, the offset door's, step-import's) toward a longer
lever, by at most 25%, so it wants its own differential. Then
`extent_of` is `edge_extent_inf(carrier, t0, t1)` and its pin falls.
