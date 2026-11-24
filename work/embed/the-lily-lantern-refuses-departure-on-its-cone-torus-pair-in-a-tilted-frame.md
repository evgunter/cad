---
id: the-lily-lantern-refuses-departure-on-its-cone-torus-pair-in-a-tilted-frame
kind: issue
title: Since PR 7131's carrier frames, the tour's lily lantern refuses Departure on its cone-torus pair in a tilted frame, the same regression as the balls' through another reader, present_chart's clip
status: open
opened: 2026-11-22
priority: P1
cost: M
---

Found by EMBED's one-plane lane (`embed/one-plane`), on the tour walk; not
fixed there.

## What

The tour's `lily/lily_lantern` refuses the injectivity question,
`Uncertified { a: 3v1, b: 12v1, shared: Edge(19v1), arm: Departure }` at
33 candidates, 20,254 discharged, 76 refused, 20,297 splits, on main
`2da45c59f` with or without `embed/one-plane`'s reading (the walk's line is
identical). PR 7131 (the carrier frames) moved it, the same regression as the
balls' seam meridians
(`a-balls-seam-meridians-in-one-bracket-are-not-one-plane-so-its-pole-refuses-at-interval`)
through another reader: the clip, not `planes_through`. The review of
PR 7328 bisected it to `e30349eab` embedded and `9bc9d6517` (the merge)
refusing with this payload, and this lane's walk agrees: the whole walk on that merge's first
parent `e4d679d37` reads the lantern embedded at 5,619 / 0 / 5,586
(discharged / refused / splits), 82 of 85 holding; on the merge
`9bc9d6517` it refuses as above, 78 of 85, the three dice with it. On
`embed/one-plane`'s head (main `4d3402ecb` merged, PR 7120 in it) the
dice hold again and the lantern alone still differs from `e4d679d37`.

The pair is a cone (`3v1`) against a torus (`12v1`), both in the lantern's
tilted frame (axis `(0.2079…, 0, …)`). Their shared edge `19v1` is one of two
semicircles of one circle: the stored frames of `19v1` and `20v1` have `u`
opposed and centres two ulps apart in `z` (`0.4296608638255445` and
`…443`), so they are not one conic stored or lifted, and they were not
before PR 7131 either.

## The exits (`CV_TRACE`, on `embed/one-plane-measure`)

Over the whole walk of the lantern, `present_chart` refuses `Local` at:
- 6,576 times, the cell outside a line on no inside one and not
  `vacuous` (`present_chart`'s `!inside && !outside.is_empty()`);
- 1,720 times, two lines on one side that hold no common value;
- `zero_of`, 203 times.

On the refusing pair, the cone's cells at `y ∈ [-0.336, -0.126]` present
with `zero = Side(YLo)` and the torus's cell refuses `Local`, or the cone's
cell reads `Vacuous` and the torus's refuses `Local`. Which line the torus's
clip reads, and whether its lift is the polar read's width, is not traced.

## Owed

Trace the torus cell's clip on `3v1 × 12v1`; say whether its line is a
lifted frame's rounding, as the meridians' was
(`a-balls-seam-meridians-in-one-bracket-…`).

## Embedded with one line read by key (EMBED collinear-cover lane, 2026-11-23)

With PR 7322's joint (`injective::joints`: rows joined at a vertex by
key are one line whatever values they hold), the tour walk's
`lily/lily_lantern` stop reads **`Embedded`**
(`cand 33 disc 5620 refused 0 splits 5587`) where main reads
`Refused Uncertified { 3v1, 12v1, Edge(19v1), Departure }`
(`disc 20254 refused 76 splits 20297`). Measured twice, on the branch
merged with main `a3d5ce89c` and with main `b5b3fe7da`; in both the
walk's `diff` against main's own is this one stop (78 of 85 → 79). Not
re-walked on the landing tree. The 1,720 "two lines on one side" exits
this row traced are the merge's value compare, which the joint
replaces for rows joined at a vertex.
