---
id: a-heuristics-choice-has-no-door-of-its-own
kind: issue
title: A heuristic's choice decided on the least band takes the logged door unless someone argues it out (F31, F32): one door for such choices would make the next one land there
status: closed
opened: 2026-11-03
priority: P2
cost: E
closed: 2026-11-09
pr: 6602
branch: onecopy/choose-door
---



Found by the design fork on `a-foots-parameter-is-read-without-its-certificate`.

Ledger rows F31 (`curve_foot_branch`, `angle_near`'s half-turn flip) and F32 (`curve_foot_polish`, the polish's tie-break) are one class. Each is a heuristic's choice between candidates, decided on `Band::least()` so it takes the sign alone, and each was first written through the logged `decide`. The logged door is the default, so each one escalated at `Interval` and gave the driver (`editor-core` `drive::log_read`) a cue no box answers, until a lane argued it out with a ledger row.

Other sites take the same shape on the logged door, `decide(.., Band::least())`. One is `geom::curves::on_carrier`'s `curve_carrier_speed`, which reads whether a speed bound is positive. A sweep for `Band::least()` across the kernel finds the rest. Some are verdicts rather than choices, so each needs its disposition before any moves.

Fix: give such choices their own door, for example `k_stats::choose(name, margin, ledger_row)`. It would be `check_unlogged` on the least band with the row required, and the least band would come off the logged `decide`'s callers. Then the next heuristic lands there by construction, and `flagged_census` checks its row.

**A third instance landed while this row was open (2026-11-04):** ledger row F33 (`curve_pair_pose_across`, `geom-brep` `curve_pair::pose`'s `conic_conic`, which picks which of two conics to read across the other's plane) took the same route. It is a choice decided at `Band::least`, argued out of the log with its own row.

**F33's tie (from PR 6525, 2026-11-04):** when F33 moves onto the heuristic-choice door, give that door a `Tie` variant. F33 reads the first conic at a tie. Two conics tangent at a point tie but for rounding, each tangent to the other's plane, and there the bare difference picks by rounding. That is order-free, because the difference is antisymmetric bit for bit, and since PR 6525 either reading certifies the touch (the window reads its height and level locally). A charged tie band reading the first, PR 6525's first head, made `meet` order-dependent on tangent pairs: the review measured 123 of 400. A door with `Tie` should keep the choice antisymmetric, not fall back to argument order.


**The proof door beside it, and the next site (`onecopy/polish-meters`, 2026-11-04).** That unit built the door for a proof's inequality: `geom_core::k_stats::prove(name, margin)`. It is exact (the least band), takes no `Band`, and is logged. `curve_pair::compare`'s callers and the closest-pair basin (`curve_pair_pose_basin`, with its move `curve_pair_pose_move` and the walk's `curve_pair_walk_basin`) read it. The choice door is still owed. Name it beside `prove`, for example `k_stats::choose(name, margin, ledger_row)`. F32 (`curve_foot_polish`) and F33 (`curve_pair_pose_across`) stay unlogged under their rows until it exists. The next site it takes is `geom_brep::curve_pair::walk`'s `step_taken` (`curve_pair_walk_polish_step`). That is a polish step kept or dropped, which "can lose a meeting's foot but never invent one", a choice in F32's class. It is still on the logged `decide` at `Band::least()`, with no ledger row (ONECOPY orchestrator's ruling, 2026-11-04).
