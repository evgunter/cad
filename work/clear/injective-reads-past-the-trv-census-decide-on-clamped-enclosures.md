---
id: injective-reads-past-the-trv-census-decide-on-clamped-enclosures
kind: issue
title: injective.rs reads newer than the TRV census decide on wall-image and jet enclosures that may clamp
status: open
opened: 2026-11-01
---


Found by TRV (`clear/trv`) while re-pinning `geom-core`'s
`certified_endpoint_census` (`ROSTER`) for `crates/topo/src/injective.rs`.
TRV made every value the clearance engine lifts refuse unless it
certifies (`clearance.rs` `certified_lift`, read by `window_of` and
`boundary_of`), so a presentation's carrier and boundary certify. Reads
added to `injective.rs` after the census lane's classification
(`clearance-lifts-trv-brackets-and-reads-their-ends`) were not examined
beyond that. Two of them read an enclosure computed FROM certified
inputs that can still come out `Trv` with ordinary ends:

- **`Wall::on`'s `u` arm** (`injective.rs`, `impl Wall`): `f` from
  `geom::spiric_radial` is a square root of `(R + r·cos t)² − d²`, which
  clamps where the enclosure of the radicand dips below zero. Its reads:
  `present_wall`'s `held` against `within(held, x)` and the `g` hull
  `(cu.0.min(g.lo()), cu.1.max(g.hi()))`; `graph_read`'s `inside` and
  `rb` where `g` is a wall image. A clamped `f` is a sound bracket of the
  defined values, so the question is whether a decision there needs the
  enclosure to be of the whole parameter range.
- **`speeds`** (`injective.rs`): `[ju.norm().hi(), jv.norm().hi()]`
  mints `f64` levers from `rect_jet`, read by `present_line_corner`'s
  sliver (`at(across) * at(speed[..])`), `leaves_line` and
  `leaves_graph`. TRV changed `present_chart`'s sliver to keep the
  lever as its interval; this is the same read in its siblings. Over a
  certified analytic carrier the jet has no clamping operation, and a
  rational net's zero-crossing weight gives an infinite end, which the
  norm's `hi` carries, so it may be sound as it stands; that argument
  is not checked against `geom::NurbsSurface::rect_jet`.

Also newer than the census and read at R1 only: `meets_range`,
`widened`, `within`, `present_line_corner`'s `across`/`along`,
`reach_of`, `run_on` (behind `along`'s ask). Each is a helper or a read
of a lifted value; list them here if one turns out not to be.

Close by asking or arguing each read at its site and replacing the
census note's pointer to this row with the disposition.
