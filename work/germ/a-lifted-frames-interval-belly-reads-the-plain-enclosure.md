---
id: a-lifted-frames-interval-belly-reads-the-plain-enclosure
kind: issue
title: At Interval, a conic lifted the production way reads its belly as the plain enclosure, about 1e16 u of the belly wide at far scale
status: open
opened: 2026-11-23
priority: P2
cost: M
refs: [7553]
---


Filed by GERM's `germ/harmonic-belly` lane (PR 7553), from its confirmation review.

## Finding

`geom_core::Real::harmonic_belly` at `Interval` (`crates/geom-core/src/interval.rs`) encloses the exact expansions only where every input is a **stated point**: each enclosure's two ends are one `f64`.
- **The production lift is not that.** `Curve3::map_scalar(Interval::from_f64)` lifts a conic's frame with its axis and `u_ref` as enclosures of the polar factor, which are not points. So `Conic::belly_along` at `Interval` falls back to the plain formula's enclosure.
- **The reviewer measured** the plain enclosure's width over the belly at about 1e16 to 5e17 u, at far 1e4 to 1e6.
- **What it costs.** That is sound: it holds the belly, and a decision on it escalates rather than misreading. But the certified lane cannot decide a far-centred conic's graze that `f64` decides to its digits.

## Settles when

A conic lifted the production way reads an `Interval` belly whose width is a few ulps of the belly. For example:
- the exact expansions enclosed over the polar factor's enclosure (each term's monotone ends, or an expansion of the enclosure's centre plus a first-order radius); or
- the lift stating the stored directions as points beside their enclosure.

A row would assert it: `a_far_centred_harmonics_belly_is_read_to_its_own_digits` read through `Curve3::map_scalar(Interval::from_f64)`, with the width bounded by a fixed number of u of the belly at far 1e4 to 1e6.
