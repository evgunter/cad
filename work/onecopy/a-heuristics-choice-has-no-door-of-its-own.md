---
id: a-heuristics-choice-has-no-door-of-its-own
kind: issue
title: A heuristic's choice decided on the least band takes the logged door unless someone argues it out (F31, F32): one door for such choices would make the next one land there
status: open
opened: 2026-11-03
priority: P2
cost: E
---



Found by the design fork on `a-foots-parameter-is-read-without-its-certificate`.

Ledger rows F31 (`curve_foot_branch`, `angle_near`'s half-turn flip) and F32 (`curve_foot_polish`, the polish's tie-break) are one class. Each is a heuristic's choice between candidates, decided on `Band::least()` so it takes the sign alone, and each was first written through the logged `decide`. The logged door is the default, so each one escalated at `Interval` and gave the driver (`editor-core` `drive::log_read`) a cue no box answers, until a lane argued it out with a ledger row.

Other sites take the same shape on the logged door, `decide(.., Band::least())`. One is `geom::curves::on_carrier`'s `curve_carrier_speed`, which reads whether a speed bound is positive. A sweep for `Band::least()` across the kernel finds the rest. Some are verdicts rather than choices, so each needs its disposition before any moves.

Fix: give such choices their own door, for example `k_stats::choose(name, margin, ledger_row)`. It would be `check_unlogged` on the least band with the row required, and the least band would come off the logged `decide`'s callers. Then the next heuristic lands there by construction, and `flagged_census` checks its row.

**A third instance landed while this row was open (2026-11-04):** ledger row F33 (`curve_pair_pose_across`, `geom-brep` `curve_pair::pose`'s `conic_conic`, which picks which of two conics to read across the other's plane) took the same route. It is a choice decided at `Band::least`, argued out of the log with its own row.

**Class sweep from PR 6483's review (2026-11-04).** These are more `decide(.., Band::least())` sites of this shape, each still on the logged door. Each needs its disposition (a choice or a verdict) before it moves:
- `geom-brep` `curve_pair::compare` ("a parameter is no distance the band resolves") and its callers:
  - `curve_pair_meet_order`
  - `curve_pair_pose_branch`
  - the span's branch window
  - `curve_pair_walk_same_foot`
- `geom-brep` `curve_pair::walk`'s `step_taken`

**A sibling door was decided on the meet-interval row (2026-11-04): "a proof's inequality, read exactly".** It is a named `geom_core` door that takes no `Band`, and it stays logged, because a `Meter` refusal is something a narrower box can answer. `compare`, `step_taken`, `curve_foot_polish` and the pose basin all sit on `Band::least()` today. So the sweep above sorts each site into one of three doors:
- **a heuristic's choice**: this row's door, unlogged, with a ledger row required;
- **a proof's inequality**: exact and logged, with no band;
- **a reading**: the logged `decide` against the model's band.

Design both doors in one place, so `Band::least()` stops being spelled at call sites at all. Then the next site has to pick a door by name.
