---
id: the-glue-doors-tangency-swallows-its-escalations
kind: issue
title: The glue door's tangency read swallows every refusal, escalations included, so an in-band tangency surfaces later as CurvedBooleanUnsupported
status: closed
opened: 2026-10-12
priority: P3
cost: E
branch: contacthold/glue-door-escalates
pr: 4918
closed: 2026-10-13
---


## What

glue::tangency tests the witness lane with .is_ok(), discarding an escalation; the stage that meets the pair then refuses CurvedBooleanUnsupported, naming a declaration recourse D10 retires. An in-band tangency should escalate typed at the door.

Found by a designer of the overlapping-tangency fork (fork log row 118), read from the code at 63288134b8, not measured.

## Measured (PR 4831, 2026-10-12)

Still swallowed: the claim-free door treats any refusal of
`verify_tangency` or `route_rim` as no tangency. Propagating
`TangencyFailure::Refused` moved five sweep rows to door refusals
(`mate7a_torus_rest` ×2, `a_ruling_lying_on_a_wall`,
`review_contact_edge_must_carry_r1_probes`, `offer_rows::every_sphere_offer_passes_just_below_it`),
and not all were in-band tangencies: the G1 tube chain refused
`contact_tangent_on_1` at margin 1.21, the witness on the surface but its
far side resting on a loose span sag. The fix has to tell a genuine
in-band reading from an uncertifiable one before it escalates.

## C8 is unmet here (PR 4831's fix pass)

`crates/topo/README.md` C8 says a tangent pair the witness lane has a
locus for is glued and never meets `CurvedBooleanUnsupported`. A pair
whose reading the door swallows is left unglued and meets it at:

- `vtxfac::classify_vertex_on_face`, the C8 guard on an unglued curved
  on-carrier sector;
- `vtxfac::classify_vertex_on_face`, an unglued planar sector on a curved
  pierced face (the `pierced_carrier` match);
- `recl::require_same`, the unglued curved pair on either side.

The glued pairs that met it at `recl::require_same`'s tangent arm (the
stadium, the kiss with overlap, a plane covering part of a hole's ruling)
answer since `resolve_edge_sector` lumps them. Two routes by which a
pair the lane has a locus for reaches the frontier refusal remain after
this row: a rim junction the routing places outside what the zip builds
(`TangencyFailure::Unrouted`, scheduled under
`a-touch-only-rim-with-no-derivable-locus-refuses-every-op-as-a-cusp.md`),
and a pair the door glues in the zero band whose later stage then refuses
(`a-brick-on-a-tube-ruling-in-the-zero-band-glues-then-meets-the-pierce-refusal.md`).
A span the finest jet schedule cannot reach was a third; the door now
refuses it typed (`a-span-the-finest-jet-schedule-cannot-reach-leaves-a-locus-pair-unglued.md`).

## Fixed (PR 4918)

The door now propagates `TangencyFailure::Refused` and an undecidable
rim identity; counter-evidence, a pair outside the lane and a rim
junction the routing places outside what the zip builds
(`TangencyFailure::Unrouted`) stay no tangency. Of the refusals the
door used to swallow, three kinds were not about the faces' geometry
and are fixed where they start:

- `contact_verify::tangent_table` reads a span whose only unsettled
  readings are span bounds (the witness's sag, the touch's, the drift of
  `κ_rel`) again on finer schedules, up to sixty-four spans to each of
  the nine-sample schedule's, reading every sample on each; past that a
  bound beyond the band is `NotCertifiable` and one in band escalates.
  The G1 tube chain, read on every sample, escalates on its meridian's
  poles, where the two tubes osculate;
- `geom_brep::tangent_locus` answers `NotTangent` on a definite gap
  before an in-band axis tilt (the leaned ruling);
- `geom_brep::tangent_locus_of_faces` answers `NotTangent` on
  `tangent_locus_face_clear` where a face's extent across the locus
  stands definitely clear of the other carrier (a brick beside a
  tube's ruling, a cylinder wall turned off it).
- `rim_wedge::same_circle` reads circles by the identity ladder's rule
  (`carrier_eq::identity_data`, shared with the carrier ladder): a datum
  decided apart answers no, so an in-band radius no longer refuses a
  pair whose circles stand far apart (the pre-gate's frustum-and-plate
  pose).
- A span bound the finest schedule leaves beyond the band is refused at
  the door, typed (`UnsupportedDeclarationClass { Tangent }`), rather
  than read as no tangency.
