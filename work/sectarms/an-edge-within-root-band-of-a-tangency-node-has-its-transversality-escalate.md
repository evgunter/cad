---
id: an-edge-within-root-band-of-a-tangency-node-has-its-transversality-escalate
kind: issue
title: An edge within √band of a tangency node is described by two nearly tangent walls, so its transversality escalates
status: open
opened: 2026-11-03
priority: P2
cost: M
---


Filed by the cylinder × cylinder stage 3b-ii lane (branch
`sectarms/cylinder-node-listing`), on the orchestrator's ruling.

## What

Stage 3b-ii retired the bisector lane: a chord of the bisector pose now
rides the carried lane, and the closed-form arm of
`chord_join::carried_chord_spec` describes it as the face's wall against
the partner wall. The retired lane described it against the ellipse's
bisector plane (`AuxDatum::Bisector`, an auxiliary plane surface), which
is transverse to the wall everywhere.

The two walls are tangent at a node, and their dihedral grows like the
distance from it. A chord from a pierce to the node, where an operand
edge crosses the partner wall `δ` off the node, is `2δ` long at a
dihedral of about `δ`. Its edge certification reads the crossing angle,
which is about `0.28·δ²` for A's seam and `0.84·δ²` for B's (unit rods at
right angles). Below the escalation edge it refuses typed:
`Euler(Certification { NotTransverse })` where the angle reads `Zero`,
and `Escalated { check: Transversality }` in the band.

**These poses built under the retired bisector lane. This is a
capability regression**, the one outcome stage 3b-ii moved toward
refusing.

## The rows and their measurements (ε 1e-9, band `[1e-9, 1e-8]`)

`crates/sweep/tests/bisector_nodes.rs`:
- `the_near_node_poses_at_the_witness_band`: each seam at
  - `δ = 1e-6`: `NotTransverse`, margin `2.8e-13` (A's seam), `2.0e-12`
    (B's);
  - `δ = 1e-4`: `Transversality` escalated, margin `2.8e-9` (A's),
    `8.4e-9` (B's).

  Every op, both orders, both poses. The caps still build.
- `an_edge_near_a_node_crosses_through_the_bisector_planes`: each seam
  at `δ = 30·escalate` (`3e-7`): `NotTransverse`, margin `2.5e-14` (A's)
  and `1.8e-13` (B's). At `5·√escalate` everything builds.
- `an_oblique_seam_near_a_node_builds`: at `δ = √escalate` (`1e-4`),
  `Transversality` escalated, margin `4.7e-9` at 35°, `r = 1`, and
  `1.65e-9` at 60°, `r = 0.5`. At `3·√escalate` and `5·√escalate`
  everything builds.

**Measured reach.** The margin is quadratic in `δ`, so a seam builds from
about `δ ≈ √(escalate / 0.28)`. That is about `2e-4` at ε 1e-9 for A's
seam on unit rods, and about `1.1e-4` for B's. The smallest `δ` that
builds in the census is `3·√escalate = 3e-4` (both oblique poses).
`5·√escalate = 5e-4` builds in every family. The band-relative rows
scale with ε.

## Open design question (for a future fork; not recommended here)

May a planar-conic carried chord be described against its own plane,
where that plane cuts the face's surface in exactly that conic, rather
than against the partner surface? It would restore these poses. It also
re-introduces an auxiliary surface of the kind the retired lane carried,
and changes the closed-form arm's description for every planar conic
(stage 3b-i's coaxial circles included).

## Done when

The question is decided, and either the description changes and these
cells build again, or the refusals are accepted as the reach of the
two-wall description and this row closes on that ruling.
