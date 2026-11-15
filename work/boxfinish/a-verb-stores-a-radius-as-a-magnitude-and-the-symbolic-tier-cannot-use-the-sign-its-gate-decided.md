---
id: a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided
kind: issue
title: A verb stores a radius as its magnitude, and the symbolic tier cannot use the sign the verb's gate decided, so an arc read through its recorded radial misses its own description by |r| − r over a box
status: open
opened: 2026-11-15
priority: P0
cost: H
design: true
---


Split from `the-sweep-doors-read-the-profiles-recorded-directions`
(2026-11-15), which keeps the line half and the validated type. This
row is the arc half: it holds the tour's plate with its holes
subtracted, which refuses on main at its first `Subtract`.

**Measured** by the doors' lane (`boxfinish/doors-read`, head
`fc11523f`; the arc half as built is kept on
`boxfinish/doors-read-arcs`), on the tour's plate over its study box,
`Sym<Interval>`, main's switch.

- **Base** (the frames and the proven-zero mint, `1705653d`): the cut
  plate's first refusal is at the first `Subtract`, an unfinished
  operand: `SurfaceDatumUndecided { Cylinder, URef, Length, Upper,
  [-2.97e-4, 2.41e-4] }`. The door's `u_ref` is the start radial over
  its own length, `(a − c)/‖a − c‖`. For the circle verb's vertex
  `a = c + r·x` that is `r·x/|r|`, which the tier does not fold to
  `x`: over a box on `r` it is as wide as `r`'s box relative to `r`.
- **With the door reading the recorded radial `x`:** the `u_ref` is
  exact, and the hole's extrude then refuses at its own door, one
  check after another as each is met with a registration:
  1. `carrier_endpoint_start [0, 1.6e-4]`;
  2. with the placed start registered, `carrier_endpoint_end
     [0, 1.6e-4]`;
  3. with the turned-radial facts, `carrier_matches_mapped_source
     [0, 1.78e-4]`: the edge's description is anchored at the vertex
     (`a + rot(a − c)`), the carrier at `c + |r|·rot(x)`, so every
     interior sample reads `|r| − r`;
  4. with `|r| ≡ r` registered in the verb (a local probe), a
     `pcurve_map_residual [0, 1.6e-4]` on the same extrude, not traced.
- **A regression that reading the radial brings:** the uncut plate
  over the study box passes every geometry check on the base and
  refuses at the hole's extrude with the radial read, so the tour's
  first stop certifies nothing. At 1e-9 of the study both certify:
  `|r| − r`'s width is below the band.

**Where it starts.** `circle_loop` (`crates/profile/src/path.rs`)
gates the authored radius positive and stores `radius.abs()`, saying
why: the magnitude is non-negative in the symbolic tier's own syntax,
which lets the tier fold a rim read through a square root
(`‖a − c‖ = √(r²) = |r|`) onto the stored radius. So every reader that
divides by the rim folds, and every reader that multiplies the stored
radius by a recorded direction meets `|r|·x` where the vertex is
`r·x`. The profile already registers one consequence
(`Arc2::register_radials`: centre plus radius along the radial is the
vertex, "wherever the door decided `r` positive"). The same magnitude
is stored by `Arc2::from_chord_radials` (`signed_radius.abs()`), the
fillet (`carrier.radius.abs()`) and two `rho.abs()` sites.

What the kernel lacks is any way for a sign a gate decided to reach
the symbolic tier: `|r|` and `r` are two spellings of one real on
every valuation the verb accepts, and the tier treats them as two
functions.

**One implementer's sketch of answers** (a data point, not a proposal):
keep the quotient for arcs (the cut plate stays refused); store `r`
where the gate decided it positive, or register `|r| ≡ r` there; have
the edge's description read the radial.

**The tests that say it is closed.**
- The tour's plate with its holes subtracted, over the tour's study
  box on main's switch, no longer refuses at its first `Subtract` on a
  `u_ref` or on anything that is `|r| − r` in another form. What it
  does next is measured and, if something else holds it, filed.
- The uncut plate over the study box keeps certifying.
- An arc's stored `u_ref` and its rim circles' are exact over a box on
  the radius, for each verb that makes an arc.
- The row red on a wrong side through the curved edge arm (carried
  from PR 6979's review through the doors' row), if no curved body
  finishes over a box before this row's change.
