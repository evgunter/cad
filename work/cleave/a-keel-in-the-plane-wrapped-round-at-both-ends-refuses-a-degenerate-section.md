---
id: a-keel-in-the-plane-wrapped-round-at-both-ends-refuses-a-degenerate-section
kind: issue
title: A split whose plane holds a valley edge that the material above wraps round at both ends refuses DegenerateSection
status: closed
opened: 2026-10-11
priority: P2
cost: M
branch: cleave/keel-slit-ring
closed: 2026-10-19
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

## Notes from the radial-edges fork (2026-10-14)

Both designers keep the manifold model; see 4795's round 2. Things the
build owes or should check:
- **Mesh.** `crates/mesh/src/validate.rs` counts edges by vertex-index
  pair. Two coincident straight edges sharing both end vertices count
  4, so the ring may hit `NonManifoldEdge`. Measure when the ring lands.
- **Lineage.** The split turns one keel into two coincident edges, each
  holding the whole line. That is neither "divided" nor "substituted".
  See how 4622's slit form records it, and NAMING's policy for crease
  copies.
- **Shared curve.** Both edges can share one curve entry. Today only
  null-scaffold curves are barred from sharing (validate.rs pass 13).
  Sharing makes the doubled line structural, as fork-log row 45 did for
  shared point keys.
- **If Ev rules B's representability question:** `NonManifoldResult`'s
  "permanent" claim for two pinches crossing on one line falls. That
  covers `boolean/insert.rs` `reconcile_shared`, the variant doc, and
  `union_flush_onto_edge_contact.rs`. File it on the boolean's slate
  then.

## Decided (Ev, `[ev]` PR 4795, 2026-10-19)

Ev ratified the D1 sentence in `crates/topo/README.md` tier 3: where
more than two sheets meet on a line, the material decides the edges.
Ev kept the manifold model and declined radial edges ("ok phew i'm glad
our current representation is better"). The keel builds in the slit
form: Above's face carries the keel as a zero-area ring of two
coincident edges. The build notes above stand. Ev also ruled on two
neighbouring sentences:
- the knife slit is "an isolated cusp's `revert` image";
- the representability boundary speaks of non-solids only.

Code follow-up: `non-manifold-result-calls-a-buildable-solid-permanently-unrepresentable`.

## Built (`cleave/keel-slit-ring`, 2026-10-19)

- The join reads a slit trace from topology before the area check: a
  loop that runs along each of its edges both ways (a tree walk; for one
  keel, two struts). That covers two crossing keels and a bent one too,
  which refused the same way. A tree with a curved edge refuses typed
  (`CurvedSlitTrace`), since its ring would fail tier 3′.
- The finish drops the strut loop's face, a component of its own, and
  nests the slit as a ring of the section face containing it, read by
  containment: a vertex of it read against each loop on its own
  carriers (`nest_slit`, `point_in_loop`). A reading that escalates or
  cannot be made refuses `SlitHoming`; a definite contradiction refuses
  `SlitUnplaced`.
- `plane_section` skips slit traces.
- Volumes: 15.7 above and 16 below. The 12.65 in 4795's round-1 report
  does not match the closed form, 16 − ½ · 1 · 0.6 · 1.
- Filed:
  - `a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs`: the
    curved ring fails tier 3′, as 4622's one-end form already does;
  - `the-split-keeps-an-in-plane-edge-it-doubles-at-a-dead-key`:
    lineage;
  - the mesh: evidence added to
    `work/tess/two-coincident-edges-between-one-vertex-pair-mesh-non-manifold.md`.
