# ISO — the log

## 2026-09-20 — opened

Cut out of CHART, which was carrying 105 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed CHART's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

7 rows arrived by `git mv` with their ids, bodies and history
unchanged. CHART keeps its band 6200-6299; band 7700-7799 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

- 2026-09-28 — Seam note from ENCL: PR 3348 (merged `95b59b9361`) homes the domain-uniform refinement grid in `geom_core::spline::algebra::domain_grid_points(kv, pieces, GridSkip)`. `GridSkip` is `BitEqual` or `WithinUlps(u32)`, and `pub const SLIVER_CLEARANCE_ULPS` replaces `quad.rs`'s private `SLIVER_CUT_ULPS`. It is used at `props/quad.rs` `refine_dir` and `bezier_blocks`, `ssi/certify.rs` `refined`, `edge_nurbs.rs` `localized::breaks`, and one tcost/tint test. Bits are unchanged at every site (pinning rows added first). Each caller still chooses its own skip guard and control-count cut-off, so the NURBS hairline fix is now a one-argument change at each site. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3351 (merged `e39a5c4cc4`) routes certification refusals per D4 ¶1 as Ev ruled on PR 3352. Recourse belongs to the decision (`CertCheck::ending()`, one table) and the reading belongs to the door: `geom_brep::certify::recourse(check, RefusedArm, Reading::{Build, AtRest, Adopt})`. `CertifyError`/`PlaneNurbsRefusal` `Display` is now payload-only. Each door appends `ending(reading)`. The `certification: ` prefix is gone. `edge_nurbs.rs`: `PlaneNurbsRefusal` gains `TransversalityEscalated`, `Escalated` is now only the rung-3 certificate, and `ending(reading)` is added. `TubeStraddles` routes as Transversality's Zero-or-Negative verdict (lever alone). This repairs the `Display` half of `plane-nurbs-escalation-renders-the-coincidence-menu-unlabelled`; its remaining half is yours to judge. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3382 (merged `9bf495c768`) adds `geom_brep::recourse`, the one table for sized decisions. `Reading`/`RefusedArm` moved there from `certify`, alongside `SizedPass`, `SizedDecision` and `Classified`. certify and the offset meters both route through it. The shared unreadable-margin note now reads "an unreadable or collapsed margin may indicate a kernel bug worth reporting". `edge_nurbs.rs`: `Reading` path and `Zero` → `Zero(None)` only; its endings are byte-identical. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3392 (merged `277dcb052b`) splits certify's conflated Zero/Negative verdicts: `IntervalNotForward { verdict }`, `WindingExceeded` routed as sign-certain, the tangent tube as its own `CertifyError::TubeNotSeparated` (`CertCheck::TangentTube`, lever alone at every reading), and `TubeStraddles { verdict: Refused }`. `RefusedArm::ZeroOrNegative` is deleted; `geom_brep::recourse` now holds `Refused` and `Definite`. A zero span stays a defect at every reading (Ev, e1600790f9). `edge_nurbs.rs`: `PlaneNurbsRefusal::TubeStraddles { verdict: Refused, boxes }`; its Zero arm quotes 'tighten below m/K' when the clearance is positive. (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4422, merged): `geom_brep::recourse::RefusedArm::SignCertain` now takes `Option<MarginDiag>`; construct with `SignCertain(None)` unless the decision is a residual miss, and match with `SignCertain(_)`. `certify::definite_miss_in_file` / `Unsized::definite_residual_in_file` are gone; `Unsized::residual_in_file` is the one door. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4427, `encl/limb-refusal-margin`, merged at `e3646226c0`): `SsiError::CertificateLimb`, `PlaneNurbsRefusal::Limb` and `AnalyticRung3Refusal::Limb` now carry `margin: MarginDiag` beside `value`, minted by `decide_reported`; the decision bit is identical to `decide`. Match them with `..`. The margin is for error text only: never branch on it (Bounds clause 2). `MissReading::DefiniteUnvalued` is gone, and `geom_core::FileCoincidence::new` now takes only `eps_in`. A new limb mint on your ground should use `decide_reported` and carry the margin, so that the import door can say whether the miss lies within the file's ε_in. Known gap: a hull-sup limb's margin is an upper bound, read today as the miss (ENCL row `hull-sup-limb-reads-its-bound-as-the-miss-at-the-import-door`). (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4457, merged at `dcff36e784`): the import-door twins `ending_in_file`, `render_in_file` and `recourse_in_file` are removed. The one body now takes `at: impl Into<ReadAt>`, so the import door is the same call without the suffix: `err.render(file)`, `err.ending(file)`, `decision.recourse(arm, file)`. An in-flight branch that calls a `*_in_file` will fail to compile at merge; drop the suffix. `Unsized::residual_in_file` and geom-core's `sized_recourse_in_file` / `miss_recourse_in_file` keep their names. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4475, merged at `61977302ae`): a poisoned margin's note now depends on the door that reads it. At a build it is "an unreadable margin may indicate a kernel bug worth reporting" ("or collapsed" is gone). At rest or at the import door it is "…a kernel or file defect worth reporting". `geom_brep::recourse::unreadable_margin_note(reading)` picks it. geom-core's `SizedWords` gained `unreadable: &str`, and `Indeterminate::ending_noted(levers, unreadable)` exists beside `ending`. An at-rest reader of a nested refusal composes its own ending (fork-log row 9): it does not add `ending(Reading)` to the type. `topo::props::SHELL_ROLE`, `ShellClassifyError::arm()`, `JoinUndecided::diag()` and `JOIN_SUBJECT`/`JOIN_LEVER` are public for that. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4504, merged): `geom_brep::recourse::Unsized::LastResort` is renamed `Unsized::Fit`, with the same behaviour, and a new `Unsized::Bound` ends a refusal on a certified upper bound in the kernel-limit last resort at every reading. At the import door, `MissReading::Bound` names the bound. `CertCheck::bounds_a_miss()` answers whether a check is a bound. A table on your ground that refuses on a bound, not a measured miss, should take `Bound`. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4529, merged): `SsiError::CertificateLimb`, `PlaneNurbsRefusal::Limb` and `AnalyticRung3Refusal::Limb` no longer carry `value: f64`; the margin is the only copy and their Displays print it. Refinement routes on a crate-private refused residual recorded with the refusal (`LimbRefusal`), not on the margin. The step-budget ending reads a decided `RefineStop::StepBudget { trend: ResidualTrend }`, and `STEP_BUDGET_FLOOR_RECOURSE` now says "the refused residual stopped falling". `RoundMargin` and `pcurve_cache::FittedMagnitude::LimbResidual` hold `MarginDiag`. Tests that read a margin's upper end use `geom_core::test_support::upper`, the one helper. This supersedes the earlier note saying the margin sits "beside `value`". (ENCL orchestrator)
- 2026-10-13 — **Seam (ENCL):** PR 4976 merged. Every geom-brep Zero-arm payload (`CertifyError`, `TorusConvention::refused`, `MeterError` NormalFloor/CurvatureHeadroom) now reads "is not shown … (margin {:e} m)". Whether a reading shows no size is left to the ending (`MarginDiag::shows`). A row PR 4976 filed on this slate asks for the same idiom on this ground. (ENCL orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5499 merged (`16b6b72f`, NURBS build C, TESS ground, FULL). **Lofted `Circle` sections now mesh.** The mesher's only door, `geom_brep::patch_bound::patch_cells_across_creases`, admits degree-≥2 creases on the rational arm; the integral arm, degree-1 creases and every other `patch_cells` caller (offset_fit, offset_meters) stay C¹-gated. A crease becomes a CDT constraint in `mesh::trimmed`. `geom_brep::sub_arc_joint(k, m)` is the one spelling of a sub-arc joint (shared by `sweep::skin::segment_curve`, `IsoArc` breaks and chord schedules; bits identical). An `IsoArc` rim with m ≥ 2 sub-arcs rounds its chord count to a multiple of m, so the joint is a chord point. A boundary crossing a crease off a chord point refuses `UnsupportedCurve`; creases in both directions refuse `UnsupportedNurbsFace`. No golden moved; the falsifier stays ≤ 0.4999 over 11 lofts × 20 δ. Closes `lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`. Still open: `work/tess/lofted-rational-walls-refuse-at-some-deltas` (P0, design), refusals away from the crease. (NURBS orchestrator)

## 2026-10-23 — picked up; the tail goes to ISOTAIL

An orchestrator holds the track (`status: active`). The track measured
48 budget points against 30, so its P2–P3 tail and four unbanded rows
moved to a new program, ISOTAIL, by `git mv` with ids and bodies
unchanged (`work/isotail/log.md` names how the unbanded rows were
priced). ISO keeps its P0 and four P1 rows, 21 points.

The P0's filed refusal text ("the seam carrier is not the chart's own
boundary row (its knot/weight structure differs)") is no longer in the
tree, so a measurement lane re-takes its grid on main before any
implementer is briefed. — (ISO orchestrator)

- 2026-10-23 — P0 PR 5629 is up and green: `skin_on` now picks its lane per control row. All 38 refusing cells failed the weights compare, never the knots, always the line wall's end column against the arc wall's corner column. All 48 cells build after the fix. One new row from it, `a-loft-whose-arc-bulge-varies-per-section-refuses-its-arc-rim` (P2): the CARVE row's second shape, which has another mechanism (the arc-rim pick on an unsplit body). Single FULL review dispatched on frozen `8ca0f5481a`. — (ISO orchestrator)
## 2026-10-23 — the P0 measured and dispatched

The measurement lane re-took the P0's grid on main (the row's
`## Measured on main` section): it is live, and CARVE's
`a-rational-wall-beside-an-integral-one-skins-its-shared-corner-off-unit-weight`
is its mechanism, moved here to ride with it. No design fork: the fix
acts at the producer (`skin_on` chooses its lane per control row, so an
integral corner column stays bitwise `1.0`), which is `skin`'s own
fit contract, and the certifier's exact one-spline-space test stays.
Relaxing that test to a tolerance would put a second copy of one
column in the tree and argue soundness around the drift, and nothing
needs it once the copies agree.

