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
needs_ev: true
---

INTENT stage 3, PR C. Spec: `docs/INTENT-STAGE3-SPEC.md` §4.

`Place { body, mates }` defines a copy, and its mates are its bundle (FORK-S3-2). The world is the one undeletable node; the product is the bodies it names, which lie in one space that one `Place` relates to it. Retired here:

- `PlaceInWorld`, `Node::Gauge`, `InstantiatePart.gauge`/`offset`;
- `SetOffset`/`SetGauge`/`Promote`/`Fold`/`regauge_then_mate`;
- `check_offsets` and `OffsetDisagrees`/`OffsetUnchecked`;
- the spanning tree, roots and `MateRole`.

The refactor doors are rewritten over placements. Instances gain no `frame` port (FORK-S3M). A mate beyond its bundle's pin refuses from this unit on (FORK-S3O, row 96); today's declaring mates become assertions or are dropped and named, with no interim that verifies them.

The one-time migration check: every corpus copy's pose and the product digests are bit-equal. The unit closes `a-declaring-mates-alignment-is-never-read`, `gauge-of-recomputes-the-clusters-per-placement-lookup`, `a-placer-row-states-what-a-poisoned-row-cannot`, `placement-step-slots-are-spelled-three-ways` and `a-mate-frame-is-written-in-the-reading-instances-coordinates`, plus the placement half of `a-boxed-rotation-refuses-not-rigid-at-every-placer`. FORK-S3-2 and FORK-S3-5 go to designer pairs, then `[ev]`, before dispatch.

FORK-S3-2 and S3-5 were weighed with S3-3 as FORK-S3M (fork log row 97,
PR 4326), and this unit builds on that answer. `Place` owns its mates,
each a clause addressed by (placement, position), never a node. Which
copy is defined is which `Place` reads it, so there is no tree, root or
declaring role. `Place` defines no `Frame` port (FORK-S3O, row 96): a
copy's poses are read as `Carried { copy, pose }`, keyed by the
placement. Nothing moves a body: `Transform` and `PlacedFrom` retire,
each use becoming a `Place`, and a profile reused at several positions
is several constructions reading it through frames off geometry, not
copies (the die's pips are revolves of the ball's profile on frames off
the die's faces). `Pattern` is a `Place` of several copies over a pose
family read off geometry. The world names bodies for the product and
defines no copy; the named bodies lie in one space, and one `Place` of
its own relates that space to the world, read by export alone. An
instance defines no `frame` port. Ev chooses between two shapes, and
this unit builds the recommended one meanwhile: `Place { shapes, mates
}` reads a list of shapes of one space, and the instance keeps FORK-1's
one `Body` port per body the part's world names, all in one space of
the instance's own; a re-pin that names another body mints a port no
`Place` reads yet, and the maintenance report names it. The alternative
is `Place` reading one shape, with the instance defining `bodies:
Bodies` whose members are named by the part's world. Stage 2 C ships
one whole-world `body` port in the meantime
(`an-instance-defines-one-body-per-part-placement`). A10's rewrite
lands with this unit.
