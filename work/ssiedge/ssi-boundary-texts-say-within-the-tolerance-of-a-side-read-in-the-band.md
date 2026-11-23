---
id: ssi-boundary-texts-say-within-the-tolerance-of-a-side-read-in-the-band
kind: issue
title: ssi: the boundary pass's texts say a side lies within the tolerance of the plane, but it is read within the band
status: open
opened: 2026-10-11
priority: P3
cost: E
---


## What

Three texts in the plane × NURBS boundary pass say a wall side "lies
within the tolerance of the plane":

- `SsiError::BoundaryTangent`'s Display (`crates/geom-brep/src/ssi.rs`,
  "ssi: {side} lies within the tolerance of the plane, …");
- `SsiBoundaryContact::Side` and `SsiBoundaryContact::Corner`'s Display
  (`crates/geom-brep/src/ssi/boundary.rs`, "a region along {side}, which
  lies within the tolerance of the plane …").

Each is raised on a side `boundary_section` returned
`BoundarySection::On` for (`ssi/boundary.rs` `Pass::run` →
`side_region`), and `On`'s own doc (`ssi/section.rs`
`BoundarySection::On`) says the side's certified sup distance from the
plane "is not decided positive": that covers the ambiguity band
(ε, Kε], not only ε. So on a side whose sup lies in the band the text
claims a nearness the kernel did not find.

Read from the tree, not run: measure first (a NURBS wall with one side
offset from the plane by 2ε–9ε, at every ε row).

The text should say what was read: within the band, or name the sup
margin, as the split's vertex refusal now does ("whether a vertex lies
on the split plane is undecided: margin … lies inside the ambiguity
band …", branch `cleave/band-on-pinch-residue`).

Found by the CLEAVE lane sweeping "within tolerance" refusal texts for
the split's `SliverVertex` fix (2026-10-11).
