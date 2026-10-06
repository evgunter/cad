---
id: meridian-edge-has-no-minting-builder
kind: issue
title: the minting builders spell a meridian vertex but not a meridian edge, so a seam or cap meridian is hand-spelled
status: open
opened: 2026-10-06
priority: P3
cost: E
refs: [band-rim-pi-has-no-minting-builder]
---


## Finding

`pncad::select`'s minting builders (`editor-core/src/names/role.rs`:
`band`, `band_pi`, `band_rim`, `band_rim_pi`, `meridian_vertex`,
`carried`) cover the revolve's faces, its rims and its meridian
vertices, but not its meridian EDGES, `RoleSeg::Meridian(end, run)`
(the seam meridian on a full revolve, the wedge-cap meridians on a
partial one, and `MeridianEnd::Pi` in the wire case). A consumer
authoring a selection over one hand-spells
`StableName { kind: EntityKind::Edge, node, path: vec![RoleSeg::Meridian(end, piece.into())] }`.

Met at `demos/tour/tests/teapot_document.rs`, `seam_of`; found by the
sweep for hand-spelled revolve role names in the PR that added
`band_rim_pi` (`band-rim-pi-has-no-minting-builder`).

## Fix shape

`meridian(end, node, piece: ProfileEdgeRef) -> StableName` beside
`meridian_vertex`, through `pncad::select` and the python `select`
module, with its census entry. Not a design question: the role and
its kind (`Edge`) are fixed; only the argument order wants to match
`meridian_vertex`'s.
