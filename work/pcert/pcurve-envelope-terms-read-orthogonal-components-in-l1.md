---
id: pcurve-envelope-terms-read-orthogonal-components-in-l1
kind: issue
title: pcurve envelope: the sphere pole meridian's orientation and the projected cone's half-angle drift sum orthogonal components, a lever up to √2
status: open
opened: 2026-10-12
priority: P3
cost: E
---


(PCERT implementer, the sweep of
`pcurve-envelope-terms-sum-the-cos-and-sin-coefficients-of-a-deviation`,
which fixed the `cos t`/`sin t` pairs; these two are the neighbouring
shape it could not match.)

## What

Two check-4 terms bound one vector by the sum of its components along
orthogonal directions, `|x| + |y|` for `‖(x, y)‖`:

- `incidence`, `SphereMeridian { pole: true }` in
  `crates/geom-brep/src/pcurve_cache.rs`: **orientation**
  `|b·e| + |σ·(a·e) + ‖b_r‖|`, the axial and the radial component of
  the `sin t` coefficient's defect.
- the projected envelope's cone arm in
  `crates/geom-brep/src/pcurve_cache/projected.rs`:
  `drift = |sin − s| + |cos − c|`, the stored half-angle's sine and
  cosine against the twin's, levered by the slant reach — the radial
  and the axial displacement of one point.

Each overshoots by up to `√2`. Both are rounding-scale on a minted
image, so neither is measured to cost a row; an attach-path image that
stands near the band off in both components would read up to `√2` of
it. The fix is `harmonic_sup_scalar` (the exact norm of the pair) at
each site, and a row whose two components put it in `(zero/√2, zero)`.
