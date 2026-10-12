---
id: blend-support-ring-meters-read-two-ways
kind: issue
title: blend: the support ring meters (ring_clearance_pass arms a, b, d) read a ring wholly inside the strip a band replaces as a RingClearance crossing, two ways where C8 reads three
status: open
opened: 2026-10-12
---


## Finding

Found by the class sweep of the predicate-2 three-way reading (step 1
of `ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver`).
Predicate 2's reach (`crates/sweep/src/blend/reach.rs`, `band_reach`)
and the cut-off's end-face meter (`ring_clearance_pass` arm (c),
`crates/sweep/src/blend/surgery.rs`) now read a feature three ways:
clear, wholly inside the band's material (`BlendError::WhollyInside`),
or crossing it (a measured `FaceClearance` / `RingClearance`).

The support meters still read two ways. In `ring_clearance_pass`:

- arm (a), open links: each ring of a support face against the link's
  trimline, through `piece_along` — a ring whose lowest reach is past
  the trimline refuses `RingClearance { bounded: false }` whether it
  straddles the trimline (crossing) or lies wholly in the strip
  between the edge and the trimline (wholly inside the material the
  band removes or fills);
- arm (b), rims (`support_boundary_clearance`): the same, against a
  rim's trim circle;
- arm (d), planar strips (`strip_clearance`): an outer-boundary edge
  against the strip's bounding rectangle, refused as
  `FaceClearanceUncertified`.

A bore through a support that lies wholly inside the strip goes with
the material under Ev's ruling (PR 4285); today it refuses
`RingClearance` worded as "lies in the part of a face the blend cuts
away", which is true of both readings and tells neither. Step 4 of
that unit's build order (the planar band, the concave fill, the
chamfer) is where a consuming carve would need these meters to tell
the two apart; until then the refusal is correct, only two-valued.

Whether a support ring wholly inside the strip is reachable before
the battery's support screen (`FaceClearanceUncertified`,
`crates/sweep/src/blend/battery.rs`) refuses it is not measured here.
