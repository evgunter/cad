---
id: a-value-on-a-placed-copy-target-is-charted-on-the-space
kind: issue
title: A value whose mate targets a Pattern or Transform copy is charted on the space's frame, not the copy's
status: closed
closed: 2026-10-28
opened: 2026-10-17
priority: P2
cost: M
refs: [5506, 5954]
---


INTENT stage 3 C charts a value on its mate's target body's own frame
(ruling C-1). That frame is read by `chart_body`
(`crates/editor-core/src/mate/solve.rs`): the placement whose copy the
target pose reads, with that placement's motion as the frame. It is
the identity for geometry in the space's own coordinates.

A target read off a copy that a `Pattern` or a `Transform` places is
no placement's copy. These nodes still place by a rule (§0's named
interims, which D and G end), and a pattern's value keeps no motion
per copy. `chart_body` therefore reads such a target as space
geometry, and a value on that mate is measured from the space's
origin, not from the named copy's own frame.

Effect: a seat with zero values on copy `i` of a pattern lands over the
space's origin rather than over copy `i`. The face is read at copy `i`,
so the mate's own equation holds, and only the values' origin differs.

Rows that would pin this, restated in C to the interim:
`p2_face::a_mate_on_a_pattern_copy_reads_the_masters_face_at_the_copy`,
`msolve13_read_at_operand::a2_a_part_and_its_pattern_naming_one_copy_are_one_member`.

D (`transform-retires-into-a-placement`) and G
(`patterns-are-index-variables`) make every such copy a placement's,
which ends this. If either lands without that, the chart must read the
copy's motion from the placer.

## Closed by G (`patterns-are-index-variables`)

A pattern is a placement over an index, so a target read off a member is read off a placement's copy, and `chart_body` reads that placement's motion as for any copy (D's `Transform` retirement ended the other half).
