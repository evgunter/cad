# M5 PR 2 spec (binding): the C9 interval ring + hull-bound primitives

Status: BINDING. Deviations reported, never improvised. Authority:
CURVED-DESIGN C9 (ratified: the in-house ring, OQ8), C2.2 (hull
bounds are an ENTRY requirement for fitted-cache certification —
OQ2), C12.8 (no new `Real` methods); M5-PLAN PR 2 + R2.

## What this PR is

The default-build-path enclosure arithmetic that all M5 fitted-
cache certification stands on: a small MIT-clean interval RING
type in geom-core (±, ×, ÷ with outward ulp-widening — no
transcendentals, no rounding-mode fiddling), the `Bounds`-trait
seam so certification code is generic over which interval it got,
and the control-coefficient hull-bound primitives (per-span convex
enclosure of a spline in B-spline form — the Eq. 9.81 mechanism
C2.2/C9 cite). NO consumer rewiring beyond tests (certification
still runs T: Decide today; PR 4+ wires fitting/certification
through this), NO transcendentals (C9: every M5 enclosure is
ring-only), NO feature gating (this is the DEFAULT build path —
that is its reason to exist).

## Homes and shapes (binding)

- `geom-core::ring_interval` (name at implementer's discretion if
  something cleaner exists; NOT `interval` — that module is the
  Real-trait scalar behind the feature): `RingInterval` — plain
  struct { lo: f64, hi: f64 }, Copy, no PartialEq/PartialOrd
  (matches the linalg charter and the Interval scalar precedent).
  ALWAYS COMPILED (no cfg gate). This type is NOT a Real
  instantiation and must not implement Real — it is certification
  substrate, not an evaluation scalar; keep the two roles visibly
  distinct in module docs (the `interval` feature's scalar
  evaluates recipes; RingInterval bounds certification
  arithmetic).
- Construction: `point(f64)` (non-finite ⇒ poison), `from_bounds`
  (NaN/inverted ⇒ poison), `hull(a, b)`. Poison = NaN brackets
  (the Bounds convention: a poisoned bound can never certify —
  residual.hi() <= eps is false for NaN). No decoration channel:
  the ring has exactly two states, enclosure and poison, and
  poison flows through values only.
- Ring ops: Add/Sub/Mul/Neg/Div by outward ulp-widening
  (next_down/next_up around the RN result of each endpoint
  candidate; Mul via 4-corner min/max; Div refuses divisor
  straddling/touching zero ⇒ poison — certification never needs
  the half-line semantics, and poison-on-straddle is the honest
  conservative answer). Widening EVERY op is acceptable
  conservatism (C9: "sound, slightly conservative"); do NOT add
  exactness witnesses here in v1 — that is tightness optimization
  with its own proof burden, and interval-transcendentals already
  owns that craft; note the door in the module docs instead.
  `powi` via repeated squaring with the even-power lo >= 0
  clamp-to-zero-when-straddling rule (the memories/
  interval-square-poison lesson — even powers of straddling
  inputs have exact lower bound 0).
- `Bounds` impl (real.rs:376 — lo/hi with the NaN-poison
  convention). This is the seam: certification helpers written
  against `T: Bounds`-style access work for f64 (degenerate
  bracket), the feature-gated Interval scalar, and RingInterval.
  NOTE the existing `Bounds: Real` supertrait constraint —
  RingInterval must NOT implement Real (above), so either (a) add
  a new minimal trait (e.g. `Enclosure { lo, hi }`) in geom-core
  that `Bounds`-implementors get blanket-covered by, and the hull
  primitives consume `Enclosure`; or (b) relax `Bounds` — DO NOT:
  it is consumed as a Real-subtrait in tests. Ship (a); one-line
  blanket impl keeps every existing Bounds consumer working
  unchanged. Report the final trait shape.
- `geom-core::spline::hull` (new module beside basis/algebra):
  the hull-bound primitives over the PR 3 substrate —
  - per-span control-coefficient enclosure: for a spline in
    B-spline form with coefficient intervals (RingInterval per
    coefficient), the value enclosure over a span is the hull of
    the span's p+1 coefficients (partition of unity + convexity —
    positive weights invariant is what makes this true for
    rational forms in homogeneous coordinates; state the
    positive-weight precondition and take it from the validated
    types).
  - whole-domain enclosure = hull over spans; per-span function
    returning the span's enclosure so subdivision consumers (C3)
    get the granular form.
  - derivative-coefficient enclosure: hull bounds on the
    derivative spline's coefficients (knot-difference formula —
    structure-f64 divisors per PR 3's convention, coefficient
    arithmetic in RingInterval).
  - scalar-composite helper: given per-coefficient RingIntervals
    of a scalar spline (e.g. a residual composite f∘C sampled
    into B-spline coefficient form — the PR 4/7 consumers build
    these), the sup-norm bound over a span/domain. Keep the API
    shape data-in/bounds-out; do NOT build the f∘C composition
    itself here (that is PR 4's fitting-side work; building it
    now would guess its shape — the F5 lesson).

## Oracles and tests (the substance of this PR — be thorough)

1. Ring-op soundness fuzz (seeded, foreground): for each op,
   millions of random f64 pairs across magnitude windows
   (normals, subnormals, near-MAX, signed zeros, exact dyadics):
   the RingInterval result contains the exact rational result
   (integer mantissa/exponent comparison — the PR 1 review's
   comparator technique; cite it in the test header) and contains
   the f64 RN result. Poison paths: any NaN/inverted input ⇒
   poison out; div by straddling/touching-zero ⇒ poison.
2. Differential vs the two in-repo interval implementations:
   (a) the `interval` feature's scalar (dev only, cfg-gated test)
   and (b) interval-transcendentals' DInterval (dev-dependency of
   geom-core? NO — keep geom-core's dep tree clean: put the
   differential lane in a tests/-only sibling location if a
   dev-dep would drag anything; if a plain dev-dep on the
   path-crate is clean — it is libm-only — take it and report).
   Assert mutual containment consistency: RingInterval ⊇ their
   tighter results on shared ops.
3. Hull-bound honesty: for random splines (PR 3 constructors,
   degrees 1-5, adversarial weights within the positive
   invariant): dense-sample the spline; assert every sample lies
   within the span enclosure and the domain enclosure; assert the
   enclosure is not vacuously wide (width <= hull-of-coefficients
   width — the partition-of-unity tightness fact); a PLANTED
   out-of-hull perturbation (corrupt one coefficient after
   enclosure computation) is caught by re-check.
4. The C2.2 shape rehearsal (the acceptance the whole PR exists
   for): take a known closed-form case — the PR 3 circle-as-
   rational-quadratic vs Curve3::Circle — build the coefficient
   intervals of the DIFFERENCE composite's coordinates sampled
   into B-spline form at f64, hull-bound it, and verify the
   sup-norm bound is (a) sound vs dense sampling and (b) small
   (the two curves agree to fp noise — the bound must come out
   near fp scale, demonstrating hull bounds are USABLY tight for
   the exactness-adjacent cases PR 4 will certify). Document the
   observed bound magnitude honestly in the test.
5. D9: bit-identical repeated evaluation; no platform branches.
6. Battery: full workspace at 3ε default + interval lanes,
   clippy both ways, doc, fmt — all green; new module doc-clean.

## Out of scope

Transcendental pads (interval-transcendentals owns them); wiring
certification/fitting through the ring (PR 4/7); any Real trait
change; any feature-gate change; BVH cells (PR 8 consumes the
per-span form); tightness optimizations (exactness witnesses —
door noted, not built).
