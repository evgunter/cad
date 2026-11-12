---
id: a-face-moved-onto-a-record-reads-its-volume-at-the-records-anchor
kind: issue
title: A face moved onto a one-carrier record reads its volume term at that record's anchor, so the shift is bounded by the carriers' gap there, not over the faces
status: closed
opened: 2026-11-04
closed: 2026-11-10
---



Found deriving the far-origin union row's volume shift
(`work/onedoor/a-far-origin-rods-caps-moved-onto-the-plates-record-move-the-union-volume-by-four-thirds-eps.md`,
closed as correct).

`Body::unify_surfaces` moves a face onto the kept record and leaves its
boundary where it was (`Vouch::OneCarrier`). A plane face's volume term
is `(1/6) a · ∮ p × dp`, with `a` the record's anchor (its origin). So
the term moves by `(1/3) · A · h(a)`, where `h(a)` is the distance
between the two carriers measured at the kept record's anchor.

The coincidence is decided over the two faces' extent
(`carrier_pair::pair_extent`), not at the anchor. A record's origin is
whatever its constructor wrote, and can stand far outside that extent.
Two planes leaning `λ` apart are in band over the faces wherever
`λ · reach` is in band. At an anchor a distance `L` away, though, they
stand `λ · L` apart. The volume would then move by `A · λ · L / 3`,
which is not bounded by the band times the area.

The far-origin row does not reach this: its anchors sit at the rod's cap
centre and at the plate's origin, both inside the extent.

Wanted: a witness, meaning a glued plane pair whose kept record's origin
stands far outside both faces. Read its volume against the closed form.
If the volume moves past the band times the area, the fix is either to
re-anchor the kept record inside the extent the verdict was decided on,
or to restate the moved face's boundary onto the kept record. Curved
records have the same shape (per-surface anchors) and want the same
reading.

## Fixed (2026-11-10)

A face's measure is read at the face (`crates/geom-brep/README.md`
`[geom-brep.face-anchored]`): the plane term is its boundary's flux
about a point of the face (`geom_brep::props::planar_face`), a
cylinder's origin is re-anchored at the face before its closed form
(`geom_brep::surface_anchored_at`, through `topo`'s
`props::face_anchored`), and `props::rounding_bound` levers from the
same anchor. No record's origin enters a face's term, so a face moved
onto a record anchored far off reads its own fan. Witnesses, both
operand orders, within ε times the moved face's area of the operands'
sum: `crates/topo/tests/a_verdict_moves_only_the_faces_it_read.rs`
(R1's lean kiss, R2's turned kiss at 0.1, 0.3 and 0.6 ε, two flush
blocks, the slotted plate). The guard is `topo`'s
`props::face_list_door_tests::sliding_every_record_along_its_symmetry_moves_no_reading`.
