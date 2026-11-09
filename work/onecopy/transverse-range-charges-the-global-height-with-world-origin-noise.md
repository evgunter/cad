---
id: transverse-range-charges-the-global-height-with-world-origin-noise
kind: issue
title: conic_conic's world-origin noise is charged into the PARALLEL screen, coplanar_circles and transverse's Range
status: closed
opened: 2026-11-09
priority: P2
cost: M
closed: 2026-11-09
pr: 6638
---


Filed by `onecopy/gate-blind-spots`, from `a-rounding-charge-through-rounding-charge-escapes-the-bare-epsilon-gate` (whose third site this is).

## What

`geom-brep` `curve_pair::pose` `conic_conic` mints a single charge:

    noise = rounding_charge(|o_a| + |o_b| + 2·lever)

Its term grows with the conics' distance from the world origin, not with the magnitudes the readings actually sum. The heights read from it are local: `height` is `plane_offset` of the two centres, and the swing is `|(cp, sp)|`. At coordinates of about 180 the charge reaches about 7.5e-13, the same scale that refused `sweep`'s leaning prisms at ε = 1e-12 before PR 6525 re-posed `Window::read`.

Three decisions consume it:
- the PARALLEL height screen, `constant(row::PARALLEL, c0, swing + noise, band)`;
- `coplanar_circles`, `(noise, slant)`;
- `transverse`'s `Range` (`Range::first_harmonic(.., noise)`), through:
  - `band_arc`'s `e = escalate + noise`;
  - `harmonic_roots`' `extreme_sign(row::EXTREME, r.lo | r.hi, r.noise, band)`;
  - `least`, the rule that roots the noise alone puts apart read as a touch: `cross · past_noise(size, noise) / size.max(noise)`.

The bare-rounding gate counts the mint as one site in `pose.rs`'s row. It cannot see the three consumers, because a charge carried in a variable is a stated gap of the gate.

**At noise = 0, `least` is `0/0`** when an extreme is exactly zero (`size = 0`), and it yields NaN.

## Proposed fix

Delete the mint. All three readings then take the heights as exact, per `[simple.conditioned]`. Guard `least` so that an extreme in the zero band reads its touch at the extreme itself, with no NaN path. This changes what the zero-band touch arm decides. `meet_rows::coplanar_conic_rims_tangent_at_a_vertex_meet_once_there` refused at ε = 1e-9 on an exact jet when PR 6541 moved `coplanar_ellipses` the same way (see the closed `coplanar-ellipses-reads-the-level-from-its-global-harmonics`). So the lane runs `geom-brep`'s `curve_pair` meet rows and `sweep`'s prism rows at both ε, and treats a touch-arm refusal as the evidence that decides the guard. Once all three consumers are gone, lower `pose.rs`'s row in `scripts/gates/bare-rounding-sites.tsv`.

## Evidence elsewhere

`meet-refuses-a-near-tangent-tilt-in-both-orders`: at a shared end tilted ±1e-5 about the cross, `transverse`'s extreme escalates at −5.0e-12 through this charge.

## Resolved (PR 6638)

- **The charge.** Deleted. The PARALLEL screen, `coplanar_circles` and `transverse`'s `Range` read their local differences uncharged.
- **Measured at the needle's shared end turned ±1e-5 about the cross:** the charge was 1.2e-14 there, against a true extreme of −5.0e-12. That pose's escalation was not the charge's; PR 6623 settled it. Far from the origin the charge was what refused:
  - the far vertex turned 3.16e-5 about its tangent refuses at 1e2 and at 1e3, while it reads apart at the origin;
  - at 1e3, 8 of 60 grid poses read other than at the origin with the charge, and 1 without it.
  - `order_rows::the_needle_far_from_the_origin_reads_as_at_it` pins this.
- **`least`.** Deleted, by the orchestrator's ruling (T): an extreme in the zero band reads its touch at the extreme (C5). The root gap decides only between two definite crossings, so there is no 0/0 path left.
- **The line router's two mints** are not this row's. They are filed as `the-line-router-charges-its-margins-with-world-origin-noise`.
- `pose.rs`'s row in `scripts/gates/bare-rounding-sites.tsv` drops from 8 to 7.
