---
id: chord-join-copies-only-line-and-circle-edges-and-ends-an-ellipse-as-an-invariant
kind: issue
title: chord_join's along_edge_spec and closed_edge_spec end an ellipse edge as a SectionInvariant, which a lane choice's general form reaches; chord_section_loop ends an in-band Zero the same way
status: open
opened: 2026-10-20
---


Filed by PCERT E1's fix pass (PR 5500, P6), from its census of the
`SectionInvariant` producers. A D3 lane choice (`docs/DESIGN.md`,
`[verdicts.consumers]`) ends as no defect; these two sites can end one
as a kernel-invariant break.

## What

- `crates/topo/src/chord_join.rs:4143` (`along_edge_spec`, the `_`
  arm): a section segment along an edge whose carrier is neither a line
  nor a circle refuses `SectionInvariant` ("only lines and circles are
  copied"). `crates/topo/src/chord_join.rs:4169` (`closed_edge_spec`):
  a closed segment edge that is not a circle refuses the same way.
  The join's operand gate admits ellipse edges (`boolean/join.rs:2512`),
  and whether a plane cuts a cylinder or cone in a circle (the axis-normal
  closed form, at Zero) or an ellipse (the tilted general form) is a
  section-formula lane choice: a cut whose class reads in band is stored
  as an `Ellipse`, and a later germ along that edge reaches this arm
  where a circle would have been copied. Not driven to a body here; the
  path is read off the gates.
- `crates/topo/src/chord_join.rs:1373` (`chord_section_loop`): which
  loop of its section a chord's site rides is a structural act, but a
  Zero there refuses `SectionInvariant` rather than the decision's own
  lever. The sibling in-band straddle of `split_conic_inplane_mid` is
  `work/honejoin/join-adjacency-escalation-surfaces-as-a-section-invariant.md`.

## Fix

Copy any certified carrier over its sub-span along an edge (the copy
does not depend on the carrier's kind): generalise `arc_along` to an
ellipse, and let `closed_edge_spec` take a closed ellipse. Where a kind
genuinely has no copy, refuse typed "not yet" (as
`SplitJoinError::RunImageNotYet` does). `chord_section_loop`'s Zero
refuses with the decision's lever. Each needs a row through a body.

## `along_edge_spec`, landed elsewhere (SPLITDOOR, 2026-10-28)

`splitdoor/tilted-plug-rosters` took the first half of the Fix: past a
line and a circle, `along_edge_spec` copies the edge's own spec over
its span (`EdgeCurve::restated_spec`), run back where the chord runs
against it (`chord_join::run_back`). Still open here: a scaffold edge
that is neither a line nor a circle has no reversal (`run_back` refuses
it `SectionInvariant`), `closed_edge_spec` still copies only a circle
(with `cleave/one-segment-ellipse-rims-tangent-at-a-point-refuse-where-circles-build`),
and `chord_section_loop`'s Zero.
