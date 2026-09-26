---
id: census-touch-cones-are-a-third-vertex-sector-builder
kind: issue
title: The census's touch cones (Cone::vertex) are a second vertex-neighbourhood sector builder beside boolean::sectors::build_sectors and splitting::neighborhood, and a third edge-convexity reader beside classify_dihedral and classify_material_pairing
status: open
opened: 2026-09-26
priority: P1
cost: D
---


Filed by CONTACT-1's dual-review fix pass (R1, R2), as one class. The
census's touch analysis (`crates/topo/src/census.rs`, `Cone::vertex`,
`Cone::wedge`, `Cone::within`) builds a vertex's sector fan and reads
edge convexity itself, beside two existing builders and two existing
convexity readers that have drifted from it:

- `crate::boolean::sectors::build_sectors` (`boolean/sectors.rs`) —
  the boolean's vertex-neighbourhood sectors: the same `sector_face` +
  `sector_shape` walk, different orientation bookkeeping (`start`/`end`
  bounds, chained entries) and an orbit/curved-carrier reach the census
  refuses (`Unreadable`).
- `crate::splitting::neighborhood` (`splitting/neighborhood.rs`, its
  orbit walk) — the splitting lane's twin of the same walk.
- `boolean/vtxfac.rs` (`classify_vertex_on_face`) — a vertex-on-face
  classification over the same neighbourhood.
- Convexity: `census_touch_dihedral` (decided in `Cone::fan`) is
  a third reader beside `geom_brep::classify_dihedral` (census.rs's
  crossing rung, `validate.rs` tier 3, `boolean/ops.rs`,
  `boolean/rim_wedge.rs`, `splitting/finish.rs`) and
  `geom_brep::classify_material_pairing` (census.rs, `validate.rs`,
  `boolean/rim_wedge.rs`).
- Naming: `Cone::within` (cone in a closed half-space) shares a name
  with `boolean::sectors::within` (a direction inside a sector) and
  means something else.

The fix is one vertex-neighbourhood reader both lanes and the census
consume, with the convexity read through the classifiers that already
exist. Difficulty D.
