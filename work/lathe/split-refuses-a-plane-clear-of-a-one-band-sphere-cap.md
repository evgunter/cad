---
id: split-refuses-a-plane-clear-of-a-one-band-sphere-cap
kind: issue
title: Split refuses a plane clear of a one-band sphere cap as one that may meet the sphere face, which the two π-bands' caps split under
status: closed
opened: 2026-11-23
priority: P1
cost: M
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
closed: 2026-11-26
pr: 7484
---


Found by the LATHE one-band lane (2026-11-22, PR 7414), measured on its
branch `lathe/full-revolve-one-band`. Outcome class moved: the input
split before and refuses now.

## What

`sweep` `reach_split_gate_per_face::a_plane_clear_of_a_sphere_or_torus_face_splits_the_body`:
the unit cylinder under a spherical cap (`capped_cylinder`, the cap
`y ∈ [1, 1.5]`) split by the plane `y = 0.5` (`phi 0`) refuses
`SplitReduceError::CurvedBooleanUnsupported { kind: Sphere }`, "the split
plane may meet a sphere face, and the split cannot cut such a face yet".
The plane is half a unit clear of the cap. With two π-bands each
half-cap's reach cleared the plane and the body split; the one-band cap
is one face whose loop is its rim and its seam out to the pole and back,
and some reader of it now reaches the plane. The two candidates are the
carrier gate's reach box (`crates/topo/src/splitting/classify.rs`
`carrier_gate`, through `census::face_reach_in`), which may read a face
whose surface boundary is its rim and a held tip as more of the sphere
than its cap, and the star at a section vertex
(`splitting/neighborhood.rs` `split_star`); not yet told apart.

## Ground

`crates/topo/src/splitting/`, `crates/topo/src/census*`: topo's split
and census ground. Claimed by LATHE, since it gates the one-band item.
