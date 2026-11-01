---
id: sym-header-dag-paragraph-disagrees-with-the-code
kind: issue
title: sym.rs's DAG paragraph: the at-zero fold is not 'where that value is rational', and the node list omits Opaque
status: open
opened: 2026-09-13
priority: P4
cost: E
---


**Found by SYM-2's header pass**, and left untouched by it: the unit
edits no sentence's technical content, so this is filed rather than
fixed.

Two sentences of `crates/geom-core/src/sym.rs`'s `# The DAG, and what
is opaque in it` do not say what the code does.

**1. The at-zero fold is not "where that value is rational".** The
header:

> an atom applied to a zero form folds to the value the function takes
> at zero where that value is rational (`sqrt 0 = 0`, `cos 0 = 1`,
> `acos 0 = π/2`).

π/2 is not rational, and the example contradicts the rule it
illustrates. The code says it correctly: `unary_at_zero`'s own doc is
"the value an opaque UNARY atom takes at argument zero, **where that
value is expressible in the form's own vocabulary**", and its `Acos`
arm builds `π/2` as half the indeterminate `INDET_PI` — "expressible,
because π is an indeterminate of the form rather than a number". The
form's coefficients are rational; its indeterminates are not numbers at
all, and that is the distinction the header sentence loses.

**2. The node list omits `Opaque`.** The header:

> Nodes are `Param(symbol)`, `Lit(f64 bits)`, `Pi`,
> `Add`/`Sub`/`Mul`/`Neg`/`Powi`, `Div` as `Mul(a, Inv(b))`, and OPAQUE
> atoms for every other [`Real`] operation (…)

`SymOp::Opaque` is in none of those buckets: it is minted by
`Sym::opaque`, which is not a `Real` operation but the door for a lane
with no expression to track, and each call mints its OWN indeterminate
(`indet_opaque`, keyed by `OPAQUE_SEQ`). It is the one node kind whose
id is not a function of an expression, which is the interesting thing
about it, and the tier's own node census does not mention it. The
enumeration also predates it: `SymOp::Opaque`'s doc carries the
soundness defect it was minted to close.

Both are the class the parent item
(`sym-rs-is-one-file-with-a-347-line-header`) was filed for — a
contract sentence a long way from the code it binds — and both survived
the split because SYM-2 moves prose without editing what it claims.
