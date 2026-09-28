---
id: certify-span-and-zero-arms-cannot-carry-their-decisions-full-ending
kind: issue
title: geom-brep: IntervalNotForward and WindingExceeded end in no routed recourse, and the zero arms quote no margin, because their variants carry no verdict or value
status: open
opened: 2026-09-28
---


(ENCL implementer, from the D4 ¶1 reshape of
`certify-escalation-renders-the-coincidence-menu-unlabelled`.)

## What

`geom_brep::certify::recourse` gives every refused arm of one
`CertCheck` its decision's one ending (D4 ¶1 (i)/(iv)). Two gaps
remain in `crates/geom-brep/src/certify.rs`:

- **The span decisions' definite arms are not routed.**
  `CertifyError::IntervalNotForward` answers both a Zero span (an edge
  of zero length: band-decided, so the lever and the conditional
  tolerance apply) and a Negative one (a reversed interval: sign
  certain, the lever alone), and the variant does not say which.
  `WindingExceeded` is `ParamWinding`'s sign-certain arm. Both keep
  their developer prose and name no recourse, while
  `Escalated { check: ParamSpan | ParamWinding }` ends in the span's
  lever.
- **A zero arm quotes no margin.** D4 ¶1 (i) phrases the tolerance
  "below m" on a Zero-classified arm. `NotTransverse`,
  `NotSecondOrderSeparated` and `PlaneNurbsRefusal::NotTransverse`
  carry only a sample index, so `RefusedArm::Zero` renders the
  conditional without a value.

- **The tube's definite arm conflates Zero and Negative.** The tangent
  lane's uniqueness tube (`tangent_tube_margin` in `certify_via`,
  `crates/geom-brep/src/certify.rs` ~:2501) maps
  `Ok(Sign::Zero | Sign::Negative)` to one
  `NotSecondOrderSeparated { sample: 0, .. }`, and that variant reports
  `RefusedArm::Zero`. A Negative tube margin is sign-certain, so it
  should end in the lever alone; it ends in the lever and the
  conditional tolerance instead, which no smaller tolerance fulfils.
  The same shape as `IntervalNotForward`, one decision over.
- **The collapsed-arm gates are their own decision.** `dihedral_arm`
  (`crates/geom-brep/src/dihedral.rs`, `classify_dihedral`) and
  `nurbs_span_meter` (`certify_via`'s NURBS span arm) run
  `decide_positive` on a lever arm or a metered extent before the
  decision proper, and a definite non-positive verdict escalates as
  `MarginDiag::Invalid`. Certification reports that escalation under
  the decision it guards (`Transversality`, `ParamSpan`), so it ends in
  that decision's lever plus the unreadable-margin note ("an unreadable
  or collapsed margin may indicate a kernel bug worth reporting"). The question those
  gates ask — is there an arm, is there a metered extent — is a
  decision of its own, with its own verdict and its own ending, and
  routing it as the guarded decision's poisoned margin is a stand-in.

- **The plane × NURBS lane's tube conflates them too.**
  `certify_rung3` (`crates/geom-brep/src/ssi/certify.rs` ~:916-921)
  raises `SsiError::TubeStraddles` on
  `decide("ssi_tube_transversality", …) == Ok(Sign::Zero |
  Sign::Negative)`, and `edge_nurbs::refusal` forwards it as
  `PlaneNurbsRefusal::TubeStraddles`, whose `certified_clearance` can
  be positive inside the zero band. PR 3351 routes it through
  `RefusedArm::ZeroOrNegative` (the transversality lever alone at every
  reading), which holds for both halves but gives neither its own
  ending.

Until their verdicts are split, `IntervalNotForward` and
`WindingExceeded` can route the same way, through
`RefusedArm::ZeroOrNegative` on `ParamSpan` / `ParamWinding` — the
lever alone — rather than ending in no recourse at all.

## Repair shape

Give `IntervalNotForward` its verdict (or split it) and route both span
arms through `recourse`; carry the classified margin on the three zero
arms (`classify_dihedral`'s `Smooth`, `tangent_second_order`'s zero,
the lane's per-sample transversality) and let `RefusedArm::Zero` take
it.

Split the tube's definite arm as the span's (Zero to
`NotSecondOrderSeparated`, Negative to a sign-certain arm). Give the
collapsed-arm gates a `CertCheck` of their own (or carry the gate's
verdict on the escalation) so `recourse` routes them as themselves.

## Note (from the offset-meters reshape)

The shared table now takes a valued zero arm:
`geom_brep::recourse::RefusedArm::Zero(Some(Classified { margin, band }))`
ends in the conditional tighten below `m/K` when `m > 0`, and otherwise
in the lever alone (plus `SizedDecision::at_zero` where the decision has
one). The offset meters use it. certify's zero arms pass `Zero(None)`
because their variants carry no margin. The repair here is for the
variants to carry `(margin, band)`. Once they do, delete `Zero(None)`,
the `Option` in `RefusedArm::Zero`, and `SizedDecision::recourse`'s
`tighten(None)` (the unvalued conditional arm): nothing else produces
them.

A decided margin is spelled three ways: `recourse::Classified`
(margin, band), `offset_meters::Refused` (the verdict carrying a
`Classified`), and `sweep::blend::ClassifiedMargin`
(`crates/sweep/src/blend/mod.rs` ~:241: predicate, reading, band,
sign). The payload types should converge on one of them.
