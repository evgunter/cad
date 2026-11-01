---
id: the-vertex-vertex-material-rows-read-a-curved-face-with-no-curvature-charge
kind: issue
title: The vertex-vertex material rows read a curved partner face at its tangent plane with no curvature charge, where the vertex-on-face pass charges the sagitta
status: open
opened: 2026-11-01
priority: P2
cost: M
---


Found by all three designers weighing `the-booleans-cone-reader-is-a-second-link` (2026-11-01). Unverified: no witness built.

`crates/topo/src/boolean/sectors.rs` `wedge_read` and `cone_side` class an own edge against a curved face of the other operand through `plane_side_code`, which passes `NO_CURVATURE`: the first-order rise `d̂·n̂·L` takes no charge for the face's sagitta. `NO_CURVATURE`'s doc justifies that for `pair_codes` (two tangent planes crossing), but these rows are a material class. The vertex-on-face pass charges it (`read_side`'s lever at the face's least radius, `min_radius_of_curvature`). Example: a fillet face of radius 1 mm and an edge leaving at a slope of 1e-6 over 1 m reads `Out` while the edge is inside the material past about 1e-9 m.

`the-boolean-reads-the-other-operand-through-one-cone` moves these readings onto a `Cone` of tangent sectors without fixing this; afterwards the charge belongs on the `Cone` builder's tangent sectors, one home. ONEHOME's `the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways` is the layer below (a germ's side of one face).

Owed: a witness, and the charge, or a stated reason a germ reading owes none.
