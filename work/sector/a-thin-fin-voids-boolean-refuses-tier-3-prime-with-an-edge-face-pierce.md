---
id: a-thin-fin-voids-boolean-refuses-tier-3-prime-with-an-edge-face-pierce
kind: issue
title: A boolean against a void whose apex is a thin fin refuses tier 3′ with an undeclared edge-face pierce on the fin, on main too
status: closed
opened: 2026-10-08
priority: P3
cost: M
closed: 2026-10-26
---



Filed by TANG (PR 4289's fix pass), from the review's fin scene.

## Witness

- **The body.** `meeting::posed_crown(&meeting::fin(), [0.0, 0.2, -0.6], pose, tol)`, subtracted from the block `[1, 2] × [0.5, 1.5] × [0.3, 1.5]` (`posed_box`). This is a void whose apex at `MEET` is a crown with a thin fin: two faces folded at a 1 mm edge, between corners 1e-4° apart.
- **The op.** `subtract(fin_void, over)`, where `over` is the pyramid `apex_pyramid(&corners(50.0, 0.7, 0.5), pose, tol)`. Pose at rest, ε = 1e-9.
- **The refusal.** The result builds at tier 3, then `validate_pseudomanifold` refuses `UndeclaredContact { contact: EdgeFacePierce { .. } }`, witness `(1.8774, 1.0000007, 1.0)`. That point is on the fin, away from `MEET`, and no record declares it. PR 4289's second review found a second pierce on the fin's short edge, at `(1.501, 1.000013, 0.999996)`.
- **On main.** Main (c839f03c) refuses the same, so it is not the polygon-cone reader's. The reader writes naming rows only.

## What is open

Two things, neither settled:
- whether the result truly holds a contact there: two fin faces 7e-7 m apart at the far corners, folded at the short edge;
- or whether the census reads the near-coincident fin faces as a pierce.

`crates/topo/tests/a_vertex_read_by_two_sector_passes.rs` (`three_prime`) would admit this scene as a matrix row once it is settled. Today the fin is a row only of the naming oracle (`a_vertex_read_again_classes_every_edge`).

## Measured: the input pierces itself (FLUSH, PR 5534)

Both witnesses are the fin crown's own self-crossings. On its old bottom
`(0, 0.2, −0.6)` the bottom fan's face on the fin's long edge `c0 c1`
passes through the far fin face and the top face beyond it. The census
over the crown alone refuses it with two `EdgeFacePierce` at every pose,
and the at-rest gate, which runs no census, passes it
(`work/flush/a-fin-crown-at-rest-holds-a-region-winding-minus-one-that-check-7-and-the-census-pass.md`).
Through this row's op, at rest and ε 1e-9, the old crown still gives the
two pierces at `(1.87741, 1.00000066, 1.0)` and `(1.501, 1.000013, …)`.

The fixture moved: `meeting::posed_fin_crown` closes the slit with a lid
and fans its bottom to the ring without the slit's inner corner. On it,
`subtract(block − fin crown, over)` builds and passes
`validate_pseudomanifold` against its own contacts at all five poses,
ε 1e-9. Closing this row is SECTOR's call.

## Closed (2026-10-26, tracker sweep)

Settled by FLUSH's PR 5534 (`work/flush/a-fin-crown-at-rest-holds-a-region-winding-minus-one-that-check-7-and-the-census-pass.md`,
closed 2026-10-21), as the "Measured" section above records. The refusal
was true: both witnesses are the old fin crown's own self-crossings, so
the result does hold a contact there and the census read it right. The
fixture is corrected on main: `meeting::posed_fin_crown`
(`crates/topo/src/test_support_meeting.rs:1097`) builds as a solid at
every pose (`crates/topo/tests/the_fin_crown_winds_as_a_solid.rs`).
The fin void against standing and hanging pyramids is a row of
`crates/topo/tests/a_vertex_read_again_classes_every_edge.rs:885-887`.
Admitting the scene to `a_vertex_read_by_two_sector_passes.rs`'s
`three_prime` matrix is optional coverage, not a defect. File it
separately if it is wanted.
