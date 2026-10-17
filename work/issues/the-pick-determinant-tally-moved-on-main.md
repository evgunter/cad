---
id: the-pick-determinant-tally-moved-on-main
kind: issue
title: viewer's review_pick_r2 corpus tally moved from its pin on main: the certified determinant now refuses fewer crossings
status: open
opened: 2026-10-17
---

`viewer::all review_pick_r2::the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus`
(`crates/viewer/tests/review_pick_r2.rs:199`) is red on main at
`82d0f5c2b7`. It is in the slow set, so it runs only on a PR that seeds
`viewer`; PR 4674 is the first seen hitting it. Reproduced on a clean
checkout of `82d0f5c2b7`:

```
the tally over the corpus moved from its pin
  left: (441198, 141201, 12294, 6504)
 right: (441198, 141261, 14088, 7230)
```

The tuple is (rays, answered at the aimed vertex, candidates refused at
the determinant, rays with a refusal). The ray count held. 60 fewer rays
answer at the aimed vertex, 1794 fewer candidates are refused and 726
fewer rays carry a refusal. The test's two stronger asserts, no wide
winner and no genuine determinant refused, both held. So the
certification changed what it refuses, and the pin is stale, but no
crossing that should certify was refused.

Not bisected. The likely cause is a main change to the corpus bodies'
geometry or to the pick's certification since the pin was last taken.
Re-derive the tally with `--nocapture` on the change that moved it, say
what moved, and re-pin (`PINNED` in the same file).
