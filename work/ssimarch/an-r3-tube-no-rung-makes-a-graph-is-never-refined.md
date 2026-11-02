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

`geom-brep` `pair_section::wiring::close_rings_carry_counted_and_marched`
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
