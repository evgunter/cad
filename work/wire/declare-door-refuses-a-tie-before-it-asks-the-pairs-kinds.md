---
id: declare-door-refuses-a-tie-before-it-asks-the-pairs-kinds
kind: unit
title: resolve_declarations refuses Ambiguous for a tied name before DeclareUnsupportedPair can ask the pair's kinds
status: closed
opened: 2026-09-15
branch: wire/tie-before-kind
pr: 2681
closed: 2026-09-15
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

## In review (PR 2681, `wire/tie-before-kind`)

Taken with its two siblings, as this row asked. `resolve_declarations`
now walks rungs 1 and 3 per name through `declare_landing`, asks the
PAIR's kind question through `declared_step` — which is the v1
vocabulary's ONE enumeration, over the two names' kinds and the
operands they landed in, with the door's arms projecting the keys of
whatever variant it returns rather than re-listing the shapes — and
only then runs `ladder::resolve`, so the tie is the one refusal the
kind question outranks. `ladder::vanished` gives rung 3's payload one
home, which `route_declarations` now reaches too. The tie machinery is
untouched.

**`DeclareBothOperands` deliberately stays above the kind question**,
and the argument is written at the site rather than assumed:
`DeclareUnsupportedPair` carries `cross_operand`, a fact about which
operands the names landed in, so that refusal cannot be BUILT over a
name that landed in both — a field of it has no value. A tie leaves no
field empty, which is why the tie waits and this does not.

**Five outcomes moved, not one.** Deferring the first name's rung 2
past the second name's rungs 1 and 3 means a tied first name now waits
behind the second name's `NodeGone`, its `Vanished`, its
both-operands refusal, and (in a union) `step_diagnosis`'s re-saying
of that `Vanished` as `UnionDeclareStep`. All four change a Python
tag. The first two are pinned by
`m4_pr5_declare::a_tied_first_name_waits_behind_the_second_names_own_faults`;
the third and fourth are disclosed in PR 2681's body with their
reachability. The justification is NOT the ladder's rung ranking — the
ladder ranks within one name's walk — but this door's own rule: the
tie is the one per-name refusal the PAIR question outranks, and a pair
question cannot be asked before both names have landed.

The **fifth** is not about the tie and was found by the delta review:
the old same-operand vertex-face arm matched on KEYS, so a table whose
key kinds disagreed with its authored kinds was silently accepted and
the `debug_assert!` never fired. The new arm picks the vertex from the
step's own `VertexAndFace` witness, derived from the authored kinds,
so that table now reaches `broke` and says so. Strictly more
fail-loud — recorded because the paragraph above claims completeness.

**The third outcome's reachability, measured rather than guessed.** PR
2681 first reported it unconstructible because "only a pass-through op
keeps names verbatim, and it keeps all of them". That was **wrong**:
`NameTable::project`, reached from `PartSelect::SplitHalf`, keeps a
STRADDLING tie's row verbatim in both halves — its own doc says so and
a probe confirms it. On the `u_cutter_tie` fixture, a split at `y=2`
leaves `tied-in-A-only=0, in-BOTH=2`; a split at `x=3.0` leaves
`tied-in-A-only=4, tied-in-B-only=4, in-BOTH=0`. What actually blocks
the construction is that this fixture has exactly TWO ties and both
are symmetric about the same plane, so a plane either splits both
(leaving nothing tied) or neither (leaving nothing shared). Row 3
wants a document with **two ties of different symmetry**, which the
tree has no fixture for; the two halves then feed one `Boolean` and
the pair is (a tie whole in one half, a straddling row present in
both). That is the row a taker should build.
