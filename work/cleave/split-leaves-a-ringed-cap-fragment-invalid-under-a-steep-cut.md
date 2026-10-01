---
id: split-leaves-a-ringed-cap-fragment-invalid-under-a-steep-cut
kind: issue
title: A steep plane split through a bored solid leaves one cap fragment's ring touching its outer loop and the other cap fragment inverted (RingMeetsOuter, LoopRoleInverted)
status: dispatched
opened: 2026-09-28
cost: H
priority: P0
branch: cleave/section-rings
---

Found by CONTACT-6 while measuring a split's section-face senses
(`crates/sweep/tests/pis_cut_cavity.rs`, `STEEP_RESIDUE`).

**Fixtures.** Both are built through the public doors:
- the brick `[−2, 2]² × [0, 2.5]` minus a unit rod about `(0.8, 0)`,
  split through `(0, 0, 1.25)` with normal `−(sin 1.4, 0, cos 1.4)`;
- `bored_cylinder(0.3, 0.2, 0.37)`, split through `(0, 0, 0.5)` with
  normal `−(sin −1, 0, cos −1)`.

**What tier 3 finds.** These are on OPERAND cap faces, not on section
faces:
- **Below:** the cap fragment keeps the bore's rim as a ring, and one
  of the ring's vertices lies on an outer edge:
  `RingMeetsOuter { contact: VertexOnEdge }`. That is face 2v1 on the
  brick and face 1v1 on the cylinder.
- **Above:** one cap fragment has its outer loop wound against its
  sense: `LoopRoleInverted`. That is the `z = 0` cap (15v3) on the
  brick and the `z = 1` cap (13v3) on the cylinder, each `sense: true`
  with no rings.

The same findings are there at `a8f006ab5` (main before CONTACT-4) and
at CONTACT-6's head. On the base, the above half also had a
`LoopRoleInverted` on a section face, which CONTACT-6 removes.

`point_in_solid` reads both halves correctly on a 9³/11³ grid at six
poses (0 wrong), and the volumes integrate from the windings. So the
defect is in the split's topology of a ringed operand face that the
cut crosses. Its sibling is the hole-section encoding, where a square
plus a cancelling disc stands in for one face with a ring.

Other nearby settings refuse the split instead of answering:
- the rod at `(0.4, −0.3)` refuses `Join(RingHomingAmbiguous)`;
- the rod at `(0.5, 0)` refuses `Finish(TornComponent)`.
Both refusals are the same at base.
