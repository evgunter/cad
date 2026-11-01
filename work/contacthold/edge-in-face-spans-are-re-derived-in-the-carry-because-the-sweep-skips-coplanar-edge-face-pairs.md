---
id: edge-in-face-spans-are-re-derived-in-the-carry-because-the-sweep-skips-coplanar-edge-face-pairs
kind: issue
title: An edge-in-face span is re-derived in the carry by midpoint containment, because the reduction sweep skips coplanar edge–face pairs rather than deciding them
status: open
opened: 2026-10-19
priority: P3
cost: M
---


Found by a FLUSH designer while weighing
`work/flush/coincident-faces-are-not-imprinted-on-each-other-before-the-lump.md`
(2026-10-19). It was read, not measured.

**The defect.** `record_spans` (PR 4737, "a line edge in a plane face
records its span") reconstructs an edge-in-face span from the edge's
bounds by midpoint containment. It has to, because the reduction sweep
skips coplanar edge–face pairs instead of deciding them. So whether
that edge lies in that face is decided nowhere and re-derived in the
carry. `[coincidence.door]` asks that every coincidence the kernel
infers from values is decided and recorded at the one door.

**What is owed:**
- confirm it on PR 4737's fixtures;
- decide coplanar edge–face pairs at the door (or in the sweep, as a
  recorded verdict);
- have the carry read that verdict rather than re-deriving it.
