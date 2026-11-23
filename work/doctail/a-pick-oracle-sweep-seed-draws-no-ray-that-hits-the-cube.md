---
id: a-pick-oracle-sweep-seed-draws-no-ray-that-hits-the-cube
kind: issue
title: The pick-face exact-oracle sweep has seeds whose 120 rays all miss the cube, and its anti-vacuity assert fails red
status: open
opened: 2026-10-12
priority: P3
cost: E
---


Found by FUSE while running the editor-core suite for PR 4893, which
touches no picking code. The failure is in the generator, not the
kernel.

## What

`review_gui1_r1::random_integer_rays_match_the_exact_oracle`
(`crates/editor-core/tests/review_gui1_r1.rs:533`) draws
`fuzz::scaled(120)` rays with integer origins in [-3, 3]³ and integer
directions in [-2, 2]³, and checks `pick_face` against the exact
oracle on each. Its last assert (`:609`) requires at least one draw to
hit the cube.

At ε = 1e-9 (`CAD_TOLERANCE_EPS=1e-9`) the ci run drew seed
`0x02e42a5ed469140d` and failed with:

```
no draw hit the cube — generator shape broke; reproduce with CAD_FUZZ_SEED=0x02e42a5ed469140d CAD_FUZZ_EFFORT=1
```

It reproduces every time with that env override
(`CAD_TOLERANCE_EPS=1e-9 CAD_FUZZ_SEED=0x02e42a5ed469140d CAD_FUZZ_EFFORT=1`).
Every one of the 120 draws is a miss for the exact oracle too, and the
service agrees with it on each (the mismatch arms never fire). So
`pick_face` is right on every case. The sweep is only vacuous, and the
anti-vacuity assert turns that into a red run on an unlucky seed. The
seed varies per run, so this shows up as an intermittent red in any
branch's CI.

## Owed

A generator that cannot draw a vacuous sweep. For example, aim a share
of the rays at the cube (a direction from the origin to a point of the
cube), or keep drawing until it has a fixed number of hits within a
draw budget. Then this seed passes, and the anti-vacuity assert can no
longer fail by chance.
