---
id: a-convex-graze-of-a-cone-refuses-at-some-azimuths
kind: issue
title: a plane holding a cone's apex refuses at the apex vertex, grazing or cutting
status: open
opened: 2026-10-02
priority: P3
cost: M
---


## What

A full cone (base r = 1, apex at y = 1, revolved about y), cut by a
plane that holds its apex, refuses `Reduce(SliverSector)` on
`sector_straight` at the apex vertex, with margin 0, under either
normal. This happens both when the plane grazes the cone along a ruling
and when it cuts through it:

- grazing along a ruling at every azimuth of `THETAS`
  (`split_tangent_edge_curved.rs`), at ε 1e-6, 1e-9 and 1e-12
  (`cleave/inband-graze`, 2026-10-06);
- turned off tangency by t ∈ {1e-3, 0.05, 0.4, 1, π/2, 2, 3, −0.4, −1.2}
  at θ ∈ {0, 0.3, 2}, at the same three ε (`cleave/seam-ruling-split`'s
  sweep, main at 78bee3ac68).

Each such plane holds the apex, so each side is a cone over a base
segment, and its volume has a closed form. None of these poses is
pinned.

## Where to look

`sector_straight` at a cone's apex vertex (`topo/src/sector_shape.rs`).
Not measured: whether the plane×cone apex lane's tangent line plays any
part. That lane reads a residue generator on its `Zero` arm
(`work/germ/a-plane-cone-apex-tangent-reads-a-residue-generator-off-the-tangency.md`,
the same mechanism the crossing lane had). The secant poses, which never
reach that arm, refuse the same way, so this door comes first.

## Resolved elsewhere

The row first held the frusta poses as well (narrowing θ = 0.3; widening
θ ∈ {1.1, 2.9}; `SliverSector` on `split_conic_departure`, and
`UnpairedLooseEnds` at 1.1) and the filleted slab at φ = 1.2. Their
concave twins were the filleted hole at φ = 1.2 and DR-4098's L-bracket
cove at φ = 4.2 (margin 5.3e-9). All of them stopped because the crossing
lane put its graze root on the residue's crossing rather than the
extremum. PR 4179
(`an-in-band-concave-graze-refuses-at-certification-on-one-side-of-tangency`)
fixed that. The frusta and the slab now answer at their closed forms, at
all three ε, and `CONE_GRAZES_REFUSED` and the slab's φ = 1.2 allowance
are gone. The filleted hole and a cove
(`a_concave_graze_of_a_cove_refuses`) refuse the knife edge.

## Found by

CLEAVE DR-51's review of PR 3892, `review-tests/dr51` (74e151b6).
