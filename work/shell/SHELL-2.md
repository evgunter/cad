---
id: SHELL-2
kind: unit
title: transform_rigid maps an Approx face — the mapped fit re-certified through the scalar's lane
status: spec
opened: 2026-09-04
branch: shell/2-transform-approx
refs: [transform-rigid-refuses-approx-face, 1020, 1012]
---


`topo::transform_rigid` refuses every `Surface::Approx` face typed
(`TransformError::ApproxSurface`) although the composition law is
pinned: a rigid map of an offset is the offset of the rigid map, so
the mapped fit certifies against the mapped base at the same `d` and
tolerance. This unit maps the base net and the fit net rigidly and
RE-DERIVES the certificate through the scalar's own lane — never
carries the stored one across — so an `Approx`-faced body moves at
`f64` and refuses typed, naming the lane, at every scalar that
cannot certify a fit. Closes `transform-rigid-refuses-approx-face`
(issue record 1020); the stale `NurbsPlaceholder` message is
corrected alongside. Spec `docs/SHELL-2-SPEC.md`. Pre-draw
difficulty S–M, task class STRUCTURAL/NUMERIC.
