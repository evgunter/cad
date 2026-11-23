---
id: a-glued-tangency-records-a-sampled-margin-no-symbolic-tier-reads
kind: issue
title: A glued tangency's row records the witness lane's sampled margin, not the closed-form locus residual, so the door's symbolic rungs cannot read it
status: open
opened: 2026-10-30
priority: P2
cost: M
---

Owed by the kernel per the INTENT stage 6 ruling (PR #6006, recommendation 5: "the glue's tangency margin must become the closed-form residual instead of a sample. That is owed regardless and is not promised as the door's proof"). Found while building stage 6 D (`work/intent/the-door-proves-a-constructed-tangency.md`), which proves a constructed tangency at rung one and leaves this margin as it is.

A glued tangency records `topo/src/boolean/mod.rs` `tangent_row`: a `SurfaceTangent` row at `DecisionSite::TangentWitness`, carrying the margin the witness lane sampled along the locus. Neither tangency row measured so far is shown zero by the door's replay. Both report `Unreplayed::Numeric` (not identically zero within the door's budget):
- `coincidence_door::a_boss_typed_tangent_to_a_cylinder_glues_one_unproven_tangency` pins `SurfaceTangent TangentWitness unproven/Numeric/OneConstruction`;
- the boss drawn on a frame read off a rim (`intent_s6_d_tangency::a_frame_read_off_a_rim_is_unproven`, before D's recourse applies) reported `Numeric` with `Recourse::Budget`.

Unmeasured: whether that is the sample, or the budget. A row whose margin is a sample cannot be discharged at rung three whatever the geometry.

Done when a tangency the boolean decides in a closed-form lane (cylinder × plane along a line first) records the locus residual as its margin, and the door's replay reads it: the value-typed boss's row then reports a residual the replay read whole, rather than `Numeric`.

