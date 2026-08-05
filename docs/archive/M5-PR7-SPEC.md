# M5 PR 7 — SSI: march-then-certify + in-op exhaustiveness (binding spec)

Executes M5-PLAN PR 7 (C2, C3, OQ3, C12.8; deps [4, 5, 6] all
merged; BVH optional per the plan — brute-force subdivision
acceptable, swap rides PR 8's already-merged differential suite).
Branch `ev/m5-pr7-ssi` from main. This is the milestone's heart:
rung 3 of the C1 ladder becomes real, and
`CurvedBooleanUnsupported`'s rung-3 refusals in the C5 table gain
their implementation.

**OQ4 checkpoint, discharged at spec time:** carrier-primary
stands. The ℝ⁴ trace yields the 3-D curve and both pcurves as
projections of ONE parameterized object — a shared parameter,
which is exactly the parameter-identity contract PR 6 ratified
for caches. The 3-D carrier stays the certified authority
(witness, dihedral, certification machinery unchanged); pcurves
from the trace are stored via the PR 6 doors (fitted 2-D NURBS
with the same certificate). No re-plumbing; no fork for Evan.

## 1. The marcher (untrusted, f64, D9-clean)

Hoffmann §6.2 as C3 ratifies: third-order Taylor approximant of
the local parameterization from the underdetermined system
∇f·r⁽ᵐ⁾ = b (Eq. 6.1), Frenet choice of free coefficients
(γ₂ = 0, γ₃ = −κ²), step size by the small-contribution
heuristic, Newton refinement to the surface pair. The fixed-shape
2×3/3×4 SVD (Givens/Householder, fixed rotation order — C12.8)
joins `geom-core::linalg` HERE, its first consumer; σ₂ > 0 is the
transversality signal, U₃ the tangent. libm-only, fixed iteration
order, bit-replayable.

- **Nothing the marcher outputs is trusted.** Its product is a
  polyline+frames PROPOSAL handed to the PR 4 fitting stack; the
  C2 certificate is the only gate. A branch jump is a certificate
  refusal (tube fails to connect or transversality dies), typed.
- **The idealized/realized split ships in this PR** (T4,
  PERF-PLAN §4.4 — the flagged dual-code pilot): the idealized
  stepper (tangent-line steps, tiny fixed h) is the spec; the
  differential suite pins realized-vs-idealized agreement in CI
  from day one.
- **σ₂ sliver band ⇒ refuse toward C7** (`TangentIntersection`'s
  regime, PR 9): named trilean on σ₂ with a lever arm; in-band ⇒
  F6; never desingularize (Hoffmann §6.5 deliberately NOT
  adopted). Closure/loop decisions ("returned to start", "branch
  closed") are named Q1 trileans on parameter-space distances —
  never raw comparisons in the marcher.

## 2. Traces

- **Implicit×implicit / analytic pairs**: march in ℝ³ on the
  implicit pair (the C5 rung-3 analytic arms).
- **Parametric×parametric**: ℝ⁴ trace (u₁,v₁,u₂,v₂) on
  G₁ − G₂ = 0 (§6.3.2, 3×4 SVD). Both pcurves are coordinate
  projections; the 3-D curve via either chart.
- **Mixed analytic×NURBS**: march in ℝ³ on (implicit, chart) or
  in the analytic chart where cheaper — the choice is PER-ARM in
  the C5 table, documented at each arm (no runtime fallback; an
  arm's trace shape is a compile-time decision).

## 3. The full C2 certificate (OQ2: both, always)

Every rung-3 fitted cache carries all three limbs before it
reaches an at-rest body:
1. **On-locus residuals**: implicit residuals in meters along the
   schedule for analytic surfaces; for NURBS surfaces the PR 4
   certified foot points (projection orthogonality residual
   checked — a bad projection cannot launder a bad cache).
2. **Sup-norm honesty**: control-coefficient hull bounds on the
   residual composites per span (C9/PR 2 machinery + the PR 4
   compose substrate) — the sampled max steers, the hull bound
   certifies.
3. **Uniqueness tube (component selection)**: over a chain of
   boxes covering the cache with certified radius, the system's
   solution set is connected and transversal (normal cross
   product bounded away from zero by enclosure). Two branches
   within the band ⇒ genuine sliver ⇒ F6. Witness =
   carrier(mid), unchanged.

## 4. In-op exhaustiveness (the never-silence obligation)

Subdivision of the bounded domain (UV-box pairs / session-box
slab), per cell proving exclusion (C9 hull: f₁ ≠ 0 or f₂ ≠ 0) OR
accounted (inside a found branch's uniqueness tube) OR refine; at
the named floor (constant tied to ε) ⇒ typed
`SsiExhaustivenessInconclusive`. The subdivision doubles as the
marcher's seed generator (boundary-curve×surface seeds + surviving
cells) — found-ness never depends on luck. Brute-force cell
enumeration is acceptable in this PR; BVH-backed pruning swaps in
under the merged differential suite when profiled.

## 5. Consumers wired here (minimal)

The C5 table's rung-3 arms whose SURFACE PAIRS the acceptance
shapes need: plane×NURBS-wall (shape (iii)'s cut) and the
cylinder×torus (or equivalent) small-loop pair (shape (iv)).
Other rung-3 arms stay typed refusals citing their unimplemented
trace shape — retiring per-arm, never wholesale (C12.1). Booleans
END-TO-END on rung-3 carriers are PR 9 (zip/tangency); this PR
proves carriers + certificates + exhaustiveness on the
intersection layer plus split_edge integration where PR 5's lanes
already consume carriers.

## 6. Acceptance

- **Shape (iv), the milestone's signature**: the planted
  small-loop fixture (near-tangent cylinder×torus or equivalent
  where naive marching MISSES a branch) is FOUND by the
  subdivision-seeded march or refused typed — never silent. Both
  outcomes pinned (the found case as the primary row; a
  floor-clamped variant demonstrating the typed refusal).
- Shape (iii) substrate: a definitional NURBS wall (loft/sweep
  surfaces are PR 10 — use a directly-authored NURBS surface)
  cut by a plane: rung-3 marched+fitted+certified carrier, all
  three limbs, both lanes, bit-replay.
- Idealized/realized differential rows in CI (tiny-h vs
  production stepper agreement on the fixture set).
- A deliberately-corrupted fitted cache fails each limb
  separately (three rows: residual, hull, tube).
- σ₂-sliver refusal row; exhaustiveness-floor refusal row;
  closure-trilean trio.
- ℝ⁴-trace parameter identity: the projected pcurves certify
  through the PR 6 doors on the shared parameter (the
  OQ4-discharge demonstration).
- M2/M5 pair suites bit-identical (no touched arm regresses).
- Local: touched crates both lanes, fmt, clippy touched. CI
  gates the matrix.

## 7. Out of scope

Tangency construction (C7/PR 9 — refusals point there); booleans
end-to-end on rung-3 carriers (PR 9); loft/sweep surface
DEFINITIONS (PR 10); BVH-backed cell pruning (post-profile swap);
curved census/tessellation/props (PR 11+); any new Real methods
beyond the C12.8 linalg additions.

## 8. Process

Standard: foreground rows, one per Bash call, `pgrep -x cargo`
polling only; push per unit; adversarial e2e review + fix pass
(expect the review to attack the uniqueness-tube enclosure and
the exhaustiveness accounting hardest); PR by orchestrator.
OUTPUT DISCIPLINE per standing header.
