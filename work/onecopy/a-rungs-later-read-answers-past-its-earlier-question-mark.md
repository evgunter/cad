---
id: a-rungs-later-read-answers-past-its-earlier-question-mark
kind: issue
title: Three rungs refuse at a ? where a later read of the same rung answers the same way
status: open
opened: 2026-11-10
priority: P3
cost: M
---

Found while building the `rung:` register (`scripts/gates/escalation-question-marks-rung.counts`, from `a-question-mark-inside-a-rung-is-registered-nowhere`).

**The shape.** A rung reads `x?`, answers `P` where `x` holds, and otherwise reads `y`, answering `P` where `y` holds too. Where `x` is in band and `y` decides `P`, the answer is `P` whichever way `x` really reads. The `?` refuses there anyway. The rung's refusal is kept and ranked by the ladder, so the climb answers through another rung if one answers. Where none does, it refuses where it could have answered. That is the ladder rule's search, one level down: a later read that does not need the earlier one to hold.

`path_parity`'s two pairs of segment ends (`side(from)?, side(to)?`) had this shape and are ported in the PR that landed the register. These three remain, each registered `SEARCH`:

- `boolean::sectors::crescent` (crates/topo/src/boolean/sectors.rs:2200, :2210). `!parallel_same(da, db, arm, ..)?` answers `false`, and so does the across reading (`bool_sector_crescent_across`) decided not `Negative`. The across reading is computed from `da` whether or not the bounds are parallel, so where `parallel_same` is in band and across decides, the pair answers `false` either way. The port reads across before `parallel_same` refuses. Mind the telemetry: across would be recorded on pairs it is not recorded on today.
- `boolean::insert::held_cut` (crates/topo/src/boolean/insert.rs:1478, :1481). `nested(..)?` answers `false`, and so does `on_arc`'s `[Some(false), _] | [_, Some(false)]`. Where `nested` is in band and `on_arc` places the cut off the run, the cut is not held either way.
- `ring_path::path_parity` (crates/topo/src/ring_path.rs:1274, :1278). On a conic arc off arrival, where `plane_roots` is in band, `misses_in_plane`'s first arm (`split_ring_path_coplanar_apart`) holds whatever the planes, by its own doc. A decided miss there answers `Some(flip)` past the refusal. Its second arm (`split_ring_path_same_circle`) needs `plane_roots`' `Sign::Zero` and does not answer past it. So the port reads the apart arm alone, not `misses_in_plane` whole.

Each port is a refusal that becomes an answer, so each wants a row that reds against today's order: an `x` in the gray band with a `y` decided. `a_segment_ending_on_the_crest_cylinder_answers_past_an_end_in_band` in `ring_path::tests` is the pattern.

## Done when

Each of the three reads its later conjunct before the earlier refuses, under a row that reds against the old order, and its `rung:` entry is lowered and dispositioned `PREMISE`. Or a reviewer rules that a site's later read does not answer alone, and its entry says why.
