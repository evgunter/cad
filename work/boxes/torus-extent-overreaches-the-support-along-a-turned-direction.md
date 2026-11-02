---
id: torus-extent-overreaches-the-support-along-a-turned-direction
kind: issue
title: boxes::torus_extent reaches (R+r)|a⊥| + r|aᵢ| along a direction whose true support is R|a⊥| + r, so a face just off a torus refuses CurvedWithinReach
status: parked
opened: 2026-10-10
priority: P3
cost: M
blocked_on: [cover-geom-readings]
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

## Since PR 6336 (BOXES, 2026-11-02)

`torus_window_extent` is gone; the windowed arm is the closed form, whose full-turn reading is the exact support `R·|e⊥| + r` this row asks for. `torus_extent` survives as the windowless arm and as a meet that cannot bind on a point description (PR 6336's review, n2), so it is the only remaining overreach. (BOXES orchestrator)

## Since `cover-geom-readings` (BOXES, 2026-11-02)

The tube's exact support exists: `geom::cover::Cover::tube`, read by `Surface::cover` for a torus, reaches `c·e + R·|e⊥| + r` along a unit `e` (`cover::tests::the_tubes_support_is_r_times_the_cross_part_plus_r`). `boxes::torus_extent` itself is untouched there: it is the windowless arm of the two face box lanes, which `cover-topo-faces` deletes when `face_box` and `census::face_reach(_in)` read the face's cover. The pin this row asks for lands with that arm.
