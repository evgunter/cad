---
id: an-r3-tube-no-rung-makes-a-graph-is-never-refined
kind: issue
title: An R3 tube that no rung makes a graph is never refined: two cylinder x sphere rings 3e-4 apart refuse TubeStraddles at eps 1e-6
status: open
opened: 2026-10-31
priority: P3
cost: M
---


Filed by SECT's `sect-onesix` lane (the PR closing
`sect/main-red-five-torus-section-rows-refuse-at-1e-6-since-components-by-foot`).

## Measured

`geom-brep` `pair_section::wiring::close_rings_carry_counted`
(`crates/geom-brep/src/pair_section.rs`): the wall of radius 1/2
against a ball at `(0.1, 0, 0.2)` of radius `0.6 + δ`, whose two rings
come within `δ` of each other at the far ruling. At ε 1e-6 and
`δ = 3e-4` the march refuses
`TubeStraddles { verdict: Zero(Classified { margin: 0.0 … }), boxes: 206 }`.
It passes at the default ε and 1e-12, and at 1e-6 for every larger δ
(3e-2 down to 1e-3).

A probe of `limb_three` (`crates/geom-brep/src/ssi/certify.rs`) on that
carrier: every rung from 1.5e-1 down to the floor 9.16e-6 reads graph
margin `0` over the same 206 boxes, so no rung is a graph. The padding
is not what breaks the graph. Some raw hull box of the chain is already
too wide for the rings' 3e-4 separation near the pinch. At `δ = 1e-3`
the same pose finds a graph at rung 4.7e-3.

`ssi::refine::refine_for_tube` refines a carrier only once it is
certified (`certified_states` first). A chain that no rung makes a
graph refuses before any refinement. The march's samples are spaced as
`ε^¼` (`the-chart-and-pair-tubes-are-held-by-their-coarsest-box`), so
they are coarser at 1e-6 than at the default. This row is the same
shape as that one, one step earlier: there the coarse box holds the
tube to a low rung, and here it leaves no rung at all.

The row is pinned red at 1e-6 (`test_utils::pinned_red::row`) and cites
this file.

## What closing it takes

Refine the samples under a chain box that is not a graph at the floor
rung (the `coarse` reading, applied to a carrier that has no certified
rung yet), or show that rings 300 ε apart are past what the march can
separate at this band and say so in the row.

- 2026-11-02 — moved from SSIMARCH to SSIARITH by `git mv` when SSIMARCH merged into SSIARITH, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and SSIMARCH had fallen far under it).

## 2026-11-11 — a second instance: the fit's ends at a closed loop's sharpest turn

Filed by SECTARMS's `sectarms/ruling-sweep-straddle-1e-6` lane, from
`work/sectarms/main-red-the-cylinder-sphere-ruling-sweep-refuses-tube-straddles-near-the-node-at-1e-6.md`.

A closed ℝ³ trace starts and ends at its seed (`ssi::march_seeds`,
`march::march_both_to`), and `ssi::finish_r3` fits it as an open spline
(`fit_branch(&points, [None, None])`), so the seam is the fit's two free
ends. Where the seam lies on a tight turn, the end spans' control hulls
are much wider than the samples' chords, and limb 3's graph margin
straddles there at every rung. Measured at ε 1e-6 on the cylinder × sphere
loop of `pair_section::one_loop_keeps_its_order_near_the_node` (`d 0.2,
R 0.699`, seeded at `(−0.478, 0.145, 0.115)`, beside the far branch
point): 435 boxes. At the floor rung (1.07e-5) boxes 1 and 2, the second
and third spans from the seam, read margin `0`. Each is 4.3e-2 by
3.3e-2 across, against chords of 1.7e-2 to 1.9e-2 between the samples it
hulls. Box 0 and box 3 read margins 8e-3 and 6e-3. The refusal is
`TubeStraddles { Zero(margin 0.0), boxes: 435 }`, and as above nothing
refines it.

That lane moved the cylinder × sphere witness off the turn
(`pair_section::ruling`, module docs), so the seam stands clear of the
branch points and the far ruling. The same loop then certifies at ε 1e-6. Doors whose seeds are
cell centres (`exhaust::seed_r3`) still put the seam wherever their first
seed lands. A fix here would cover both cases: refine under the
non-graph box, or fit a closed trace as periodic.
