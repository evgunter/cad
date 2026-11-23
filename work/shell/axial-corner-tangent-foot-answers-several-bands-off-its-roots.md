---
id: axial-corner-tangent-foot-answers-several-bands-off-its-roots
kind: issue
title: the axial corner's tangent foot answers a corner several zero bands off both roots when the profile curve is wider than the edges' reach
status: open
opened: 2026-11-21
---

Found by the review of PR 7227 (`origin/analysis/oneround/unsourced-bits-review`, `analysis/oneround/unsourced-bits/review-report.md`, MAJOR 1) and kept by `oneround/unsourced-unions`, which reads `offset_axial::corner_arms` as each edge's reach from the corner.

`offset_axial::solve_pairs` and `cap_pair_corner` read `offset_axial_corner` (a pair's transversality levered at the corner's arms) and, on Zero, answer the pair's `tangent_foot`. Its doc says the foot sits `√(2r·gap)` off a root wherever a root is determined. With the arms the corner's own edges' reach (the README's lever), a station line `ρ = 11 − δ`, `δ = 8·10⁻¹²`, against the unit profile circle reads Zero at a half-turn rim of reach 2 under the band `(10⁻⁵, 10⁻⁴)` and answers `(11 − δ, 0)`, `4·10⁻⁶` off both roots `(11 − δ, ±4·10⁻⁶)` (row `offset_axial::tests::a_tangent_corner_answers_its_foot_at_the_rims_reach`, at a band where `4·10⁻⁶` is inside the zero). At the review's band `(10⁻⁶, 10⁻⁵)` with a reach of `0.2` the same foot is four zero bands off. The window is `det·reach ≤ ε` with a profile radius past the reach: the foot's error is about `r_profile·ε/reach`, more than ε once the profile curve is wider than the edges.

**Measured** by `oneround/unsourced-unions` on the review's figure (row `offset_axial::tests::a_wide_profiles_tangent_foot_stands_four_bands_off_its_roots`). The station line `ρ = 11 − δ`, `δ = 8·10⁻¹²`, against the unit profile circle (a profile wider than the edges), at an arm of `0.2`, under the band `(10⁻⁶, 10⁻⁵)`:

- The transversality `4·10⁻⁶` reads Zero over the arm, and `solve_pairs` answers the foot `(11 − δ, 0)`.
- The true roots are `(11 − δ, ±√(2δ − δ²)) = (11 − δ, ±4.0·10⁻⁶)`. The foot is `4.0ε` from each.
- The foot lies on the station line, and `8·10⁻¹²` off the circle.

**Read by PR 7294's second review: not a wrong answer** (`likely`).

- The line and the circle lie within ε of each other wherever `|h| ≲ √(2(δ + ε)) ≈ 1.41·10⁻³`, about 1 400ε along the line. So the corner is determined to no better than that under the run's tolerance.
- The two roots are `8ε` apart, inside the escalation width (`10ε`), so they are one root to within the band.
- The foot is a vertex within the band of both moved surfaces, which is what a solved corner owes. It is served by the tangent arm and never claimed transversal.
- Being `4ε` from each root is a statement about a meeting the band does not resolve.

**Still open.** A door-built body was not built: a near-tangent axial offset (the moved station plane within `δ ≪ ε` of tangency to a moved profile circle) at a corner whose edges reach less than the profile's radius. Nothing excludes one. If one were built, the question is whether anything downstream reads the foot as a transversal corner, or needs the vertex within less than ε of a particular root. No clause read so far asks either. No suite lands there: no verdict moved at `corner_arms` over the 204 `ci` tests at 1e-9 and 205 at 1e-12 that reach it, and the 22 tour tests.

The lever is not the question: it is the edges' reach, read from both sides.
