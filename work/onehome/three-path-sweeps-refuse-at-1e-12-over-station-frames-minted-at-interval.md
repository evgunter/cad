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

**Why they refuse.** These rows build through `crates/sweep/src/loft.rs`'s enclosure arm. Its sections are points and the placements have an `f64` channel, so the walls are the `f64` skin lifted as points (`lift_loop_walls`). What is minted at `Interval` is the placements, and the caps and pcurves read on them. On the closed spine the wall poles are 0 wide and the placements 3.17e-13 (6.7e-16 on main), as measured by PR 6949's reviewer. So each refusal is a minted placement, and the caps built on it, read against point walls: `f64` geometry selected on the channel planes meets placements wider than the 1e-12 band's zero half-width allows. The fit does not amplify anything.

**Open.** Two answers to choose between:
- The enclosure arm reads its caps and pcurves on the channel planes the walls were selected on, with the minted placements admitted beside them as an enclosure.
- The lane mints walls and caps alike at `T`, so the certificate compares like with like.

Either way the per-station floor stays: the tangent's derivative enclosure, about 4e-13, and `from_aim_and_reference`'s residual. The wide-start-frame row (`a_loft_computes_its_walls_in_t.rs`) passes at 1e-12 at `Interval` and at `Sym<Interval>`.

**The closed INLANE row's premise no longer holds.** `work/inlane/a-path-sweeps-station-turn-at-interval-widens-its-walls-past-the-band-at-the-tightest-eps.md` was closed by PR 6138 on the premise that every station's rotation enters the lane scalar as a point (`RigidMotion::computed_at`). That door is gone, and the entries are no longer points. Its reproduction (the wide-start-frame row at `CAD_TOLERANCE_EPS=5e-13`) still passes, by the PR 6949 reviewer's run, so the row stays closed. These three rows are where the premise's loss shows.

**The refusal does not name the station frame.** Each refuses as the wall certificate's own text (`Pcurve(Escalated … pcurve_loop_continuity)`, `Euler(Certification … pcurve_map_residual)`), not as a typed refusal naming the station whose frame is too wide for the band. That is the same ask as the variable-path row's (`work/onehome/a-path-held-at-a-lane-scalar-owes-a-roll-that-follows-it.md`, "Must hold"): a typed refusal naming the station frame.

**Pinned.** The three rows are pinned red at 1e-12 (`test_utils::pinned_red::row`) on these refusals, citing this row; the pins lift with its fix.
