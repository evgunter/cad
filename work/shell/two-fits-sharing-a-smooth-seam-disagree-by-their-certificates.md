---
id: two-fits-sharing-a-smooth-seam-disagree-by-their-certificates
kind: issue
title: two walls offset independently agree along a shared smooth seam only to the sum of their fit certificates, so a vase-class seam has no edge after a move; fitting them together is undesigned
status: open
opened: 2026-10-08
priority: P3
cost: H
refs: [a-fitted-wall-has-no-section-with-a-moved-cap]
---

Filed from the same designer pair. The vase's two walls meet along
tangent-continuous seams; offset independently, the two fits are tangent
there, so no transverse section exists and the edge must be a boundary
row both fits share — which two independent fits agree on only to the
sum of their certificates. Today's refusal is `FittedBoundaryUnsupported`
("a seam shared with another fitted face", `crates/topo/src/replace_face.rs`).

Owed: a design (fit walls together, or constrain each fit along the
shared seam) — a design fork for a designer pair. Related, unreached
today: `shell_open`'s lift of a cavity `Approx` wall back onto its NURBS
wall would be an offset of a fit (`Offset { base }` takes a
`NurbsSurface`, so a fit of a fit is unrepresentable); the expected
answer is that the lift restates the original NURBS surface.

## Measured

2026-10-09, after plane × `Approx` routed over the fit
(`a-fitted-wall-has-no-section-with-a-moved-cap`). The vase never reaches
its smooth seam, at ε = 1e-6, 1e-9 and 1e-12:

- `shell` moves a cap first, and the cap refuses at its rim's plane ×
  NURBS limb 2 on the rational wall (`RechartFalsifies { PlaneNurbs(Limb {
  HullSup, ≈ 4.8e-4 }) }`), as before;
- a wall moved alone refuses at its fit: `Fit { PatchBound(Crease) }`,
  its net carrying an interior multiplicity equal to its degree.

The guard's text this item quotes now reads "a row of this fit shared
with another fitted face" (`crates/topo/src/replace_face.rs:2034`), and
"… with a spline face" where the neighbour is an unmoved NURBS wall.
Pinned in `crates/sweep/tests/encl_curved_loft_shell.rs`,
`shelling_the_vase_refuses_at_its_rims_certificate`.

2026-10-10, after the iso-row arm narrowed
(`the-iso-row-arm-reads-a-u-moving-chart-image-as-a-u-row`): the
guard and both of its strings are gone. A row of a moving fit beside a
distinct spline or fitted face is not extracted:
`offset_derive::holds_the_move` answers false for a spline neighbour
rather than deciding the C7 normal-alignment margin along the seam,
which is this item's, so the seam takes the section route and refuses
`NeighborPairUnroutable` naming the pair (`Approx × Nurbs` beside an
unmoved wall) until a NURBS × NURBS arm, and then this item's seam
question, answer it.
