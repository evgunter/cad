---
id: surface-pair-sections-are-classified-by-tables-beside-route
kind: issue
title: Surface-pair sections are classified by five tables beside intersect::route, which only replace_face consults
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`, #5078). The rule it tests is D3's "One reading per question" (`docs/DESIGN.md`, D3): every decision about carriers is a reading (of one carrier against a point, of a carrier pair's pose, or of a pair outcome that takes the pose); each reading is one exhaustive match with one method per arm and no fallback, and every consumer reads it. C5 (`crates/geom-brep/README.md`) is that rule's instance for surface and curve pairs.

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

Parallel cylinders of unequal radius showed the cost: `route_pose` refused them, while
`join` (`parallel_radical_plane`), `section_cert::cylinder_cylinder` and `locus::cylinders_locus` each served them.
Since SECTARMS's PRs 6139 and 6151, `route_pose` and the section certificate both read the pair's
pose on the section value (`geom_brep::pair_section::cylinders`); `join` and `locus` still read their own.

## Owed

Re-aimed to D3 "One reading per question". The section is a pair *outcome*. One outcome reading owns it: it takes the pair's pose reading (`route_pose`'s classification, decided once) as input and decides no pose of its own. `route` is that reading's kind-level dispatch (kinds to arm, then the arm's method), not a table beside it, so which pairs the reading serves is derived from what it does.

- Each of the five consumers reads the outcome reading. One that serves fewer pairs (`chord_join::section_case`'s declared narrowing, `section_closed`, the `offset_axial` profile solve) matches on the reading's answer and refuses the rest typed. It never matches on kinds to choose its own method; a consumer that asks at kind level before it holds the operands asks `route`.
- `join`'s `KindArm::None` read as "try the face-pair arm, else the edge-plane lane", and the skew-cylinder drop-through, are fallbacks D3 forbids.
- A pair a consumer can section and the reading cannot (cone×cone, cone×sphere, cone×torus, sphere×torus, torus×torus, cylinder×torus; parallel cylinders of unequal radius) is an arm owed to the reading, with its pose owed to the pose reading, rather than a lane kept beside it.

Line numbers are as of main, 2026-10-14.
