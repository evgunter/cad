---
id: a-round-lifted-within-band-off-the-rim-refuses-its-union-on-the-volume-bound
kind: issue
title: A quarter round lifted within the band about its rim refuses its union ResultVolumeImplausible, the result a few bands over vol(A) + vol(B)
status: closed
opened: 2026-10-11
priority: P2
cost: M
branch: cleave/volume-relations-a-test-oracle
pr: 5842
closed: 2026-10-27
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

The cause is not measured. The overshoot is about 2θ, which matches
neither the in-band gap wedge (about 4.5·tan θ) nor a glue, which would
close a gap rather than count it. Measure which body the union builds,
and what the volume backstop's bound allows, before acting on a theory.

## Closed (2026-10-27)

The volume backstop left the boolean's operation path (`[ev]` PR 5780: the volume relations are a test oracle the suites run on their own results, `topo::test_support::volume_relations`), so nothing refuses on it. `review_4704_r1_probes::r1_turned_about_the_rim` on PR 5842's head, ε 1e-9, the round lifted `θ` about its rim: every in-band union builds one solid within the probe's `1e3·ε` of `vs + vr` — flush 31.141592653591797 (θ 1e-12) and 31.141592653789797 (θ 1e-10), area 71.85398163397448 exactly the flush closed form's; corner 27.141592653593793 and 27.141592653989793. The `2θ` over the closed form is the glue closing the lifted bottom onto the slab's top, inside the ε of displacement a correct result may carry. At `θ ≥ 1e-9` (past the band) the union escalates at `Containment { Loop(Boundary) }`, as it did.
