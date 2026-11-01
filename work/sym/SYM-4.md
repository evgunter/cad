---
id: SYM-4
kind: unit
title: the cost of a form: the polynomial's storage as a sorted vector and the ring's normalisation skipped on the dyadic shape, every decision bit-identical
status: closed
opened: 2026-09-14
branch: sym/4-form-cost
refs: [symbolic-tier-costs-95-percent-of-the-m10-3-drive, SYM-1]
closed: 2026-09-14
pr: 2565
---


## What

Ask 3 of `symbolic-tier-costs-95-percent-of-the-m10-3-drive` on the two
in-session levers SYM-1 ranked: `Poly`'s `BTreeMap` becomes a sorted
vector in the map's own order (so every digest, atom key and decision
is unchanged, which the pins and a new walk-ledger row — every form
the walks build, digested per walk and origin, with the largest form
pinned — hold); `Rat::from_parts`, `Rat::add` and `Rat::mul` skip the
gcd and the products by one on the dyadic shape. The degree cache and
the inline monomial were measured and not taken (each bounded at
about one percent). No count moves, no dial, no rule. The drive-scoped memo and the
assertion discharge are explicitly not taken: the first goes to Ev as a
decision document once this unit says what remains, the second is on
the item. Block SYM-B1 slot 0; the full v6 dual. Spec:
`docs/SYM-4-SPEC.md`.
