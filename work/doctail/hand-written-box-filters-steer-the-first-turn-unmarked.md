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

- `boolean/ops.rs` `fa.bbox.overlaps(&fb.bbox)` — the idealized sweep's face-pair filter (`SweepStrategy::Idealized`), the one the `m4_pr4_*` rows run;
- `boolean/ops.rs` `edge_box(..).overlaps(region)`, `row.bbox.overlaps(&y_row.bbox)`, `face_box.overlaps(ball_box)`;
- `boolean/nary.rs` `hulls[p].overlaps(&hulls[q])`;
- `boolean/reduce.rs` `boxed.overlaps(other_box)`;
- `boolean/glue.rs` `ha.overlaps(&hb)`;
- `boolean/mod.rs` `curved_boxes.get(&face).is_none_or(|b| b.overlaps(&point))`;
- `boolean/carrier_touch.rs` `edge_box(y, edge, pad).overlaps(..)`;
- `pieces.rs` `a.overlaps(b)`;
- `boxes.rs` `!b.overlaps(&far)`;
- `separation.rs` `hulls[i].overlaps(&hulls[j])`, `x.hull.overlaps(&y.hull)`.

A heuristic's `k_stats::choose` is unlogged by design (its outcome may differ between the box lane and the point witness): `boolean/edge_join.rs` `edge_join_carried_far`, `geom_core::period` `period_run_behind` and `period_window_end`, `geom_brep::intersect::conic_arc`, `geom_brep::curve_pair::span` `curve_pair_window_halve`, `geom::curves::on_carrier` `curve_foot_polish`. One `unlogged` reading was seen turning before a logged choice under a probe that logged every unlogged reading (`geom::azimuth` `azimuth_near_cut_side`, ahead of the blend's `fillet3_ring_clearance` in `edit_concave_bar` and `edit_planar_carve`); it reads which side of a branch cut an angle lies on and maps it near its reference after, so it steers no enumeration, and is no instance.

## What would close it

Route each pruning filter through a marking door (a `bvh` query, or a `k_stats::mark_candidates` call at the site with the pair's indices), or show per site that what it prunes is never decided by a choice. Measure first: a probe that marks every `.overlaps(` result at the listed sites, run over the resolver, drive and range rows, says which of them steers a named turn.
