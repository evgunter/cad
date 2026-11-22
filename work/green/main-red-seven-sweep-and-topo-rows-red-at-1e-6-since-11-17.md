---
id: main-red-seven-sweep-and-topo-rows-red-at-1e-6-since-11-17
kind: issue
title: Main red (ε = 1e-6): seven sweep and topo rows no row held, red since between the 11-17 and 11-20 nightlies (TubeStraddles near the node ×3, parallel_cylinder_join's tipped rod ×2, review_curved ×2)
status: dispatched
opened: 2026-11-21
priority: P1
cost: M
---


Found by GREEN's nightly read (2026-11-22) of the scheduled nightly 44870651337 (main `f5ef8f2b`, 11-21) and of 11-20's (`8039ba06`); green on 11-17 (`cdee7461`); 11-18 and 11-19 ran no suite legs. Every row below is red at ε = 1e-6 only.

**TubeStraddles near the node** (`boxes: 437`, the count the held `sectarms/a-sliver-loop-that-carried-as-a-clamped-fit-refuses-its-tube-as-a-periodic-one-at-a-coarse-band` row's `pair_section::one_loop_keeps_its_order_near_the_node` carries, so likely its cause):
- `sweep cylinder_sphere_frame::a_ball_across_a_cylinder_wall_wider_than_the_wall` — "one loop, ball wider than the wall, A ∪ B: refused at None: GermSectionUnbuilt {… Cylinder × Sphere, why: Carry(March(TubeStraddles { Zero … band zero 1e-6 …"
- `sweep cylinder_sphere_frame::every_segment_joins_neighbouring_sites_along_its_loop` — "the matcher refused GermSectionUnbuilt … TubeStraddles"
- `topo ring_path::spline_fuzz::a_path_across_a_carried_loop_reads_the_membership_parity` — "unwrap() on Err: Refused(March(TubeStraddles { … boxes: 437 }))"

Candidates in `cdee7461..8039ba06` touching the section/march/carrier code: PR 7052 (`sectarms/periodic-loop-carrier`, the sliver row's finder), 7057, 7083, 7112, 7128.

**`parallel_cylinder_join`'s tipped rod**: `a_rod_tipped_off_parallel_over_a_long_wall_takes_a_decided_door` ("drum × rod r 0.2 at 0.5, half-length 50, tipped 2e-7 | U ab: Ok(\"a body\")") and `a_tipped_rod_refuses_alike_in_both_operand_orders` ("U ab: refuses"). Same window; also sweep-touching merges 7078, 7072, 7038, 7050, 7114.

**`review_arm_vs_reach_probes::review_curved` / `review_curved_plate`**: "20 failures" / "6 failures" ("r=0.5 c=0.5 lb=0.4 θ=0.0001: A ∪ B: OK BAD … t3p=false"; "θ=-0.0001: A ∪ B: ERR CurvedPierceEscalated {… decision: BoundaryMeeting"). Green on 11-20, red 11-21: candidate PR 7208 (`onehome/coplanar-thin`, the one merge touching `curve_pair/pose.rs` and the test file; `work/onehome/a-level-root-past-a-span…` says these rows are sound on main, now stale), or PR 7183.

Also unheld by name: `sweep offer_rows::every_sphere_offer_passes_just_below_it` (every ε, since 11-21; likely PR 7159, the cause of the topo twin on `work/scalar/main-red-offer-rows-false-offers-since-the-told-quantity-type.md`; SCALAR's open PR 7267 edits `crates/sweep/src/offer_rows.rs`).

## Owed
Bisect each family to its merge, decide whether each new answer is right, and re-bless, fix or pin against the causing program's row. Unit U6, `green/eps-1e-6-sweep`.
