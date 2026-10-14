---
id: a-pair-boolean-has-no-spelling-for-one-line-meeting-several-faces
kind: issue
title: A pair boolean has no N2 spelling for one operand line meeting several faces of the other operand and no edge of it
status: closed
opened: 2026-10-13
closed: 2026-10-13
branch: emit/glued-rim-build
priority: P3
cost: E
---


## What

Split off `a-pair-boolean-names-an-edge-bearing-meeting-as-a-seam-junction`
when its arm was converted. At a pair boolean's seam vertex where two or
more seam lines meet, `emit_topo::name_boolean_vertices` counts each
side's operand edges as lines, flush edges once (`flush_lines`). N2
names one line meeting one face of the other operand as that line's
`Crossing { edge, face, sense }`, and two or more lines as their
`EdgeCrossing` run. It has no spelling for one line meeting several
faces of the other operand with no edge of that operand at the vertex:
a `Crossing` cites one face, and N2's run needs two lines (a pair
boolean's seams are not lines; only a union's are). The arm refuses it
as a missing rule, `NamingError::SeamVertexParentage`. Before the
conversion it was minted as a seam junction (the run of the seam
lines' `Seam` segments).

## Measured

No test reaches it. Over the full editor-core suite, the converted arm
took 124 vertices, all in union fold steps, every one a pinch whose
edges collapse to one line meeting one face:
`(A edges, B edges, A faces, B faces)` = `(2, 0, 4, 1)` ×65,
`(3, 0, 6, 1)` ×30, `(4, 0, 8, 1)` ×24 and `(0, 2, 1, 4)` ×5.

## The question

What names it. Options seen:
- the line's `Crossing` with the least face in name order (a choice N2
  does not make today: name order spells sets);
- the line and the faces' seams as a run, as a union counts a seam (a
  pair boolean then spells seams as lines, which N2 reserves to unions);
- the old seam-junction spelling, kept for this shape only.

The shape needs a witness first: a line through a point where several
faces of the other operand meet and none of its edges remains.

## Reaching it

Conjecture, unproven: it takes a declared merge consuming the other
operand's edges there. Nothing below rules out an undeclared reach.

**Why an ordinary pair is hard.** Several faces of one operand meet at
a point only along that operand's edge `e` through the point. For `e`
to be gone at the vertex while its faces reach the vertex, `e` must lie
on the discarded side of the line operand on both sides of the point.
Near the point the line operand is a wedge about its edge. A convex
wedge (under π) holds a line through its apex on one side at most, so
`e` survives on the other side and the vertex is the two edges' run,
`(1, 1)`. Only a wedge over π, a reflex edge in the sense of the op
(kept material over π about `e`'s line), can hold `e` on the discarded
side both ways, and the faces of `e` must still reach the point between
its walls.

**The review's attempt (PR 4958).** A tilted box whose x-edge crosses
the reflex edge of a 90° V-notch prism, all four ops at tilts of 20°,
30° and 45°: 12 runs. All evaluate, and none reaches the refusal. The
box's edge is convex, so by the argument above each run keeps a piece
of the other operand's edge at the point.

## The union raises it too

A union reading the same shape off its finished body
(`emit_union::Flush::meeting`: one member line, no seam, several faces
of other members) raised `NamingError::Emission` ("one member edge meets
other than one face …"). It now raises the same typed
`SeamVertexParentage`. One member line meeting no face stays an
emission bug. No test reaches either.

## Ruling (Ev, PR 4965, fork-log row 131)

Not named. Owed: move the shape from `SeamVertexParentage` to the `Emission` residue in both emitters, and state the link argument beside it. With planar faces, the kept region either keeps the other operand's edge, holds one of its faces, or pinches into two vertices, so a manifold pair does not reach this shape. Then close this row as unreached, citing both witnesses that pinched instead:
- an L-prism with a box edge (union);
- a V-cutter on a box edge (subtract).

## Closed (branch `emit/glued-rim-build`, 2026-10-13): unreached

The shape is now an `Emission` refusal in both emitters, with the link
argument beside it:
- `emit_topo::name_boolean_vertices`, the converted arm's
  `([_], []) | ([], [_])`;
- `emit_union::Flush::meeting`, one line with several faces.

The argument: faces of the other operand (or of other members) meet at a
point only along their edges through it. With planar faces, the region
kept about the line either keeps a piece of such an edge at the point,
holds one of its faces there, or pinches into two vertices, each one line
meeting one face. A manifold pair does not reach this shape.

Neither designer's witness reached it. Each pinched into two `Crossing`
vertices instead (fork-log row 131):
- an L-prism unioned with a box edge;
- a V-cutter subtracted on a box edge.

`NamingError::SeamVertexParentage` keeps one raiser:
`name_boolean_vertices`' A-side arm for one A edge with no B edge and no
seam at the vertex. Its doc and message now say exactly that.
`refusal_concision_chains` and `display_contract` stay green.
