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

## Evidence: a kernel change to the join's visiting order re-points saved names (CLEAVE, PR 4224)

Measured by PR 4224's review, not by a document edit: PR 4224 changes
the split join's sweep order for every plane that is not an axis plane
(`topo::splitting::order::in_plane_frame` takes an oblique frame
there). A multi-region tilted split then completes its sections in a
different order, so a stored `SectionFace { side, section: 0 }`
resolves to the other section face, silently. Witness: a 4×4×4 brick
slotted along x, cut through `(2, 2, 3)` with normal `(0, 0.3, 1)`.

So the answer to this row's question is yes, and the renumbering does
not even need an edit to the document: any kernel change to the
visiting order (a frame, a pairing, a tie-break) does it. Nothing
reports or pins which section face carries which index today — no
test, no golden — so such a change lands without anyone seeing names
move.
