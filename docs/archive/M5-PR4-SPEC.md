# M5 PR 4 spec (binding): NURBS substrate part 2 — projection, fitting, LSQ

Status: BINDING. Deviations reported, never improvised. Authority:
CURVED-DESIGN C11 (§6.1 projection, §9.4 fitting stack), C6
(f64-structure / generic-certification split), C2.1/C2.2 (foot
points with certified orthogonality; hull-bound entry requirement
per OQ2), C12.8 (LSQ lands with its first consumer — here);
M5-PLAN PR 4. Dispatch gate: PR 2 (ring + hull) merged.

## What this PR is

The fitting side of the NURBS substrate: (a) the small in-house
LSQ solver joining `geom-core::linalg`; (b) point
projection/inversion (Book §6.1) with certified orthogonality
residuals; (c) the global curve-fitting stack (LSQ interpolation/
approximation Eqs. 9.63–9.67; the bounded Type-2 loop A9.10) for
3-D curves AND 2-D pcurves under C6's pinning rule; (d) the
reusable ring-coefficient COMPOSITE builder that PR 2's rehearsal
prototyped (Bernstein-product algebra), promoted from test-local
code to certification substrate — this is what makes "fit, then
certify with a hull bound" a one-call story for PR 5/6/7.

NOT in this PR: SSI/marching (PR 7), surface fitting (no M5
consumer — C11's deliberate absence), pcurve STORAGE/wiring (PR
6), certificate TYPES on bodies (the consumers own their
certificates; this PR provides the machinery + rehearsal-grade
acceptance), any new Real methods (C12.8 stands).

## (a) LSQ in geom-core::linalg (C12.8, first consumer)

- New module `linalg::lsq`: dense small least squares for the
  fitting systems (N^T N banded-ish; the Book's Eqs. 9.65–9.67
  shape). In-house, D9: FIXED elimination order, no
  magnitude-based pivoting (fixed-shape systems; a
  numerically-degenerate system is a typed refusal
  `LsqDegenerate`, never a reorder), no allocation surprises
  (Vec-based, size from inputs), f64 only (structure machinery —
  this is C6's f64 lane; say so in module docs).
- The linalg module-doc charter applies (no PartialEq/Ord, fixed
  association documented per fold, totality/poison or typed
  refusal). Banded exploitation optional; correctness first,
  document the choice.

## (b) Point projection / inversion (§6.1, C2.1)

- `NurbsCurve{2,3}::project(point) -> Result<Projection, ...>`:
  Newton on the orthogonality condition with D9-fixed iteration
  policy (fixed max iterations, fixed convergence thresholds as
  named constants, seeded from a fixed-count parameter sweep —
  document the seeding rule; no data-dependent iteration ORDER).
  f64 structure machinery.
- The Projection carries its own **certified orthogonality
  residual**: |C'(t*)·(C(t*)−P)| (and the distance), so a bad
  projection cannot launder a bad cache (C2.1 verbatim) — the
  consumer re-checks the residual through its own Decide/band
  machinery; this PR pins the value's presence and honesty
  (planted bad-seed test: a deliberately mis-seeded Newton that
  converges to a wrong branch yields a residual that FAILS the
  band — fixture required).
- Non-convergence is a typed refusal (`ProjectionInconclusive`),
  never a best-effort answer.

## (c) The fitting stack (§9.4, A9.10 shape, C6)

- `fit` entry points for 3-D curves and 2-D pcurves (one generic
  core over the point dimension — the NurbsCurve macro precedent):
  interpolation (A9.1-shape, exact solve) and
  approximation-with-bound (the Type-2 loop: fit low degree,
  knot-remove under the PR 3 removal bound, degree-elevate,
  refit — A9.10's shape with OUR bound semantics, not Eq. 9.77's
  data-deviation bound; cite the C1 rung-3 note: the Book's bound
  steers the iteration, the C2 certificate against the locus is
  the consumer's).
- C6 verbatim and binding: every branch in the fit loop (knot
  counts, degrees, convergence) is f64 STRUCTURE selection —
  deterministic (D9: same inputs ⇒ same knots/degrees bit-wise),
  never routed through k_stats (no topology decision exists
  here), never consulted by topology (cache shape is invisible to
  predicates).
- The loop's failure mode is typed (`FitBudgetExhausted` with the
  achieved bound), never silence, never an unbounded iteration
  (fixed budget as a named constant) — the Book's own "both can
  fail to converge and this eventuality must be dealt with"
  honesty.

## (d) The composite builder (promoted from PR 2's rehearsal)

- New `geom-core::spline::compose` (or extend `hull`; implementer
  chooses, reports): given a NURBS curve's structure and
  ring-coefficient control data, produce ring-coefficient
  B-spline/Bernstein form of polynomial composites — at minimum:
  products of coordinate splines, linear functionals, and the
  quadric implicit composites f∘C for f ∈ {plane, sphere,
  cylinder, cone, torus IF its implicit is polynomial in the
  point (it is, degree 4)} — enough for PR 5/7's certification
  shapes. Data-in/bounds-out; the PR 2 rehearsal
  (hull_circle_rehearsal.rs) refactors ONTO this module (test
  churn there is expected and in-scope; results must reproduce
  bit-identically or the delta explained).
- Sup-norm certification story, end-to-end acceptance: fit a
  NURBS approximation to a KNOWN locus sample set (e.g. a
  quarter-arc at deliberately coarse tolerance), compose its
  implicit residual, hull-bound it, and verify (i) the bound is
  sound vs dense sampling, (ii) a fit whose Type-2 loop stopped
  at bound B has hull-certified residual consistent with B's
  order, (iii) a PLANTED between-samples excursion (corrupt one
  control point after fitting) is caught by the hull bound while
  a 9-point schedule misses it — OQ2's argument as a standing CI
  pin (PR 2 measured 59% invisibility; this pins one concrete
  case).

## Oracles / tests

Closed-form loci (circle as rational quadratic; line; planted
perturbations thereof); curvo as interpolation oracle ONLY per
docs/CURVO-AUDIT.md (dev-dep, pinned commit, tolerant comparison,
NOT for approximation/bounds); num-dual where derivative checks
help. Battery: full workspace 3ε default + interval lanes, clippy
both ways, doc, fmt; bit-replay: fitting the same data twice ⇒
identical structure and bits (D9 pin); the projection/fit suites
join existing CI lanes (no new hosted rows expected — confirm).

## A/B note

Row 14, difficulty L (logged pre-assignment), arm = fable
(block-5 remainder).
