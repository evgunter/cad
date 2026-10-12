---
id: uncertified-door-limb-3-ask-pass-has-no-fixture
kind: issue
title: geom-brep: the uncertified plane x NURBS door's pass on refinement's limb-3 ask has no fixture since limb 2 subdivides
status: open
opened: 2026-10-10
priority: P3
cost: M
---

Filed by `encl/hull-bound-refine` (row `hull-bound-refusals-could-refine-the-composite-before-refusing`).

## What

`ssi::trace_plane_nurbs_within` (`crates/geom-brep/src/ssi.rs`) returns `Ok` for refinement's limb-3 ask (`Limbs::Tube`), so the uncertified door refines past the point where a certifying door stops. Its fixture was `m5_pr7_ssi::the_uncertified_door_refines_past_the_limb_3_ask`: the late-bend pair 1e-3 apart at 1e-9, which refined to 176 control points where the ask would have stopped it at 173.

Limb 2 now subdivides its composite before it refuses. At 173 samples that triple passes limbs 1 and 2 and limb 3 refuses it, so the door stops there and the ask is never reached (instrumented: no ask on that fixture, nor at gaps 3e-4 or 3e-3, at ε 1e-8, or at β = 1, seed `(0.1, 0.5)`). The test is re-baselined as `the_uncertified_door_refines_until_limbs_1_and_2_pass`, which pins the 173 and that limb 3 is what refuses the returned triple. The `Limbs::Tube => Ok(fitted)` arm has no test that reaches it.

## Repair shape

Find a fixture whose refused residual is a measured one that stalls (a march across two branches), where the ask fires in the uncertified door. `a_pair_bending_late_refuses_on_limb_3_without_refining_to_the_wall` still asks at 52 samples through `plane_nurbs_ssi` (gap 1e-2, ε 1e-6), so a seed on its jumping branch is the place to start. Watch the cost: the uncertified door refines on to the step wall.
