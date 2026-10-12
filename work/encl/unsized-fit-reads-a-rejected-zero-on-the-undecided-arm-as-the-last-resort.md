---
id: unsized-fit-reads-a-rejected-zero-on-the-undecided-arm-as-the-last-resort
kind: issue
title: geom-brep: Unsized::Fit at rest reads a gate's rejected Zero on the undecided arm as the last resort, where its Zero arm reads a defect
status: open
opened: 2026-10-12
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
