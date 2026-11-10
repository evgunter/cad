---
id: the-containment-move-leaves-two-names-in-boolean
kind: issue
title: After the containment component left boolean/ for the crate root, boolean still hosts a structural reading the component reads (edge_join::is_conventional_vertex) and a namesake of one of its items (records::landings beside region::landings)
status: open
opened: 2026-11-10
priority: P3
cost: E
---


Found by step 2 of
`work/onecopy/the-census-sort-of-a-boolean-refusal-has-no-legal-home-under-topo-homes.md`
(`onecopy/contain-component`). That step moves the containment component
(`contain`, `region`, the region arms, `solid_contain`, `rim_wedge`,
`surface_group`) from `boolean/` to the crate root.

## What

- **`edge_join::is_conventional_vertex`** (`crates/topo/src/boolean/edge_join.rs:322`)
  is a structural reading of a body: a vertex whose only edge is one
  closed edge at both its ends. It names no boolean state.
  `solid_contain/curved_edges.rs` reads it, which was an engine-internal
  read before the move and is now a root file naming the engine. That is
  the one cited `names-boolean` site in
  `scripts/gates/engine-leaves.counts` for that file. `census.rs` and
  `census/curved.rs` read it too, inside their own pins.
- **`records::landings`** (`crates/topo/src/boolean/records.rs:747`), the
  carry's landings of a record's cells, shares its name with
  `region::landings` (`crates/topo/src/region.rs:299`), a path's
  crossings of a face's boundary. With `region` at the root the name is
  declared under an engine and outside it, so `engine-leaves` reads it as
  ambiguous. Its `ambiguous landings` line cites this row.

- **`kind_word`**, a surface kind as a refusal sentence spells it, moved
  to `contain.rs` with the component (PR 6775) only because one member
  (`solid_contain`) reads it. Most of its readers are the Boolean's
  (`boolean/mod.rs` reads it back through a private `use`), so `contain`
  is a home of convenience: a vocabulary module for a refusal's words
  is the right one, once there is one.

## The fix

Move `is_conventional_vertex` to the crate root, beside the vertex
readings that take a body. Rename one of the two `landings`, for what
each reads (the record's cell landings, the path's boundary landings), so
neither is a namesake. Both lines in the counts file fall with it. Give
`kind_word` a home with the other refusal words.
