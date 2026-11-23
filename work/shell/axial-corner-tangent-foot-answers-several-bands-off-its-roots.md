---
id: axial-corner-tangent-foot-answers-several-bands-off-its-roots
kind: issue
title: the axial corner's tangent foot answers a corner several zero bands off both roots when the profile curve is wider than the edges' reach
status: open
opened: 2026-11-21
priority: P1
---

Found by the review of PR 7227 (`origin/analysis/oneround/unsourced-bits-review`, `analysis/oneround/unsourced-bits/review-report.md`, MAJOR 1) and kept by `oneround/unsourced-unions`, which reads `offset_axial::corner_arms` as each edge's reach from the corner.

`offset_axial::solve_pairs` and `cap_pair_corner` read `offset_axial_corner` (a pair's transversality levered at the corner's arms) and, on Zero, answer the pair's `tangent_foot`. Its doc says the foot sits `√(2r·gap)` off a root wherever a root is determined. With the arms the corner's own edges' reach (the README's lever), a station line `ρ = 11 − δ`, `δ = 8·10⁻¹²`, against the unit profile circle reads Zero at a half-turn rim of reach 2 under the band `(10⁻⁵, 10⁻⁴)` and answers `(11 − δ, 0)`, `4·10⁻⁶` off both roots `(11 − δ, ±4·10⁻⁶)` (row `offset_axial::tests::a_tangent_corner_answers_its_foot_at_the_rims_reach`, at a band where `4·10⁻⁶` is inside the zero). At the review's band `(10⁻⁶, 10⁻⁵)` with a reach of `0.2` the same foot is four zero bands off. The window is `det·reach ≤ ε` with a profile radius past the reach: the foot's error is about `r_profile·ε/reach`, more than ε once the profile curve is wider than the edges.

**Measured** (`oneround/unsourced-unions`, on the review's figure; row `offset_axial::tests::a_wide_profiles_tangent_foot_stands_four_bands_off_its_roots`). Setup: the station line `ρ = 11 − δ`, `δ = 8·10⁻¹²`, against the unit profile circle, a profile wider than the edges, with an arm of `0.2` under the band `(10⁻⁶, 10⁻⁵)`.

- The transversality `4·10⁻⁶` reads Zero over the arm, so `solve_pairs` answers the foot `(11 − δ, 0)`.
- The true roots are `(11 − δ, ±√(2δ − δ²)) = (11 − δ, ±4.0·10⁻⁶)`. The foot is **`4.0ε` from each**.
- The foot lies on the station line, and `8·10⁻¹²` (`8·10⁻⁶ ε`) off the circle.

**By the review's test it is a wrong answer.** A served corner past ε from both true roots is not either root, so the foot answers a corner the geometry does not have.

**The reading that says otherwise** is given here for the shell lane to weigh, and is not acted on. The point served is within the band of both curves. The two roots stand `8ε` apart, which is in band (`ε < 8ε < Kε`), so the band does not tell them apart as two corners. Where the band reads one tangency, the foot is the one point both curves pass through to within `δ`. Whether a corner owes ε to the curves or ε to a root is what this row asks.

**Banded P1, not P0.** It is not live. Over the 204 `ci` tests at 1e-9 and 205 at 1e-12 that reach `corner_arms`, and the 22 tour tests, no verdict moved between the arc-length arm and the reach. No suite fixture puts a profile curve this much wider than its edges' reach inside the zero band.

The lever is not the question: it is the edges' reach, read from both sides. The question is what the foot owes a corner whose roots stand past the band from it.
