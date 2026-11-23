---
id: the-census-reads-its-link-without-the-wide-parity
kind: issue
title: The census reads its link drawn in and without the wide parity, so Gap and the census read one cone two ways
status: open
priority: P2
cost: M
opened: 2026-10-26
refs: [material-and-convexity-are-read-three-ways]
---

`topo::link` reads a point against a ray two ways, by `Reading::own` (`crates/topo/src/link.rs`):

- **`Link::offset` and `Link::lowest_on`** (`Gap`'s cell arms) read with `own`: a point is read against a ray at its own distance (`wedge_side`, `ahead`).
- **The census's touch analysis** (`Cone::reads_over` through `rest::judge`, `rest::one_material`, `Cone::closes`, `Link::pieces`) reads without it: points are drawn in to the ray's real point first.

Both read the wide parity (`Cone::against`): a plane a point lies in band of, decidedly outside its sector, is not crossed. That was this row's item 2, landed by ONEREAD (`link::tests::a_point_in_a_planes_sliver_outside_its_sector_reads_out` pins it).

The split was made because the census had to stay bit-identical while `Gap` needed the own reading. The dual review measured the cost of the census's reading on `Gap`: 831 of 1500 random edge pairs refused, and a hole's short ray read the wrong sign 5e-3 out. Both are fixed for `Gap`. The census's drawn-in reading leaves only refusals, never wrong verdicts, but it leaves touches unread that the own reading would read.

**When the census can take it.** Drawn in is part of the touch analysis's soundness argument: a ray is never read past the real point that fixes it (`wedge_side`'s doc, `rest::judge`'s *each verdict at its own scale*). A crossing is confirmed inside the ball both links fill, and there the drawn-in and own readings agree. Rest certificates are read at the pieces' real points. The census can read `own` once that argument is re-made for an unbounded cone, or once it is shown that every census reading already lies within each ray's reach. It lands with the census goldens at three ε re-blessed, the moved rows named, and `link_fuzz` and `cone_fuzz` green.

Until then the two readings are one module's two modes, and `link::tests::the_link_decides_only_through_its_doors` holds both to one door.

## Note (2026-11-01, ONEREAD)

- Item 2 (the wide parity) is taken by ONEREAD's `the-link-splits-into-a-cone-and-a-link`, with the census goldens re-blessed; item 1 stays here.
- Evidence on item 1, measured by a ONEREAD designer on `cone_fuzz` (seed `0xa92fdc75aec72e5a`, effort 1): reading with `Reading::own` decides 86 of 998 probes in band at ε 1e-9 (none contradicted by the exact oracle, each wrong under the fuzz's D4 rule). `Gap` publishes `offset`'s sign under that reading, so whether a measure's sign owes D4 may make `own`'s conflation of "at its own distance" with "wide parity" a live question for `Gap` too. (ONEREAD orchestrator)
