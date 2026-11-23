---
id: hand-written-box-filters-steer-the-first-turn-unmarked
kind: issue
title: Box-overlap filters written at their call sites, and the heuristic choose sites, steer what a boolean or census decides next without a candidate mark, so the first-turn reading can still meet two different candidates at one place in two runs' logs
status: open
opened: 2026-11-18
priority: P2
cost: M
refs: [verdict-log-tells-no-validation-reading-from-a-building-decision]
---

Filed by the DOCTAIL first-turn lane, from `verdict-log-tells-no-validation-reading-from-a-building-decision`.

N5's flip is the first choice two runs of a node take differently along one recorded path (`editor_core::resolve::vdiff`, `first_turn`). The path is one only if no unlogged branch steers what is enumerated in between. Every `bvh` query now marks the candidates it hands on (`geom_core::k_stats::mark_candidates`, from `bvh::Bvh::overlapping`, `within` and `ray`), and the walk reads the marks beside the choices.

The measurement that ruled the marks in: before them, `m4_pr4_resolve`'s slid union (`slide_union`, overlapping to disjoint) read `plane_eq_parallel` `Zero → Positive` at the union's first choice. No slide turns a plane, so the two runs decided two different face pairs there. With the marks, the runs part at the union's first box query (candidate 0 against none) and name no flip.

## What still steers unmarked

A box-overlap test written at its call site prunes as a `bvh` query does, and marks nothing. The sites in `topo` (grep `\.overlaps(` outside tests and `_rows.rs`, at the merge base of the first-turn PR):

Three are now marked, on the first-turn PR's review (PR 7114), each with `k_stats::mark_kept`: `boolean/ops.rs` `walk_pairs`' `fa.bbox.overlaps(&fb.bbox)` (the production section certificate's face-pair filter, which both sweeps run), `boolean/nary.rs` `judged_pairs`' hull test, and the blend reach's cell prune `sweep::blend::reach::sure`, which is no box test but steers how many clearance readings are taken (it parted `edit_strip_mouth_cups`' row). Still unmarked:

- `boolean/ops.rs` `face_boundary_meets`' `edge_box(..).overlaps(region)`, `sphere_extent_scan`'s `row.bbox.overlaps(&y_row.bbox)`, `ball_may_reach`'s `face_box.overlaps(ball_box)`;
- `boolean/reduce.rs` `first_unsupported_pair`'s `boxed.overlaps(other_box)`;
- `boolean/glue.rs` `section_door`'s `ha.overlaps(&hb)`;
- `boolean/mod.rs` `boolean_reduce_declared_strategy`'s `curved_boxes.get(&face).is_none_or(|b| b.overlaps(&point))`;
- `boolean/carrier_touch.rs` `edge_clear_of_ball`'s `edge_box(y, edge, pad).overlaps(..)`;
- `pieces.rs` `Screen::may_meet`'s `a.overlaps(b)`;
- `separation.rs` `hulls[i].overlaps(&hulls[j])`, `x.hull.overlaps(&y.hull)`.

**Unlogged thresholds, the second sweep** (`rg -n 'escalate\(\)' crates/sweep/src/`, on the delta review of PR 7114). Every branch on `band.escalate()` in the blend now goes through one marked home in `sweep::blend::reach`:
- `sure` (`x.lo() >= escalate`), which `Least::read`'s cell test and `metered`'s `certified` used to re-spell inline. `certified` gates the sheet reading (`sheet()` → `sheet_clip` → `face_nearest` → `face_placement`), which records decisions;
- `sure_below` (`x.hi() < -escalate`), the gate on a boundary vertex's `face_clearance_margin` in the face's vacancy reading;
- `fine` (`width <= escalate`), the five small-cell, small-rectangle and small-piece stops;
- the edge refinement's `8·r <= -hi(inner)` settled test, marked where it is read.

The surgery's value classifiers call `reach::sure`. The remaining `escalate()` hits are values rather than branches (`Least::done`, the missed-box reading), `ClassifiedMargin`'s `Display`, and test code (`revolve/turn_rows.rs`, `swept.rs`'s `in_band`, `reach.rs`'s rows). Outside the blend, the same grep over `topo/src` and `geom-brep/src` (an `escalate()` threshold beside `lo() >=`, `hi() <` or `<=`) finds six hits. One is production and steers: `geom_brep::curve_face::spline_face`'s `norm(g).lo() <= band.escalate()`, an end off its rim reading no sector, unmarked. The rest are a `debug_assert!` (`pair_section/cylinders.rs`) and test code (`meet_rows.rs`, `spiric_arc/fuzz.rs`, `containment.rs`'s test module, `ellipse_rows.rs`). Thresholds on `Band::zero()` and thresholds spelled through a local binding were not swept.

A review probe that marked all of these and the `choose` sites (PR 7114, reviewer A's `PROBE7114_MARK`) moved no gate row beyond the one `reach::sure` parts: none of them steers a named turn in today's resolver, drive and range rows. That is evidence for these rows, not a proof for every document.

A heuristic's `k_stats::choose` is unlogged by design (its outcome may differ between the box lane and the point witness): `boolean/edge_join.rs` `edge_join_carried_far`, `geom_core::period` `period_run_behind` and `period_window_end`, `geom_brep::intersect::conic_arc`, `geom_brep::curve_pair::span` `curve_pair_window_halve`, `geom::curves::on_carrier` `curve_foot_polish`. One `unlogged` reading was seen turning before a logged choice under a probe that logged every unlogged reading (`geom::azimuth` `azimuth_near_cut_side`, ahead of the blend's `fillet3_ring_clearance` in `edit_concave_bar` and `edit_planar_carve`); it reads which side of a branch cut an angle lies on and maps it near its reference after, so it steers no enumeration, and is no instance.

**A consequence for the parting rung** (`Diagnosis::Parted`, PR 7149): an unmarked filter that skips a whole query in one run and not the other shifts every mark after it, so the two runs' first differing marks can belong to two different queries, or to one query asked about two different things. Each mark names the call that asked it (`k_stats::QuerySite`/`TestSite`), so two different queries part as `Parting::Step` and name nothing. A skipped call of the same query, though, reads as a `Parting::Mark` at that query, about another candidate. A probe sketch: wrap one listed filter (e.g. `separation.rs`'s `x.hull.overlaps(&y.hull)`) in `k_stats::mark_outcome` behind a temporary toggle, run the resolver rows and the slow `edit_*` rows with and without it, and diff each `Parted`'s site and step.

## What would close it

Route each pruning filter through a marking door (a `bvh` query, or a `k_stats::mark_candidates` call at the site with the pair's indices), or show per site that what it prunes is never decided by a choice. Measure first, as the review probe did: mark every listed site, run the resolver, drive and range rows and the slow `edit_*` rows, and read which part a named turn.
