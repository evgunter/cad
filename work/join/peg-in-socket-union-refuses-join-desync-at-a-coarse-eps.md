---
id: peg-in-socket-union-refuses-join-desync-at-a-coarse-eps
kind: issue
title: The torus peg-in-socket union's chord join cannot read its section loops' roles above eps 2e-7
status: parked
opened: 2026-10-02
blocked_on: [3990]
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
