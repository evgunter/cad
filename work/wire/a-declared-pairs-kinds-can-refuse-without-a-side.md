---
id: a-declared-pairs-kinds-can-refuse-without-a-side
kind: issue
title: DeclareUnsupportedPair cannot say 'no step under either operand assignment', so the declare door's both-operands refusal outranks an answerable kind question
status: open
opened: 2026-09-15
refs: [route-declarations-ranks-the-step-question-above-the-pairs-kinds]
priority: P1
cost: D
---

## Finding

Disclosed by PR 2681 and homed here so a taker who goes to the DECLARE
door finds it at that door. It first appeared inside
`work/wire/route-declarations-ranks-the-step-question-above-the-pairs-kinds`'s
"why PR 2681 did not take it", which is the wrong address for it.

`crates/editor-core/src/eval/wire.rs`'s `declare_landing` refuses
`NodeErrorKind::DeclareBothOperands` above the pair's kind question,
and PR 2681 argued at the site why: `DeclareUnsupportedPair` carries
`cross_operand: bool`, so the kind refusal **cannot be built** over a
name that landed in two operands — that field has no value. The
delta review verified the argument against the tree (a plain `bool`,
one construction site, no third state expressible, and in the
interesting case the answer itself is undetermined rather than merely
the payload) and ratified it.

**The residue the argument leaves.** A subset of unsupported pairs is
decidable with NO side at all: a pair whose two kinds have no step
under EITHER operand assignment — a body name, an edge name, a
same-kind pair the vocabulary never threads. `declared_step` refuses
those for every `(oa, ob)` it is given. For such a pair with one name
in both operands, the author is told to disambiguate a reference that
would still have no step however it disambiguated — which is the exact
shape of fault PR 2681 closed one rung down, surviving one rung up
because the payload cannot say it.

## What a taker decides

Whether `cross_operand: bool` should carry a third state (the names
did not both land on one side, so the question does not arise), or
whether a kind-only refusal wants its own variant. Either is a public
error-channel change: `crates/pncad-py/src/tags.rs` tags
`declare_unsupported_pair` and `declare_both_operands` separately and
`crates/pncad-py/pncad.pyi` documents both, so a Python caller
branches on the difference.

The same field blocks the cheap repair at the other door —
`route-declarations-ranks-the-step-question-above-the-pairs-kinds`
proposes a kind-only pre-question in the union routing door and cannot
fill `cross_operand` either. **The two rows want deciding together**:
one field, two doors, and the kind-only subset is exactly what both
would answer with.

## Sites

- `crates/editor-core/src/eval/wire.rs`, `declare_landing`'s
  both-operands arm — where the argument is written.
- `crates/editor-core/src/eval/wire.rs`, `declared_step` — what a
  kind-only pre-question would call, with any `(oa, ob)`.
- `crates/editor-core/src/eval/mod.rs`,
  `NodeErrorKind::DeclareUnsupportedPair` — `cross_operand`, the field
  with no third state.
