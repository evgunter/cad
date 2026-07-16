# CAD Kernel — Design Document

**Status: v0.** Living document. Decisions marked *agreed* are settled unless
new evidence overturns them; items in [Open questions](#open-questions) are
under active discussion and get promoted here once ratified.

## Vision

A greenfield B-rep solid-modeling kernel in Rust, built API-first: the kernel
and its programmatic modeling API are the product; any GUI is a thin client
added later. Code quality and functional style are explicit goals — a CAD
kernel's job is to define what a shape *is*, and the implementation should
read that way.

**Reach goal (shapes the architecture even while deferred):** native error
propagation — define distributions over model parameters (e.g. a hole
diameter) and propagate them through the model to detect problems like
self-intersection and to compute tolerance stackups.

## The central commitment

> **A model is a pure, replayable function from a parameter vector to a
> solid.** `fn build(params: &Params) -> Result<Solid, ModelError>` —
> deterministic, no hidden state. The B-rep is a derived value, never a
> mutated-in-place object.

Everything else follows from holding this invariant from day one:

- **Error propagation** becomes "evaluate the same function with a different
  scalar type" (intervals, dual numbers, Monte Carlo samples) instead of a
  rewrite.
- **Undo, caching, and diffing** are free — models are values.
- **Testing** is property-based testing over parameter space.

The concrete data-shape consequence: the geometry *evaluation* layer is
generic over a scalar type `T` (default `f64`). Topology stays concrete.
See [Open questions → Scalar genericity](#q1-scalar-genericity) for the
detailed design under discussion.

## Decisions

### D1 (agreed): ID-based arenas, immutable values, manifold-first, Euler operators

- Topology entities (`Solid / Shell / Face / Loop / Edge / Vertex`) live in
  generational arenas (slotmap-style) and reference each other by typed IDs —
  never `Rc`/pointers. A B-rep is a plain value: cheaply cloneable (or
  structurally shared), serializable, diffable, validatable.
- **Manifold solids only** at first. Non-manifold (radial-edge) roughly
  doubles topology complexity; add a non-manifold representation later only
  if sheet/wire bodies demand it.
- Topology is built **exclusively through Euler operators** (Mäntylä's
  `mev`, `mef`, `kemr`, …): a small closed set of primitives that provably
  preserve the Euler–Poincaré invariant. Higher-level operations are
  compositions of validity-preserving pure steps; debug builds check
  invariants after each step.

### D2 (agreed, revised 2026-07-15): Topology and geometry separated; edge/vertex geometry is intensional where possible

Topology and geometry live in separate arenas: faces reference surfaces,
edges reference curves, vertices reference points.

**Background: pcurves.** A surface is a map `S(u,v) → ℝ³`. A face is a
region of that surface's 2-D parameter plane, and each of its boundary
edges is therefore also a curve `P(t) → (u,v)` in that plane — the *pcurve*
("parameter-space curve"). An edge shared by two faces classically carries
*three* representations: a 3-D curve `C(t)` plus one pcurve per adjacent
face, with the consistency requirement `Sᵢ(Pᵢ(t)) ≈ C(t)`. Pcurves are not
optional — trimming tests, tessellation, and intersection marching all
happen in (u,v) space. The redundancy among the three peer representations
is a classic bug farm in every kernel.

**Our rule:** an edge's geometry is stored as an *intensional description*
of what the locus **is**; all concrete representations (`C(t)`, both
pcurves) are derived caches carrying certified residual bounds against the
described locus (see D4). Sketch of the sum type:

```text
EdgeGeometry =
  | Intersection { s1, s2, witness }  -- transverse surface–surface intersection;
                                      -- the edge is the connected component of
                                      -- S₁∩S₂ selected by the witness point
                                      -- (also the marching seed)
  | MappedCurve  { source, map }      -- pushforward of a lower-dim entity, e.g.
                                      -- a sketch edge/vertex under an
                                      -- extrude/sweep/revolve map
  | Seam         { surface, pcurves } -- same surface on both sides (closed-
                                      -- surface parameterization seam)
```

**Deliberately omitted: an `Explicit` (extensional) variant.** Taken as an
unconditional challenge: every edge must have an intensional description —
there is no escape hatch, so it can't be reached for when not absolutely
necessary. This holds even for imported geometry (see D7): the intrinsic
variants are checkable properties of the geometry as it now stands, and
the conventional variants carry their own defining data — so extensional
input can be *adopted* by reconstructing (or directly adopting) the
description it satisfies rather than admitted as second-class data. What import pressure-tests is
the **completeness of the variant taxonomy** (e.g. imported fillets force
`TangencyLocus`), not the need for an extensional fallback.

Validity of `Intersection` requires *transversality*: normals of S₁, S₂
linearly independent along the locus (equivalently `T_pS₁ + T_pS₂ = ℝ³`),
so the implicit function theorem makes S₁∩S₂ locally a 1-manifold. The
transversality margin (angle between normals) is a predicate-with-margin
(Q1) and governs the conditioning of every derived cache. Cases that fail
transversality get other variants: parameterization seams (`Seam`),
tangential contact such as fillet–support contact curves (a future
`TangencyLocus` variant — the fillet construction knows its contact locus
directly, but *imported* fillets force the intrinsic form: along a fillet
boundary edge the blend and support surfaces share tangent planes
identically, so `Intersection`'s precondition fails everywhere on the
locus). `TangencyLocus`'s intrinsic validity condition sits one
differential order up: surfaces coincident within ε and normal-parallel
within the derived angular threshold ε·κ_rel (D4 ¶1: lever arm
r = 1/κ_rel) *along* the locus, separating quadratically *transverse* to it
(relative normal curvature bounded away from zero — otherwise the
surfaces osculate over a patch and the "locus" is not a curve). The
uniform pattern: **every variant is a validity predicate plus a margin**
(Q1) — first-order (normal angle) for `Intersection`, second-order
(relative transverse curvature) for `TangencyLocus`. Reconstructing a
tangency locus from data is well-conditioned *despite* the tangency
because its defining system includes the first-order (normal-alignment)
equations, not just surface coincidence — the normal angle grows linearly
with transverse distance, and the second-order margin is the
implicit-function-theorem denominator for that jet system. (Order-k
contact generalizes: defining equations from the k-jet, margin at order
k+1.) In the intensional variants the invariant "the locus lies on both
surfaces" holds *by definition*; only the numerical caches need
certification. Vertices generalize the same way (intersection of three
surfaces / endpoint of a locus, with a witness point).

**Prefer-intrinsic rule.** Wherever an intrinsic description is
certifiable, it *is* the stored description — including for native
constructions: a fillet we build stores its boundary edges as
`TangencyLocus`, with the rolling-ball construction demoted to supplying
the witness and initial caches. Construction history lives in D5
provenance, never in the geometry description, so native and imported
bodies carry identical descriptions. The taxonomy is thus a dichotomy:
**intrinsic variants** (`Intersection`, `TangencyLocus`) describe loci
determined by their surfaces; **conventional variants** (`Seam`,
`MappedCurve`) carry the defining data for loci the surfaces *under*-
determine — parameterization seams (infinite-order contact; the seam's
position is pure convention), face splits at smooth profile joins
(iso-curve edges introduced by sketch entity boundaries; at a G2 join
even `TangencyLocus` fails its margin, and rightly — nothing intrinsic
distinguishes that curve from its neighbors), and user splits.
`MappedCurve` does not reintroduce `Explicit` through the back door
because of its shape: one authoritative source (`curve = map ∘ source`,
pcurves derived as certified caches), never two peer representations
needing cross-reconciliation. A locus in the ambiguous band — a dihedral
within a few derived angular thresholds of tangent (θ ≲ K·ε/r at the
governing lever arm), certifiable as neither `Intersection` nor
`TangencyLocus` — fails loudly at construction exactly as at import (D4);
a conventional description is not an escape hatch from ill-conditioned
geometry.

This makes D5's provenance load-bearing rather than bookkeeping: the
intensional description largely *is* the provenance.

### D3 (agreed): Analytic surfaces special-cased; NURBS as the general fallback

Plane / cylinder / cone / sphere / torus are first-class variants alongside
NURBS (as in Parasolid), not converted to NURBS. Most mechanical geometry is
analytic; analytic×analytic intersections have closed forms (exact,
robust), while NURBS×NURBS intersection is a numerical marching problem we
defer as long as possible.

**Extensibility:** surface kinds form a *closed enum*, not open trait
objects. Intersection requires pairwise dispatch (plane×cylinder,
cylinder×torus, …) and an open set makes that table unmanageable; a closed
enum gives compile-time exhaustiveness checking, so adding a new analytic
kind means adding a variant and letting the compiler enumerate every
dispatch site that must handle it. The `Nurbs` variant is the universal
fallback: any exotic surface is at minimum representable, and any
unimplemented analytic×analytic pair can fall back to the general path.
Same design for curves (line / circle / ellipse / … / NURBS).

### D4 (agreed, ratified 2026-07-15): Single strict global tolerance; operations fail loudly

No per-entity tolerances that grow as operations get sloppy (the Open
CASCADE model, where errors snowball silently). "Define what something is"
applied to error handling. Five commitments:

1. **One number, global per run** *(revised 2026-07-16 — originally "two
   numbers" with a global angular tolerance εₐ)*: a linear tolerance ε,
   defined once in `geom-core` as a `Tolerance` value. Compile-time
   constant vs. once-initialized at startup is an implementation detail
   (resolved: once-initialized, env-overridable per run — PR #3); the
   invariant is **one value per run, shared by all bodies, never
   loosened mid-run** (per-model ε is deliberately rejected: any two
   bodies must be boolean-combinable, and per-model ε recreates
   mixed-tolerance semantics one level up). Per-run initialization also
   enables running the test suite at several ε values to smoke out
   tolerance-sensitive algorithms.
   **Angular thresholds are always derived, never a second global**: an
   angle only means anything through the displacement it induces at a
   lever arm (d = r·θ), so a fixed εₐ would silently privilege the
   hidden length scale L* = ε/εₐ. Every angular predicate uses θ = ε/r
   with its lever arm named at the call site — 1/κ_rel for tangency
   classification (making "normal-parallel within θ" ⟺ "within ε of the
   locus"), the face extent for parallelism decisions, the session-box
   extent as the conservative universal arm.
   Exact ε default chosen empirically at M0; ε ≈ 1e-9 m gives
   micron-to-kilometer coverage with ~4 orders of f64 headroom at km
   scale. Import does *not* motivate loosening ε — see D7's input
   tolerance ε_in.
2. **Every derived cache carries a certified residual bound** against its
   intensional description (D2): fitted intersection curves, projected
   pcurves, refit 3-D curves. Kernel invariant: `residual ≤ ε` for every
   derived item in a valid body; the `topo` validator checks it.
   "Certified" is initially a conservative numerical estimate, upgraded to
   an interval-verified bound when Q1's machinery lands.
3. **Failure is a typed, actionable error**: `ToleranceExceeded { entity,
   achieved_residual, required, operation }` — consumable by humans and by
   the error-propagation machinery. Geometry that can't meet ε
   (near-tangent surfaces, sliver faces) almost always indicates a modeling
   mistake or an unstable design; surfacing it beats absorbing it.
4. **Fixed internal units — meters and radians — with a documented model
   size range** (Parasolid session-box style); geometry outside the range
   is rejected at construction. User-facing units are typed newtypes at the
   API boundary only (see D6).
5. **Strictness is enforced at the boundary, not relaxed inside**: future
   STEP *import* gets a separate adoption/healing stage (D7) that brings
   external geometry up to kernel invariants *before* it becomes a kernel
   body; entities that can't be adopted fail loudly in a typed import
   error.

### D6 (agreed): Canonical internal units; typed units at the API boundary

Kernel-internal code is raw `T` in meters/radians by convention — no
dimensional types inside. The public API uses hand-rolled newtypes
(`Length`, `Angle`, …) that convert on entry. Hand-rolled rather than
`uom`: uom's dimensional generics fight the scalar-type parameter and we
need ~five quantities, not the SI lattice.

### D7 (agreed): Import is adoption, not admission

Imported geometry is not second-class. Rather than adding an extensional
escape hatch to `EdgeGeometry`, import **reconstructs** the intensional
description that the extensional data satisfies. This is possible because
the intrinsic variants are properties of the current geometry, not
history, and the conventional variants (`Seam`, `MappedCurve`) carry
their own defining data — for those, the imported curve isn't *evidence*
of an intrinsic fact, it *is* the convention, adopted directly as the
defining data. Pipeline sketch:

1. **Surface recognition**: an imported NURBS within ε of an analytic
   surface is promoted to it (plane/cylinder/cone/sphere/torus
   recognition) — restoring D3's exactness benefits to imported bodies.
2. **Edge adoption**: for each imported edge, verify the imported curve
   lies within ε of the intersection of its two adjacent surfaces with an
   adequate transversality margin, then rebuild it as
   `Intersection { s1, s2, witness }` — the imported extensional curve is
   demoted to witness point + initial cache. Seams and tangency loci are
   recognized likewise.
3. **Healing**: where no intensional description is satisfied within ε
   (gaps, sloppy source tolerances), repair (refit/nudge) or fail loudly
   with a typed error naming the unhealable entities (D4 ¶5).

**Adoption tolerance ≠ kernel tolerance.** The generator's precision is
unknown and usually worse than ε, so adoption takes a per-import *input
tolerance* ε_in — defaulted from the STEP file's declared
`uncertainty_measure_with_unit`, overridable per call. The two play
different roles: **ε_in governs interpretation** (recognition and
classification tests — what the extensional data is evidence of); **ε
governs what gets built** — once classified, an adopted entity's caches
are recomputed from its intensional description by our own algorithms and
certified at ε like native geometry, so imported bodies are genuinely
first-class. Healing may move geometry by up to O(ε_in) to make the
chosen interpretation true — a reported model change (e.g. max
displacement), never a loosened certification. Data ambiguous at ε_in
scale (multiple consistent interpretations) fails with a typed ambiguity
error rather than a silent guess.

**Non-goal: feature recognition.** Adoption recovers *what each locus
is*, not *how the body was modeled*. Recognizing "these faces are a
radius-r rolling-ball fillet" (design-intent / feature recognition — a
hard, heuristic research problem) is not required for first-class
validity; it would add only *editability*, and is out of scope for M7.
Consistently: imported bodies carry no parameters, so error propagation
(M6) has nothing to vary over them.

Adoption reuses the kernel's own certification machinery — "is this curve
within ε of the described locus" is exactly the check the `topo` validator
already runs on derived caches. Note this is strictly *stronger* than
industry "shape healing" (which only patches data into self-consistency):
adoption must *explain* the data. Export is the easy direction (projection
from intensional to extensional); import is the inverse problem and is
deferred accordingly (M7).

### D8 (agreed 2026-07-15): The recipe is data

A model document is an operation DAG — typed feature nodes referencing
parameters and each other — plus a small expression sublanguage for
derived quantities (`hole_x = width/2 - margin`). The kernel interprets
the recipe at any scalar `T`; user-facing Rust is a *generator* of recipes
(loops that make N holes run at the structural level). Consequences
banked: the recipe is the save format; recipe node IDs are the substrate
for D5 naming; every value-dependent branch stays inside kernel code where
predicates are reified (Q1) — user models as generic Rust functions were
rejected because `if width > 10.0` in user code would silently break
interval replay; and structural parameters (hole *count*) are explicitly
distinct from continuous ones (hole *diameter*), so parameter-driven
topology change is stated, not emergent.

### D9 (agreed 2026-07-15): Determinism policy and engineering charter

- Same build + same inputs → bit-identical outputs. No hash-map iteration
  order may influence geometry; parallelism only in fixed reduction
  shapes.
- Transcendentals via the pure-Rust `libm` crate: system libm sin/cos
  differ across platforms in the last ulp — enough to flip a marginal
  predicate.
- The kernel never panics on any input: panics are bugs; every failure is
  a typed error.
- Essentially no unsafe Rust outside vetted dependencies.

### D5 (agreed): Persistent topological identity from birth

Every topological entity carries a provenance record from the moment it is
created: which operation created it, from which inputs ("side face swept
from sketch edge #3"). This does not solve the topological naming problem —
the most user-visible unsolved problem in parametric CAD — but recording
identity at birth is cheap, and retrofitting it onto anonymous entities is
nearly impossible. The parametric layer (M4) builds its stable references
on top of this record.

## Layering

Each layer depends only on the layers below it.

| Crate | Contents |
|---|---|
| `geom-core` | Scalar trait (`f64`, intervals, duals), 2-D/3-D points/vectors/transforms (hand-rolled, small, fixed-dim — we control the scalar trait), robust predicates, root finding |
| `geom-curves` / `geom-surfaces` | Analytic + NURBS types, evaluators, closest-point, curve×curve and curve×surface intersection |
| `topo` | Arenas, entities, Euler operators, validation (watertightness, orientation, Euler characteristic) |
| `kernel-ops` | Primitives; extrude/revolve/sweep (build B-reps directly, no booleans needed — hence early); then booleans; then fillets/shell/offset |
| `model` | Parametric layer: parameter space, feature DAG, persistent naming; later the sketch constraint solver |
| `mesh` / `interop` | Tessellation, STL export, STEP export (import much harder — deferred) |
| `viewer` | Deferred (GUI last). When needed: thin wgpu tessellation viewer, or `rerun` for zero-effort demos |

The API-first discipline falls out of this: layers 1–5 *are* the product,
exercised entirely by tests and code-driven models (CadQuery/OpenSCAD-style
usage as acceptance tests). The regression suite is mass-property checks
(volume/centroid vs. closed forms), watertightness validation, and
randomized parameter fuzzing — the fuzzing infrastructure is itself a
precursor of the error-propagation feature.

## Difficulty ranking (sequence around this)

1. **Fillets/blends, shelling/offsets** — hardest; even ACIS/Parasolid still
   get these wrong. Late, scope-boxed (constant-radius edge fillets on
   analytic geometry first).
2. **General surface-surface intersection (SSI) + robust NURBS booleans** —
   second hardest; deferred by D3.
3. **Booleans on analytic geometry** — hard but tractable; closed-form
   intersections make classification the main challenge.
4. **2-D sketch constraint solver** — a real subproject (graph decomposition
   + Newton) but well-trodden; parallelizable, possibly bind an existing
   solver (see open questions).
5. Everything else is careful engineering, not research.

## Roadmap

- **M0** — `geom-core`: scalar trait + intervals; arenas; validation harness.
- **M1** — Topology + Euler operators; build a cube by hand; watertightness
  and Euler checks pass.
- **M2** — Analytic curves/surfaces; extrude/revolve from polyline+arc
  profiles; tessellation; STL export. *(First "it's a CAD kernel" milestone —
  verified via exported meshes; demo viewer deferred.)*
- **M3** — Intersections for analytic pairs; booleans; mass properties.
  *(First useful parts.)*
- **M4** — Parametric model layer: parameter vector → feature DAG → solid;
  provenance-based naming; replay. STEP export.
- **M5** — NURBS depth (sweeps/lofts); first SSI marching; constant-radius
  fillets.
- **M6** — Error-propagation MVP: distributions over parameters;
  dual-number sensitivities of measurements (tolerance stackups);
  interval-based self-intersection / minimum-clearance checks over the
  parameter box. Sketch solver when sketches should become
  constraint-driven rather than programmatic.
- **M7** — STEP import as adoption (D7): analytic surface recognition,
  edge adoption, healing. Deliberately last — it is the inverse problem of
  everything above it.
- **Post-M7 (noted 2026-07-16)** — replace `inari`'s gmp/MPFR-backed
  interval transcendentals with an in-house rigorous implementation
  (proven per-function error pads over `libm`, plus monotonicity/extremum
  handling) so interval builds can drop the LGPL-3.0+ transitive
  dependencies; until then the `interval` cargo feature quarantines the
  copyleft obligation to interval-enabled builds only (issue #4).

## Open questions

### Q1: Scalar genericity (direction settled 2026-07-15)

Settled direction — **reified trilean predicates + a subdivision driver; no
persisted decision log**:

- Evaluation code (evaluators, derivatives, transforms, measurements) is
  fully generic over a `Real` trait we define. Instantiations: `f64`,
  `Interval` (inari `DecInterval`, behind the `interval` feature),
  `Dual<f64>` and `Dual<Interval>` (one in-house generic `Dual<T>` —
  num-dual was demoted to a dev-only test oracle at M0 because its
  std-backed transcendentals cannot satisfy the value-channel
  bit-identity contract; see crate table).
- Every topology-determining branch goes through a *named predicate
  function* returning a trilean sign (+ margin), generic over `T`. No raw
  `<` on control-flow paths — this code-style discipline is the one
  day-one commitment.
- At `T = f64` predicates are total (margins within K·ε escalate per D4;
  ratified in PR #5 as the *sliver band* — semantically indeterminate
  even under exact arithmetic, provisional K = 10; K is a policy dial —
  refusal rate and f64 noise headroom — not a correctness parameter:
  soundness rests on escalate-never-guess, D4 ¶2 certification, and
  interval replay, for any K > 1).
  At `T = Interval` an indeterminate predicate aborts the operation — in
  Rust, predicates return `Result<Sign, Indeterminate>` (the trichotomy
  is the primitive, ratified in PR #5; bool predicates are projections)
  and construction code propagates with `?` — unwinding to an outer
  **propagation driver**
  that splits the parameter box and re-runs (pure model ⇒ re-running
  sub-boxes is trivially correct and embarrassingly parallel). This is the
  operational form of "union over branches, pushing the distribution
  forward into each": leaves of the subdivision take definite branch
  paths; outcome probabilities are the distribution's measure on the
  sub-boxes.
- A persisted decision log (earlier proposal) is *dropped*: reified
  predicates are the load-bearing part, and margin logging can be added
  later as a pure diagnostic/optimization without restructuring.

Residue status: **`Real` trait surface — settled** (PR #3, 2026-07-16):
comparison-free by construction (no `PartialOrd`/`PartialEq` —
structural for the convenient paths, plus an explicit *evaluation-code
discipline* style rule and a CI tripwire for the residual channels:
extra bounds, `Debug`-string gadgets, `Any`/`TypeId`); all operations
total with poison propagation (NaN/empty) — poison flows through
*values*, never through *decisions*; `sin_cos` is the primitive (sin/cos
are projections, overridable only bit-identically); no fused operations
(`hypot`, `mul_add`/FMA) — cross-instantiation consistency outranks
last-ulp accuracy; no order-implicit reductions (`Sum`/`Product`).
`Tolerance` is once-initialized per run with env self-init
(`CAD_TOLERANCE_EPS`) and exhaustively recorded env errors — loud
through a test, never a panic. **Angular tolerance eliminated** (D4 ¶1
revision).
**All Q1 residue settled at M0 close (2026-07-16):**
- **Interval scalar** (PR #7): inari `DecInterval` with the *decoration
  as the poison channel* (`decoration < Def ⇒ Indeterminate(Invalid)` —
  silent domain clamps never decide); `Bounds` certification trait with
  poison-visible NaN brackets for empty AND NaI (failing certification
  outranks 1788 representational honesty); tight `pown` powi override
  (containment of the true value is the interval contract); the sliver
  band is *terminal* for a subdivision driver (an enclosure wholly
  inside (ε, Kε) never refines — escalate as a genuine sliver).
  f64 `powi(NaN, 0)` propagates NaN (un-laundered to match; `∞⁰ = 1`
  stays — ∞ is not f64 poison).
- **`Dual<Interval>` comparison/signum semantics** (PR #9/#10): resolved
  by *value-part delegation* — `Decide` classifies the value only, the
  derivative never influences a branch (tangent-space data does not
  decide base-space topology). Kink conventions: f64 tangents are
  branch-consistent with the value channel (the dual differentiates the
  program as evaluated — abs′(0) = +1, ties keep self); the interval
  instantiation carries the *Clarke subdifferential enclosure* (straddle
  hulls) — the set-valued subgradient treatment lives at the certified
  tier (cf. the GSD06 discrete-exactness philosophy, see references).
- **Genericity boundary** (PR #8): `Body<T>` = scalar-free topology
  arenas + `T`-valued geometry arenas; topology contains no `T` and
  never branches on it; keys are *body-lineage-scoped* (a foreign key
  may silently resolve — the flip side, key identity across
  same-history builds, is what lets an interval replay share topology
  with the f64 build). A non-generic `Topology` split was considered
  and rejected (cross-instantiation topology comparison is expressible
  as a plain function because keys don't carry `T`).
- **Still open, deliberately**: only the ambiguity constant K's numeric
  value (semantics ratified — sliver band, provisional K = 10, a policy
  dial not a correctness parameter; value pending multi-ε experiments
  during M1+).

### Q2: Tolerance model — **resolved**, folded into D4.

### Q3: Sketch constraint solver — build vs. bind

Ecosystem survey (2026-07) narrowed this considerably:

- **libslvs bindings (`slvs`)**: dead since 2023 and libslvs is GPLv3 like
  SolveSpace proper (the "libslvs is permissive" idea is a myth) — avoid.
- **planegcs**: no Rust bindings exist (the maintained wrapper is
  WASM/TypeScript for npm; the "Rust planegcs" is a common confusion).
- **`ezpz`** (Zoo/KittyCAD, MIT): pure-Rust solver announced May 2026,
  powering Zoo Design Studio's sketch mode; very actively released, fuzzed,
  in production. Pre-1.0 churn and roadmap driven by Zoo's product, but the
  strongest option in the ecosystem.
- **ISOtope** (CADmium, archived, non-OSI license): best free writeup of the
  constraint-as-energy math — reference only.
- **Gap**: no DCM-style graph-decomposition/DOF-diagnosis solver exists in
  Rust at all; everything is iterative/numeric. Over/under-constrained
  diagnosis would be ours to build regardless of solver choice.

Leading answer: adopt **ezpz** at M6, with "roll our own LM solver on
`levenberg-marquardt`/`faer` using ISOtope's math as tutorial" as the
fallback if ezpz's product-driven roadmap diverges from our needs.

### Q4: Units and model scale — **resolved**, folded into D4 (¶4) and D6.

### Q5: Depend on, vendor, or merely study `curvo` for NURBS algorithms

The core invariants (certified residuals, trilean predicates, generic `T`,
no hidden tolerance decisions) live *in the algorithms*, and an algorithm
behind a foreign API can't uphold them — curvo uses its own internal
epsilons and returns bare answers. Default stance: reference + test oracle
(alongside opencascade-rs) from M3. But it's MIT, so vendoring specific
algorithms and adapting them to carry our invariants is on the table;
audit its source properly before M5. Contrast ezpz, which sits *upstream*
of the certified core (its output is just numbers that then pass through
our construction and checks), so arm's-length dependency is principled.

### Q6: Recipe representation — **resolved**, promoted to D8.

### Q7: Determinism policy — **resolved**, promoted to D9.

### Q8: Definitional vs. approximating surfaces

Most surfaces are *definitional* — primitives, extrude/revolve, even lofts
(the produced NURBS *is* the definition; the recipe is provenance). Some
*approximate* an intensional spec they cannot represent exactly. The
canonical case is the **offset**: `S_d(u,v) = S(u,v) + d·n(u,v)` — each
point moved distance d along the unit normal. Analytic surfaces are closed
under offsetting (plane→plane, cylinder r→r±d, sphere, torus, cone —
another D3 payoff), but the offset of a NURBS is *not* a NURBS
(normalizing the normal introduces a square root, breaking rationality),
so the kernel must fit one — an approximating surface with intensional
spec `Offset(S, d)`, a fit, and a certified residual ≤ ε (D4 ¶2), exactly
mirroring fitted intersection curves. Some blends are the same. Needed
before shelling/offset work (M5+), stated now.

### Q9: Project license and name

License **resolved**: dual MIT OR Apache-2.0. Name: still pending —
placeholder workspace acceptable; pre-publish renames are cheap.

### Deferred to their milestones (listed so they don't get lost)

Vertex-geometry taxonomy; orientation/sense conventions (M1 — classic
bug-farm territory, document as conventions once); the validator's
concrete invariant checklist (M1); profile/sketch input format (M2); the
ambiguity constant K's numeric value (M0 experiments; εₐ itself was
eliminated by the D4 ¶1 revision of 2026-07-16 — angular thresholds are
derived per predicate); body-level
serialization beyond the recipe (post-STEP-export).

## Crate landscape (surveyed 2026-07)

Since the kernel itself is greenfield, dependencies are for the *substrate*,
not the modeling core. Candidates, all verified active unless noted:

| Area | Crate | License | Notes |
|---|---|---|---|
| ID arenas | `slotmap` | Zlib | typed keys per entity kind, `SecondaryMap` for attributes — exactly the B-rep store shape |
| Persistent collections | `imbl` (or `rpds` for MIT-only) | MPL-2.0 / MIT | `im` is unmaintained with an open soundness advisory — use the `imbl` fork |
| Interval arithmetic | `inari` | MIT | IEEE 1788, full transcendentals via GMP build dep; dormant but feature-complete against a frozen standard. Probe (issue #4): transcendentals need the `gmp` feature → LGPL-3.0+ transitive deps (`gmp-mpfr-sys`, `rug`), quarantined behind the kernel's `interval` cargo feature; hard AVX+FMA floor on x86-64 (build with x86-64-v3; aarch64 unflagged); planned in-house replacement post-M7 (see Roadmap) |
| Robust predicates | `robust` (georust) | MIT/Apache | Shewchuk adaptive predicates, battle-tested via `geo`/`spade` |
| Dual numbers / forward AD | `num-dual` (dev-only) | MIT/Apache | **Demoted at M0** (PR #10): its transcendentals route through std, not libm, so it cannot satisfy the value-channel bit-identity contract — duals are one in-house generic `Dual<T>` (f64 and Interval from the same code); num-dual serves as a dev-dependency derivative oracle in tests |
| CDT / mesh refinement | `spade` | MIT/Apache | Delaunay + constrained + Ruppert refinement; meshing happens in UV space (our code) |
| 2-D polygon booleans | `i_overlay` | MIT/Apache | robust integer-snapping booleans (now inside georust `geo`); useful for trim-loop ops in UV |
| Display triangulation | `earcut` (georust) | MIT/Apache | cheap ear-clipping for viz only |
| Sketch constraints | `ezpz` (Zoo) | MIT | see Q3 |
| STEP | `truck-stepio`/`ruststep` | Apache | basic geometry round-trips only; full-AP coverage is nobody's solved problem in Rust — evaluate at M4 |

Reference-only (read, don't depend): **truck** (only living Rust B-rep
kernel; active on git but crates.io releases stale; booleans demo-grade),
**curvo** (excellent active pure-Rust NURBS incl. SSI and trimming — study
before M5), **vcad** (new Apache-2.0 half-edge B-rep kernel with
booleans/fillets, too young to depend on but the most interesting recent
effort), **Fornjot** (archived June 2026 — see below), **opencascade-rs**
(the only production-grade-boolean route in Rust today; LGPL + C++ build
tax; useful as a *test oracle* for comparing our boolean results).

## Prior art / references

Local copies live in `references/` (git-ignored). Currently on hand:
`the-nurbs-book.pdf` (full 2nd-edition scan, verified),
**`mantyla-1988-an-introduction-to-solid-modeling-full.pdf`** (the
complete book, 424 pp., supplied by Evan 2026-07-16 and TOC-verified:
ch. 9 Euler operators, ch. 10 half-edge data structure, ch. 11
implementation incl. the low-level Lmev/Lmef/Lkemr set, ch. 12–15
sweeping/geometric algorithms/splitting/booleans — M1 through M3's
primary source; supersedes the old ch. 4–6 partial scan),
`grinspun-schroder-desbrun-GSD06-discrete-differential-geometry.pdf`
(DDG course notes — Evan-suggested during PR #9's subgradient
conversation; the discrete-exactness philosophy is the frame for how
M6's stackup design should treat kinks/subdifferentials), and `hoffmann/` (Hoffmann,
*Geometric and Solid Modeling*, complete: front + chapters 1–7 + bib,
recovered via the Internet Archive — the Purdue page is gone).

- **Mäntylä, *An Introduction to Solid Modeling*** — the Euler-operator
  B-rep reference; the `topo` layer is essentially this book.
- **Hoffmann, *Geometric and Solid Modeling*** (free online, Purdue) —
  intersections, robustness.
- **Piegl & Tiller, *The NURBS Book*** — canonical NURBS algorithms; needed
  by M5 at the latest.
- **Fornjot** and **truck** — the two serious Rust B-rep attempts; study
  the topology/geometry split. Fornjot was archived in June 2026 with a
  shutdown post ("its goals were never reached") after multiple kernel
  rewrites and never achieving robust booleans — required reading as a
  failure postmortem for exactly this project.
- **Open CASCADE** source — a catalog of what every subsystem must do, and
  a cautionary tale on tolerance philosophy.
- **Parasolid XT format spec** (public) — the cleanest picture of a
  production kernel's data model.
- **Shewchuk's robust predicates** papers + CGAL literature — for the
  polyhedral/predicate corners where exactness is achievable.
