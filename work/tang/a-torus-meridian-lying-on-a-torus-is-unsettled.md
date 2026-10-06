---
id: a-torus-meridian-lying-on-a-torus-is-unsettled
kind: issue
title: A torus meridian lying on a torus is the F≡0 case the circle×torus root door answers Unsettled
status: open
opened: 2026-10-02
---


## What

The lily's stem chain,
declared `Seam` on its walls and `Rest` on its junction discs, refuses
`CurvedPierceUnsupported` in the B ∪ A order
(`crates/sweep/tests/mate7a_torus_rest.rs`,
`the_g1_tube_chain_declared_a_seam_stops_at_the_crossing_layer`). The
refusal is on B's rim semicircle, a MERIDIAN circle (radius 0.06), lying
on A's torus wall exactly. Both of its parents (B's torus wall and B's
planar disc) are decided distinct from A's carrier.

## Why

`reduce::curved_face_arm`'s `(Zero, Zero)` arm reaches `lying_on` only
on `SpanVerdict::LiesOn`. The circle × torus root door answers
`LiesOn` for a coaxial circle (a rim or latitude) only. A meridian has
`F ≡ 0` too, but its plane holds the torus axis, and the door answers
that case `Unsettled` (the arm's own comment says no anchor can put it
definitely off). A meridian is closed-form: its plane contains the
axis, its centre is on the centre circle, and its radius is the minor
radius. So a `LiesOn` rung for it is decidable.

## Not the same cell as `germ_circle_torus`'s seam rows

`crates/sweep/tests/germ_circle_torus.rs` pins the lily's EQUATOR seams:
coaxial circles crossing the partner torus's carrier, decided by the
certified roots
(`the_lily_seams_cross_each_others_carriers_only_outside_the_windows`).
This row is about a MERIDIAN lying on the carrier identically, which is
the `F ≡ 0` branch those roots never reach.

The lily's stem: `torus-declared-rest-lane-banked` item 3. The sibling
piece is `a-torus-seam-graze-needs-the-rim-root-deflated`.
