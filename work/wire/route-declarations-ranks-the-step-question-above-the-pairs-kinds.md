---
id: route-declarations-ranks-the-step-question-above-the-pairs-kinds
kind: issue
title: route_declarations refuses UnionDeclareStep before the pair's kinds can be asked, the where-before-what shape of the doors rule
status: open
opened: 2026-09-15
refs: [a-declared-pairs-kinds-can-refuse-without-a-side]
priority: P1
cost: D
---


## Finding

Found by the style review of PR 2681 (`wire/tie-before-kind`), which
asked whether the class that PR closed has a neighbour in the same
file. It does, one shape over.

`crates/editor-core/src/eval/wire.rs`'s `route_declarations` answers,
per name, rung 1 and then `decl_site` — which fold step the name
belongs to — and refuses `UnionDeclareStep` for a pair whose two sites
share no step. All of that happens BEFORE `resolve_declarations` sees
the pair, so a union pair of kinds the v1 vocabulary has no step for
*and* with no fold step joining its two names is told which of those
two it is by routing order, not by which fault is the author's.

This is not the tie class PR 2681 closed — `decl_site` is not a
multiplicity question. It is `assembly.rs`'s `operand_answer` rung 2
stated for the other axis: **WHAT a name is precedes WHERE it is
rooted** ("a non-face never mints anywhere, so WHAT it is precedes
WHERE it is rooted"). A pair the vocabulary has no step for is
unsupported at every fold step, so the kind question is the one with
no dependence on the answer that currently precedes it.

## Why PR 2681 did not take it

It is not fixable where it stands. `DeclaredStep` — the one
enumeration of the vocabulary — asks its question of the two names'
kinds AND the operands they landed in, and in a union the two operands
of a step (the accumulation and the joining member) only exist once
routing has picked that step. So kind-before-routing needs either

- a kind-only pre-question, answerable without operands: a pair whose
  kinds have no step under EITHER operand assignment (a body name, an
  edge name, a same-kind pair the vocabulary never threads) is
  unsupported however it routes, and that subset is decidable in the
  routing door; or
- the vocabulary question moved into the routing door entirely, which
  means `route_declarations` learns the operand shape of a step.

The first is small and buys most of the actionability; the second is a
design change to where the vocabulary lives. A taker should decide
which, and should look at `DeclareUnsupportedPair`'s `cross_operand`
field first, because the first option cannot fill it.

**That field is the same obstruction at the declare door**, and it has
its own row there now:
`work/wire/a-declared-pairs-kinds-can-refuse-without-a-side`. One
field, two doors, and the kind-only subset is what both would answer
with — so the two rows want deciding together rather than one of them
re-deriving the other's argument.

## Sites

- `crates/editor-core/src/eval/wire.rs`, `route_declarations` and its
  `site` closure — the ordering.
- `crates/editor-core/src/eval/wire.rs`, `declared_step` — the
  vocabulary, and what it needs that routing does not have.
- `crates/editor-core/src/eval/mod.rs`,
  `NodeErrorKind::DeclareUnsupportedPair` — `cross_operand` is the
  field a kind-only pre-question cannot fill.
- `crates/editor-core/src/assembly.rs`, `operand_answer` rung 2 — the
  precedent for what-before-where.
