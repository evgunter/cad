---
id: an-infinite-control-net-folds-to-a-box-that-clears
kind: issue
title: A described net with an infinite channel folds to a box with infinite ends, which separates from every finite box
status: open
opened: 2026-10-10
---


## What

`Real::is_poison` is NaN-only at `f64`, so a NURBS net with `±∞` in a
channel reads `NetState::Described`, and both box folds over it answer a
box with infinite ends rather than refusing:

- `geom::surfaces::boxes::nurbs_surface_aabb` (and its curve twin):
  `net::any_poison` passes, and `Aabb::from_points` folds `±∞` through
  `lo()`/`hi()` into the box as-is.
- `topo::census::face_reach_in`'s `FaceBoxRule::ControlNet` arm, whose
  `Described` arm folds the net with `Real::min`/`max`.

A net whose every `x` is `+∞` folds to `x ∈ [+∞, +∞]`, and a gap test
against any finite box on that axis is `+∞`, which decides Positive:
the census backstop CLEARS the pair, and `Aabb::overlaps` prunes it.
The evaluation of such a net is not a locus (`∞·0` is NaN at every
basis zero), so neither answer is a claim the geometry supports.

Tier-3 check 1 refuses such a net at rest
(`NetState::Described if !net_is_finite(..)` → `PoisonedSurfaceDescription`),
so the census's production caller does not reach it; the boolean's
`face_box` runs mid-operation and was not traced. Not reproduced.
The question for the owner is whether `any_poison`'s box screen should
be a finiteness screen (`geom_core::is_finite_length` per channel), and
the census arm then follows it, keeping `the_two_box_lanes_agree_face_for_face`
true.

Found by PIPE's S350 (which screens the poisoned net in the census arm
and left this one alone because the `geom` door it mirrors does the
same).
