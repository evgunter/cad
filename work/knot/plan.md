# KNOT — the plan

The spline, net and fit doors: refinement inside an enclosure, the net-state reads, and the NURBS and fit doors that refuse for reasons that are not geometric.

Opened 2026-10-10 by FLUX's priority-seam cut (`work/README.md`,
Track size), when FLUX measured 125.5 budget points against 30. FLUX
kept the curved closed-form arms. Nothing dispatched.

## The slate

**24.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | status | title |
|---|---|---|---|---|
| P2 | `f64-refinement-inside-an-enclosure-has-five-more-sites` | H | open | An f64 knot refinement (or an f64-rounded insertion ratio) inside an enclosure: five more sites of TESS-2's class |
| P2 | `nurbs-interval-ders-at-a-wide-parameter-grows-with-translation` | M | open | geom: NurbsSurface::ders at an Interval parameter wider than a point assembles the rational quotient in the absolute frame, so its derivative enclosure grows with the net's translation (0.26 wide at the origin, 54 at 100 m, 5.4e4 at 1e5 m) |
| P2 | `project-eps-point-is-absolute-so-a-km-model-refuses-off-geometry` | M | open | PROJECT_EPS_POINT = 1e-13 m is absolute and below ulp(2048), so NurbsSurface::project refuses inconclusive on a km-scale model for a reason that is not geometric |
| P3 | `interpolate-with-params-passes-a-non-finite-point-through-as-nan` | E | open | geom fit: interpolate_with_params and interpolate_on take a non-finite point and return NaN control points instead of NonFinitePoint |
| P3 | `net-state-reads-an-infinite-net-as-described` | M +design | open | NetState answers Described for a net of infinities, so only tier-3 check 1 refuses it and every other three-state consumer takes it as geometry |
| P3 | `nurbs-curve-has-no-net-state-door` | M | open | NurbsCurve has no net_state twin, so curve-side placeholder-or-described reads were never swept for a poisoned net |
| P3 | `surface-knot-insertion-writes-weights-validate-counts-refuses` | M | open | NurbsSurface knot insertion writes weights validate_counts would refuse (a subnormal net refines to a zero weight) |
| P4 | `net-refinement-copies-the-differencing-skeleton-and-the-plan-ratio-is-optional` | M | open | geom-core: refine_u/refine_v copy diff_u/diff_v's per-line skeleton, and Step::Combo.ratio holds an invariant by convention |
| P4 | `the-approximation-refit-solves-banded-normal-equations-densely` | M | open | geom fit: approximate's least-squares refit builds dense rows and solves banded normal equations by dense Cholesky |
| P4 | `unordered-fit-parameters-refuse-as-a-count-mismatch` | E | open | geom fit: parameters that are not clamped and ascending refuse as ParamCountMismatch { params: n, points: n } |

## Order

`net-state-reads-an-infinite-net-as-described` (design) and
`nurbs-curve-has-no-net-state-door` are one question and want one
designer pair. `f64-refinement-inside-an-enclosure-has-five-more-sites`
is the hard row; two of its sites are QUAD's ground and one is CHORD's,
so it travels with announced seams. The fit rows
(`interpolate-with-params…`, `unordered-fit-parameters…`,
`the-approximation-refit…`) are independent of the rest. NURBS holds
the certified Boehm step and the projective applier; coordinate with it
on `geom-core/src/spline/*`.

## Review posture

FLUX's, inherited: the review tiers of `memories/orchestration-model.md`
(orchestrator's read, single style or full review, or a dual under
`docs/DUAL-REVIEW-PROTOCOL.md`, logged in `docs/DUAL-REVIEW-LOG.md`),
named with its reason at dispatch. FLUX's posture holds here too: a
spec requires a decide where it would otherwise assert a premise, and a
narrower bound is not evidence a width fix is right; containment,
tested against exact arithmetic, is.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`,
so the program closes without an exit walk.
