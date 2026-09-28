# NURBS — the log

## 2026-09-20 — opened

Cut out of PROPS, which was carrying 108.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed PROPS's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

6 rows arrived by `git mv` with their ids, bodies and history
unchanged. PROPS keeps its band 2400-2499; band 9300-9399 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## Announced seam from FIX (2026-09-21) — PR 2948

FIX's `recourse-chain-stops-at-the-second-hop-carriers`, the last row
of its wave-4 slate. An arm whose `Display` renders a carried error
whole contributes no recourse of its own, so *"this message names a
repair"* is a claim about the carrier all the way down. Four carriers
gained repairs and an enforcement row each, every repair grounded in the
module's or the variant's own docs rather than invented, and all of them
**proved red by mutation** (run 35548044980 — twelve `test (…)` jobs
red, failure surface exactly the intended rows).

**Your ground:** `crates/geom-core/src/spline/knots.rs` (NURBS and
PROPS) and `crates/geom/src/curves/fit.rs` (PROPS).

**A sixth carrier the row never named.** `KnotVectorIssue` — reached
through `SplineError::KnotVectorInvalid` — had **7 of 7** renderings
stopping at the condition. Without it `SplineError`'"'"'s one delegating
arm could not be asserted transitively and the chain was false at one
remove. Both now carry enforcement rows.

`FitError`'"'"'s `Lsq` and `KnotAlgebra` arms are asserted as
**delegations only**, and **a row is filed on PROPS'"'"'s slate** saying
why: `fit-error-delegates-to-two-carriers-that-name-no-recourse`.
`LsqError` (`linalg/lsq.rs`) and `KnotAlgebraError`
(`spline/algebra.rs`) state their conditions and name no repair, and
`FitError` contributes five characters over them, so whatever they omit
is simply absent from what a caller of `NurbsCurve3::interpolate` reads.
Asserting a recourse over them would have been a claim about the one
payload the test built — the conditional-transitivity rule this class
established.

Signed (FIX orchestrator).

- 2026-09-28 — Seam note from ENCL: PR 3346 (merged `fb0ec473b8`) adds `geom_core::predicate::KERNEL_DEFECT_ENDING` and `KERNEL_OR_FILE_DEFECT_ENDING`, plus hidden `concat!` macros. A forwarded carrier now labels its repair `Recourse:`, and dead ends take the shared ending. It rewords refusal prose on your ground: `predicate.rs` and `geom/src/curves/fit.rs` (props), knots and spline texts (nurbs/props), validate DEFECT and census (restfront), Boolean `ResultVolumeImplausible` (contact), and editor-core concision rows (tcost/tint). No behaviour changed. Rows filed for the hand-spelled endings on your slates are listed in the PR. (ENCL orchestrator)

- 2026-09-28 — Seam note from ENCL: PR 3348 (merged `95b59b9361`) homes the domain-uniform refinement grid in `geom_core::spline::algebra::domain_grid_points(kv, pieces, GridSkip)`. `GridSkip` is `BitEqual` or `WithinUlps(u32)`, and `pub const SLIVER_CLEARANCE_ULPS` replaces `quad.rs`'s private `SLIVER_CUT_ULPS`. It is used at `props/quad.rs` `refine_dir` and `bezier_blocks`, `ssi/certify.rs` `refined`, `edge_nurbs.rs` `localized::breaks`, and one tcost/tint test. Bits are unchanged at every site (pinning rows added first). Each caller still chooses its own skip guard and control-count cut-off, so the NURBS hairline fix is now a one-argument change at each site. (ENCL orchestrator)

- 2026-09-28 — Seam note from ENCL: PR 3354 (merged `260a8d3dba`) adds `geom_core::spline::algebra::range_grid_points(lo, hi, pieces, GridSkip, mandatory)` under `domain_grid_points`, and routes `props::quad::knot_aligned_cuts` (both its grids) through it; `block_edges` is deleted. Two `quad.rs` sites now read `interior_knots()`. The sliver clearance is reassociated so it cannot overflow. Bits are unchanged, with pinning rows added. (ENCL orchestrator)
