---
id: poleguard-door-enumeration-overstates-the-boolean-refusals
kind: issue
title: step-import/tests/poleguard.rs: the door enumeration says every sphere-face cut refuses, and macroscopic ball cuts succeed
status: open
opened: 2026-09-18
---


Found by TESS-1's lane (the rim-only cap survey's doc-rot note,
`work/tess/rim-only-sphere-cap-panics-at-census.md`, last paragraph);
filed here because the file is a `crates/*/tests/*` header whose
hand-kept enumeration describes a tree that has moved.

**The claim.** `crates/step-import/tests/poleguard.rs`, module docs,
section "The door enumeration, honest": the plane split and boolean
doors are listed as measured shut because "every sphere-face cut
refuses typed at every probed height — as `CurvedBooleanUnsupported` /
`CurvedPierceUnsupported` at the default band".

**What is true now.** A macroscopic plane cut of a ball succeeds at the
default band: `topo::boolean_op_with(Intersect, ball, slab(y ≤ ½))`
returns a body whose sphere faces are the seamed half-caps, and it
meshes watertight — pinned by TESS-1's positive control,
`crates/mesh/tests/meridian_free_face.rs`,
`the_seamed_twins_of_the_refused_caps_mesh_watertight`. So the sentence
reads as "no boolean cuts a sphere face", which is false; what the
cited review rows (`r2_split_door.rs`, `r2_bool_door.rs`,
`r1_probe_bool_route.rs`) measured is narrower — cuts whose rim lands
inside the pole band, the only cuts issue 896's guard is about.

**What the row owes.** Re-word the enumeration to the claim the rows
support (a cut placing a junction within the pole band refuses), and
check the three cited rows still assert what the sentence attributes to
them. The route argument the section exists for (no minting door puts a
non-pole junction inside the band) is not contradicted by a macroscopic
cut succeeding; only the prose over-reaches.

Not verified by this lane: whether the three cited review rows still
pass their "every probed height" sweeps as written — they were not run
or read beyond their names.
