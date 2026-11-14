---
id: three-path-sweeps-refuse-at-1e-12-over-station-frames-minted-at-interval
kind: issue
title: three path-sweep injectivity rows refuse at ε = 1e-12 once each station's frame is minted at Interval (about 1.4e-13 wide)
status: open
opened: 2026-11-14
priority: P1
---



Filed by the skin-turn lane (`onehome/skin-turn`), which mints each station's frame alone at the lane scalar (`crates/sweep/src/skin.rs`, `PathCarry::minted`; DESIGN-FORK-LOG row 266).

**What moved.** On main a station's frame entered `Interval` as twelve point enclosures (`RigidMotion::computed_at`, now gone). Minted at `Interval` it is about 1.4e-13 wide: flat along the path, at most 4e-13 on the 48-station ribbon and 6.5e-13 on the 33-station helix. At the default ε the five rows of `crates/sweep/tests/self_intersection_proves_injectivity.rs` that refused under a carry chained at `T` all pass. At `CAD_TOLERANCE_EPS=1e-12` three of them refuse inside `sweep_body_construction::<Interval>`, where main passes all three:

| Row (`self_intersection_proves_injectivity.rs`) | Refusal at 1e-12 |
|---|---|
| `a_closed_spine_reads_violated_top_against_bottom` | `pcurve_loop_continuity`, enclosure ±4.7e-11 |
| `a_long_helical_sweep_certifies_within_the_default_budget` | `pcurve_loop_continuity`, enclosure ±2.1e-12 |
| `the_s_duct_walls_hold_against_themselves` | `pcurve_map_residual`, hi 1.0e-12 |

Each reads the walls' pcurves against a frame now wider than the band's zero half-width allows at 1e-12. The wall is fitted through the minted stations, and the fit amplifies the frame's width.

**Open.** Whether the 1e-12 row owes these three rows a narrower station frame (the per-station floor is the tangent's derivative enclosure, about 4e-13, and `from_aim_and_reference`'s residual), or whether the consumers that read the frame owe the width. The wide-start-frame row (`a_loft_computes_its_walls_in_t.rs`) passes at 1e-12 at `Interval` and at `Sym<Interval>`.
