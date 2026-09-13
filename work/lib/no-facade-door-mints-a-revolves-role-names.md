---
id: no-facade-door-mints-a-revolves-role-names
kind: issue
title: no facade door mints a revolve's role names, so every consumer hand-spells the StableName
status: closed
opened: 2026-09-08
closed: 2026-09-09
---


A revolve's entities are named by ROLE — `Band(loop, segment)`,
`BandPi`, `BandRim(loop, vertex)`, `Meridian(end, edge)`,
`MeridianVertex` — and every consumer that wants one builds the
`StableName` by hand, field by field:

```rust
StableName {
    kind: EntityKind::Face,
    node,
    path: vec![RoleSeg::Band(ProfileEdgeRef { loop_index: 0, segment: seg })],
}
```

`crates/editor-core/tests/corpus/vessel.rs` spells `band`/`band_pi`
that way; `demos/tour/src/teapot.rs` spells `band`, `band_pi`,
`band_rim` and `meridian_vertex` that way through the `pncad` façade;
`demos/tour/tests/teapot_document.rs` spells `band_rim` and `carried`
again; both reviewers of PR 2206 spelled the same four a fourth and
fifth time in their probe branches. Neither `pncad::prelude` nor
`pncad::select` offers a constructor: what they carry is the
VOCABULARY (`RoleSeg`, `ProfileEdgeRef`, `ProfileVertexRef`,
`EntityKind`, `StableName` with public fields) and the doors that
answer names (`select`, `all_edges`, `face_name`, `edge_name`), so
authoring one is possible and is never one call.

The gap bites where a selection must be AUTHORED rather than
materialized — a shell's open list, a fillet's frozen selection — which
is exactly the case `Node::Shell` and `Node::Fillet` are for. A
`select`-shaped answer cannot be used there without an evaluation to
select against, and the corpus documents deliberately author instead.

What would close it: builders beside the vocabulary, at the same seat
that already carries `SegPat::tag(SegTag::Band)` for the matching
direction — so that saying "the [0, π) face of segment 3 of this
revolve" is one call, and the pass-through wrapper a survivor takes
(`FromTarget` of the name it had, which the tour spells as `carried`)
is another.

**A second duplication rides with it and is worth recording here**:
`demos/tour/src/teapot.rs` and `crates/editor-core/tests/corpus/vessel.rs`
carry the SAME vessel meridian, station for station, and the same
`R_FOOT`/`R_BELLY`/`R_NECK`/`Y_FOOT`/`Y_BELLY_C`/`Y_MOUTH`/`WALL`
block. That is deliberate — the tour is a detached workspace and the
kernel must never depend on demo tooling, and the corpus must never
depend on the tour — so it is disclosed rather than shared, and it is
a second reason a name BUILDER would earn its place: the two copies
would at least say the names the same way.

## Question for Ev (2026-09-08, LIB orchestrator; `[ev]` PR)

Five sites now hand-spell a revolve's role names field by field
(`StableName { kind, node, path: vec![RoleSeg::Band(ProfileEdgeRef {
loop_index: 0, segment })] }`) because the façade carries the
VOCABULARY and the doors that ANSWER names, and nothing that MINTS one
— and authoring a selection (a shell's open list, a fillet's frozen
selection) is exactly where a name must be written before any
evaluation exists to select against. What shape should the builders
take?

- **(A) Free functions beside the vocabulary in `pncad::select`** —
  `band(node, seg)`, `band_pi(node, seg)`, `band_rim(node, vertex)`,
  `meridian_vertex(end, node, vertex)`, and `carried(node, inner)` for
  the `FromTarget` wrapper a survivor takes — each returning a
  `StableName`, at the seat that already carries `SegPat::tag(…)` for
  the matching direction. Mechanical afterwards; the five consumers
  convert. Recommended.
- **(B) A builder type or a method chain on the node id**
  (`node.band(seg)`) — reads well at the call site, but puts an OO seam
  on an id type and a second way to say what (A) says.
- **(C) Leave hand-spelling** and document the shape; five copies stay
  five.

Recommendation: **(A)**. Small, in LIB's fence, and it is what the two
duplicated vessel spellings (tour and corpus) would at least share.

### Why (A) over (B), added 2026-09-09 after Ev asked

Essentially less machinery. `RecipeNodeId` is editor-core's type, so
`node.band(seg)` means an extension trait a caller imports (or a
wrapper type), while (A) is free functions beside `SegPat::tag(…)`,
which is how `pncad::select` already spells the matching direction.
Both mint the same `StableName`, and neither can check that the node
is a revolve before resolution. If (B)'s call-site reading is
preferred, an extension trait in `pncad::select` is a fine shape.

## Ruled (2026-09-09, Ev, `[ev]` PR 2231)

**(A).** Ev: "A is fine." Free functions beside the vocabulary,
reached at `pncad::select` — `band`, `band_pi`, `band_rim`,
`meridian_vertex`, `carried` — each returning a `StableName`.
Mechanical afterwards: the builders and the conversion of the
consumers that can reach them (the unit decides where the functions
are DEFINED so that the corpus, which cannot depend on `pncad`, can
share them too, and says so).

## Closed (2026-09-09, LIB-NAMES)

`band`, `band_pi`, `band_rim`, `meridian_vertex` and `carried` are
free functions returning a `StableName`, defined beside the vocabulary
in `crates/editor-core/src/names/role.rs` and reached at
`pncad::select`; thirteen hand-spelling sites in eight files converted
(the tour's five private helpers and the corpus's two `pub` ones
deleted), five pins one per builder, tour renders and die corpus
byte-identical. Residues: `the-role-name-builders-reach-only-the-outer-profile-loop`
and `pncad-py-has-no-door-that-mints-a-revolves-role-names`. See
`work/lib/LIB-NAMES.md`.
