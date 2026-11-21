---
id: a-turns-lever-is-read-from-below-for-every-caller
kind: issue
title: A turn's lever (pcurves::decision::edge_reach, wedge_reach) is read from below for every caller, whatever the caller acts on
status: open
opened: 2026-11-21
priority: P1
cost: H
---

`crates/topo/src/pcurves/decision.rs`'s `edge_reach` (and `stored_reach` over it) and `crates/topo/src/wedge_reach.rs`'s `wedge_reach`, `edge_wedge_reach`, `WedgeReach::{reach, edge}` read a turn's lever as the farthest of the carrier's two ends and the two faces' boundary VERTICES from the span's middle (`geom_brep::points_lever_from`). Every point is on the faces, so this is a bound from below on the faces' extent, and every caller takes it.

`crates/geom-core/README.md`, "Float rounding", the lever bullet: "a verdict that claims a definite sign reads the extent from below, one that acts on its Zero reads it from above, and one that acts on every sign reads both and acts only where they agree". So which bound a caller needs depends on how it acts on its verdict, and one door cannot choose for all of them. The shape is a read of both bounds from the geometry the door holds: the points from below and the faces' covers (`crate::cover::face_cover`) from above, in the existing vocabulary (`geom_brep::LeverBounds`, `edge_reach_bounds`). Each caller takes the bound its action needs. That moves verdicts toward refusing wherever a Zero-acting or every-sign caller has been reading from below, so it wants its own differential. The face cover needs a `Band`, and neither door takes one today.

Found by `oneround/lever-sites-topo` (PR 7228), which left both doors pinned in `scripts/gates/lever-doors.counts`.

## The callers, by how each acts on its verdict

These are the production sites in `crates/topo/src`, each classified by reading what it does with each sign. `seqgen.rs` and `splitting/finish.rs`'s test module are `cfg(test)` and are left out. Two callers outside topo are not classified: `step-import/src/adopt.rs` and `sweep/src/blend/surgery.rs`.

Shared consumers:
- **MC**, `geom_brep::must_carry_over_edge` (`dihedral_wedge`, then `tangent_second_order`). The re-asks (`reask_pair`, `reask_chart`) serve every sign. `must_carry_description` serves the tangent side only and refuses Transverse, so it acts on Zero.
- **CERT**, `EdgeCurve::certify`/`recertify` (`run_checks`). Definite sign throughout: Transverse for an intersection, and definite tangent-gap pairing, second order and tube for a tangent one.
- **WV**, `edge_wedge_over_as`. Corner → Crease, Tangent → Seam/End: every sign.
- **JT**, `JointSides::lever` → `singular_turn`. `pcurve_singular_sheet` is definite (Zero refuses `NoLever`). `pcurve_singular_turn` serves on its mark too (a slit's whole turn), so it acts on every sign.

| Caller | Door | Decision | Acts on |
|---|---|---|---|
| `attach::set_face_surfaces_certifying` | `WedgeReach::reach` | CERT | definite sign |
| `attach::vouch_described_move` | `wedge_reach` | CERT | definite sign |
| `attach::reask_on`, `reask_chart_on` | `wedge_reach` | MC re-ask | every sign |
| `boolean::combine::graft_solids_impl` | `wedge_reach` | CERT (recertify) | definite sign |
| `boolean::edge_join::joinable` | `edge_wedge_reach` | `dihedral_wedge`, `tangent_second_order` | every sign, then definite |
| `boolean::join::stands_on` | `wedge_reach` | `bool_rim_face_opposed` | definite sign (Zero reads as not opposed, unrefused) |
| `boolean::route_rim` (2) | `wedge_reach` | shared-rim class | Zero (a seam is routed, Transverse refuses) |
| `boolean::ops::describe_edges` | `wedge_reach` | seam class | every sign |
| `boolean::reduce::sheets_part` (2) | `wedge_reach` | `bool_touch_sheets_part` | definite sign (Zero reads unseen, unrefused) |
| `boolean::slit::walls_inside` | `wedge_reach` | `bool_rider_wall_side` | every sign |
| `chord_join::chord_spec` | `wedge_reach` | MC `must_carry_description` | Zero |
| `edge_wedge::edge_wedge` | `edge_wedge_reach` | WV | every sign |
| `edge_wedge::edge_wedge_read` | `edge_wedge_reach` | WV | every sign (its `splitting::finish` caller serves Smooth only, a Zero use) |
| `euler::certify_edge_spec`, `certify_rebased_run` | `wedge_reach` | CERT | definite sign |
| `offset_axial::offset_charts_together_staged` (2) | `wedge_reach` | MC re-ask | every sign |
| `offset_together::offset_planes_together_staged` (2) | `wedge_reach` | MC re-ask | every sign |
| `pcurves::decision::walk_halves`, `walk_runs` | `stored_reach` | JT | every sign (turn) and definite (sheet) |
| `pcurves::lift::decided` | `stored_reach` | JT | every sign and definite |
| `pcurves::site::split_cache` (4) | `WedgeReach::reach`, `stored_reach` | JT | every sign and definite |
| `pcurves::site::site_rows` | `edge_reach` | JT | every sign and definite |
| `pcurves::site::turn_element` | `stored_reach` | JT | every sign and definite |
| `pcurves::tier3::pcurve_pass` (2) | `stored_reach` | JT | every sign and definite |
| `replace_face::plan_edge` (5) | `wedge_reach` | MC re-ask | every sign |
| `replace_face::plan_reanchors` | `wedge_reach` | MC re-ask | every sign |
| `split::split_edge_onto` | `wedge_reach` | MC `must_carry_description` | Zero |
| `splitting::finish::describe_section_boundary` | `wedge_reach` | seam class, then WV | every sign |
| `splitting::rules::edge_wedge` | `edge_wedge_reach` | `dihedral_wedge`, then material side | every sign |
| `validate::tier3_local_checks_marked` | `WedgeReach::reach` | CERT, then WV | definite, then every sign |

About 41 call sites, counted as the reading pass counted them:
- every sign: about 18;
- definite sign: 7;
- Zero: `route_rim` (2 calls), `chord_spec` and `split_edge_onto`;
- both a definite and an every-sign decision on one lever: about 13;
- none unclassified.

Only the definite-sign sites are served by the bound from below that every site reads today. The Zero sites need the bound from above, and the every-sign and mixed sites need both.

The classification was made by a reading pass, spot-checked at `stands_on` and `split_edge_onto`. Check each against the code when converting it.

