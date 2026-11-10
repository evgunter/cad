---
id: the-torus-harmonics-charge-is-loose-on-fat-tori-and-near-tight-on-others
kind: issue
title: conic_torus_harmonics: its charge squares R² + r² where the error is linear in it (loose by (R² + r²)/|R² − r²| on fat tori), is within 2× of its 16-ulp constant on some non-fat draws, and its f_per_metre_lo floor reads the rounded R² − r²
status: open
opened: 2026-11-10
priority: P3
cost: E
---


Three findings from PR 6711's FULL review on `crates/geom-brep/src/implicit.rs` `conic_torus_harmonics`, none a red today. Recorded here so that a fat-torus decision that refuses, or a fuzz seed that fails, has a place to start.

## 1. The charge is loose on a fat torus

PR 6711 charges `terms = (|S| + R² + r²)² + 4R²(|S| + |h|²)`. The rounding `R² − r²` carries is linear in `R² + r²`, and it reaches the coefficients through `p⊗p` once, as `2·|p|·δk`. So the sound term is about `|p|_old·(|S| + R² + r²)`, with `|p|_old = |S + R² − r²|`. Squaring the whole magnitude over-charges by about `(R² + r²)/|R² − r²|`. On the pinned draw (`circle_lane_rows::a_fat_toruss_coefficients_charge_its_radii_whole`) the review measured the slack at about 2⁹.

**Where to look:** a circle or ellipse against a fat torus (`r` near `R`) whose `circle_torus` or `ellipse_torus` decision refuses, or escalates, on a noise larger than its residual reads. **The fix:** charge `|p|_old² + 2|p|_old·(|S| + R² + r²)` plus the `4R²` part, with the same proof (`δp₀ ≤ c·ε(|S| + R² + r²)`, entering `p⊗p` linearly), and measure as PR 6711 did. It was not tightened in 6711 because nothing refuses on it today, and the looser charge is sound.

## 2. Near-tight draws on non-fat tori

The review's sweep found draws whose exact excess sits within 1–2× of the 16-ulp constant (`SHORT_CHAIN_ULPS`) on ordinary tori: seeds 5, 11, 12, 23 and 56 of its sweep. They pass. They are the first to fail if the constant is lowered or the evaluation order changes, and are recorded as that warning.

## 3. `f_per_metre_lo` reads the rounded `R² − r²`

`f_per_metre_lo = 2r·k`, with `k = R.powi(2) − r.powi(2)` rounded. On a fat torus `k` can stand above the exact value by its rounding, about `ε(R² + r²)`, which is about 18ε relative on the pinned draw. So the floor can stand that far above the true floor, and `noise / f_per_metre_lo` understates by the same relative amount. That is negligible beside the 16-ulp constant, but it is not a floor.

**Why it was not folded into the follow-up.** Lowering it by `rounding_charge(R² + r²)` is one line, but the floor can then reach zero on a torus the ring gate certified. `geom::ring_torus` decides `R − r` against the band, and at ε 1e-12 with `R + r` near 2000, `R − r` can be smaller than `k`'s rounding. There `half_angle::HalfAngleFrame::noise_reading` would divide by zero rather than by a tiny positive floor. What a certified ring with no certified floor should do (refuse at the door, or read the floor as an interval) is a decision for whoever takes this row.
