---
id: main-red-torus-region-probe-fuzz-reads-on-boundary-near-the-tube
kind: issue
title: Main red: torus_region_probe's random queries read OnBoundary where the truth is In near the tube, at ε 1e-6, since the test landed (PR 5269)
status: closed
opened: 2026-10-19
priority: P1
cost: M
pr: 5363
closed: 2026-10-19
---


Found by CLEAVE, in PR 5185's review.

**The red.** `sweep::all torus_region_probe::random_queries_read_the_truth` fails at ε = 1e-6 (`torus_region_probe.rs:505`) on `origin/main` 759dcbe3: "near the tube: 63 wrong answers (reproduce with CAD_FUZZ_SEED=0x8e54900f8aa4853b CAD_FUZZ_EFFORT=1)". The answers are like "the donut, the spiric box, T ∪ P at (…): OnBoundary, the truth In". The seed varies per run; an unpinned run drew 0x35698f988bc16e94 and got 75 wrong. It passes at default, 1e-9 and 1e-12.

**Red since the test landed.** d65863afea, the merge of PR #5269 (`restread/torus-region`), gives the same 63 with that seed. The test file was created in e69cdb7f1e.

**Owed.** Each wrong answer is a point-in-solid misclassification (OnBoundary for In) near a torus tube at 1e-6. Find out whether the classifier is wrong or the oracle's truth is, then fix the classifier or the test.

## Closed

The oracle was wrong, not the classifier. At ε 1e-6 the run's band is
`(1e-6, 1e-5)`, and the near population drew offsets from `1e-7`. All
63 wrong answers at seed `0x8e54900f8aa4853b` sat `1.02e-7` to
`9.34e-7` off the tube, inside the band's zero, where the pre-pass
decides the residual Zero and the door owes `OnBoundary` on a kept
wall. The probe now labels a near point within `band().zero()` as on
the tube, so its truth is the tube's (the side where the op cut the
wall away); points between the zero and the escalation edge still
demand their side. Mutating the torus pre-pass to read OnBoundary
within `1e-4`, or within `K·ε`, turns the probe red (138 and 45 wrong).
