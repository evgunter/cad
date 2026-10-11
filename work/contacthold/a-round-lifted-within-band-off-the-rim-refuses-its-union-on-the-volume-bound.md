---
id: a-round-lifted-within-band-off-the-rim-refuses-its-union-on-the-volume-bound
kind: issue
title: A quarter round lifted within the band about its rim refuses its union ResultVolumeImplausible, the result a few bands over vol(A) + vol(B)
status: open
opened: 2026-10-11
priority: P2
cost: M
---


## Finding

PR 4704's dual review. Both reviewers saw it, and it is the same on
that PR's base.

R1's `review_4704_r1_probes.rs` `r1_turned_about_the_rim` turns the
quarter round about its rim line by `θ`, which lifts its bottom off the
slab's top. For `θ` inside the band, slab ∪ round refuses, flush and
corner alike:

```
ResultVolumeImplausible { which: "vol(A ∪ B) ≤ vol(A) + vol(B)" }
```

The result comes out about `2·θ` over `vs + vr`:

| ε | θ | flush ∪ | bound |
|---|---|---|---|
| 1e-9 | 1e-10 | 31.14159265378979 | 31.141592653589793 |
| 1e-9 | 1e-12 | 31.1415926535918 | 31.141592653589793 |
| 1e-6 | 1e-7 | 31.14159285358979 | 31.141592653589793 |

The ops glue the near-coincident bottom onto the top, as an in-band
pair should be glued, and the union's volume then counts the in-band
gap. The volume backstop's bound carries no allowance for an in-band
glue, so a legitimate glue fails it.
