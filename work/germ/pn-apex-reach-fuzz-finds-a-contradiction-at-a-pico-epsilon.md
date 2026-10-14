---
id: pn-apex-reach-fuzz-finds-a-contradiction-at-a-pico-epsilon
kind: issue
title: The plane×cone apex reach fuzzer finds a contradiction with the oracle at ε ≈ 8.7e-12
status: closed
opened: 2026-10-10
closed: 2026-10-13
priority: P1
cost: M
branch: germ/pn-apex-reach-pico-eps
pr: 4966
refs: [pn-pair-bound-rounding-assumes-an-orthonormal-cone-frame]
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

## Second seed (CLEAVE orchestrator, 2026-10-11)

Red again on hosted CI for PR 4565 (`cleave/apex-section-corners`, head `c756c24`, run
38099971804), whose diff touches no `geom-brep` code:

```
1 contradictions (reproduce with CAD_FUZZ_SEED=0x5a26154859e91c73 CAD_FUZZ_EFFORT=1),
first: conic 2.884e-16 off the plane, 2.845e-16 off the cone, reaching 6.768e-2:
ε 1.372e-12, α 0.4783, extent 16.74, D·extent −9.33·ε, gap −0.169·ε, oracle reach Some(0.0856)
```

The lane's reach (0.0677) falls short of the oracle's (0.0856) by about 21% at a pose by the band
edge. That is the same shape as the first seed. The fuzzer reds unrelated PRs at random, so this
is a live red on the default suite.

## Measured and fixed (`germ/pn-apex-reach-pico-eps`)

**The oracle was wrong; the lane's reach was right.** Both seeds
replay on the tree before PR 4592 (which replaced
`pn_apex_point_reach.rs` with `pn_apex_reach.rs` and re-cut the draws,
so neither seed reproduces on main). Each draw's cone had a seam off
square to its axis: the fuzzer built `u_ref` by projecting a random
`side` off the axis, and `side` fell `3.9e-3` and `1.8e-5` from the
axis, so the projection's rounding left `u_ref·axis = −7.9e-14` and
`−3.8e-12`. The oracle walked its generators off that `u_ref`
(`Pose::generator`), leaning them off the cone by `τ·sin α`, which
moved their slopes to the plane by a part of these `D`
(`−1.39e-12` and `−3.43e-12`, `D·extent` −11.68·ε and −41.8·ε at the
pinned poses; the logs above quote the draws the old fuzzer intended). The served ellipses lie on both surfaces to
`3e-15` and `3e-16`. At those slopes, a point `0.06` short along the far
generator would stand `8e-14` off the plane. Re-squared, the oracle's
reach is `2.94385` (served `2.943636`) and `0.067685` (served
`0.067684`).

PR 4592's fuzzer draws `u_ref` by a cross product, which hides the
skew from its oracle rather than making the oracle frame-free. A cone's
`u_ref ⊥ axis` is checked nowhere (`Surface::representability_margins`
margins no cone frame). The arm, though, reads it at the line pair and
at both circle carriers, which is the sibling item
`pn-pair-bound-rounding-assumes-an-orthonormal-cone-frame`, fixed in
the same PR:

- `pn_apex_reach.rs` `Pose::generator` builds its frame off the axis
  alone, so no oracle reads a stored seam;
- the fuzzer draws half its seams tilted by up to `1e-10`, so the arm's
  reading of the seam is under test (the pre-fix arm reds it on every
  run, 31–48 contradictions);
- the two seeds are pinned as
  `pn_apex_reach::the_two_fuzz_poses_serve_the_ellipse_their_tilted_seams_cut`,
  red against the frame-reading oracle with the item's own numbers
  (`3.0044913412023497`, `0.08555884434047788`).
