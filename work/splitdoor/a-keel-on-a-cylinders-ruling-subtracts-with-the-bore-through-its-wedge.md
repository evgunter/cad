---
id: a-keel-on-a-cylinders-ruling-subtracts-with-the-bore-through-its-wedge
kind: issue
title: A keel resting on a bore's ruling subtracts as one edge between the walls, the bore's face running through its material wedge, and every tier passes it
status: closed
closed: 2026-10-30
opened: 2026-10-29
priority: P0
cost: M
refs: [census-passes-a-face-inside-an-edges-material-wedge, a-slit-tip-on-a-curved-face-or-at-a-cone-apex-reads-its-full-turn-as-a-spike, 6061]
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

## Triage (PR 6061): a live wrong answer

Measured on main `c7d569e2` at ε 1e-6, 1e-9 and 1e-12, identical at all
three.

- **Wrong.** The point set and the volume hold, and the mesh is
  closed. The body breaks tier 3's rule all the same, and `split` at
  `z = 3` turns it into a wrong boundary. Each side's cap face covers the
  trench void, cancelled by a triangle face of the opposite sense; tier 3
  passes both sides and only tier 3′ refuses them (evidence on
  `work/heldrecords/the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime.md`).
  The controls cap correctly at the same cut: (a) the trench's apex off the
  bore gives a ring; (b) the slit form built by union gives a pinched cap;
  (c) a planar wall gives a pinched cap.
- **A second witness.** `crates/sweep/tests/verbs_ga_r2_probes.rs`,
  `r2_a_box_with_on_carrier_rulings_builds_every_op_but_its_void_which_refuses_typed`: the pipe
  less a box whose four z-edges are the pipe's rulings pinned the one-edge
  form as valid at tiers 3 and 3′.
- **Causes.** `crates/topo/src/boolean/slit.rs`, `lies_in`, held plane faces
  only, so `part_riders` never read the keel against the bore. The census's
  pass 4 and its `Link`/`Sector` readers are planar-only, so an edge lying in
  a curved face of its own solid was never found.

## Closed by PR 6061

The defect, a one-edge form shipping through every validator, is closed
on two sides:

- The Boolean reads a line, no wall of it tangent to the face, lying
  on a cylinder or cone face, and refuses `EdgeInFaceGap::CurvedFace` where the
  parting is due. The slit form is buildable, but it is neither meshed
  (`work/tesslie/the-walk-panics-on-a-slit-in-a-curved-face.md`) nor
  measured as a ring
  (`work/nearpoint/a-zero-area-ring-in-a-cylinder-face-measures-degenerate.md`).
- A transverse curved wall is covered too: the trench whose closing
  wall is an arc of bulge ±0.2 (a cylinder crossing the bore along the
  keel) shipped the one-edge form through every validator after the
  first fix, and refuses now (the review of PR 6061).
- The census's curved arm (`sweep_edge_in_curved_face`) refuses the
  one-edge form at tier 3′ with `FaceThroughEdgeWedge`, whatever produced
  it.

Building the slit form instead of refusing is
`work/splitdoor/the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured.md`.
Pins: `crates/sweep/tests/a_keel_on_a_bores_ruling.rs`.
