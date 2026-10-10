---
id: an-ellipse-trimmed-ring-on-a-cone-wall-has-no-volume-lane
kind: issue
title: A cone wall carrying a ring trimmed by ellipse arcs (T1's lune) has no volume lane: RingOnCurvedFace
status: open
opened: 2026-10-09
refs: [an-ellipse-trimmed-ring-on-a-cylinder-wall-has-no-volume-lane]
---


## What

The cone twin of `an-ellipse-trimmed-ring-on-a-cylinder-wall-has-no-volume-lane`
(flux). `topo::mass_properties` refuses a cone wall that carries a ring
trimmed by ellipse arcs: `RingOnCurvedFace`
(`crates/topo/src/props.rs`, the curved-face loop walk that returns
`MassPropsError::RingOnCurvedFace`).

Witness (GERM, 2026-10-09, `germ/cone-join-lane-rows-after-cert`): T1,
TANG's box turned −50° against the π/6 cone
(`crates/sweep/tests/a_ring_on_a_cone_face.rs`' `cone` and
`wedge(0.6, identity)`), run through the whole op (`topo::union`,
`subtract`, `intersect`). Cone ∪ box (both member
orders) and cone ∖ box keep the lune between the two section ellipses
as a ring on the cone face, and refuse at the result's tier 3 with
`ResultInvalid { VolumeUncomputable { source: RingOnCurvedFace } }`, at
every ε leg. The ∩ (both orders) and box ∖ cone build and measure their
closed form (overlap `0.057152625`, by quadrature of the slice area).
`crates/sweep/tests/cone_join_lane.rs`' row
`each_poses_body_is_its_closed_form_in_every_op` pins the refusal.

## Owed

A volume lane for a cone face with a ring of ellipse arcs (the chart
Green form over the ring, as the cylinder twin owes), then the three
T1 ops re-pinned to `OK SOUND` against `216 + π·1.2²·H/3 − 0.057152625`
and `π·1.2²·H/3 − 0.057152625`, `H = 1.2/tan(π/6)`.
