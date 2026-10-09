---
id: a-placement-is-the-bundle-of-mates
kind: issue
title: D10 stage 3 PR C: Place { body, mates } defines a copy and the world is a frame; PlaceInWorld, gauges, offsets, the spanning tree, roots and declaring-by-gauge retire
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [a-mate-relates-two-poses]
refs: [intent-stage3-is-built, mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion]
---

INTENT stage 3, PR C. Spec: `docs/INTENT-STAGE3-SPEC.md` §4.

`Place { body, mates }` defines a copy, and its mates are its bundle (FORK-S3-2). The world is one frame among many that cannot be deleted; the product is every copy whose space reaches it. Retired here:

- `PlaceInWorld`, `Node::Gauge`, `InstantiatePart.gauge`/`offset`;
- `SetOffset`/`SetGauge`/`Promote`/`Fold`/`regauge_then_mate`;
- `check_offsets` and `OffsetDisagrees`/`OffsetUnchecked`;
- the spanning tree, roots and `MateRole`.

The refactor doors are rewritten over placements. Instances gain no `frame` port (FORK-S3M). A mate beyond its bundle's pin refuses from this unit on (FORK-S3O, row 96); today's declaring mates become assertions or are dropped and named, with no interim that verifies them.

The one-time migration check: every corpus copy's pose and the product digests are bit-equal. The unit closes `a-declaring-mates-alignment-is-never-read`, `gauge-of-recomputes-the-clusters-per-placement-lookup`, `a-placer-row-states-what-a-poisoned-row-cannot`, `placement-step-slots-are-spelled-three-ways` and `a-mate-frame-is-written-in-the-reading-instances-coordinates`, plus the placement half of `a-boxed-rotation-refuses-not-rigid-at-every-placer`. FORK-S3-2 and FORK-S3-5 go to designer pairs, then `[ev]`, before dispatch.

FORK-S3-2 and S3-5 were weighed with S3-3 as FORK-S3M (fork log row 97,
PR 4326), and this unit builds on that answer. `Place` owns its
constraints, mates and values on equal footing, each addressed by
(placement, an id minted with it), never by its position in the list
and never a node. A value sets one freedom the mates leave, a slide or
a spin, to a `Length` or `Angle` variable, charted on the two bodies'
own coordinates (the copy's origin from the target's; the angle between
their references), zero always valid; a constraint fixing nothing still
free refuses, and so does a mate taking a freedom a value sets. Pinning
within a stated symmetry is gone: a round pin's spin is a value too,
which the façade writes as a free `0` variable. The `Offset` mate target
carrying in-plane numbers retires; a standoff stays a construction on
the target. Which
copy is defined is which `Place` reads it, so there is no tree, root or
declaring role. `Place` defines no `Frame` port (FORK-S3O, row 96): a
copy's poses are read as `Carried { copy, pose }`, keyed by the
placement. Nothing moves a body: `Transform` and `PlacedFrom` retire,
each use becoming a `Place`. No construction reads a frame (FORK-S3P
round 8), so a feature at several positions is one body placed several
times: the die's pips are copies of one ball placed against the die's
faces, then subtracted. A pattern is a `Place` whose reads reach an index, its values
expressions in it (FORK-PAT). The world is one frame among many that cannot be deleted, read by
placements and export alone; the product is every copy whose space
reaches it (Ev: one relation for the whole product is a style, placing
one body against the world and the rest against it, not a rule). An
instance defines no `frame` port. `Place { shapes, constraints }`
reads a list of shapes of one space, as `union` reads its operands (a
list of reads, not a list literal), and the instance keeps FORK-1's one
output per world placement of the part, keyed by that placement's node
id and a family under the part's own index where that placement is one
(FORK-S3M round 9, both designers); a re-pin that adds a world copy
mints a port no `Place` reads yet, and the maintenance report names it.
Stage 2 C ships
one whole-world `body` port in the meantime
(`an-instance-defines-one-body-per-part-placement`). A10's rewrite
lands with this unit.
