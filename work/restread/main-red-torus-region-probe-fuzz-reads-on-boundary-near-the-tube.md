---
id: main-red-torus-region-probe-fuzz-reads-on-boundary-near-the-tube
kind: issue
title: Main red: torus_region_probe's random queries read OnBoundary where the truth is In near the tube, at ε 1e-6, since the test landed (PR 5269)
status: open
opened: 2026-10-19
priority: P1
cost: M
---


Found by CLEAVE, in PR 5185's review.

**The red.** `sweep::all torus_region_probe::random_queries_read_the_truth` fails at ε = 1e-6 (`torus_region_probe.rs:505`) on `origin/main` 759dcbe3: "near the tube: 63 wrong answers (reproduce with CAD_FUZZ_SEED=0x8e54900f8aa4853b CAD_FUZZ_EFFORT=1)". The answers are like "the donut, the spiric box, T ∪ P at (…): OnBoundary, the truth In". The seed varies per run; an unpinned run drew 0x35698f988bc16e94 and got 75 wrong. It passes at default, 1e-9 and 1e-12.

**Red since the test landed.** d65863afea, the merge of PR #5269 (`restread/torus-region`), gives the same 63 with that seed. The test file was created in e69cdb7f1e.

**Owed.** Each wrong answer is a point-in-solid misclassification (OnBoundary for In) near a torus tube at 1e-6. Find out whether the classifier is wrong or the oracle's truth is, then fix the classifier or the test.
