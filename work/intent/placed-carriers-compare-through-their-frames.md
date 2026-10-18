---
id: placed-carriers-compare-through-their-frames
kind: issue
title: D10 stage 4 PR H: a placed carrier's canonical form reads its placement's frame, so a mate-placed face is structural
status: parked
opened: 2026-10-08
priority: P0
cost: H
branch: intent/s4-h-placed-carriers-compare-through-their-frames
pr: 5300
blocked_on: [a-mate-placed-seat-is-unproven-at-the-doors-budget]
---

INTENT stage 4, PR H. Spec: `docs/INTENT-STAGE4-SPEC.md` §9. FORK-S4-5 is settled by FORK-S3O (fork log row 96, PR 4325), below.

`PoseForm` reads a placement's `Frame` variable (stage 3) instead of C's opaque chain atom, so a mate-placed face reduces equal to its partner. `compose_placed` and `GeomSource`'s `Placed` arm go.

FORK-S4-5 was weighed with FORK-S3-4 as FORK-S3O (fork log row 96) and
went to Ev in an `[ev]` PR. After Ev's comments on #4325 there is no
rewrite system. A placed copy's frame is the construction its ordered
bundle states, and the door proves a mate-placed face (and a fillet
flank on it) by replaying that construction at `Sym` with the rest of
the document, as #4322's door does for any coincidence. So H is "the
mate fold runs at `Sym`" (`SolveScalar` for `Sym`, the fold's `atan2`
an opaque symbol per #4322). Measure first whether rung 3 proves
`turn/4` identities through the fold.

After the stage 3 re-spec (2026-10-09), H waits on stage 3 C (`a-mate-relates-two-poses`), not on the placement unit B. The bundle exists after B, but its mates read poses off geometry only after C. Replaying B's offset frames at `Sym` would be written twice.

- 2026-10-16 (INTENT): when H lands, two things should change. The tour bench's three `unproven-coincidence` findings (one per mate's seat, the viewer's `checks: 3 finding(s)`, `gui-gauge-assembly-shows-three-check-findings`) should go quiet. And `mate6_gather_mints::a_mated_record_cites_the_census_row_the_lint_reports`, which pins today's unproven state, turns red and is re-pinned to the proof. Update the comment beside `bench` in `demos/tour/src/assembly.rs` too.

## 2026-10-18 — infrastructure landed by PR 5300; parked on the seat item

PR 5300 lands what the replay needs. It does not switch on reading placed copies, because that costs the door 10–20× on mated assemblies and proves no seat today (orchestrator's ruling, option 2).

- **The fold's map is a function of its poses alone.** The line two planes meet in, and both anchor steps, carry no lever arm (`mate/coset.rs` `parallel`, `UnitVec3::new_scaled`). A pose's frame is the decided shortest turn, inverted by transpose (`mate/solve.rs` `frame_of`).
- **A symbolic run resolves parts.** Each document version's variables are its own symbols, keyed by document, pin and id (`analysis::symbol_scope`, `var_env_symbolic_pinned`), so `SymbolsAcrossAPartSeam` retires. `Evaluation::resolver` is the run's resolver, and `DoorJob` and `coincide::prove` carry it.
- **A mated census row names each face in its copy** (`checks/at_rest.rs` `copy_cell`).
- **One switch for the rest.** `coincide::REPLAYS_PLACED_COPIES` is off. Turning it on resolves parts in the door's replay and reads an at-rest row by the carrier verdict on the copies' faces. The code is in `coincide/replay.rs` `prove_row`. Pinned off by `mate6_gather_mints::a_mated_record_cites_the_census_row_the_lint_reports` and `asm2a_instantiate::a_symbolic_run_builds_a_part_and_the_door_leaves_a_row_on_one_unreplayed`.

What remains is the row's outcome: a mated seat proven, so the tour bench's three findings go quiet. That waits on `a-mate-placed-seat-is-unproven-at-the-doors-budget`, which carries the measurement and the cost its fix must beat.
