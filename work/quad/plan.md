# QUAD — plan

Live state is `work/quad/log.md`'s tail and the item files beside
this plan, never this file. Branch prefix `quad/`; away-channel tag
`(QUAD orchestrator)`; A/B band **5700-5799**, claimed in
`docs/MODEL-AB-LOG.md`'s banding entry at the cut.

## Lanes

- **The engines** (the structural half): `C3` splits the four
  quadrature engines and de-triplicates the convergence block; `D30`
  is the second span search and the second index clamp that exist
  because `KnotVector` could not answer the question — read `D30`
  first, since what it wants from `KnotVector` may decide how `C3`
  splits.
- **The premises**: `quad-face-extent-trusts-caller-perimeter` (a
  caller-supplied perimeter that is sometimes not an upper bound) and
  `quad-last-round-margin-has-no-sign-premise` — both are "nothing
  states that this cannot happen" rows on the same engine, and they
  want measuring together.
- **The refusal's shape**: `quadrature-budget-conflates-its-lanes-and-budgets`
  (six sites, three lanes, three budgets, told apart by a payload
  convention) with `quadrature-budget-prints-its-two-lengths-alike` as
  its rider. The PROPS budget-faces unit is the worked precedent for
  this exact shape on a sibling enum — read its `## Closed` section in
  `work/props/budgetexhausted-conflates-three-terminations.md` before
  specifying.
- **The dial** (`[ev]`, ask early):
  `quad2-rational-max-rounds-dial-decision`.
- **Frame dependence**: a certified width or a verdict that moves when
  the body is translated or rotated. Ev's criterion (in chat,
  2026-10-21) is that such a row is P1 when it was encountered
  naturally, that is, when it actually blocked something.
  Every row left in it was found by probes and sweeps:
  `quad-rational-patch-cross-grows-with-translation` (P2),
  `quadrature-interval-floor-grows-with-the-body-past-the-band` (P2),
  `a-translated-arc-prism-panics-the-area-gauge-through-mass-properties`
  (P2) and `quad-area-pads-read-the-cross-norm-off-per-coordinate-boxes`
  (P3). Any of them rises to P1 once something real meets it.
  INTENT stage 3 E's test 13 (`docs/INTENT-STAGE3-SPEC.md`) computes
  every corpus operation in a second, rotated and translated frame and
  expects the same verdicts, so these rows sit on that P0's path.
  Most were measured before FLUX's `shell_reference` (2026-10-20) moved
  the props fold's flux origin to a vertex of each shell, so re-measure
  every one of them on current main first.
- **The P2–P4 tail** moved to QUADTAIL (2026-10-21), including
  `purchasable-area-tightness-valve`.

## Review posture

Not yet set. The parent program ran full v6 duals on kernel units
with E riders single-reviewed; whether that is right here is the
orchestrator's call at its first dispatch, and Ev took S-TCOST off
the protocol entirely on 2026-09-12, so the question is live.

## Exit shape

Every item above landed or ruled; the walk convention applies.

## From QUADTAIL (merged 2026-11-20)

Live state is `docs/doc-ledger/quadtail-and-spanfold-merge-into-quad.md`'s tail and the item files beside
this plan, never this file. Branch prefix `quadtail/`; away-channel tag
`(QUADTAIL orchestrator)`.

Opened 2026-10-21 by QUAD's priority-seam cut (`work/README.md`, Track
size). Nothing dispatched.

### Ordering notes

- Several rows were measured before QUAD's P1 rows land (C3's engine
  split, the frame-dependence rows). Re-measure a row on current main
  before specifying it: the widths may have moved.
- `purchasable-area-tightness-valve` is `deferred` (S-CERT Q1, Ev,
  2026-08-29): it waits for a consumer asking for a tighter area, and
  is not to be built ahead of one.
- `patch-face-area-gauge-panics-on-random-bicubic-faces` is the same
  tripwire QUAD's `an-integral-patch-with-knots-hugging-its-edges-trips-the-area-gauge`
  (P2) reaches. A fix there, where the lane refuses typed before the
  A2 gauge fires, may close this row too, so re-run its probe first.
- `a-uniformly-scaled-rational-patch-refuses-the-quadrature-as-a-straddling-weight`
  has its fix written (`geom_core::spline::weight_scale::unit_scale_exact`).

### Review posture

The repo-wide tiers (`memories/orchestration-model.md`), recorded per
unit at dispatch. The v7 dual-triage question is OPEN for the first
dispatch, as it was for QUAD.

