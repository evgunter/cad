---
id: a-keel-on-a-cylinders-ruling-subtracts-with-the-bore-through-its-wedge
kind: issue
title: A keel resting on a bore's ruling subtracts as one edge between the walls, the bore's face running through its material wedge, and every tier passes it
status: open
opened: 2026-10-29
priority: P1
cost: M
refs: [census-passes-a-face-inside-an-edges-material-wedge, a-slit-tip-on-a-curved-face-or-at-a-cone-apex-reads-its-full-turn-as-a-spike]
---

Found while building the witness of
`a-slit-tip-on-a-curved-face-or-at-a-cone-apex-reads-its-full-turn-as-a-spike`
(branch `splitdoor/slit-tip-readers`), measured on main `c295cb56` at
ε 1e-9.

## What

The block `[−2, 2]² × [0, 4]` with a bore of radius 1 about the z axis
(an extruded square with a circular hole), less the V-trench prism over
the triangle `(1, 0)`, `(1.6, ±0.3)` from `z = 2` up out of the top: the
trench's keel lies on the bore's ruling through `(1, 0)`, its walls
leaving the bore outward. About the keel the material is two wedges,
bore | wall a and wall b | bore, so by tier 3 (`crates/topo/README.md`,
"no face passes through an edge's material wedge") the bore's face
should carry the keel as a slit of two coincident edges, as the planar
floor does (`crates/topo/tests/a_keel_on_the_floor_subtracts_as_a_slit.rs`).

`subtract` builds instead one keel edge, wall a | wall b, with no bore
face at the tip vertex `(1, 0, 2)`: the bore's two cylinder faces run
through the keel's wedge untouched. The volume is the closed form
(51.0736…), and `validate_geometric` and `validate_pseudomanifold`
(no records) both pass it. The same body comes back from
`(block ∖ trench) ∖ cylinder`, `(block ∖ trench) ∩ bored box` and
`bored box ∩ (block ∖ trench)`, and from a half-bore in the block's
side less the same trench.

A slit-form body of the same shape does build, as the union of the
block's two sides of the trench
(`crates/sweep/tests/a_slit_tip_on_a_curved_face_reads_its_full_turn.rs`,
`slit_block`), so the form is representable; the subtract does not reach
it, and the tiers do not see the difference.

## Where to look

- The census's wedge check that closed
  `census-passes-a-face-inside-an-edges-material-wedge` for a plane:
  whether it reads a curved face through an edge's wedge at all.
- The Boolean's edge-on-face contact for a line lying on a cylinder's
  ruling (the keel against the bore), which the planar floor reads as an
  edge in the face and cuts into the slit form.
