---
id: edge-in-face-spans-are-re-derived-in-the-carry-because-the-sweep-skips-coplanar-edge-face-pairs
kind: issue
title: An edge-in-face span is re-derived in the carry by midpoint containment, because the reduction sweep skips coplanar edge–face pairs rather than deciding them
status: closed
opened: 2026-10-19
priority: P1
cost: M
parent: the-parting-reads-no-edge-lying-in-a-face-no-record-names
branch: parting/sweep-records-edge-in-face
closed: 2026-11-01
pr: 6225
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

## Claimed by PARTING (2026-11-01)

Moved from `work/contacthold/` and raised P3 → P1: it is the first step
of the final state the designers agreed on for
`the-parting-reads-no-edge-lying-in-a-face-no-record-names` (see that
row's `## Design`), and that row's parting half waits on it, so it
carries P1 by the blocker rule (`work/README.md`, Priority). Scope here:
the sweep decides a line or a conic lying in a plane face and records
one edge-face record per piece the face holds, cited by that decision;
`record_spans` retires. Curved faces wait on ContactHold's
`an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane`,
since a curved edge-face record the census cannot confirm refuses at the
carry. (PARTING orchestrator)

## Closed

2026-11-01, PR 6225 (full review, fix pass, delta review, final pass). The reduction sweep records an edge-face record per piece of a line edge lying in a plane face, each citing the records that place its own two ends on the face; `record_spans` and `RestPieceUnread` retire. Conics in planes and curved faces wait on ContactHold's curve-touch lane. Residue: `contact-record-insertion-and-cell-mapping-have-several-homes`. (PARTING orchestrator)
