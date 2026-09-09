---
id: LIB-NAMES
kind: unit
title: builders mint a revolve's role names, reached at pncad::select
status: review
opened: 2026-09-09
branch: lib/names
refs: [no-facade-door-mints-a-revolves-role-names]
pr: 2248
---


Closes `no-facade-door-mints-a-revolves-role-names` under its
`## Ruled` (A): `band`, `band_pi`, `band_rim`, `meridian_vertex` and
`carried` are free functions returning a `StableName`, reached at
`pncad::select`.

## Delivered

- **Where they are DEFINED: `crates/editor-core/src/names/role.rs`**,
  beside the `RoleSeg` arms they mint, re-exported through
  `names::mod`, `editor_core`'s root and `pncad::select`. The ruling
  left the seat to the unit because `crates/editor-core/tests/corpus/`
  cannot depend on `pncad`; defining them one crate down is what lets
  the corpus share them. `names/README.md` argues nothing against it —
  its own module table puts the N1 vocabulary in `role.rs`, and
  `names/mod.rs` already holds free functions beside the types
  (`all_edges`, `all_faces`, `all_vertices`).
- **Signatures are the ruling's, verbatim**, including
  `meridian_vertex(end, node, vertex)` with the end first (the order
  `RoleSeg::MeridianVertex(end, ref)` carries). Each fixes the
  `EntityKind` its role denotes: `Face` for `band`/`band_pi`, `Edge`
  for `band_rim`, `Vertex` for `meridian_vertex`; `carried` takes the
  inner name's kind, since a survivor is the same entity one op later.
- **`loop_index: 0`**, the outer loop, is fixed by the `u32`
  signature. Disclosed and filed as
  `the-role-name-builders-reach-only-the-outer-profile-loop`.
- **Carried at `pncad::select` only, not in `pncad::prelude`.** The
  ruling names that seat; the prelude's group 9 is curated by hand
  with its own argument and nothing mechanical forces it to mirror
  `select`. Both tour files already import from `pncad::select`.
- **Consumers converted**: `demos/tour/src/teapot.rs` (its five
  private helpers deleted), `demos/tour/tests/teapot_document.rs`
  (two), `crates/editor-core/tests/corpus/vessel.rs` (its two `pub`
  helpers deleted, and `vessel::band`/`band_pi`'s callers in
  `lib_g17_shell_node.rs`, `lib_g17_r1_probes.rs`,
  `lib_g17_r2_probes.rs` moved to `editor_core::band`), plus the
  hand-spelled `BandRim` names in `blend5_r1_probes.rs`,
  `blend5_r2_probes.rs`, `blend5_rim_support.rs` (two) and
  `seat6_param_source.rs`, and the `FromTarget` wrappers in
  `m6_5_downstream.rs` and `lib_g17_shell_node.rs` (two).
- **Pins**: five, one per builder, in `role.rs`'s own `mod tests` —
  each asserts the exact `StableName` against the hand-spelled form it
  replaces, and `carried`'s also pins that the wrapper keeps the inner
  kind.
- **No `pncad-py` change.** Python doors are not this unit; nothing
  was filed, because the item's ruling scopes the request to the Rust
  façade and no Python consumer hand-spells one today.
