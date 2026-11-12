---
id: a-sphere-pole-start-meridian-escalates-at-the-interval-mint-door
kind: issue
title: A sphere meridian starting near its pole escalates at the interval mint door, its radial part's enclosure wide over its length
status: open
opened: 2026-11-12
priority: P3
cost: M
---

Filed by `pcert/derived-sphere-image-accuracy` (PR 6870). This was there
before that PR, which keeps it no wider than it was.

## What

At the `Interval` mint door (`PcurveCache::<Interval>::certify`), check
4's sphere meridian incidence term (`EnvelopeTerm(Radius)`) for a
meridian whose start lies `s` rad from the pole reads an enclosure about
`ulp(r)·r/‖a_r‖` wide. That is because `‖a_r‖` and `δ`'s reach are
enclosures of a short cancelled vector. So a correct image escalates
there:

- At ε 1e-12, on `closed_image_rows`'
  `a_derived_circle_image_is_on_its_circle_where_its_azimuth_shows_most`
  sphere (`r = 2`), the term's upper end is 7.2e-12 to 9.4e-12 at
  `s = 1e-3`, 6.6e-10 to 8.9e-10 at `1e-5`, and 7.5e-8 to 9.6e-8 at
  `1e-7`.
- The pinned seeds in `a_pole_start_meridian_at_a_fuzz_seed_certifies`
  read 3.1e-10 and 1.7e-10.
- Main's arithmetic reads the same or wider (9.2e-12 to 1.0e-11 at
  `s = 1e-3`). The f64 mint door certifies every one of them, and
  their true misses are below 1e-13.

It ends in the lever, so nothing false is certified.

## Direction

Read `a′` and `b′` off directions with no short cancellation: `d̂` is
already `m = e × n` (`in_carrier_plane`). The remaining width is in
`s_r = ‖a_r‖`. Reading it as `m · a` would take `a`'s whole enclosure
rather than the cancelled one, but its sign needs a decided read.
