---
id: declare-door-refuses-a-tie-before-it-asks-the-pairs-kinds
kind: issue
title: resolve_declarations refuses Ambiguous for a tied name before DeclareUnsupportedPair can ask the pair's kinds
status: open
opened: 2026-09-15
---


## Finding

Found by PORT-DOORS-1's **second** sweep (PR 2635,
`port/doors-1-refusal-order`), the one shaped for CALLER-SIDE kind/tie
pairings after the full review challenged the first instrument. It is
the third known instance of the class PORT-DOORS-1 closed in
`crates/editor-core/src/assembly.rs`'s `resolve_face`, and the second
that the `Entry::Tied`-adjacent grep could not see.

`crates/editor-core/src/eval/wire.rs`'s `resolve_declarations` resolves
both names of a declared pair before it asks what either one denotes:

```rust
let (o1, k1) = resolve_one(n1)?;      // <- refuses here
let (o2, k2) = resolve_one(n2)?;
let unsupported = || NodeErrorKind::DeclareUnsupportedPair {
    kinds: (n1.kind, n2.kind),
    cross_operand: o1 != o2,
};
```

`resolve_one` ends in `ladder::resolve(live, landing)`, whose
`Landing::Tied` arm returns `ResolveError::ambiguous(...)`, wrapped as
`NodeErrorKind::DeclareResolve`. So the tie refusal is raised *before*
`unsupported` can ever be built, and the consequence is the same
divergence, one door over:

- a UNIQUE pair of the wrong kinds (two vertices across operands, say)
  refuses `DeclareUnsupportedPair`, which names both kinds and whether
  the pair crossed operands — an author can read what they declared
  wrongly;
- the SAME pair with one name tied refuses `DeclareResolve
  { Ambiguous }`, which tells them to narrow a reference that would
  still not be a supported pair however narrow they made it.

`ladder::resolve`'s own comment says the tie row *"is the name
itself"* — which is true and is exactly why the kind is answerable
without resolving: every candidate of an `Entry::Tied` carries the
name's kind (`NameTable::insert_tied_ref` kind-checks each candidate,
and it is the only non-test constructor of a tie; `NameTable::project`
reaches it through `defer::narrow_into`). `n1.kind` and `n2.kind` are
already what the refusal reports.

## Its sibling on this slate

`work/wire/the-declared-pair-refusal-reads-the-authored-kind` is about
**the same two lines**, and the two should be taken together by one
lane. That row asks whether `kinds: (n1.kind, n2.kind)` should read the
AUTHORED name's kind or the resolved `EntityKey`; this row asks whether
the kind question should be asked at all before the tie refusal
preempts it. They interact: if the `found` word must come off the
resolved key, then a tied name has no single key and the kind-first
answer has to come off the name — which is the answer PORT-DOORS-1
took in `resolve_face`, and the argument for it is that the table makes
the two the same thing.

## Why PORT-DOORS-1 did not take it

`crates/editor-core/src/eval/wire.rs` is **WIRE's** territory, and this
changes a `NodeErrorKind` a Python caller branches on
(`crates/pncad-py/src/tags.rs` tags both `declare_resolve` and
`declare_unsupported_pair`). It wants the same treatment its two
siblings got: a decision recorded, then landed with the rows that pin
it.

## Sites

- `crates/editor-core/src/eval/wire.rs`, `resolve_declarations` and its
  `resolve_one` closure — the ordering.
- `crates/editor-core/src/eval/wire.rs`, `ladder::resolve`'s
  `Landing::Tied` arm — where the tie refusal is minted.
- `crates/editor-core/src/eval/wire.rs`, `face_name` — the same file's
  counter-example, which tests `name.kind` before it reads a candidate
  and is the shape to copy.
- `crates/editor-core/src/assembly.rs`, `resolve_face` — the same rule
  already landed.
- `work/wire/interrogate-read-answers-a-tie-before-the-door-s-kind` —
  the second instance, filed by the same unit.
