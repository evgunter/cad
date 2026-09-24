# EXCH — the plan

exchange: STEP and STL

Re-scoped 2026-09-20 by EXCH's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**24.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `torus-rim-mint-abandons-a-half-applied-split` | H | The torus rim mint bails with Ok(()) after `split_at_midpoint` has already mutated the solid |
| P1 | `recognize-normalizes-without-a-length-decision-and-cannot-mint-the-witness` | D | step-import recognize.rs normalizes a plane normal with no length decision and plus_zero's it, so it cannot mint the unit witness — the bare Vec3::orthonormal_basis stays for it |
| P1 | `step-import-circle-promotion-has-no-map-obligation` | D | the circle limb certifies locus and closure, not the map: a re-timed closed carrier promotes on locus alone |
| P1 | `step-import-eps-in-ambient-two-dial-strand` | D | recognition promotes at eps_in while selection and certify run at ambient: a column bent between the dials strands its edge |
| P3 | `arc-rim-gate-reports-a-degenerate-carrier-as-an-infinite-residual` | E | step-import RimOffWallBoundary's residual is a measurement type carrying a non-measurement sentinel |
| P3 | `import-normalizes-the-rim-only-cap` | D | STEP import re-mints a rim-only sphere cap into the seamed form, as a reported normalization |
| P3 | `step-import-curve-recognition-named-exclusions` | H | step-import stage-1 curve recognition — the NAMED EXCLUSIONS (open arcs, ellipse, helix) and the surjectivity certificate |
| P3 | `step-scaffold-strut-offset-is-absolute-in-a-unit-free-format` | D | step-import mints its scaffold strut at a fixed 1.0 offset, in a format whose coordinates carry no unit contract |
| P4 | `chart-review-fuzz-frame-hand-rolls-the-helper-axis-cone` | E | chart_review_fuzz's frame() hand-rolls the |x| < 0.9 helper-axis cone instead of calling Vec3::orthonormal_basis |
| None | `D343` | None | Sweep Class B (typed payloads through Debug) over the two STEP crates as one lane, with two riders |
| None | `EXCH-E1` | None | D343 — typed payloads stop rendering through Debug in the two STEP crates, with its two riders |
| None | `EXCH-E2` | None | the step-import chart-coherence consumer — findings reach the importer's diagnostics |
| None | `EXCH-H1` | None | degree-1 line promotion and the ExtrudedPoint rung in nurbs_iso_derive |
| None | `coherence-findings-have-no-step-import-consumer` | None | the step-import diagnostics half of the coherence consumer: examine_chart_coherence at the door where defective source coordinates actually arrive |
| None | `epsilon-has-no-type-of-its-own` | None | Epsilon has no type of its own, so StepOptions and step-import restate Tolerance::init's rule by hand (S4's shape) |
| None | `step-import-degree-one-line-promotion` | None | step-import — promote degree-1 NURBS carriers to Curve3::Line, needs an ExtrudedPoint rung in the NURBS-chart pcurve lane first |
| None | `step-writer-hardcodes-user-header-fields` | None | The STEP writer still hardcodes two Part 21 header fields the standard assigns to the user |
| None | `stl-header-refuses-plausible-names` | None | A plausible part name is a hard panic in both demos — StlOptions::header refuses solid-block and anything over 80 bytes |

## Order

`torus-rim-mint-abandons-a-half-applied-split` first: it bails with
`Ok(())` after `split_at_midpoint` has already run, so the failure is
silent AND leaves the body half-modified — the worst pairing on this
slate.

Then the recognition chain in dependency order:
`recognize-normalizes-without-a-length-decision-and-cannot-mint-the-witness`,
then `step-import-circle-promotion-has-no-map-obligation`, then
`step-import-curve-recognition-named-exclusions`, which is the
open-arc/ellipse/helix breadth the charter names and the largest unit
here.

## Review posture

OPEN, for this program's first dispatch. EXCH inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
