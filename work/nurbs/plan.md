# NURBS — the plan

the spline, net and fit doors: knot insertion and splits, the net-state reads, wide-parameter evaluation, and fit doors that refuse for reasons that are not geometric

Opened 2026-09-20 by PROPS's priority-seam cut
(`work/README.md`, Track size). Worked 2026-10-09 to 2026-10-21; its
wave 1 and wave 2 rows are done (the log has each merge). Handed back
`ready` on 2026-10-21 under the stopping rule
(`memories/orchestration-model.md`): no P0 or P1 row is dispatchable.
On 2026-10-26 the tracker sweep merged KNOT (cut from FLUX onto the
same spline ground on 2026-10-10, nothing dispatched) into this track.

## The slate

**31.5 budget points** of dispatchable work against a ceiling of 30.
It is over by a point and a half because KNOT's rows are on this
track's own `geom-core/src/spline/*` ground and its `geom` callers; a
sitting that files more splits along the P2/P3 seam.

| pri | item | cost | what it is |
|---|---|---|---|
| P1 | `parametric-polygon-loop-certifies-nothing` | H | parked on the D10 hold (`d10-one-way-to-say-intent-is-unbuilt`) |
| P2 | `nurbs-interval-ders-at-a-wide-parameter-grows-with-translation` | M | `NurbsSurface::ders` at a wide `Interval` parameter carries the net's translation in its width |
| P2 | `project-eps-point-is-absolute-so-a-km-model-refuses-off-geometry` | M | `PROJECT_EPS_POINT` is absolute, below ulp(2048) |
| P3 | `net-state-reads-an-infinite-net-as-described` | M, design | `NetState` reads a net of `±∞` as `Described` |
| P3 | `nurbs-curve-has-no-net-state-door` | M | the curve has no three-state door |
| P3 | `nurbs-curve-interval-eval-loses-the-partition-of-unity-over-a-wide-t` | M | a curve's eval at a wide `t` keeps no `Σ Nⱼ = 1` |
| P3 | `equal-split-points-insert-a-value-twice-on-a-narrow-span` | M | the split schedule's rounding refuses a narrow span |
| P3 | `elevation-past-degree-ten-agrees-to-1e-11-with-no-bound` | — | design: refuse past the estimated bound, or carry the estimate |
| P3 | `implicit-composite-refuses-a-net-whose-weights-sit-near-the-bottom-of-f64` | E | read the composite on `unit_scale_exact`'s weights |
| P3 | `interpolate-with-params-passes-a-non-finite-point-through-as-nan` | E | `check_params` refuses no non-finite point |
| P3 | `two-reversal-policies-for-a-spline-net` | E | two "run it backwards" doors under two rules |
| P4 | `a-pair-walk-end-at-a-domain-corner-refuses-undecided` | H | a corner root has no reading of its own |
| P4 | `net-refinement-copies-the-differencing-skeleton-and-the-plan-ratio-is-optional` | M | four per-line loops; `Step::Combo.ratio` optional |
| P4 | `the-approximation-refit-solves-banded-normal-equations-densely` | M | dense Cholesky on a banded system |
| P4 | `unordered-fit-parameters-refuse-as-a-count-mismatch` | E | `check_ordered` refuses as `ParamCountMismatch { n, n }` |
| P4 | `ring-refinement-inserts-one-point-per-plan-over-the-whole-net` | — | deferred |
| P4 | `the-integral-arm-reads-below-the-rational-arm-on-an-equal-weight-net` | — | deferred |
| P4 | `the-integral-meters-piece-refinement-is-not-monotone` | — | deferred |

## Order

- The two P2 rows first. Both are a door answering for `f64`
  resolution or an enclosure's dependency rather than the geometry, on
  a model a caller can build (a km-scale part; a wide parameter); the
  `ders` row's first question is whether the evaluation lane owes a
  translation-invariant enclosure at all, or is a thin-parameter door
  that should say so. Its curve-side sibling is the partition-of-unity
  row, which wants the same answer.
- `net-state-reads-an-infinite-net-as-described` (design) and
  `nurbs-curve-has-no-net-state-door` are one question and want one
  designer pair.
- `equal-split-points-insert-a-value-twice-on-a-narrow-span`: its item
  carries the repair shape and what moves (two refusal arms become
  bug-only, the seed arm's door row moves).
- The elevation row is a design fork. Its item records step 0's
  measurements (the certified ring bound is unusable past degree about
  4; the first-order estimate tracks within 10–10³) and the ruling that
  the bound is the estimate, worded "estimated". Whether a disagreement
  past it refuses or carries goes through the two-designer weighing
  before Ev.
- The fit rows (`interpolate-with-params…`, `unordered-fit-parameters…`,
  `the-approximation-refit…`) are independent of the rest; the two E
  rows are one short lane.
- The corner row waits on a consumer; no current row reaches a corner
  end.
- The polygon row stays parked until the D10 hold lifts. Its fix site
  is `editor-core/src/drive.rs`, not the spline layer, and its
  measurements were taken on `Expr::param`, which stage 1 retired: the
  first step is to re-take them, and to re-home the row with them.

## Review posture

The repo-wide tiers (`memories/orchestration-model.md`). Each unit's
tier and reason are recorded in the log at dispatch. KNOT's posture
holds for its rows: a spec requires a decide where it would otherwise
assert a premise, and a narrower bound is not evidence a width fix is
right; containment, tested against exact arithmetic, is.
