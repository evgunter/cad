# curvo audit (C12.9 / DESIGN.md Q5)

- **Audited**: `mattatz/curvo` @ `47d19d560bf301108804dc43f933cb80381e76dd` (2026-06-25), MIT.
- **Auditor**: M5 S3 lane, 2026-07-27. Source-only review of the clone at
  `~/.local/share/cad-work/curvo-audit/curvo`; GitHub issue tracker not consulted (offline) —
  correctness track record below is inferred from source + history only.
- **History**: 836 commits, 2024-04-18 → 2026-06-25, essentially single-author (mattatz 787 + 47
  under his full name). Active and healthy as an upstream.

## Headline correction to DESIGN.md's crate-landscape row (APPLIED — DESIGN.md's row and its Q5 entry both carry the corrected scope; kept as the evidence record)

The pre-audit row claimed curvo offered NURBS "incl. SSI" and trimming.
**curvo has no surface–surface intersection.** `src/intersects/` covers curve–curve,
curve–plane, surface–curve, surface–plane, and mesh–plane only; `src/marching/mod.rs` is an
empty one-line placeholder. Its "booleans" (`src/boolean/`) are **planar 2-D curve/region
clipping**, not solid booleans, and contain a reachable `todo!()` (`src/boolean/clip.rs:330`,
`Status::None` arm). Its "trimming" is curve parameter-range trimming (`src/trim/mod.rs`) plus
trimmed-surface *tessellation*. So the most SSI-shaped thing to vendor does not exist; the Q5
question is narrower than the landscape row implied.

## Findings by area

### (a) NURBS foundation (basis eval, knot algebra, degree elevation)

- **Basis/eval quality**: textbook and correct-looking. `KnotVector::basis_functions`
  (`src/knot/knot_vector.rs:306`) is A2.2; derivative bases follow A2.3; curve/surface eval and
  rational derivatives (A4.2-shape) in `src/curve/nurbs_curve.rs` / `src/surface/nurbs_surface.rs`
  match The NURBS Book. Per-call `Vec` allocation throughout (no fixed-shape hot path).
- **Hidden epsilons gate decisions**, exactly as Q5 predicted: span lookup fuzzes both domain
  ends by `T::default_epsilon()` (`knot_vector.rs:202`); knot multiplicity clustering compares
  `|k_i − k_j| > default_epsilon` (`knot_vector.rs:153`) — a topology-adjacent equality decision
  with a buried machine-eps threshold; knot refinement gates an alpha on `default_epsilon`
  (`nurbs_curve.rs::try_refine_knot`).
- **Knot insertion** (`try_add_knot`, `nurbs_curve.rs:1498`): in-place Boehm variant, but the
  correction's homogeneous **weight component is explicitly zeroed** (`p[D::dim()-1] = 0`) —
  a deviation from the textbook homogeneous treatment I would not trust for rational curves
  without independent verification. **Refinement** (`try_refine_knot`, :1793) is A5.4 but has a
  silently-skipped out-of-bounds branch literally commented `// TODO: resolve this issue` —
  fail-quiet, the opposite of our contract. **Removal** (`try_remove_knot`, :1575) is A5.8 with
  a hardcoded default `1e-6` acceptance tolerance measured as a **norm in homogeneous space**,
  not the Tiller geometric bound of Eq. (5.30); the knot itself is located by bitwise `==`.
  **Degree elevation** (`try_elevate_degree`, :1312) is A5.9 via Bézier decomposition, textbook
  shape, the cleanest of the four. No surface-side knot removal or degree elevation.
- **Genericity**: generic over `FloatingPoint = nalgebra::RealField + ToPrimitive + Copy`,
  implemented **only for f32/f64** (`src/misc/floating_point.rs`). Pervasive
  `T::from_f64(..).unwrap()`, `default_epsilon()`, and RealField transcendentals (std-routed,
  not libm) mean it cannot host our `Real` trait (interval/dual scalars, bit-determinism) —
  the same disqualifier that demoted num-dual at M0.

### (b) Fitting vs A9.10 needs

`src/interpolation/` provides global curve **interpolation** only (A9.1-shape): chord/centripetal
parameterization, averaged knots, dense collocation matrix solved by nalgebra LU
(`src/interpolation/curve.rs:294` — no banded exploitation), plus a periodic variant. Surfaces get
constructive loft/sweep/revolve (`nurbs_surface.rs:752,1195,1239`), not fitting. **There is no
approximation-to-tolerance, no least-squares fitting with error bounds, no A9.10 stack at all**,
and `try_reduce_knots` (curve only) inherits the uncertified 1e-6 removal gate. Our
f64-structure/generic-certification fitting split has nothing here to vendor.

### (c) SSI

Absent (see headline). The nearest analog — curve–curve intersection
(`src/intersects/curve_curve/`) — is architecturally the same shape we planned: AABB-tree
subdivision seeding (convex-hull property gives conservative candidate boxes), argmin-BFGS
refinement per candidate pair, then dedup. But: the tree **splits at a randomly perturbed
midpoint** using an unseeded `rand::rng()` (`src/bounding_box/curve_bounding_box_tree.rs:80`,
±5% of the interval) — candidate generation is **nondeterministic run-to-run by design**
(curve tessellation `ThreadRng` and offset likewise). Dedup uses hardcoded gates
(`minimum_distance` 1e-5 default, plus a literal `parameter_minimum_distance = 1e-3` in
`intersection_curve_curve.rs:240`) that can merge or drop close/tangential roots. No
exhaustiveness argument beyond the box-tree conservatism; termination/acceptance are bare
solver tolerances (`curve_intersection_solver_options.rs`: 1e-5/1e-8/1e-10 defaults). Fits our
"UNTRUSTED candidate generator" category, but we'd have to add the determinism and the
certification — i.e., everything the C11 design is about.

