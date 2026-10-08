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

`Place { body, mates }` defines a copy, and its mates are its bundle (FORK-S3-2). The world is an undeletable `Frame` variable, and the product is the copies placed in the world's space. Retired here:

- `PlaceInWorld`, `Node::Gauge`, `InstantiatePart.gauge`/`offset`;
- `SetOffset`/`SetGauge`/`Promote`/`Fold`/`regauge_then_mate`;
- `check_offsets` and `OffsetDisagrees`/`OffsetUnchecked`;
- the spanning tree, roots and `MateRole`.

The refactor doors are rewritten over placements. Instances gain `world: Bodies` and `frame: Frame` ports (FORK-S3-5). Until F, a mate beyond its bundle's pin is verified and minted, which is today's declaring behaviour respelled.

The one-time migration check: every corpus copy's pose and the product digests are bit-equal. The unit closes `a-declaring-mates-alignment-is-never-read`, `gauge-of-recomputes-the-clusters-per-placement-lookup`, `a-placer-row-states-what-a-poisoned-row-cannot`, `placement-step-slots-are-spelled-three-ways` and `a-mate-frame-is-written-in-the-reading-instances-coordinates`, plus the placement half of `a-boxed-rotation-refuses-not-rigid-at-every-placer`. FORK-S3-2 and FORK-S3-5 go to designer pairs, then `[ev]`, before dispatch.

FORK-S3-2 and S3-5 were weighed with S3-3 as FORK-S3M (fork log row 97)
and went to Ev in an `[ev]` PR; this unit builds on the answer
provisionally. `Place` owns its mates, each a clause addressed by
(placement, position), never a node. Which copy moves is which `Place`
reads it, so there is no tree, root or declaring role. `Place` defines
no `Frame` port (FORK-S3O, row 96): a copy's poses are read as
`Carried { copy, pose }`, keyed by the placement. The world placement
and `Transform` are one-mate `Place`s. Ev chooses between two shapes,
and this unit builds the recommended one meanwhile: `Place { shapes,
mates }` reads a list of shapes of one space, and the instance keeps
FORK-1's one `Body` port per world placement of the part, plus `frame:
Frame`, all in one space of the instance's own. A re-pin that adds a
part placement mints a port no `Place` reads yet, and the maintenance
report names it. The alternative is `Place` reading one shape, with the
instance defining `bodies: Bodies` whose members are named by the
part's placements. Stage 2 C ships one whole-world `body` port in the
meantime (`an-instance-defines-one-body-per-part-placement`). A10's
rewrite lands with this unit.
