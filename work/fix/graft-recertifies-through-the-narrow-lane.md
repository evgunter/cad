---
id: graft-recertifies-through-the-narrow-lane
kind: issue
title: boolean graft re-certifies through the plain certify door, the second instance of the transform split
status: open
opened: 2026-09-12
---


Disclosed by `transform-recertifies-through-the-narrow-lane`, which
named it and left its reachability unestablished. That unit gave
`transform_rigid` a lane-injecting twin; this is the same shape at the
other door, and it is filed rather than taken because its class was
NOT established as reachable.

## The site

`boolean::combine::graft_solids_with` re-certifies every grafted
carrier against the destination body's points and surfaces through the
plain `geom_brep::EdgeCurve::certify`
(`crates/topo/src/boolean/combine.rs:440`, inside the curve pass), so a
grafted edge of the M7-8 class — an `Intersection` between a plane and
a DESCRIBED NURBS wall — would refuse `CertifyError::Unimplemented`,
surfacing as `BooleanError::GraftRecertify`. The split is stated at
neither door.

## Why it is not that unit's fix

**The bound cannot simply be raised.** `graft_solids_with` is
`T: geom_core::Decide` and sits under `boolean_op_with`, which
`verbs::Verb`'s `impl<T: Decide + Bounds + geom_brep::PcurveFittedLane>`
block runs and the dual corpus instantiates at `Dual64`
(`crates/editor-core/tests/r1_dual_probes.rs`). No `Dual` implements
`geom_core::CertifiedEnclosure`, so tightening this chain to
`CertifiedBounds` does not compile — the hazard `crates/geom-core/src/real.rs`
already records for that same `verbs` block. The remedy is the one
`transform_rigid_via` took: `EdgeCurve::certify_via` with the lane as
an argument, and a `_via` door on the graft for a caller that can name
it.

## What was NOT established

**Whether a body of that class ever reaches the graft.** The boolean
pipeline has a NURBS re-gate ahead of it, so the refusal may be
unreachable in practice and this may be a latent split rather than a
live defect. Establishing it needs a boolean over a described-NURBS
operand that survives to the graft, which was not cheap at the time of
filing. A unit taking this row should settle reachability FIRST: if
the class cannot reach the graft, what is owed is the sentence at the
door and not a new door.
