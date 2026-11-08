---
id: a-face-bend-holds-the-face-and-its-sense-twice-unchecked
kind: issue
title: FaceBend::Implicit holds the face twice (jet and round) and its sense twice, with nothing checking they agree; hand-stated jets never reach the witness
status: open
opened: 2026-11-03
priority: P3
cost: M
---

Found by PR 6385's re-review.

**The face, twice.** `topo/src/bound_side.rs` `FaceBend::Implicit` holds
the face's jet (`gradient`, `hessian`, `radius`) and, since PR 6385, its
material (`round: Option<link::Round>`, from `FaceBend::at(surface,
sense, p)`).

**The sense, twice.** The sense is once in the datum's normal
(`FaceDatum::of_face`'s `OutwardNormal`) and once in `round`'s
`convex`. Nothing checks that they agree. `Tie::witnessed` assumes
`Round`'s "away from material" is the datum's `Above`, and a caller that
passes a datum normal of the other face would flip the witness.

**Hand-stated jets.** `FaceBend::ball` / `FaceBend::cylinder` carry
`round: None`, so they never reach the witness. They are used at 16 test
sites (`bound_side.rs` 10, `boolean/vtxfac.rs` 4, `boolean/sectors.rs`
1, `boolean/recl.rs` 1), and may pin a path production no longer takes
for a sphere or a cylinder.

**Folded in.** `BoundSupport::Conic.arc`'s convention, `None` only for a
spiric or spline and for a conic read against a plane (`bound_side.rs`
≈84), is checked only at run time. A conic with no arc against a sphere
or a cylinder reaches `ReadFault::Unwitnessed`, a
`ClassificationInvariant`.

Seen once already (PR 6385's last merge, with ONEREAD's cone sites):
`boolean/recl.rs` `a_curved_flank_pair_tied_at_first_order_reads_alike_at_both_sites`
hand-built its bore's `FaceBend` with a sense opposite to its sector's
outward normal. The witness then read A's flank against the wrong side
and refused (`Coincidence(SectorSide)`, margin 0.414). The fixture now
passes the bore's sense. Nothing in the reading caught the
disagreement; only the test's expected verdict did.
