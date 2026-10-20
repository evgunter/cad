# CHORD — the log

## 2026-09-20 — opened

Cut out of TESS, which was carrying 69 budget points, when Ev ratified
the priority and track-size conventions in chat the same day. The cut
followed TESS's PRIORITY seam per `work/README.md` "Track size": TESS
keeps the ten rows where the mesh LIES or REFUSES — an unsound NURBS
face bound with a recorded failing seed, a panic on a rim-only sphere
cap the shape door admits, a lofted Circle section that cannot be
meshed at all — and this program takes the arithmetic and the
certificates beneath them.

16 rows arrived by `git mv` with their ids, bodies and history
unchanged. Band 6900-6999 claimed in this commit
(`docs/MODEL-AB-LOG.md`). Nothing dispatched.

## Announced from ENCL (2026-09-28): `C23`'s question is answered

ENCL's PR 3338 (merged `b3e5937b8e`) settled the first half of `C23`: `RATIONAL_CERT_SPLITS` and `RATIONAL_METER_SPLITS` are two independent choices, now documented as such at both sites. The evidence is appended to `C23`, and the close is left to CHORD.

Signed: (ENCL orchestrator)
- 2026-10-08 — Filed from ENCL's §5 sweep (`encl/offset-cert-coefficient-norms`): `mesh-nurbs-cert-reads-partial-norms-off-per-coordinate-cell-boxes` (P3). `PatchCell` gained `s_u_sup`/`s_v_sup` (coefficient-norm speeds on the integral arm); `nurbs_cert` still folds the boxes. (ENCL implementer)
- 2026-10-08 — Seam note from ENCL (PR 4348, `encl/offset-cert-coefficient-norms`, merged): the offset certificate's vector upper bounds (‖Y‖ via `Composite::y_sup`, `m_tilde_sup`, and the integral arm's chart speeds via `PatchCell::s_u_sup`/`s_v_sup`) now read coefficient norms (D4 ¶2), not per-coordinate boxes. geom-core gains public `tensor::coefficient_norm_bound` (`[&[Interval]; 3]`, refuses ragged rows), `coefficient_norm_sup` and `PatchSpans::cell_norm_sup`. The bounds are tighter or equal, and the stored offset numbers were re-baselined. The three `rigid_map_near_eps_approx` rows moved to the slow set, and their witness now mints on round 2 at `MARGIN = 1/4096`. Your ground: `work/chord/mesh-nurbs-cert-reads-partial-norms-off-per-coordinate-cell-boxes.md` (P3) was filed for `nurbs_cert::cell_readings`; the norm doors above are what it can read. (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4754, merged at `40eefeb668`): `mesh/nurbs_cert.rs` `whole_net_bound` matches `TensorCoeffs::derived_u/_v` (same values). `chords-carrier-gate-re-spells-check-direction` now points `chords.rs` at geom-core's `KnotVector::c1`. (ENCL orchestrator)
- 2026-10-12 — Note from NURBS: PR 4860 (merged `564664c8a7`) removed the geom rational speed meter's refined scan, and `RATIONAL_METER_SPLITS` with it. CHORD's `C23` asks whether `RATIONAL_CERT_SPLITS` and `RATIONAL_METER_SPLITS` are one schedule. Its geom side is gone, so only the certification schedule remains and the question may close. (NURBS orchestrator)

- **2026-10-13 — seam note from NURBS: PR 4923 merged.** `geom_core::spline::weight_in_regime` (positive and finite) and `check_weight_values` are now the one weight predicate. Every knot-algebra plan that writes a weight (insert, refine, elevate, removal, surface twins) refuses with `WeightCollapse` through `written_weight`.
  - Your file's own spelling of the positive-and-finite check now reads `weight_in_regime`. Behaviour is unchanged. — (NURBS orchestrator)

- 2026-10-13 — **Seam note from NURBS: PR 4984 merged** (`2639b944`), unit 2 of the quotient ruling.
  - **Second order on the quotient form.** `C″`, `S_uu`, `S_uv` and `S_vv` now read the one Bernstein quotient bound over `W³`. Entry points: `geom_core::spline::{accel_pieces, cell_second_quotients, polynomial_area_pieces}` and `NurbsCurve3::accel_upper_bound`.
  - **Moved onto it:**
    - `geom-brep` `patch_bound` `rational_cells` (first- and second-order boxes, `m_sup`). The refinement and `coordinate_hull` are gone.
    - `mesh` chords' rational `sup|C″|`.
    - `offset_meters` `cell_normal` / `patch_regularity`, which now reads the area floor `PatchCell::m_inf`.
  - **Integral arm:** the area floor reads `A_u×A_v` at `2p−1` and certifies to degree 28. Past that it uses a conservative floor of 0.0.
  - **Refusals:**
    - a rational degree above 18 gives `PatchBoundError::DegreeAboveCeiling`, a typed recourse;
    - chords past the ceiling give `UnsupportedCurve`;
    - `QuotientRefused` is new (kernel defect);
    - `RefinedWeightLostPositivity` is gone;
    - `RefinementFailed` now means the integral insertion chain only.
  - **Mesh consequence:** a rational face above degree 18 now refuses `UnsupportedNurbsFace` in nurbs_cert, and offset refuses through `MeterError`.
  - **Goldens re-baselined, with causes in the PR body:** `m_sup`, chord counts, offset goldens, the budget and endpoint censuses, and meter ceilings.
  - **Perf:** the per-face bound is about 2× base on small rows and up to 11× on wide rational faces. Filed at `work/nurbs/second-order-quotient-pieces-cost-the-face-bound-eleven-times-main.md`. — (NURBS orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5489 merged (`5b6ac199`). `mesh::nurbs_cert`'s `NurbsCellGrid::cert` now answers `+∞` for any triangle whose UV box strictly crosses a crease cut (an interior knot of multiplicity ≥ degree, in either direction). The max-of-cells argument needs C¹ across the cut, and interior grid triangles straddle u-cuts as well as boundary fans; the docs now say so. `NurbsCellGrid::from_cells(cells, (creases_u, creases_v))` takes the creases as an argument. `geom_core::spline::KnotStructure::crease_runs` is the one statement of where a crease is, and `KnotVector::c1` reads it (same verdicts). Nothing that meshes today moved: the C¹ gate still refuses creased faces. The follow-up (NURBS build C) relaxes the gate for rational creased faces and meshes lofted circles. (NURBS orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5499 merged (`16b6b72f`, NURBS build C, TESS ground, FULL). **Lofted `Circle` sections now mesh.** The mesher's only door, `geom_brep::patch_bound::patch_cells_across_creases`, admits degree-≥2 creases on the rational arm; the integral arm, degree-1 creases and every other `patch_cells` caller (offset_fit, offset_meters) stay C¹-gated. A crease becomes a CDT constraint in `mesh::trimmed`. `geom_brep::sub_arc_joint(k, m)` is the one spelling of a sub-arc joint (shared by `sweep::skin::segment_curve`, `IsoArc` breaks and chord schedules; bits identical). An `IsoArc` rim with m ≥ 2 sub-arcs rounds its chord count to a multiple of m, so the joint is a chord point. A boundary crossing a crease off a chord point refuses `UnsupportedCurve`; creases in both directions refuse `UnsupportedNurbsFace`. No golden moved; the falsifier stays ≤ 0.4999 over 11 lofts × 20 δ. Closes `lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`. Still open: `work/tess/lofted-rational-walls-refuse-at-some-deltas` (P0, design), refusals away from the crease. (NURBS orchestrator)
