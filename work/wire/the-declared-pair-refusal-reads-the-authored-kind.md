---
id: the-declared-pair-refusal-reads-the-authored-kind
kind: unit
title: resolve_declarations answers DeclareUnsupportedPair from the authored StableName's kind, not the resolved key
status: closed
opened: 2026-09-13
branch: wire/tie-before-kind
pr: 2681
closed: 2026-09-15
---


## Finding

Found by the full review of PR 2517 (S1), triaging that PR's own sweep
to the end. The sweep's pattern was right —
`rg 'EntityKey::' crates/editor-core/src/` returns this site — and the
sweep stopped early.

`crates/editor-core/src/eval/wire.rs`'s `resolve_declarations` builds

```rust
let unsupported = || NodeErrorKind::DeclareUnsupportedPair {
    kinds: (n1.kind, n2.kind),
    cross_operand: o1 != o2,
};
```

thirty lines after it has resolved both names to `EntityKey`s (`k1`,
`k2`). **`n1.kind` is the kind the AUTHORED `StableName` declares; the
key is what the table actually resolved to.** The entity door's own
docs, in this file, say the answer to "what was found" comes off the
resolved thing and is never a caller's to write; this is the same
question answered off the other one.

## Why it is safe today, and why that is not the same as right

`NameTable::insert_ref` refuses a row whose `StableName.kind` disagrees
with its `EntityKey`, so the two agree for every name a table can
return. The refusal therefore says the truth today — by an INVARIANT
one module away, not by construction here. The door exists so that this
kind of reasoning is not needed per site.

## Why PR 2517 did not convert it

Stated so a taker does not re-derive it: `entity(key, read, refuse)`
tests ONE key and hands `refuse` ONE kind. This refusal is about a
PAIR — which pairs of kinds the declaration vocabulary has a step for
— and its `cross_operand` bit is a fact about the two operands rather
than about either kind. Threading a pair through the single-key door
would either mean calling it twice (two refusals where the recipe fault
is one) or widening the door's shape for one caller. **That is a real
reason, and the repair is probably not "use the door" but "read `k1`
and `k2` instead of `n1.kind` and `n2.kind`"** — a two-token change
that makes the refusal answer off the resolved entity, with no door
involved.

`crates/editor-core/tests/wire_entity_door.rs`'s census cannot see this
site: it keys on variants with a `found:` field and this one carries
`kinds:`. That blind spot is stated in the census's own doc, with this
row named.

## In review (PR 2681, `wire/tie-before-kind`)

Answered the other way from this row's guess, and the sibling row's
ordering is why. The premise is verified **by construction**, not by
an invariant one module away: `NameTable::forward` is private and its
only two writers, `insert_ref` and `insert_tied_ref`, both refuse a
row whose `name.kind` disagrees with a candidate's `key.kind()`. Once
the kind question is asked BEFORE resolution — which
`declare-door-refuses-a-tie-before-it-asks-the-pairs-kinds` requires,
because a tied name has no single key — the word must come off the
NAME. So `kinds: (n1.kind, n2.kind)` stays, and what changes is that
it is now load-bearing rather than incidental.

The resolved keys keep a reading of their own: each projection arm's
`let-else`, reachable only on a table that broke its own rule, calls
`broke(<shape>)` — a `debug_assert!` naming which projection failed,
and in release an answer off `k1.kind()` / `k2.kind()`. That is the
`resolve_face` split, where the guard answers off the name and the key
projection answers what the key IS.

**Note on this row's own citation.** Its title and its finding said
`route_declarations`; the site is and always was `resolve_declarations`
one function below. Corrected here, and the same mis-citation is
corrected in `crates/editor-core/tests/wire_entity_door.rs`'s census
and in `work/wire/the-entity-kind-door-has-six-spellings`.
