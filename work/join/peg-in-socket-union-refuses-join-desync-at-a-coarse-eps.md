---
id: peg-in-socket-union-refuses-join-desync-at-a-coarse-eps
kind: issue
title: The torus peg-in-socket union's chord join cannot read its section loops' roles above eps 2e-7
status: open
opened: 2026-10-02
---


CI's `CAD_TOLERANCE_EPS=1e-6` row on PR 3790 (JOIN-1) at 8f8206f8 found this. PR 3790's delta-2 review measured the ε ladder.

## Repro

`crates/sweep/tests/mate7a_torus_rest.rs`: the torus peg-in-socket union of `socket()` and `segment_a()` under `wall_declarations(.., ContactClass::Rest)`. Those declarations cover the walls as `Rest` and the flush caps as continuations.

| ε | the chord join |
|---|---|
| 1e-12 … 2e-7 | builds it: sound, additive, a legal operand |
| 3e-7 … 2e-6 | refuses `Join(SectionLoopUndecided)` |

The threshold lies between 2e-7 and 3e-7.

The refusal comes from the section-loop role probe, `loop_roles` (`crates/topo/src/boolean/join.rs`). No region of either section loop holds a witness that reads which side of the other solid it lies on. That is the curved-face frontier of `work/cleave/the-uncut-shell-witness-reads-no-curved-face-interior`, and it is not a kernel defect. Until PR 3790's fix pass 3 it was refused as `JoinDesync`, whose text reads "kernel bug or corrupt reduction"; it now has its own typed refusal, which ends in the shared NOT_YET ending.

Since main's declared-REST zip learned to match arcs (TANG, PR 3823), the zip takes that refusal over. The union now builds at every ε, with the same census (4 faces, 6 edges, 4 vertices, one shell), so `peg_in_socket_union_holds` requires the build at every ε.

On main 1ff6064e1, before the merge, the pose refused `Join(UnpairedLooseEnds { count: 8 })` at every ε.

## What is not known

I have not measured which witness reads in band above 2e-7, or whether a witness off the curved faces' boundaries would decide it.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: it builds only because the declared-REST zip takes over SectionLoopUndecided at ε≥3e-7; stage 4 retires that zip. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released (2026-10-08, INTENT stage 4 A (`intent/s4-a-join`))

The zip that built this union at ε ≥ 3e-7 is deleted, so the join's `Join(SectionLoopUndecided)` stands there again, as the stage-4 spec says it does (§2: a band escalation that stays a refusal). `mate7a_torus_rest.rs`'s `peg_in_socket_union_holds` requires the build below 3e-7 and that refusal from it. No stage-4 unit fixes it: the open question is this row's, which witness reads in band above 2e-7.
