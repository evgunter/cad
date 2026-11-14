---
id: a-family-placed-from-a-previewed-copy-is-not-carried
kind: issue
title: The placement editor's preview carries no family: a row placed from a copy the draft moves, or a row's own placement, is drawn where it stood
status: open
opened: 2026-11-14
priority: P3
cost: M
---


Found by the review of PR 6938 (its MINOR 1, probes A and B), filed by
`placing/p3p4-cleanup`.

The placement editor previews a waiting draft by one carry per node
(`display::PreviewedCopy::carry`), and
`pickindex::preview_copies` (`crates/viewer/src/pickindex.rs`) admits
a copy only where its value is one body and the index draws one part
of it. A placement over an index evaluates to a family (`Instances`),
so neither of these is drawn where the draft stands it:

- **A.** A row of copies placed from the startup copy, while a waiting
  slide moves that copy 5 mm: the row is drawn unmoved.
- **B.** The row's own placement slid in the editor: the panel says
  "the copy is drawn where it would stand", and the row is drawn
  unmoved.

Both held before PR 6938 too. The heatsink is this shape: its fin row
is placed from the seated fin placement (`demos/tour/src/heatsink.rs`,
the `linear_pattern` over `fin_e`). One carry per node cannot move a
family: each member's frame is its own pose over the source's, so a
change of the source moves the members by different maps.

**Owed.** Carry per drawn part (node and member body) rather than per
node, composing each member's frame from the scratch evaluation, or
say in the panel's line that a family is drawn where it stands. **Done
when** a row holds probe A's draft and reads every member of the row
drawn moved with its source, and probe B's reads the row drawn where
its placement's draft stands it.
