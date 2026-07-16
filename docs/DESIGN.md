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

libslvs (SolveSpace) is GPLv3; planegcs (FreeCAD) is LGPL; pure-Rust
options may not exist. Licensing of the eventual project constrains this.
Crate-ecosystem survey in progress.

### Q4: Units and model scale

An absolute tolerance implies a validity range for model size (Parasolid's
"session box" approach). Decide units convention and document the supported
size range, or adopt scale-relative tolerance. Interacts with Q2.

## Prior art / references

- **Mäntylä, *An Introduction to Solid Modeling*** — the Euler-operator
  B-rep reference; the `topo` layer is essentially this book.
- **Hoffmann, *Geometric and Solid Modeling*** (free online, Purdue) —
  intersections, robustness.
- **Piegl & Tiller, *The NURBS Book*** — canonical NURBS algorithms; needed
  by M5 at the latest.
- **Fornjot** and **truck** — the two serious Rust B-rep attempts; study
  the topology/geometry split and Fornjot's kernel-rewrite history
  (instructive failure modes).
- **Open CASCADE** source — a catalog of what every subsystem must do, and
  a cautionary tale on tolerance philosophy.
- **Parasolid XT format spec** (public) — the cleanest picture of a
  production kernel's data model.
- **Shewchuk's robust predicates** papers + CGAL literature — for the
  polyhedral/predicate corners where exactness is achievable.
