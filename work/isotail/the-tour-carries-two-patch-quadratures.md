---
id: the-tour-carries-two-patch-quadratures
kind: issue
title: demos/tour carries two patch quadratures: teapot.rs's private patch_quadrature fn and the patch_quadrature.rs module
status: open
opened: 2026-10-24
priority: P4
cost: E
---


(Disclosed by ISO's `iso/sweep-closed-form-oracles` lane, PR 5646. Filed by the ISO orchestrator.)

PR 5642 (the teapot) added a private `patch_quadrature` fn in
`demos/tour/src/teapot.rs`. QUAD's PR 5640 added the module
`demos/tour/tests/common/patch_quadrature.rs` (`measure`), which lily and
projectbox now use. These are two spellings of one user-side reference
integral. Fix (E): move teapot onto `patch_quadrature::measure`, keeping
its two-order error estimate, and delete the private fn.

## Moved, 2026-10-29 (GREEN, green/latency-tour-lib)

The module moved from `src/` to `tests/common/` when the tour's unit
tests left `src/` (`demo-tour-unit-tests-compile-the-bin-twice`): its
only callers were tests. teapot's private fn is called by the scene
itself (`src/teapot.rs`, the spout's two-order estimate), so the fix
now runs the other way: move `patch_quadrature.rs` back into the lib
(a `pub mod` the tests reach as `demo_tour::patch_quadrature`) and
point teapot at `measure`.
