---
id: step-import-discards-the-entity-ids-that-are-its-identity-channel
kind: issue
title: step-import decides which surface record a face wears by comparing f64 bits; the file's surface references should decide it, with distinct entities joined only by one_record's reported act and the exporter writing one entity per surface key
status: closed
opened: 2026-09-12
refs: [2404]
priority: P1
cost: H
branch: export/import-records
pr: 7361
closed: 2026-11-22
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

## This row is STEP 2 of a ratified sequence (WIRE orchestrator, 2026-09-12)

Added after cutting `work/wire/axis-shaped-identity-channel.md`, whose
cut section carries the sequence, its owners and its ordering.

**Step 1 is `work/topo/geom-source-absence-conflates-four-origins.md`**,
which decides how origin is represented positively — the vocabulary this
row's mark would be written in. In principle the two are parallel; in
practice this one lands after, because a mark written before the
vocabulary exists is a mark to be rewritten.

The reverse dependency is worth knowing too, and is recorded on that
row: whoever designs the origin representation should look at what
`import_step` can actually supply, since this is the step that has to
write one of the four origins with real content rather than a bare tag.

Nothing here claims EXCH's ground or reorders EXCH's slate — it is a
cross-reference so the sequence survives WIRE's closure.

## The design fork (EXPORT, 2026-11-17 to 2026-11-22)

Weighed by three designers (`docs/DESIGN-FORK-PROTOCOL.md`, row 297 of `docs/DESIGN-FORK-LOG.md`).

**The premise is stale.** The stamp channel this row would extend was retired: `GeomSource`, `SourceExpr`, `GeomOrigin`, `AxisSource` and `stamp_minted` went in INTENT stage 4 unit E, under PR 4322, and `docs/AXIS-DECLARATION-DESIGN.md` is deleted. `[import]` makes an imported body indistinguishable from a native one by intent. No identity token is built.

**What the row is still evidence of.** The importer decides which surface record a face wears by comparing f64 bits (`adopt::surface_sig`, read at about six sites: `attach_surfaces`, `build::choose`, `snap`'s grouping, and the vertex-rooting reads in `build.rs`), beside a per-face slice of the file's own entity. That is a production bit-identity coincidence decision outside `scripts/gates/bit-identity-consumer.sh`. It joins silently what `one_record` would join reported, and it can disagree with `one_record`, which never joins opposite-facing faces.

**The agreed answer** (all three designers, after one round):
- surface records follow the file's references, with records a normalization mints (a knot split's sub-patches) as records of their own;
- two distinct entities join only through `one_record`'s reported act;
- `surface_sig` and its readers go;
- `step-export` writes one surface entity per surface key;
- `[import]` gains the paragraph "Records follow the file";
- one import row per sharing pattern, each held to its native twin's at-rest verdict.

**Measured, read-only.** Across the tree's 114 STEP files, no spline surface is stated twice. Every duplicated entity group is a plane or round surface facing one way (96 groups in 15 files), the case `one_record` joins.

**Ev's concern** ("imported files can reach states that native files cannot") was weighed in two more rounds and probed. One record worn both ways in one solid is produced natively by `topo::union` (run). Every sharing pattern a file can state has a native producer, and import is narrower (it never shares across shells or solids). The unit therefore adds one import row per pattern, each held to its native twin's at-rest verdict. The one import state native bodies also reach and the boolean refuses, non-maximal planar faces, is its own row: `work/export/an-imported-union-ships-coplanar-same-record-neighbours-no-boolean-takes.md`.

**Ruled (Ev, PR 7331, 2026-11-22):** approved, together with the `[import]` paragraph "Records follow the file". This row is now that unit. `work/origin/axis-source-lowered-bytes-carry-no-minter-namespace.md` closed as moot in the same PR.

## Closed (2026-11-22)

PR 7361 (`export/import-records`): surface records follow the file per shell; `surface_sig` and its readers are deleted; the writer emits one surface entity per key; per-pattern pins hold imports to their native twins. Dual review, fix pass, merged.
