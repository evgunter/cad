---
id: surface-pair-sections-are-classified-by-tables-beside-route
kind: issue
title: Surface-pair sections are classified by five tables beside intersect::route, which only replace_face consults
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`), which tests every geometric pair question against C5's rule: one total kind-pair table, one method per pair, no runtime fallback (`crates/geom-brep/README.md`, C5).

## What

`intersect::route` (`crates/geom-brep/src/intersect.rs:189`) is the one exhaustive
`(SurfaceKind, SurfaceKind)` table C5 names. In the tree, only `replace_face`
consults it (`crates/topo/src/replace_face.rs`, ~l.2130, 2574, 2924). Five other places answer
"what is the section of these two carriers" with their own tables, and each ends in a wildcard:

- `section_cert::classify` (`crates/topo/src/boolean/section_cert.rs:629`):
  - torus and cone sub-tables plus cylinder, sphere and Nurbs×plane arms, with its own predicates;
  - `_ => Section::Intractable`;
  - it serves pairs `route` marks unimplemented (cone×cone, cone×sphere, cone×torus,
    sphere×torus, torus×torus, cylinder×torus).
- `join::kind_arm` (`crates/topo/src/boolean/join.rs:273`) and `pair_section_frame_at` (`:2813`), with `frame_extent` beside them:
  - a second kind table;
  - `_ => KindArm::None`, which `bool_connect` (`:957`) reads as "try the face-pair arm, else the edge-plane lane" (a runtime fallback);
  - a cylinder pair with skew axes drops through at runtime.
- `offset_derive::section_closed` (`crates/topo/src/offset_derive.rs:518`):
  - plane×{plane by its own `plane_plane`, cylinder, cone, sphere, torus} and cone×cylinder;
  - `_ => Unsupported`.
- `chord_join::section_case` (`crates/topo/src/chord_join.rs:1275`): plane×{cylinder, cone, sphere, torus}, `_` refused typed.
  Its narrowing is documented.
- `offset_axial`'s meridian `Profile` solve: `crates/topo/src/offset_axial.rs` header says
  "It does not route through the C5 table". It re-solves coaxial pairs, and calls `plane_torus_section` by name as a wildcard arm (~l.2117).

Parallel cylinders of unequal radius show the cost. `route_pose` refuses them, while
`join` (`parallel_radical_plane`), `section_cert::cylinder_cylinder` and `locus::cylinders_locus` each serve them.

## Owed

One table in the C5 sense. Each of these consumers either reads `route`'s arm, or states
why its lane is a declared narrowing of it (as `chord_join` already does). A pair one
consumer can section and `route` cannot is an arm owed to `route`. Line numbers are as of main, 2026-10-14.
