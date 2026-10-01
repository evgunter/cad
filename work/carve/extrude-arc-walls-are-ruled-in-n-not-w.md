---
id: extrude-arc-walls-are-ruled-in-n-not-w
kind: issue
title: extrude rules an arc leg's wall cylinder in the sketch normal, not in w, so an admitted tilted Vector fails untyped
status: open
opened: 2026-10-01
---


## Finding

Found by BAND's cap-rim smooth-arm unit while writing the row that pins
which wall kinds can carry the admitted tilt.

`extrude`'s `Extrusion::Vector` door admits a `w` whose in-plane
component the `extrusion_obliquity` gate reads `Zero` (up to ε), and
then builds the walls inconsistently with it
(`crates/sweep/src/extrude.rs`, `side_surface`):

- a LINE leg's wall is `newell_plane` over the quad
  `qs[j] + w, qs[j], qs[j+1], qs[j+1] + w` — ruled in `w`;
- an ARC leg's wall is a cylinder with `axis: turn_axis(turn, normal)`
  — ruled in the sketch normal `n`, not in `w`.

The top rim arc is minted at `top_place = translation(w) ∘ place`, so it
sits off the axis-`n` cylinder by the in-plane component of `w`. At the
largest admitted in-plane component that offset is ε, and an obround
(`fillet_h6_cap_rim`'s `obround_loop`) under it does not build. Measured
on the witness tolerance (ε = 1e-9, K = 10):

| `w` | result |
| --- | --- |
| `(ε, 0, 1)` | `Op { Certification { Escalated { check: Surface2Residual, .. } } }` |
| `(ε, 0, 1e-3)` | same |
| `(ε, 0, K·ε)` and `(ε/2, 0, K·ε)` | `Op { Certification { ResidualExceeded { check: ChartResidual } } }` |
| `(0, ε, K·ε)` … `(0, ε, 1e-3)` | `SliverJoin` under `dihedral_wedge` at the tangent line–arc strut (the plane wall leans by the tilt, the cylinder does not) |
| `(0, ε/2, 1e-3)` | `Op { Certification { ResidualExceeded { check: TangentParallel } } }` |
| `(ε/2, 0, 1)`, `(ε/2, 0, 1e-3)`, `(0, ε/2, 1)` | builds |

So an input both direction gates admit refuses as an untyped
certification error from deep in `mef`, rather than either building or
refusing typed at the door. An all-line profile under the same `w`
builds (`every_extruded_cap_rim_is_transverse`'s "worst admitted tilt"
row), so it is the arc leg's ruling that disagrees.

## Shape of a fix (not decided)

Either rule the cylinder in `w` (its axis is then not the sketch normal,
and every consumer that assumes `axis = ±n` — the cap-rim smooth arm's
reachability argument among them — moves with it), or project `w` onto
`n` once the obliquity gate has read the in-plane component `Zero`, so
every wall and both caps are built from one direction. The second makes
line walls perpendicular to the caps as well — the cap-rim `Smooth` arm
in `upgrade_rim` would then be unreachable at every K, and
`fillet_h6_cap_rim::only_line_walls_carry_the_admitted_tilt` (whose
tilted-plane count would drop to 0) and the crossover row would report
it.
