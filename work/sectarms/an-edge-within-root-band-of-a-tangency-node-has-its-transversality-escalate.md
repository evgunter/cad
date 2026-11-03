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
dihedral of about `δ`. The crossing angle its certification binds on
measures `0.2795·δ²` for either seam (unit rods at right angles). Below
the escalation edge it refuses typed:
`Euler(Certification { NotTransverse })` where the angle reads `Zero`,
and `Escalated { check: Transversality }` in the band.

**These poses built under the retired bisector lane. This is a
capability regression**, the one outcome stage 3b-ii moved toward
refusing.

## The measured threshold (ε 1e-9, band `[1e-9, 1e-8]`)

Unit rods at right angles, one seam turned `δ` off a node. Every op,
both orders, both poses: 12 cells per seam per `δ`, from a census run on
the branch. The margin is the first failing sample's.

| δ | A's seam | B's seam |
|---|---|---|
| 1.2e-4 | refuses, `Transversality` escalated, `4.02e-9` | refuses, `8.05e-9` (another chord, sample 6) |
| 1.5e-4 | refuses, `6.29e-9` | refuses, `6.29e-9` |
| 1.8e-4 | refuses, `9.06e-9` | refuses, `9.06e-9` |
| 1.9e-4 | builds | builds |
| 2e-4 | builds | builds |
| 2.2e-4 | builds | builds |

The binding margin is `0.2795·δ²` for both seams, past `1e-8` from
`δ ≈ 1.89e-4`, so **both seams build from about `2e-4`, or `2·√escalate`**.
The earlier "`0.84·δ²`, from `1.1e-4`" for B's seam read the margin of
another chord (sample 5), not the one that binds. That was corrected
on the dual review's measurement (R2).

`bisector_nodes::a_seam_builds_from_twice_root_band_off_a_node` guards
the threshold: `1.8e-4` refuses typed, and `2e-4` builds at the
Steinmetz solid's closed form, for both seams.

## The rows below the threshold

`crates/sweep/tests/bisector_nodes.rs`:
- `the_near_node_poses_at_the_witness_band`, each seam:
  - `δ = 1e-6`: `NotTransverse`, margin `2.8e-13` (A's) and `2.0e-12`
    (B's);
  - `δ = 1e-4`: `Transversality` escalated, `2.8e-9` and `8.4e-9`.
  The caps still build.
- `an_edge_near_a_node_crosses_through_the_bisector_planes`: each seam at
  `δ = 30·escalate` (`3e-7`): `NotTransverse`, `2.5e-14` and `1.8e-13`.
  At `5·√escalate` everything builds.
- `an_oblique_seam_near_a_node_builds`: at `δ = √escalate` (`1e-4`),
  `Transversality` escalated, `4.7e-9` at 35°, `r = 1`, and `1.65e-9` at
  60°, `r = 0.5`. At `3·√escalate` and `5·√escalate` everything builds at
  its closed form.

The band-relative rows scale with ε.

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
