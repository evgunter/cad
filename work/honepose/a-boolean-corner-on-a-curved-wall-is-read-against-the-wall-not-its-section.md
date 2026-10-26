---
id: a-boolean-corner-on-a-curved-wall-is-read-against-the-wall-not-its-section
kind: issue
title: A Boolean vertex read on another solid's curved face decides its residual, not its distance from the section a grazing planar face of its own cuts there
status: open
opened: 2026-10-12
priority: P2
cost: M
---


Found by the sweep of
`the-split-reads-a-vertex-against-the-section-it-ends-not-the-plane`
(branch `cleave/vertex-against-section`). A live definite defect
today, measured below; PR 4858's review found the pose.

## What

That unit reads a vertex ON a plane against the section the plane cuts
in each curved face at the vertex, `δ / sin ψ`
(`vertex_section::decide_vertex_on_plane`), at the split and at the
Boolean's vertex against a planar face of the other solid. The mirrored
reading is not covered: in `boolean::reduce::curved_face_arm`, a vertex
of one solid is decided ON a curved face of the other by its residual
(`bool_vertex_face_side` over `geom_brep::implicit_residual`). A planar
face of its own solid at that vertex, grazing the curved face at
dihedral `ψ`, cuts it in a section that passes `δ / sin ψ` from the
vertex, which the join then certifies.

The pose: a box `[1, 2] × [0, 1] × [0.5, 1.5]` whose corner sits on the
unit cylinder's wall at `(1, 0, 0.5)`, its face `x = 1` turned about
the corner onto `n = cos ψ·w + sin ψ·(cos α·ẑ + sin α·t)`, the box moved
`δ` along `n`, the cylinder turned φ, the cylinder less the box
(`VS_DOOR=mirror` in
`crates/sweep/examples/vertex_against_section_probe.rs`).

- `α = 0` (`VS_ALPHA` unset): every row with `|δ| < ε` refuses
  `CurvedPierceUnsupported` first, at ε = 1e-9 and 1e-12.
- `VS_ALPHA=0.7`, φ = 0.9, ε = 1e-9: the corner reaches the join, which
  refuses `Join(Euler(Certification { ResidualExceeded { EndpointStart } }))`,
  a definite miss on a corner the sweep read on the wall. `ψ = 1e-3`:
  2.615e-7 at δ = −0.1ε, up to 2.589e-6 at −0.99ε; `ψ = atan 0.05`:
  2.614e-8 to 5.175e-8 at δ ∈ −{0.5, 0.9, 0.99}ε; `ψ = atan 0.1`:
  2.349e-8 and 2.584e-8 at δ ∈ −{0.9, 0.99}ε. The review measured the
  same on `origin/main`.

## Owed

Decide the corner against the sections its own planar faces cut in the
wall there, through the same rule (`vertex_section`), so a definite miss
is read in the sweep and an in-band one refuses typed with its lever.
Pin the α = 0.7 family over the ψ ladder at every ε.

## Slate triage (2026-10-26, CLEAVE)

- Moved to HONEPOSE (`work/honepose/`) by `git mv`, id and body unchanged: the P2 track opened for in-band refusals at measured poses: a reading that lands in band where the pose is not near a decision. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
