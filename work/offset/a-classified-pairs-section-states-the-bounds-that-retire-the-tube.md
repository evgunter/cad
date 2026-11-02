---
id: a-classified-pairs-section-states-the-bounds-that-retire-the-tube
kind: unit
title: Conic build unit 3: a classified pair's section function states the crossing-angle floor and the overlap clearance for its pose, and the certificate owes no tube where it does
status: closed
opened: 2026-10-25
priority: P0
cost: M
parent: offset-lane-has-no-conic-carrier
closed: 2026-10-28
pr: 5897
---


**Parked (2026-10-25, PR 5692's dual review).** This unit is what lets a
steep open-conic arc rest in a body, and `edge_extent` levers such an
arc by its chord, up to ten times short of its diameter, which can read
a Positive margin as a definite Zero in the stand-in, coincidence and
tangency predicates that consume it. It waits until that arm is a true
bound on the arc.

**Un-parked (2026-10-27).** The edge extent is now two true bounds,
`geom_brep::edge_extent_inf` and `geom_brep::edge_extent_sup`, and each
consumer levers at the side its acted-on verdict needs: a reading that
acts on a Zero (the wedge, the census's corners, the rest locus's ball,
the contact verifier, the fate cut's corner) at the bound from above, a
certificate's transversality, tube and roots at the bound from below
(`edge-extent-reads-a-steep-nurbs-arcs-chord-four-to-ten-times-short`'s
`## Closed`).

Ev's ruling on PR 5652, C2 of `crates/geom-brep/README.md`: limb 3 is
owed unless the pair's section function states, for the pose, (a) a
lower bound on sin θ over the carrier's stretch of the section and (b)
the clearance of the ε/sin θ_min tube from every other piece of the two
surfaces' ε-overlap (other components, near-miss regions such as a
plane's apex neighbourhood at small stand-off). Plane×plane, ×cylinder,
×sphere, ×cone and equal-radius cylinder×cylinder state them in closed
form (likely); the torus rows owe the tube unless their section can.

**Lands last and alone, by design.** Ev approved with a reservation
("i'm still not totally sure about having special cases for the parabola
and hyperbola"), and both designers called this exemption severable. If
Ev drops it, this row closes unbuilt and the arcs wait on CHARTTAIL's
`the-cones-rung3-certificate-has-no-metres-enclosure`, as the other cone
routes do. Ask before building if anything Ev says later bears on it.

## Closed

Built on branch `offset/tube-exemption-by-pair`, severable as the ruling
asked: one place states the bounds, one place reads them.

- **The statement.** `geom_brep::intersect::tube_bounds`
  (`crates/geom-brep/src/intersect/section_bounds.rs`), in certification
  arithmetic: for plane×plane, plane×cylinder, plane×sphere, plane×cone and
  equal-radius cylinder×cylinder (one stored centre and radius), the
  closed-form floor on `sin θ` over the carrier's stretch (the section
  inside the carrier's control hulls, padded), the factor `m` of each
  surface's `Q = h·m`, and a moat check — no point of the ε-overlap lies
  between `8ε/sin θ` and `16ε/sin θ` of the section's component — with a
  Poincaré–Miranda certificate of a section point beside the carrier's
  witness. The torus rows, and every other pair, state nothing.
- **The reader.** `edge_nurbs::analytic_rung3`: limb 3 is skipped where the
  bounds are stated; otherwise the tube runs as before (`NoTubeEnclosure`
  for a cone).
- **What rests.** The axial door mints a moved cone/cap hyperbola between
  the moved corners (`ConicBranch::arc`, now minted at any scalar), and the
  conical wedges at a quarter and a third of a turn and the three-quarter
  turn frustum hollow to closed-form volume; a cone cap with a rational
  rim takes the certified rim lane about its apex
  (`geom_brep::props::spiric::cone_cap_rounds`). No door builds a parabola
  arc (an axis-parallel cap cuts only hyperbolas); its bounds are
  certified at the certificate.
- **What still owes the tube.** A stretch reaching the apex's height, a
  near-tangent pair whose second component reaches the moat, the
  cylinders' crossing, the torus rows.

Residue filed: `a-cone-wedges-apex-corner-beside-two-moved-caps-is-read-as-a-pole`,
`doors-that-read-the-plane-cone-section-do-not-carry-its-open-conic`,
`a-family-scalars-cone-pose-too-wide-for-the-tube-bounds-refuses-rather-than-bisecting`.
Claimed and closed: SHELF's
`partial-cone-frustum-three-quarter-turn-refuses-edge-disagreement`.
