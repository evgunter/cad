---
id: step-import-discards-the-entity-ids-that-are-its-identity-channel
kind: issue
title: import_step keys its maps by the file's entity ids and discards them at the door, so an imported body has no provenance and cannot be told from a hand-built one
status: open
opened: 2026-09-12
refs: [2404]
---


## Finding

**Ev's idea**, in-chat and on PR 2404, 2026-09-12; filed here by the
WIRE orchestrator because `crates/step-import/*` is EXCH's. It arose
answering *"for imported geometry, we don't mark it persistently as
imported right?"* — correct, and it is stronger than that: an imported
body has no provenance at all.

`import_step` (`crates/step-import/src/lib.rs:638`) is a **free function
at the kernel seat**. There is no `Node::Import`, so `stamp_minted`
never runs on the result, and placed instances go through
`transform_rigid`, which clears `GeomSource` and relies on a recipe
layer that is not there. The shipped body is a `Body` whose descriptions
have no source — indistinguishable from a hand-built one.

**And the identity channel is already in hand at assembly.**
`import_step`'s own comment says so:

> a `SolidSpec`'s maps are keyed by the file's entity ids, so two copies
> assembled into one arena would collide id for id.

The ids are used to keep copies apart and then dropped.

## Why a STEP entity id is real identity

It is not a fabrication. Two faces referencing one surface entity
genuinely share that surface — the same claim `GeomSource` makes about a
recipe expression, sourced from the file's own structure rather than
invented by the reader. The retirement theorem (N6: same source ⇒
bit-identical descriptions) holds for it: same entity ⇒ same parsed
parameters ⇒ same bits.

## The instancing case already has its shape

M8 instancing builds N occurrences of one component as N independent
bodies, each placed by its own frame, so two copies must **not** compare
equal or the retirement theorem is false. `SourceExpr` already carries
exactly this:

```
Placed { node, instance, inner: Minted { index } }
```

whose `instance` is documented as *"the pattern instance index (0 for a
plain Transform)"*. An imported instance is a pattern instance in all
but name, so the existing variant serves it unchanged — same entity in
`inner`, different `instance`, different token, correct answer.

## The one thing that must be designed, not assumed

**Namespace disjointness.** `GeomSource`'s `node` is an opaque `u64` the
kernel only compares (`crates/topo/src/source.rs`'s module doc: *"this
crate only ever compares them for identity and flips orientation"*), so
an import needs no recipe anchor — but `RecipeNodeId(pub u64)` is a
**full `u64` with no free high half**, and an import id colliding with a
recipe node id would make two unrelated surfaces read as same-source and
be glued by the boolean's coincidence rung. That is silent wrong
geometry, not a refusal, and the partition has to be designed.

Also: the re-stamp must happen **after** `transform_rigid`, or the
placement clears what the reader just attached — the same discipline the
recipe layer follows.

And `crates/topo/src/source.rs`'s scope sentence holds identity *per
evaluation against the current document*; an imported body is not
evaluated from a document, so what the claim means for one has to be
written rather than inherited.

## What it buys, beyond this crate

`docs/AXIS-DECLARATION-DESIGN.md` is ratified with **absence refuses**,
which today locks imported geometry out of axis-shaped declarations
entirely. With a source derived from the file's entity ids, imported
carriers gain structural coaxiality on the same terms as recipe-built
ones — two cylinders whose axis placements resolve to one STEP entity
are coaxial by the file's own structure, decided by token equality. It
also shrinks
`work/topo/geom-source-absence-conflates-four-origins.md` from four
conflated origins to three.

Not a prerequisite for either: both stand without it and get better with
it.