### (d) Trimming / booleans maturity

2-D only, demo-to-solid-modeling gap: planar region clipping with a degeneracy module
(`src/boolean/degeneracies.rs`) driven by more `1e-` literals, one reachable `todo!()`, and no
solid/B-rep boolean at any level. Not a vendoring target; mildly useful as a UV-space
trim-loop oracle.

### (e) Tests / track record

58 `#[test]` functions plus dense doctests (144 `assert_relative_eq` sites) — real, numeric,
example-grade; no property-based or adversarial suites, no tolerance-boundary tests. Six
TODO/unimplemented sites in `src`. Single-author project; issue/PR history unverified offline.
Verdict-relevant: quality is good for a rendering/design-tool library, but nothing here
constitutes a correctness pedigree for kernel duty.

### (f) Licensing

Crate is MIT. Runtime dep tree (nalgebra/simba [Apache-2.0], spade, geo, argmin, robust,
gauss-quad, itertools, ordered-float, hashbrown, anyhow, num-traits, rand — permissive
MIT/Apache throughout; bevy stack is optional/feature-gated) — no copyleft anywhere. Clean for
dev-dependency use and for MIT-attribution vendoring alike.

## Vendor-vs-reimplement calls

1. **Knot algebra** — REIMPLEMENT. Insertion deviates from the textbook homogeneous form
   (zeroed weight delta), refinement has a fail-quiet TODO branch, removal's acceptance gate is
   a hardcoded homogeneous-space 1e-6 instead of Tiller's bound, and span/multiplicity logic
   runs on `default_epsilon`. Retrofit = replacing the decision structure of every routine on
   top of a scalar trait swap; that *is* a rewrite from A2.1–A5.9.
2. **Fitting stack** — REIMPLEMENT (nothing to vendor). Only A9.1 interpolation exists, dense-LU,
   f64-family-only; A9.10 approximation with certified residuals has no counterpart in curvo.
3. **Point inversion** — REIMPLEMENT. Seeding (uniform `n·p` samples + segment projection,
   `nurbs_curve.rs:1885`) is fine but trivial; the Newton itself (argmin `Solver` impls in
   `src/closest_parameter/`) terminates on `delta < F::epsilon()` with `max_iters(5)`, has a
   hardcoded `1e-5` singularity guard (surface variant), and returns a bare parameter — no
   orthogonality residual (NURBS Book 6.4/6.5), no certification, wrong scalar trait. Keeping
   argmin's framing buys nothing once the acceptance logic is ours.
4. **SSI seeding heuristics** — REIMPLEMENT (nothing surface–surface exists to vendor). The
   curve–curve box-tree pattern is worth *studying* as a shape (conservative subdivision →
   untrusted refine → dedup), but its randomized splits and literal-tolerance dedup are
   precisely what C11 forbids; our seeding must be deterministic (D9) and its dedup
   margin-carrying. Reference, don't port.

## Verdict

**Q5's default stance is confirmed, with the vendoring half now effectively closed: reference +
test oracle only; no curvo code enters the kernel.** The audit found no algorithm where
retrofitting our invariants would be cheaper than reimplementation from The NURBS Book: the
invariant-relevant surface area of every candidate routine (equality/acceptance gates, scalar
trait, termination criteria, error reporting) is exactly the part that would have to be
rewritten, and the two biggest hoped-for vendoring targets — an A9.10 fitting stack and SSI —
do not exist in curvo at all (DESIGN.md's landscape row should drop "incl. SSI"; booleans are
2-D). What survives is real: curvo is a well-built, active, MIT, pure-Rust NURBS library whose
textbook-faithful evaluation path makes a good independent oracle, and whose curve–curve
intersection architecture independently corroborates our march-then-certify shape. Q5
revision (APPLIED to DESIGN.md — both the Q5 entry and the landscape row): "resolved — study +
dev-dependency test oracle; vendoring rejected by the M5 audit (docs/CURVO-AUDIT.md)."

## As a test oracle

Usable, with scoping. **Good**: curve/surface point + derivative evaluation, basis functions,
degree elevation, interpolation (A9.1), arc length — deterministic, clean APIs
(`NurbsCurve3D::point_at` etc.), pin as a dev-dependency at commit `47d19d5` and compare against
our f64 path at oracle tolerances. **Usable with care**: curve–curve intersection as a
cross-check for our curve–curve results — candidate *sets* are nondeterministic (randomized
box-tree splits), so compare as tolerant point-set matching, never exact counts on tangential
cases. **Not an oracle**: SSI and solid booleans (absent — opencascade-rs/truck remain the
oracles there, per C12.9), knot removal near tolerance boundaries (its 1e-6 homogeneous gate
measures a different quantity than our Tiller bound), and anything bit-exact (std-math
scalars).
