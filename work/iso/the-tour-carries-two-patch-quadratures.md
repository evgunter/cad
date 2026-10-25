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
`demos/tour/src/patch_quadrature.rs` (`measure`), which lily and
projectbox now use. These are two spellings of one user-side reference
integral. Fix (E): move teapot onto `patch_quadrature::measure`, keeping
its two-order error estimate, and delete the private fn.
