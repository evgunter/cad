---
id: a-rim-grazing-the-plane-at-a-vertex-read-off-by-its-section-leaves-two-ends-unpaired
kind: issue
title: A cylinder split whose plane grazes the wall at a seam vertex and dips under ε into the cap refuses UnpairedLooseEnds: the rim reads as a touch while the vertex reads off by its section
status: open
opened: 2026-10-12
priority: P2
cost: M
---


Found by `the-split-reads-a-vertex-against-the-section-it-ends-not-the-plane`
(branch `cleave/vertex-against-section`).

## What

The unit cylinder (`0 ≤ z ≤ 1`) turned `φ` about its axis, cut by the
plane through `v + δ·n`, `v = (cos φ, sin φ, 0)` its seam vertex and
`n = cos ψ·w + sin ψ·ẑ` (`w` the wall's normal at `v`). The plane is
tangent to the bottom rim at `v` and grazes the wall at dihedral `ψ`.
Measured with `crates/sweep/examples/vertex_against_section_probe.rs`
at ε = 1e-9, 1e-6 and 1e-12, φ ∈ {0.9, 2.2}:

- `ψ = atan 0.05`, `δ ∈ {−0.5, −0.9, −0.99}·ε`, and `ψ = 1e-3`,
  `δ ∈ {−0.1, …, −0.99}·ε`: `split` refuses
  `Join(UnpairedLooseEnds { count: 2 })`, and `topo::subtract` of the
  slab whose face is the plane refuses the same.
- The same poses at `δ > 0` split to the closed form, and the
  difference refuses `CurvedBooleanUnsupported { kind: Cylinder }`.
- `δ = ±20ε` builds at both doors.

The root cause is two decisions that read one place differently.
`vertex_section::decide_vertex_on_plane` decides `v` by its section
reading, definitely off the plane (`|δ| / sin ψ ≥ Kε`), which is the
input's truth: the wall's section passes that far from `v`. The rim
beside it is still read by its plane offset. For `δ < 0` the plane dips
`|δ| < ε` into the cap, so the rim crosses it at `θ ≈ ±√(2|δ|)` and lies
within the band of the plane between, and the root lane reads that
stretch as an in-band touch with no crossing. The vertex is off, the rim
is on, the wall's section has no rim crossing to end at, and the join
ends `UnpairedLooseEnds`.

## Owed

Decide what the rim reads where it lies within the band of the plane
over a stretch whose vertex is definitely off the plane's section. The
honest outcomes are a build, or a typed refusal of the input naming the
cap's near-coplanar touch with its lever. The join's loose-end count is
neither. The Boolean's `δ > 0` frontier refusal is a separate question
of the curved-face arm.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: two decisions read one place differently: margins decided one at a time, or one verdict read off two decisions: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEPOSE (`work/honepose/`) by `git mv`, id and body unchanged: the P2 track opened for in-band refusals at measured poses: a reading that lands in band where the pose is not near a decision. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
