---
id: census-arm-1-reads-directions-from-the-pairs-own-faces-only
kind: issue
title: The census's arm 1 proposes directions from the refusing pair's own faces only, so a sliver its own flat parts from a block, or two pieces either side of a lens, refuse CurvedWithinReach
status: closed
closed: 2026-10-28
opened: 2026-10-26
priority: P1
cost: M
refs: [census-cross-solid-curved-pairs-undecidable-on-shell-results]
branch: restread/arm1-third-face-directions
pr: 5858
---


Measured by RESTREAD on `restread/curved-pairs-apart`, with the
offset directions and the held sphere reach in. Twelve lines of
`sweep` `join3_r2_probes::j3r2_pocket_battery` (ignored, release) still
fail tier 3′ on `CensusUndecidable { CurvedWithinReach }` alone, all at
the offset `(0.75, 0.75)`:
- block ∪ `D-0.45` turned 4 rad, entering from the top, bottom,
  through, flush top and flush bottom, in both orders (10 lines);
- block ∖ `lens` through, at φ 0.7 and 4.0 (2 lines).

## The sliver

`D-0.45` is the thin segment of the rod's disc past its chord at
`x = −0.45`. Turned 4 rad and moved by `(0.75, 0.75)`, it stands just
off the block `[−1, 1]² × [0, 1]` at the corner `(1, 1)`, its flat chord
facing the corner. Three pairs refuse: the sliver's arc wall against
block faces. The wall's axis is the disc's centre `(0.75, 0.75)`, which
lies inside the block. So the offset from that axis to the block's
nearest segment (`census::depth_gap::toward`) aims across the block, not
between the two solids. The direction that parts them is the normal of
the sliver's flat face, a face that is not in the refusing pair. Arm 1's
`backstop_apart` proposes planar normals of the pair's two faces only.

## The lens

Block ∖ lens through severs the corner. The two pieces lie either side
of the lens's material, which is gone. Their walls are arcs of the
lens's two circles, and the pieces are parted by the line through the
lens's tips. The cylinder × cylinder pair has no planar normal, and
neither axis's offset is that line's normal.

## The general form

The boolean's narrow phase reads every planar face normal of both
operands (`boolean::separating::operand_axes`). The census could read
the planar normals of both refusing solids, but on a large body that
is a lot of directions per pair. Price that first, against reading the
normals of the faces adjacent to the pair's faces. The lens needs a
direction no planar face carries: the chord between an arc's ends is
one.

## Two walls whose nearest points are off their seams

`census::depth_gap::toward` aims the offset only at a cover's segments.
A cylinder wall's segments are its seam rulings, so between two
parallel walls the offset aims at the other wall's seam rather than its
nearest point. Two walls less than about `r²·sin²φ / 2D` apart then
stay undecided, where `φ` is the seam's angle off the line of centres.
A cylinder wall also has no least depth about an axis other than its
own, so the depth gap does not decide them either. The direction that
parts them is the line between the two axes; for two parallel axes it
is closed-form.

The witness is `editor-core`
`intent_s5_b_interference::an_undecided_pair_beside_the_press_fit_keeps_it_loud`:
a hole of radius 0.05, 5e-4 clear of the pin's wall on the diagonal.
That row *depends* on the pair staying undecided, so the lane that
parts it must re-fixture the row in the same PR. The row's doc says
how.

## Closed

Closed by PR 5858 (merged 2026-10-28). The verifier measured cone × rod and cone × cone pairs and found no pose the removed common normal would clear that the head refuses; no committed row witnesses that, filed below.

## Built

On `restread/arm1-third-face-directions`, arm 1 reads two more
families from the pair's own faces (`census::depth_gap::across`): each
arc's chord normal, and the line between the two carriers' axes or
centres. The sliver's arc chord is its flat's normal, so no third face
is read; the general form's normals of other faces were not needed by
any measured class. `j3r2_pocket_battery` goes from 12 `BAD` to 0, and
the INTENT row is re-fixtured to a hole 1.5 box pads clear.
