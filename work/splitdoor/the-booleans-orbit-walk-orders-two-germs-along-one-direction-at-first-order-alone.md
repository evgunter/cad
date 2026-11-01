---
id: the-booleans-orbit-walk-orders-two-germs-along-one-direction-at-first-order-alone
kind: issue
title: The boolean's orbit walk reads two germs along one direction in one entry at first order alone: strut_order refuses them and walks_after calls them tied
status: open
opened: 2026-10-28
priority: P3
cost: M
---


Found by the sweep of
`a-boolean-whose-face-is-tangent-to-a-holes-rim-at-a-vertex-refuses-its-strut-order`
(branch `splitdoor/boolean-strut-order`).

## What

Two germs along one direction now read one order down in two homes:
the split's `splitting/insert.rs` `section_chords` and the pierce
ring's `boolean/vtxfac.rs` `germ_order`, both through
`geom_brep::tied_germ_turn`. The boolean's orbit walk at a vertex
pair does not:

- `boolean/insert.rs` `walk_faces_first` calls `strut_order` with a
  tie that reads nothing (`|| Ok(None)`), so two germs of one entry
  along one direction refuse `Escalated { Coincidence(Sectors, Moot) }`
  on `bool_strut_order`, unless both run beside an edge of the other
  solid (`beside_tie`).
- `boolean/insert.rs` `walks_after` (`bool_shared_cut_order`) returns
  `None` at the same tie, and every reader of `walks_before` takes that
  `None` as the one tie: `on_arc`, `arc_holds`, `holds_whole`,
  `tied_held`, `strut_anchor`, and `walk_order`/`leads` after
  `beside_tie`.

## Reading

A germ here is the section of the entry's face with a face of the
other solid, so its bend in the entry's face is
`geom_brep::implicit_section_acceleration(own face, other face, p, d,
n × d)`. The walk's `Germ` and `Cut` carry a direction only, and
`mint_directed` holds one body, so the bend has to be read where
`plan_null_pairs` holds both (one bend per solid, since each orders
the germ in its own face). `walks_after` returning an order where it
now returns `None` changes what every reader above calls a tie, which
is the larger half.

Unmeasured: no pose reaches either tie. A brick whose corner stands on
the hole rim's vertex of the holed blocks in
`crates/sweep/tests/a_split_through_a_wrap_edges_vertex.rs` (tilts
±0.4, ±0.9, six spins) never reads `bool_strut_order` or
`bool_shared_cut_order` in band or at zero; it fails earlier
(`a-brick-corner-at-a-holes-rim-vertex-leaves-loose-ends`).

## One tie arm (2026-11-01, `cleave/orbit-readings-read-the-beside-tie`, PR 6184)

Every reader in the second bullet, and `walk_faces_first`, now asks
`insert::cut_order`, the orbit's one position order. Its `Tied` arm is
the one place a first-order tie is read further (`beside_order`, a
`Sign`, `Zero` where it reads nothing); `beside_tie` and
`walks_before_beside` are gone. A one-order-down reading of the bend
goes in that arm, and in `Cut::tie_reads`, which says which pairs
`walk_faces_first` asks it about.
