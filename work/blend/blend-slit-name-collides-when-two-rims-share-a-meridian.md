---
id: blend-slit-name-collides-when-two-rims-share-a-meridian
kind: issue
title: the blend name emitter refuses a roll of two rims whose bands slit ONE seam meridian (RoleSeg::BandSlit has no discriminator)
status: open
opened: 2026-09-08
---

`fillet_edges` rolls the teapot lid's three latitude rims in ONE
request and returns the body the tour ships. The same three rims
through `Node::Fillet` refuse before any geometry is doubted:

```
Naming(Duplicate { name: StableName { kind: Edge, node: <fillet>,
  path: [BandSlit(StableName { kind: Edge, node: <revolve>,
    path: [Meridian(Seam, ProfileEdgeRef { loop_index: 0, segment: 1 })] })] } })
```

`crates/editor-core/src/names/role.rs:610` defines
`BandSlit(Box<StableName>)` as *"the source edge whose severed piece
became it"*, and `crates/editor-core/src/names/emit_blend.rs:212`
inserts one row per `(edge, meridian)` pair in `rec.slits`. A band
slits exactly ONE of its two supports' seam meridians, so two bands
collide exactly when they slit the SAME one — and on this lid the
flange's rim (profile vertex 1) and the dome's foot (vertex 2) are the
two ends of the flange cone, whose seam both of them take.

## The invariant is the shared MERIDIAN, not adjacency

An earlier statement of this issue said "any two ADJACENT latitude
rims on any solid of revolution collide the same way". **That is false
on this very lid**, and both reviewers of PR 2206 executed the
falsification. Which meridian each band slits, measured over the lid's
six rims:

| rim (profile vertex) | slits the seam of segment |
|---|---|
| 0 | 5 |
| 1 | 1 |
| 2 | 1 |
| 3 | 2 |
| 4 | 3 |
| 5 | 5 |

