---
id: embedded-picks-its-witness-by-an-unasked-upper-end
kind: issue
title: Unsure: clearance::embedded picks its witness pair by a separation's upper end before asking it, so an uncertified pair can displace a certified crossing
status: open
opened: 2026-11-14
priority: P3
cost: E
---


Filed by GREEN (`green/censuses`, PR 6921), from the review of that PR. Unsure: a reviewer raised it, and its owner should read it before acting on it.

## What

`crates/topo/src/clearance.rs`, `embedded` (PR 6885), chooses the witness from the stations on the two faces as the pair whose separation, read on the candidate's carriers, has the least upper end: `if best.is_none_or(|(_, w)| d.hi() < w.hi())`. Only the chosen pair is then decided (`ask.confirms(distance, band)`). The comparison reads `d.hi()` of separations that have not been asked. An uncertified pair with a small upper end can therefore displace a certified pair. In that case `confirms` refuses the uncertified witness, and a body that would have reported `Crosses` reports a refusal instead.

This is loud, not wrong: the answer is a refusal, not a false `Embedded`. GREEN's census disposition reads the 3 reads as a selection followed by the decision, which holds for soundness and does not address this.

## Owed

Decide whether the choice should prefer certified separations, or try the next candidate when `confirms` refuses, or whether the refusal is the intended answer.
