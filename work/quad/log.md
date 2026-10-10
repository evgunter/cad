# QUAD log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/quad/plan.md`. A/B band 5700-5799
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Cut from PROPS (2026-09-20)

Ev, in chat: PROPS' slate had reached 66 open rows, and the question
put back was whether to cut new programs for the logical chunks and
leave in PROPS only what that orchestrator would finish itself. Ev:
*"that sounds like a good plan!"* This program is one of three cut
that day (QUAD, ENCL, FRAME). No orchestrator is seated yet — the
items, the charter and the lanes are here so that seating one is the
only remaining act.

The cut deliberately does NOT contort the territory globs to keep the
at-rest overlap check quiet. Ev, same conversation: *"it's ok if units
have shared ground, they should just be aware of each other if working
at the same time"* — so the `keep_out` above records the real
relationships rather than a partition, and the awareness mechanism is
the per-branch territory check plus the announced-seam convention.

- 2026-09-28 — Seam note from ENCL: PR 3348 (merged `95b59b9361`) homes the domain-uniform refinement grid in `geom_core::spline::algebra::domain_grid_points(kv, pieces, GridSkip)`. `GridSkip` is `BitEqual` or `WithinUlps(u32)`, and `pub const SLIVER_CLEARANCE_ULPS` replaces `quad.rs`'s private `SLIVER_CUT_ULPS`. It is used at `props/quad.rs` `refine_dir` and `bezier_blocks`, `ssi/certify.rs` `refined`, `edge_nurbs.rs` `localized::breaks`, and one tcost/tint test. Bits are unchanged at every site (pinning rows added first). Each caller still chooses its own skip guard and control-count cut-off, so the NURBS hairline fix is now a one-argument change at each site. (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-10-02 — New P0 row from SHOW: `check-7-refuses-the-reporting-budget-on-a-definite-sign` (PR 3838, the lily's lanceolate blades). Check 7's sign walk is cut off by the round-0 `last_round_refuses` exit against the REPORTING target, so a loft or cubic sweep of an arc section is refused tier 3 at the default ε — a regression of the refusal TCOST-K3 retired. Same exit in all three patch lanes `sign_walk` reaches. (SHOW lane, lily-lanceolate)
- 2026-10-08 — Filed from ENCL's §5 sweep (`encl/offset-cert-coefficient-norms`): `quad-area-pads-read-the-cross-norm-off-per-coordinate-boxes` (P3), the D4 ¶2 shape in `props/quad.rs`'s area pads. (ENCL implementer)
- 2026-10-08 — Seam note from ENCL (PR 4348, `encl/offset-cert-coefficient-norms`, merged): the offset certificate's vector upper bounds (‖Y‖ via `Composite::y_sup`, `m_tilde_sup`, and the integral arm's chart speeds via `PatchCell::s_u_sup`/`s_v_sup`) now read coefficient norms (D4 ¶2), not per-coordinate boxes. geom-core gains public `tensor::coefficient_norm_bound` (`[&[Interval]; 3]`, refuses ragged rows), `coefficient_norm_sup` and `PatchSpans::cell_norm_sup`. The bounds are tighter or equal, and the stored offset numbers were re-baselined. The three `rigid_map_near_eps_approx` rows moved to the slow set, and their witness now mints on round 2 at `MARGIN = 1/4096`. Your ground: `work/quad/quad-area-pads-read-the-cross-norm-off-per-coordinate-boxes.md` (P3) was filed for `props/quad.rs`'s pads; the norm doors above are what it can read. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL: filed `nightly-k-lint-dies-at-1e-9-on-the-lily-leaf-b-quadrature-wall` (P1) here. Today's nightly dev-probe k-lint dies on the pinned `LEAF_B_VOLUME_WALL` refusal at 1e-9, so k-lint never runs. (ENCL orchestrator)
