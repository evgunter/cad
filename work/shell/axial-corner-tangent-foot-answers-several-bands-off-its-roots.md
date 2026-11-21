---
id: axial-corner-tangent-foot-answers-several-bands-off-its-roots
kind: issue
title: the axial corner's tangent foot answers a corner several zero bands off both roots when the profile curve is wider than the edges' reach
status: open
opened: 2026-11-21
---

Found by the review of PR 7227 (`origin/analysis/oneround/unsourced-bits-review`, `analysis/oneround/unsourced-bits/review-report.md`, MAJOR 1) and kept by `oneround/unsourced-unions`, which reads `offset_axial::corner_arms` as each edge's reach from the corner.

`offset_axial::solve_pairs` and `cap_pair_corner` read `offset_axial_corner` (a pair's transversality levered at the corner's arms) and, on Zero, answer the pair's `tangent_foot`. Its doc says the foot sits `√(2r·gap)` off a root wherever a root is determined. With the arms the corner's own edges' reach (the README's lever), a station line `ρ = 11 − δ`, `δ = 8·10⁻¹²`, against the unit profile circle reads Zero at a half-turn rim of reach 2 under the band `(10⁻⁵, 10⁻⁴)` and answers `(11 − δ, 0)`, `4·10⁻⁶` off both roots `(11 − δ, ±4·10⁻⁶)` (row `offset_axial::tests::a_tangent_corner_answers_its_foot_at_the_rims_reach`, at a band where `4·10⁻⁶` is inside the zero). At the review's band `(10⁻⁶, 10⁻⁵)` with a reach of `0.2` the same foot is four zero bands off. The window is `det·reach ≤ ε` with a profile radius past the reach: the foot's error is about `r_profile·ε/reach`, more than ε once the profile curve is wider than the edges.

The lever is not the question (it is the edges' reach, read from both sides); what the foot owes a corner whose roots stand past the band from it is. No suite lands there (no verdict moved over the 193 fast-set rows that reach `corner_arms`).
