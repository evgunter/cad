---
id: an-in-face-crossing-is-spelled-as-a-seam-of-the-face-it-lies-in
kind: issue
title: A vertex where an edge lying in a face ends in it is published as Seam{merged face, its own seam}, losing which member it marks
status: open
opened: 2026-10-06
priority: P3
parent: edge-pieces-are-named-by-their-ends
design: true
---


Found by the designer pair on PR 4134 (fork-log row 73), as off-question.

**The case.** In `wire_legal_union_refusals`, the area-overlap fold
fixture, a short seam of `s ∪ big` lies wholly in member `a`'s y = 0
wall. Its two ends are ranked today as "crossings" of that wall. At the
union's collapse they are re-cited as degree-2 vertices on the long
published seam, spelled
`Seam{Merged(a.wall, s.wall), Seam{big.Cap(Start), Merged(..)}}`. That is
a face the edge lies on, not a face that crosses it. The spelling also
loses which member (`s`) the vertex marks the end of.

Under PR 4134's ruling these vertices carry their sense, read at minting
and carried through the collapse, so they are told apart. This row asks
whether the published spelling should say what the vertex is: the end
of a member's coplanar stretch inside a merged face.
