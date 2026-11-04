---
id: an-open-tangent-chain-of-mixed-links-refuses-at-the-open-carve
kind: issue
title: An open tangent chain of mixed links refuses, because the open carve is plane–plane only
status: open
opened: 2026-11-04
priority: P0
cost: H
refs: [a-rounded-outline-is-a-closed-chain-of-mixed-links-the-blend-refuses, a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds]
---

Filed by the band-mixchain lane (2026-11-04), measured while building the closed mixed chain.

It blocks `work/band/a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds.md`: that row's chains run rim → torus → line and end at cut-offs, which is an open mixed chain. A blocker takes the band of the row that waits on it, so this row is P0.

An OPEN tangent-continuous chain whose links mix support pairs (a line into an arc into a line, ending at turns, cut-offs or corners) refuses `UnsupportedChain` at `OpenBand::admit` (`crates/sweep/src/blend/admit.rs`): "an open chain's supports are neither a plane–plane nor a ruled cylinder pair". The arc's torus arm does not get through. Its junctions get past the battery: predicate 4's band reading (`battery::band_g1`) certifies each section exactly as it does on a closed chain.

The witness is the teardrop, `crates/sweep/tests/band_planar_cut_off.rs::a_closed_rim_with_one_turn_breaks_there_and_refuses_its_mixed_chain`. Its closed rim breaks at its one turn into the open run line, arc, line, and that run refuses at the door.

What carries over from the closed mixed rim (`surgery::rim_phase_mixed`):
- the junction record: the section centre, the host foot and the mate foot;
- the junction moves: split the edge off the rim at the mate foot, strut to the host foot, the strut's `kev` re-basing the split piece into the section edge, described as a `CornerArc`;
- the `BandSection` name.

What does not:
- the open carve's whole plan is plane–plane. `blank_phase` (`crates/sweep/src/blend/open/planar.rs`) carves its supports locally with line trims.
- `RequestedBoundary::admit` admits only planar supports whose requested edges end at planned stations (corners, joints, cut-offs, turns), and a junction is none of those.
- the band face is ONE face over a run of `Joint`s on the same supports.

An open mixed chain needs that carve to take a torus link mid-band: the mate face of an arc is a cylinder wall, not a plane. It also needs a junction station on each support beside the joint, the corner, the cut-off and the turn.
