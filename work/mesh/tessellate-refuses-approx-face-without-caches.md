---
id: tessellate-refuses-approx-face-without-caches
kind: issue
title: mesh::tessellate refuses an Approx-faced body whose half-edges carry no stored pcurve cache
status: open
opened: 2026-09-04
refs: [1758]
---


Found by a SHELL-2 reviewer's end-to-end consumer (2026-09-04): an
`Approx`-capped box built through the public doors (storage door,
`set_face_surface`) tessellates with "NURBS-face half-edge carries no
stored pcurve cache" — the same wall `mass_properties` reports. The
cache cannot be minted for the cap's straight-carrier edges (the
iso-line seam class refuses a non-spline carrier), so today the only
`Approx`-faced body that meshes is the loft, whose walls were minted
with their caches. Recorded so the mesh lane knows the class exists;
whether the fix is a cache mint for straight carriers on an `Approx`
chart or a tessellation arm that reads the description directly is
the mesh program's call.

Home: `crates/mesh` (S-MESH).
