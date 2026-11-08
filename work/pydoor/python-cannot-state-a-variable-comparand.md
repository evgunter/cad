---
id: python-cannot-state-a-variable-comparand
kind: issue
title: Python has no way to state a variable comparand in a selector predicate
status: open
opened: 2026-10-06
priority: P3
cost: M
---


`GeomPred.datum_distance` (`crates/pncad-py/src/py/select.rs:704-717`)
takes a `Formula` and lowers it with no document in scope, so a formula
that writes a name refuses at construction (`EvalError`,
`unlowered_name`), and `Formula` has no by-id constructor in Python. A
selection rule whose comparand is a document variable — what
`docs/SELECT-DESIGN.md` §5 calls the whole point of the Rust field being
an expression — has no Python spelling at all. The Rust door takes a
stored `Expr`, so a Rust caller can write one.

This sits beside G1's residue in `docs/guide/north-star-audit.md` (the
expression half of "arcs and circles in profiles"). It is not new in
INTENT-LITERALS PR B: before it the name was never lowered and
evaluation refused it; PR B refuses it earlier. A door would either
take the document (lower against its scope at construction) or carry
the formula to `select_where` and lower it there against the evaluated
document.

Raised by the PR B review (#4072, NOTE-1).

- 2026-11-03 — moved from LIB to PYDOOR by `git mv` when LIB closed, id and body unchanged; banded P3 and priced M in the move (LIB had three P1 rows left behind 124 closed ones; its library-door rows went to PYDOOR, opened for them, and its tag, stub, census and projection rows to BIND; Ev, in chat).
