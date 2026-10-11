---
id: pn-apex-reach-fuzz-finds-a-contradiction-at-a-pico-epsilon
kind: issue
title: The plane×cone apex reach fuzzer finds a contradiction with the oracle at ε ≈ 8.7e-12
status: open
opened: 2026-10-10
---


## What

`geom-brep`'s `pn_apex_point_reach::apex_lane_outcomes_agree_with_the_oracle`
(`crates/geom-brep/tests/pn_apex_point_reach.rs`, the fuzzer GERM's
PR 4478 added) red on hosted CI for PR 4557 (`flux/ellipse-ring`, head
`405b9ab`), whose diff touches only `geom_brep::props` and nothing
`plane_cone_section` reads:

```
1 contradictions (reproduce with CAD_FUZZ_SEED=0xba10fe80e6ee4af9 CAD_FUZZ_EFFORT=1),
first: conic 2.8130106413936114e-15 off the plane, 2.831068712794149e-15 off the cone,
reaching 2.943635961705671e0: ε 8.720852666426868e-12, α 0.6514115937405218,
extent 73.16333515552087, D·extent -11.268303727345286·ε, gap -0.470·ε,
oracle reach Some(3.0044913412023497)
```

It reproduces locally with that seed. The lane served a conic reaching
2.94 where the oracle's section reaches 3.00, at a draw whose `D·extent`
is −11.3 ε and whose gap is −0.47 ε: a pose by the band edge, at
ε ≈ 8.7e-12 against an extent of 73.

## What is owed

Pin the seed as a deterministic row beside the fix (implementer
discipline §8), and decide which side is wrong: the lane's reach or the
oracle's.
