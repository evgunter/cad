---
id: the-side-of-a-spline-face-is-read-by-no-dihedral
kind: issue
title: The dihedral side reading has no arm for a spline face: classify_dihedral and implicit_residual are poison on a NURBS surface
status: open
opened: 2026-10-26
priority: P1
cost: M
refs: [a-span-is-read-against-a-faces-boundary-edges-first, a-unions-flush-reading-cannot-follow-a-nurbs-carrier]
---


Filed by the SPLICE orchestrator from the design Ev took on PR 5727
(fork log row 198), on a designer's finding.

Once a shared rim is glued at the coincidence door, what remains is
which side of the shared edge each wall lies on. `classify_dihedral`
and `implicit_residual` are poison on a spline surface, per the curved
arm's own comment. The reading needs a spline arm built from
`patch_bound`'s signed normal enclosures. Lily wall 8 needs it beside
the boundary-first reading and the two flush rows.
