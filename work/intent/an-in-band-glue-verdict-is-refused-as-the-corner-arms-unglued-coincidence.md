---
id: an-in-band-glue-verdict-is-refused-as-the-corner-arms-unglued-coincidence
kind: issue
title: A pair the glue door reads in band is refused at the corner arm as an unglued coincidence, naming the corner instead of the in-band glue verdict
status: open
opened: 2026-10-11
priority: P2
cost: M
---


Found by stage 4 F (`declared-pairs-retire`), on the rung a declared
`Rest` used to bridge.

## What

`crates/editor-core/tests/docm2_part_interval.rs`,
`a7_the_width_ladder_of_the_split_of_a_widened_box`, at the ε/16 rung:
the corpus `part_select` document at the `Interval` scalar, with the
split body's height widened by ε/16. The union of the split's two halves
refuses
`Escalated{Coincidence(Carriers, Settles(Rest))}` with predicate
`carrier_unglued_coincidence` (`crates/topo/src/boolean/recl.rs`, the
corner arm), carrying the enclosure `[0, 0]`.

The cause is the glue door, not the corner. The glue door
(`boolean::glue::decided_declarations`) reads the cut faces' plane
offset at `[−1.5e-9, 1.5e-9]` (`bool_plane_offset`, among the
refusal's escalations), inside the sliver band (zero `1e-9`). It leaves
the pair unglued, which D10 says it must: the band refuses. The corner
then reads the pair one carrier over its own arm, finds it unglued, and
refuses with that reason. So the refusal names a Zero reading at the
corner when the deciding fact is an in-band margin over the faces'
extent: the user is told the pair is coincident and unglued, not that
its coincidence is too close to call.

## Owed

Have a pair the glue door leaves in band refuse with the glue door's
reading (`Coincide::Carriers` with the in-band margin, the geometry and
the tolerance as recourse) wherever a later arm meets it. Do not have
the corner arm report an unglued Zero for such a pair. The a7 ε/16 rung
is the witness, pinned as `Rung::Escalates { node: union, predicate:
"carrier_unglued_coincidence" }`, and that pin moves with the fix.
