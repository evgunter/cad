---
id: a-split-along-a-seam-at-a-vanishing-tilt-returns-an-invalid-body
kind: issue
title: A split through a cylinder's seam vertex at a vanishing tilt returns Ok bodies that fail validate_geometric (VolumeUncomputable)
status: dispatched
opened: 2026-10-13
priority: P1
cost: M
branch: cleave/seam-split-invalid-ok
---


Found by the review of PR 4858
(`the-split-reads-a-vertex-against-the-section-it-ends-not-the-plane`,
branch `cleave/vertex-against-section`). Main and that branch return the
same bodies here: the branch leaves these vertices on the offset's
verdict.

## What

The unit cylinder (`0 ≤ z ≤ 1`, seam vertices at `(±1, 0)`) turned
`φ = 2.2` about its axis. It is split by the plane exactly through its seam vertex
`v = (cos φ, sin φ, 0)`, with normal `n = cos ψ·w + sin ψ·ẑ` (`w` the
wall's normal at `v`), its stored origin moved `L` along the rim's
tangent, in the plane. Measured with
`crates/sweep/examples/vertex_against_section_probe.rs`
(`VS_DOOR=witness VS_PSI=1.1e-7 VS_L=…`):

- ε = 1e-9, `ψ = 1.1e-7`: `split` returns `Ok` at `L ∈ {0.02, 0.1,
  0.18, 0.5}`, and both halves fail `validate_geometric` with
  `VolumeUncomputable { source: Face { … Escalated { …
  props_chart_junction_closed, margin ≈ 2.588e-9 … }, check: Inventory } }`
  on the wall faces. The neighbouring `L` refuse at the join.
- ε = 1e-12, same `ψ`: `Ok` and invalid at `L ∈ {0.1, 0.5}`.

The review measured 3 such bodies at `ψ = 1e-7`, `L ≤ 0.22`. A split
that returns a body its own validation refuses is a wrong answer: the
result gate passes what `validate_geometric` refuses.

## Owed

Find where the split's finish lets these halves through, and make the
split refuse them typed (or build them valid). Pin the family at every
ε, with `validate_geometric` on every body returned.