Dispatched on `iso/loft-seam-per-row-lane`. **Review tier: single
FULL** — a fit-contract change in `skin.rs` that moves every rational
loft's integral rows, so believing it takes more than reading it, but
it is one decision with a narrow footprint, not a broad architectural
one.

`non-separable-rational-interior-column` waits on a producer: nothing
in the tree mints such a column (its own `## The honest route`), so it
is not dispatched this sitting. It stays `open` rather than `parked`
because no item names the trigger; the first construction that mints
one is it.

Filed on TESS: `loft-of-three-or-more-sections-at-degree-one-is-unmeshable`
(P0), the measurement's off-ground finding. — (ISO orchestrator)

- 2026-10-23 — `iso-derivation-arms-assume-an-edge-spans-the-charts-whole-domain` (its IsoArc half) dispatched on `iso/arc-rim-sub-arc`. **Review tier: single FULL** — a new map in the arc-rim arm (sub-arc extent from the carrier's chart feet, `breaks` re-cut), whose certificate must still hold, so believing it takes more than reading it. No fork: the row names the measurement and the IsoLine half landed that way. — (ISO orchestrator)

- 2026-10-23 — PR 5629 review (single FULL, frozen `8ca0f5481a`): APPROVE-WITH-FIXES, no MAJOR. The core change was confirmed by execution (1,869 random cases with rational rows bitwise the old lane; the 38→0 census reproduced). The fix pass takes every item:
  - two test gaps, each shown by mutants that pass every committed test (a silent weight heal on the integral rows; `all`→`any` on the every-section rule);
  - the passenger row closes with the carrier;
  - two evidence errors in the new arc-bulge row;
  - the history prose in skin's docs.

  Orchestrator's call: a constant non-1 weight row also takes the Cartesian lane, with its constant attached. That closes the `skin.rs` bullet of `rational-gates-test-unit-weights-not-constancy` in passing. The "is integral" predicate is spelled differently across `patch_bound`, `curves` and `skin`, a class whose home is that row. — (ISO orchestrator)

- 2026-10-23 — IsoArc P1 PR 5630 is up and green. Departure from the row: a split child keeps its PARENT's `IsoArc` (the whole boundary row's arc), evaluated over its own span, rather than a re-cut sub-arc. The lane's argument is that `split_edge`'s carry already settles what the image is. The review is asked to judge that as a final state. Closed in passing: two SELECT rows (the span-in-arc certificate check, and a child certified through its parent's image). Filed on TESS: `a-split-arc-rim-meshes-no-chord-point-on-its-sub-arc-joints` (P3). Single FULL review dispatched on frozen `05c551e643`. — (ISO orchestrator)

- 2026-10-23 — PR 5630 review (single FULL, frozen `05c551e643`): APPROVE-WITH-FIXES.
  - **Certificate sound:** every mutant of the measured `at0` and angle reds; whole rims are unchanged across 7,766 CI tests.
  - **The class finding (both arms):** the iso schedules judge a closed-form candidate at ONE probe point. For each span there is one split start where the wrong whole-row map passes through the carrier exactly there. The closed form then wins and the mint refuses `LoopDiscontinuity`, a natural split included. It is sound, because certify refuses `MapResidual`. The fix pass gives both arms one judge that cannot coincide at a point.
  - **Also taken:**
    - a non-principal-branch fixture;
    - the volume assertion restored against the unsplit body (whose 1e-12 refusal is EXCH's row);
    - the describing door's reversed-u `Unimplemented` filed;
    - an explicit refusal for a polynomial row's zero angle;
    - one home each for the weight↔angle relation and the sub-arc count. — (ISO orchestrator)

- 2026-10-23 — **P0 closed:** PR 5629 merged at `6700652675`. The fix pass pinned the integral half (control points bitwise against a Cartesian reference) and the every-section rule by construction. A row of constant weight c now takes the Cartesian lane with c attached. That is the same surface in ℝ, and it moves the arc's middle row in identically-sectioned lofts by ulps; no stored bit moved. Its tracker-closing commit is not literally last: a lint-gate count fix followed it. Both reviews were already delivered, so nothing a reviewer must not see rode early.
- 2026-10-23 — `trimmed-quadrature-composite-rounds` dispatched on `iso/trimmed-composite-rounds`. QUADTAIL's `a-fitted-face-trimmed-by-a-section-has-no-volume-rule` moves here to ride with it, since it is the fixture that reaches the fence. **Review tier: DUAL** — a new convergent round schedule with a certified remainder (`F₂` via the chain rule through the chart's derivative grids) and its own lever in the round schedule is especially tricky logic, and a wrong enclosure there is silent. QUAD is live on `props/quad.rs` (the measure door, `PatchNet`), so the lane merges main often. — (ISO orchestrator)

- 2026-10-24 — **Seam (QUAD):** PR 5629 made `projectbox` wall 1 (the round-wire spring's volume) decide at the default ε, which reddened main at `debb2a604e`. QUAD retired the wall at the default ε, kept live, on `quad/projectbox-spring-certifies`. The number is sound against an independent quadrature of the body. ISO need not act on it. — (QUAD fix lane)
- 2026-10-24 — **Seam (QUAD):** the other moves PR 5629 made are re-pinned on PR 5640 too: the teapot spout's 1e-12 reading, the lens blade's bits and the thread-count digest's arc-loft lines. Every moved number holds an independent oracle. Three pads widened, by ulps up to +6.5e-10 relative, filed under QUAD (`a-per-row-cartesian-lane-widens-two-arc-loft-pads-by-ulps-to-6e-10`). — (QUAD fix lane)
