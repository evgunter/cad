---
id: arc-arc-secant-arm-misses-crossings-at-unequal-radii
kind: issue
title: At f64, arc_arc's secant arm misplaces the crossings of two circles whose radii differ by orders of magnitude, so arcs that cross read no contact and a self-crossing profile validates
status: open
opened: 2026-10-12
priority: P0
---



Found by both reviews of #4290 (the shallow arc × arc corner P0), and
identical on `main` at `8192224a78`. Not that PR's change: it does not
touch the secant arm, and its fix-pass fuzz reads the same misses with
its settle switched off.

## The defect

`crates/profile/src/seg.rs` `arc_arc`, the `carrier_circles_internal`
Positive arm ("Proper secant"), places the two crossings by the
radical-line closed form, `a = (d² + r₁² − r₂²)/2d` from `c₁` and the
half-chord `h = sqrt(r₁² − a²)`. Both are differences of squares. When
the radii differ by orders of magnitude, `a` is within about `r₂` of
`r₁` and `r₁² − a²` is a difference of two numbers near `r₁²` whose
true value is about `h²`, so at `f64` it loses about `u·r₁²` (u the
unit roundoff) against `h² ≲ r₂²`. The crossings land off both
carriers and off the arcs, both spans read them as definite misses,
and the ends, none of which stands near the true crossing, read clear.
The pair reads no contact, and a profile whose arcs cross validates.
`Interval` encloses the cancellation and reads the crossing or
escalates.

## Reproductions

- Re-measured at ε = 1e-9 (f64, `pair_contacts`, and a 50-digit check
  of the crossing): `Arc { c: (-106.67744886718727, 59.13052839188616),
  r: 62.755865313469435, start: 0.9863779592320079, sweep:
  4.084286531899824 }` × `Arc { c: (-86.95998719693317,
  -0.44732256827762046), r: 1.2844201989327708e-7, start:
  3.0227778671179593, sweep: -3.85588577470399 }` read no contact.
  The circles cross 2.43 inside the first arc and 4.49e-7 / 4.6e-8
  (46ε) inside the second's two ends: distance 0.
- The fix-pass fuzz family `seg_reach_fuzz::a_crossing_near_its_rival_reads_no_contact_only_past_eps`
  with its radius-ratio cap lifted (seed `0x1`, effort 4) finds 40
  such pairs at 1e-9 and 47 at 1e-12, every one `f64` arc × arc, radius
  ratio 9.4e4 to 4.9e11. None at 1e-6. With the cap at 10⁴, seeds
  `0x4` and `0x5` (effort 10, 1e-12) still find one each, at ratios
  7.8e3 and 2.5e3: `Arc { c: (6.744101644309793, 79.7053630720293), r:
  80.4431499885935, start: 5.633354791777812, sweep:
  -0.9028462827882873 }` × `Arc { c: (8.18313046102052,
  -0.7503314934178684), r: 0.03169669054487094, start:
  2.2749547818688507, sweep: 4.970712391069452 }` crosses 2.0e-11
  (20ε) inside the first arc's end and 0.157 inside the second, and
  reads no contact on main and on #4290. The family caps the ratio at
  10² until this lands; lifting the cap is this item's regression row.
- One review: 57 of 387 flags at 1e-9 confirmed at distance 0, ratio
  about 1e5 to 1e8 (e.g. r = 2151 against r = 4.7e-6, crossing 2000ε
  inside both).
- The other review: at 1e-12, r ≈ 90 and 139 with centres about 100
  from the origin, crossing 12–24ε from the ends, read no contact at
  `f64`. Not re-measured here; that ratio is about 1.5, so if it
  reproduces the class is wider than unequal radii (the same
  cancellation at large coordinates).

## Where to look

A factored form, `r₁ − a = (r₁ + r₂ − d)(r₁ − r₂ + d)/2d` and
`h² = (r₁ − a)(r₁ + a)` with `r₁ + a = (d + r₁ − r₂)(d + r₁ + r₂)/2d`,
taken from the smaller circle's centre, keeps the cancellation in `d`
(the oracle in `crates/profile/src/seg_reach_fuzz.rs` computes it so).
The `internal`/`external` tangent arms place their candidate with the
same `a` (`an-internal-tangency-reads-its-spans-at-the-radical-foot-off-both-arcs`).
