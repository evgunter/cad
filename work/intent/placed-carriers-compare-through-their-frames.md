---
id: placed-carriers-compare-through-their-frames
kind: issue
title: D10 stage 4 PR H: a placed carrier's canonical form reads its placement's frame, so a mate-placed face is structural
status: closed
opened: 2026-10-08
priority: P0
cost: H
closed: 2026-10-18
branch: intent/s4-h-placed-carriers-compare-through-their-frames
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

## Closed

H's mechanism has landed. The door replays a placed copy as the construction its bundle states (D10 `[placement.placement]`, FORK-S3O). On main, before H, `GeomSource` and `compose_placed` were already gone and there was no `PoseForm`. The rungs were the `Sym` replay, and at-rest rows were `NotReplayed`, so this is what H changed:

- **At-rest rows are read off the replay's copies.** A mated pair's census row names each face in its copy, as the unattributed rows already did (`assembly.rs` `mint`, `checks/at_rest.rs` `copy_cell`). The door reads it by the kernel's carrier verdict on the two copies' faces, each bundle folded at `Sym` (`coincide/replay.rs` `prove_row`).
- **Parts cross the `Sym` seam.** A symbolic run binds each document's variables as its own symbols, keyed by document and id (`analysis::AxisScalar::axis_of_in`), so `ParamBoxError::SymbolsAcrossAPartSeam` retires. `Evaluation::resolver` is the run's resolver, and the door replays with it (`DoorJob`, `coincide::prove`'s `resolver`).
- **The fold's map is a function of its poses alone.** The line two planes meet in, and both step arms, carry no lever arm (`mate/coset.rs` `parallel`, `UnitVec3::new_scaled`). A pose's frame is the decided shortest turn, inverted by transpose (`mate/solve.rs` `frame_of`), with no guard the tier keeps as an atom.

The outcome this row named is not reached. The tour bench's three findings and mate6's row are replayed but unproven, because the fold's frames hold component symbols renormalised at every hop. The measurement and the routes are in `a-mate-placed-seat-is-unproven-at-the-doors-budget` (P0), and `mates-declare-no-contact` now waits on that row.
