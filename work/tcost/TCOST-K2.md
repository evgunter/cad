---
id: TCOST-K2
kind: unit
title: offset_fit::fit_offset at 3.5 s per station: the Bernstein product weight, hoisted
status: closed
opened: 2026-09-03
branch: tcost/k2-unit
pr: 1697
closed: 2026-09-03
---

Candidate from TCOST-3 (`log.md`): `offset_fit::fit_offset` is 99.9 %
of the recentring row at 3.5–3.7 s per station against 0.004 s for its
oracle. Spec after TCOST-K1's Phase 1 says whether the same
exhausted-budget shape is at work in the fit loop.

Spec: `docs/TCOST-K2-SPEC.md` (ratified 2026-09-03; pre-draw fields
difficulty M, task-class STRUCTURAL). The drafting measurement answered
the item's question: K1's exhausted-budget shape is NOT at work — every
round refines the schedule the certificate is read at, and the 3.5 s
is `Composite::build`'s tensor-Bernstein products (99 % of `measure`,
linear in cells). The lever is a constant factor inside them: the
structural ring weight `point(C(a,i)·C(b,j))/point(C(a+b,k))` is
recomputed per coefficient per cell per round; hoisted per degree
pair it is bit-identical and measured −26 % on the `offset_fit` suite
(dev profile, one box), with a per-cell laziness behind the sign
witness as a conditional second lever. The stop clause makes the
unit a report if the hoist measures under 10 %. Lives in
`geom-core::compose`, so SSI's composite path may gain too — Phase 1
measures it. Dispatches after TCOST-K1 lands (block TCOST-KB1).
