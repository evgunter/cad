---
id: a-tied-bounds-first-order-residue-is-not-read-beside-its-zero-band-parting
kind: issue
title: A tie whose parting reads within the zero band rides with its first-order residue unread beside it
status: open
opened: 2026-11-02
priority: P3
cost: M
---

Raised by ONEREAD (`work/oneread/the-line-cone-reads-a-flanks-side-beside-the-one-bound-side-reading.md`
`## Decided`, item 2); filed by `a-leaves-verdict-carries-no-witness-point`'s
lane (PR 6385).

`bound_side::Tie::descend` decides a band tie's parting
(`Departure::Parted`, `(κ_b·n̂ − II_F(d̂, d̂))·l²/2`) alone: the tie's
first-order residue `d̂·n̂·l`, decided Zero and so at most the zero band
at the arm, is not part of it. A slack's tie already reads both
(`Departure::Jet`). For a band tie the residue and the parting are
summed nowhere, so a parting just inside the zero band beside a residue
just inside it reads `Rides` for a bound whose point at the arm may
stand up to twice the zero band off.

Where the witness reads (a line or conic edge against a sphere or a
cylinder, PR 6385), a parting off the zero band is settled by the
edge's own point, which holds every order, the residue included. The
residue is still unread at the `Rides` rung, and everywhere for a
bound no witness reads (a bisector, a face that is neither a sphere
nor a cylinder).

Owed: one rule for the tie and the residue, on the tie's `Reading`.
Either a band tie descends by `Departure::Jet` as a slack's does, or the
residue is charged into the parting's band. Measure the move first.
A plain witness at the `Rides` rung is not the remedy for a bisector:
PR 6385's review measured that `a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order`'s
bisector lies in its plane exactly (its normal section is in the plane),
so `Rides` is right there. An earlier version of this row read
3.3e-6 / 1.75e-5 off the plane at its arm, but that came from a
nearest-point foot, not from a point of the bisector's normal section.
