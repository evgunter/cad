---
id: ssi-section-and-box-cut-relate-nets-to-vectors-by-count
kind: issue
title: ssi: Pieces::enclosed / SectionReader::of and NurbsBoxes::cut / bezier_on relate a control net to its vector or span by count
status: open
opened: 2026-10-10
---


Found by the `coefficient-vector-pairing-survivors` fix-pass sweep
(NURBS, PR 4485): the shape that unit closed elsewhere — a coefficient
array beside a knot vector, or beside a span, related by length alone.

- `crates/geom-brep/src/ssi/section.rs` `Pieces::enclosed(kv, control,
  weights, …)` and `SectionReader::of` (its `pub(crate)` door, called
  from `ssi/one_arc.rs` with a row curve's parts): the homogeneous
  channel `h` and the weights `w` are built beside `kv` with a
  truncating `zip`, so a short `control` or `weights` silently drops
  coefficients before `CurvePlan::apply_certified` reads them.
- `crates/geom-brep/src/ssi/enclose.rs` `bezier_on(line: &[[Interval; 4]],
  span: Span, …)`: a slice beside a `Span`, refused when
  `line.len() != p + 1`. Its caller `NurbsBoxes::cut` reads `pts` and
  `nv` beside a `SurfaceWindow`, the count relation held by the
  struct's construction.

Disposition: take the curve (whose constructor checked the counts) or a
pair minted once at the door — `RationalCoeffs` for the section's
`(h, w)` lines, a `CoeffWindow`-shaped window of the four-channel line
for `bezier_on` — and delete the `zip` truncation and the
`len != p + 1` refusal with it.
