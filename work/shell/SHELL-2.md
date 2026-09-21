---
id: SHELL-2
kind: unit
title: transform_rigid maps an Approx face — the mapped fit re-certified through the scalar's lane
status: closed
opened: 2026-09-04
branch: shell/2-transform-approx
refs: [transform-rigid-refuses-approx-face, no-approx-faced-body-is-both-movable-and-valid, 1020, 1012]
closed: 2026-09-04
pr: 1758
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

## Reference note (FIX's sweep, 2026-09-21)

`transform-rigid-refuses-described-nurbs` was dropped from this row's `refs` because the row closed with **FIX**, which left the tracker at sweep 18 — `work/fix/` is deleted and `docs/DOC-LEDGER.md` is its done-state of record. The finding is unchanged and still readable: `git show 6f0e04ce1534:work/fix/transform-rigid-refuses-described-nurbs.md`.
