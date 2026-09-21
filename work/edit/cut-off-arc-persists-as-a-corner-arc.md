---
id: cut-off-arc-persists-as-a-corner-arc
kind: issue
title: names: a ruled band's cut-off arc persists under RoleSeg::CornerArc
status: closed
opened: 2026-09-05
pr: 2717
branch: edit/cutoff-arc-rename
closed: 2026-09-16
---

## Finding

FILLET-H7's ruled carve records its cut-off arcs in `BlendNaming::arcs`
(`(edge, source vertex, source edge)`, `crates/sweep/src/blend/open/ruled.rs`,
`ruled_phase`), the row the planar open band uses for its CORNER arcs.
`crates/editor-core/src/names/emit_blend.rs` keys that row as
`RoleSeg::CornerArc { vertex, edge }` — so a transverse cap's cut-off arc
persists, in the document's stable names, as a "corner arc" of the cap
vertex, although the ratified vocabulary (`CornerConfig::TransverseCap`,
`docs/FILLET-H7-SPEC.md`) says a transverse cap is NOT a corner: the ball
does not turn there and no corner patch is minted.

Likewise the cap feet ride `feet` (`RoleSeg::FootVertex`) and the
surviving rim piece rides `meridian_remnants` (`RoleSeg::BandCut`) —
roles whose words are the corner's and the ladder's. The names are
STABLE and DETERMINISTIC (the rows are keyed by source entities), so no
document breaks; the question is whether the persisted role vocabulary
should say what the entity is.

## Why not changed in H7

The persisted `RoleSeg` vocabulary is one of the three fences V3 keeps
fillet-named on purpose (`crates/sweep/README.md`, "V3"), and adding a
role is a document-format change with a migration story; no editor row
exercises a ruled carve today (`editor-core/tests` has no ruled fixture),
so the shape of the name has no consumer to be wrong for yet.

## Fix shape

Either a `RoleSeg::CutOffArc { vertex, edge }` (and `CapFoot`, `CapRimCut`)
with the format version bumped and an editor row driving a rod-with-a-
flat through `emit_fillet`, or a written ruling that the corner-arc role
is the right family for any arc that closes a band at a source vertex.
Ev's call, since it is persisted vocabulary.

## Cross-program note

The code is `editor-core`'s names layer (NAMING ground). Filed under
FILLET as the program that minted the entity; the owner places it.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/wire/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): the file it names is WIRE's (`names/emit*.rs`, `eval/wire.rs`, `product.rs` are in WIRE's paths). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Read against the tree (2026-09-15) — live, premise intact, re-kinded `ruling`

Read by the WIRE orchestrator before dispatch.
`crates/editor-core/src/names/emit_blend.rs` still keys the ruled
carve's cut-off arcs as `RoleSeg::CornerArc { .. }` (now ~line 156, filed
at ~212), and `RoleSeg::CornerArc` is still the planar open band's corner
role in `role.rs`.

**The row's own reason for not fixing it in H7 still holds, and it was
checked rather than assumed.** `crates/editor-core/tests` contains no
ruled-carve fixture and no mention of `TransverseCap` anywhere in
`editor-core` — every `grep` hit for "ruled" under those tests is the
English word in a sentence about a ruling. So the persisted name shape
still has **no consumer to be wrong for**, which is what keeps this a
vocabulary question rather than a defect with a victim.

**Re-kinded from `issue` to `ruling`**, which is what the row asked for
in its `Fix shape` section: *"Ev's call, since it is persisted
vocabulary."* Filed as an issue it sat on the board looking dispatchable.

Goes to Ev with `blend-slit-name-collides-when-two-rims-share-a-meridian`
as one question in two instances: whether the persisted `RoleSeg`
vocabulary may grow a role that says what the entity IS, and what the
format-version and migration story costs. This row is the "the word is
wrong" instance; that one is the "the word cannot discriminate two legal
entities" instance.

## RULED (Ev, in chat, 2026-09-15): option (b′) — one role, and the CLAIM gets fixed

The row asked for either a new `RoleSeg::CutOffArc` (with `CapFoot`,
`CapRimCut`) or a written ruling that the corner-arc role is the right
family. Ev ratified a third shape, **(b′)**: keep ONE role, and repair
what it *claims* rather than growing the enum.

### Why not a new variant, which is the part that generalises

`CornerConfig::TransverseCap` against `ThreeConvexEdges` is a
**geometric classification** — `fillet3_cap_transverse` at the link's own
extent, plus the convexity and valence reads. A separate variant bakes
that classifier's verdict into a **persisted identity**, so an edit
carrying a vertex across the transverse/corner boundary re-spells the
name and a stored selection stops resolving — for an entity that is
structurally the same entity before and after: *the arc where this band
closes at this source vertex*.

That is the hazard class this vocabulary already names in its own words,
at `RimSupport`: *"A pair of structural ROLES, not a geometric
classification… the planarity boundary is where this vocabulary is NOT
covariant, and it is load-bearing enough to state here."* N1's design is
that a name says how an entity was DERIVED, not what shape the geometry
came out.

**A discriminator field was considered and rejected**, though it mirrors
`BandTrim { edge, support: RimSupport }` and the `BandSlit` fix now on
BLEND's slate. It is not the same shape: on `BandSlit` the discriminator
is needed for UNIQUENESS (two bands genuinely collide), whereas a source
vertex is either a corner or a transverse cap and never both, so
`(vertex, edge)` is already unique. The field would be derived data
inside an identity, carrying (a)'s instability and buying nothing.

