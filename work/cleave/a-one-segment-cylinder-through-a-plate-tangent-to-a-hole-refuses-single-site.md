---
id: a-one-segment-cylinder-through-a-plate-tangent-to-a-hole-refuses-single-site
kind: issue
title: A one-segment cylinder through a plate, a hole tangent to its wall, refuses SingleSiteSectionLoop{count: 2} in every op where the two-arc form builds
status: closed
opened: 2026-10-27
priority: P1
cost: M
closed: 2026-11-01
branch: cleave/one-segment-cylinder-through-a-holed-plate
pr: 6177
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

## Why

Three decisions, each measured with the review probe on `main`, where
every op refused at the default ε, `1e-6` and `1e-12`:

- **The join had no arm for a wrap edge at a boundary vertex.** With
  the cylinder's vertex alone on the ruling, `boolean/join.rs`
  `site_cell` reads `Wrap` on the cylinder and `Other(top)` on the
  plate, and `closes_at_its_site` took a wrap edge only against a
  pierce.
- **A closed rim's germ was read along it.** With both vertices on the
  ruling the plate's cell read `Closed(rim)`: `boolean/sectors.rs`
  `germ_locus` asked the reduction's record at the rim's far end, which
  for a closed edge is the site's own vertex pair, so the germ ran
  `OnEdge(rim)` although the section circle only touches the rim.
- **The joiner's null strut.** The null edge at a boundary vertex hangs
  into the face; the `mef` at its free end walls off an empty run, and
  the second chord, run back as for two sites, left the strut's face
  winding twice round the circle and the disc inside out
  (`SeamOrientation` in ∩ and A ∖ B). With the hole inside the circle,
  the face the circle walls off alone holds the old face's outer loop,
  which `mef` cannot place.

## Closed

2026-11-01, PR 6177.

- `join.rs` `closes_at_its_site` takes `(Wrap, Other(f))` on a planar
  `f` as it takes `(Wrap, Pierce(f))`.
- `sectors.rs` `closed_reach` is the one reading of a closed germ edge
  against the partner, for `germ_loci`'s one-sided and two-sided
  branches and the vertex-on-face pass: the edge's own carrier on each
  face's surface (`slit.rs` `carrier_on_surface`, shared with the rider
  reading, at points spread over the whole turn; nine on a torus), and
  on a plane its midpoint in the trim. A pair that reading does not take
  refuses `GermEdgeCarrierUnsupported`; in the face the germ runs in,
  a closed edge outside the trim is a `ClassificationInvariant`.
- `chord_join.rs` `Lone` / `strut_of`: a null strut's two chords both
  run from its half leaving the ring's vertex, and `place_strut` reads
  the lone face against the old face's outer loop
  (`RimSide::holds_outer`): it is the disc the circle encloses, or the
  hole the circle cuts, made a ring of the old face, the hole's ring
  with the circle's promoted to the crescent (`cut_hole`,
  `promote_ring`, shared with `join_lone_ring`). The enclosed region
  takes the old face's rings it holds. A strut's face is a plane, or
  the lane's invariant refuses.

Every one-segment pose with a vertex on the ruling builds `SOUND` and
passes tier 3, but the op keeping both walls where only one vertex is
there, which refuses `EdgeInFace { gap: CurvedFace }` (that operand's
wrap edge lying along the other's wall). Pinned by
`crates/sweep/tests/a_cylinder_through_a_hole_tangent_to_its_wall.rs`
(holes `r = 0.1, 0.15`, outside and inside, four vertex poses, at rest
and turned, and beside two more holes), at the default ε, `1e-6` and
`1e-12`.

What stays open, all reached past the join that refused first on `main`:

- the floors flush with a curved wall piece whose edges all lie on the
  other boundary refuse `SectionLoopUndecided`:
  `work/honesplit/the-uncut-shell-witness-reads-no-curved-face-interior.md`;
- a closed rim flush in its partner's face, tangent inside its boundary
  at its vertex, `SingleSiteSectionLoop { count: 1 }`:
  `a-flush-closed-rim-at-its-vertex-is-joined-reversed`;
- one-segment ellipse rims, now `SingleSiteSectionLoop` in both poses:
  `one-segment-ellipse-rims-tangent-at-a-point-refuse-where-circles-build`;
- no vertex on the ruling:
  `a-cylinder-through-a-hole-tangent-along-a-ruling-with-no-vertex-there-refuses-curved-pierce`;
- the two-arc form refusing in one operand order:
  `a-two-arc-cylinder-through-a-hole-tangent-inside-refuses-with-the-plate-first`.
