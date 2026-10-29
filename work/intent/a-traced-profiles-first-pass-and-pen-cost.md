---
id: a-traced-profiles-first-pass-and-pen-cost
kind: issue
title: A traced profile costs an extra nominal evaluation per analysis evaluation, and each pen replays its whole prefix
status: open
opened: 2026-10-29
priority: P3
---


Found by INTENT stage 6 PR C's review (`work/intent/a-sketch-continues-tangent-to-a-surfaces-trace.md`, PR #6043). Two costs, neither measured.

1. **The nominal evaluation.** At every lane but f64, an evaluation of a document holding a traced profile first evaluates the whole nominal document at f64 (`nominal_ev`, `crates/editor-core/src/eval/mod.rs:3660`). The first pass reads the traces off that evaluation. A drive that evaluates the document once per leaf pays it once per leaf, though the nominal document is the same at every leaf.
2. **The pen.** Each step that runs from the pen replays the whole prefix before it (`profile::pen`, `crates/profile/src/path/program.rs:2989`; called from `crates/editor-core/src/trace.rs:347`). A chain with `n` such steps replays O(n²) steps.

**Done when** both are measured on a drive over a traced document. Then either the nominal evaluation is shared across a drive's leaves and the pen is carried along the resolution, or the measurement says neither matters.
