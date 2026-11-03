---
id: tangent-zero-decided-by-the-bend-is-spelled-three-ways
kind: issue
title: A tangent reading Zero decided by the bend, its residue charged, is spelled three ways with three dip bounds
status: open
opened: 2026-11-03
priority: P3
cost: M
---


Found by PR 6415's reviews (`restread/edge-on-ruling-in-face`).

## What

Three readers take a tangent that reads `Zero` against a plane, decide the side by how the curve bends, and charge the first-order residue the `Zero` left:

1. `geom_brep::conic_bend_side` (`crates/geom-brep/src/conic_bend.rs`): a conic edge leaving a vertex. It reads the lift `|N̂·n̂|·2·sin²(least·l/2)/least`, less the residue `|t̂·n̂|·l`, at the witness length `l`, against the band (`conic_bend`, `conic_bend_side`).
2. `topo::link::FaceSector::chord` (PR 6415): a cylinder or cone face across its ruling. It reads the section arc's real point `q` at the lever `ℓ` decided, with the tangent point `Zero`. The dip toward the side opposite `q`'s is at most `z/(4K)`, or `z/(2K)` along a cone's cell (`crates/topo/README.md`, the chord reading).
3. PR 6422's `Reading::charged`, which charges the residue on a link reading. Its bound is that PR's.

Each states its own bound in its own terms.

## Fix

Give the three one home, a reading that takes the tangent's residue and the curve's certified bend and returns the side with its dip bound, if one shape serves all three; otherwise state, beside each, why it cannot.
