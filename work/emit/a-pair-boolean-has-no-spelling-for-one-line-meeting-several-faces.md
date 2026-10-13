---
id: a-pair-boolean-has-no-spelling-for-one-line-meeting-several-faces
kind: issue
title: A pair boolean has no N2 spelling for one operand line meeting several faces of the other operand and no edge of it
status: open
opened: 2026-10-13
design: true
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
faces of the other operand meet and none of its edges remains, which
takes a declared merge consuming the other operand's edges there.
