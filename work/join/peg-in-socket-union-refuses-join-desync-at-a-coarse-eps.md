---
id: peg-in-socket-union-refuses-join-desync-at-a-coarse-eps
kind: issue
title: The torus peg-in-socket union builds at the default eps but refuses JoinDesync at eps 1e-6
status: open
opened: 2026-10-02
---


Found on PR 3790 (JOIN-1). CI's `CAD_TOLERANCE_EPS=1e-6` row on 8f8206f8 reported it, and I measured it locally.

## Repro

`crates/sweep/tests/mate7a_torus_rest.rs`: the torus peg-in-socket union of `socket()` and `segment_a()` under `wall_declarations(.., ContactClass::Rest)`. Those declarations cover the walls as `Rest` and the flush caps as continuations.

| tree | ε = 1e-12 | ε = 1e-9 (default) | ε = 1e-6 |
|---|---|---|---|
| main 1ff6064e1 | — | `Join(UnpairedLooseEnds { count: 8 })` | `Join(UnpairedLooseEnds { count: 8 })` |
| JOIN-1 8f8206f8 | builds, sound | builds: sound, additive, a legal operand | `JoinDesync { what: "neither section loop's regions hold a decisive witness" }` |

The refusal comes from the section-loop witness reading, `loop_roles` (`crates/topo/src/boolean/join.rs:1873`). At the coarse ε, no region of either section loop holds a decisive witness.

No ε ships an unsound body. Both rows that build the union (`peg_in_socket_union_holds`) accept that typed refusal only above the default ε.

## What is not known

I have not measured which witness reads in band at 1e-6, or whether a finer witness choice would decide it.
