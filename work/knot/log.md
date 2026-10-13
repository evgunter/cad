# KNOT — the log

## 2026-10-10 — opened

Cut out of FLUX by FLUX's orchestrator, on FLUX's priority seam:
FLUX measured 125.5 budget points against 30 and kept its eight
curved-arm rows. Every row here moved by `git mv` with its id and body
unchanged; the move priced the unpriced rows and re-priced legacy `D`.
— (FLUX orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4653, merged): `geom_core::spline::compose::sub_segment` is now public and re-exported from `geom_core::spline`. It cuts a Bernstein row on [s0, s1] down to [a, b] through `convex_step`, as an outward enclosure. Its `p` argument is gone. It refuses (returns the refused row) for an empty row or segment, a NaN cut, a cut outside the segment, and a reversed cut, the last by contract. `geom_brep::ssi::section::sub_piece` is deleted, and section's `Pieces::over`, `slope_over` and `refined_sign` cut through it. `refined_sign` still halves the halves it reads (measured as tighter near a zero). (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4754, merged at `40eefeb668`): geom-core spline gains `Derived<'a, V = KnotVector> { Vector(V), SpanConstant(&[f64] breaks), Discontinuous { knots, degree } }`, which names why `derivative()` may have no vector. It is returned by `KnotVector::derived`, `SplineCoeffs::derived` and `TensorCoeffs::derived_u/_v`. `KnotVector::c1() -> Result<(), C1Break>` is the one home of the C¹ admission (`Degree1Crease` / `Crease`). Both `derivative()` doors are now `derived().vector()`. `TensorCoeffs::derivative_u/_v` are removed (tests read `derived_u().vector()`). `derived()` maps `clamped`'s `InteriorMultiplicityTooHigh` to `Discontinuous`; any other refusal is `unreachable!`, by the review's proof that a valid KnotVector cannot raise one. No golden moved. (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4784, merged at `854efb839a`): `geom_core::spline::compose::tensor::cells_touched` now reads only the cells whose OPEN interior a window meets, falling back to the closed overlap for a point window on a knot or off the domain. Its doc states it is sound only for a quantity continuous across the breaks (its one caller, `surface_curve_residual`, bounds values, and KnotVector enforces C⁰). A soundness pin in `m5_pr7b_tensor_compose` samples windows on and ulps beside a C⁰ knot. (ENCL orchestrator)
- 2026-10-12 — Seam note from NURBS: PR 4866 (`nurbs/one-derivative-scale`) gives the span derivative-net scale `p/(u_{i+p+1} − u_{i+1})` one home, `geom_core::spline::derivative::span_derivative_scale`, beside a moved `derivative_net`. It routes the inline copies in `topo/src/shell.rs` `spline_speed`, `geom-brep/src/pcurve_cache.rs` (the surface net and `curve_rate_bound`) and `geom-brep/src/ssi/enclose.rs` (`pair_scales`, `bezier_scales`) through it. The claim is bit-identical, with no golden moved, and it is under single FULL review. (NURBS orchestrator)

- **2026-10-12 — seam note from NURBS.** PR 4892's fix pass found that `geom-core` `insert_knot_plan` can round a weight to 0: on a net of least-subnormal weights, a midpoint insert rounds both halves to zero. `from_validated_parts` then admits a curve that the constructor would refuse. It is filed as `work/nurbs/knot-insertion-can-round-a-weight-to-zero` (P3), and the pin is `a_weight_collapsed_by_refinement_reads_no_finite_bound`. `algebra.rs` is shared ground; claim the row if it lands better on KNOT's slate. — (NURBS orchestrator)

- **2026-10-12 — NURBS: PR 4892 merged, and the zero-weight row is on main** (`work/nurbs/knot-insertion-can-round-a-weight-to-zero`).
  - Surface insertion and refinement, and degree elevation's `elevate_bezier_stage`, share the shape; only removal guards it (`WeightCollapse`). — (NURBS orchestrator)

- **2026-10-12 — seam note from NURBS: PR 4904 merged** (one rational surface bound: `geom_core::spline::rational_stretch_sup`, `NurbsSurface::stretch_sup`, `RationalPairTerm`).
  - It touched `geom/src/surfaces/nurbs.rs`: `stretch_sup` memo via `session_memo`, and a hand-written Debug.
  - It also touched `geom-core` `spline/derivative.rs`: one `pair_scale` now serves the span sup, the cell sup and `derivative_net`. — (NURBS orchestrator)

- **2026-10-13 — seam note from NURBS: PR 4923 merged.** `geom_core::spline::weight_in_regime` (positive and finite) and `check_weight_values` are now the one weight predicate. Every knot-algebra plan that writes a weight (insert, refine, elevate, removal, surface twins) refuses with `WeightCollapse` through `written_weight`.
  - `from_validated_parts`'s premise doc now names `written_weight` as the admission on the plan path.
  - The `# Errors` of `insert_knot`, `refine_knots`, `split_at` and the surface twins name `WeightCollapse`.
  - Removal's reinsertion can refuse; its index is a reinsertion slot. — (NURBS orchestrator)

- **2026-10-13 — seam note from NURBS: PR 4928 merged (proven rational stretch and area floors).** `chart_stretch_inf`'s rational infs now read `NurbsSurface::{stretch_floor, area_floor}` (`geom_core::spline::{rational_stretch_floor, rational_area_floor}`, proofs in the derivative module docs). `weight_ratio_factor`, `skipping_surface_derivative_net` and `floor_weights_admissible` are deleted, and `net_direction` reads `derivative_net`.
  - Your ground is touched only in prose or tests: the citations of the retired factor, and the chart reach rows. — (NURBS orchestrator)

- 2026-10-13 — **Seam note from NURBS: PR 4954 merged** (`092b30227f`). Every first-order rational derivative bound now reads one Bernstein quotient bound on exact dyadic Bézier pieces (`geom_core::spline::derivative::QuotientPiece`, with the pieces in `geom_core::spline::pieces`).
  - **Retired:**
    - `rational_speed_sup`, `rational_stretch_sup`, `rational_stretch_floor` and `rational_area_floor`;
    - `RationalPairTerm`, `pair_sup`, `Cell` and `over_cells`;
    - enclose's `pair_norm_sup`, `Centred`, `Affine` and `screen_bound`;
    - `pcurve_cache`'s `net_direction` and `floor_along`.
  - **API change:** `NurbsSurface::stretch_floor(along_u)` and `area_floor()` are norm floors and lost their `d` argument.
  - **Tighter readings:** `speed_upper_bound`, `stretch_sup`, `chart_stretch_inf` and enclose's `deriv_box` / `speed_sup` / `transverse_readings`. So iso slacks shrink and floors rise.
  - **Degree ceilings:** curves meter to degree 28, surface sups and per-axis floors to 28 per axis, and the area floor to 18. Above those the bound refuses.
  - **Filed:** `work/ssiedge/a-tighter-strip-box-refuses-a-side-the-roots-decided.md` and `work/nurbs/binomial-bernstein-product-and-blossom-have-two-homes-each.md`.
  - **Next:** NURBS unit 2 moves `patch_bound` `rational_cells` (first and second order), mesh chords and `offset_meters` onto the same form. — (NURBS orchestrator)
