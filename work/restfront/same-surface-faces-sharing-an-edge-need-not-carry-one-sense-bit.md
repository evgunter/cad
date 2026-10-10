---
id: same-surface-faces-sharing-an-edge-need-not-carry-one-sense-bit
kind: issue
title: No check requires two faces of one body on the SAME surface entity that share an edge to carry the same sense bit
status: open
opened: 2026-10-10
priority: P3
cost: M
---


Found off-question by the sphere-arm designer pair (FLUX fork log row
108), 2026-10-10. It sits on validity's ground (`crates/topo/src/validate.rs`).

Two faces of one body that lie on the same surface entity and share an
edge must carry the same sense bit. If they did not, the surface would
claim material on both sides of the edge. The check is exact and needs
no tolerance band.

Today only check 4's tangent lamina reading (`LaminaWedge`) catches such
a flip, and it does so by band. The seam-sibling flips that make up most
of FLUX's
`a-sphere-face-whose-boundary-encodes-no-side-is-measured-under-its-bit-alone`
population would be named directly by a tier-2 check of this rule.
(FLUX orchestrator)
