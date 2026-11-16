---
id: a-curved-partners-partial-is-read-two-ways
kind: issue
title: A curved partner's partial across a window is read two ways: the boundary pass hulls 8x8 sub-windows, limb 3's ChartPhi::partial reads the window whole
status: open
opened: 2026-11-11
priority: P2
cost: M
---


Filed by the plane-partial lane (PR 6814, branch
`sectarms/plane-partial-closed-form`, 2026-11-11), on the review's
finding.

## What

`φ`'s partial across a window of the wall, `∇f(S)·S_d`, has two readings
for a curved partner (cylinder, sphere, torus):

- **the boundary pass**: `impl Partner for &ImplicitForm::partial`
  (`crates/geom-brep/src/ssi/boundary.rs`) hulls a
  `CURVED_PARTIAL_PIECES`² = 8×8 grid of sub-windows, each `∇f` over the
  piece's box against the piece's derivative box. Its doc gives the
  reason: over a whole strip the two boxes forget their correlation, and
  a side lying on the partner would read no slope one-signed;
- **limb 3 and the one-arc walk**: `ChartPhi::partial`
  (`crates/geom-brep/src/ssi/enclose.rs`) reads `plane_partial(∇f over
  the window's box, …)` over the whole window, as one reading.

So the two readers can answer differently on the same window. Their
enclosures differ in width (one subdivides), never in soundness. The
`[one-way.readings]` question is which of them is the partial's one
reading.

For a plane both now read the constant gradient once (PR 6814), so the
split is a curved partner's only.

## What the change could be

Give `ChartPhi::partial` the piecewise reading and route the pass through
it, or argue why limb 3's window never needs the correlation the pass's
strips do. Mind the cost: the 8×8 hull is what made the plane rows slow
before PR 6814, and limb 3 reads many windows.
