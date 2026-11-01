---
id: a-gates-rejected-zero-on-the-undecided-arm-drops-at-zero
kind: issue
title: A gate's rejected Zero routed as RefusedArm::Undecided never shows the decision's at_zero note
status: closed
opened: 2026-10-12
priority: P3
cost: E
branch: encl/rejected-zero-at-zero
pr: 4855
closed: 2026-10-12
---


(Found by the `a-proven-zero-margin-still-offers-to-tighten` lane, PR 4840, in review.)

A gate that rejects a `Sign::Zero` returns an `Indeterminate` whose margin carries the rejection tag. The tag is minted in geom-core `k_stats.rs` (`margin.rejected_by(sign, passes)`, about k_stats.rs:512). Readers take that escalation onto the shared table as `RefusedArm::Undecided`, for example topo `splitting/containment.rs` `Escalation::arm` (`Self::Margin => RefusedArm::Undecided(diag)`, about :276), and every `RefusedArm::Undecided(diag)` call site in topo and geom-brep.

`geom_brep::recourse::SizedDecision::recourse` treats its arms differently:
- The Zero arm passes the decision's `at_zero` note as `otherwise`: `self.words(at_zero.map(|note| note.at(reading)), reading)`, about recourse.rs:436.
- The Undecided arm passes `self.words(None, reading)`, about recourse.rs:455 and :459.

So a gate's rejection of a Zero never shows `at_zero` on a decision that has one. The decisions with one are in certify.rs, offset_meters.rs and dihedral.rs (`at_zero: Some(..)`). This holds even where the ending has no tolerance to offer: a zero on the rejected side, or, since PR 4840, a margin the symbolic tier proved zero.

The `Indeterminate::reading` word ("a decided zero …", "a zero the symbolic tier proved …") already says the reading is a zero. So the missing piece is the decision's own note, not a false offer.

Repair direction: let `SizedDecision::recourse`'s Undecided arm take `at_zero` where `cause.margin.rejected_sign() == Some(Sign::Zero)`, or route a tagged Zero onto `RefusedArm::Zero` at the readers. Pin it with a gate-rejected Zero on a decision with `at_zero`.
