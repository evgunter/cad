---
id: the-blend-joints-prove-only-past-the-door-s-budget
kind: issue
title: A filleted disc's two blend joints prove as theorems at 1024 terms and not at the coincidence door's 256, a cost question beside the mitre's identity
status: open
opened: 2026-10-11
priority: P2
cost: M
---

Found by INTENT stage 4 C+D (`carriers-compare-in-canonical-form`). It
is the mitre row's sibling
(`the-mitres-feet-distance-freezes-at-sym`), but a different case: the
mitre's form never closes, while these do close, at a cost.

A filleted disc (r = 1, h = 1, rim fillet 0.1) has two `BatteryJoint`
rows (`sweep::blend::battery`, `fillet3_chain_g1`), each a DAG of 1325
nodes and 13 parameter symbols. Measured in release, replaying the
document at `Sym` with every variable a symbol in a deferring session:

| budget | retry ladder | joints proved | the disc's four rows |
|---|---|---|---|
| 256 terms | `kept_atom` (the door's one strength) | 0/2 | 0.64 s |
| 1024 terms | `kept_atom` | 2/2, `SymbolicZero` | 1.6 s |
| 4096 terms | `kept_atom` | 2/2 | 5.5 s |
| 4096 terms | none | 0/2 | 14 s |

The door's budget is set by the mitre, which costs 172 s per box
at 1024 terms with the ladder against 0.9 s at 256. A tier that failed
fast on a form that will not close would let the door raise its budget
and prove these. So would the parked per-node cache
(`a-construction-s-identities-are-a-generated-cache`).

Done when the door proves both joints within the door's cost
class, or this row records why it cannot.
