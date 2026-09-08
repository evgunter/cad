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
