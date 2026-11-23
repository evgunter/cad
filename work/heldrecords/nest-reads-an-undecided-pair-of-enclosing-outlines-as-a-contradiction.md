---
id: nest-reads-an-undecided-pair-of-enclosing-outlines-as-a-contradiction
kind: issue
title: The section nesting refuses NestingContradiction, a kernel bug, where two outlines enclosing a hole read each other undecided
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Found by the review of PR 7436 (`heldrecords/nesting`).

## What

`splitting::section_loops::nest` reads `encloses(outer, inner)` as
`false` both where `inner` is decided outside `outer` and where nothing
decides it: `outlines_disjoint` in the band, or `point_in_loop` not
`In`. Among the outlines that enclose a hole, the innermost is the one
every other encloses. Where two outlines both enclose the hole and
each reads the other undecided, both are innermost, and `nest`
returns `NestFault::Contradiction`. The split renders that
`SplitFinishError::NestingContradiction` ("a kernel bug",
`defect_ended`), and `plane_section` renders it
`SectionError::NestingContradiction`, also a defect.

That is an undecided case, not a contradiction. Disjoint loops
cannot each enclose the other, but loops not *decided* disjoint
can come within the band of each other. Now that
`nest_hole_sections` refuses `UnplacedHole` instead of keeping a face
of its own, this is the one path where an undecided nesting reading
names itself a kernel bug.

## Owed

`encloses` returns its reading three ways (in, out, undecided). A pair
of enclosing outlines whose mutual reading is undecided refuses as
undecided: `UnplacedHole`, or an arm that names the two outlines,
with the coincidence recourse. Keep `Contradiction` for two outlines
each *decided* to enclose the other. Add a row with two outlines
within the band of each other around a hole, at a pose that reaches
it.
