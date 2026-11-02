---
id: a-fallback-cut-refuses-beside-a-ring-a-seam-or-a-pinch
kind: issue
title: The containment fallback's fate cut refuses beside a ring, across a curved seam or at a pinch, and leaves the dropped face whole
status: open
opened: 2026-10-25
priority: P3
cost: H
refs: [5706]
---

## What

PR 5706 has the containment fallback divide its dropped coincident
faces through the section path's own `fate::cut`
(`crates/topo/src/boolean/ops.rs`, `fallback_fates`), one face at a
time. Where a face's cut meets one of the cut's placement limits
(`fate::Limit`), the result is left as built, since the
fallback's body never reads the dropped clone. That face alone is
restored: no `BooleanNaming::substituted` or `discards` row names it,
and a `BooleanNaming::uncut` row carries the refusal. The naming then
says nothing of which part of the face the kept face holds. No reader
reads `uncut` yet (BORDERS' `no-reader-reads-a-fallbacks-uncut-faces`).

Measured on PR 5706's head, topo + sweep + editor-core under the `ci`
profile at ε 1e-9: five refusals, in four tests, all
`ClassificationInvariant`:

- `sweep` `a_closed_conic_in_the_partners_face::a_boss_beside_a_ring_and_over_a_hole`,
  twice, and the review probe `boss ∖ (holed plate ∪ lid)`, whose lid
  cuts: "a fate cut's end lies in no corner of the dropped face". This
  is the boss `r = 0.3` over a plate whose top holds a hole's ring
  `r = 0.1` inside the rim. The same pose is pinned by
  `the_fallback_cuts_a_closed_rim_or_records_why_not`.
- `sweep` `an_annular_tube_through_a_plate::a_plug_flush_in_a_bore_is_sided_by_its_caps`,
  the plug over z 0..0.5: the same words. The kept rim runs in the
  bore's cylinder wall to a vertex on its floor rim.
- `topo` `pierce_strut_at_a_pinch::a_pinch_standing_on_a_face_builds_in_every_op`,
  `pinch − B`: "a ring of a face a fate cut divides is placed at none
  of its vertices". The ring is a lone vertex on the point key of a
  vertex of the new face (a pinch), so `rehome` reads `OnVertex` at its
  only vertex.

On the section path these poses meet the join first, which shapes the
dropped face (`join`'s `one_site_segments`, `LoneRing::Closed`) or
refused typed, until the carrier ladder's cone arm let the cylinder
and cone straddles build
(`a_closed_edge_in_a_cylinder_or_cone_face_holds_every_op`).
The fallback asks `cut` alone. The section path's `boss/holedlow ∩`
(the PR 5706 review corpus) refuses at the same corner, on both trees:
the cut now divides its two closed rims, then stops at a chord end on
the hole's rim.

## Shape of a fix

Teach `fate::cut` each pose, and leave the fallback as it is. The
`uncut` rows then empty, and `the_fallback_cuts_a_closed_rim_or_records_why_not`'s
holed case moves to a cut:

- **The corner reading** (`fate::places`): a closed kept edge leaving a
  vertex that a ring of the dropped face passes through.
- **`rehome`**: a ring whose one vertex shares the new face's vertex's
  point key.


## The ring pose cuts at a true corner lever (OFFSET, 2026-10-27)

`fate::leaving` levered a corner's readings (`corner_holds`, the tip
turn) at each bound's chord, for every carrier kind. A closed bound (the
hole's ring `r = 0.1` here) has a chord of zero, so every side read Zero
and the rim's vertex read in no corner. OFFSET's
`edge-extent-reads-a-steep-nurbs-arcs-chord-four-to-ten-times-short`
(PR 5875) levers it at `geom_brep::edge_extent_sup`, the ring's
diameter, and the boss-over-a-hole pose now cuts: the holed plate's top
and the lidded case's top are held in part and nothing is uncut. The
pinning row is renamed
`a_closed_conic_in_the_partners_face::the_fallback_cuts_a_closed_rim_beside_a_ring`.
That PR did not measure the plug-in-a-bore pose or the pinch's `rehome`
pose, so this item stays open for them.
