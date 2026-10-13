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
