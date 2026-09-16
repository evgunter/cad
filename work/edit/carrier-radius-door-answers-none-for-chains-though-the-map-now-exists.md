---
id: carrier-radius-door-answers-none-for-chains-though-the-map-now-exists
kind: issue
title: LoopProgram::carrier_radius answers None for chain loops though the step-to-profile-edge map now exists: the widening waits on the content-key attach
status: open
opened: 2026-09-16
---

Disclosed by the DM8 unit
(`authored-step-to-canonical-segment-map-has-no-home`, PR 2759) and
filed at the disclosure rather than left in the PR body.

## The finding

`LoopProgram::carrier_radius` (`crates/editor-core/src/program.rs`)
answers `Some(radius)` for the complete-loop carrier forms and `None`
for every CHAIN loop. Until DM8 its doc gave the missing step→segment
map as the reason: a chain's arc steps carry their own radii
(`StepArg::CarrierRadius` and the arrival spec's twin), each addressing
one segment, and pairing those with the walls they swept needed a map
nobody had. That map now exists —
`ProfileProgram::profile_edges_of` — so the stated reason is gone
and the `None` stands on the OTHER reason the same doc gives.

## Why it was not widened with the map

The memo's guard on this channel is scoped at the ATTACH, not at the
key. `eval::content_key` writes a carrier radius's spelling whenever
any migrated verb declares a profile edge's radius into a field
(`param_source::operand_flow_bearing`), which is already true, so the
key cannot notice that chain radii are UN-attached. Widen the door to
answer per segment and the stale-token class reopens silently for
exactly those loops: a chain radius re-spelled value-preservingly
would be attached to a wall while its key still says the value alone.

So the two changes are not separable in the safe direction — chain
radii enter the content key in the SAME change that attaches them —
and only the second is safe on its own. That is the obligation the
door's doc states and this row schedules.

## What a taker does

1. Widen the door (or add its per-segment sibling) to answer a chain
   loop's per-step radii, addressed through
   `ProfileProgram::profile_edges_of`.
2. In the same change, carry the chain radii into `eval::content_key`,
   with the feed's sentence updated to match.
3. A row that re-spells a chain radius value-preservingly and asserts
   the content key MOVES — the stale-token shape the attach guard
   exists for.

Citations accurate at PR 2759's head; the stable halves are
`LoopProgram::carrier_radius`, `ProfileProgram::profile_edges_of`,
`eval::content_key` and `param_source::operand_flow_bearing`.
