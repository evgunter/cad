---
id: a-curved-coincident-wall-ships-its-seam-where-operand-a-ends
kind: issue
title: Two coincident curved walls ship their seam where operand A's copy ends, so the finished body differs by operand order
status: open
opened: 2026-10-20
priority: P3
cost: M
refs: [5368]
---

Found by the second review's re-check of PR 5368 (frozen `1fe2763cf0`, executed, body counts plus border coordinates).

**What happens.** A union of two coincident curved walls ships an unmerged seam where operand A's coincident copy ends. The merge does not join curved faces, so the seam moves with the operand order:
- coaxial rods over z 0..2 and z 1..3 ship the seam circle at z 2 with the lower rod as A, and at z 1 when swapped;
- a slanted cylinder against a coaxial one ships it at z 0 in one order and at the ellipse in the other.

Face and edge counts are equal in both orders; only the seam's placement differs. The merge base behaves identically, so PR 5368 did not cause it. Its design records it as a carve-out to "the finished body is the same in both orders up to the swap" (row `coincident-faces-are-not-imprinted-on-each-other-before-the-lump`, `## Design`, Symmetry), and the pin is `sweep` `curved_coincident_fates::coaxial_rods_border_the_seam_circle_only_where_the_result_holds_it`.

**Owed:** decide whether the merge should join two coincident pieces of one curved carrier, as it does for planes, so that the finished body is order-free. If it should, build it on the merge's ground, then remove the carve-out and its pin.

