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

The ports:
- **`crescent`** reads its four conjuncts through `all_hold`, lazily and in the old order: parallel bounds, the across reading, the run's clause, then the opening. The first conjunct decided not to hold answers `false`. The first refusal stands only where none decides against.
  - The run's clause holds off a run, or where the bounds stay parallel along it. So `parallel_same(.., l, ..)` is now read where `coincident_run` is in band; before, it was read only under a run.
  - Every added read sits on a path that used to refuse. Paths that decided read what they read before.
- **`held_cut`** reads a cut through `past_nesting`. A nesting in band reads the walk (`on_arc`) and answers `false` where the walk places an end off the run. A nesting that holds still answers without reading the walk.
- **`path_parity`**: where `plane_roots` is in band for a conic arc off arrival, `coplanar_apart` (split out of `misses_in_plane`'s first arm) answers the pair's miss.

**Pinned**, each red against the old order (measured with all three reverted at once: 0/4 pass):
- `boolean::sectors::tests::a_conjunction_answers_false_past_an_earlier_refusal`: `all_hold`'s order and laziness.
- `boolean::sectors::tests::a_crescent_answers_past_bounds_parallel_in_band`: a hand-built pair of plane sectors through `crescent` itself.
- `boolean::insert::tests::a_walk_off_the_run_answers_past_a_nesting_in_band`: `past_nesting`'s table. `held_cut`'s own wiring is held by the `rung:` register, where a `nested(..)?` would raise the count.
- `ring_path::tests::a_circle_apart_from_a_conic_misses_it_past_a_plane_meeting_in_band`: a unit circle rising to a conic's plane within the band, against a conic decided apart, through `path_parity`.

**Reach**: no suite reaches a ported path. A probe on each new answer-past branch logged nothing over the topo, sweep and editor-core `ci` sets at default ε, 1e-12 and 1e-6.

**Folded in from PR 6757's review.**
- **ST1.** `census.rs`'s `TouchUnread::unrest` and `TouchUnread::scan` each matched on `refusal()`. Both now carry one reading of the touch's standing, `TouchUnread::backstop_standing` (`Kept::{Waits, Aborts}`).
- **ST2.** A site read on a declaration's word (`site_scan`, non-strict) used to drop an `Unrest::Aborts`, taking a fault on the record's word. It now aborts on a fault: a corrupt body, readings that disagree, a normal's fault or a contradicted edge tangent.
  - It still takes an unreadable curved germ (`TouchUnread::curved_unread`) on the declaration's word: a designed curved contact is what one declares.
  - A probe of the declared aborts over the topo, sweep and editor-core `ci` sets at all three ε met exactly two, both `Touch(Curved(Unreadable))`. They are the sweep rows `m5_pr9_boss_union::a_touching_curved_assembly_validates_declared_and_refuses_undeclared` and `tube_rim_touch_rows::a_tube_rim_vertex_touching_a_turned_face_unions_past_tier_3_prime`, and both went red when every declared abort was honoured. So the narrowed rule moves no suite verdict.
  - Pinned by `census::blocked_rows::a_declared_site_still_aborts_on_a_fault`.
