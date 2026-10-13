---
id: unsized-fit-reads-a-rejected-zero-on-the-undecided-arm-as-the-last-resort
kind: issue
title: pcurve_cache's FittedEscalated drops which SSI decision escalated and ends every escalation as Unsized::Fit, so one SSI decision ends two ways across doors
status: closed
pr: 4874
opened: 2026-10-12
priority: P3
cost: E
branch: encl/fitted-escalated-keeps-decision
closed: 2026-10-12
---


(Found by the `a-gates-rejected-zero-on-the-undecided-arm-drops-at-zero` lane's sweep.)

## What

`geom_brep::recourse::Unsized::recourse` splits its `Fit` arm by `RefusedArm` at rest: `(AtRest, Undecided | Straddle)` ends in `KERNEL_LIMIT_RECOURSE`, and `(AtRest, Zero | SignCertain)` ends in the file's defect ending. It does not read the gate's tag (`MarginDiag::rejected_sign`). So a gate's rejected Zero that a reader routes as `RefusedArm::Undecided` ends in the last resort at rest. The same verdict routed as `RefusedArm::Zero` ends as a stored contradiction.

`SizedDecision::recourse` now reads the tag for its `at_zero` note. `Unsized` does not.

## Readers at rest that can carry one

Only a form selection (an `Unsized` decision that passes on a definite sign) can see a rejected Zero. A residual passes at zero.
- `geom_brep::ssi` `TraceDecision::ending`, `ClosureReturn` (about ssi.rs:2226). Its doc says the last resort is the intended ending there: "only where an untrusted marched sample landed against the seed refuses it, which no lever the caller holds moves". So the Zero arm's defect ending would be wrong for it.
- `geom_brep::pcurve_cache` `PcurveCertifyError::ending`, `FittedEscalated` (about pcurve_cache.rs:1952): the cause is the fitted lane's SSI certificate's. Not traced.
- `geom_brep::props` `PropsCheck::ending`, `Converged` (about props/mod.rs:630). Not traced.

## Question

Decide whether `Unsized::Fit`'s at-rest split is by verdict or by route. If it is by verdict, read the tag in the table, as `SizedDecision` does, and keep `ClosureReturn`'s last resort by its own row. If it is by route, say so in `Unsized::recourse`'s doc.

## Re-scoped (2026-10-12, ENCL orchestrator, from one designer)

The premise above does not hold on the tree. No reader routes a gate-rejected Zero into `Unsized::Fit`:
- Only gates (`decide_positive` and kin, through `k_stats::mint_rejection`) write the rejection tag.
- No unsized decision is a gate, so `Unsized`'s arms are its verdicts.
- The three cited readers decide with the plain `decide` (`ssi/march.rs` `ssi_closure_return`; props via `quad::classify_len`; the fitted lane via `certify_branch`), which never tags.
- The `ClosureReturn` doc's last resort is agent-written (`689d80a2f4`), reasoned, and stays.

The real defect: `pcurve_cache.rs` `ssi_refusal` folds `SsiError::Escalated { decision, .. }` and `SsiError::CertificateEscalated { limb, .. }` into `FittedEscalated { cause }`, dropping the decision. `PcurveCertifyError::ending` then ends every one of them as `Unsized::Fit`. That breaks "each decision ends by its own kind, the same at every door", and `SsiLimb::check`'s doc ("every door that reports the limb reads"). Example: limb 3 (the uniqueness tube) escalating in band ends `CertCheck::Transversality` (sized: "move the geometry so the surfaces cross at a clearer angle" plus the tighten offer) at the SSI door, but in the last resort through the pcurve door at rest (`topo::validate`). Poisoned margins diverge too: Fit gives the defect ending, while the sized check keeps its lever plus the unreadable note. `edge_nurbs::refusal`'s `PlaneNurbsRefusal::Escalated { limb, cause }` is the precedent that keeps the limb.

Repair: `FittedEscalated` carries the SSI decision (the escalation, or a small `TraceDecision`/`SsiLimb` enum) and ends through `SsiError::ending`. Add one line to `Unsized`'s doc: no unsized decision is a gate, so its arms are its verdicts. Optionally add a debug_assert that an `Unsized` cause carries no gate tag. Expected re-baselines:
- `topo::validate`'s at-rest endings for fitted-lane escalations (validate.rs about L3246/L11844);
- `test_support_samples.rs` about L622;
- `topo/tests/review_ssiflat_r1_probes.rs`.
