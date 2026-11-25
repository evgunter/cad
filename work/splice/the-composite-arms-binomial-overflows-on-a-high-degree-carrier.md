---
id: the-composite-arms-binomial-overflows-on-a-high-degree-carrier
kind: issue
title: The composite arm's binomial overflows usize on a high-degree carrier, a panic in dev and wrapped weights in release
status: open
opened: 2026-11-23
---


Found by the measurement lane of `work/scalar/a-crossing-parameters-check-bounds-across-the-face-and-its-readers-decide-along-the-edge.md`, 2026-11-23, while sweeping carrier degrees.

## What

`binomial` in `crates/geom-brep/src/curve_face/composite.rs` (about line 546) folds `acc * (n - i) / (i + 1)` in `usize`, and its doc says it is "exact for the degrees a carrier reaches". `product` calls it with the summed degree of two Bernstein rows, and multiplies two of its results before dividing (`binomial(p, i) * binomial(q, k - i)`), and `slope_row` calls `product` on the composite's numerator and denominator rows.

**Demonstrated** (dev profile, `crossings` through its public door): a degree-9 non-rational Bézier segment against a torus of major radius 10 and minor radius 1 panics with `attempt to multiply with overflow` at `binomial`, the backtrace through `product`. The composite against a torus has four times the carrier's degree, so the slope row's product is over degrees 35 and 36. A degree-7 carrier against the same torus runs.

**By inspection**: `[profile.release]` in the workspace `Cargo.toml` leaves `overflow-checks` off and says so, so the release build wraps instead. The weights `product` forms are then wrong numbers in point enclosures, the slope row built from them encloses nothing in particular, and `one_signed(slope_row(..))` is the reading the walk takes a piece as monotone on. A piece read monotone that is not is settled by its ends alone. Not run in release.

**Estimate, not verified**: the fold overflows once `C(n, i)·(n − i)` passes `2⁶⁴`, near `n = 62`; the summed degree in `slope_row` is twice the composite's less one, so the composite's degree at about 32: a torus from carrier degree 8, a cylinder, sphere or cone from 16, a plane from 32.

## What it asks

A binomial that cannot wrap on any degree the carrier's door admits (an exact wider integer, or the weights formed in certification arithmetic by the recurrence), or a typed refusal above the degree it is exact to. A row at the degree that overflows today.

The reproduction is not committed as a test: it is red in dev and silently passes in release. The carrier is `segment(9, ..)` of `crates/geom-brep/src/curve_face/composite/crossing_parameter_rows.rs` against that file's torus frame with radii 10 and 1.
