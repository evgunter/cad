---
id: a-sqrt-band-refused-ring-around-every-pinch-node
kind: issue
title: A √band refused ring around every pinch node: the pierce vertex reads a family pierce through the pierced wall's tangent plane
status: open
priority: P2
cost: H
opened: 2026-10-15
refs: [pinch-carrying-machinery-valence-4, 5192, 5193]
---


For two equal-radius cylinders whose axes meet (the U1 pinch family of
`pinch-carrying-machinery-valence-4`), an operand edge that lies on one
wall and passes within about √band of a node refuses, although U1 places
the node definitely inside the face. On main, at the witness band (1e-9),
with two extruded rods crossing at 90°:

| Pose | Refused | Builds |
|---|---|---|
| Seam turned φ off the node | φ = 0 … 1e-6 (`CurvedPierceUnsupported`); escalates at 1e-4 | from φ = 3e-4 |
| Branch cap dz from the axes' meeting point | dz = 0 … 1e-4 | from dz = 3e-4 |

Cause: the crossing sweep reads such an edge against the partner wall
through the full quadric. Near a node the edge is nearly tangent to the
partner wall, so its two real crossings, δ apart, show a discriminant
margin of order δ², and `line_wall_roots` reads Zero as
`WallRoots::Tangent` (`crates/topo/src/boolean/solid_contain.rs:4491`,
reached from `crates/topo/src/boolean/reduce.rs:3598` and `:3641`). Node
placement decides at δ and the sweep at δ², so between band and √band
the two disagree. U1's rows avoid the ring by posing their seams at
φ = 0.4.

The fix the U2 design fork converged on is to read the partner wall
through the two bisector planes: on wall A, f_B = (w·(â₁−â₂))(w·(â₁+â₂))
with w = x − p, so each crossing is a first-order line×plane or
conic×plane root, δ/sin(θ/2) or δ/cos(θ/2) from the node. Closure is
that design's U2a acceptance rows (seam and cap offsets of 1e-8, 1e-6
and 1e-4 build at the closed forms; band offsets escalate). Found by the
fork's designer probes (a temporary test in
`crates/sweep/tests/bisector_nodes.rs`, reverted).

## The crossing half, closed by U2a (`tang/pinch-u2a-factored-crossing`)

The crossing sweep reads such an edge through the bisector planes
(`crates/topo/src/boolean/factored_crossing.rs`). The two crossings now
stand `2δ` apart, a first-order margin, so the sweep's ring is one
band wide. In band their distance escalates `VertexOnVertex`; in the
zero band the edge keeps the quadric's door.

## The pierce vertex, at second order (closed by U2p, PR 5203)

Before U2p, past the sweep the ring kept its √band width, one layer in. At a
pierce of wall B by an edge on wall A, the walls' normals stand about
`δ` apart, and the pierce's sector-side reading
(`crates/topo/src/boolean/sectors.rs`, `read_side`'s
`bool_pierce_sector_side_curved`) charges a sagitta of
`slope²·R/4 ≈ δ²/4`. Probed on the same rods, both seam poses and both
cap poses:

| ε | refuses `CurvedSectorSideUnsupported` | escalates `PierceCurvature` | builds |
|---|---|---|---|
| 1e-9 (witness) | δ = 1e-8, 1e-6 | δ = 1e-4 | δ = 3e-4 |
| 1e-6 | δ = 1e-4, 3e-4 | | δ = 3e-2 |
| 1e-12 | δ = 1e-8, 1e-6 | | δ = 1e-4 |

`bisector_nodes::an_edge_near_a_node_crosses_through_the_bisector_planes`
pins that sequence relative to the band.

**One step further, measured, not merged** (branch
`tang/u2a-pierce-plane-reads-experiment`, 4d6290606). An experiment read a
bound lying on the piercing wall against the bisector plane the pierce
lies on, signed to the pierced face's outward side: `sign(dⱼ(p)) ·
sign(departure·n̂ᵢ)`, from `f_B|A = c·d₀·d₁`. It also took the wall
sector's germ as `n̂_wall × n̂ᵢ`, the ellipse that sector rides, in
place of `vtxfac::pierce_germ_dir`'s `n̂_s × n̂_p`, which is `δ` long.
At the witness band:

- δ = 1e-6, both seam poses: ∪ and ∩ build at their closed forms.
  A ∖ B and B ∖ A fail tier 3 at rest, `ResultInvalid [LaminaWedge …]`,
  on the arcs from the pierce to the node: about `2δ` long, with a
  dihedral of about `δ`.
- δ = 1e-8 seams: `Containment { decision: None, escalation: Margin }`
  escalates (margin 1e-8).
- caps at 1e-8 and 1e-6: `ClassificationInvariant "a sector bound read
  On departs definitely from the plane"`, from the lump path (A's cap).
  B's cap still reads the sagitta on a bound the experiment did not
  route.
- δ = 1e-4: `SeamWedge` escalates (margin about `δ²`); the caps at
  1e-4 still escalate `PierceCurvature`.

The experiment mapped bound `k` to sectors `k` and `k + 1`; entry `k`
is sector `k`'s end and sector `k − 1`'s start, and that misread is
what left the caps on the sagitta.

**U2p (PR 5203)** carries the experiment as the decision below states
it. The sweep records each pierce's plane (`ContactAcc::branches`), and
the pierce reads its wall bounds and germ through that record, with the
off-by-one fixed. At the witness band, on main with U2p:

- every cap pose at 1e-6 and 1e-4, and both caps, builds at its closed
  form;
- every seam pose at 1e-6 and 1e-4 builds every op at its closed form,
  once U-lever (PR 5204) levers the short arcs' wedge at their faces'
  reach; before it the seam's ∖ at 1e-6 read `LaminaWedge` and the seam
  at 1e-4 `SeamWedge`;
- 1e-8 is the witness band's escalation edge: poses there escalate in
  band (`Containment`, or the caps' `Coincidence(Sectors | SectorSide)`),
  and a single cap builds in some poses;
- at ε 1e-12 the near-node caps refuse at a short ruling's pcurve
  (`work/pcert/a-short-ruling-on-a-cylinder-chart-mints-its-pcurve-at-the-seams-u.md`).

What remains open here is that last pcert row.

## The ring at oblique angles reaches further (PR 5192)

Measured at the witness band, with A's seam `δ` off a node and every
operation:

| pair | `CurvedSectorSideUnsupported` | `PierceCurvature` escalates | builds |
|---|---|---|---|
| 90°, r = 1 | | δ = 1e-4 | δ ≥ 3e-4 |
| 35°, r = 1 | δ = 1e-4 | δ = 3e-4 … 1e-3 | δ ≥ 1.5e-3 |
| 60°, r = 0.5 | δ = 1e-4 | δ = 3e-4 … 5e-4 | δ ≥ 7e-4 |

`bisector_nodes::an_oblique_seam_near_a_node_escalates_at_the_pierce`
pins the two oblique poses at `δ = 5·√band` and `4·√band`. A unit that
re-levers the pierce's side reading has to close them.

## The decision (Ev, PR 5193)

Approved on PR 5193 (fork-log row 150): the near-node bullet in
`pinch-carrying-machinery-valence-4.md` ("An edge near a node, not
through it"). The pierce reads the partner wall through the branch the
sweep decided; at rest a wedge is levered at the faces' curvature and
extent, never the edge's chord. Units: U-lever (in progress), then U2p,
whose acceptance closes this row.

The experiment branch above and its `LaminaWedge` result (arcs about
`2δ` long with a dihedral about `δ`, refused at rest) are evidence for
U-lever's acceptance.
