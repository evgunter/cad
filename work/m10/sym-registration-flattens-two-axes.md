---
id: sym-registration-flattens-two-axes
kind: issue
title: SymRegistration flattens witness and registry into six arms: the call, and what it costs
status: open
opened: 2026-09-06
---

**Raised in review of M10-9** (the registered-identity door) and
DECIDED by the implementer in the fix pass; filed so the decision and
its cost are on the record rather than only in a doc comment.

`geom_core::real::SymRegistration` has six arms — `Recorded`,
`Already`, `Contradicted`, `Cyclic`, `Witnessed`, `Unwitnessed` — over
two independent axes:

- the **WITNESS**: did the value channel find the two values one real
  (witnessed / contradicted / unable to say);
- the **REGISTRY**: recorded / already there / refused as cyclic / not
  consulted.

## The call: keep the six flat arms

Two reasons, and one cost.

1. **The impossible states are unspellable.** A `{witness, registry}`
   struct makes `{Contradicted, Recorded}` a value the type permits and
   the door must never produce. The flat enum's arms are exactly the
   reachable combinations.
2. **A registrant reads one answer.** `register_equal` returns to a
   constructor that must decide, now, whether it built what it claims.
   One `match` over one value is the shape that decision has; two field
   reads is not.

**The cost, stated.** "Was this refused?" is a two-arm match
(`Contradicted | Cyclic`) rather than a field read, and every caller
pays it by hand: both shipped registrants, both planted-lie pins, and
the door's own tests each spell the same alternation. A helper on the
type would remove the duplication without reopening the shape, and is
the obvious next edit if a third registrant lands.

Recorded in the type's own doc comment
(`crates/geom-core/src/real.rs`), so a reader meets the argument where
the type is.
