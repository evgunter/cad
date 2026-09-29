---
id: DECIDE-4
kind: unit
title: rule D past the unit bulge: what stands at a bulge that is not 1 on today's tree, the sign-free part, and the bulge's sign
status: closed
opened: 2026-09-24
priority: P1
cost: D
branch: decide/4-bulge-reach
refs: [rule-d-reaches-the-unit-bulge-only]
closed: 2026-09-25
---

## What

The item's owed measurement re-taken on `props/sign-hull` (DECIDE-3's
canonical root and SYM-9's ladder have both landed since SYM-3 and
SYM-5 measured it). The boss's residue was NOT gone: one value-free
`arc_span` residual stood on it and bounded its ceiling (measured at
Phase 1), and rule G's exact quotient takes it. What stands on
the D-tab and the dyadic controls is attributed per decision to a
freeze, a sign-free `abs`/`sqrt` pairing, or the bulge's sign. The
sign-free part is then taken by the narrowest value-free rewrite. The
sign, if it still blocks, goes to Ev as the item's route A / route B
fork. Spec: `docs/DECIDE-4-SPEC.md`. Opus implementer. Review tier
DUAL (M / NUMERIC), or a single STYLE review if the unit closes at its
measurement — `work/decide/log.md`, 2026-09-24.

## Closed (2026-09-25)

Merged as #3192 into `props/sign-hull` (landing head `de02c07a1`, run
36079428012).

**Phase 1.** It attributed every still-numeric arc-family decision on
the five bulge documents and on the bracket's fillet arcs:
- (ii) is empty;
- the D-tabs at `0.4` stand on the ring;
- the `0.5` parameter control carries four sign-of-`b` decisions and six
  apothem-sign ones;
- one value-free decision bounded the boss's ceiling.

**Phase 2a.** It is rule G's exact quotient (`SymRules::root_quotient`,
`Poly::div_exact`). It takes that decision, and the boss certifies
0.5024 / 0.7267 / 0.7271 of its real study at ε = 1e-6 / 1e-9 / 1e-12
(it was `1.0309e3·ε`). No other split moves on nine documents, and it
costs +0–3% on the leaf instrument, best of 3.

**Phase 2b** stopped for Ev's fork on `[ev]` #3186, and the item stays
open on it. The trade the quotient makes (it no longer meets the split
spelling) is filed as `the-exact-quotient-re-keys-a-root-the-split-met`.

**Review: the dual on `334bb2aa2`** (`docs/DUAL-REVIEW-PROTOCOL.md` at
`c3129311b`).
- R1: APPROVE-WITH-FIXES, 0/7/4.
- R2: APPROVE-WITH-FIXES, 0/5/5.
- Coded blind, it has no tally candidate and is a fair pair.

**The fix passes.**
- The union fix pass A–L ran at `bfb059b7f`: the monomial helpers given
  one home in `form.rs`, the product check dropped for the loop
  invariant, the trade disclosed and pinned, the renders re-taken uncut,
  and best-of-3 costs.
- R1's delta was APPROVE-WITH-FIXES. It measured the budget-sized step
  cap making a declined division cost 0.78 s.
- The second pass is at `de02c07a1`. It adds the leading- and
  trailing-term conditions before any step and a division linear in
  steps × |d|. The declined shape now takes 0.3 ms.

The DR row is recorded on `main`.

