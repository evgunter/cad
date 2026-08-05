# M5 PR 5 — `Ellipse` carrier + the C5 dispatch table (binding spec)

Executes M5-PLAN PR 5 (C1 rung 2, C5, C12.1, C12.3, R1). Base:
main after PR 4 and PR 8 merge. Branch `ev/m5-pr5-ellipse`.

## 1. The `Ellipse` carrier (C1 rung 2, OQ1 (b)-staged-via-(a))

`Curve3::Ellipse`: center `c`, orthonormal right-handed frame
(û major, v̂ minor), semi-axes `a ≥ b > 0` (a = b is a `Circle`,
refused at construction — one kind per configuration, the D3
closed-enum discipline; the constructor is the only place that
decides). Parameterization `P(θ) = c + a·cosθ·û + b·sinθ·v̂`,
θ in radians, libm-only, derivatives closed-form. D9: no new
`Real` methods (ring ops + existing trig suffice — C12.8).

- Ellipse is chosen because axes/center are the data downstream
  predicates consume (C1's rejection of rational-quadratic NURBS
  as *carrier*); the NURBS form remains the export/tessellation
  round-trip (NOT built here — PR 11/13 consume it).
- Persistence: the new variant round-trips in the current schema
  shape (a new variant arm, no migration — R3: migration is not a
  commitment). Round-trip rows in the persistence matrix.
- Bounds: ellipse-arc AABB via corner-evaluated extremal-angle
  INTERVALS, generalizing `circle_arc_aabb` exactly as PR 8's fix
  shaped it (branch-cut wedges include both extrema; same fuzz
  row pattern with a discriminating span).
- Parabola/hyperbola do NOT land (R1). No speculative fields, no
  reserved discriminants.

## 2. The C5 dispatch table (C12.1)

`topo::splitting::classify` / `boolean::reduce`'s plane×plane
special case refactors into THE exhaustive
`(SurfaceKind, SurfaceKind)` match. Rules, verbatim-binding from
C5:

- **Compile-time routing, no runtime fallback.** Every arm is a
  documented decision into rung 1 (closed form), rung 2 (conic),
  or rung 3 (march+fit — NOT ready until PR 7, so a rung-3 arm
  refuses typed, naming the routing: "this pair routes to the
  general rung, unimplemented until SSI"). No wildcard `_` arms
  anywhere in the table — adding a `SurfaceKind` must break the
  build (D3).
- **M2 pairs enter unchanged** with their existing certificates
  (rung 1). Their behavior is pinned by the existing battery; the
  refactor must be bit-invisible for them.
- **Within-pair degeneracy trileans run BEFORE any rung**: named
  Q1 predicates over configuration invariants with named lever
  arms (axis parallelism at derived angular thresholds,
  center/axis distances vs radii). definitely-generic ⇒ the arm's
  rung; exactly-degenerate ⇒ the degenerate closed form; in-band
  ⇒ F6 escalated typed error (ill-conditioned operand pair at
  this ε). New error Display text follows the ratified
  two-tolerance message shape (D4 ¶1 addendum; the S6 sweep's
  shared recourse carrier — coordinate at merge, S6 is in flight
  on `ev/m5-s6-messages`).
- `CurvedBooleanUnsupported` retires PER ARM implemented here,
  never wholesale (C12.1); arms still refusing cite their rung
  routing.

## 3. Closed forms landing in this PR (rung 2 arms)

1. **plane×cylinder, tilted**: exact `Ellipse`
   (zero-residual-by-construction, D4 ¶2 identically zero — the
   M2 rim case stays rung 1 `Circle`). The trilean: axis·normal
   angle vs derived threshold (parallel ⇒ line pair/tangent/empty
   — closed degenerate forms; near-parallel in-band ⇒ escalate).
2. **cylinder×cylinder, equal radii, intersecting axes**: two
   `Ellipse` carriers. Trileans: radius equality (structural or
   declared only — NEVER inferred from values, the coincidence
   ladder), axis coplanarity/angle. Unequal or skew ⇒ rung 3
   typed refusal (until PR 7).
3. **plane×cone, exact-degenerate cases only** (R1): apex-through
   plane (two lines / single tangent line / apex point),
   axis-normal cut (`Circle`), tangent plane (line). Generic tilt
   routes to rung 3 **explicitly and permanently** — a documented
   arm decision moved only by a future PR that adds the conic
   trio; until PR 7 it refuses typed.

## 4. `split_edge` conic lane (C12.3)

`EdgeCurveSpec::split_specs` gains the ellipse arm: parameter-
interval split, bounded like circles (the M3 restrict-a-bulge
machinery generalizes; arc-AABB from §1). Knot-insertion NURBS
splitting is PR 6/7 territory — not here.

## 5. Pcurves for conic carriers (C4 slice)

Closed-form per chart where the chart map keeps a conic exact;
fitted (PR 4 machinery, C6 pinning rule) where transcendental.
Certified in meters through the map. Full per-half-edge cache
storage/semantics is PR 6 — this PR provides only the
carrier-side pcurve constructors PR 6 will store.

## 6. Acceptance

- **Shape (i)**: tilted-plane×cylinder cut end-to-end — exact
  `Ellipse` carrier on the minted edges, residual identically
  zero, joins the Band 4 corpus with standard persistence/latency
  rows.
- Ellipse evaluator differential vs the exact rational-quadratic
  NURBS form (§7.3 shape factor) at fuzzed configurations —
  agreement to certified enclosure width, both lanes.
- Equal-radius cylinder×cylinder: the two-ellipse split verified
  against parametric substitution residuals (exact in ℝ; enclosure
  width in f64).
- Table exhaustiveness: no `_` arm (grep-able discipline row),
  compile-break demonstrated in a doc-note (not a committed test).
- Trilean coverage: each named predicate gets exactly-degenerate,
  definitely-generic, and in-band(escalation) rows — the F6/K
  pattern, K-funnel registered (k-lint clean).
- Full battery 3ε + interval; M2 rows bit-identical (the refactor
  is invisible to them); corpus/persistence/latency lanes green.

## 7. Out of scope

Parabola/hyperbola (R1); SSI/rung-3 implementation (PR 7); pcurve
cache storage (PR 6); NURBS export form of conics (PR 11/13);
tangency/TangentIntersection construction (PR 9 — a pair whose
transversality margin dies along the locus is CLASSIFIED here as
a typed refusal naming C7, never marched); curved census; any
`i_overlay`/preview lane (R6).

## 8. Process

Standard: foreground battery (chunked per-package), push per
unit, adversarial e2e review + fix pass, PR by orchestrator,
tier-aware merge gate. OUTPUT DISCIPLINE per standing header.