**The counterargument, recorded because it is real and was weighed.** A
corner arc bounds band|octant; a cut-off arc bounds band|cap — different
neighbours, and `role.rs` does hold that stopping loudly beats silently
retargeting. It was judged to prove too much: by it every role whose
neighbours depend on geometry would need a configuration tag, and
`RimSupport`'s doc declines that generalisation explicitly.

### What the work is

**Three doc comments stop making a geometric claim the role cannot
keep.** Each states the structural role instead, of which the octant seam
and the transverse cut-off are two configurations, told apart by the body
and the minting node — the `OpGroup::Fillet` move `crates/sweep/README.md`
V3 already ratified, applied one level down:

| role | the claim to retire |
| --- | --- |
| `CornerArc { vertex, edge }` | *"where an octant meets one of its three incident blends"* — false for every ruled carve |
| `FootVertex { vertex, support }` | *"a corner foot … from a source corner vertex"* — a transverse cap vertex is not one |
| `BandCut(NameRef)` | *"the shortened meridian"* — the ladder's word; on a ruled band it is a cap rim edge |

**One rename: `CornerArc` → a structural word.** Ratified with the rest,
and it is the only judgement in the package rather than a derivation.
Renames are free today (`persist/mod.rs`: no schema version, nothing
shipped, every checked-in document regenerable) and never will be again.
`EndArc { vertex, edge }` is the orchestrator's suggestion, not part of
the ruling — the taker picks the word.

**`CornerFace` is NOT renamed and that is the test of the rule.** It
names the octant patch itself, which exists only where there IS a corner,
so its word is structurally true. Only `CornerArc` names a thing that
exists at every band end.

**The V3 check is the taker's first task.** `crates/sweep/README.md` V3
keeps this vocabulary *fillet-named on purpose*, and that is a ratified
clause. The fence reads as being against renaming it BLEND-ward, and
corner→structural is a different axis — but if a lane reads it as a fence
crossing, **stop and report rather than amend**, per this program's
standing rule.

**Not in scope, and filed separately:** the rod-with-a-flat editor
fixture. It is worth having on its own merits rather than as the price of
a vocabulary change, and it is now
`a-ruled-carve-has-no-editor-fixture-so-emit-fillet-s-band-end-roles-are-unexercised`.

## Re-homed to EDIT (2026-09-15)

WIRE owns neither file this touches. `crates/editor-core/src/names/role.rs`
is **EDIT's** (`work.py territory --files -`) and
`crates/editor-core/src/names/emit_blend.rs` is claimed by no program.
This row reached WIRE at DOCM's exit sweep on the boilerplate *"the file
it names is WIRE's (`names/emit*.rs` … are in WIRE's paths)"* — a glob,
where WIRE's path list names `emit.rs` and `emit_topo.rs` explicitly and
no `emit_blend.rs`. The same error moved
`blend-slit-name-collides-when-two-rims-share-a-meridian`, now on BLEND's
slate. EDIT's charter is the persisted recipe and the edit vocabulary,
which is exactly what a persisted role word is.

Signed (WIRE orchestrator).

## Built (2026-09-16) — PR #2717, `edit/cutoff-arc-rename`

E-class. The ruling is built as written, and nothing in it was
re-litigated.

**The V3 read came first and the fence is not crossed.**
`crates/sweep/README.md` V3's axis is the VERB — blend-named machinery,
fillet-named fences against renaming BLEND-ward. `CornerArc → EndArc` is
corner → structural, a different axis, and V3's own reason for keeping
`OpGroup::Fillet` (a name that under-describes what it groups, with the
minting node telling the configurations apart) is the move applied one
level down. `crates/sweep/` is untouched and the README is not amended.

**Landed.** `RoleSeg::CornerArc` is `RoleSeg::EndArc`, chosen over
`BandEndArc`/`BandCloseArc` because the `Band*` prefix is already the
closed-chain rim family's in the same enum. The three docs state the
structural role: `EndArc` is the arc where a blend band closes at a
source vertex (octant seam and transverse cut-off two configurations of
it), `FootVertex` is retracted from the source vertex where the band
ends rather than from "a source corner vertex", `BandCut` is the
surviving piece of a source edge the trimline cut rather than "the
shortened meridian". `CornerFace` is NOT renamed: the octant patch
exists only where there is a corner.

The sweep covered `role.rs`, `emit_blend.rs`, `select.rs`,
`attribute.rs`, `resolve/mod.rs`, `refactor.rs`, `eval/anchor.rs`,
`eval/mod.rs`, four `crates/editor-core/tests/` files the brief did not
anticipate, and the compile-enforced Python mirror
(`crates/pncad-py/src/py/select.rs`, `pncad.pyi`).
`sweep::blend`'s `ContactCarrier::CornerArc` is a different type and is
left alone.

**No committed document, golden or corpus carried the old word** — the
persisted text digests of all 28 registry documents are byte-identical.
Four COMPUTED goldens moved, being FNV-1a over the name tables' `Debug`
encoding: `lib_g16_corpus_name_digests`, `perf2_name_keying_differential`
(name-table column only) and `seat4_verb_lowering`, on the four
fillet/chamfer-bearing documents and no others. Re-baselined from the
printed fresh tables.

**Not built:** the rod-with-a-flat editor fixture, which is
`a-ruled-carve-has-no-editor-fixture-so-emit-fillet-s-band-end-roles-are-unexercised`.
The band-end roles are still unexercised through the document layer, so
this row's premise (no consumer to be wrong for) still holds.

No rows filed: the sweep turned up no defect outside the fence.

## Closed (2026-09-16, EDIT orchestrator)

Merged as PR #2717 on green CI (run 35046820507, full matrix) and the
orchestrator's read: the diff is the ruling and nothing else. The
fixture row stands on its own.