### Exit shape

Every item landed or ruled. The plan sets no exit criteria, so no walk
is owed.

## From SPANFOLD (merged 2026-11-20)

Opened 2026-10-27 by the re-cut of FLUX and FLUXTAIL. Together they
held 95.5 budget points against 30, and both shared
`props/curved.rs` and `props/spiric.rs`, so their orchestrators
could not run side by side. The rows were re-cut by logical thread
instead of by priority, so that the five successors can run in
parallel: each mostly owns its own files, and where two share a file
they work on different functions of it
(`docs/doc-ledger/flux-leaves-the-tracker.md`). Nothing dispatched.

### The slate

**15.5 budget points** of dispatchable work against a ceiling of 30, in 8 rows (2026-10-27).

| pri | item | cost | title |
|---|---|---|---|
| P2 | `a-cylinder-split-just-off-its-rim-vertex-exhausts-the-volume-quadrature-at-rest` | M | A cylinder split by a plane tilted 1e-2 just off its seam rim vertex leaves a sliver whose wall's volume quadrature stops at its floor over the target at 1e-12, so the split refuses at rest |
| P3 | `a-loop-area-nurbs-segment-integrates-an-inverted-window-as-zero` | E | props loop_area: a NURBS boundary piece with t0 > t1 integrates to zero per span (half_len.max(0)) where the signed integral is negative |
| P3 | `a-loop-area-spline-carrier-past-the-gauss-inventory-refuses-with-no-composite-arm` | M | loop_area refuses a non-rational B-spline carrier of degree > 5 at the end of its exact Gauss inventory, with no composite fallback |
| P3 | `a-rational-rim-with-a-near-zero-weight-is-not-live-on-equal-pieces` | M | A rational rim with a weight near zero is not live on equal pieces: its 1/w² peaks over a sliver of the parameter no schedule resolves |
| P3 | `piece-monotone-span-drops-a-refused-step-through-f64-min` | E | props/quad/ piece_monotone: the span fold's f64::min drops a refused step's NaN, so the span is the other steps' minimum |
| P3 | `the-gating-corpus-reaches-no-collapsed-arm-gate` | M | The gating corpus reaches no collapsed-arm gate: five of the eight predicates have zero rows, three have only positive |
| P3 | `trim-piece-monotone-span-fold-drops-a-refused-window` | E | props_trim_piece_monotone folds its span with f64::min, so a refused chord window drops out of the margin instead of escalating it |
| P3 | `trim-walk-chord-lengths-are-rooted-to-nearest` | M | The trim walk's chord lengths are f64 roots to nearest, read as a pad, a refusal threshold and a direction |

### Order

The two `f64::min` fold rows (`piece-monotone-span-drops-a-refused-step-through-f64-min`, `trim-piece-monotone-span-fold-drops-a-refused-window`) are one fix in `trimmed.rs` and should land as one unit, with `trim-walk-chord-lengths-are-rooted-to-nearest` beside them. The two `loop_area` rows batch together. `a-cylinder-split-just-off-its-rim-vertex-…` is the P2 and leads; re-measure it first, since PR 5741 changed what a quadrature stop reports. `the-gating-corpus-reaches-no-collapsed-arm-gate` gives this class its witness rows; build it alongside the fold fix, so that the fix has rows that go red without it.

`props/quad/` is QUAD's ground. Read `work/quad/plan.md` before dispatching, and announce the seam.

### Review posture

FLUX's, inherited: the review tiers of `memories/orchestration-model.md`
(orchestrator's read, single style or full review, or a dual under
`docs/DUAL-REVIEW-PROTOCOL.md`, logged in `docs/DUAL-REVIEW-LOG.md`),
named with its reason at dispatch. Expect a dual on a row that changes
what the kernel will certify over a family: a wrong closed form is the
error that reads as an improvement.

What PROPS and FLUX learned, stated so that briefs carry it:

- **A spec that asserts a premise hands the unit a defect.** One
  premise about `du_of_rims` failed three times running in PROPS, the
  last time on main. A spec here requires a decide.
- **A re-baselined golden is not a re-pointed row.** Check what a row
  witnesses before moving its constants: its subject can move under a
  change while its numbers stay movable.
- **A narrower bound is the expected outcome of a width fix, and
  therefore not evidence that the fix is correct.** Containment is.
  Test against exact arithmetic, with cases chosen to break it.

### Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`,
so the program closes without an exit walk.
