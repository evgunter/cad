---
id: a-pinned-fuzz-witness-reads-the-environments-seed
kind: issue
title: Two pinned envelope-lemma fuzz witnesses are unpinned by CAD_FUZZ_SEED in the environment
status: open
opened: 2026-11-23
priority: P3
cost: E
refs: [7442]
---



Found by `oneround/readings-not-levers` (PR 7442) and confirmed by its review (finding m5).

## What

`crates/geom-brep/src/pcurve_cache/envelope_lemma_fuzz/projected.rs` holds two rows named as pinned witnesses:

- `pcurve_cache::envelope_lemma_fuzz::projected::a_line_near_a_ruling_pinned_where_the_frame_cost_refuses` (assertion at `projected.rs:1526`)
- `pcurve_cache::envelope_lemma_fuzz::projected::a_carrier_on_its_chart_pinned_where_the_frame_cost_refuses` (assertion at `projected.rs:303`)

Each fails with `pinned to escalate pcurve_envelope at Interval on some draw at ε 1e-12; none did` when `CAD_FUZZ_SEED` is set in the environment:

| `CAD_FUZZ_SEED` at ε 1e-12 (`ci` profile, geom-brep) | result |
|---|---|
| unset | both green (also hosted CI's 1e-12 row on PR 7442) |
| `12345` | both red, on PR 7442's head and on its merged bases alike |
| `1` | one of the two red |
| `999` | both green |

So the witness is a draw, and the environment's seed replaces the draw it was pinned on. A lane that pins every fuzzer for a differential (`CAD_FUZZ_SEED=…`, as the `oneround` units do) reads two reds that are no defect of the kernel.

## Done when

A row that asserts a pinned witness draws from its own fixed seed whatever the environment says (the env override is for the counterexample searches beside it), or the witness is written down as a static fixture (`docs/prompts/implementer-discipline.md` §8: a witness you can write down is not searched for).
