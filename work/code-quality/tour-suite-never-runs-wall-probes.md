---
id: tour-suite-never-runs-wall-probes
kind: issue
title: demos/tour's test suite never executes the wall-probes loop — the walls gate only through the render walk
status: open
opened: 2026-08-31
github: 1434
refs: [1421]
---

## From GitHub issue 1434

Opened 2026-08-31; 0 comments.

(S-MESH orchestrator) Filed from MESH-2's dual review ([#1421](https://github.com/evgunter/cad/pull/1421)). Proven by execution in two independent lanes: `cd demos/tour && cargo test --release` — the spec-level local acceptance command, and the "demos tour suite" CI row — is fully green at a base where all four Klein wall-7 lottery cells refuse `Triangulation`. The wall-probes loop (`klein::wall_probes` and its siblings) runs only in the binary's render walk (`main.rs::walk_tour`), i.e. the k-lint gate's tour step and the render lanes — a green "demos tour suite" job name sits over unexecuted wall probes, the silent-coverage class's fourth face (after CONFLICTING-no-run, queued-with-zero-jobs, and the skipped-step k-lint rows).

The ask: either the tour test suite gains a row that drives the wall-probe loops (cheap: a `#[test]` that calls the walk's probe pass over the scenes that declare walls), or the coverage split is stated where a reader of the CI row will see it (the row's name currently claims "the tour's own probes"). Not urgent — the k-lint gate does execute the walls — but the local acceptance command being blind to wall regressions cost nothing only because CI's sampled k-lint row happened to draw.

Ground: `demos/` is SMELL Track X's fence (its Python J's); filed for whoever holds X. Line numbers as of #1421's head.

## Home

`work/code-quality/` — the issue names `demos/` as SMELL Track X's fence and files it for whoever holds X; the code-quality register owns Tracks K–X.
