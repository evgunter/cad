---
id: peg-in-socket-union-refuses-join-desync-at-a-coarse-eps
kind: issue
title: The torus peg-in-socket union's chord join cannot read its section loops' roles above eps 2e-7
status: closed
closed: 2026-10-08
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

## Closed (2026-10-08, INTENT stage 4 A (`intent/s4-a-join`))

Main's join built the union at every ε before stage 4 A: at A's base `4908d4725`, with the zip door still in place, both reviews of PR 4364 instrumented the door and found the union built by the join at 3e-7, 1e-6 and 2e-6, the zip never reached (`zip=false`, tier 3′ ok, volume 0.048854541785392246). Which merge to main did it is not bisected; stage 4 A's germ change did not. With the zip deleted, `mate7a_torus_rest.rs`'s `peg_in_socket_union_holds` (tiers 2 and 3′, the at-rest certificate, additive volume, 4 faces, 6 edges, 4 vertices, one shell, a legal operand) passes at 3e-7, 1e-6, 2e-6, 1e-12 and the default row, through `a_declared_torus_rest_pair_passes_the_declaration_door` and `a_partly_covered_torus_pair_is_no_longer_a_gate_question`.
