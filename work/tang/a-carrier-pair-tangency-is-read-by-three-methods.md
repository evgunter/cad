---
id: a-carrier-pair-tangency-is-read-by-three-methods
kind: issue
title: A plane-sphere or sphere-sphere touch point and a plane-cylinder or cylinder-cylinder tangent line are each read by three methods
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`), which tests every geometric pair question against C5's rule: one total kind-pair table, one method per pair, no runtime fallback (`crates/geom-brep/README.md`, C5).

## What

- **Point touch (plane×sphere, sphere×sphere)** is read by:
  - `intersect::{plane_sphere,sphere_sphere}_section`'s `TangentPoint` (`crates/geom-brep/src/intersect.rs`);
  - `census::point::face_face` (`crates/topo/src/census/point.rs:313`), on its own private `Surface{Plane,Sphere}` enum with `pm_census_point_*` predicates and `_ => None` for every other kind. Its module doc admits the overlap;
  - `section_cert::{sphere_plane, sphere_sphere}` (`crates/topo/src/boolean/section_cert.rs`).
- **Tangent line (plane×cylinder, parallel cylinder×cylinder)** is read by:
  - `locus::closed_form_locus` (`crates/geom-brep/src/locus.rs:232`), whose `_ => Unsupported` is typed;
  - the `TangentLine` outcomes of `plane_cylinder_section`/`cylinder_cylinder_section`;
  - `section_cert`'s `Section::Tangent`.

## Owed

The tangency outcome is one arm of the pair's section, so it is read from the one surface-pair table (see `hone/surface-pair-sections-are-classified-by-tables-beside-route`), not decided again beside it. The second-order wedge duplicate (`dihedral::tangent_second_order` against `contact_verify`'s own) is already on `pred/lever-arm-fold-six-hand-rolled-siblings` (issue 1439) and is not refiled.
