---
id: doors-carrying-a-shared-row-have-no-row-at-rest
kind: issue
title: The doors that carry a shared-row selector (rigid map, key remap, edge join, split, re-anchor) have no row: no body at rest carries one yet
status: open
opened: 2026-10-28
priority: P2
cost: M
refs: [the-boolean-recovers-no-structural-selector-for-a-rim-known-in-advance]
---

Filed by SPLICE's structural-rim lane.

## What

An `Intersection` under `Selector::Structural(SharedRow)` travels through
these doors with its row images verbatim, and its re-run certificate
decides whether the rows still hold:

- `topo/src/transform.rs` `map_isometry`: each wall's row maps as a
  chart image does (`mapped_image`, reflected under an improper map);
- `geom-brep/src/certify.rs` `with_remapped_surfaces`, and
  `topo/src/boolean/combine.rs`'s graft and
  `topo/src/replace_face.rs` `remap_description`: keys re-mapped through
  `Selector::remapped`, which re-orders the images when the pair's order
  flips;
- `geom-brep/src/certify.rs` `split_specs`, `topo/src/boolean/edge_join.rs`
  `joined_spec` and `topo/src/replace_face.rs`'s re-anchor and
  `read_ends`: the rows kept over the new interval;
- `topo/src/offset_restate.rs` `restate`: the rows kept.

None of them has a row through a body. The only producer is the
boolean's recovery, and its one witness (lily wall 8,
`crates/sweep/tests/a_sheath_grafted_on_its_blade.rs`) stops at the
interior-loop guard, so no body at rest carries a shared row. The
remap's re-ordering is guarded at `geom-brep`
(`a_shared_row_is_symmetric_in_its_pair`); the others are not.

## What would close it

Once a boolean returns a body with a shared-row rim, a row per door: a
rigid map (proper and improper) of that body, a split of the rim edge,
and an offset of one wall, each certifying or refusing typed.

