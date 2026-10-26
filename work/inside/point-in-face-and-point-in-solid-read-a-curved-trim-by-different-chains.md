---
id: point-in-face-and-point-in-solid-read-a-curved-trim-by-different-chains
kind: issue
title: Point-in-face and point-in-solid read a curved face's trim through different fallback chains
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`, #5078). The rule it tests is D3's "One reading per question" (`docs/DESIGN.md`, D3): every decision about carriers is a reading (of one carrier against a point, of a carrier pair's pose, or of a pair outcome that takes the pose); each reading is one exhaustive match with one method per arm and no fallback, and every consumer reads it. C5 (`crates/geom-brep/README.md`) is that rule's instance for surface and curve pairs.

## What

- `contain::face_placement` (`crates/topo/src/boolean/contain.rs:870`) delegates `_ =>` to the curved door.
- `curved_interior_placement` (`:935`) returns `Trim(None)` for plane, Nurbs, Approx and ringed non-sphere faces. Its cylinder outline chain is a fallback.
- `solid_contain::face_geo` (`crates/topo/src/boolean/solid_contain.rs:995`):
  - a `Some(s) => KindUnsupported` catch-all;
  - cone and sphere arms that are fallback chains;
  - face trims read differently from `contain`'s.
- The planar point-in-face has three entries (`contfp`, `point_in_face`, `point_in_loop`) over one core.

## Owed

Re-aimed to D3 "One reading per question". Point-in-face is a reading over trimmed faces, so it lives in `topo` and reaches carrier facts (on-carrier, chart coordinates) only through the carrier readings (`honeone/point-to-carrier-distance-is-spelled-seven-times`, `chart/chart-inversion-is-spelled-three-times`).

- One point-in-face reading, one exhaustive match over surface kinds with a typed refusal for the rest. Point-in-face and point-in-solid both read it.
- `face_placement`'s `_ =>` delegation, the cylinder outline chain, and `face_geo`'s cone and sphere chains are methods reached because another declined: fallbacks D3 forbids. Each arm has one method.
- `face_geo`'s `Some(s) => KindUnsupported` catch-all becomes the reading's typed refusal, listed per kind.
- The three planar entries (`contfp`, `point_in_face`, `point_in_loop`) are one reading with one entry.

## Moved by CHART (2026-10-24)

Branch `chart/ringed-curved-doors` gives cylinder and cone faces one
reading, `topo::boolean::ruled_region`. `curved_face_containment` and
`face_geo` both read it, and `curved_interior_placement` is now one
exhaustive match over the surface kinds. The cylinder outline chain is
deleted, and so is `face_geo`'s cone chain (the wrapped group's
representative, the trimmed window, `PartialCone` reached on a
declined trim). A face whose region the reading cannot take refuses
inside its ball, the same on both doors. Still open: the sphere's
closed-group / patch chain, `face_placement`'s `_ =>` delegation, and
the three planar entries.
