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

### D1 (agreed; clarified 2026-07-16; extended at M1 2026-07-16): ID-based arenas, immutable values, manifold-first, Euler operators

- Topology entities (`Solid / Shell / Face / Loop / HalfEdge / Edge /
  Vertex`) live in generational arenas (slotmap-style) and reference
  each other by typed IDs — never `Rc`/pointers. A B-rep is a plain
  value: cheaply cloneable (or structurally shared), serializable,
  diffable, validatable. *(Realized at M1 as Mäntylä's half-edge
  structure in typed arenas: an `Edge` is two antiparallel half-edges,
  the mate computed and never stored; the empty loop — `mvfs`'s state —
  is a typed `LoopBoundary::Empty | Cycle` rather than GWB's nullable
  placeholder half-edge, so every half-edge field is non-optional and
  the sole `Option` in topology is the vertex's emanating half-edge
  (ratified PR #15, implemented PR #16). A face's outer loop is
  excluded from its ring list — `outer ∉ rings`, so rings coincide
  exactly with the Euler–Poincaré r-term (a GWB deviation, ratified in
  PR #16's conversation).)*
- **Manifold solids only** at first. Non-manifold (radial-edge) roughly
  doubles topology complexity; add a non-manifold representation later only
  if sheet/wire bodies demand it.
- Topology is built **exclusively through Euler operators** (Mäntylä's
  `mev`, `mef`, `kemr`, …): a small closed set of primitives that provably
  preserve the Euler–Poincaré invariant. Higher-level operations are
  compositions of validity-preserving pure steps; debug builds check
  invariants after each step (each operator debug-asserts its
  postcondition — a per-call instance of the soundness theorem checked
  against our implementation, never a semantic gate on legitimate
  intermediate states). "Exclusively" is realized, not aspirational:
  the operator set below is the only public construction path — raw
  insertion exists solely as crate-internal test scaffolding.
- **A `Body` is never authoritative** *(clarified 2026-07-16, ratified
  via the M1-PLAN conversation)*: it is the materialized evaluation of
  a construction (an Euler-operator sequence; at M4, a recipe) at some
  scalar `T`, coherent iff bit-identical replay reproduces it (D9);
  lineage-scoped keys and D5 provenance are the derivation's
  fingerprints in the materialization. Mutation exists only as
  evaluator-internal linear working state (`&mut` during an operator
  sequence is exclusive, hence unobservable — linear use of a value);
  a body at rest is a plain value, and modification means deriving a
  successor body by further construction, never editing in place.
  Nothing about a body is true that is not derivable from its
  construction. (For imported bodies — M7 — the authoritative layer is
  the adopted intensional descriptions plus the import record, per D7.)

**Topology conventions (ratified 2026-07-16, PR #16).** One rule, from
which everything else is a corollary — never an independent choice:
walking any loop in `next` order with the face's outward normal toward
the viewer, the face interior lies to the **left** of every half-edge.
Corollaries: outer loops run counterclockwise viewed from outside and
rings run clockwise; an edge's two half-edges are antiparallel
(`end(he) = start(mate(he))`); `Edge::he_plus` defines the edge's
intrinsic direction, with the forward contract that M2's curve
geometry MUST agree — increasing curve parameter runs from
`start(he_plus)` to `end(he_plus)`, pcurves and per-face traversal
senses derived from that, never stored as peers; the vertex-orbit step
`next(mate(he))` visits a vertex's outgoing half-edges **clockwise**
viewed from outside (`mate(prev(he))` is the counterclockwise
inverse). Named transcription hazard: GWB/Mäntylä's diagrams orient
face boundaries clockwise viewed from outside — mirrored relative to
us — so figures, argument orders, and traversal idioms from the book
are never transcribed directly, only rederived from the interior-left
rule and pinned by construction tests. The normative derivations live
in the `crates/topo/src/entity.rs` module docs. (This discharges the
deferred-list item "orientation/sense conventions — document as
conventions once".)

**The operator set (M1; ratified PR #15 and the per-PR sign-offs
#16/#17/#20; kill duals #23).** Ten operators in five make/kill pairs —
`mvfs`/`kvfs`, `mev`/`kev`, `mef`/`kef`, `kemr`/`mekr`,
`kfmrh`/`mfkrh` — plus the `ring_move` reparenting helper, which is
deliberately **not** an Euler operator (`mef` does not reclassify
rings; ring reassignment is a separate non-Euler step, after GWB's
`ringmv`). Addressing is by half-edge key plus per-op **site enums**
whose variants are the degenerate cases (e.g. `MevSite::{Fan, Lone}`) —
the typed-`Empty` consequence: degenerate sites live in the argument
types, not behind null checks. GWB's id-scan layer is dropped
entirely; arena keys are already the stable O(1) handles it existed to
provide. The uniform per-op contract: **atomic** (typed-error
preconditions fully resolve before an infallible mutation phase; a
failed op consumes no key slots), **deterministic minting order**
(documented per op — D9 lineage replay), and a **debug-asserted tier-1
postcondition** (the per-call soundness-theorem instance of the clause
above — never a semantic gate). Association convention, uniform: **the
given/first half-edge's side is the new or affected thing** — `mef`'s
`he1` side becomes the new face's outer loop, `kemr`'s `he1` side
becomes the ring, `kef` kills the given half-edge's face, `kev` the
vertex it points at. Cross-shell `kfmrh` (shell merge rather than
genus) is a typed error until M3's splitting demands it (ratified
PR #15).

**Validity tiers (ratified 2026-07-16 in PR #15's conversation; the
component-aware E–P form found and corrected in M1 PR 4).**

1. **Tier 1 "euler-valid"** — the structural invariant of every
   Euler-reachable state, construction scaffolding included (empty
   loops, struts, self-loop edges, laminae are mandatory
   intermediates); this is what each operator debug-asserts. The
   checklist: referential integrity across all arenas (topology and
   geometry — orphan geometry is an error); half-edge chain
   coherence; mate involution/antiparallelism; vertex anchoring
   (every vertex is referenced by ≥ 1 half-edge XOR is the lone
   vertex of exactly one Empty loop — the restated M0 orphan-vertex
   deferral, discharged); vertex-orbit closure (manifoldness —
   watertightness is structural in the half-edge form); the
   ownership/back-pointer partition (every loop/face/shell owned
   exactly once, spine back-pointers matching); shell-partition/
   edge-adjacency coherence; arity floors;
   bidirectional D5 provenance; and the **component-aware per-shell
   Euler–Poincaré**: per connected component of a shell's incidence
   complex, v − e + f − r = 2(1 − g) with g a non-negative integer,
   summing per shell to 2(c − Σgᵢ) over its c components. The naive
   per-body form is *wrong* for tier-1 bodies — `mfkrh` on a detached
   ring disconnects a shell's surface while a single shell entity
   remains (PR 4 finding).
2. **Tier 2 "closed solid"** (`validate_closed`) — tier 1 plus: no
   empty loops, no valence-1 vertices, and c = 1 per shell. The third
   ban is independent of the first two: a promoted detached cycle ring
   disconnects a shell with neither an empty loop nor a strut present.
   Finished bodies must pass tier 2; tier-1-only states are visible
   solely inside operation *sequences* (a consumer holds scaffolding
   bodies between public calls mid-construction; nothing at rest
   crosses an API boundary without tier 2).
3. **Tier 3 "geometric"** (M2+ — named now, not implemented): D4 ¶2
   residual certification, plus the **material wedge-angle
   predicate** — at every edge the material wedge ∈ (0, 2π), bounded
   away from the ends by the derived threshold θ = ε/r; wedge = π is
   the legal smooth-seam case (ratified in PR #15's conversation).
   Laminae live here, not at tier 2: two faces glued along their whole
   shared boundary is exactly a two-hemisphere ball's incidence
   structure, so a zero-volume lamina is a geometric defect, not a
   topological one. Global self-intersection / minimum clearance stays
   deferred (M3 partial via booleans, M6 interval clearance).

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
  a typed error. *(Honest M1 footnote: operator debug postconditions
  are `debug_assert`s, but they are unreachable by input through the
  public API — raw insertion is crate-internal, and the eleven public
  mutators all preserve tier 1: the ten Euler operators by the
  soundness theorem, and `ring_move` — the one public non-operator
  mutator — by the separating-curve argument documented on the method
  (a ring on a genus-0 component is a Jordan curve, so cross-component
  moves re-partition into legal pieces; non-separating rings force
  g ≥ 1). A firing postcondition is therefore a kernel bug by
  definition. Corrupt in-crate states get typed errors where cheaply
  detectable, or documented garbage-out in release — never a hang;
  every traversal is bounded.)*
- Essentially no unsafe Rust outside vetted dependencies.

**Replay with kills (M1, pinned in PRs #20/#23):** the determinism
contract holds with destructive operators in the history. Identical
histories replay bit- and key-identically, kills included; a failed
operator consumes no key slots (the lineage contract's error half —
tested by interleaving failing calls into builds and deep-comparing
snapshots). Convergence with a kill-free history is **per-arena**: a
balanced kill/make pair (kemr∘mekr) re-converges the half-edge, edge,
and curve arenas immediately and the loop arena one loop-mint later
(recycled slot, bumped generation); an unbalanced kill history offsets
the killed arenas' allocation cursors permanently — arenas the kill
never touched stay aligned forever, killed arenas never re-align.

### D5 (agreed): Persistent topological identity from birth

Every topological entity carries a provenance record from the moment it is
created: which operation created it, from which inputs ("side face swept
from sketch edge #3"). This does not solve the topological naming problem —
the most user-visible unsolved problem in parametric CAD — but recording
identity at birth is cheap, and retrofitting it onto anonymous entities is
nearly impossible. The parametric layer (M4) builds its stable references
on top of this record.

Realized at M1 (PRs #17/#20/#23): provenance is a typed per-operator
**birth record** — the operator plus its argument keys — carried by
every entity of all seven topology arenas. Kills remove the record
together with the entity; survivors keep theirs; reparenting or
demotion (`ring_move`, `kfmrh`'s loop demotion) is not a re-birth. The
validator enforces the record bidirectionally: every live entity has
one, and no record outlives its entity.

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
| `editor-core` | *(added 2026-07-19)* Headless document/editor layer: document-as-value (recipe + metadata), typed edit vocabulary (`DocEdit` + pure `apply`), stable-reference/selection model, incremental evaluation service (preview/commit, epochs, cancelation). No rendering dependency — most of "the GUI project" is library work that ships and tests before a pixel exists. See `docs/GUI-DESIGN.md` |
| `viewer` | Deferred (GUI last; sequenced after usable-as-library). Architecture: `docs/GUI-DESIGN.md` (G1 three-layer split). Until then: `rerun` for zero-effort demos |

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

- **M0** — `geom-core`: scalar trait + intervals; arenas; validation
  harness. *(Complete 2026-07-16.)*
- **M1** — Topology + Euler operators; build a cube by hand; watertightness
  and Euler checks pass. *(Complete 2026-07-16.)*
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
- **Post-M7** — the usability program: see
  [Beyond the kernel](#beyond-the-kernel-the-usability-gap) below.
  Licensing-hygiene work with no usability payoff is deliberately
  *not* sequenced here — it lives in [Tabled](#tabled-far-future).

## Beyond the kernel: the usability gap

*(Added 2026-07-19, from the usability-scoping conversation with Evan.
This is a **scoping section, not a milestone plan** — it names the
work between "the M0–M7 kernel exists" and "a person can actually use
this," so that none of it gets invented ad hoc or discovered late.
Items marked **(design-now)** are cheap at design time and expensive
to retrofit; each gets folded into the existing plans rather than
waiting for a usability milestone. Several items below need their own
design documents with D1–D9 rigor before they are plannable —
flagged individually.)*

**Sequencing stance (agreed 2026-07-19): "usable as a library" ships
before any GUI work begins.** After M4 the kernel has parametric
models, mass properties, and STEP export; adding language bindings
(Python — the CadQuery/build123d audience), documentation, and
feature breadth yields a genuinely usable code-first tool years
before an interactive application could exist. The GUI is a separate
layer and effectively a second project of comparable size to the
kernel (Fornjot's postmortem and Zoo's app-team scale are the
evidence); its architecture lives in **`docs/GUI-DESIGN.md`** (G1
three-layer split ratified 2026-07-19: kernel / headless
`editor-core` / interaction; **ratified**: GQ1 solver-replay
boundary (witness = authoritative branch selection), GQ2
per-node-result-DAG, GQ3 persist-all-edits, GQ4 document scope
(local refs + assembly-era wrapper; **assemblies are recipes of the
same formalism** — the document boundary is a namespace/versioning
seam, so GQ1–GQ3, naming, and undo apply to assemblies unchanged;
binding semantics: Cargo.lock-style pinned-with-explicit-update,
ratified in direction), GQ5 typed
quantities in the expression sublanguage (dimension-algebra extent
banked for M4); GQ6/GQ7 deliberately deferred to GUI time.
Remaining pre-M4 design work: GQ1 mechanism details and the
selection-stability/naming design doc).

### Band 1 — kernel-side services an interactive client requires

The "any GUI is a thin client" claim (Vision) is true only if the
kernel exports these. None are research; all are load-bearing:

- **Incremental recompute.** "Caching is free — models are values"
  is true semantically; interactive editing needs it *engineered*:
  memoized feature-DAG evaluation keyed on input slices, invalidation
  of only downstream features, partial re-tessellation. Target shape:
  edit one parameter mid-DAG → new solid at interactive latency. D9
  determinism is what makes the memo keys well-defined.
- **Picking back-references (design-now — ratified into M2 PR 6).**
  Tessellation output carries per-triangle source-`Face` keys and
  per-boundary-polyline source-`Edge` keys, so a viewport ray hit
  resolves to a topology entity. Cheap at tessellator-design time,
  painful retrofit. Spatial indexing (BVH) for hit-testing sits on
  top, client-side or in `mesh`.
- **Cancelation and progress.** Long operations (booleans, fillets)
  need cooperative yield points and progress reporting; pure-value
  semantics makes abandonment safe, but the yield points must be
  designed in, not bolted on.
- **Selection stability across edits** — the user face of D5/M4's
  persistent naming, and the single most usability-determining piece
  of parametric CAD: the user fillets edge E, changes a parameter,
  topology shifts, and E must re-resolve or fail with an actionable
  typed error. M4's "builds stable references on top of the birth
  record" sentence is months of work. **Needs its own design doc
  before M4 planning**, with the explicit goal that our architecture
  (D5 birth provenance + D8 recipe node IDs + D9 replay) makes
  correct resolution *structurally* easy — as much "automatic" as the
  design can extract. Ratified 2026-07-19 (GUI-DESIGN.md G1): the
  GUI's selection type and the recipe's entity references are **the
  same type** (a stable name), so the naming problem is solved once,
  not twice. Founding pillar ratified 2026-07-19: naming is
  localized to reified predicate flips (see Banked principles
  below).
- **Appearance attributes (design-now, as an empty container).**
  Per-face/body display attributes (color, name, visibility) must
  live somewhere that survives recompute — which means they attach
  via the same stable-naming machinery, not arena keys. An empty
  typed container lands early (M2) so a home exists; durable
  attachment semantics arrive with M4 naming.

### Band 2 — the interactive application (a second, kernel-sized project)

Named here so its cost is never underestimated; sequenced after
usable-as-library; architecture to be ratified separately.

- **Viewport**: real-time tessellation with LOD, edge/silhouette
  rendering, section views, snapping, navigation. A demo viewer is
  ~10% of this.
- **The interactive sketcher** — the largest single item: dragging,
  dimension placement, constraint inference, and visual over-/under-
  constraint feedback. Q3's ecosystem gap (no DOF-diagnosis /
  graph-decomposition solver in Rust) becomes **user-facing** here
  ("why is my sketch red?"), converting that solver from optional to
  mandatory for the GUI milestone. Sketch-on-face and projecting
  model edges into sketches are further consumers of M4 naming.
- **Feature tree UI**: rollback, reorder, suppress, edit-in-place —
  D8's recipe-as-DAG is exactly the right substrate.
- **Error UX.** D4's fail-loud typed errors are correct for a kernel
  and brutal in a GUI if presented raw; `ToleranceExceeded { entity,
  … }` must become "this fillet fails *here*" with the entity
  highlighted. The typed-error discipline is what makes this
  *possible*; the presentation layer is real work.
- **Direct manipulation** (drag a face → parameter change) is an
  inverse problem on top of everything above; optional for v1 except
  dragged sketch dimensions, which users assume.

### Band 3 — missing subsystems (in no current milestone)

- **Assemblies.** Multi-part documents, mates (a rigid-body-DOF
  constraint problem, distinct from the 2-D sketch solver),
  cross-document references, interference checks (the latter falls
  out of M3 booleans / M6 clearance). Even hobbyist use wants this.
  *Reference architecture ratified 2026-07-19 (GUI-DESIGN.md GQ4):
  an assembly document is a recipe DAG of the same formalism —
  instantiate-part (via the doc-identity × local-ref wrapper),
  mates, and patterns are ordinary feature nodes, so the editor and
  solver machinery (incl. mate witnesses per GQ1) transfers
  unchanged; binding semantics ratified in direction —
  pinned-with-explicit-update, the Cargo.lock model (details at
  assembly design).*
- **Engineering drawings.** Dimensioned 2-D drawings require
  projection plus **hidden-line removal**; HLR on curved B-reps is
  SSI-grade (silhouette curves) and belongs on the difficulty
  ranking near fillets. Explicit near-term dodge: export STEP, make
  drawings elsewhere.
- **Feature breadth.** Post-M7 the kernel has extrude/revolve/sweep/
  loft, booleans, shell, constant-radius fillets. Daily use assumes:
  chamfers, variable-radius fillets, draft, hole features
  (counterbore/countersink/tapped), linear/circular patterns and
  mirror (D8's structural parameters are the substrate), datum
  planes/axes, helixes, rib/text features. Individually small; the
  long tail dominates "why can't I model my part."
- **Interchange breadth**: 3MF (supersedes STL for printing), DXF
  in/out (profiles, drawings), OBJ. Each small; STEP remains the
  only hard one.

### Banked principles (ratified 2026-07-19, rounds 6–7 of the usability conversation)

Cross-milestone commitments extracted from the "where do we get more
for free / where is the danger" review; each lands at the milestone
named.

- **Naming is localized to reified predicate flips** *(pillar of the
  pre-M4 naming doc)*. Topology is a function of the recipe and can
  change only where a structural parameter (D8) changed or a trilean
  predicate (Q1) flipped. Within a flip-free parameter region,
  replay is history-identical and M0's lineage-scoped key identity
  makes name resolution *provably* trivial; at a flip, the flipping
  predicate itself names what changed and why. Resolution policy:
  trivial where provable, loud typed failure carrying the flip's
  diagnosis where not; re-binding cleverness only as ratified opt-in
  policies. Margin-based pre-flip *warnings* ("this reference is
  within K·ε of vanishing") are noted as a natural extension —
  deliberately far-future.
- **Content-keyed cache transfer** *(key shape lands with M2 PR 6;
  service at editor-core)*. D9 bit-determinism makes any derived
  artifact (certified residual, tessellation patch, BVH node) keyed
  by the bit-content of its geometric inputs transferable across
  rebuilds by equality check — the key *is* the correctness proof;
  no dirty-flag invalidation logic. Finer-grained than (and
  complementary to) feature-DAG memoization.
- **Coincidence is structural or declared, never inferred from
  values** *(pre-M3; ratified round 8 — Evan's
  explicit-intent revision of the round-6 proposal, which had a
  latent defect: treating bit-equal descriptions as semantic
  coincidence makes topology hinge on an UNMARGINED predicate — a
  razor-thin equal-vs-one-ulp cliff with no escalation band,
  exactly what Q1 forbids — and value equality is not evidence of
  intent anyway)*. The ratified ladder: (a) **shared surface key** —
  coincidence explicit by construction; (b) equal-but-independent
  descriptions do **not** glue — if the user means flush, the recipe
  must say so (share the surface, or an explicit recipe-level
  relation declaration that makes it structural); description-
  equality *detection* is a diagnostic/affordance only ("these
  faces coincide exactly — declare the relation?"), never
  semantics; (c) near-coincidence between unrelated definitions is
  a typed sliver error whose resolution is an **explicit
  repair/adoption operation** — D7's machinery applied natively:
  reconcile geometry to make the coincidence definitional, moving
  it by a reported amount, like import healing. Consequences:
  undeclared-but-touching booleans fail loudly with a one-step
  resolution instead of working by luck, and the naming pillar
  stays airtight — topology depends only on recipe structure and
  margined predicate verdicts, so predicate flips remain the *only*
  topology-change sites.
- **The editor-core evaluation service is generic over `Real`** from
  day one — M6's error-propagation UI rides the same memoization /
  cancelation / per-node-result machinery as f64 rebuilds; no
  parallel path, no retrofit.
- **ε and persistence** *(rules for the first persisted document)*:
  a document records the ε it was authored under; the application
  pins the run's ε to the document's; an assembly whose referenced
  documents disagree on ε is a typed error (D4's per-model-ε
  rejection, enforced at the seam). **Changing ε is a recorded
  `SetTolerance` document edit** (Evan's addition): apply = replay
  at the new ε and structurally diff — D9 key identity makes "did
  topology change" a free comparison, and the delta is reported as
  exactly which predicates changed verdict (escalations included);
  any change is a typed error requiring explicit user resolution.
  Same diff machinery as the naming pillar — ε changes and
  parameter changes are both "same recipe, different evaluation
  context."
- **Flags banked for later milestones**: mate solving at assemblies
  needs witnesses/interval contraction on SE(3), not ℝⁿ — budget
  for it, don't assume the sketch machinery drops in; recipe-level
  provenance must carry **pattern indices** explicitly so references
  into indexed families never degrade to positional guessing (naming
  doc requirement); the Band 4 model corpus comes online **at M4**,
  not with the GUI — rebuild latency is an architectural property
  and must be measured while the architecture is still cheap to
  change.

### Band 4 — product-grade infrastructure

- **Recipe schema versioning/migration from the first persisted
  file** (D8 is the save format), autosave/crash recovery, and
  embedded derived caches so opening a model isn't a full rebuild.
- **Performance at scale**: hundreds of features / thousands of
  faces; the parallel-evaluation story under D9's fixed reduction
  shapes deserves early thought.
- **A real-model corpus** as the usability regression suite: "these
  N parts rebuild in < T seconds with identical topology" — the
  usability analog of the mass-property suite.
- **Docs and onboarding** for the API-as-product: tutorials,
  examples, and Python bindings (see the sequencing stance above).

## Tabled (far future)

Deliberately unsequenced — kept off the roadmap so it never reads as
preceding the usability program above.

- **In-house rigorous interval transcendentals** *(moved from the
  roadmap's post-M7 note, 2026-07-19)*: replace `inari`'s gmp/MPFR-
  backed transcendentals with proven per-function error pads over
  `libm` plus monotonicity/extremum handling, so interval builds can
  drop the LGPL-3.0+ transitive dependencies. Until then the
  `interval` cargo feature quarantines the copyleft obligation to
  interval-enabled builds only (issue #4). Licensing hygiene, not
  usability — do not schedule ahead of anything users can feel.

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
  during M2+ — M1's topology is scalar-free and consulted no predicate,
  so it generated no evidence; M2's geometric predicates are the first
  data source).

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

Vertex-geometry taxonomy (M3, when intersections exist); profile/sketch
input format (M2); the ambiguity constant K's numeric value (M2+ —
topology is scalar-free and consults no predicate, so M1 generated no
new evidence; εₐ itself was eliminated by the D4 ¶1 revision of
2026-07-16 — angular thresholds are derived per predicate); body-level
serialization beyond the recipe (post-STEP-export). *(Discharged at
M1: orientation/sense conventions and the validator's concrete
invariant checklist — both ratified into D1.)*

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
M6's stackup design should treat kinks/subdifferentials),
`vida-martin-varady-1994-survey-of-blending-methods-parametric-surfaces.pdf`
(Computer-Aided Design 26(5) — the canonical blending survey, supplied
by Evan 2026-07-16; primary source for M5's fillet scope-boxing:
terminology/classification of blends, rolling-ball and trimline
methods, the open problems that motivated D2's `TangencyLocus`
treatment), and `hoffmann/` (Hoffmann,
*Geometric and Solid Modeling*, complete: front + chapters 1–7 + bib,
recovered via the Internet Archive — the Purdue page is gone).

- **Mäntylä, *An Introduction to Solid Modeling*** — the Euler-operator
  B-rep reference; the `topo` layer is essentially this book. One
  erratum on record: our reading notes carry a dated erratum for
  Program 11.6 — `lmev`'s printed `addhe` order (PLUS-half first)
  breaks both `he1 == he2` cases; MINUS-first is coherent — found by
  hand-trace during M1 PR 2 and verified against the scan.
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
