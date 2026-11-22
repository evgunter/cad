---
id: main-red-seven-sweep-and-topo-rows-red-at-1e-6-since-11-17
kind: issue
title: Main red (ε = 1e-6): seven sweep and topo rows no row held, red since between the 11-17 and 11-20 nightlies (TubeStraddles near the node ×3, parallel_cylinder_join's tipped rod ×2, review_curved ×2)
status: closed
opened: 2026-11-21
priority: P1
cost: M
closed: 2026-11-22
pr: 7383
branch: green/eps-1e-6-sweep
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

## 2026-11-22 — measured (U6, PR 7383)

Every row run on hosted CI at ε = 1e-6, in a child process the gate's
default step spawns per row (a throwaway probe test per bisect branch;
the gate's own 1e-6 step skips the slow set, where all eight rows sit).
All eight reproduce on main `12a43a7e68` with the nightly's messages.

| family | green at | red at | cause | verdict | green by |
|---|---|---|---|---|---|
| TubeStraddles near the node (3 rows) | `fa4e9ba921` (7052^1) | `6c2430e888` (7052) | PR 7052, the periodic fit | the sliver pose `(0.2, 0.699)` refuses its tube, `boxes: 437`, as SECTARMS' sliver row measures; a refusal, not a wrong answer | pinned at 1e-6 on `work/sectarms/a-sliver-loop-that-carried-as-a-clamped-fit-refuses-its-tube-as-a-periodic-one-at-a-coarse-band.md` (evidence added there) |
| tipped rod (2 rows) | `e7d1a14b11` (7114; also 7052, 7078, 7080; 7112^1 is `f7f9c07ada`, the three merges between touch no geometry crate) | `7ad94ddffb` (7112) | PR 7112, ONEHOME's span foot | tips past the band now build bodies at their volume (certified, inside their pads) where they took a decided door; pads to 2.6e-3, r 0.2 bodies fail tier 3′; no wrong volume | pinned at 1e-6 on the new `work/onehome/main-red-a-rod-tipped-past-the-band-builds-at-1e-6-since-the-span-foot.md` (P1) |
| review_curved (2 rows) | `7542ba04cd` (7208^1) | main (only 7208 and the code-free 7259 between) | PR 7208, ONEHOME's slant reading | 26 runs, bodies at their volumes whose tier 3′ census escalates on `curve_pair_span_past_reach_straddle`, crossings within 1e-3 rad: the class ONEHOME's level-root row holds, wider at 1e-6 | the filters know the class at 1e-6; `work/onehome/a-level-root-past-a-span-on-a-shared-stretch-escalates-the-census.md` corrected (it said the rows are sound on main) |
| `offer_rows::every_sphere_offer_passes_just_below_it` | 7088 and before | `7542ba04cd` and main | (SCALAR's told-quantity type, not bisected further) | false offers | PR 7267 greens it: passes at its head `e29abc3a90` at 1e-6 (probe) and at the default (its gate); not pinned |

The rim-ball row (`a_ball_across_a_cylinder_wall_r_0_1_across_the_rim`,
U2's) was not in these runs; nothing here shares its cause as far as
these bisects reach: the sliver family is the carry of one pose, and
the rim row is a different pose and every ε.

## 2026-11-22 — fix pass (PR 7383's review)

- The topo row (`ring_path::spline_fuzz::a_path_across_a_carried_loop_reads_the_membership_parity`) is now bisected too: it passes at `fa4e9ba921` and fails at `6c2430e888` (7052) at 1e-6, `boxes: 437`.
- 7112's first parent is `f7f9c07ada`, not `e7d1a14b11` (7114). The three DOCTAIL merges between them touch no geometry crate, so the attribution stands.
- The review_curved filters now read each body's volume and name their 20 + 6 runs at 1e-6. The sliver pin is the whole six-run list. The tipped rods' parallel pose runs ahead of the pin.
- On `827f317373`, all seven pinned or filtered rows pass at 1e-6 (hosted probe), and `gate ok` passes.
- Filed `review-curved-accepts-an-invariant-refusal-at-any-pose` (P2, here) for the pre-existing srec clause.
