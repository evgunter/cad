---
id: role-resolution-interior-tiers-certify-only-planar-region-faces
kind: issue
title: Role resolution's region-interior and vertex-chord tiers read the region face through the planar-only face_plane, so a curved region face refuses KindUnsupported naming a kind the containment door does have an arm for
status: open
opened: 2026-09-25
priority: P1
cost: D
---


Found by CONTACT-2's sweep for the planar-carrier premise.

## What

`boolean/join.rs` `resolve_roles_geometric` probes the regions flanking
a seam in four anchor tiers. Tiers 3 and 4 (`Anchor::RegionInterior`,
`Anchor::RegionVertexChord`) certify a candidate interior to its region
face through `point_in_face`, taking the projection normal from
`solid_contain::face_plane` — which answers only for a `Plane` and
returns `PointInSolidError::KindUnsupported { kind }` for anything else.
A curved region face that reaches tier 3 therefore refuses
`Containment(KindUnsupported { kind: Cylinder })`, and
`KindUnsupported`'s own docs say the containment door HAS a cylinder
arm, so the sentence sends a reader to the wrong capability.

## Reproducer

On CONTACT-2's tree with only its chord-join fix,
`rod − brick((-1, 1), (0, 1), (-1, 5))` — rod `r = 0.5` over
`z ∈ [0, 4]`, an extruded circle — reached it: every vertex and every
chord midpoint of the flanking regions sat on the cutter's face `y = 0`
(a semicircle's chord midpoint is the circle's centre). CONTACT-2 added
a tier between the chord midpoints and tier 3 that probes each CURVED
edge at its carrier's parameter midpoint (`Anchor::EdgeOnCarrier`),
which resolves that pose, so the tree has no reproducer now; tiers 3–4
are still planar-only for any curved region whose earlier candidates
all read `OnBoundary`.

## A second premise beside it

Tier 2 probes a curved edge's CHORD midpoint uncertified, as if it were
on the region boundary; for a conic it is not (it is inside the
operand for a convex arc, outside it for a concave one). CONTACT-2
first replaced it with the on-carrier point and three rows moved to the
tier-3 refusal (`curved_mergedoor`
`floating_and_mid_bore_pegs_refuse_at_the_zip_seam_chord_today` and
`consumed_side_of_the_pair_is_gone_and_one_record_ships`,
`r1_probes_m9_3` `probe_partial_engagement_never_silent`): there the
rim lies ON the other body's bore wall, so the on-carrier point reads
`OnBoundary`, and it is the off-edge chord midpoint that decides. So
the chord midpoint was kept and the on-carrier tier added after it. The
question for the taker is whether an off-region point is a sound
witness for the region's role — the tier docs promise "never
classification by guess".

## What the taker owes

A curved-face interior certificate for tiers 3–4 (a chart-space
`point_in_face`), or a typed refusal of their own that names the region
kind as the tier's gap rather than as the containment door's; and a
ruling on tier 2's off-edge witness for curved edges.