So the colliding pairs are `{1, 2}` and `{5, 0}` — the two pairs that
share a slit — while `{2, 3}` and `{3, 4}` are ADJACENT and compose in
one request. Adjacency is necessary (two bands can only share a
support's seam if they share a support) and it is not sufficient.
`demos/tour/tests/teapot_document.rs` is that table, executed: the
three-rim request and the `{1, 2}` pair refuse at the slit's name;
`{1, 3}`, `{1, 4}`, `{2, 3}`, `{2, 4}` and `{3, 4}` each build 8/16/8.
At a quarter of the scene's radius — small enough that the concave
vent and knob rims have headroom — the only two adjacent pairs that
collide are still `{1, 2}` and `{5, 0}`.

## What the vocabulary is missing

A discriminator on `BandSlit` saying WHICH band slit the edge — the
way `RoleSeg::BandTrim` already carries its `RimSupport`. Until then a
document cannot NAME the output of a single request that rolls two
rims sharing a slit, whatever the kernel does with the geometry.

## Recourse today

Split the roll into two `Node::Fillet` requests at one radius, naming
the second request's rims as the first carried them through
(`FromTarget` of the name they had). That is what
`demos/tour/src/teapot.rs` does (its module docs' sixth finding), and
what it costs is measured rather than assumed: the two spellings build
the same census, the same three band tori bit for bit and the same
mass, and differ in the FACE ORDER — which moves three `teapotlid`
rows of `docs/tess-budget-data/tess-budget-baseline.csv` (the same
three triangle counts, permuted among themselves) and re-slots the uv
sheet's `teapotlid` cells, both re-baselined with that reason. When
this issue closes, the scene goes back to ONE request and the baseline
is re-cut BACK; `per_rim_answers`' live pin is what says so on the day.

**The code named here is `crates/editor-core`'s, which is outside
LIB's fence** (`work/lib/program.md`'s `paths` and its `keep_out`
clause). It is filed on this slate because `docs/LIB-TEAPOT-SPEC.md`
§8's stop clause routed an emitter gap here by name; re-homing it is
the orchestrator's call.

## Re-homed (2026-09-08, LIB orchestrator)

Moved from `work/lib/` to `work/docm/`: the code named is the blend name emitter (`crates/editor-core/src/names/{role,emit_blend}.rs`), DOCM's territory; the file itself says re-homing is the orchestrator's call. Id, body and header
are unchanged; the directory is the claim (`work/README.md`). LIB's
half — the Python/façade rows that move when this closes — is named in
the body and stays LIB's to execute once the kernel side lands.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/wire/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): the file it names is WIRE's (`names/emit*.rs`, `eval/wire.rs`, `product.rs` are in WIRE's paths). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Read against the tree (2026-09-15) — live, re-kinded `ruling`, one type citation stale

Read by the WIRE orchestrator before dispatch.
`crates/editor-core/src/names/role.rs` still declares **`BandSlit(NameRef)`
— a bare one-field variant with no discriminator** — while `BandTrim { .. }`
beside it is a struct variant carrying its `RimSupport`, which is exactly
the asymmetry the row's `What the vocabulary is missing` section names.
`emit_blend.rs` still inserts one row per `(edge, meridian)` pair. The
collision the row measures is therefore still reachable.

Stale in the row, and harmless: it cites `BandSlit(Box<StableName>)`; the
payload is now `NameRef`. The shape — one field, no discriminator — is
what the finding rests on and that is unchanged.

**Re-kinded from `issue` to `ruling`, and this is the orchestrator's
judgement rather than the row's own words.** The row states its fix shape
confidently and the precedent (`BandTrim`'s `RimSupport`) is right there,
so it reads dispatchable. It is not: `RoleSeg` is **persisted** document
vocabulary, so adding a discriminator is a document-format change with a
migration story, which `cut-off-arc-persists-as-a-corner-arc` says in as
many words is Ev's call. The two rows are one decision — *may the
persisted `RoleSeg` vocabulary grow, and at what migration cost* — with
two instances, and they go to Ev together rather than as two questions.

Unchanged and worth keeping in view: the recourse works today
(`demos/tour/src/teapot.rs` splits the roll into two `Node::Fillet`
requests and builds the same three band tori bit for bit), so nothing is
blocked on the answer — only the single-request spelling is.

## Re-homed to BLEND (2026-09-15), and why the ruling call was withdrawn

Two corrections by the WIRE orchestrator, in the order they were found.

**First: it is not a ruling.** The read above re-kinded this `ruling` on
the argument that a `RoleSeg` change is a document-format change with a
migration story. **That premise is false**, and three checks say so:

- `crates/editor-core/src/persist/mod.rs` — ratified at M4 PR 6 — opens
  *"No schema version, on purpose… Schema breaks are not at all a
  problem, because this is not released yet: no document exists outside
  this repository, and every checked-in document is a regenerable
  artifact."* Versioning is Band-4 work for the day a document ships.
- `crates/editor-core/src/names/README.md` N1 (Ratified, #74) ratifies
  the STRUCTURE — `RoleSeg` is one closed enum grouped by op, role
  arguments are themselves names, no floats, no arena keys — and lists
  variants illustratively, with ellipses, never enumerating the blend
  group. A new blend variant or a new field on one is an instance of N1,
  not an amendment to it.
- `crates/sweep/README.md` V3 (Ratified, #992) keeps this vocabulary
  *fillet-named on purpose*: the fence is against renaming it blend-ward,
  not against growing it.

No serialized document in the tree carries `BandSlit` either — every hit
is Rust source. So this is an ordinary defect with an ordinary fix, and
`kind` goes back to `issue`. Ev agreed in chat, 2026-09-15.

**Second, and the reason the file moves: WIRE owns none of it.** DOCM's
exit sweep re-homed this row here with the boilerplate *"the file it
names is WIRE's (`names/emit*.rs` … are in WIRE's paths)"*. That glob is
not WIRE's path list, which names `emit.rs` and `emit_topo.rs`
explicitly and **no `emit_blend.rs`**. Measured with
`scripts/work.py territory --files -`, the three files a fix touches are:

| file | owner |
| --- | --- |
| `crates/sweep/src/blend/naming.rs` (`rec.slits`) | **BLEND** |
| `crates/editor-core/src/names/emit_blend.rs` (the emitting arm) | unowned |
| `crates/editor-core/src/names/role.rs` (`BandSlit`) | **EDIT** |

BLEND, because the kernel half is BLEND's, because the vocabulary is
blend vocabulary whose fence (V3) lives in BLEND's own ratified README,
and because BLEND is live on this machinery. If the fix turns out not to
need the kernel record, it is one announced seam to EDIT — the ordinary
shape — rather than three fences none of which was WIRE's.

## What the taker inherits, so it is not re-derived

**The fix's precedent is in the adjacent variant.** `BandTrim { edge,
support: RimSupport }` already carries the discriminator this one lacks;
`BandSlit(NameRef)` is a bare one-field variant keyed on the source
meridian alone, which is exactly why two bands slitting one meridian
collide.

**The open design question, stated rather than guessed.** `rec.slits` is
`Vec<(EdgeKey, EdgeKey)>` — (the slit edge, the source meridian) — so
nothing there says WHICH band slit it. Either the kernel record grows the
band's identity (a `crates/sweep` change, BLEND's), **or** the emitter
derives it from the band face incident to the slit edge, which it already
names (`RoleSeg::BandFace(Vec<StableName>)`, the chain's source edges as
a sorted set). **Which of those two is available was not determined
here**, and it decides whether this is a one-crate or two-crate change.
Determine it before estimating.

**Nothing is blocked on it.** The recourse works today:
`demos/tour/src/teapot.rs` splits the roll into two `Node::Fillet`
requests and builds the same three band tori bit for bit. Only the
single-request spelling is unavailable.

Signed (WIRE orchestrator).
