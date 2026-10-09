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

`Place { body, mates }` defines a copy, and its mates are its bundle (FORK-S3-2). The world is an undeletable `Frame` variable, and the product is the copies placed in the world's space. Retired here:

- `PlaceInWorld`, `Node::Gauge`, `InstantiatePart.gauge`/`offset`;
- `SetOffset`/`SetGauge`/`Promote`/`Fold`/`regauge_then_mate`;
- `check_offsets` and `OffsetDisagrees`/`OffsetUnchecked`;
- the spanning tree, roots and `MateRole`.

The refactor doors are rewritten over placements. Instances gain `world: Bodies` and `frame: Frame` ports (FORK-S3-5). Until F, a mate beyond its bundle's pin is verified and minted, which is today's declaring behaviour respelled.

The one-time migration check: every corpus copy's pose and the product digests are bit-equal. The unit closes `a-declaring-mates-alignment-is-never-read`, `gauge-of-recomputes-the-clusters-per-placement-lookup`, `a-placer-row-states-what-a-poisoned-row-cannot`, `placement-step-slots-are-spelled-three-ways` and `a-mate-frame-is-written-in-the-reading-instances-coordinates`, plus the placement half of `a-boxed-rotation-refuses-not-rigid-at-every-placer`. FORK-S3-2 and FORK-S3-5 go to designer pairs, then `[ev]`, before dispatch.

**A host construction never reads a part's world** (from the second review of INTENT stage 2 C, PR #4359, NOTE-F). In stage 2 an instance's one `body` output is its part's whole product, every copy at its pose (`eval/parts.rs`), so a host boolean reading an instance depends on the part's placement poses: the pose is read "by the gather" only in the letter. Stage 2 C refuses the visible edges of this (`Uncarried::Posed` and `SplitError::RemainderReadUncarried` where split or inline would carry such a read across the seam), but the read itself stands. This unit's instance ports must close it: no port a host construction reads carries a part's world placements.
