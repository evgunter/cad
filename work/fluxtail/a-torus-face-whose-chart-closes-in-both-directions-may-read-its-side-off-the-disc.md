---
id: a-torus-face-whose-chart-closes-in-both-directions-may-read-its-side-off-the-disc
kind: issue
title: Probe: a one-loop torus face whose chart domain closes in both directions without a wrap edge (torus minus a disc) may be read as the disc by the Green-area side
status: open
opened: 2026-10-10
priority: P3
cost: M
---


Found off-question by the sphere-arm designer pair (fork log row 108),
2026-10-10. Unverified.

A torus is a closed surface, like a sphere. `torus_face` and
`torus_material_sign` read a face's side off the sign of its chart
Green area. That reading assumes the face's chart domain does not close
in both directions without a wrap edge. A one-loop face of the form
torus-minus-a-disc breaks that assumption, and its area would then be
read as the disc's.

One designer believes no public path builds such a face, because
`merge_faces` refuses a run that closes its chart's full period. The
first step is a probe row that builds or imports one. If it reaches the
props arm, fix the side reading; if nothing reaches it, state the
premise at the reader and close this row. (FLUX orchestrator)
