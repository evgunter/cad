---
id: the-declared-pair-refusal-reads-the-authored-kind
kind: unit
title: route_declarations answers DeclareUnsupportedPair from the authored StableName's kind, not the resolved key
status: dispatched
opened: 2026-09-13
branch: wire/tie-before-kind
---


## Finding

Found by the full review of PR 2517 (S1), triaging that PR's own sweep
to the end. The sweep's pattern was right —
`rg 'EntityKey::' crates/editor-core/src/` returns this site — and the
sweep stopped early.

`crates/editor-core/src/eval/wire.rs`'s `route_declarations` builds

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
