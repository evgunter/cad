---
id: a-carrier-pair-tangency-is-read-by-three-methods
kind: issue
title: A plane-sphere or sphere-sphere touch point and a plane-cylinder or cylinder-cylinder tangent line are each read by three methods
status: open
priority: P3
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`, #5078). The rule it tests is D3's "One reading per question" (`docs/DESIGN.md`, D3): every decision about carriers is a reading (of one carrier against a point, of a carrier pair's pose, or of a pair outcome that takes the pose); each reading is one exhaustive match with one method per arm and no fallback, and every consumer reads it. C5 (`crates/geom-brep/README.md`) is that rule's instance for surface and curve pairs.

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

Re-aimed to D3 "One reading per question". The earlier target, "the tangency outcome is one arm of the pair's section", has moved: point touch and line tangent are classes of the pair's *pose*, decided once by the pose reading. The touch point and the tangent line are the section outcome's witnesses, and the outcome takes the pose reading's answer rather than deciding tangency itself.

- The pose reading decides point touch (plane×sphere, sphere×sphere) and line tangent (plane×cylinder, parallel cylinder×cylinder).
- `census::point::face_face` (its private enum and `pm_census_point_*` predicates), `locus::closed_form_locus` and `section_cert`'s `Section::Tangent` each read that pose reading and the outcome that takes it. None of them decides the tangency again.
- A consumer that serves fewer pairs matches on the pose reading's answer and refuses the rest typed (the `_ => None` in `face_face` goes).

The second-order wedge duplicate (`dihedral::tangent_second_order` against `contact_verify`'s own) is already on `predtail/contact-verify-spells-the-orientation-and-second-order-margins-beside-their-homes` (cut from issue 1439's lever-arm row) and is not refiled.
