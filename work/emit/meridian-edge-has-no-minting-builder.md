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
`band_rim_pi` (`band-rim-pi-has-no-minting-builder`). The other
single-piece hand spellings a builder would replace:

- `crates/editor-core/tests/m4_pr3_names.rs` — the wedge-cap meridians
  of the partial revolve (`:274`), the seam meridians of the full
  square torus (`:344`), of the flange (`:387`) and of the holed
  revolve (`:443`), and the seam and `Pi` meridians of the wire case's
  segment 0 (`:554`, `:555`);
- `crates/editor-core/tests/ring_r1_names_probe.rs` (`:88`), the seam
  meridians of the ring's second loop;
- `crates/editor-core/tests/corpus/die_composed.rs` (`:203`).

The multi-piece run sites in `crates/editor-core/tests/band_run_wall_names.rs`
(`:201`, `:215`, `RoleSeg::Meridian(.., run_of(..))`) are out of a
single-piece builder's reach, as the run sites of `band` are.

## Fix shape

`meridian(end, node, piece: ProfileEdgeRef) -> StableName` beside
`meridian_vertex`, through `pncad::select` and the python `select`
module, declared in `pncad.pyi` (which is how the binding census
accounts it). Not a design question: the role and
its kind (`Edge`) are fixed; only the argument order wants to match
`meridian_vertex`'s.
