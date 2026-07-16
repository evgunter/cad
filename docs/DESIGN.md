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

### D2 (agreed): Topology and geometry separated; pcurves are primary

Topology and geometry live in separate arenas: faces reference surfaces,
edges reference curves, vertices reference points.

**What a pcurve is.** A surface is a map `S(u,v) → ℝ³`. A face is a region
of that surface's 2-D parameter plane, and each of its boundary edges is
therefore also a curve `P(t) → (u,v)` in that plane — the *pcurve*
("parameter-space curve"). An edge shared by two faces classically carries
*three* representations: a 3-D curve `C(t)` plus one pcurve per adjacent
face, with the consistency requirement `Sᵢ(Pᵢ(t)) ≈ C(t)`. Pcurves are not
optional — point-in-face trimming tests, tessellation, and intersection
marching all happen in (u,v) space. The redundancy among the three
representations is a classic bug farm in every kernel.

**Our rule:** one representation per edge is *authoritative* and the others
are derived, carrying a certified residual bound (see D4). Concretely: one
adjacent face's pcurve is designated primary; the 3-D curve is the
composition `S∘P` (cached, possibly refit); the other face's pcurve is
derived by projection. Which side is primary is recorded explicitly. Note
this still leaves genuinely redundant *data* in caches — the invariant is
that there is never a question of which representation wins.

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

### D4 (agreed in principle): Single strict tolerance; operations fail loudly

No per-entity tolerances that grow as operations get sloppy (the Open
CASCADE model, where errors snowball silently). One kernel tolerance;
derived geometry must meet it or the operation returns a typed error with
diagnostics. "Define what something is" applied to error handling.

Exact semantics (what carries a residual, what the error looks like, how
tolerance relates to model scale/units) are under discussion — see
[Open questions → Tolerance model](#q2-tolerance-model).

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

## Open questions

### Q1: Scalar genericity

How far does the generic scalar `T` reach, what is the trait, and how do
algorithms that *branch* on geometric predicates behave under interval
types (which lack a total order)? Current proposal under discussion:
two-tier design — evaluation code fully generic; topology-determining
decisions run concrete at `f64` but record *witnesses* (predicate + signed
margin) that can be re-checked under intervals/duals for error propagation.

### Q2: Tolerance model

Precise semantics of "geometry fails to meet tolerance": which derived
artifacts carry certified residual bounds, what the typed error contains,
absolute vs. scale-relative tolerance, units policy. Under discussion.

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

### Q4: Units and model scale

An absolute tolerance implies a validity range for model size (Parasolid's
"session box" approach). Decide units convention and document the supported
size range, or adopt scale-relative tolerance. Interacts with Q2.

## Crate landscape (surveyed 2026-07)

Since the kernel itself is greenfield, dependencies are for the *substrate*,
not the modeling core. Candidates, all verified active unless noted:

| Area | Crate | License | Notes |
|---|---|---|---|
| ID arenas | `slotmap` | Zlib | typed keys per entity kind, `SecondaryMap` for attributes — exactly the B-rep store shape |
| Persistent collections | `imbl` (or `rpds` for MIT-only) | MPL-2.0 / MIT | `im` is unmaintained with an open soundness advisory — use the `imbl` fork |
| Interval arithmetic | `inari` | MIT | IEEE 1788, full transcendentals via GMP build dep; dormant but feature-complete against a frozen standard |
| Robust predicates | `robust` (georust) | MIT/Apache | Shewchuk adaptive predicates, battle-tested via `geo`/`spade` |
| Dual numbers / forward AD | `num-dual` | MIT/Apache | generic `DualNum<F>`, arbitrary nesting, simba `RealField`. **Dual-over-interval does not exist off the shelf** — we write a `DualNum` newtype over `inari::Interval` in-house (comparison semantics for zero-straddling intervals is a design decision — consistent with Q1's two-tier view) |
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
