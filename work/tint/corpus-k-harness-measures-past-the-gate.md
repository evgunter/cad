---
id: corpus-k-harness-measures-past-the-gate
kind: issue
title: The corpus K harness reads each body's volume through mass_properties and no tier-3 gate, so a body with only a bracket at an eps row panics the sweep
status: open
opened: 2026-10-03
priority: P3
cost: M
---


Found by the REACH lane `reach/lily-leaf-1e12`, sweeping for the shape
of `work/reach/lily-leaf-b-mass-exhausts-the-quadrature-budget-at-eps-1e-12.md`: a K-sweep harness whose ladder measures a
body through `topo::mass_properties` and panics on any refusal, where
the kernel's own classification (`SignCertificate::measure`, the one
home of it) answers a schedule that cannot reach the reporting target
with a bracket.

That lane fixed the two ladders that were the same shape as the demo
pass's: `demos/tour/src/probe.rs` now gates through the tour's own
`gated` door, and `crates/sweep/tests/k_report.rs`'s `run_shape`
continues its tier-3 certificate. It left this one, because its fix
moves the linted population:

`crates/editor-core/tests/m4_pr8_k_probe.rs`'s `run_doc` runs tiers 1
and 2 and then `mass_properties(body, Tol::witness()).expect("mass
properties")` (≈ line 87) — no tier 3 at all. No corpus document
brackets at any ε today (the sweep's corpus pass is green at 1e-6,
1e-9, 1e-12 on `origin/main` 55ba6226), so nothing is red; the first
corpus document with a rational wall whose last-round Taylor bound
exceeds `1024·ε` at some row panics the nightly's `k-lint (dev-probe)`
sweep, as `lily_leaf_b` did.

The fix is the demo pass's: `validate_geometric_certificate` (or
3′ where the document is a boolean result) and `.measure()`, a
`TargetUnreached { bracket: Some(_) }` accepted. That adds the tier-3
gate's samples to the `corpus/<doc>` rows and drops the separate mass
pass's, so it is a baseline re-derivation under the K-REPORT runbook,
not a free edit — hence its own row.
