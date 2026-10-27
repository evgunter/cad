---
id: the-door-reads-each-seam-members-loops-three-times
kind: issue
title: The measurement door parses each sphere seam member's loops three times: its own side, the region's, and its flux arm's
status: open
opened: 2026-10-21
priority: P4
cost: E
refs: [5556]
---


## Measured

The door's seam pass (`crates/topo/src/props.rs`, `seam_refusals`, run
once per `References::of`) reads each sphere seam member's loops three
times:
- for its own side (`boundary_material_sign_loops`);
- as part of its region (`geom_brep::props::seam_loops`, shared by the
  pole read and the census);
- again in the flux arm (`sphere_face`).

On the seam corpus (the six `seam_siblings_flipped_together` pairs, the
split lune, the slab band; release; 300 iterations, three interleaved
runs each way), `mass_properties` costs about +25% with the pass
(1.32 ms against 1.06 ms per corpus pass, about 33 µs a body), and
`validate_geometric` about +5% (9.32 ms against 8.86 ms), 2026-10-21.

## What closes it

One `SphereLoop` parse per face per door call, handed to the own-side
read, the region parse and the flux arm alike. That needs a parsed-loop
type `geom_brep::props` exposes, or a door into the flux arm that takes
one.
