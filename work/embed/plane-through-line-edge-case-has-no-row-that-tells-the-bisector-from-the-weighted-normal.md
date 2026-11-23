---
id: plane-through-line-edge-case-has-no-row-that-tells-the-bisector-from-the-weighted-normal
kind: issue
title: plane_through's one-line-edge case has no row that tells the heading bisector from the old weighted normal; across the sweep and topo suites the two disagree only at a wedge-0 kiss, where neither separates
status: open
opened: 2026-11-01
priority: P3
cost: E
refs: [departure-graph-separator-weighs-each-heading-by-its-cells-offset-along-the-edge]
---

## Finding (HEADING, 2026-11-01)

HEADING moved `topo::injective`'s `plane_through` one-line-edge case
(`[d] => …`, `crates/topo/src/injective.rs`) to `bisector`, as it did
`graph_of`. `graph_of`'s change has a row that refuses before and holds
after (`a_twisted_loft_holds_along_its_seams_at_both_scalars`);
`plane_through`'s does not, and no argument that no supported input
reaches it holds either.

**What reaches it.** A probe in the `[d]` arm, over the whole `sweep`
and `topo` suites (debug, default eps), counted 1707 visits. It also
asked whether the old normal `unit(perp(hx, d) − perp(hy, d))` and the
bisector each put the two headings on opposite sides:

- 1659 are between two cones or two cylinders: frusta, rods, drums, the
  tee, the vessel and the kiss. The probe did not record which of them
  are one face against itself across its seam line (`certify_self`),
  where the headings are opposite and both forms give the same plane.
  1641 of them agree.
- 48 are a cap rim against a loft wall (`the_half_turn_helixs_caps_…`)
  where the cap's carrier arm did not decide. The two forms agree.
- 18 disagree. All are in
  `a_declared_wedge_0_kiss_refuses_naming_the_second_order_arm`, at the
  kiss's strut, where the dihedral is 0. The row passes under both
  forms: its refusal names the second-order arm.

**Why no row was built.** To reach the arm with an acute dihedral, two
faces with no deciding carrier arm have to share a straight edge.
Neither the spline pyramid nor the sheared spline box (flat bilinear
charts, Euler-op lines) came out as a finished body with a window:

- A triangle's diagonal is not an iso of its chart, so
  `Side::windows` refuses the face ("a free-form face with a boundary
  edge that is not an iso of its chart").
- On a quadrilateral, the iso image `p0 + pl·t` with `pl = 1/L` ends
  exactly at the chart's `1` only where `L` is a power of two. A
  slanted edge with dyadic ends cannot have that length, so at
  `Interval` the at-rest gate's rectangle lane refuses it ("a NURBS-face
  pcurve endpoint is not exact structure").

A flared two-section loft (cap–wall rims at 22°) never reaches `[d]`:
the cap's plane decides.

**Also:** with `graph_of` already the bisector, reverting
`plane_through` alone cannot red a row, because `graph_of` runs next on
the same line edge with the same separator. A row for this arm has to
be one where `graph_of` returns `None`, or a unit test that calls
`plane_through` directly.

