---
id: injectivity-refuses-at-interval-what-it-certifies-at-f64
kind: issue
title: The injectivity check refuses at Interval what it certifies at f64: SHELL-4's vessel refuses Departure on a half-rim at Body<Interval>, and no self-intersection row runs at Interval
status: dispatched
opened: 2026-10-30
priority: P1
cost: M
branch: clear/rimscalar
---


## Finding

Found by SHELL-4's lane, which stopped on it as its spec required. The
evidence is the WIP test module `shell_cavity_embedded` on
`clear/shell4` at `623daf636b`. The test: the self-intersection sweep on
the operand vessel `vessel_at(1.0, 2.0)`
(`crates/sweep/tests/common/shell_operands.rs`), at the strict bound,
over all faces, with `same_body = true`.

- `Body<f64>`: Holds (9 candidates, 50 discharged, 14 certified,
  `cornered = 0`). The carrier arm certifies the disc/cylinder half-rims,
  with the disc's plane implicit as ψ against the wall's iso rectangle.
- `Body<Interval>`: refused
  `Uncertified { a: 2v1 (disc), b: 5v1 (cylinder band), shared: Edge(9v1), arm: Departure }`,
  with `cornered = 4`. Edge 9's witness point carries ulp-wide
  enclosures (`x ∈ [−1.8e-16, 4.8e-16]`), and the corner presentation
  is taken in place of the carrier arm.

**Class.** Every row of `crates/sweep/tests/self_intersection_proves_injectivity.rs`
builds `Body<f64>`. The certificate is never exercised on a
`Body<Interval>`, which is the certifying scalar SHELL-4's gate and the
document door use.

CORNER (#6045) and its reviews measured the vessel at `f64` only. So
did SHELL-4's unpark note.

Blocks SHELL-4 (re-parked on this row). Unit RIMSCALAR,
`docs/RIMSCALAR-SPEC.md`.

## Cause (RIMSCALAR)

At `Interval` the wall's half-rim images hold their height only to
rounding (`a ∈ [−8.9e-16, 8.9e-16]`), so they are no coordinate lines,
and the wall reads each rim as a curve through its corners. That
reading needs `injective::Trig::of` to find the winding's rate exactly
`±1` (`exactly(pl, ±1)`); `clearance::harmonic_of` handed it
`angle_rate(w) = w·τ/τ`, which `Interval` encloses as
`[0.9999999999999998, 1.0000000000000004]`. So the corner presentation
was unread and departure failed. The rate is now read off the stored
winding count; the vessel holds at `Interval` by the corner route
(`cornered` 8) where `f64` holds by the carrier arm (`cornered` 0).

