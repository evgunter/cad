# SPANFOLD — the plan

Opened 2026-10-27 by the re-cut of FLUX and FLUXTAIL. Together they
held 95.5 budget points against 30, and both shared
`props/curved.rs` and `props/spiric.rs`, so their orchestrators
could not run side by side. The rows were re-cut by logical thread
instead of by priority, so that the five successors can run in
parallel: each mostly owns its own files, and where two share a file
they work on different functions of it
(`docs/doc-ledger/flux-leaves-the-tracker.md`). Nothing dispatched.

## The slate

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

## Order

The two `f64::min` fold rows (`piece-monotone-span-drops-a-refused-step-through-f64-min`, `trim-piece-monotone-span-fold-drops-a-refused-window`) are one fix in `trimmed.rs` and should land as one unit, with `trim-walk-chord-lengths-are-rooted-to-nearest` beside them. The two `loop_area` rows batch together. `a-cylinder-split-just-off-its-rim-vertex-…` is the P2 and leads; re-measure it first, since PR 5741 changed what a quadrature stop reports. `the-gating-corpus-reaches-no-collapsed-arm-gate` gives this class its witness rows; build it alongside the fold fix, so that the fix has rows that go red without it.

`props/quad/` is QUAD's ground. Read `work/quad/plan.md` before dispatching, and announce the seam.

## Review posture

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

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`,
so the program closes without an exit walk.
