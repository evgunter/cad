---
id: a-face-moved-onto-a-record-reads-its-volume-at-the-records-anchor
kind: issue
title: A face moved onto a one-carrier record reads its volume term at that record's anchor, so the shift is bounded by the carriers' gap there, not over the faces
status: open
opened: 2026-11-04
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
