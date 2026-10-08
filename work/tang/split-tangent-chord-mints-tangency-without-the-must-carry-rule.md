---
id: split-tangent-chord-mints-tangency-without-the-must-carry-rule
kind: issue
title: the split's tangent section chord mints TangentIntersection unconditionally, so an under-determined tangency refuses instead of describing conventionally
status: open
opened: 2026-10-06
priority: P2
cost: E
---


## Finding

Found by CLEAVE's smooth-arms sweep (branch `cleave/smooth-arms`,
every site that decides what description a definitely-smooth join
stores). `geom_brep::must_carry_over_edge` (`crates/geom-brep/src/dihedral.rs`)
is the one home of that decision: jet-determinate ⇒ the intrinsic
`TangentIntersection`, under-determined ⇒ conventional, in band ⇒ a
typed refusal.

`crates/topo/src/chord_join.rs`, the tangent-ruling arm of the split's
section-chord mint (the `SectionCase` arm that builds
`EdgeCurveSpec { description: TangentIntersection { s1: wall_key, s2:
plane_key, .. } }` after `split_tangent_chord_forward`), stores the
intrinsic description without asking the rule. Where the surfaces
determine the locus the two agree. Where they do not, the chord is not
described conventionally. It refuses at the certificate instead.

Measured on the rounded-shoulder split
(`crates/sweep/tests/wedge_end_doors.rs`,
`a_split_tangent_to_a_rounded_shoulder_cuts_at_a_seam`: unit-radius
quarter arc, plane `y = 1` tangent along the ruling), varying only the
extrusion depth `h` (the edge's extent), at `Tol::witness()`:

| h | outcome |
|---|---|
| ≥ 2e-4 | cuts; the seam stores `TangentIntersection` |
| 1.4e-4 … 5e-5 | `Join(Euler(Certification { Escalated { check: TangentSecondOrder, .. } }))`, the sagitta in band (9.8e-9 … 1.25e-9) |
| ≤ 3e-5 | `Join(Euler(Certification { NotSecondOrderSeparated { verdict: Zero(..) } }))` |

The in-band rows are the rule's own answer (a typed refusal), reached
through the certificate rather than the rule. The zero-side rows are
not: the rule answers `UnderDetermined` there, and its contract says
"the conventional description is the honest one". The split refuses a
thin body that the rule would describe as an image in the section
chart.

Not fixed in place. It changes which bodies the split accepts, and
this arm is TANG's ground (the tangent-contact chord). The fix routes
the mint through `must_carry_over_edge(.., carrier, s1, s2, extent,
band).description(..)` and maps `Conventional` to
`EdgeDescriptionSpec::chart(plane_key)`. The split's
`describe_section_boundary` already decides the same edge through the
rule, so after that fix the two would agree by construction.

## A second premise the arm assumes (CLEAVE review of PR 4157, NOTE-2)

The cylinder's `PlaneCylinderSection::TangentLine` is not the only way
into the `SectionCase::Tangent` arm. A cone's
`PlaneConeSection::ApexTangentLine` enters it too (`chord_join.rs`,
the plane–cone section arm). For a plane tangent to a cone along a
ruling through the apex, `must_carry_over_edge` answers
`UnderDetermined`: the carrier is outside `tangent_certificate_lane`,
so the rule demands no intrinsic description. Certification of the
`TangentIntersection` the arm mints then refuses with
`CertifyError::TangentCertificateUnsupported`. So the cone case is a
second refusal that the rule would describe conventionally, with a
different cause from the thin-cylinder zero side above. The fix above
covers both.
