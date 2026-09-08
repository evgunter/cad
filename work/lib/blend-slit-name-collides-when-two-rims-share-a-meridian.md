---
id: blend-slit-name-collides-when-two-rims-share-a-meridian
kind: issue
title: the blend name emitter refuses a roll of two rims that share a meridian segment (RoleSeg::BandSlit has no discriminator)
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

Two of the rims — the flange's rim and the dome's foot — stand at the
two ends of ONE meridian segment (the flange cone), so both bands slit
that segment's seam meridian.
`crates/editor-core/src/names/role.rs:610` defines
`BandSlit(Box<StableName>)` as *"the source edge whose severed piece
became it"*, and `crates/editor-core/src/names/emit_blend.rs:212`
inserts one row per `(edge, meridian)` pair in `rec.slits`. Two slits,
one source name, one `DuplicateName`.

The shape is general rather than this lid's: ANY two adjacent latitude
rims on ANY solid of revolution rolled in one request collide the same
way, which is the ordinary case for a turned part. The vocabulary is
missing a discriminator saying WHICH band slit the edge — the way
`RoleSeg::BandTrim` already carries its `RimSupport`.

Recourse today, and what `demos/tour/src/teapot.rs` does (its module
docs' sixth finding): split the roll into two `Node::Fillet` requests
at one radius, naming the second request's rims as the first carried
them through (`FromTarget` of the name they had). The geometry is the
same by every number the scene asserts — 9/18/9, three ring-free tori
at the three closed-form spine stations, the same ΔV — and the face
ORDER is not: three `teapotlid` rows of
`docs/tess-budget-data/tess-budget-baseline.csv` permute their
triangle counts (`{16200, 2048, 42560}`, same multiset, same per-scene
total), and the uv sheet's `teapotlid` cells, which are labelled by
face index, show different charts.

**The code named here is `crates/editor-core`'s, which is outside
LIB's fence** (`work/lib/program.md`'s `paths` and its `keep_out`
clause). It is filed on this slate because `docs/LIB-TEAPOT-SPEC.md`
§8's stop clause routed an emitter gap here by name; re-homing it is
the orchestrator's call.
