---
id: an-annulus-rim-around-a-mixed-rim-refuses-in-one-call
kind: issue
title: An annulus rim around a mixed rim refuses in one call
status: open
opened: 2026-11-12
priority: P2
cost: M
refs: [a-mixed-rim-and-an-annulus-rim-on-one-support-refuse-in-one-call, a-mixed-ring-inside-a-few-wide-arcs-refuses-at-the-support-meter]
---

Filed by the band-mixgates lane (2026-11-12), measuring the shared-support gate.

`surgery::shared_support_gate` (`crates/sweep/src/blend/surgery.rs`) refuses `UnsupportedChain` "an annulus and a ladder or mixed rim share a support; blend them in SEQUENTIAL calls" when an annulus rim and a mixed rim of one request share a face. A mixed rim beside a ladder rim carves in one call.

Witness: a disc with the rounded outline through it.
- The disc: radius 1.5, authored as eight arcs, 0.5 thick. Its rims are hostless annuli on the caps' outer cycles.
- Through it, the rounded rectangle `[−1, 1] × [−0.6, 0.6]`, corner radius 0.3. Its rims are mixed rims, rings of the caps.
- Every edge filleted at 0.05 in one call refuses at the gate.
- The outline's rims first, then the disc's, builds at the closed form (`band_mixed_rim_shared_supports::an_annulus_rim_around_a_mixed_rim_refuses_and_composes_sequentially`).

Lifting the gate's arm for this pair alone is not enough; it was measured on the lane's scratch:
- The one call then refuses `RingClearance` at margin −0.0247. `support_boundary_clearance` reads the disc's 45° trim arc (radius 1.45) against the right straight link's strip (`0.95 ≤ x ≤ 1`, `|y| ≤ 0.3`). It takes the arc's range across the strip (`x ∈ [1.025, 1.45]`) and along it separately. That product straddles the strip, though the part of the arc inside the window lies at `x ≥ 1.42`.
- The same read refuses a two-arc disc's outline on its own (`a-mixed-ring-inside-a-few-wide-arcs-refuses-at-the-support-meter`).
- Unmeasured: whether the annulus walk and the mixed walk each take the cap the other carved. Neither walk's plan names what the other carve consumes there (a hostless annulus mints its struts in-phase, and the mixed walk reads its loop live). But no fixture reached that carve, because the meter refuses first.
