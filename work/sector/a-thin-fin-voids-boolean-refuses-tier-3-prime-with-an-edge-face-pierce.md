---
id: a-thin-fin-voids-boolean-refuses-tier-3-prime-with-an-edge-face-pierce
kind: issue
title: A boolean against a void whose apex is a thin fin refuses tier 3′ with an undeclared edge-face pierce on the fin, on main too
status: open
opened: 2026-10-08
priority: P3
cost: M
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
