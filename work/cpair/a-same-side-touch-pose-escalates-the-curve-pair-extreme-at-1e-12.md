---
id: a-same-side-touch-pose-escalates-the-curve-pair-extreme-at-1e-12
kind: issue
title: review_4781_r2_probes' same-side touch row is red at ε 1e-12 on main: the census escalates curve_pair_pose_extreme
status: open
opened: 2026-10-16
priority: P3
cost: M
---

`sweep::all review_4781_r2_probes::r2_every_same_side_touch_pose_answers_as_a_touch` (slow set) fails at `CAD_TOLERANCE_EPS=1e-12` on clean main `a68bc36d1` (and on TANG's U-lever branch, PR 5204, the same way):

    pc corner +y ×1e3, A ∪ B: 3′ on its records: Err([CensusEscalated { cause: Indeterminate { margin: MarginDiag(Value(-9.301125163396388e-12, None, ZeroWord(0))), .., band: Band { zero: 1e-12, escalate: 1e-11 }, predicate: Some("curve_pair_pose_extreme") } }, CensusUndecidable { a: Face(FaceKey(3v1)), b: Face(FaceKey(5v1)), .. }, ..])

The census's curve-pair pose reading (`curve_pair_pose_extreme`) lands at −9.3e-12, in the fine band, on the `pc corner +y ×1e3` pose, so 3′ refuses the union its records describe. Found while running the slow set at every ε for PR 5204; not caused by it. Reproduce with `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --profile ci --ignore-default-filter -E 'test(=review_4781_r2_probes::r2_every_same_side_touch_pose_answers_as_a_touch)'`.

