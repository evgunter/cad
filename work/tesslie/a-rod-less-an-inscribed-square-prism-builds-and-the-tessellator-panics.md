---
id: a-rod-less-an-inscribed-square-prism-builds-and-the-tessellator-panics
kind: issue
title: A rod less an inscribed square prism builds sound, and mesh::tessellate panics on it
status: open
priority: P2
cost: M
opened: 2026-10-09
refs: [tessellator-panics-on-a-self-slit-face-every-validator-passes]
---

Found by the dual review of the tube-on-a-ball unit (PR 4399, review r2, its N5, family F7), and present on base before that PR.

## What

- **The operands:** a square prism inscribed in a rod. The square has corners `(±1, 0)` and `(0, ±1)`, over `z ∈ [−1, 3]`. The rod is a `z`-axis cylinder of radius 1 over `z ∈ [0, 2]`. The prism's four vertical edges lie in the rod's wall.
- **The result:** `rod ∖ prism` builds SOUND. That covers tiers 2 and 3′, the certificate, a legal operand, and the volume `2π − 4 = 2.283185307`. The same holds turned 0.3 about `z`.
- **The failure:** `mesh::tessellate` then panics at `crates/mesh/src/tessellate.rs:592`, "edge of 4 face triangles": the four pieces meet along lines.

The class is the one of `tessellator-panics-on-a-self-slit-face-every-validator-passes`: a consumer panicking on a body every validator accepts. Either the mesher handles four faces on one line edge, or a validator refuses the body.

## Repro

Review r2's probe file `crates/sweep/tests/r2_probes.rs`, family F7 ("inscribed square in a rod"), on branch `join/tube-ending-on-a-ball-review-r2`. Its lines are in `review-r2/probe-head.txt` (`mesh=PANIC` on `B ∖ A`).

## Witness: a slab the ruling pierces twice (2026-10-15)

Found by the full review of PR 5187.

**The pose.** The rod and ruling-square pair of `crates/sweep/tests/a_pierce_held_in_a_curved_face.rs` (`a_wall_the_ruling_pierces_twice_builds_in_every_op_and_order`), against the slab z ∈ (0.8, 1.2).

**What happens.** `slab ∖ pair` builds sound at every turn. `mesh::tessellate` then panics at `crates/mesh/src/tessellate.rs:592` ("edge of 4 face triangles"). With three or four pierces of the wall, the union panics as well.

**The shape.** A ruling segment the two lumps share, with both ends at held-pierce vertices.

**This pose is not where the defect lies.** On main, before PR 5187, the same body built as `(slab ∖ rod) ∖ square` or as `(slab ∖ square) ∖ rod` panics identically. It has the same face, edge and vertex counts (12, 32 and 20) and the same volume. This is the same class as `two-coincident-edges-between-one-vertex-pair-mesh-non-manifold`.

## Moved to TESSLIE (the mesh answers Ok, or panics, where it should refuse or mesh right) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.
