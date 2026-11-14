---
id: cone-rows-certified-answers-fail-at-1e-12-on-one-seed
kind: issue
title: boolean::conic_quadric::cone_rows::certified_answers_hold_against_the_quadric_form fails at ε 1e-12 on seed 0x3bf35ddc97f968bd
status: open
opened: 2026-11-09
priority: P2
cost: M
---


Found in PR 6598's delta-review runs and carried on `work/onecopy/the-pierce-pass-ranking-is-pinned-only-at-the-ladder.md` until filed here (PR 6637's review, S7).

`topo boolean::conic_quadric::cone_rows::certified_answers_hold_against_the_quadric_form` fails at `CAD_TOLERANCE_EPS=1e-12` with `CAD_FUZZ_SEED=0x3bf35ddc97f968bd`. It is case 41: an ellipse root 2.7e-6 along the arc from the oracle's. It also fails on 6598's base `fbc704f1`, so it is not that PR's, and no ladder code is on its path.

Repro: `CAD_TOLERANCE_EPS=1e-12 CAD_FUZZ_SEED=0x3bf35ddc97f968bd cargo nextest run -p topo --profile ci -E 'test(=boolean::conic_quadric::cone_rows::certified_answers_hold_against_the_quadric_form)'`.

Reproduced on `onecopy/refusal-standing` during PR 6637's fix pass (it fails at `cone_rows.rs:161`).
