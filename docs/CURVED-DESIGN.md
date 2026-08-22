# M5 curved-geometry design: SSI, pcurves, NURBS depth, fillets (pre-M5 design doc)

Status: **RATIFIED (Evan, PR #85, 2026-07-24: "lgtm!").** This
document is the design record for M5 curved geometry. Every fork was
decided in the #85 review conversation and is recorded inline as
DECIDED with its ground; candidate decisions C1–C12 are the ratified
decisions of this doc. Tensions T1–T6 remain flags on ratified
decisions elsewhere (flag != reopen). The K = 10 tangent was resolved
at issue #89 (CLOSED — K = 10 permanent; docs/K-REPORT.md).

*Reading this document against the code (M5 complete 2026-08-03):*
the C1–C12 decisions are the ratified design record, but several
were implemented with recorded deviations and several frontiers
moved. Implementation truth lives in `docs/archive/M5-LOG.md` and
`docs/M5-EXIT-WALK.md`; where they disagree with this doc, the log
wins. One decision was overtaken outright: **C9's inari quarantine
is dead vocabulary** — M5 PR 1 (#127) removed inari and its LGPL
stack from the tree entirely, so the transition allowance C9 granted
was never needed and issue #4's exit condition is met by removal.

Grounding read for this doc: DESIGN.md (D2–D4, D9, Q1, Q5, Q8, the
Banked principles — especially SSI-completeness-is-an-interval-
obligation and fillet-validity-is-reified-predicates); M3-PLAN F5 (the
fourfold curved dependency chain and the no-speculative-abstraction
inverse commitment); PERF-PLAN §2.1/§4.4/§5 (BVH triggers, the SSI
stepper as idealized/realized pilot); M2's certified analytic pairs
(geom-brep `implicit.rs`/`certify.rs`). Reference results are cited by
chapter/page: Hoffmann *Geometric and Solid Modeling* ch. 6; Piegl &
Tiller *The NURBS Book* 2e ch. 5–7, 9, 10; Vida–Martin–Várady 1994
(CAD 26(5), the blending survey). Condensed lit notes: appendix.

## 0. What M5 is, in our terms

M3-PLAN F5 ratified the dependency chain that defers curved booleans
to M5 *as a unit*: (a) intersection-locus representation beyond
`Line | Circle`, (b) general pcurves, (c) second-order sector
classification (the `TangentIntersection` regime), (d) certified marching
numerics. M5 = that chain, plus the NURBS substrate it rides on
(sweeps/lofts per the roadmap), plus constant-radius fillets (whose
validity predicates were banked pre-M5), with the M3 face-intersection
seam refactored against these real requirements (the F5 inverse
commitment's second half).

The design thesis applied to curved geometry, stated once: **exactness
where closed forms exist (D3), intensional descriptions with certified
caches where they don't (D2/D4), and every completeness claim backed
by an exclusion certificate rather than an algorithm's diligence**
(the banked SSI contract). Marching produces candidates; certification
produces truth; nothing the marcher does is trusted.

## C1 — Intersection-locus representation: a three-rung ladder

What is the carrier cache of an `Intersection` edge between curved
surfaces? Three rungs, dispatched by surface-kind pair (C5), most
exact wins:

1. **Closed-form analytic carriers** — `Line`/`Circle`, the M2
   certified pairs (plane×plane, plane×cylinder rims, revolve meridian
   pairs, …). Already shipped; slots into the C5 table unchanged.
2. **Exact conics** — extend `Curve3` with an `Ellipse` variant (and
   see OQ1 for parabola/hyperbola). A tilted plane×cylinder cut is an
   ellipse; plane×cone cuts are conics; equal-radius
   cylinder×cylinder splits into two ellipses. These are *the* common
   curved-boolean cuts in mechanical parts, and a conic carrier makes
   their D4 ¶2 residual identically zero-by-construction, exactly as
   for M2's circles — no fitting, no exhaustiveness apparatus, no
   escalation surface. D3 anticipated this growth ("closed enum …
   adding a new analytic kind means adding a variant and letting the
   compiler enumerate every dispatch site"). M3-PLAN F5 already named
   the option.
3. **Fitted NURBS cache** — the general rung (generic quadric×quadric
   is degree 4+; anything involving a NURBS surface is worse). The
   intensional description stays `Intersection { s1, s2, witness }` —
   unchanged, that is the point of D2 — and the carrier cache is a
   NURBS curve fitted by our own machinery, carrying the C2
   certificate. The NURBS Book's Type-2 fitting loop (A9.10,
   `GlobalCurveApproxErrBnd`, p. 431) is the right *shape*: fit low
   degree, knot-remove under a bound, degree-elevate, refit. Its
   bound, however, certifies deviation from the *data/previous curve*
   (Eqs. 9.77–9.83, pp. 424–428), not distance to the described locus
   — our certificate is C2's, computed against the surfaces
   themselves; the book's bound only steers the fitting iteration.

**Alternatives considered.**
- *No conic rung; everything non-Line/Circle is a fitted NURBS.*
  Uniform, exercises the general path early, one less `Curve3`
  variant across every dispatch site. Rejected: it converts the most
  common curved cuts from exact to approximate, buys per-edge fitted
  caches + certificates + (for closed cuts) seam bookkeeping where a
  five-float carrier suffices, and forfeits a D3 exactness payoff the
  architecture was explicitly shaped for. The general path gets
  exercised regardless (torus cuts, NURBS walls).
- *Conics as exact rational-quadratic NURBS* (NURBS Book §7.3–7.4:
  conic shape factor k = w₀w₂/w₁², Eq. 7.25 p. 292, classification by
  w₁² p. 293; circular/elliptic arcs ≥ 180° via infinite control
  points §7.4 pp. 295–297). Representationally exact, no new variant.
  Rejected as the *carrier* form: it papers over kind information the
  dispatch and predicates want (an ellipse's axes/center are the data
  every downstream classification consumes; recovering them from
  weights is inverse work), and rational parameterization is
  non-uniform in arc length, complicating derived angular predicates'
  lever arms. Kept as the *export/tessellation* form — conics
  round-trip into NURBS exactly when needed (STEP wants this anyway).
- *An `Explicit`/polyline rung.* Not considered. D2 omits it on
  purpose; nothing in SSI needs it (the fitted rung IS the honest
  extensional-shaped object, but pinned to an intensional description
  and a certificate).

**Recommendation:** rungs 1–3 with `Ellipse` added at M5; the
parabola/hyperbola question is OQ1 (they arise only from plane×cone
at specific tilts; both are unbounded like `Line`, so bounds-from-
vertices machinery transfers).

## C2 — The certificate for a fitted curve

D4 ¶2 says every derived cache carries a certified residual ≤ ε
against its description. For M2's closed-form carriers, certification
sampled a 9-point schedule of closed-form residuals. A fitted SSI
cache has no closed form on either side of the comparison, and the
witness contract's component-selection semantics "activates with real
SSI at M5" (D2's S2 sharpening, verbatim). The certificate must
therefore answer three questions, not one:

1. **On-locus residual (nearness).** For analytic surfaces, the
   existing linearized implicit residuals (`implicit.rs`: meters,
   signed-distance-to-first-order) evaluated along the cache:
   max over the schedule of |f₁(C(t))|, |f₂(C(t))|. For a NURBS
   surface (no implicit form) the residual is |C(t) − S(u*,v*)| with
   (u*,v*) the certified foot point — point projection (NURBS Book
   §6.1, pp. 229–234) with the orthogonality conditions' own residual
   checked, so a bad projection cannot launder a bad cache.
2. **Sup-norm honesty (between samples).** A sampled max is not a
   bound. For M2's pairs this gap was closed by closed-form reasoning
   per pair; the general rung needs a mechanism: the residual
   composites f∘C are piecewise-polynomial/rational in t for analytic
   f (the implicit forms are polynomial; C is a NURBS), so **spline
   enclosure by control-coefficient hulls** (the convex-hull/partition-
   of-unity property — the same mechanism that powers the book's
   knot-removal bounds N_{r−k,p}(u)·B_r, Eq. 9.81 p. 427 and Eqs.
   9.86–9.89 pp. 433–434) yields true interval bounds per span with
   ring arithmetic only. Sampled max governs the fit; the hull bound
   certifies. See C9 for the arithmetic substrate.
3. **Component selection (the witness's job, now real).** "The
   connected component selected by the witness" is only checkable if
   branch-uniqueness near the cache is *proved*: a **uniqueness-tube
   certificate** — over a tube of certified radius ρ around the cache
   (in practice: a chain of boxes covering it), the system
   (f₁ = 0, f₂ = 0) has its solution set connected and transversal
   (normals' cross product bounded away from zero over the tube — the
   Q1 margin, first-order, exactly D2's transversality
   predicate-with-margin lifted from samples to an enclosure). This
   is the same move as SOLVER-DESIGN W2 ("selection is certification,
   not search"): the f64 artifact proposes, an enclosure computation
   proves one-branch-ness, and refusal is typed, never a retry loop.
   Two branches passing within the band of each other is a genuine
   sliver of the operand pair — escalation is correct and is F6's
   ladder speaking, not a weakness.

The witness pin itself transfers verbatim: witness = carrier(mid)
(`WitnessMidpoint`), minted by the constructing op from the cache the
certification schedule sees — no change, S2 stays discharged.

**Alternatives considered:** (i) *schedule-max only* (the literal M2
shape) — rejected as the standing definition for the general rung:
between-sample excursions are exactly where a marched-and-fitted curve
lies about the locus (the fit is smooth, the locus is smooth, but a
missed wiggle or a wrong-branch segment between samples is the failure
mode with no closed-form backstop); acceptable only as a staged
implementation step with the hull bound as the ratified target — the
staging question was OQ2, decided 2026-07-24: no such stage ships
(hull bounds are an entry requirement). (ii) *Full interval-Newton tube from inari at
T = Interval* — soundest, but puts LGPL-quarantined machinery on the
default build path (Tension T2) and is unnecessary: the exclusions
here are polynomial. (iii) *Trust the marcher's step control* —
rejected on principle; see C3.

## C3 — SSI: march-then-certify; the stepper is untrusted

The question the plan asked — "how does a step certify it hasn't
jumped branches?" — gets inverted: **it doesn't.** No per-step
predicate can carry that burden honestly (a step certificate would
have to prove a global property — no other branch within reach — from
local data). Instead:

- **Marching is a candidate generator.** Hoffmann §6.2 is the adopted
  stepper shape: third-order Taylor approximant of the local
  parameterization r(s) solved from the underdetermined linear system
  ∇f·r⁽ᵐ⁾ = b_{f,m}, ∇g·r⁽ᵐ⁾ = b_{g,m} (Eq. 6.1 p. 209) with the
  Frenet-frame choice of the free coefficients (γ₂ = 0, γ₃ = −κ²,
  pp. 212–214 — the approximant then carries curvature and torsion);
  step size by the heuristic that quadratic/cubic contributions stay
  small (p. 215); Newton refinement to the surface pair (p. 215–216);
  SVD solves throughout, with σ₂ > 0 the transversality signal and U₃
  the tangent ∇f×∇g (p. 217). All of it f64, libm-only, fixed
  iteration order, D9-clean — and none of it trusted. Its output is
  a polyline+frames proposal handed to fitting (C1 rung 3), and the
  C2 certificate is the only gate. A branch jump becomes a certificate
  refusal (the tube fails to connect or fails transversality), typed.
  This is PERF-PLAN §4.4's idealized/realized split applied at the
  semantic level — and the SSI stepper is already flagged there as
  the dual-code pilot: the idealized stepper (tangent-line steps,
  tiny fixed h) doubles as the spec the differential suite pins.
- **Parametric×parametric marches in ℝ⁴.** Hoffmann §6.3.2
  (pp. 222–223): trace (u₁, v₁, u₂, v₂) on the system
  G₁(u₁,v₁) − G₂(u₂,v₂) = 0 (three equations, four unknowns; same
  SVD machinery at (n−1)×n). Both pcurves fall out as coordinate
  projections of the traced curve and the 3-D curve via either chart
  — one trace, all three classical representations, which is exactly
  the alignment C4 wants. Mixed analytic×NURBS pairs march in ℝ³ on
  (implicit, chart) or in the analytic surface's own chart when that
  is cheaper; the choice is per-arm in the C5 table, documented.
- **Exhaustiveness is a separate, box-based obligation** (the banked
  principle, mechanized): subdivide the bounded domain (UV-box pairs
  for parametric pairs; the session-box slab for implicit pairs) and
  per cell prove one of: (i) *exclusion* — f₁ ≠ 0 or f₂ ≠ 0 over the
  cell (hull/interval bound, C9); (ii) *accounted* — the cell's
  solution set lies in a found branch's uniqueness tube (C2.3);
  (iii) neither ⇒ refine; at the floor (a named constant tied to ε),
  typed `SsiExhaustivenessInconclusive`. Outcome: "every branch
  found" as a theorem about enclosures, or a typed failure — never
  silence about small loops (the classic silent disaster the banked
  principle names). Seeding the marcher: boundary-curve×surface
  intersections plus cells surviving exclusion — the subdivision is
  both the safety net and the seed generator, so "marching finds"
  never depends on luck.
- **Closure and loop topology are margined decisions**: "the trace
  returned to its start" and "this branch is closed" are named Q1
  trileans on parameter-space distances (lever arms named), never raw
  comparisons inside the marcher. Singular/near-tangential
  configurations (σ₂ in the sliver band along the trace) refuse
  toward the C7 regime rather than desingularize: Hoffmann §6.5's
  quadratic-transformation tracing through singular points
  (pp. 243–251) is deliberately NOT adopted — tangential contact is
  `TangentIntersection`'s domain and in-band contact is a genuine sliver
  (F6); we refuse loudly where he traces through.

**Alternatives considered:** (i) *per-step certified stepping*
(interval step widths with containment proofs per step) — sound but
couples correctness to the hardest, most performance-critical code,
exactly what PERF-PLAN's dual-code section warns about; the
march-then-certify split keeps the hot loop free and the proof
obligation where it can be audited. (ii) *Pure subdivision SSI* (no
marching; intersect by recursive box refinement alone) — robust,
horrifically slow at fine ε, and its output still needs fitting;
subdivision is retained for what it is uniquely good at
(exhaustiveness), marching for what it is good at (fast accurate
point strings). (iii) *Algebraic elimination to a plane curve*
(Hoffmann §6.4, resultants/projections) — powerful for low-degree
implicit pairs but degree-explodes past quadrics and sits poorly with
NURBS; not adopted; the ℝ⁴ trace covers its use case for us.

## C4 — Pcurves: per-half-edge certified caches, certified in meters

Background restated from D2: a face is a region of its surface's
(u,v) plane; each boundary edge of that face therefore also has a
2-D parameter-plane curve — the pcurve — and an edge between two
faces classically carries three peer representations (C(t) + two
pcurves), the classic bug farm. Our rule (D2, verbatim): the
intensional description is authoritative; *all* concrete forms are
certified caches.

- **Home: per half-edge.** A pcurve belongs to an (edge, face-side)
  incidence, and the half-edge IS that incidence in our structure.
  Seam edges force this granularity: both half-edges of a seam edge
  lie on the SAME surface with different pcurves (u = 0 vs u = 2π
  sides), so "per edge-per-face" under-keys — per half-edge is the
  only shape with no special case. Planar faces keep M2's trivial
  status (pcurve derivable on demand from the plane chart; nothing
  stored) until a consumer wants otherwise — no speculative caches.
- **Orientation from the ratified forward contract**: the pcurve's
  parameter is the edge's carrier parameter (he_plus-forward,
  increasing start→end), traversal sense per face derived — "derived,
  never stored as peers" (D1 topology conventions, verbatim).
- **Certification is in meters, through the map.** The residual that
  means anything is |S(P(t)) − C(t)| ≤ ε — 3-D displacement between
  the surface-composed pcurve and the carrier cache, on the shared
  certification schedule, hull-bounded per C2.2. A raw (u,v)-space
  tolerance is dimensionally dishonest (D4 ¶1's lesson transposed:
  chart units are not meters; the map's local stretch σ_max(dS) is
  the lever arm, and it varies — near a cone apex a tiny UV error is
  a tiny displacement, near a large-R torus equator the reverse).
  Any UV-space step/tolerance inside algorithms is an implementation
  dial; certified statements convert through the map.
- **Domain validity is part of the certificate**: P(t) stays in the
  face's trim region; periodic charts unwrap along one continuous
  branch pinned at the start point (the M2 PR 5 meridian-unwrap
  finding generalized: nearest-previous unwrapping per sample is a
  bug; the branch is chosen once and continuity is certified).
- **Construction**: for the general rung the ℝ⁴ trace (C3) yields
  pcurve data natively; for analytic/conic rungs pcurves are
  closed-form per pair (a plane×cylinder ellipse's pcurve on the
  cylinder chart is a sinusoid graph — transcendental in the chart,
  fitted as a 2-D NURBS cache with the same certificate; on the
  plane chart it is the exact ellipse). Fitted pcurves are 2-D NURBS;
  the fit loop is the same A9.10 shape as C1 rung 3, under C6's
  pinning rule (structure f64-selected, certification scalar-generic
  — D9 replay of pcurve caches is the same story as every other
  fitted cache, libm-only and bit-replayable; chart evaluations are
  the only transcendental sites and already live on `Real`).

**Alternatives considered:** (i) *derive pcurves on demand, store
nothing* — attractive purity (fewer caches, no staleness class), but
trimming, tessellation, census extension, and SSI-on-trimmed-faces
all consume pcurves on hot paths, and re-deriving means re-running
point *inversion* per query (Book §6.1 is iterative — a hidden
unreliable loop on every trim test); rejected — content-keyed cache
transfer (banked) already makes stored caches cheap to keep valid.
(ii) *pcurve-primary, 3-D carrier derived* for the general rung
(the ℝ⁴ trace makes this natural — see OQ4): defensible; recommended
against, narrowly, because every existing certification, witness,
and dihedral mechanism keys off the 3-D carrier, and demoting it
re-plumbs certified machinery for symmetry's sake. The fork is real
and cheap to flip pre-implementation; it is listed as OQ4.

## C5 — Dispatch: one total kind-pair table, no runtime fallback

The M3 face-intersection seam (`splitting::classify` /
`boolean::reduce`, plane×plane closed-form behind a thin interface —
built deliberately thin per F5's inverse commitment) is refactored
into **the** pairwise dispatch table D3 promised: an exhaustive match
over (SurfaceKind, SurfaceKind) where every arm is a *compile-time*
routing decision into rung 1, 2, or 3 of C1 — closed-form, conic, or
march+fit. Rules:

- **No runtime fallback.** "Try closed-form, else march" is a silent
  semantic downgrade (a conditioning-dependent representation change
  the recipe never sees). An arm's rung is a documented decision; an
  unimplemented closed form routes to the general rung *explicitly
  and permanently* (until a PR moves it), or refuses typed if the
  general rung isn't ready for that pair. The compiler enumerates the
  table; adding `Ellipse`/`Nurbs` arms is exhaustiveness-checked (D3).
- **Within-pair degeneracies are trilean, before any rung runs**:
  coaxial cylinders, concentric spheres, tangent pairs, plane through
  cone apex — configuration invariants (axis parallelism at derived
  angular thresholds with named lever arms, center/axis distances vs
  radii) classified by named Q1 predicates: definitely-generic ⇒ the
  arm's rung; exactly-degenerate ⇒ the degenerate closed form (two
  lines, a point, a `Seam`-adjacent case, a `TangentIntersection`
  candidate); in-band ⇒ escalated typed error (F6 verbatim — an
  ill-conditioned operand pair at this ε).
- The M2 pairs enter the table unchanged; their certificates already
  match rung 1. `Nurbs`×anything routes to rung 3. The table is also
  where the *tangency* dispatch lives: a pair whose transversality
  margin dies along the whole candidate locus is a `TangentIntersection`
  construction (C7), reached by classification, never by marching
  into it.

**Alternative considered:** keep the thin M3 interface and grow it
case-by-case — rejected; that is exactly the guessed abstraction F5
banned M3 from building, now buildable against real requirements, and
the closed-enum table is the ratified D3 shape for it.

## C6 — Fitted-cache structure is an f64-lane artifact; certification is scalar-generic

The fitting loop (A9.10: add/remove knots until the bound holds) is
value-branching in a way Q1's trilean discipline cannot and should
not absorb: its branches decide *cache shape* (knot count, degree),
never topology. Pinning the discipline's boundary explicitly:

- **Selection at f64**: knot vectors, degrees, and control-point
  counts of fitted caches are produced by the deterministic f64
  pipeline (D9: same inputs ⇒ same bits, libm-only, fixed iteration
  and pivot orders — in-house solvers for the small LSQ systems,
  fixed elimination order, no external BLAS nondeterminism).
- **Certification at any T**: the C2 certificate re-evaluates against
  the pinned structure generically — the interval/Probe lanes *prove*
  what the f64 lane *chose*. This is SOLVER-DESIGN W2's division of
  labor, verbatim, applied to fitting ("the interval lane contracts,
  it does not solve"); it is also how GQ1 resolved the same tension
  for constraint branches (f64 witness authoritative, interval
  certifies).
- **Topology never reads cache shape.** No topology-determining
  predicate may consult knot counts, spans, or fitted coefficients
  except through certified residual/margin values that are themselves
  named predicates. (The tier-3 validator consumes certificates, not
  structure.) This keeps the naming pillar airtight: the name table
  is a function of recipe structure + verdicts (N4), and cache shape
  is in neither.
- Under M10 interval replay the fitted structure transfers with the
  body (lineage-scoped keys; content-keyed transfer applies — the
  cache is keyed by the bit-content of its inputs), and the interval
  lane certifies residuals over the parameter box; an indeterminate
  certificate joins the subdivision-driver posture like every other
  interval refusal. Refitting per sub-box is an *optimization*
  decision for M10, never a semantic one.

**Alternative considered:** make the fitting loop's convergence tests
trilean and replay the whole fit at every T — rejected: it manufactures
escalations with no semantic content (a knot-count difference is not a
topology event), multiplies interval-lane cost by the fit's iteration
count, and W2's precedent already gives the sound split.

## C7 — `TangentIntersection` and second-order sector classification

The second intrinsic variant lands (D2 specified its validity
predicate in full; this section only mechanizes it):

- **Shape**: `TangentIntersection { s1, s2, witness }` — mirroring
  `Intersection`, witness pinned at carrier(mid) by the same S2
  argument. (Rename decided per OQ7, Evan 👍 #85 2026-07-24: the
  variant becomes `TangentIntersection`, a D2 sharpening at
  ratification.) No stored contact-order field: order-k contact beyond
  k = 1 is out of scope (D2 notes the generalization; nothing at M5
  produces it).
- **Certification** (the jet system, D2 verbatim, made a schedule):
  per sample — surface coincidence within ε (both implicit residuals);
  normal parallelism within the derived angle ε·κ_rel (lever arm
  r = 1/κ_rel, D4 ¶1); relative transverse normal curvature bounded
  away from zero (the margin, second-order — the IFT denominator for
  the jet system). Plus the C2.2 hull bounds between samples and a
  C2.3-style uniqueness tube built on the *jet* system (which is
  well-conditioned along a genuine tangency precisely because it
  includes the first-order equations — D2's own observation; the
  reconstruction conditioning argument is already ratified text).
- **Second-order sectors** (F5 chain (c)): M3's neighborhood
  classification ranks sectors by first-order data (face normals at
  the vertex/edge); curved sectors tie at first order exactly when
  surfaces are tangent there. The extension: where the first-order
  trilean returns exactly-on (a tie), classification descends one
  order — compare normal curvatures of the tied sectors along the
  probe direction, as a *new named trilean* with margin = curvature
  difference against the derived threshold at lever arm 1/κ (the
  displacement an angular/curvature difference induces at feature
  scale, D4 ¶1 discipline). In-band second-order ties escalate (a
  genuinely osculating pair is a sliver at this ε — F6). This
  predicate family is the second genuinely ill-conditioned crop the
  K funnel will see (after `solver_branch_margin` — K-REPORT's scope
  honesty predicted exactly this corpus; the M5 exit must include a
  K-telemetry snapshot).
- **Tier-3 interaction (decided per OQ7, Evan 👍 #85 2026-07-24 —
  the two-level shape)**: prefer-intrinsic today enforces
  definitely-transverse ⇒ `Intersection`. The symmetric side gets two
  levels, not one rule: (i) **the mark** — every definitely-tangent
  edge carries the tangency verdict as a named recorded
  classification (tier 3 already samples dihedrals per edge; same
  data, kept as a verdict); (ii) **the must-carry rule**
  (`TangentNotIntrinsic`, the `TransverseNotIntrinsic` sibling) fires
  only on **jet-determinate** tangencies — definitely-tangent AND
  second-order separation definite — so G2 conventional `MappedCurve`
  joins are exempt by the predicate itself (their second-order margin
  is zero-side: the surfaces under-determine the locus), never by an
  exemption list; in-band second order escalates (F6). Escalated and
  `Seam` edges stay exempt exactly as today, preserving the
  ε-tightening-never-flips-valid-to-invalid property.
- Tier 3′/census: **not extended to curved contacts at M5**
  (recommendation): the census stays exact-on-planar
  (`CensusUnsupported` refusal for curved inventories is already
  typed and honest); curved boolean results that *touch* therefore
  refuse at the 3′ gate rather than certify. Curved-contact census
  (coincident cylinders, tangency contacts as declared records) is
  real work with its own coincidence-ladder story — deliberately
  out of M5 (OQ5). Consequence stated honestly: M5 curved booleans
  produce tier-3 (transverse, non-touching) results; touching curved
  configurations are the M5 envelope's typed frontier, exactly as
  boundary-on-boundary seams were M3's.

## C8 — Fillets: reified predicates first, rolling-ball with analytic-first blends

Vocabulary (Vida–Martin–Várady pp. 341–345, Figs. 4/6, adopted): the
**base surfaces** are blended; the blend meets them along
**trimlines** (our contact curves — `TangentIntersection` loci); the
**spine** is the center curve of the rolling ball; a **profile** is
the cross-section. Constant-radius rolling-ball **edge blends** only
at M5 (the survey's own observation that edge blends dominate
practice, p. 343).

- **The banked principle is the API**: fillet validity is a set of
  named margined predicates over the *inputs*, evaluated before
  construction — r vs 1/κ_max of each support along the edge (blend
  self-intersection/local interference; the survey's too-large-ball
  global-interference warning p. 342 is the same fact globalized),
  r vs adjacent-face extent (face consumption), spine regularity
  (the rolling-ball center curve's own smoothness margin — it is an
  offset-locus and degenerates where supports' curvature ≈ 1/r),
  edge-chain smoothness (G1 chain closure for the spine to exist),
  convexity-sign consistency along the edge (a dihedral flipping
  convex↔concave mid-edge has no constant-radius rolling-ball blend
  — trilean per sample, escalate on flip), corner configuration
  (enumerated; see scope). Every one is a Q1 trilean through k_stats
  from birth, which is what lets M10 certify fillet validity over a
  parameter box (the banked payoff, restated not re-argued).
- **Blend surface representation, analytic-first (D3 payoff)**: the
  constant-radius rolling ball over analytic supports lands on
  analytic surfaces in exactly the cases that dominate: plane–plane
  edge → cylinder patch; edges with a straight-line spine → cylinder;
  circular-arc spine with fixed profile orientation → torus patch;
  vertex ball → sphere patch; cone cases → cone/torus per
  configuration. Where the spine is a general curve, the blend is a
  canal surface, NOT exactly NURBS-representable in general (same
  square-root obstruction as offsets — Q8's canonical case; Hoffmann
  §6.3.3's envelope formulation, Eqs. 6.7–6.9 p. 225, is the
  defining system: S = 0, ∂S/∂α₁ = 0, ∂S/∂α₂ = 0 over the
  sphere family) — so it is Q8's **approximating surface**: intensional
  spec `Blend { s1, s2, r }` (the rolling-ball definition), a fitted
  NURBS cache, certified residual ≤ ε — mirroring fitted intersection
  curves exactly as Q8 anticipated. This adds the first approximating
  *surface* to the kernel; its certificate is C2's lifted one
  dimension (schedule over (u,v), hull bounds per patch,
  envelope-system residuals as the on-locus test).
- **Prefer-intrinsic applied (D2 verbatim)**: trimline edges are
  stored `TangentIntersection`; the rolling-ball construction is demoted to
  supplying witnesses and initial caches. Construction history lives
  in D5 provenance. Native and (future, M7) imported fillets carry
  identical descriptions — the D7 story depends on M5 doing this
  honestly now.
- **Scope box (OQ6)**: closed smooth edge chains and open chains with
  the two run-out policies the survey's termination discussion forces
  us to name (stop-at-vertex with a corner patch, or feather-out) —
  recommendation: M5 ships closed smooth chains + the
  three-convex-edge vertex = sphere-octant corner patch, refuses
  typed everything else (`FilletCornerUnsupported`); run-out
  policies are a taxonomy decision Evan should own before any lands.
  (Scope decided per OQ6, #85 2026-07-24: as recommended, with the
  die-with-pips demo upgrade as the acceptance target. Run-out
  vocabulary decided too, Evan 👍: minimal two-policy refusal-payload
  names — `RunOutStopAtVertex`, `RunOutFeather`, corner-configuration
  tags — zero constructor surface; see OQ6.)

## C9 — The exclusion arithmetic: hull/ring intervals

C2/C3/C7's enclosure obligations (exhaustiveness exclusion, sup-norm
hull bounds, uniqueness tubes) make interval arithmetic load-bearing
on the default build path — which at the time meant dragging the
`interval` feature's LGPL transcendental stack (issue #4) into every
consumer, and D4/D9 forbid quietly weakening the claims instead. The
resolution is structural, not a compromise:

- **Every enclosure M5 needs is transcendental-free.** The implicit
  residuals of all five analytic kinds are polynomial in the point
  (with per-surface constants like cos α computed once and enclosed
  by outward ulp-widening); NURBS evaluation is ring arithmetic (de
  Boor); residual composites f∘C are piecewise-rational; hull bounds
  are convexity facts about control coefficients (partition of
  unity — the Book's Eq. 9.81 mechanism). So M5 certification needs
  an **interval ring**: ±, ×, ÷, with directed rounding — no sin, no
  exp.
- **Recommendation**: a small in-house `geom-core` interval-ring type
  (MIT-clean, f64 endpoints, outward rounding via nextafter-style ulp
  widening — sound, slightly conservative, no rounding-mode fiddling,
  hence also trivially D9-deterministic and thread-safe), used by the
  certification layer; `inari` remains the full-transcendental lane
  behind the `interval` feature for T = Interval replay (Q1
  machinery), unchanged. The two meet at the `Bounds` trait — the
  certification code is generic and does not know which it got. This
  is also a concrete first step of the Tabled in-house-interval
  program, scoped to the ring (the Tabled item's hard part —
  transcendental pads — stays tabled).

**DECIDED (Evan, #85, 2026-07-24):** the in-house ring is approved
(OQ8). *(As built, it went further than this section proposed: M5
PR 1 (#127) grew the in-house crate to full transcendentals and
retired inari from the tree altogether, so the second bullet's
"inari remains the full-transcendental lane" is historical — see
DESIGN.md's crate table.)*

**Alternatives:** (i) require the `interval` feature for curved
certification — makes default builds unable to validate curved bodies
(non-starter against D4 ¶2's "kernel invariant, the topo validator
checks it") or drags LGPL into every consumer; (ii) rational-exact
arithmetic for exclusions — exact but unbounded coefficient growth on
degree-8 composites, and unnecessary: conservative enclosures are
enough for exclusion/sup-norm duty (only *containment*, never
tightness, is load-bearing).

## C10 — The BVH crate (proposed; mostly restated obligations)

PERF-PLAN already scoped it; M5 is its ratified trigger ("M5 curved
booleans at the latest"). Restated as commitments with the M5-specific
additions:

- One `bvh` crate, deterministic AABB tree: arena-order build, fixed
  split rule, total tie-breaks, no hash iteration, no parallel build
  in v1 (D9). **Conservative-superset contract** (a BVH may only
  prune pairs the exact predicate would reject) — the D9 obligation
  that keeps results a function of exact tests only.
- Consumers at M5: the boolean edge×face sweep (retiring M3's
  documented quadratic), SSI seeding and the C3 exhaustiveness
  subdivision (the box tree IS the subdivision structure — cells
  carry the C9 enclosures; one structure, two duties), and later
  viewport picking (Band 1).
- Idealized/realized pilot per PERF-PLAN §4.4: idealized = brute-force
  all-pairs; pin = realized candidates ⊇ idealized pairs plus
  bit-equal final results; CI differential suite from day one (the
  pattern is only permitted WITH its suite).
- AABBs of curved entities are themselves certified-conservative
  boxes: analytic extents closed-form; NURBS extents from control
  hulls (convexity again — free and sound). A box is a cache with a
  containment contract, not a tolerance object.

## C11 — NURBS substrate scope

What "NURBS depth" must actually contain, bounded:

- **Types/evaluators**: non-uniform rational B-spline curves (2-D for
  pcurves, 3-D) and surfaces; evaluation and derivatives generic over
  `Real`, de Boor with fixed recursion order (ring ops only — no new
  `Real` surface); **positive weights invariant** (w > 0 enforced at
  construction: negative weights void the convex-hull property — Book
  p. 293 — on which every C9 hull bound stands; zero/infinite control
  points §7.4 are an export-form nicety, not a kernel representation).
- **Algorithms** (The NURBS Book, by section): knot insertion §5.2
  (pp. 141–161: evaluation-stable splitting — `split_edge` on NURBS
  carriers is knot insertion to full multiplicity), refinement §5.3,
  removal §5.4 (pp. 179–188) with the Tiller error bounds as used by
  fitting; degree elevation §5.5; point inversion/projection §6.1
  (pp. 229–234) for foot points (C2.1) with certified orthogonality
  residuals; the global fitting stack §9.4.1–9.4.4 (least squares
  Eqs. 9.63–9.67; bounded Type-2 loop A9.8–A9.10 pp. 428–432, surface
  form Eqs. 9.86–9.89) as the fit engine under C6's f64-pinning rule.
- **Sweeps/lofts** (the roadmap's M5 features): skinned and swept
  surfaces per ch. 10 (§10.3 pp. 457–472, §10.4 pp. 472–485). These
  are **definitional** surfaces (Q8): the produced NURBS *is* the
  definition, recipe as provenance — no residual obligation, no
  approximating-surface machinery; only their *derived* items
  (intersections with them, pcurves on them) carry certificates. The
  loft/sweep feature nodes join the M4 vocabulary as ordinary ops.
- **Deliberately absent**: surface fitting to scattered data (M7
  adoption's tool, not M5's), offset surfaces (Q8 names them for
  shelling, M5+ — but C8's blend fitting builds the machinery they
  will reuse), trimmed-surface *booleans in UV* beyond what trim
  loops need (i_overlay stays banked), degree reduction §5.6 (no
  consumer).

## C12 — The refactor inventory: M3/M4 seams touched against real curved requirements

The F5 inverse commitment promised M5 would refactor the thin
boundary rather than inherit a guess. The concrete list (each a
PR-plan line item, none re-ratifying anything):

1. **Face-intersection seam** (`topo::splitting::classify`,
   `topo::boolean::reduce`): plane×plane special case →
   the C5 dispatch table; `CurvedBooleanUnsupported` retires
   incrementally per table arm, never wholesale.
2. **Neighborhood/sector classification** (`topo::splitting::
   neighborhood`): first-order sector ranking gains the C7
   second-order tie-break lane; the ON-set machinery consumes curved
   carrier tangents (they exist — carriers are complete loci with
   derivatives) instead of assuming straight edges.
3. **`split_edge`/`EdgeCurveSpec::split_specs`**: NURBS carrier
   splitting = knot insertion (C11); conic splitting = parameter
   interval split (bounded like circles). The M3 restrict-a-bulge
   machinery generalizes; the `MappedCurve` arc lane's old coverage
   note (unreachable-at-rest) is retired — curved booleans and the
   fillet verbs split mapped arcs mid-operation, so the lane is
   exercised end-to-end by whole-body rows.
4. **Census** (`topo::census`): stays planar-exact; the
   `CensusUnsupported` boundary text names the frontier explicitly.
   *(OQ5 has since CLOSED — the boundary text's target is now
   CONTACT-DESIGN's classes and recourse, not a deferral; see that
   doc's C8 refusal migration.)*
5. **`merge_coplanar_faces`**: the structural/declared rungs are
   already kind-agnostic in principle; the op generalizes to
   same-surface (cosurface) merging for curved seams the boolean zip
   manufactures (a cylinder split by a through-cut re-merging its
   wall pieces) — same ladder, same never-numeric rule, N3 naming
   semantics unchanged. (M4's GeomSource retirement makes the
   declared rung a provenance lookup before M5 starts — this item
   consumes that, not bit_identity.)
6. **Tessellation** (`mesh::curved`): UV-grid + CDT extends to NURBS
   faces with pcurve-driven trim loops (pcurves are the trim-loop
   polylines' source — C4's consumers begin here); the
   certified-conservative chordal bound for NURBS patches comes from
   second-derivative hull bounds (C9 machinery — closed-form sagitta
   generalizes to hull-bounded Hessians). Watertightness across
   shared curved edges keeps the compute-chords-once-per-edge rule.
7. **Mass properties** (`geom-brep::props`): divergence-theorem
   contributions for NURBS-walled faces need certified quadrature
   (interval/hull-bounded remainder) — the first quadrature in the
   kernel; scope-boxed to what M5 acceptance shapes need (D4-honest:
   certified bounds or typed refusal, no silent Gaussian trust).
8. **`Real`/solvers**: no new `Real` methods expected (ring ops
   suffice — C9/C11); small dense/banded LSQ and SVD (Hoffmann's
   2×3/3×4 systems, Givens/Householder with fixed order, in-house)
   join `geom-core::linalg` under the D9 fixed-shape rules;
   PERF-PLAN's monomorphization lesson applies to their hot use.
9. **Q5 discharge**: the curvo audit (depend/vendor/study) runs at
   M5 start with a written verdict; the default stance (reference +
   test oracle, vendor specific algorithms if their invariant
   retrofit is cheaper than reimplementation) is DESIGN.md's, and the
   audit either confirms it or proposes the change to Evan. truck +
   opencascade-rs join as SSI/boolean test oracles per the standing
   review policy (real e2e comparisons, not diff-reading).

## The M5 envelope (typed frontiers, proposed — the honest-scope section)

- Curved boolean results that TOUCH → typed 3′ refusal (C7; census
  stays planar). Transverse curved booleans are the milestone.
- `TangentIntersection` from *classification of independently modeled
  tangent pairs* → escalated sliver (F6) unless structurally/
  declaredly coincident — the repair/adoption op (D7-style, banked at
  M3's F6 text as "M5+") is still NOT in M5; near-tangent operands
  fail loudly with the one-step resolution story documented.
- Offsets/shelling → M5+ (Q8 stands; C8 builds reusable machinery).
- Variable-radius fillets, chamfers as features → post-M5 feature
  breadth (Band 3); chamfer-as-two-plane-boolean composition works
  today and is the documented dodge.
- HLR/silhouettes (SSI-grade per Band 3) → untouched.
- STEP: M4's export scope grows to conics + NURBS entities
  (AP203/214 have exact forms for both — Book ch. 12 §12.3.2); import
  stays M7.

## D7 leave-room obligations (mostly mechanical; listed so M5 cannot foreclose M7)

Import adoption (D7) and the native repair/adoption op (F6's sliver
resolution, banked "M5+") stay out of M5. What M5 must *leave room
for* — each an obligation on M5 code shape, none a new decision:

- **Certifiability from data alone.** Every intrinsic description's
  certificate (C2, C7) must consume only the description + caches —
  never construction-context side data — because adoption's whole
  mechanism is re-running exactly these certificates on reconstructed
  descriptions (D7: "adoption reuses the kernel's own certification
  machinery"). This is already the M2 contract; the flag is to keep
  it true through the fitted-cache generation (e.g. the C2.3 tube
  must be re-derivable from witness + carrier, not from marcher
  internals that import will never have).
- **Tolerance as an explicit argument internally.** ε stays one
  global per run (D4 ¶1, untouched); but D7's ε_in ("governs
  interpretation") means the *classification* layers C5 builds
  (configuration trileans, tangency-vs-transverse dispatch) will be
  re-run at a different tolerance by adoption. Predicates already
  take `Tolerance` as a value — M5 must not regress this by baking
  the global into new classification code paths.
- **Error text names the future op.** The F6-mandated escalations on
  near-coincident/near-tangent *operands* (C5, the envelope) should
  point at the explicit repair/adoption operation as the resolution —
  the `FullRevolveHoles` precedent: a standing rule whose error text
  names the front door that does not exist yet.
- **Recognition's substrate.** D7 step 1 (NURBS-within-ε-of-analytic
  promotion) will want closest-analytic-fit machinery; C11's fitting
  stack and C9's enclosures are its substrate, and nothing more is
  built for it at M5 (non-goal reaffirmed — no feature recognition,
  D7's own text).
- **`TangentIntersection` is the pressure-test variant.** D2 predicts
  imported fillets force the intrinsic form; C8 storing native
  trimlines as `TangentIntersection` from birth is what makes M7's adoption
  of imported fillets a reconstruction into an *existing, certified*
  variant rather than a taxonomy scramble.

## Open questions for Evan (genuine forks)

**OQ1 — Conic carriers: how far up the ladder?** (a) `Ellipse` only
(bounded cuts; plane×cone generic-tilt refuses to rung 3 or typed);
(b) full conic trio as variants (parabola/hyperbola are unbounded
Line-like loci; plane×cone closes exactly); (c) no conics — fitted
NURBS for everything past `Circle`. C1 recommends (b)-staged-via-(a):
`Ellipse` lands with plane×cylinder booleans, the trio decision rides
on whether plane×cone acceptance shapes make M5. The real fork is
taste about enum growth vs uniformity — D3 licenses either.

**DECIDED (Evan, #85, 2026-07-24) — pending whole-doc ratification:**
(b) staged via (a), as recommended.

**OQ2 — Certificate strength staging.** Is the C2.2 hull-bound
(sup-norm-honest) certificate an M5 *entry* requirement for fitted
caches, or does a schedule-max-only stage ship first with the hull
bound following inside the milestone? (C2 recommends hull bounds
before any fitted cache reaches an at-rest body; the cost is real —
it is the difference between "certified" meaning what D4 says vs
meaning "spot-checked" for the one cache class with no closed-form
backstop.) Related sub-fork: is the C2.3 uniqueness tube required for
every fitted `Intersection` at rest, or only where the exhaustiveness
pass (C3) found multiple branches for the pair? (Recommendation:
always — the witness semantics is a kernel invariant, not a
circumstance.)

**DECIDED (Evan, #85, 2026-07-24) — pending whole-doc ratification:**
both halves as recommended — hull bounds are an *entry* requirement
(no fitted cache reaches an at-rest body on a schedule-max-only
certificate), and the C2.3 uniqueness tube is required for every
fitted `Intersection` at rest. This discharges T1's flag: "residual
≤ ε" keeps a single strength across all cache classes.

**OQ3 — Where does the exhaustiveness gate sit?** (a) Inside the
boolean/SSI op: the op does not return until every branch is found or
it refuses typed (recommendation — matches fail-loud and makes curved
boolean results unconditionally trustworthy); (b) at-rest tier
obligation (tier 3 gains an SSI-completeness clause; ops may return
uncertified intermediates); (c) a separate certification lane (the
banked text's "preview may march uncertified" knob — but M5 has no
preview lane yet, so (c) collapses to (a) until editor-core preview
exists). The banked principle fixes the *contract* (found-or-typed,
never silence); the placement is genuinely open.

**DECIDED (Evan 👍, #85, 2026-07-24) — pending whole-doc
ratification:** (a) in-op — the boolean/SSI op does not return until
every branch is found or it refuses typed. Recorded ground (the
deciding asymmetry): every other tier-3 obligation certifies *caches*
against a fixed topology, so a failure means "this body's numbers are
bad"; SSI exhaustiveness decides the topology *itself* — a missed
branch is a missing edge/face/loop — so an uncertified intermediate
under (b) would be a body whose connectivity may simply be wrong,
consumed by downstream ops before the at-rest gate can refuse, with
the failure surfacing far from its cause. (c) is not foreclosed: a
future editor-core preview lane relaxes the gate for
explicitly-preview results rather than moving it for real ones. Cost
is mitigated twice: the exclusion subdivision doubles as the
marcher's seed generator (C3), and the box tree is the C10 BVH — one
structure, two duties.

**OQ4 — Pcurve-primary vs carrier-primary for the general rung.** The
ℝ⁴ trace produces pcurves natively; C4 recommends keeping the 3-D
carrier as the primary fitted cache (existing witness/dihedral/
certification plumbing) with pcurves fitted alongside. The symmetric
alternative — fit pcurves, derive the 3-D cache through the chart —
is cleaner for trimmed-NURBS-heavy futures and slightly worse for
everything M5 actually validates. Cheap to flip now, expensive later.

**DECIDED (Evan 👍, #85, 2026-07-24) — pending whole-doc
ratification:** carrier-primary. Recorded grounds: every existing
certified mechanism keys off the 3-D carrier (witness = carrier(mid),
certification schedules, dihedral/sector classification, `split_edge`
parameter bounds, endpoint pinning — nothing re-plumbs), and — the
invariant-shaped argument, beyond plumbing inertia — the edge's
parameter stays **chart-neutral**: pcurve-primary must pick one
face's chart as the parameter source, privileging a side with no
principled tie-break (seam edges make "which side" degenerate).
Honest counterweight on record: if post-M5 work goes heavily
trimmed-NURBS, pcurve-primary would have been the leaner substrate;
the flip was offered pre-implementation and declined with eyes open.

**OQ5 — Curved 3′/census.** Confirm the C7 recommendation that M5
curved booleans refuse touching results (census stays exact-on-planar)
rather than extending the census to curved inventories mid-milestone.
The alternative (curved census sweeps for the
coincident-cylinder/tangent-contact classes) is a real
coincidence-ladder design of its own; recommending it wait for its
own doc.

**DECIDED (Evan, #85, 2026-07-24) — pending whole-doc ratification:**
as recommended — the census stays exact-on-planar through M5; curved
boolean results that touch refuse typed at the 3′ gate; the curved
coincidence census waits for its own design doc.

**CLOSED (Evan 👍, PR #178, 2026-08-04):** the design doc the
deferral waited for exists and is RATIFIED —
`docs/CONTACT-DESIGN.md`, proposals C1–C8 (the pair-germ census,
the structural-only conformality boundary with the
correctly-scoped identity lemma, CurveContact/PatchContact
records, the Rest/Tangent/Fit declaration vocabulary with
per-class verification tables, the signed gap co-designed for
M10, interference-fit semantics, the join-lane target, and the
disposition itself). Ratification changes no verdict on any body
(CONTACT-DESIGN C8's invariant); implementation is sequenced
separately (banked past M6 unless M7 adoption pulls it). The
refusal-text migration named in CONTACT-DESIGN C8 rides any
touching PR.

**OQ6 — Fillet scope box.** Closed smooth chains + three-convex-edge
sphere corner, everything else `FilletCornerUnsupported` (C8's
recommendation)? Or narrower (closed chains only)? And which run-out
policy vocabulary should exist even as typed-refusal names? This is
the survey's termination/corner problem — the one part of fillet
scope where reasonable kernels genuinely differ.

**Scope DECIDED (Evan, #85, 2026-07-24) — pending whole-doc
ratification:** closed smooth chains AND the three-convex-edge
sphere-octant corner are in scope; the **die-with-pips demo upgrade**
(closed chains on the pip rims, open chains terminating in octant
corners on the cube edges) is the named acceptance target. Everything
else refuses `FilletCornerUnsupported`.

**Run-out vocabulary DECIDED (Evan 👍, #85, 2026-07-24) — pending
whole-doc ratification:** the minimal two-policy vocabulary ships as
**refusal-payload names only** — `FilletCornerUnsupported` carries an
enumerated payload naming the configuration hit and the policy that
would handle it: `RunOutStopAtVertex` (blend runs full-radius to the
vertex; a corner patch fills the junction — the sphere-octant corner
is its three-convex-edge case), `RunOutFeather` (radius decays to
zero before the vertex; the blend fades into the sharp edge), plus
corner-configuration tags for the N-edge / mixed-convexity vertex
cases. Honest, actionable frontier error text (the `FullRevolveHoles`
precedent: errors name the front door that does not exist yet), zero
constructor surface; the finer taxonomy (per-end assignment, setback
parameters) is left to the post-M5 design that implements run-outs.

**OQ7 — Symmetric prefer-intrinsic enforcement.** Extend tier 3 with
definitely-tangent-smooth-contact ⇒ must-carry-`TangentIntersection`
(mirroring `TransverseNotIntrinsic`)? Recommended in principle
(unenforced preferences drift — the ratified rationale), but it
redraws the boundary between conventional `MappedCurve` smooth joins
(G2 sketch joins stay conventional BY DESIGN — D2's G2-join story)
and genuinely intrinsic tangencies (fillet trimlines). The
discriminator is "do the surfaces determine the locus" — sharp in
theory; the enforcement predicate needs a margin story at the
boundary and Evan's eyes on which side conventional splits land.

**DECIDED (Evan 👍, #85, 2026-07-24) — pending whole-doc
ratification:** Evan's two-level shape is adopted and folded into C7:
(i) **the mark** — every definitely-tangent edge carries the tangency
*verdict* as a named recorded classification (tier 3 already samples
dihedrals per edge; the same data kept as a verdict instead of
discarded) — this is what stops the preference from drifting, at zero
enforcement risk; (ii) **the must-carry rule** (`TangentNotIntrinsic`,
the `TransverseNotIntrinsic` sibling) fires only on
**jet-determinate** tangencies — definitely-tangent AND second-order
separation definite — so G2 conventional joins are exempt *by the
predicate itself* (their second-order margin is zero-side; the
surfaces under-determine the locus), never by an exemption list;
in-band second order escalates per F6. The discriminator "do the
surfaces determine the locus" is thereby the enforcement predicate,
not prose. **Rename decided:** the variant is
`TangentIntersection { s1, s2, witness }` (mirroring `Intersection`
— same shape, same witness pin, margin one differential order up),
landed as an explicit **D2 sharpening** in DESIGN.md and applied
throughout this doc.

**OQ8 — The in-house interval ring** (C9). Sign off on adding a
second interval type (ring-only, MIT-clean, ulp-widened) alongside
inari rather than either (a) putting inari on the default path or
(b) blocking curved certification on the Tabled full in-house
interval program. This is a licensing/architecture fork, small code,
long shadow.

**DECIDED (Evan, #85, 2026-07-24) — pending whole-doc ratification:**
the in-house ring is approved, doubling as the seed of the eventual
inari replacement; see the C9 transition note for the temporary
inari-on-default-path allowance while the ring lands.

**OQ9 — Q5 closure trigger.** The curvo audit verdict will land
during M5 planning; if it recommends vendoring specific algorithms
(most likely: fitting/knot machinery, possibly their SSI seeding
heuristics as reference), does that change the build-vs-study stance
enough to warrant a DESIGN.md Q5 revision, or is a memories/audit
note sufficient? (Process question as much as technical.)

**DECIDED (Evan, #85, 2026-07-24, deferred-to-judgment; the revision
path is taken) — pending whole-doc ratification:** the audit verdict
lands as a lean DESIGN.md Q5 revision — standing stance and its
supersession in one place, no re-litigating — pointing at the audit
note for detail.

## Tensions (ratified decisions under curved pressure — flagged, not reopened)

**T1 — D4 ¶2's "certified" vs sampled schedules.** The ratified text
already anticipates this ("initially a conservative numerical
estimate, upgraded to an interval-verified bound when Q1's machinery
lands"). The machinery now exists; fitted caches are the first class
where the sampled estimate is *structurally* weaker than the
closed-form classes it was written for. OQ2 is the decision; the flag
is that "residual ≤ ε" silently meaning two different strengths for
two cache classes is the kind of asymmetry this project exists to
avoid.
*(Resolved by the OQ2 decision, #85 2026-07-24: hull bounds are an
entry requirement, so "residual ≤ ε" keeps a single strength across
cache classes — the two-strength state this flag named was the trap
the C2 design avoids, never a feature, and the staging door that
could have reintroduced it mid-milestone is closed.)*

**T2 — Interval certification on the default path.** C2/C3 make
enclosures load-bearing for default-build validation of curved
bodies, which the interval backend then had to support without
dragging a copyleft dependency into every consumer. *(RESOLVED by
removal, M5 PR 1 (#127): the in-house `interval-transcendentals`
crate replaced inari outright and its LGPL stack left the tree, so
there is no quarantine boundary to draw. See DESIGN.md's crate
table.)*

**T3 — Witness component-selection semantics.** D2's S2 sharpening
says the selection semantics "activates with real SSI at M5." It
activates as a *proof obligation* (C2.3's tube), and acquires a new
escalation class (two branches within a band of the witness — the
operand pair is a sliver). This is a strengthening consistent with
the ratified text, but the tier-3 validator grows a check family, and
"escalated" here can fire on *operands* users consider innocent
(near-osculating cuts). Worth saying out loud before it ships.

**T4 — D9 and iterative numerics.** Marching/Newton/SVD/fitting are
the first substantial iterative f64 pipelines in the kernel.
D9-compliance is achievable (libm-only, fixed iteration orders/counts
policies, in-house solvers) but is a *discipline* across a large new
code surface, not a one-time proof — the M5 CI must extend the
bit-replay suites to SSI outputs early, not at exit (the M1 lesson:
pin the contract while the code is small).

**T5 — K = 10.** K-REPORT's scope honesty predicted the
discriminating evidence would come from boolean/SSI predicates. C7's
second-order family and C2.3's branch-separation margins are that
corpus. The M5 exit K-snapshot is not optional bookkeeping; it is the
report's own revisit condition firing.
*(#85 tangent, 2026-07-24: Evan raised whether K should be much
larger — "any sliver that looks exactly-equal in the GUI is probably
a mistake." Split to issue #89 rather than decided in #85: the
kernel-K half waits on the M5 exit K-snapshot (this flag's own
trigger); the GUI-indistinguishability half is proposed there as a
scale-relative document/editor-layer lint, not a kernel refusal.)*

**T6 — Tessellation cost.** PERF-PLAN rank 1 (CDT quadratic) was
measured on analytic UV grids; trimmed NURBS faces raise point counts.
CDT bulk-loading's trigger ("first real fine-δ export need, or corpus
CDT dominance") very likely fires during M5 — noted so it is planned
as an M5-adjacent PR, not discovered as a regression.

## Appendix — condensed literature notes (in lieu of references/notes, which is git-ignored)

**Hoffmann, *Geometric and Solid Modeling* (1989), ch. 6 "Surface
Intersections" (pp. 205–255).** §6.2 (pp. 207–219): tracing two
implicit surfaces — local parameterization r(s) analytic at regular
points; approximant from the underdetermined system ∇f·r⁽ᵐ⁾ = b_{f,m},
∇g·r⁽ᵐ⁾ = b_{g,m} (Eq. 6.1, p. 209; b's from lower-order data, worked
to third order pp. 210–211); solve by SVD (A = VSUᵀ, 2×3), free
coefficients chosen via the Frenet frame — γ₂ = 0 makes r″ = κn,
γ₃ = −κ² (pp. 212–214), so the degree-3 approximant carries curvature
and torsion ("a good balance", p. 208); step size: keep quadratic and
cubic terms < |s₀|/10 with a minimum step (p. 215 — a heuristic, and
labeled as such); Newton refinement ∇f·Δ = −f, ∇g·Δ = −g with the U₃
(tangent) component of Δ zeroed, stop at relative 10⁻ᵗ, t ≈ 10
(pp. 215–216); transversality ⇔ σ₂ > 0, tangent = U₃ ∥ ∇f×∇g
(p. 217). §6.3 (pp. 220–229): the same machinery for n−1 hypersurfaces
in ℝⁿ; §6.3.2 (pp. 222–223): parametric×parametric as
G₁ − G₂ = 0 — three equations in (u₁,v₁,u₂,v₂), the traced ℝ⁴ curve
projects to both pcurves and recovers the 3-D curve through either
chart. §6.3.3 (pp. 224–229): offsets/blends as envelopes — the sphere
family S(x,y,z,α₁,α₂) with S = 0, ∂S/∂α₁ = 0, ∂S/∂α₂ = 0
(Eqs. 6.7–6.9, p. 225) — the defining system for canal/offset loci in
one higher dimension (the two-sided offset is algebraic even though
the unit-normal formula is not, p. 224). §6.4–6.5 (pp. 230–252):
mapping SSI to plane algebraic curves and tracing *through*
singularities via quadratic transformations — read and deliberately
not adopted (we classify-and-refuse at tangency; F6). His framing of
the efficiency/robustness/accuracy triangle (p. 205) and "the
numerical approach cannot be used without considerable changes near
singularities" (p. 208) are the chapter-level endorsement of
march-then-certify with typed refusal.

**Piegl & Tiller, *The NURBS Book* 2e.** §9.4.4 (pp. 424–435),
approximation to within accuracy E: Type 1 (add knots) / Type 2
(fit + bounded knot removal) loops; deviation measures Eq. 9.77
(at-parameter) vs 9.78 (max-norm via projection, Eqs. 6.4–6.5);
"both … can fail to converge, and this eventuality must be dealt
with" (p. 427 — their own fail-loud note); knot-removal error bounds:
one-removal control-point perturbation B_r (Eqs. 9.80/9.82) gives
|C(u) − Ĉ(u)| ≤ N_{r−k,p}(u)·B_r (Eq. 9.81) resp. the odd-parity form
(Eq. 9.83), surfaces via row sums (Eqs. 9.86–9.89, pp. 433–434) —
all convexity/partition-of-unity facts, the same mechanism C9's hull
bounds generalize; algorithms A9.8 (bounds), A9.9 (bounded removal),
A9.10 (GlobalCurveApproxErrBnd, p. 431). Note for C2: these bounds
certify deviation between successive *fits* and from *data points* —
locus-distance certification is ours. Ch. 7 (pp. 281–330): conics as
quadratic rational Béziers — shape factor k = w₀w₂/w₁² (Eq. 7.25,
p. 292), ellipse/parabola/hyperbola ⇔ w₁² <,=,> 1 (p. 293); w₁ < 0
loses the convex hull (p. 293 — grounds for the positive-weights
invariant); circular arc w₁ = cos θ (Eq. 7.33, p. 295); ≥ 180° arcs
need infinite control points (§7.4, pp. 295–297) — fine for
export forms, unwanted in kernel carriers (C1). §5.2–5.5
(pp. 141–212): knot insertion/refinement/removal/degree elevation —
the split/refit toolkit; §6.1 (pp. 229–234): point
projection/inversion (Newton on orthogonality conditions) — C2.1's
foot points. Ch. 10: skinning §10.3, sweeping §10.4 — M5's
loft/sweep features, definitional per Q8.

**Vida, Martin, Várady, "A survey of blending methods that use
parametric surfaces," CAD 26(5) 1994 (pp. 341–365).** Terminology
adopted (Fig. 4 p. 344): base surfaces, trimlines, profile,
assignment; spine + rolling ball (Fig. 6). Classification: global vs
local (p. 342 — local is the B-rep-native choice, "blending
operations which in principle are local may have global consequences
if … the ball … too large" — the r-vs-κ predicate's citation); edge
vs vertex vs region blends (pp. 342–343 — edge blends dominate the
literature and practice); blend *representations*: superficial /
implicit / parametric / procedural (p. 343) — our split is
"analytic where exact, else Q8 approximating surface with intensional
spec," which in their taxonomy is parametric with a procedural
definition retained as the certification target. Their repeated
practical warning — trimline/termination handling at vertices is
where published methods go quiet (p. 343) — is why OQ6 exists.

**Mäntylä**: not re-read for this doc; M5 consumes ch. 12–15 only
through the ratified M2/M3 machinery and the existing notes.
