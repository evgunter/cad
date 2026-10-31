---
id: a-one-segment-cylinder-through-a-plate-tangent-to-a-hole-refuses-single-site
kind: issue
title: A one-segment cylinder through a plate, a hole tangent to its wall, refuses SingleSiteSectionLoop{count: 2} in every op where the two-arc form builds
status: open
opened: 2026-10-27
priority: P1
cost: M
---

Found by PR 5808's claims review, in the class of
`one-segment-tangent-rims-refuse-where-their-two-arc-forms-build`.

## The pose

A cylinder `r = 0.3`, `z ∈ [−1, 1]`, through the plate
`[−1, 1]² × [−0.5, 0]` holed through by `r = 0.1` tangent to the
cylinder's wall along the ruling through `(0.3, 0)`, outside it (about
`(0.4, 0)`) or inside it (about `(0.2, 0)`). With both vertices at the tangent azimuths, or the
cylinder's alone, the one-segment form refuses
`Join(SingleSiteSectionLoop { count: 2 })` in every op, while the
two-arc form builds; the same on `main` (the review's probe, `S4`
scenes).

## Why, as far as read

The cylinder crosses the plate's top and bottom faces, so each cut is a
one-site section loop at its rim's vertex, and the hole's wall is tangent
to the cylinder's along the ruling through both sites: the loops meet
an edge of another face there (`boolean/join.rs` `site_cell` reads
`SiteCell::Other` against the wall's rim on each side), which no arm of
`closes_at_its_site` takes for a transverse section.

## Built (branch cleave/one-segment-cylinder-through-a-holed-plate)

The reading above was the first of three causes, each measured with the
review probe on `main` (refusals reproduced at the default ε, `1e-6`
and `1e-12`):

- **The join had no arm for a wrap edge at a boundary vertex.** With
  the cylinder's vertex alone on the ruling, `site_cell` reads `Wrap`
  on the cylinder and `Other(top)` on the plate, and
  `closes_at_its_site` took a wrap edge only against a pierce. It now
  takes `(Wrap, Other(f))` on a planar `f` as it takes `(Wrap,
  Pierce(f))`.
- **A closed rim's germ was read along it.** With both vertices on the
  ruling the plate's cell read `Closed(rim)`: `sectors.rs` `germ_locus`
  asked the reduction's record at the rim's far end, which for a closed
  edge is the site's own vertex pair, so the germ ran `OnEdge(rim)`
  although the section circle only touches the rim. `germ_locus` now
  reads a closed edge's midpoint against the partner's face
  (`closed_on_partner`: on the plane, or the carrier's linearized
  residual within the band, `bool_germ_closed_on_carrier`), at the
  vertex pair and in the vertex-on-face pass alike; the germ runs in the
  face and the cell reads `Other(top)`.
- **The joiner's null strut.** The null edge at a boundary vertex hangs
  into the face; its second `mef` walls off an empty run, so the second
  chord ran back against the first and the face left holding the strut
  wound twice round the circle, the disc inside out (`SeamOrientation`
  in ∩ and A ∖ B, which keep the disc). `chord_join.rs` `strut_half`
  runs both chords from the strut's half at the ring's vertex, and
  `place_strut` reads the lone conic's face against the old face's
  outer loop (`RimSide::holds_outer`): inside, the circle encloses the
  hole, so that face becomes the face's ring and the hole's ring with
  the circle's is promoted to the crescent (`kfmrh_minting`, `mfkrh`,
  as a lone ring does). The same strut stood in the boss-over-a-hole
  joins PR 5808 built, where every op that builds drops the disc.

Measured against `main` at `c36ea15ee7` with the review probe
(`rev5808r`, every scene, rest and turned at the default ε, rest at
`1e-6` and `1e-12`): see the PR body for the outcome diff. Every S4
one-segment pose with a vertex on the ruling builds `SOUND` and passes
tier 3, but the op keeping both walls where only one vertex is there,
which refuses `EdgeInFace` with the `CurvedFace` gap (the wrap edge's
slit in the other wall). Pinned by
`crates/sweep/tests/a_cylinder_through_a_hole_tangent_to_its_wall.rs`
(holes `r = 0.1, 0.15`, outside and inside, four vertex poses, at rest
and turned).

Filed: `a-cylinder-through-a-hole-tangent-along-a-ruling-with-no-vertex-there-refuses-curved-pierce`,
`a-two-arc-cylinder-through-a-hole-tangent-inside-refuses-with-the-plate-first`.
