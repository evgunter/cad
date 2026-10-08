---
id: two-fits-sharing-a-smooth-seam-disagree-by-their-certificates
kind: issue
title: two walls offset independently agree along a shared smooth seam only to the sum of their fit certificates, so a vase-class shell refuses FittedBoundaryUnsupported; fitting them together is undesigned
status: open
opened: 2026-10-08
priority: P3
cost: H
refs: [a-fitted-wall-has-no-section-with-a-moved-cap]
---

Filed from the same designer pair. The vase's two walls meet along
tangent-continuous seams; offset independently, the two fits are tangent
there, so no transverse section exists and the edge must be a boundary
row both fits share — which two independent fits agree on only to the
sum of their certificates. Today's refusal is `FittedBoundaryUnsupported`
("a seam shared with another fitted face", `crates/topo/src/replace_face.rs`).

Owed: a design (fit walls together, or constrain each fit along the
shared seam) — a design fork for a designer pair. Related, unreached
today: `shell_open`'s lift of a cavity `Approx` wall back onto its NURBS
wall would be an offset of a fit (`Offset { base }` takes a
`NurbsSurface`, so a fit of a fit is unrepresentable); the expected
answer is that the lift restates the original NURBS surface.
