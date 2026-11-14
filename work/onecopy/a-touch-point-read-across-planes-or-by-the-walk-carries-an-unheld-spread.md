---
id: a-touch-point-read-across-planes-or-by-the-walk-carries-an-unheld-spread
kind: issue
title: A touch point read across planes or by the walk carries an unheld spread
status: open
opened: 2026-11-10
priority: P1
---

Found by the review of PR 6768 (`onecopy/touch-point-spread`), which holds the **coplanar** touches to the band (`work/onecopy/touch-point-spread.md`). Two more paths report a touch's one point, and nothing holds how far two readings of it can lie apart.

- **Across planes**: `crates/geom-brep/src/curve_pair/pose.rs` (~`:1450`, `:1466`), a `Pose::Tangent` placed by `Polish::closest`.
  - The polish's certified reach `m₋ ≈ 2(σ_w + e)/(s + √D)` (module docs, "The gap's lower bound") places every pair within the band's escalation `e` of each other.
  - Near tangency `s` and `D` shrink, and `m₋` grows to about `√e` scale (about `10⁻⁵` m at `e = 10⁻⁸`), far past the band. The point is reported at `(p, q)` all the same.
- **The walk**: `crates/geom-brep/src/curve_pair/walk.rs` (`floor`, `:349`). Its polished spline pairs near `geom_core::bisect::tangency_floor` (of `1/κ`) report one point of a stretch the floor stops subdividing, with no spread carried.

Readers that match the point by distance against the band (census pass 5′ `Declared::confine`, the boolean's curved pierce `held_on`) rest on it, as they did on the coplanar touches.

**Measurement method** (PR 6768's): a temporary log at each site of the spread the path can certify (here `m₋`, and the walk's floor stretch), beside the band's zero, over geom-brep, topo, sweep and editor-core's `ci` profile at default ε and 1e-12, counted per test. Then decide what escalates, and pin a case that reaches it inside the admission rule.

**Done when**: each path computes its point's spread and holds it to the band as `pose::spread_held` does (`curve_pair_pose_touch_spread`, or a row of its own), escalating in the gap, and pins show both a held and an escalating case at an admitted scale.

Not in scope: whether one point stands for the in-band arc (PR 6570, layer 2).
