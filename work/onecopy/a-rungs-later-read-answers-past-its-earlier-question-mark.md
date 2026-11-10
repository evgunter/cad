---
id: a-rungs-later-read-answers-past-its-earlier-question-mark
kind: issue
title: Three rungs refuse at a ? where a later read of the same rung answers the same way
status: closed
opened: 2026-11-10
priority: P3
cost: M
closed: 2026-11-10
pr: 6786
---

Found while building the `rung:` register (`scripts/gates/escalation-question-marks-rung.counts`, from `a-question-mark-inside-a-rung-is-registered-nowhere`).

**The shape.** A rung reads `x?`, answers `P` where `x` holds, and otherwise reads `y`, answering `P` where `y` holds too. Where `x` is in band and `y` decides `P`, the answer is `P` whichever way `x` really reads. The `?` refuses there anyway. The rung's refusal is kept and ranked by the ladder, so the climb answers through another rung if one answers. Where none does, it refuses where it could have answered. That is the ladder rule's search, one level down: a later read that does not need the earlier one to hold.

`path_parity`'s two pairs of segment ends (`side(from)?, side(to)?`) had this shape and are ported in the PR that landed the register. These three remain, each registered `SEARCH`:

- `boolean::sectors::crescent` (crates/topo/src/boolean/sectors.rs:2200, :2210, :2215, :2221). The opening (`bool_sector_crescent`) reads none of the reads before it, and decided off `Positive` it answers `false` alone. So three `?`s refuse ahead of it: the first `parallel_same`, the across reading (`bool_sector_crescent_across`) and `coincident_run`. The first `parallel_same` also refuses ahead of the across reading, which is computed from `da` whether or not the bounds are parallel, and decided off `Negative` answers `false` either way. Only the second `parallel_same`, read under `coincident_run &&`, is a premise (PR 6763's review, F1). The port reads the independent readings before any refuses. Mind the telemetry: they would be recorded on pairs they are not recorded on today.
- `boolean::insert::held_cut` (crates/topo/src/boolean/insert.rs:1478, :1481). `nested(..)?` answers `false`, and so does `on_arc`'s `[Some(false), _] | [_, Some(false)]`. Where `nested` is in band and `on_arc` places the cut off the run, the cut is not held either way.
- `ring_path::path_parity` (crates/topo/src/ring_path.rs:1274, :1278). On a conic arc off arrival, where `plane_roots` is in band, `misses_in_plane`'s first arm (`split_ring_path_coplanar_apart`) holds whatever the planes, by its own doc. A decided miss there answers `Some(flip)` past the refusal. Its second arm (`split_ring_path_same_circle`) needs `plane_roots`' `Sign::Zero` and does not answer past it. So the port reads the apart arm alone, not `misses_in_plane` whole.

Each port is a refusal that becomes an answer, so each wants a row that reds against today's order: an `x` in the gray band with a `y` decided. `a_segment_ending_on_the_crest_cylinder_answers_past_an_end_in_band` in `ring_path::tests` is the pattern.

## Done when

Each of the three reads its later conjunct before the earlier refuses, under a row that reds against the old order, and its `rung:` entry is lowered and dispositioned `PREMISE`. Or a reviewer rules that a site's later read does not answer alone, and its entry says why.

## Outcome

Each of the five `SEARCH` `?`s now lets the later read answer past it. The `rung:` register drops in step:
- `crescent`: 5 → 0, so its entry is retired;
- `held_cut`: 4 → 3;
- `path_parity`: 6 → 5.

The ports read each conjunct as a rung of `climb(Answering::Any, …)` (PR 6786's review, M1). A reading decided not to hold answers. One in band is kept by its standing (`BooleanError::refusal`) and refuses only where none decides against. A fault aborts at once and is read past by nothing. The first draft's hand-rolled helpers (`all_hold`, `past_nesting`) answered a decided false past a fault too, so they are gone.
- **`crescent`** reads its conjuncts through `every_holds`, a `climb` over them, lazily and in the old order: parallel bounds, the across reading, the run's clause, the opening. The run's clause is a disjunction, off a run or the bounds parallel along it, read through `any_holds`, a `climb` that answers on a reading that holds. So `parallel_same(.., l, ..)` is now also read where `coincident_run` is in band. Every added read sits on a path that used to refuse.
- **`held_cut`** reads a cut through `unheld_walk`, a two-rung `climb`: the strut's nesting, then the walk (`on_arc`) placing an end off the run. Either answers "not held". The walk is read only where the nesting does not answer, and its reading is kept beside the climb for `tied_held`.
- **`path_parity`**: where `plane_roots` is in band for a conic arc off arrival, `coplanar_apart` (split out of `misses_in_plane`'s first arm) answers the pair's miss.

**Pinned**, each red against the old order:
- `boolean::sectors::tests::a_conjunction_answers_past_a_margin_and_never_past_a_fault`: `every_holds` and `any_holds`. Red against the hand-rolled helpers, which answered past a fault (measured by making every fault stand as a margin).
- `boolean::insert::tests::a_walk_off_the_run_answers_past_a_nesting_in_band_never_past_a_fault`: `unheld_walk`, red the same way.
- `boolean::sectors::tests::a_crescent_answers_past_bounds_parallel_in_band`: hand-built plane sectors through `crescent`.
- `ring_path::tests::a_circle_apart_from_a_conic_misses_it_past_a_plane_meeting_in_band`: through `path_parity`.

The four port rows were measured red with all three sites reverted to the first-refusal order (0/4).

**`held_cut`'s wiring outside the climb**, by mutant over the topo and sweep `ci` sets:
- A non-strut reading the nesting reds 3 topo rows (`union_flush_onto_edge_contact`) and 3 sweep rows.
- The nesting forced to `false` reds 3 topo rows and `rest_nested_strut::a_pinch_apex_meeting_one_vertex_builds_in_either_order`.
- The half-edge short-circuit answering `false` reds nothing. It is reached 312 times across 12 rows, and the walk places the cut off the run at every one, so the two answers never differ. A strut's run lies in its own physical sector, which places such a cut off it. The comment at the site says so.

**Reach**: no suite reaches a ported path. A probe on each new answer-past branch logged nothing over the topo, sweep and editor-core `ci` sets at default ε, 1e-12 and 1e-6.

**Folded in from PR 6757's review.**
- **ST1.** `census.rs`'s `TouchUnread::unrest` and `TouchUnread::scan` each matched on `refusal()`. Both now carry one reading of the touch's standing, `TouchUnread::backstop_standing` (`Kept::{Waits, Aborts}`).
- **ST2.** A site read on a declaration's word (`site_scan`, non-strict) used to drop an `Unrest::Aborts`, taking a fault on the record's word. It now aborts on a fault: a corrupt body, readings that disagree, a normal's fault or a contradicted edge tangent.
  - It still takes an unreadable curved germ (`TouchUnread::curved_unread`) on the declaration's word: a designed curved contact is what one declares.
  - A probe of the declared aborts over the topo, sweep and editor-core `ci` sets at all three ε met exactly two, both `Touch(Curved(Unreadable))`. They are the sweep rows `m5_pr9_boss_union::a_touching_curved_assembly_validates_declared_and_refuses_undeclared` and `tube_rim_touch_rows::a_tube_rim_vertex_touching_a_turned_face_unions_past_tier_3_prime`, and both went red when every declared abort was honoured. So the narrowed rule moves no suite verdict.
  - Pinned by `census::blocked_rows::a_declared_site_still_aborts_on_a_fault`.
