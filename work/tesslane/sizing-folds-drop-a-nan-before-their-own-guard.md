---
id: sizing-folds-drop-a-nan-before-their-own-guard
kind: issue
title: sizing's window folds drop a NaN (sphere_face_widest_cos, cone_face_rho_max), cap_angular reads one as the cap, and require_swept_rectangle's refusal payload overwrites one
status: open
opened: 2026-11-16
---



## Finding

Found by the class sweep of
`work/tesslane/the-cylinder-certificate-reads-a-ruling-sided-triangle-as-zero.md`:
a `max`, `min` or comparison in `crates/mesh/src` that reads a NaN as a
number. None of these decides a triangle's verdict: each sizes a lattice
or fills a refusal's payload, and the per-triangle certificate still
decides. Read, not run: no body reaching any of them with a NaN is known.

- `sizing::sphere_face_widest_cos`: the face's height window is folded
  with `lo.min(z0)` and `hi.max(z1)`, which drop a NaN edge height
  (`geom_brep::harmonic_range` takes `e0.min(e1)` and `e0.max(e1)` inside
  too), so the guard two lines below, `lo.is_nan() || hi.is_nan()`, cannot
  see one: the window is the other edges'. Its last line,
  `(… / radius).min(1.0)`, reads a NaN quotient as 1, the widest cosine 0.
- `sizing::cone_face_rho_max`: `|v_min|.max(|v_max|)` and the fold
  `m.max(rho)` drop a NaN radius unless it is the only edge, and the
  largest radius is understated.
- `sizing::cap_angular` and `sizing::sagitta_step`: a NaN step or radius
  fails `step < MAX_ANGULAR_STEP` or `target < rho` and takes the cap, the
  coarsest step, where `sizing::ceil_count` would have refused it. On a
  curved face the certificate then refuses; an edge of a planar face has
  no certificate after it.
- `curved::require_swept_rectangle`: the refusal's `max_distance` is kept
  by `d.is_nan() || m.is_nan() || d > m`, so a later finite distance
  overwrites a NaN: the shape `cert::worse` exists to prevent. It is the
  payload of a refusal already being returned.

## Cure

Each fold keeps a NaN (`cert::worse`'s rule), and `cap_angular` refuses
one where it caps. A row per site with a NaN operand.
