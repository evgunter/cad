---
id: LIB-POLYGON
kind: unit
title: the lattice-backed façade polygon door
status: review
opened: 2026-09-08
branch: lib/polygon
refs: [facade-polygon-door-demoted-without-replacement]
---


Builds `pncad::authoring::polygon(&[(f64, f64)], tol) -> Result<ProfileLoop<T>,
PathError<T>>` — Ev's ruling (A) on PR 2017 — spelled through the PATHS lattice
exactly as the tour's deleted `path_polygon` spelled it: `Open.at(p0)`, a
`line_to` per vertex, `line_to(Start)` as the seam. A within-band-tangent or
cusped corner therefore refuses AT AUTHORING, and the emitted loop is the raw
vertex table (bulge 0, no declared joints).

**Two cross-fence touches, both ruled:**

- `crates/profile` — one new `PathError` arm, `PolygonTooFewVertices { given }`,
  with its `PathErrorKind` mirror, `Display` sentence and `kind()` row. Ruled
  cross-fence vocabulary change (Ev, PR 2017); the only edit in that crate.
- `demos/` — the tour's `paths.rs` helper deleted and its call sites moved onto
  the door. Render-lane touch, no scene change, no frame moved.
