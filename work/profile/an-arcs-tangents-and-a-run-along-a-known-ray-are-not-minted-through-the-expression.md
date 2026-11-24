---
id: an-arcs-tangents-and-a-run-along-a-known-ray-are-not-minted-through-the-expression
kind: issue
title: An arc's chord tangents, and lines pushed along a known ray, do not go through the direction mint that asks the expression: a chord along an axis keeps a quotient's width there
status: open
opened: 2026-11-23
---

Found by the review of BOXFINISH's expression-question unit (PR 7488),
which put one mint (`crates/profile/src/seg.rs` `axis_or`) behind the
`line_to` verb, an arc verb's leg lowered to a line
(`crates/profile/src/path.rs` `arc_angles`) and validation. The mint
stores the axis, with no width, where a component of the chord is zero
as an expression. These directions are derived from a chord and do not
go through it:

- **An arc's tangents from its chord**: `chord_tangents`
  (`crates/profile/src/path.rs`) turns `chord_director(d).unit` by half
  the sweep, for every arc verb's leg (`arc_angles`, the `Some` arm)
  and for the bulge-shaped carriers
  (`crates/profile/src/path/family.rs` `bulge_carrier`, and the tangent
  mode beside it). A chord along an axis over a box carries the
  quotient's width into both tangents. The turned ray is not an axis,
  so the fix is not the hop's: the chord's ray would be minted, then
  turned.
- **A line pushed along a ray the path already holds**:
  `emit_straight_leg_at` (`to(point)` on a directed leg),
  `push_straight_run_in` (a fillet's run in) and `end_side_at` push a
  line whose direction the path knows (`ang`, the run's ray) and record
  nothing on the loop, so validation mints the stored director from the
  chord. Where the chord's component is an identity the tier proves,
  the two agree; where it is not, the stored director is the chord's
  quotient and the ray the author gave is not consulted.

Neither changes which documents build on the corpus (the slow sets of
`profile`, `sweep`, `topo` and `editor-core` are the same colour before
and after PR 7488). What would close it: each of these names what it
holds (the minted chord ray, or the authored ray) and records it, as
`line_to` does (`Core::direct_leaving`).
