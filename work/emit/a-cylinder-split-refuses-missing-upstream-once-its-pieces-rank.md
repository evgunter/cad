---
id: a-cylinder-split-refuses-missing-upstream-once-its-pieces-rank
kind: issue
title: A cylinder split whose same-side pieces are let through refuses MissingUpstream on the extrude
status: closed
opened: 2026-09-30
priority: P0
cost: M
closed: 2026-09-30
---


## What

A cylinder (`circle(0, 0, 0.5)` extruded 1.0) split by a plane that
crosses its wall twice (through `(0, 0.2, 0)`, normal `(0, 1, 0)`)
refuses today at the split ranker's `face_plane`
(`a-plane-split-of-a-curved-face-refuses-as-an-emission-bug`). A
designer lane on the curved-seam fork (fork-log row 22) let the
same-side pieces through as a tie in a scratch probe (not committed);
the split then refused `MissingUpstream { node: <the extrude> }`.

So a second, undiagnosed defect sits behind the first: whatever rule
the fork settles for the split's same-side pieces, this recipe will
still refuse. Root-cause it independently of the fork: reproduce by
tying the same-side group in the split fragment ranker, find which
lookup expects an upstream name the extrude never published.

## Closed

**Root cause.** The split's edge and vertex lane chased an operand
edge's split lineage inside one half. Both halves are carved from one
scratch arena, and a half keeps only the records of the keys it holds
(`SplitNaming`). The plane crosses the start rim arc twice. The second
crossing splits the first crossing's child, and that middle piece lies
Above. So the Below crossing vertex's record, and one Below rim piece's
record, name a key the Below half does not hold. Chased inside Below,
the lineage stopped at that key. The extrude never named it, and the
crossing-vertex lane's `upstream_name` refused `MissingUpstream`. The
edge lane would have refused next with "edge descent reached no
operand edge".

**Fix.** `emit_topo::chase_split_edge_to_table` chases across both
halves, reading each hop's record from the half that holds the key.
The face ranking fork plays no part: the twice-crossed rim is the
cause.

**Planar reach.** None. A plane crosses a straight edge at most once.
The recipe that reaches this with no face ranking is the cylinder split
by the plane through `(0, 0.2, 0)` with normal `(0, 1, -1)`: it crosses
the start rim twice and leaves the wall and cap one piece per side.
That recipe refuses `MissingUpstream` on main.

**Left.** The split still refuses there, now with `Duplicate`: a
twice-crossed edge's same-side pieces and crossing vertices share one
name and the split has no multiplicity rule for them. That rule is
part of the ranking fork and is filed as
`a-split-mints-a-twice-crossed-edges-pieces-under-one-name`. The same
broken lineage inside one half reaches `topo::props` and `mesh::memo`;
that is filed as `a-split-half-loses-the-lineage-of-a-twice-crossed-edge`.
