---
id: torus-extent-overreaches-the-support-along-a-turned-direction
kind: issue
title: boxes::torus_extent reaches (R+r)|a⊥| + r|aᵢ| along a direction whose true support is R|a⊥| + r, so a face just off a torus refuses CurvedWithinReach
status: open
opened: 2026-10-10
priority: P3
cost: M
---



Found in the review of PR 4550 (CONTACTHOLD, `contacthold/census-backstop`),
which made the census backstop's arm 1 clear along directions that turn
with the pair (`census::backstop_apart`, over `boolean::separating::apart`).
**Measured** by the reviewer's torus rows in
`crates/sweep/tests/tube_rim_touch_rows.rs`
(`a_turned_face_clear_of_a_torus_passes_tier_3_prime`, torus `R = 2`,
`r = 1`, axis `y`, cube faces with normals `TORUS_NORMALS`).

## What

`boxes::torus_extent` bounds a whole torus along frame row `i` by
`(R + r)·|a⊥| + r·|aᵢ|`, where `a⊥` is the part of the row the axis
leaves free (`perp_room`) and `aᵢ` the axis's component. The torus's true
support along a unit direction `e` is `R·|e⊥| + r`, which is smaller
whenever `e` is not along the axis. In an aimed frame
(`BoxFrame::aimed`) the overreach is up to about `r·(|e⊥| + |eₐ| − 1)`,
a fair fraction of `r`.

So a cube face 1e-4 off the torus along its own normal still refuses
`CensusUndecidable { CurvedWithinReach }`; at 0.5 off it clears. The
same width costs the boolean's narrow phase candidates.

## The shape to give

Read the support exactly: `c·e ± (R·|e⊥| + r)` for the whole torus,
where it is cheaper and tighter than the chart window
(`torus_window_extent`, which `face_reach_in` meets with this extent).
Pin a turned face 1e-3 off a torus clearing tier 3′, beside the
existing touch/cut refusals.
