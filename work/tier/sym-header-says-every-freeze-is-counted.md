---
id: sym-header-says-every-freeze-is-counted
kind: issue
title: the # Freezing section says every freeze is counted; only the plain walk's freezes are
status: open
opened: 2026-09-13
priority: P4
cost: E
---


**Found by SYM-2's header pass**, and left untouched by it: the unit
edits no sentence's technical content, so this is filed rather than
fixed.

`crates/geom-core/src/sym.rs`'s `# Freezing: the budget, and why it is
sound` closes with

> Every freeze is counted ([`SymCounts::frozen`]).

Only the PLAIN walk's freezes are. `form_in`'s `frozen` closure
(`crates/geom-core/src/sym.rs`, ~`:2179`) reads

```rust
let frozen = |sess: &mut Session, id: SymId| -> Rc<Form> {
    if !early {
        sess.counts.frozen += 1;
    }
    Rc::new(Form::poly(Poly::indet(id.bits())))
};
```

so a node the EARLY walk freezes (`early_form`) and a node the DOOR
walk freezes (`door_form`, which also passes `early = true`) is not
counted anywhere. `SymCounts::frozen`'s own doc — "Nodes frozen into
indeterminates (a budget or an overflow)" — carries no qualifier
either.

The counting is very likely deliberate: the three walks visit the same
DAG, so counting each would report one node's freeze up to three times
and `frozen` would stop being a per-node honesty column. What is owed
is one of the two — the sentence saying which walk it counts, or the
column counting distinct nodes across the walks. The second matters
because the early walk is where the shipped rules run: a form the
ALGEBRA freezes (the size cap, a coefficient past the bound in a
substituted product) is exactly the freeze M10-10's cost argument is
about, and the receipt is silent on it.

Not SYM-2's to take: the unit is a pure move and a header that
describes the code it moved, and this is a claim about what the code
should do.
