---
id: a-keel-in-the-plane-wrapped-round-at-both-ends-refuses-a-degenerate-section
kind: issue
title: A split whose plane holds a valley edge that the material above wraps round at both ends refuses DegenerateSection
status: open
opened: 2026-10-11
priority: P2
cost: M
needs_ev: true
---


Found by the class sweep of
`a-crown-touching-the-plane-along-one-edge-panics-at-the-section-spur-check`
(branch `cleave/crown-spur-panic`).

## What

The box `[0, 4] × [0, 4] × [−1, 1]` less a trench along x: the prism
over the (y, z) triangle (2, 0), (2.6, 2), (1.4, 2), from x = 1.5 to
2.5 (`topo::subtract`, a body). Its keel, the edge from (1.5, 2, 0) to
(2.5, 2, 0), is a valley lying in the plane z = 0. Split by that plane
with either normal, it refuses `Join(DegenerateSection)` on the
section the keel makes (measured at the default ε). With the keel at
z = 0.2 the same body splits sound, tier 3 and corners disjoint on
both sides.

Rule (b) reads the keel reflex and sends it Below
(`crates/topo/src/splitting/rules.rs`, the reflex row). At each end of
it every other edge rises, so the material above joins round both
ends in one run: one null edge at each end. The two null edges close
into a section of their own, two chords along the keel, which bounds
nothing; `Sweep::certify_section_area`
(`crates/topo/src/splitting/join.rs`) refuses it.

The input is valid and the answer exists. Above touches itself along
the keel: its section is the box's square with the keel as a slit
inside it, a zero-area ring of two chords. Below's section is the
square, with the keel and its two end vertices gone. The refusal's
text is true of the polygon, but it does not name the cause, and the
`DegenerateSection` docs (`crates/topo/src/chord_join.rs`: "A pinch
does not reach it") do not hold for a pinch along an edge.

## Neighbour

Where the material joins round ONE end only (a crown with one ring
corner in the plane), the keel's trace joins the real section's loop
as a spur: a strut on Below's loop and a slit on Above's, which the
join pairs and whose strut it kills (`Sweep::strip_spurs`). This row is the same edge with both
ends wrapped, where the trace is a loop of its own.

## Owed

Either build it, Below's strut loop killed whole and Above's
two-chord slit nested as a ring of the square's section face (tier 3
has to accept a zero-area ring, which is a design question), or refuse
it with a payload that names the self-touch along the edge.

## Build notes (from the design fork, PR 4795)

- A completed section is an area polygon or a slit trace, decided from
  topology before `certify_section_area`: a loop made entirely of
  struts (`strut_turn`) is a slit. Its strut loop dies whole on the
  side away from the keel's walls (Below under +z, Above under −z);
  the other loop is nested as a ring of the section face of its side
  that contains a tip vertex, by containment, not winding
  (`section_sense` refuses a zero winding, so the role is read first;
  `nest_hole_sections` needs a slit entry). A slit landing in no
  outline, or inside a hole, refuses as a kernel defect.
- Killing the strut loop leaves a lone-vertex ring (`kev` leaves an
  empty loop); no single operator kills it today (`mekr` then `kev`,
  or a new kill). `below_chord_u_ref` reads a chord that no longer
  exists; the slit's u_ref comes from the face it nests into.
- `plane_section` skips slit sections.
- `DegenerateSection`'s docs in `chord_join.rs` ("A pinch does not
  reach it") and the `strip_spurs` paragraph citing this row are
  rewritten: an edge pinch wrapped at both ends does reach it today.
- Pin rows: the input at both normals and three ε values, plus a
  curved-keel twin. A later cut through the slit refuses `SliverSector`
  until `work/tang/declared-cusps-second-order-wedge-arm.md` lands.
