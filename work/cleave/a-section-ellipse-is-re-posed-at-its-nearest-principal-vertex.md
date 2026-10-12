---
id: a-section-ellipse-is-re-posed-at-its-nearest-principal-vertex
kind: unit
title: A section ellipse is re-posed at the principal vertex nearest its arc, with a signed span, so its parameter resolves to model scale
status: open
opened: 2026-10-12
priority: P2
cost: M
parent: a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join
---

Part 1 of the design on
`a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join.md`
(read its "Designed" section).

## What

A section ellipse's parameter resolves only to ulp(θ)·A where the face
uses it. A plane nearly along a cylinder's axis cuts an ellipse with
A ≈ r/θ, and the face's part of it sits near θ = ±π/2: at θ = 3e-5
that is 7.3e-12. A clockwise arc's span, computed as 2π minus a small
span, adds ulp(2π)·speed. This is the kernel's share of the in-band
and definite endpoint misses at 1e-12. It is also, likely, FLUX's
`a-slab-face-nearly-along-a-cylinder-ruling-exhausts-the-volume-quadrature`.

## Owed

- Each section chord's ellipse puts its seam (θ = 0) at the principal
  vertex nearest its arc. The choice must be a function of the inputs
  (D9), with a documented deterministic tie-break.
- Its parameter window is a signed span about the seam, never reduced
  through 2π (`oriented_arc`'s clockwise branch).
- `Curve3::Ellipse`'s convention becomes "`u_ref` is a principal
  direction". Rename the two semi-axis fields so neither name claims
  "major", and retire `AxesSwapped`. Say in the PR that restfront's
  `an-ellipse-stored-minor-over-major-passes-tier-3` becomes the
  intended state.
- Readers that assume `u_ref` on the major axis:
  - `chart_pcurve`'s cone/`Ellipse` focal-section image: read the
    major frame through `FocalImage`'s `t0` phase field.
  - `arc_leaving`, which levers by `conic.sa` and must take the larger
    semi-axis.
  - Sweep for the rest, by shape, not only by these names.
- The prototype diff is at the orchestrator's scratchpad
  `se1/proto3.diff`; it is a sketch, not the spec.
- Pins at all three ε:
  - the review's coin-flip family (66 poses) and through-axis planes
    θ = 1e-6…3e-4, which build;
  - the cone family, unchanged;
  - FLUX's quadrature pose, re-measured and noted on its row.

The seam crosses CURVED's (`geom/src/curves.rs`), GERM's (`intersect.rs`)
and PCERT's (`pcurve_cache.rs` / `chart_pcurve`) ground; the PR announces it.
