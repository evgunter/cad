---
id: dihedral-arm-lever-says-longer-for-an-arm-built-from-the-extent
kind: issue
title: The dihedral arm's lever says 'clearly longer' for an arm metered at the edge's extent
status: open
opened: 2026-10-03
priority: P4
cost: E
---


`DIHEDRAL_ARM`'s lever (`crates/geom-brep/src/dihedral.rs`, the
`DIHEDRAL_ARM` const) reads "move the geometry so that edge is clearly
longer", and its size reads "length". The arm it levers is
`folded_lever_arm`, the shorter of the edge's extent
(`geom_brep::edge_extent`) and its faces' radii of curvature. The extent is
a lower bound on the edge's diameter, not its length, so for a closed
circle "longer" can be false: a circle of diameter 5e-9 is about 1.6e-8
long, past K·ε, while its extent is not. Lengthening the edge is then not
the move that passes the decision. Spanning more is.

PR 3992 corrected the same wording in the boolean gate's `NEIGHBOUR_LEVER`,
`CORNER_EDGES` and `CORNER_LEVER` (`crates/topo/src/boolean/refusal_routes.rs`),
whose arms are also extents. It left this lever alone, as outside that
unit's fence.

Its text pins: `crates/geom-brep/src/certify.rs` (the "Recourse: move the
geometry so that edge is clearly longer" assertion),
`crates/topo/src/validate.rs` (three sites) and
`crates/topo/src/boolean/refusal_routes.rs` (one). A re-wording has to keep
`every_escalation_renders_within_the_viewers_word_budget` (< 75 words).
The size word "length" is the same question. So is the sector rungs'
"this length of the corner's shorter edge" (`crates/topo/src/boolean/recl.rs`
and `sectors.rs` pins), whose arm is also an extent.

**Done when:** each lever over an extent-built arm says what moves the
extent, and its size names the span rather than the length.

## Re-homed at TOPO's cut (2026-10-11)

Moved from TOPO to TOPOTAIL when TOPO dropped to P2 and split on its priority seam (`work/topotail/log.md`). TOPO kept the Euler surgery doors; this row is a torn-record, refusal-recourse or code-hygiene row, or a parked boolean or placement row.

## The edge extent's new home (OFFSET, 2026-10-27)

`geom_brep::edge_extent` no longer exists (OFFSET, PR 5875). Its home is `geom_brep::extent`, beside the reach family, as three functions: `edge_extent_inf` (≤ the edge's diameter `D`: the chord, the minor-axis folds and the sampled pairs; the lever of a reading whose Zero only refuses), `edge_extent_sup` (≥ `D`; a reading whose Zero is acted on and whose definite signs only refuse) and `edge_reach_from` (the edge's farthest distance from a point of it; a reading that acts on every sign, read from that point). This row's citation is to the function now named `edge_extent_inf`.
