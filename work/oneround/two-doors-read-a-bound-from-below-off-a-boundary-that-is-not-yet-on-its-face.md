---
id: two-doors-read-a-bound-from-below-off-a-boundary-that-is-not-yet-on-its-face
kind: issue
title: set_face_surfaces_certifying and the boolean's graft read a lever's bound from below off boundary points that stand off the face they are read for
status: open
opened: 2026-11-23
priority: P2
cost: M
refs: [a-turns-lever-is-read-from-below-for-every-caller]
---


Found by the turn-lever lane's unfolded recount (2026-11-23, sweep suite at ε 1e-9; its message of 19:33Z, the notes on `oneround/turn-lever` under `analysis/oneround/turn-lever/`) and named as a row by one of the fork's designers (round 15, on `analysis/design-fork/turn-lever-composite-verdicts`). Measured by the lane, not by me.

A lever's bound from below is read "off points known to lie on the region" (`crates/geom-core/README.md`, "Float rounding", the session-box bullet's phrase and the lever passage in front of Ev on PR 7619). At four doors a face is read mid-construction, with its surface or its boundary not yet its own. At two of them the stale side is the surface or the faces (a rebased run before its move lands; an edge certified onto a face still on its seed surface): the points are what the door holds, and the bound from below is good. **At the other two the stale side is the boundary, so the points the bound from below is read off are not on the face:**

- `attach::set_face_surfaces_certifying` (offset and shell): the surface is set before the boundary moves, and the boundary stands exactly the offset distance (0.02 or 0.05 in the suites) off the new surface.
- the boolean's graft (`boolean/combine.rs`, `graft_solids_impl`, reading its working source body): a working face's boundary 0.16 to 0.56 off its own radius-0.2 cylinder. **560 `tangent_second_order` verdicts are served Positive there**, each a definite sign levered from below off such points.

Nothing is known wrong: tier 3 re-asks at rest, where the finished bodies pass, and no Zero is served at any of these reads. What is open is that a definite verdict is served mid-construction on a lever read off points that are not the face's. Not established: whether `set_face_surfaces_certifying` re-certifies its edges after the boundary moves or leans on tier 3 alone.

Owed, with the turn-lever unit's first step (the typed missing bound from above, entered from what each door knows): these two doors read their bound from below off the boundary as they leave it, or hold their definite verdicts as provisional by type until the face is its own.

## Added 2026-11-23: `set_face_surfaces_certifying` does not re-certify after the move

Read in `crates/topo/src/attach.rs` by the turn-lever lane (its message of 23:04Z). The door makes its `WedgeReach` on the body BEFORE the move (`WedgeReach::new(self, band)`), certifies each listed edge's new spec against the PLANNED surfaces at that reach's bound from below (`certify_stated`), has every moved face's boundary vouched for on its new chart by residuals (`unvouched`, refusing any not vouched), then mutates, with only `assert_tier1_postcondition` after it. So the lever its certificates read is the pre-move body's, and what stands at rest afterwards rests on the residual vouching and on tier 3, not on a certification of the moved body.
