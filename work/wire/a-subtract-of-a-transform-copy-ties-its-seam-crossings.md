---
id: a-subtract-of-a-transform-copy-ties-its-seam-crossings
kind: issue
title: A subtract whose operands share face names (a Transform copy) publishes its four Crossing seam vertices Tied, because the pair's seam sides and crossings cite bare operand names
status: closed
opened: 2026-10-20
priority: P2
cost: M
refs: [should-pair-seams-take-name-ordered-sides]
pr: 5469
branch: wire/pair-seam-name-order
closed: 2026-10-20
---


Measured by FORK-PAIRSEAM's designer probe (worktree off origin/main
`0c9595b606`):
- Box `x` at [0,4]×[0,2]×[0,1].
- Tool `Transform(x)`: `Step::Rigid`, axis (h,h,0), angle π, translation
  (1,−1,1.5).
- Then `Subtract { from: x, tool }`.

`Transform` adds no name segment, so both operands' tables hold the
same names:
- The four `Crossing { edge, face }` seam vertices publish Tied. "From's
  edge crosses tool's face" and "tool's edge crosses from's face" are
  spelled alike.
- Every edge whose `Ends` cites one of these vertices inherits the tie.
- The seams themselves stay distinct, but only through their `Sided`
  slot order: `Seam{Cap(End), Lateral₂₂}` against
  `Seam{Lateral₂₂, Cap(End)}`.

Root: `emit_topo`'s `plain` / `upstream_name` and the seam-vertex pass
cite the operand's bare face name rather than the face's name in the
boolean's own table (`From { read, of }`). The fix is FORK-PAIRSEAM's
question; `Transform` sits on D10 ground, but the fixture only exercises
the naming.
