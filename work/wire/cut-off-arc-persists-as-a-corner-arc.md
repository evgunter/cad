---
id: cut-off-arc-persists-as-a-corner-arc
kind: ruling
title: names: a ruled band's cut-off arc persists under RoleSeg::CornerArc
status: open
opened: 2026-09-05
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
