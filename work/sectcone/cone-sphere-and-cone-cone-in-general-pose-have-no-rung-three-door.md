---
id: cone-sphere-and-cone-cone-in-general-pose-have-no-rung-three-door
kind: issue
title: Cone × sphere and cone × cone in general pose route to rung 3 and refuse: no pair door with its gate, tangency decision and certified pose
status: open
opened: 2026-11-21
priority: P0
cost: M
---


Filed 2026-11-21 by the SECTCONE orchestrator. The P0's designers noted that no row owns these pairs.

`crates/geom-brep/src/intersect.rs` `route(Cone, Sphere)` and `route(Cone, Cone)` answer `implemented: false`. Once the cone has its limb-2 reading at the search and its tube arm (the two units this row is parked on), each pair needs a door like `ssi::sphere_torus_ssi`: a pair gate, the pair's own tangency decision before any rung, then `implicit_pair_r3`, and `route` flipped.

- **Cone × cone:** the common-apex pose stays the rulings' closed form, decided at the gate.
- **Cone × sphere:** the coaxial pose stays `cone_sphere_section`.
- **Both:** an apex on the partner is C5's node or touch margin, decided before rung 3 (`[ev]` PR 6432).

This row is P0 for the reason its cone × torus sibling (`cone-torus-rung-three-arm-waits-on-the-cone`) is: a union of a cone and a ball or another cone on everyday geometry refuses.
