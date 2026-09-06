---
id: resolution-failure-arms-are-unmatchable-under-resolution
kind: issue
title: ResolveError's three arms are unmatchable under Resolution's carrier
status: open
opened: 2026-09-03
refs: [LIB-B-RESOLVE]
---

Banked at LIB-B-RESOLVE. **Not a request to relitigate the GUI-2
disposition** — it is honored throughout that unit, and the façade
states it so precisely that the binding was written straight off it.
What is recorded here is the third instance of a shape this track has
now met three times, and the first where the carrier is a VALUE rather
than a refusal.

## The finding

`Resolution` (`crates/editor-core/src/resolve/mod.rs:525`) has three
arms and Python can branch on all three, because `Resolution` itself is
curated. None of its payloads is:

```rust
pub enum Resolution {
    Resolved(Resolved),
    Failed(ResolutionFailure),
    Indeterminate(ResolveIndeterminate),
}
```

`Resolved`, `ResolutionFailure`, `ResolveError` and
`ResolveIndeterminate` are all in `NOT_CARRIED`
(`crates/pncad/tests/all.rs:3154-3157`, stanza at `:2974`). So a façade
consumer can name the verdict, match its three arms, and read fields
off the bound payloads — and cannot name the payload TYPES, which means
it cannot match `ResolveError`'s three arms
(`crates/editor-core/src/resolve/mod.rs:89`) or
`ResolveIndeterminate`'s three (`:465`).

## What crosses today, and what does not

`crates/pncad-py/src/py/resolve.rs` binds what the arms give up without
naming a payload type:

- `Resolved` projects fully. Its fields are reachable by field access,
  so `node`, `entity.body` and `entity.key.kind()` all cross (the arena
  key itself does not, correctly — G1).
- `Failed` projects `offers` (a `Vec<StableName>`, which crosses as
  name texts) and its `error` as `Display` PROSE ONLY. There is no
  `vanished` / `ambiguous` / `node_gone` tag, because writing one
  requires `match err { ResolveError::Vanished { .. } => … }` and that
  path cannot be spelled through `pncad`.
- `Indeterminate` projects `Display` prose only, for the same reason:
  `target_failed` / `target_poisoned` / `target_not_evaluated` are
  three different facts about the run and cross as one word plus a
  sentence.

## Why the census's own rule notices this

The rule the CUR3/CUR4 pair settled, stated at
`crates/pncad-py/tests/test_binding_census.py`'s `BlendError` entry:
**a payload's category follows what its CARRIER does at the crossing.**
`Resolution`'s carrier projects a discriminant —
`crates/pncad-py/src/tags.rs`'s `resolution_status_tag` — so by that
rule its payloads should project theirs too, and here not one of the
three does.

This is the third instance, and naming the three together is the
argument:

1. `DanglingRef` under `ReadbackError` (CUR3, closed) — a payload with
   arms, curated so the arms cross.
2. `MeshPickError` under `NodePickError`
   (`work/lib/mesh-pick-error-is-unmatchable-under-node-pick-error.md`,
   open) — one arm of five, unmatchable.
3. This one — **every** payload of the carrier, and the carrier is not
   a refusal at all but a value a caller reads on every re-evaluation
   of every stored name.

## What it costs today

- A selection panel can say "this name failed" and print a sentence. It
  cannot say "this one is a TIE, offer a refinement" and "that one
  VANISHED, offer a rebind" without reading prose, and prose is not a
  stable interface.
- `offers` is the only structured repair signal that crosses, and it is
  empty on both `node_gone` and `ambiguous` — the two arms where a
  caller most wants to know which it is.
- `crates/pncad-py/src/tests.rs::resolution_status_tags_are_stable`
  cannot CONSTRUCT a `Resolution` at all. It obtains all three by
  resolving real names against real runs, which is a better test than
  the literal pin it would otherwise have been — but it is a workaround
  for the same fact, and no per-arm pin is writable at any price.

## The counter-argument, stated fairly

`crates/pncad/tests/all.rs:2969-2973` already answers this once: GUI-2
carried the ladder's payloads briefly and PUT THEM BACK, because "the
panel renders the failure through its `Display`, so nothing consumed
the payload types, and a door carried for a consumer that does not
exist is a claim nobody is checking." That reasoning stands, and it is
why this is a banked finding and not a proposed change.

What has changed since is that there is now a SECOND consumer with a
different shape. A Rust panel that renders `Display` has the payload
one field away whenever it wants it; a Python caller holds a string and
has no other access at all. So "the panel reads `Display`" is a
complete story on one side of the boundary and a lossy one on the
other, and the disposition was taken when only one side existed.

## The shape of a fix, if one is wanted

Curate `ResolveError`, `ResolutionFailure` and `ResolveIndeterminate`
into `crates/pncad/src/select.rs` beside `Resolution`, the way
`DanglingRef` rides beside `ReadbackError` — WITHOUT `Diagnosis`,
`Tombstone`, `TieWitness`, `RecipeEditRef` or `Resolved`, which are the
key-bearing and telemetry half that stanza is really about. Then
`crates/pncad-py/src/tags.rs` gains a `resolve_error_tag` and a
`resolve_indeterminate_tag`, and `Resolution` gains a `variant`
attribute beside `status` on the arms that have one.

That is a strictly smaller carriage than the one GUI-2 tried and put
back. It is a curation judgement, not a binding one — it joins the
queue `DanglingRef` and `MeshPickError` are already in. A binding unit
cannot make it; this one did not try.
