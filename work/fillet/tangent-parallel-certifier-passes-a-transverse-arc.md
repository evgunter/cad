---
id: tangent-parallel-certifier-passes-a-transverse-arc
kind: issue
title: "certify: TangentParallel admits a 90-degree crossing described as a tangent intersection"
status: open
opened: 2026-09-05
---

## Finding (a reviewer's mutant on PR 1897)

Drop `ContactCarrier::TransverseArc` from the `transverse` set in
`crates/sweep/src/blend/surgery.rs`'s `attach_contact`, so the ruled
band's cut-off arcs are described `TangentIntersection { band, cap }`
instead of `Intersection`. The arcs then CERTIFY at the attachment gate
and the body PASSES tier 3 — although the band (a cylinder) and the cap
(a plane perpendicular to the cylinder's axis) cross at exactly 90° along
the arc, so the normals are nowhere near parallel there.

The certifier's tangent arm (`crates/geom-brep/src/certify.rs`, the
`TangentParallel` check) meters `sin θ / |κ_rel|` — the normal
misalignment divided by the relative curvature — and along this arc
`κ_rel` is the cylinder's `1/r` against the plane's zero, which is large
enough to let `sin θ = 1` through the gate. A tangent description that
is false by a right angle is therefore not caught; only a description
that is false by a small angle would be.

## Consequence

`attach_contact`'s comment used to claim "certification measures exactly
that" of the tangent/transverse distinction; it does not, and the comment
now says so and points here. The description is correct in the tree
(`Intersection`), chosen for what the geometry IS; this issue is about
the certifier's blind spot, which is pre-existing and outside FILLET's
fence.

## Fix shape

A `TangentParallel` margin that is the misalignment ANGLE at a lever
(the arc's own radius, or the carrier extent), not the angle divided by
`κ_rel`; or a second reading that refuses a tangent description whose
surfaces' normals are definitely non-parallel at the witness regardless
of curvature. Either needs the K-REPORT runbook for the new margin's
distribution before it can gate.

## Cross-program note

The code is `geom-brep`'s certifier (CURVED / CERT ground). Filed under
FILLET as the program whose review found it; the owner places it.
