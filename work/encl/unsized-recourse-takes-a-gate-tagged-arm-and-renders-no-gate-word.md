---
id: unsized-recourse-takes-a-gate-tagged-arm-and-renders-no-gate-word
kind: issue
title: Unsized::recourse accepts a gate-tagged arm, and an escalation's labelled ending drops the cause's 'decided sign this decision cannot use' phrase, so nothing tells the user a gate rejected a decided sign
status: closed
opened: 2026-10-12
priority: P3
cost: M
branch: encl/unsized-gate-word
pr: 5428
closed: 2026-10-20
---

(Split from `work/pcert/a-villarceau-tangent-parts-gated-zero-ends-as-a-kernel-defect.md` and from review of PR 4874. `recourse.rs` is ENCL ground, which is why this row is ENCL's.)

## What

**By type.** `geom_brep::recourse::Unsized::recourse` (recourse.rs, `impl Unsized`) takes any `RefusedArm`. That includes `RefusedArm::Undecided(cause)` whose margin carries a gate's rejection tag (`geom_core::MarginDiag::rejected_sign`, minted by `k_stats::mint_rejection` under `decide_positive` and kin). It never reads the tag. `SizedDecision::recourse` does read it, for its `at_zero` note. PR 4874 first added a doc line, "no unsized decision is a gate", with a `debug_assert!` on the tag, and dropped both because the tree contradicts them. The counterexample is `pcurve_cache` `torus_oblique_circle` (about pcurve_cache.rs:8247), where `decide_positive` → `incidence_escalated` → `PcurveCheck::ChartIncidence` → `Unsized::Defect`.

**In words (evidence from PR 4874).**
- `Indeterminate`'s own rendering names a gate's rejection: `geom_core::predicate` `Indeterminate::reading` (about predicate.rs:2351) gives a past-the-band margin "a decided sign this decision cannot use".
- Since PR 4874, both `PcurveCertifyError` escalation arms render `cause.payload()` plus their decision's labelled ending (`SsiDecision::ending`, `PcurveCheck::recourse`). That drops the under-tail, and that phrase with it.
- So for a gate-tagged cause, neither the payload nor the ending tells the user that a gate rejected a sign the classifier decided. They read "undecided".

## Question

Should `Unsized::recourse` refuse a gate-tagged arm by type? For example, a gate's rejection could reach the table as its own `RefusedArm` (`Zero` or `SignCertain` with the tag), never as `Undecided`. Or should every unsized row read the tag, as `SizedDecision` does?

Separately: should an ending composed from `payload()` carry the reading phrase where the cause is gate-tagged? It could route through the `Indeterminate::ending` family instead.

- 2026-10-13 (ENCL implementer, `encl/no-size-note`): `SizedDecision::recourse` no longer reads the gate's tag. Its note now comes from one classification of the reading on `geom_core::MarginDiag` (by reading, band and pass set), which `Indeterminate::reading`'s word and `MarginDiag::sized_recourse`'s ending both read. "a decided sign this decision cannot use" is now that classification's Past class, the same one under which the ending adds nothing. The "By type" contrast with `SizedDecision` above is therefore moot. The question of whether a `payload()`-composed ending should carry the word still stands, and so does the `Unsized` question.
- 2026-10-13 (ENCL implementer, `encl/definite-arm-one-story`): `MarginDiag::shows` now classes a reading past the band that lies wholly on a refused side as its own class, `Shows::Refused`, which ends as `NoSize` does, so its ending carries the decision's note. `Indeterminate::reading` gives it and `Past` alike "a decided sign this decision cannot use", so the gate's word is unchanged (`k_stats_doors::a_rejection_tells_the_decided_margin_it_carries`).
