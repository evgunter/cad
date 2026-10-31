---
id: early-round-decisions-past-convergence-the-extent-gate-and-the-meter-lever
kind: issue
title: The face-extent gate and the meter's lever guard decide at a round a later one could answer
status: open
opened: 2026-10-22
priority: P3
cost: E
---


## What

Two decisions in the face lanes of `crates/geom-brep/src/props/quad/`
and `spiric.rs` are taken at one round and end the schedule, where a
later round's narrower enclosure could answer. Found by the sweep of
`an-undecided-ring-at-the-first-converged-round-refuses-instead-of-refining`;
neither is measured to fire.

1. **`props_quad_face_extent`**, read on the converged enclosure in
   every lane (`cylinder_cut_face_rounds`, the rational and integral
   patch lanes, `trimmed_patch_face_rounds`, `rim_face_rounds`):
   an in-band margin escalates, and a `Zero` refuses
   `DegenerateFace`, at the first converged round. The margin is
   `area.lo()/perimeter`, and a later round raises `area.lo()`. At
   convergence the flux width is at most `3·1024·ε` times the area
   (`mean_boundary_displacement`), so a later round can move the
   margin only by about that fraction of itself: a verdict that a
   later round flips needs a margin within ~3·10³·ε relative of a band
   edge. Real but narrow; `quad::Run::ringed`'s `refinable` reading is
   the mechanism if it is taken (the `Zero` arm too, which reads a lower bound as the
   area).
2. **`displacement_len`'s lever guard** (`denom <= 0.0` →
   `DegenerateFace`), at any round: its docs argue it reaches the
   extent gate's verdict "one step earlier, on the same evidence", but
   the extent gate reads only converged enclosures, and the guard fires
   on a round whose symmetric pads cancel the area midpoint (the
   documented extreme-weight bilinear square, pad ≈ 1.7e20 against
   1 m²), where a later round's pads are smaller. Whether a later
   round answers on that fixture is unmeasured.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Both halves still hold, at new homes.

1. The extent gate is one function now, `run::face_extent`
   (`crates/geom-brep/src/props/quad/run.rs:210`), since PR 5815.
   `Run::ask` (`:323`) asks it once, on the round the caller accepts,
   and a refusal there ends the call. "Converged" now reads "accepted
   by the caller's predicate" (PRs 5589 and 5628).
2. `displacement_len` is now `mean_boundary_displacement` (`run.rs:493`).
   Its lever guard (`denom <= 0.0`, `:508`) is unchanged. It is read
   at EVERY round by topo's resolution comparison
   (`crates/topo/src/props/quad_lane.rs:513`, `?` on its error), so a
   `DegenerateFace` from a cancelled lever still ends a sizing walk at
   the round it fires.
