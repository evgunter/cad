---
id: cone-incidence-decides-over-lever-toward-the-needle-and-the-flat
kind: issue
title: The cone's rim-incidence decides over-lever (1/sin α, 1/cos α) and the generator fix has no pinning row
status: open
opened: 2026-10-11
priority: P3
cost: E
---


Found by PR 4696's second verifier (probes on `review/4696-verify2-01Lo` @ 306ad9c3), on head f29d6ca9d6.

1. **`require_rim_incidence` over-levers** (`crates/geom-brep/src/props/curved.rs` ~:809-820).
   - `props_rim_axis_parallel` meters `r·θ`, but the rim's true distance from the cone is `r·θ·sin α`, so it over-levers by 1/sin α toward a needle cone.
   - `props_rim_center_on_axis` meters the offset `o`, against a true distance of `o·cos α`, so it over-levers by 1/cos α toward a near-flat cone.
   - Both refuse `OffSurface` at α ≤ 1e-3 (tilt) and α ≥ π/2−1e-3 (offset) for rims lying 0.1·zero off the cone.
   - This fails closed: R2's 5616-case sweep shows no default-ε regression from it.
   - The probe `incidence_decides_meter_true_distance` is red until fixed.
2. **The generator half of 4696's lever fix has no pinning row** (`curved.rs` ~:1900, `props_meridian_generator` = `sin(β−α)`). Reverting it to the cosine difference leaves every committed cone row green. At the needle, a generator 100·escalate off the cone then passes `props_meridian_generator` and is caught only by `props_loop_closed`, under the wrong name.
3. **NOTE, pre-existing.** `props_meridian_generator` is levered by the span `t1−t0`, while a line through the apex is off the cone by up to `t1·sin(β−α)`. At v = 1001 that understates by 1001×, and closure catches it instead.

**Fix shape:**
- lever each incidence decide at its true distance from the cone;
- add rows near both α ends for all four decides;
- add a row that turns red when the generator lever is reverted.

(FLUX orchestrator)
