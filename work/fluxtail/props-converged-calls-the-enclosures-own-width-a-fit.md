---
id: props-converged-calls-the-enclosures-own-width-a-fit
kind: issue
title: geom-brep props: PropsCheck::Converged meters the quadrature enclosure's own width but routes as Unsized::Fit, not Bound
status: open
opened: 2026-10-13
priority: P4
cost: E
---


(PCERT implementer, from the `recourse` sweep for the envelope-as-bound
unit, `pcurve-envelope-bound-ends-as-a-stored-contradiction`.)

## What

`crates/geom-brep/src/props/mod.rs` `PropsCheck::ending` routes
`Converged` through `Unsized::Fit`, and its doc calls it "a kernel
approximation limit". But the check meters "the enclosure's own width,
nothing of the model's" against the target. That is a certified bound
too loose to read, which is `Unsized::Bound`'s statement ("a loose bound
is the certificate's own limit, and contradicts nothing stored").

## Why only P4

No text moves today. `Converged` is read only on its undecided arm
(`PropsError::Escalated`, at `Reading::AtRest`), and there `Fit` and
`Bound` both end in the kernel-limit recourse. Its definite arm is
`PropsError::QuadratureBudget`, which carries its own recourse. The
name contradicts the doc, and a future definite arm read at rest would
end in the file's defect ending under `Fit`.

## Direction

Route `Converged` through `Unsized::Bound` and re-word its doc to match.
