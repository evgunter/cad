---
id: a-point-has-no-place-in-a-nurbs-faces-trim
kind: issue
title: Point-in-face has no NURBS arm: contain::curved_interior_placement answers Trim(None) for a spline face, so a pierce found on one could not be placed
status: open
opened: 2026-10-26
priority: P1
cost: M
refs: [an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane]
---


Filed by the SPLICE orchestrator from the design Ev took on PR 5727
(fork log row 198), on a designer's finding.

`boolean::contain::curved_interior_placement` answers `Trim(None)` for
a `Nurbs` (and `Approx`) face. A meeting the curve × face table finds
inside a spline face's chart is placed in or out of the face by the
face's trim (C5's clause). C4's derived pcurves make that a 2-D
point-in-loop reading at `(u, v)`. Teapot wall 3's pierce needs it.
