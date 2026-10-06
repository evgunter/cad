---
id: section-face-and-hole-rim-are-ordinals-over-their-group
kind: issue
title: SectionFace { section } and HoleRim { hole } are ordinals over a group, so a new section or hole renumbers the others
status: open
opened: 2026-10-06
priority: P3
parent: edge-pieces-are-named-by-their-ends
design: true
---


Found by the designer pair on PR 4134 (fork-log row 73), as off-question.

**The flaw.** Two role segments are ordinals over a group. That is the
same non-locality PR 4134 removed for crossings:
- `SectionFace { section }` numbers sections in completion order;
- `HoleRim { hole }` numbers a shell's holes in the kernel's pairing
  order (`names/emit_shell.rs`).

So adding a section or a hole can renumber the others, and rename what
cites them.

**The question.** What vertex- or face-local fact could name a section,
or a hole rim, in place of its number? This needs measuring before it
goes to a designer pair: does the renumbering actually happen through
today's edits?
