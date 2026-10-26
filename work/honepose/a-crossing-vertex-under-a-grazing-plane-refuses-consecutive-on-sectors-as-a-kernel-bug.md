---
id: a-crossing-vertex-under-a-grazing-plane-refuses-consecutive-on-sectors-as-a-kernel-bug
kind: issue
title: A plane through a cylinder's seam vertex, crossing the rim and grazing the wall (psi <= 3 deg), refuses ConsecutiveOnSectors (kernel bug) at a crossing vertex
status: open
opened: 2026-10-12
priority: P2
cost: M
---


Found by `the-split-reads-a-vertex-against-the-section-it-ends-not-the-plane`
(branch `cleave/vertex-against-section`). It refuses the same on
`origin/main` before that branch.

## What

The unit cylinder (`0 ≤ z ≤ 1`) turned `φ` about its axis, cut by the
plane through `v + δ·n`, `v = (cos φ, sin φ, 0)` its seam vertex, with
`n = cos ψ·w + sin ψ·(cos α·ẑ + sin α·t)` (`w` the wall's normal at
`v`, `t` the rim's tangent), `α = 0.7`. The plane crosses the rim at `v`
and grazes the wall at dihedral `ψ`. Measured with
`crates/sweep/examples/vertex_against_section_probe.rs`
(`VS_ALPHA=0.7`), φ ∈ {0.9, 2.2}:

- `ψ = atan 0.05`, `δ = ±0.5ε`, at every ε; `ψ = 1e-3`,
  `δ ∈ {±0.1, …, ±0.99}·ε` at ε = 1e-9 and 1e-12, and `δ = 0` too at
  ε = 1e-6: `split` refuses `Reduce(ConsecutiveOnSectors { vertex })`
  at a vertex the reduction inserted on an edge, ending "which the
  coplanar gate rules out (kernel bug)".
- `ψ = 1e-6` at ε = 1e-9 and 1e-12: the same refusal at nearly every
  `|δ| < ε`, `δ = 0` included.
- The same poses at `ψ = 0` and at `ψ ∈ {90°, 30°}` build or refuse
  typed (`SliverVertex`, `CrossingEscalated`).

The Boolean's difference with the slab whose face is the plane refuses
`CurvedBooleanUnsupported` on the same rows, so only the split reaches
the kernel-bug ending.

## Owed

Find which reading leaves two consecutive on-plane sectors at the
inserted vertex on valid input, and either build these poses or refuse
them typed with the input's lever. A kernel-bug ending on a pose the
operand gate admits is a defect.

## Slate triage (2026-10-26, CLEAVE)

- Moved to HONEPOSE (`work/honepose/`) by `git mv`, id and body unchanged: the P2 track opened for in-band refusals at measured poses: a reading that lands in band where the pose is not near a decision. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
