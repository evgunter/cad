---
id: circle-pair-home-is-planar-by-name-and-hands-out-an-undecided-half-chord
kind: issue
title: geom_core's circle-pair home is filed under arc as planar, hands out a half-chord outside a decided secant, and is accuracy-guarded only from profile
status: open
priority: P3
cost: M
refs: [5124, check-9-arm-5-is-a-second-copy-of-seg-planar-edge-pair-intersector]
opened: 2026-10-14
---

Found by the review of PR 5124 (`paths/circle-pair-shared-home`), which
gave the circle pair its one home in `crates/geom-core/src/arc/pair.rs`.
Three points about that home, filed rather than fixed in that PR.

## Where it lives and what it says it is

`geom_core::Circles` holds `d` and the two radii, no centres, and
nothing in it depends on a plane: the same margins, radical foot and
half-chord describe two spheres whose centres stand `d` apart, and
sphere × sphere is a named future reader
(`work/roots/sphere-sphere-section-radius-cancels-at-unequal-radii.md`).
Yet it lives under `arc` (`arc.rs` is the planar arc carrier), and its
module and type docs say "two circles in one plane". A sphere-pair
reader would either find it under a planar name or mint a copy. Only
`CirclesAt` (the placement at the centres) is about a plane.

## The half-chord is public outside a decided secant

Three classifiers make the same five-way circle-pair split: the
validator's `CirclePair` (`topo/src/validate.rs` `circle_pair`),
profile's `carrier_circles_identity` / `_external` / `_internal`
(`profile/src/seg.rs` `arc_arc`, `joint_tangency`), and arc_fillet's
`path_carrier_meet` (`profile/src/path/arc_fillet.rs` `circle_circle`).
That duplication is
`check-9-arm-5-is-a-second-copy-of-seg-planar-edge-pair-intersector`'s
matter. What it leaves for the home: `Circles::half_chord()` is public
and unclamped, and its square is positive only where the caller has
decided the external clearance negative and the internal one positive.
Nothing stops a caller from reading it without that decision, where
the result is the poison. A type that hands out crossings only from a
decided `Crossing` (the split decided once, in the home, through the
caller's predicate names) would hold "the factors are the decided
margins" by construction rather than by each caller's discipline, and
would be the one classifier the three sites read.

## Its accuracy promise is guarded from one consumer

The module doc promises crossings on both circles to a few ulps of the
largest of `d`, the radii and the centres' coordinates. The only test
of that is `profile/src/seg.rs`
`circles_accuracy_tests::circle_pair_points_sit_within_a_few_ulps_of_scale`,
beside its double-double oracle (`profile/src/dd.rs`, crate-private and
test-only). geom-core's own suite does not check the promise, so a
change in geom-core that breaks it goes red only in profile's.
Moving the row needs the oracle in reach of geom-core's tests, which
is `work/issues/double-double-arithmetic-is-spelled-in-five-places.md`'s
matter.

