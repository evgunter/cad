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
