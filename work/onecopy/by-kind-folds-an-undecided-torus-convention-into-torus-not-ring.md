---
id: by-kind-folds-an-undecided-torus-convention-into-torus-not-ring
kind: issue
title: injective.rs by_kind reads a torus convention half the band cannot decide as a definite TorusNotRing
status: open
opened: 2026-11-09
---


Found by the FULL review of PR 6641 (NOTE 2). It was recorded on the closed row `the-ring-torus-question-has-two-homes`, and the delta review asked for its own row.

**The fold.** `topo/src/injective.rs:536-541` `by_kind` reads the ring decision through `geom::RingTorus::of_at` and maps every `NotARing` to `EngineRefusal::TorusNotRing` with `.map_err(|_| ..)`. A torus half (`r` or `R − r`) decided zero or negative is a definite "not a ring". A half the band cannot decide is not, yet it reaches the caller as the same definite refusal. The old `_ =>` arm folded the ring half the same way, so this is not new in PR 6641. That PR added the tube half to the fold.

**Why it was not routed.** `EngineRefusal` (`topo/src/clearance.rs:481`) has no variant carrying an undecided reading:
- `Sliver` (`clearance.rs:485`) means the deciding enclosure is wholly in band, which is a specific claim about the enclosure, not about the torus convention;
- `Budget` and `Unsupported` mean other things.

`NotARing::escalation(band)` already gives the undecided half's `Indeterminate`.

**The choice.** Either:
- add an escalation variant to `EngineRefusal` (an API change, on a refusal the engine's callers match), or
- rule that an undecided convention is read as "not a ring" here and write that into `by_kind`'s doc.
