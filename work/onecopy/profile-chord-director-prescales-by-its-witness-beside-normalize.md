---
id: profile-chord-director-prescales-by-its-witness-beside-normalize
kind: issue
title: profile's chord_director prescales a chord by its own witness, a second copy of Vec2::normalize's range handling
status: open
opened: 2026-11-06
priority: P3
cost: S
refs: [6572]
---


Found by `onecopy/normalize`'s second pass (the `norm_squared().sqrt()` spelling of a hand normalisation).

## What

`profile::path` `chord_director` divides its chord by `norm_witness()` (`let scaled = d / witness;`) so the squares stay of order one, then normalises the scaled chord. `geom_core::Vec2::normalize` (`crates/geom-core/src/linalg/vec.rs`) already does that range handling, with an exact power-of-two prescale. The final step now reads `scaled.normalize()`, which is bit-identical because the scaled chord's largest component is 1, but the witness division is still a copy of the home's prescale.

Swapping it for `d.normalize()` moves bits: the division by the witness rounds and the power-of-two prescale does not. At `Interval`, the division by an interval witness also widens the enclosure by the witness's own width. Neither has been measured.

## Owed

Measure `d.normalize()` against `(d / witness).normalize()` at `f64` (the ulps that move) and at `Interval` (the enclosure width for a chord with width), and route the function to the home if it is no worse. Its zero and non-finite guards (`Dir::from_angle` past the range, `+x` for the zero chord) stay: `normalize` answers NaN there.
