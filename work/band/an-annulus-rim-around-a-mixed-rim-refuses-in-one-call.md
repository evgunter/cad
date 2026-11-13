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

Lifting the gate's arm was measured on the lane's scratch, by review lane band-gates-r1 on PR 6865, and again by the band-meter lane and both reviewers of PR 6887 once the support meter read circles clipped to each window:
- **Arm lifted alone, with the meter's exact read** (PR 6887): every edge of the disc filleted in one call builds, at tier 3 and at the closed form 0.0164274846917652, for 8-, 4- and 2-arc discs, at `f64` and at `Interval` (ε 1e-9). The whole trim circle of radius 1.45, clipped to each link's window, lies far from every strip. ΔV at `f64`: 0.0164274846917665 (8 arcs), 0.0164274846917647 (4), 0.0164274846917651 (2); the `Interval` enclosures hold the closed form, width ≤ 6.1e-12.
- **Before the exact read**, the arm lifted alone refused `RingClearance` at −0.45 for 2-, 4-, 8- and 16-arc discs alike: the meter read the whole trim circle's ranges across and along each strip separately. Windowing the annulus trims (each arc read between its own feet) brought an 8-arc disc to −0.0247 and let a 16-arc disc build at tier 3, ΔV 0.0164274846917665.

So the meter no longer stops the pair: the exact read alone suffices, and the annulus trims need no windowing. What lifting the arm still owes is the gate's own reason, that each carve's walk survives the other's on the shared face (`surgery::shared_support_gate`'s docs); the disc is the witness to pin when it lands.
