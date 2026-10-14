---
id: point-in-face-and-point-in-solid-read-a-curved-trim-by-different-chains
kind: issue
title: Point-in-face and point-in-solid read a curved face's trim through different fallback chains
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`), which tests every geometric pair question against C5's rule: one total kind-pair table, one method per pair, no runtime fallback (`crates/geom-brep/README.md`, C5).

## What

- `contain::face_placement` (`crates/topo/src/boolean/contain.rs:870`) delegates `_ =>` to the curved door.
- `curved_interior_placement` (`:935`) returns `Trim(None)` for plane, Nurbs, Approx and ringed non-sphere faces. Its cylinder outline chain is a fallback.
- `solid_contain::face_geo` (`crates/topo/src/boolean/solid_contain.rs:995`):
  - a `Some(s) => KindUnsupported` catch-all;
  - cone and sphere arms that are fallback chains;
  - face trims read differently from `contain`'s.
- The planar point-in-face has three entries (`contfp`, `point_in_face`, `point_in_loop`) over one core.

## Owed

One point-in-face reading per surface kind, total over the kinds with a typed refusal for the rest, which both point-in-face and point-in-solid read.
