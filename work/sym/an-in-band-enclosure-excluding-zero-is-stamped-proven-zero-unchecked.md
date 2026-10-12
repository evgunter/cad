---
id: an-in-band-enclosure-excluding-zero-is-stamped-proven-zero-unchecked
kind: issue
title: sym: an Exact in-band enclosure that excludes 0 can be stamped proven zero with no contradiction check
status: open
opened: 2026-10-12
priority: P3
cost: E
---


(Filed by the ENCL orchestrator from the review of PR 4840, 2026-10-12. The reviewer's confidence was unsure; it is filed so the owner can confirm or close it.)

`Sym::decide_sign` (`crates/geom-core/src/sym.rs`, about L5395–5450) checks a symbolic proof against the numeric channel only when the numeric sign is definitely nonzero. A sound Exact (Interval) enclosure of a quantity proved identically zero must contain 0. An in-band enclosure that excludes 0, such as `[2e-9, 5e-9]` at band (1e-9, 1e-8), would still be stamped `proven(theorem)`, and no assertion would fire, although the two channels contradict each other.

Since PR 4840 (merge `8e772980bb`), such a state has its own sentence: `IndeterminatePayload` renders a proven in-band enclosure as "reaches into the ambiguity band", and `Indeterminate::reading` says "a zero the symbolic tier proved". The pin `predicate::tests::a_proven_zero_is_offered_no_tolerance` uses exactly that unsound fixture.

Owed: decide whether an Exact enclosure that excludes 0 under a zero proof is a kernel defect, to be refused or asserted the way the definite-nonzero disagreement is. If it is, the pin's fixture should become a straddling enclosure.
