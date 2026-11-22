---
id: a-profile-point-has-no-2d-spin-move
kind: issue
title: A Point2 or Direction2 has no 2-D spin move, so a tooth cannot be written in copy-0 coordinates and turned
status: parked
opened: 2026-11-22
priority: P3
cost: M
blocked_on: [a-profile-director-holds-one-direction2-and-reads-a-straight-pieces-heading]
---


Filed by the cut of `work/repeat/a-loop-cannot-repeat-inside-a-profile.md`
(both designers of fork log row 296 asked for it). It sits on SELFREF's
ground because the values it would act on are SELFREF's `Point2` and
`Direction2`.

## What

`PoseDef::Spin` (`crates/editor-core/src/pose.rs`) is 3-D only: its
`about` must be an `Axis`. No 2-D move exists. Once SELFREF's U1 and U4
land, a profile point or director could be turned about a point by an
angle, so a gear's tooth could be written once in copy 0's coordinates
and turned by `scalar(k)·turn/N` inside `each k`. Today each copy is
positioned by `cos`/`sin` expressions over `k`, which works and which
the ring's exactness does not rest on (`docs/PATHS-DESIGN.md` §2
*Composition*: the ring closes by a value read twice). So this is a
convenience, banded P3.

`[variables.poses]` admits such a move (designer B's reading, not
checked here). It is parked on U4, the last of the two kinds it acts on.
