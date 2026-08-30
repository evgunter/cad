# CAD Kernel — Design Document

**Status: v0.** Living document. Decisions marked *agreed* are settled unless
new evidence overturns them; items in [Open questions](#open-questions) are
under active discussion and get promoted here once ratified.

## Companion documents

Ratified design lives in this document AND in per-topic companions;
a reader entering here should know all of them exist.

| Document | Status | Scope |
|---|---|---|
| `docs/CURVED-DESIGN.md` | RATIFIED (#85) | Curved-geometry program: C1–C12 (locus ladder, certificates, SSI, pcurves, dispatch, fillets, NURBS scope) |
| `docs/NAMING-DESIGN.md` | RATIFIED (#74) | Persistent naming N1–N7 (derivation-path names, split/merge policy, name table) |
| `docs/SOLVER-DESIGN.md` | RATIFIED (#79) | GQ1 witness mechanism W1–W9 (solved assignments, certification, `WitnessBifurcation`) |
| `docs/ERROR-DESIGN.md` | RATIFIED (#110) | Error-propagation program E1–E11 (duals, stackups, subdivision driver, trichotomy); RUNNING as the M10 program (`docs/M10-PLAN.md` / `docs/M10-LOG.md`) |
| `docs/DUAL-DESIGN.md` | RATIFIED (#1146) | The Dual contract DL1–DL6 (M10-D): a Dual is tangent transport and never certifies (D1's hedge closed); ContentBits feeds both channels; certified gates absent at Dual by scalar policy; Enclosure gated; the delegation rule; poison-vs-widen in certified lanes |
| `docs/PROFILE-LIFT-DESIGN.md` | RATIFIED with a recorded hedge (#1151) | The profile-parameter lift PP1–PP6 (M10-P): guided replay — structure f64-once as the witness, geometry at the lane scalar with every consumed decision re-verified at `T`; canonicalization and naming pinned; the f64 build path bit-identical |
| `docs/CONTACT-DESIGN.md` | RATIFIED (#178) | Contact census & declared contact C1–C8 (closes CURVED OQ5); the C7 join lane shipped at M9 |
| `docs/PATHS-DESIGN.md` | RATIFIED (#124) | PartialPath authoring algebra (S5); implemented at LIB U2 |
| `docs/PROFILES-V2-DESIGN.md` | RATIFIED (#242) | Profiles-as-programs V1–V8: the stored profile-program, Expr-bearing steps, the replay driver; implemented at the LIB SWITCH units |
| `docs/SELECT-DESIGN.md` | RATIFIED (#286) | Geometric selectors, the detect/declare protocol, and the GQ7 re-homing |
| `docs/GUI-DESIGN.md` | RATIFIED (G1–G5) | GUI/editor architecture: three-layer split, document-as-value, edit vocabulary; the v1 GUI program is CLOSED — plan `docs/GUI-PLAN.md`, exit walk `docs/GUI-EXIT-WALK.md` (ratified #1121, 2026-08-28); GUI-5 and GUI-6 banked post-v1 |
| `docs/ASSEMBLY-DESIGN.md` | RATIFIED (#333) | Band 3 assemblies A1–A13 + AQ1–AQ8: scope ladder, assembly-evaluates-to-a-body, mates-as-declarations, pins/split-inline, validity, mirror, relative freedom, product roots, the constructive-solve boundary; implementation ladder R0–R4, CLOSED at v1 scope through R1–R2 (`docs/ASM-EXIT-WALK.md`) |
| `docs/LIBRARY-DESIGN.md` | RATIFIED (#229) | Usable-as-a-library program L1–L8: façade, Python bindings via the document layer, v2-fronted PATHS, authoring-ergonomics unit ladder; the program is OPEN and resting — dispatchable column at the `docs/LIB-LOG.md` tail |
| `docs/DISCIPLINES-DESIGN.md` | WIP — provisionally accepted (2026-08-25) | Disciplines/checks registry DS1–DS9: the identification criterion, the severity invariant, the four grades, the recording dial, out-of-tree checks; two residents SHIPPED (`editor_core::checks`: connectedness, separation) |
| `docs/PCURVE-UNIFY-DESIGN.md` | RATIFIED (#514) | Pcurve unification (#427): `EdgeGeometry`'s conventional variants collapse to ONE (surface, `Pcurve`) form, the exact variants kept as certification lanes; `MappedCurve` demotes to an authority record behind a transience fence. Executed by the PCURVE program (`docs/PCURVE-PLAN.md` / `docs/PCURVE-LOG.md`) |
| `docs/CENSUS-REST-CLOSURE-DESIGN.md` | RATIFIED (#965) | At-rest census structural identity (#943 + #591 Door-2): the world-space Door 2 for declared planar pairs with its C3/C4 revision; cross-instance curved declared `Rest` as named residue |
| `docs/RECIPE-DOORS-DESIGN.md` | RATIFIED (2026-08-29, in-chat: D2–D5; D1 reclassified as orchestrator sequencing) | Recipe doors for the shipped surgery verbs — chamfer, tube, shell. D2/D3: `Node::Chamfer` is `Node::Fillet`'s twin and reuses the fillet ROLE vocabulary (the minting node is the discrimination); the emitter pays #708's tie-deferral debt at the same time. D4: `Node::Tube` is ONE node kind carrying `wall: Option<Expr>`. D5: shell WAITS on a kernel `ShellNaming` birth record. D2/D3 implemented at LIB-G16 (schema v16, #918) |
| `docs/GROUP-BOOLEAN-DESIGN.md` | RATIFIED (#496, option A′) | Group boolean in the recipe layer (D2 + F4): `PlacedUnion` — "a Pattern that fuses", one prototype, one body out, `Instance{i}` naming unchanged; implemented by LIB (#571, schema v12) |
| `docs/OFFSET-DESIGN.md` | RATIFIED (#907) | Offset & shell O1–O6: analytic offsets minted by struct-update, the approximating-surface lift, the offset certificate and its two meters, what shell IS. DESIGN.md Q8 is its ratified seed; implemented across VERBS Wave 3 (OFF-A…OFF-D + the teapot) |
| `docs/MIRROR-DESIGN.md` | RATIFIED (#909) | Patterns & mirror P1–P6: the chart-handedness convention (u ↦ −u), mirror's own door beside rigid transform, and the boundary of ASSEMBLY-DESIGN A6's equivariance audit (VERBS) |
| `docs/DRAFT-DESIGN.md` | RATIFIED (#908) | Draft, the molding taper, DR1–DR6: plane walls only at v1, a certified re-geom pass, the pull-direction selector as a SELECT-DESIGN amendment, survivor naming; NOT YET IMPLEMENTED (VERBS) |
| `docs/ARMS3-DESIGN.md` | RATIFIED (#992) | ARMS-3, A3-1…A3-3: the general sphere×sphere fillet arm, the valence-4 "corner" that is not a corner, and what a run-out at a seam vertex IS; implemented at #1028, the recourse's missing door is #1022 (VERBS) |
| `docs/ENCLOSING-TANGENCY-DESIGN.md` | RATIFIED (#1210) | The enclosing (ρ < 0) fillet tangency: the class is permanently unreachable — no door emits it — and a radius demanding it refuses typed (closes #827); implemented at S-BLEND's BLEND-7 |
| `docs/KERNEL-VERBS.md` | Reference register | The modeling verbs the kernel does not yet have, each with its prerequisites; worked in dependency order by the VERBS program (`docs/VERBS-PLAN.md` / `docs/VERBS-LOG.md`). The register never schedules |
| `docs/K-REPORT.md` | Reference | K-constant evidence record (#89 CLOSED, K = 10 permanent) + milestone addenda |
| `docs/PERF-PLAN.md` | Merged-and-advisory (D9 addendum) | Performance plan and Q-P answers |
| `docs/CURVO-AUDIT.md` | Reference | curvo vendor audit behind Q5's resolution |
| `docs/LONGTERM-IDEAS.md` | Parked, non-binding | Idea bank with a graduation rule |
| `docs/MODEL-AB-LOG.md` | Experiment log | Model A/B protocol + running data; process data, not design |
| `docs/NAME-CANDIDATES.md` | Reference | Q9 project-name candidates and registry availability (re-sweep before ratifying) |
| `docs/predicate-dimension-audit.md` | LIVE working audit | Dimensional-analysis sweep of predicate comparands against D4's ε semantics; its own *Findings (dispositions)* section is the live list of what is open, and a restatement here would drift out of step with it |

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

> **A model is a pure, replayable function from a parameter vector and a
> tolerance to a solid.** `fn build(params: &Params, tol: Tol) ->
> Result<Solid, ModelError>` — deterministic, no hidden state. The B-rep is
> a derived value, never a mutated-in-place object.

Determinism is over the pair: the same parameters at the same ε give the
same solid. ε is one value per run, committed once and never changed after
— see D4 ¶1.

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

**Face orientation sense** *(ratified 2026-08-02, M5 S10; the closure of
PR 9c deviation 3)*. A face carries an explicit orientation bit,
`Face::sense: bool`: **`true` iff the face's material side agrees with
its surface's chart normal**, so the face's outward normal at a point is
`sense_sign · n(u, v)` with `sense_sign = ±1`. Before this the outward
normal simply *was* the chart normal, and that identity was not merely
convention but a **representation gap**: `revert` — which must produce
the body bounding the complementary volume — had nothing to write on a
curved face. The analytic chart normals do not admit a reversal by
reparameterization: the cylinder's, cone's and torus's are *odd* in the
radius (outward for either sign, so neither an axis flip nor a negative
radius moves them), and the sphere's is *even* in the radius, hence
outward exactly under the ratified `radius > 0` convention (a
negative-radius sphere is a de facto reversed sphere and is **rejected**
as a representation — it breaks that convention and inverts every
consumer that meters a sphere residual by `2r`). The three alternatives
— a `Surface::Reversed` wrapper, NURBS conversion on revert, and
negative-radius spheres — were costed and rejected; the bit on the face
is the ratified fix. Consequences, all normative:
- The interior-left rule above is stated against the face's **outward
  normal**, which is now the sense-signed chart normal. Nothing about
  the rule changes; its input does.
- Orientation reversal is **exact structure**, never a numeric decide:
  reverting a curved body flips `sense` rather than perturbing geometry,
  so `revert` stays a bitwise involution and a `revert ∘ revert` round
  trip is bit-identical at every scalar backend. *Shipped as of M5 S12*,
  which wired the writer: `revert` flips the bit on every face carried
  by a non-plane surface and keeps M3's stored-normal negation for
  `Plane`-carried faces. The two encodings are **exclusive by surface
  kind** — a plane can represent its own reversal exactly, the analytic
  and NURBS charts cannot — so every face's outward normal is negated
  exactly once, and the planar arm stays bit-for-bit what M3 pinned. The
  split is a **conservatism choice, not a forced one** — either encoding
  is correct, and moving M3's planar pins is its own design conversation
  (M5 S12 review).
  `RevertError::UnsupportedSurface` is retired; the reversal itself has
  no per-class residue left.
- A face **fragment** inherits its parent's `sense` (M5 S12). `mef` and
  `mfkrh` still mint `true` when the caller hands them a new or foreign
  surface — the material side is not op-level knowledge there — but when
  the new face lands on the OLD FACE'S surface key it is a region of
  that same face, so it takes that face's bit. Key equality, never a
  numeric compare. Without this, a boolean split of a reversed wall
  silently resets the bit on the pieces.
- What curved `revert` does **not** by itself unblock is a curved
  boolean's *seam*. Subtract and intersect are open on the classes whose
  germ pairs have a join lane (plane × cylinder, plane × sphere) and refuse
  typed, per class, on the ones that do not — never wholesale, and never
  by silently falling through to a containment verdict a curved boundary
  can defeat.
- Every "which way is out" consumer (tier gates, mass-properties flux,
  boolean sector classification and point-in-solid, splitting
  classification, tessellation winding, export winding) reads the signed
  normal, or documents in place why it is sense-invariant.
- The bit is exactly STEP's `advanced_face.same_sense`
  (ISO 10303-42 `face_surface`), so the exporter consumes it rather than
  deriving it.
- Persistence is unaffected: bodies are not serialized — they re-derive
  from recipes (D9) — so there is no schema change.

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
vertex it points at. Cross-shell `kfmrh` is the shell-fusion form
(same solid), with `mfkrh` its inverse motion; `ring_move` reparents
only within one shell (`EulerOpError::CrossShell`).

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
3. **Tier 3 "geometric"** (implemented across M2 PRs 3–7): D4 ¶2
   residual certification, plus the **material wedge-angle
   predicate** — at every edge the material wedge ∈ (0, 2π), bounded
   away from the ends by the derived threshold θ = ε/r; wedge = π is
   the legal smooth-seam case (ratified in PR #15's conversation);
   and the ends carry a **declared second-order arm** (ratified with
   Evan 2026-08-23, closing #131): wedge = 0 (a cusp — two kissing
   cylinders with one side cut away) and wedge = 2π (a knife slit,
   the cusp's `revert` image — revert is an involution, so the two
   are legal together or not at all) are legal iff the tangency is
   **declared** (the C7 `Tangent` contact vocabulary — never
   inferred from values, per the coincidence ladder) and
   **jet-determinate**: quadratic transverse separation with κ_rel
   bounded away from zero — `TangentIntersection`'s own margin, so
   the declaration verifies against the same second-order schedule
   and the cusp edge's honest description IS `TangentIntersection`.
   In-band κ_rel (osculation) escalates; an undeclared cusp refuses.
   The arm admits no laminae — conformal contact over a patch fails
   the curve-locus condition, so zero-volume bodies stay geometric
   defects and the PR #15 rationale for the bound is untouched. A
   doubled cusp (two material wedges on one tangent line — the
   kissing union, a slit interior to material) is not one 4-face
   edge but F2's coincident-distinct-edges class, each edge
   classifying separately under this rule. Consumers with no
   wedge-0/2π answer (fillet, offset, mesh sizing, sector
   classification, …) refuse typed at the consumer. Implementation
   is banked at #941; the deferred material-side check adopts this
   verdict table when built.
   M2 classifies the tangent-plane wedge; the 0-vs-2π lamina side
   distinction needs pcurves (M3+). Also at tier 3 (M2 additions):
   **prefer-intrinsic enforcement** (definitely-transverse edges must
   carry `Intersection` — see D2) and the **positive-volume
   orientation invariant** (exact-B-rep signed volume
   definitely-negative ⇒ invalid; margin V/A_total — a length, the
   mean boundary displacement of the volume defect; zero and
   escalated exempt: it is an orientation probe, not a thinness
   gate, and ε-tightening never flips valid→invalid).
   Laminae live here, not at tier 2: two faces glued along their whole
   shared boundary is exactly a two-hemisphere ball's incidence
   structure, so a zero-volume lamina is a geometric defect, not a
   topological one. Global self-intersection / minimum clearance stays
   deferred (M3 partial via booleans — tier 3′ below discharges the
   coincidence census, exactly on the planar inventory and by named
   class elsewhere; M10 interval clearance).
4. **Tier 3′ "pseudomanifold" (`validate_pseudomanifold`; ratified at
   the M3 exit sweep per M3-PLAN F1/F2, resolved with Evan #42;
   implemented M3 PR 6a, #75)** — the honest at-rest tier for boolean
   results that *touch*: contacts limited to
   entirely-coincident-but-distinct edges, edge-on-face,
   vertex-on-face, vertex-on-edge/vertex — touching allowed, proper
   self-intersection not. Composition: tier 3's **local battery
   verbatim** (shared extraction — 3′ and tier-3 bodies run identical
   local checks) plus a **global coincidence census** plus
   **two-directional declared-contact certification**:
   - The census is exact on the planar inventory (`Line` carriers /
     `Plane` faces) and, since M9-2, ADMITS every carrier kind:
     same-key opposed-sense curved pairs certify through the conformal
     arm, declared curve/patch records through the jet schedule and the
     patch certifier, cross-solid pairs with a curved side in reach
     refuse as undecidable, and same-solid distinct-key curved pairs
     stay undetected until C9/C6 (`topo::census` module docs). A record
     or candidate outside a certifier's lane refuses typed
     `CensusUnsupported`, never samples. Five quadratic sweeps (vertex–vertex, vertex-on-edge,
     vertex-on-face, edge×face, edge×edge), every comparison a named
     Q1 trilean; indeterminates surface as typed `CensusEscalated`,
     never a silent skip.
   - Certification runs **both directions and never scans-to-bless in
     either**: a census finding with no backing declaration is
     `UndeclaredContact` (discovery is never declaration); a
     declaration with no geometric witness is
     `StaleContactDeclaration` (dead keys, self-pairs,
     coincidence-free records). Structural sharing (same key) is the
     coincidence ladder's first rung and needs no record.
   - Contact records carry two granularities: vertex (`VvContact`,
     `VfContact`) — with edge-on-face and coincident-edge *segments*
     certified by reconstruction from their bounding vertex records
     (rule derived and pinned in `topo::census` module docs: between
     two backed bounds, two lines sharing two points are one line; a
     missing bounding record is `UndeclaredContact`, never inferred) —
     and, since M9-2, face (`CurveContact`, `PatchContact`;
     `docs/CONTACT-DESIGN.md` C3), whose rungs back a subordinate
     vertex event from a declared face pair.
   - Census posture, stated honestly: **certification strength equals
     its skeleton** — a `CurveContact` is certified at its jet samples
     plus hull bounds, a `PatchContact` by definitely-positive region
     overlap in the shared chart, and a vertex-granularity area contact
     via its corner/segment records;
     **nested-shell pure containment (zero coincidences — a void) is
     census-invisible and certifies** — that is the F8 voids story
     (see the M3 conventions below), not a gap the census must catch.

   **Touching is always backed by explicit intent (Evan's condition,
   #42, part of the ratified invariant)**: (i) operand coincidences
   are only ever structural (shared key) or declared (recipe data) —
   near-coincidence NEVER silently becomes contact (escalated typed
   error instead, per F6); (ii) result-side touching arises only from
   those intentional coincidences propagated through the boolean node,
   and the result carries machine-checkable declared-contact records
   (the ON-set survivors, carried across seam-zip/merge mints by a
   descendant map — never re-derived, which would be scan-to-bless);
   (iii) an *undeclared* contact discovered at validation is a hard
   error, never blessed.

   **Validity class rides the result wrapper, never a mutable `Body`
   field** (the binding F1 interpretation, pinned in the PR 6a spec):
   a boolean result is `BooleanBody` — body + contacts — whose
   non-empty contact list is the 3′-grade currency and whose at-rest
   gate is `validate_pseudomanifold(&body, &contacts)`; empty-contact
   results remain plain tier-3 currency, and the two gates agree there
   (3′ ≡ tier 3 with empty contacts, pinned). Tier-3 bodies remain the
   default currency.

   **Representability boundary (F2, sharpening — not revising — the
   round-9 "non-manifold results are typed errors" ratification)**:
   pseudomanifold touching via *distinct* entities (two vertices at
   one point, two edges on one segment) is representable in the
   half-edge structure, is what the pipeline naturally produces, and
   is a typed *success* carrying its 3′ declarations. Genuine
   non-manifoldness — a single edge with >2 faces, a shared-entity
   wedge fan — is unrepresentable and stays a typed error at the site
   that would have needed it. "Non-manifold" means non-representable;
   3′ is the honest name for the representable touching class.

**M2 structural conventions (ratified at the M2 exit sweep, 2026-07-20/21):**

- **Sweeps emit single-shell primary boundaries; every CAVITY is born
  through the shared void-insertion door (Evan, 2026-07-20; refined
  2026-08-22, #907).** A cavity's boundary is a disconnected interior
  shell, and its bookkeeping — orientation, census participation,
  containment evidence — has exactly one home: the void-insertion
  door the boolean owns, factored callable without the SSI pipeline
  for provably-no-crossing cases. Three producers satisfy it: boolean
  subtraction; `shell`'s sealed hollow (`docs/OFFSET-DESIGN.md` O4 —
  the degenerate no-crossing arm); and the full revolve of a holed
  profile, DEFINED as `revolve(outer) − revolve(hole-as-outer)` and
  executed through the same degenerate arm — the hole's swept
  boundary provably touches nothing. `FullRevolveHoles` retired when
  that unit landed (VERBS-PLAN's RING row, 2026-08-22): the door is
  `topo::insert_void`, the boolean fallback and the holed full
  revolve are its two live producers. Recipe-layer sugar may wrap any
  of these — sugar above the kernel; the door stays the one
  birthplace. (`UnsupportedToroid` is likewise permanent: a D3
  ring-torus boundary — spindle tori have no representation — not a
  scope cut.)
- **The minimal sphere at rest is V2/E2/F2** (M2 PR 5): tier 2's
  valence-1 ban makes the "minimal" V2/E1/F1 sphere unrepresentable at
  rest — a one-band wire sweep leaves valence-1 poles, so axis-touching
  full revolves sweep two π-bands, giving poles valence 2 (the angle-0
  and angle-π meridians). A deliberate consequence of the tier
  definitions, not a defect.
- **Parameterization conventions (M2 PR 1, ratified-by-documentation;
  authoritative text in the `geom` crate docs and its `curves`/
  `surfaces` module docs):**
  curve entities are complete loci (full circle, infinite line); an
  edge's bounds derive from its vertices via the `he_plus`-forward
  contract (increasing parameter runs start→end of `he_plus`;
  certification enforces forward, nonzero, ≤ one-period spans, with
  the stored interval a certified cache reconciled against
  vertex authority by endpoint pinning). Shared azimuthal frame for
  all revolution surfaces (axis = +a₃, v_ref = axis × u_ref, seam at
  u_ref — for revolved bodies u = 0 IS the profile half-plane);
  sphere uses latitude (not colatitude); cone v = slant length with
  the apex a true chart singularity (poison normal, never sampled);
  normals are the chart's ∂u × ∂v with no "outward" contract —
  topology carries sense. `Seam` is defined SPATIALLY (the u_ref
  half-plane meridian), which on mirror-nappe cones differs from
  chart u = 0 (M2 PR 5/6 finding).
- **Profile format (M2 PR 2, ratified in the #24 conversation):**
  a profile loop is a vertex chain with bulge (b = tan(θ/4) of the
  arc to the next vertex, DXF-compatible: positive = CCW sweep) —
  zero representation-consistency conditions by construction; closed
  carriers split into ≥ 2 vertices (full-period edges stay
  representable in topo; the split is input-layer); winding is
  invisible to users (roles derive from containment; canonicalized
  internally). Downstream re-inspection of arc geometry uses the
  stored bulge/carrier data (θ = 4·atan|b| or minted parameter
  spans), never endpoint atan2.
- **Declared-tangency discipline (#101, ratified 2026-07-25; landed
  M4 #109):** profiles refuse undeclared definite-Zero tangency at
  junctions (`UndeclaredTangency`, with a repair menu); declarations
  are verified, never trusted (`TangencyContradicted`); the
  PATHS `.fillet(r)` constructor authors exact tangency by
  construction and declares it, with fit gating
  (`TangentJointOutOfRange` when a tangent point falls outside its
  leg); **same-carrier is identity, not tangency** — declared
  cocircular/collinear joints refuse with `same_carrier: true`
  (two-arc circles stay legal). Zero new ε: the per-junction
  classifier reuses the existing carrier predicates verbatim.
  Persistence keys the flags (`tangent_joints` in schema v1, #112).

**M3 structural conventions (ratified at the M3 exit sweep,
2026-07-23; forks resolved with Evan in #42, 2026-07-20/21):**

- **Curved booleans retire per arm, never wholesale (F5).** A face kind
  with no arm refuses typed `CurvedBooleanUnsupported` /
  `CurvedPairUnsupported` naming the pair, never falling through to a
  containment verdict a curved boundary can defeat. The live arms and
  the standing frontier are the (vi) entries below.
- **Coincidence discipline in the reduction (F6).** Every
  reduction/classification comparison is a Q1 trilean predicate:
  definitely-off ⇒ clean side, exactly-on ⇒ ON, in-band ⇒
  **escalated typed error** (a genuine sliver: the operand pair is
  ill-conditioned at this ε). Near-coincidence NEVER silently becomes
  contact; the round-8 ladder (structural / declared / typed sliver)
  governs — no EPS snapping anywhere in the pipeline. Consequence,
  stated honestly: booleans on independently modeled nearly-touching
  bodies fail loudly rather than guess — the design thesis; the
  resolution is an explicit D7-style repair/adoption op (M5+).
- **Maximal-faces precondition and the merge stage (F7).** Booleans
  precondition no two adjacent coplanar faces (typed
  `NonMaximalFaces`); the explicit opt-in normalization op is
  `merge_coplanar_faces` (merging is never silent, per the M2
  no-automatic-face-merging ratification), and boolean *outputs* run
  it as a **documented final stage of the op's contract** — the seam
  zip manufactures coplanar pairs by construction; the recipe records
  one boolean node, not hidden healing. Merge glues on the
  **structural and declared rungs only** (shared surface key or the
  declared bit-fingerprint rung — see the M4 retirement note in the
  roadmap); numeric coincidence never merges. Load-bearing dependency,
  recorded here: `merge_coplanar_faces` **never elides vertices** (it
  merges faces; collinear vertex chains survive), and tier 3′'s
  strict record-drop rule (a contact record whose vertex pair fused
  into one vertex is consumed and drops — the census agrees:
  structural now) is correct *because* of that; any future
  collinear-vertex elision re-opens the record-carriage class (M3
  PR 5 review, R5).
- **∅, disjoint, and voids are typed results (F8).** ∅ is a typed
  success value (`BooleanResult::Empty`), not an error — the per-node
  result DAG (GQ2) wants a value; disjoint unions and voids are
  tier-2-legal multi-shell bodies (the M2 single-shell *sweep*
  invariant is untouched). A∖B with B strictly inside A births the
  first legitimate voids. **The cavity invariant, as refined at #907
  (see the M2 bullet above): every cavity is born through the shared
  void-insertion door, with caller-certified containment** — the
  boolean fallback supplies its probe verdicts, the holed full
  revolve carries the profile's validated 2-D margins (its holes ARE
  closed inner shells, inserted through the door since VERBS-RING),
  and `shell`'s sealed hollow carries its offset margin (`topo::shell`,
  the degenerate no-crossing arm). The
  extrude/full-revolve hole asymmetry is structural, not an
  inconsistency: extruded holes are cap-to-cap tunnels (one shell,
  genus); full-revolve holes are cavities (a second shell, through
  the door); partial revolve is extrude-shaped and carries holes in
  its one shell. A void's inner shell carries zero
  coincidences and is census-invisible at tier 3′ — a valid void, not
  an undetected contact.
- **The envelope (typed refusals on record, never silent gaps;
  M3 entries updated at the M4 8c exit sweep, 2026-07-27; M5's
  curved entries added at the PR 14 exit sweep, 2026-08-03):**
  (i) **RETIRED (M4 PR 5, #102)** — the operand-internal-declaration
  gap: ops consume declarations as recipe data threaded by name
  through op composition (F5/N-decisions), and the closure corpus
  certifies that a reused 3′ body's declared coincidences re-certify
  downstream. (ii) **the both-sided pinch split
  frontier** — split's below-copy completeness is delivered via the
  exact mirror identity `split(S, n) ≡ swap(split(S, −n))`
  (piece-assignment equivariance; ruled refinement-not-fork at PR
  6a), so single-sided pinches succeed symmetrically; a *both*-sided
  zero-area pinch refuses typed (the BOTH_SIDED fixture pins the
  frontier); a native below-copy insertion lane is the recorded
  future upgrade. (iii) **boundary-on-boundary seams** — narrowed at
  M4: corner-flush/stacked-full unions and the corner-aligned table
  now certify through declared intent (#102), and the seam-region
  anchor repairs (#108, #113) closed the isolated-seam-loop and
  nested-island classes; what REMAINS typed-refusing:
  **REST-contact joins** (the crosslap mate — a pure rest contact
  needs a join-stage lane; `crosslap_rest.rs` pins both doors;
  banked opener, #102 R7) and **reflex-corner tilted crossings**
  (PR 5.5 envelope, sector-width bound documented at the error
  sites). (iv) **the post-#113/#116 join residue, honestly scoped**
  — the anchor-exhaustion arm is LOAD-BEARING (no unreachability
  claim, the #93 lesson): coincident-plane classes beyond the
  declared/anchored repertoire, sub-ε grazes (genuine slivers
  escalate typed per D4), and ill-formed input faces refuse typed
  rather than classify; CDT exterior classification itself is now
  structural (even-odd flood fill, watertight by construction,
  #116), so the mesh lane no longer contributes residue of its own.
  (v) **RETIRED (M5 S1, #140)** — the PLANAR half of the
  REST-contact join gap named in (iii): the crosslap mate zips
  through a declared-contact join lane, at exact volume, both doors
  pinned. **RETIRED (M9-3)** — curved REST contact: a declared
  cylindrical conformal class zips through the join lane, verified
  structurally against the chart-region predicate
  (`docs/CONTACT-DESIGN.md`). Carried residue: a cylindrical-only
  declared `Rest` with no planar `Rest` beside it does not reach the
  rest lane (#1032), and the torus declared-`Rest` lane is banked
  (#968).
  (vi) **the M5 curved frontier, as built** — every one of these
  refuses TYPED with a message naming its own blocker, and each is a
  banked unit rather than an open question:
  **(a) composition surgery** — an in-place edge-blend that replaces
  a rim edge with a blend band inside an existing body. *(DISCHARGED
  at M6 unit 1: `sweep::fillet::surgery` splits support faces along
  the stored trimlines with rings carried through, grafts blend
  walls/octants in place, and replaces rim chains with slit-seamed
  torus bands — the composed die is ONE body. The curved-pierce
  door's conic arm is now a genuine clearance verdict for CIRCLE
  carriers (`bool_circle_curved_clearance`); what remains typed
  there: ellipse/NURBS carriers, crossing circles, and the
  containment stage's partial-sphere extent. Run-outs at
  partially-requested corners, junction carry-through and concave
  blends remain the fillet assembly's named refusals.)*
  **(b) the SSI generic-`T` lift — CLOSED (M6-2).**
  `Pcurve::Fitted` is admissible: the enclosure
  seam takes any bracket-carrying scalar into the C9 ring
  (`T: Bounds`), the certificate is derived and carried at `T`
  (`SsiCertificate<T>`, the `Decide + Bounds` seam in
  `geom-brep/src/ssi/*`), and projection follows the ratified
  f64-structure + T-payload pattern. `Pcurve::Fitted` landed with a
  `PcurveFittedLane` static split (the `PropsQuadLane` shape: `f64`,
  `Probe` and `Interval` derive the certificate, `Dual` refuses typed
  — *a dual may not certify*, Wave 0 decision **D1** of
  `docs/SMELL-SCAN-2026-08.md`, ruled 2026-08-19; the refusal is
  `CertifiedEnclosure`'s, not `Bounds`', since a dual **does** carry a
  bracket — the value channel's). The banked walk-row-2 obligation is
  discharged: a cylinder×sphere rung-3 edge reaches a body at rest
  carrying a fitted cache whose full C2 certificate (hull sup-norm +
  uniqueness tube) is RE-DERIVED at rest, at `f64` and at `Interval`;
  `no_body_at_rest_carries_a_nurbs_carrier_or_face` flipped to its
  successor law. What the fitted analytic lane does NOT claim is
  stated in `EnvelopeStatement::OnLocusHull`: on a periodic analytic
  chart `S ∘ P` is transcendental, so the ring bounds the carrier's
  incidence with the surface between samples (plus the uniqueness
  tube), while the map residual itself is certified at the schedule.
  Remaining `f64`-only by design: `ssi::jet`/`march`/`system`
  (untrusted candidate generation) and the analytic composite's
  implicit form, which is `f64` structure and therefore refuses a
  WIDENED analytic operand typed rather than picking a representative
  surface out of the family.
  **(c) loft/sweep body assembly — CLOSED (M6-3, completed at
  #207).** `Loft`/`Sweep` nodes run
  `sweep::loft_body`/`sweep_body` (extrude's topology, skinned
  geometry; `EdgeGeometry::IsoCurve` seams with exact line-in-UV
  pcurves); tier 3's +V check consumes the exact per-span tensor
  Newton–Cotes NURBS-patch flux for non-rational walls and the
  Taylor-remainder hull composite for rational ones (M8-3, #309/#353),
  the latter refusing typed `QuadratureBudget` with its measured width
  when the enclosure will not narrow. Curved-path sweeps and
  non-uniformly-spaced lofts needed #207, which makes an integral
  input skin exactly-unit-weight. The analytic-chart pcurve
  completion (walk row 4) landed in the same unit: cone, sphere and
  torus charts certify and MINT their closed-form classes (cone
  rims/rulings, sphere polar/meridian circles, torus
  parallels/meridians — sphere walks know the chart involution and
  the pole's zero azimuth lever); the sphere's GENERAL circles
  certify through the fitted door (`certify_fitted`'s Circle-carrier
  rational-chain arm, `OnLocusHull`), and the ball/cone/donut and
  the filleted die's eight sphere octants carry stored pcurves at
  rest. The mint pass now carries the
  `PcurveFittedLane` bound (PCURVE P-2, #498), so the bound that
  blocked the fitted routes from the mint side is PAID; it is
  signature churn and not a capability loss, since `Dual<T>`
  implements the trait with a statically refusing impl and no scalar
  is excluded. What that unit wired through it is U2's `General` arm
  for interior-column `Intersection` carriers; MINT-side wiring of
  the fitted general-CIRCLE route is still open (the
  oblique-trihedron octant faces stay legally uncached), as are the
  cone/torus oblique classes, which have no ring-computable meters
  composite and refuse with the class named.
  **(d) cyl×sphere germ chords** — only `(Plane, Cylinder)` and
  `(Plane, Sphere)` germ arms are wired. (b) has landed, so the
  storage half of the blocker is gone: a fitted carrier's chart image
  now EXISTS as `Pcurve::Fitted` and certifies at rest. What remains
  is the join lane itself — `run_azimuth_window`/`chart_pcurve` have
  no cyl×sphere window analog, and `chart_azimuth_range` reads a
  closed form a spline image does not have (it answers with the empty
  range, so a window built over one refuses rather than accepts).
  Banked past M6 by M6-PLAN. Sphere×sphere seams, cone and torus
  operands refuse alongside it.
  **(e) the NURBS extent test** — the boolean fallback's curved-extent
  test is re-gated typed (`NurbsExtentUnsupported`) rather than left
  to vertex-probe silence. Half its old blocker retired at (b): the
  foot-point projection is no longer `f64`-only, so the Interval-lane
  objection is gone. The remaining blocker is that the extent
  ARGUMENT has never been written — `implicit_residual` is poison at
  NURBS, so a certified extent needs a foot point plus a bound on how
  far the patch can reach past it, which is a derivation nobody has
  done. Retiring the gate needs that test, per-arm (C12.1).
  **(f) the canal-surface general blend** — an approximating surface
  (the `Surface::Approx` class, `docs/OFFSET-DESIGN.md` O2), for fillet
  chains whose rolling-ball spine
  is neither a line nor a circle (`FilletError::SpineUnsupported`).
  Deliberately **PARKED**, not scheduled: no acceptance shape
  consumes it (the die is analytic end to end), and building reviewed
  machinery with no caller is the dead-code pattern M5's reviews
  repeatedly punished. It re-opens with the milestone that ships its
  first consumer (Band-3 fillet breadth).
  Two further M5 limitations are latent-and-loud rather than banked
  units, and are recorded so they are not rediscovered as bugs: a
  meridian-tangent circle is in-lane but uncertifiable (no
  constructor mints one; the refusal is loud), and genuinely-oblique
  trihedral corners build through tiers 1–2 and then report
  `VolumeUncomputable` — a gap in the props inventory (a
  spherical-triangle form, or quadrature extended to sphere faces),
  not in the body.

### D2 (agreed): Intensional edge descriptions; no extensional fallback

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
  | TangentIntersection { s1, s2, witness }
                                      -- tangential contact locus; same shape as
                                      -- Intersection, margin one order up
  | Seam         { surface }          -- same surface on both sides (closed-
                                      -- surface parameterization seam)
  | IsoCurve     { surface, u, v0, v1 }
                                      -- a chart iso-line (loft/sweep walls)
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
`TangentIntersection`), not the need for an extensional fallback.

Validity of `Intersection` requires *transversality*: normals of S₁, S₂
linearly independent along the locus (equivalently `T_pS₁ + T_pS₂ = ℝ³`),
so the implicit function theorem makes S₁∩S₂ locally a 1-manifold. The
transversality margin (angle between normals) is a predicate-with-margin
(Q1) and governs the conditioning of every derived cache. Cases that fail
transversality get other variants: parameterization seams (`Seam`),
tangential contact such as fillet–support contact curves (the
`TangentIntersection` variant — *the variant mirrors
`Intersection` — same shape, same witness pin, margin one
differential order up* — the fillet construction knows its contact locus
directly, but *imported* fillets force the intrinsic form: along a fillet
boundary edge the blend and support surfaces share tangent planes
identically, so `Intersection`'s precondition fails everywhere on the
locus). `TangentIntersection`'s intrinsic validity condition sits one
differential order up: surfaces coincident within ε and normal-parallel
within the derived angular threshold ε·κ_rel (D4 ¶1: lever arm
r = 1/κ_rel) *along* the locus, separating quadratically *transverse* to it
(relative normal curvature bounded away from zero — otherwise the
surfaces osculate over a patch and the "locus" is not a curve). The
uniform pattern: **every variant is a validity predicate plus a margin**
(Q1) — first-order (normal angle) for `Intersection`, second-order
(relative transverse curvature) for `TangentIntersection`. Reconstructing a
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
`TangentIntersection`, with the rolling-ball construction demoted to supplying
the witness and initial caches. Construction history lives in D5
provenance, never in the geometry description, so native and imported
bodies carry identical descriptions. The taxonomy is thus a dichotomy:
**intrinsic variants** (`Intersection`, `TangentIntersection`) describe loci
determined by their surfaces; **conventional variants** (`Seam`,
`MappedCurve`) carry the defining data for loci the surfaces *under*-
determine — parameterization seams (infinite-order contact; the seam's
position is pure convention), face splits at smooth profile joins
(iso-curve edges introduced by sketch entity boundaries; at a G2 join
even `TangentIntersection` fails its margin, and rightly — nothing intrinsic
distinguishes that curve from its neighbors), and user splits.
`MappedCurve` does not reintroduce `Explicit` through the back door
because of its shape: one authoritative source (`curve = map ∘ source`,
pcurves derived as certified caches), never two peer representations
needing cross-reconciliation. A locus in the ambiguous band — a dihedral
within a few derived angular thresholds of tangent (θ ≲ K·ε/r at the
governing lever arm), certifiable as neither `Intersection` nor
`TangentIntersection` — fails loudly at construction exactly as at import (D4);
a conventional description is not an escape hatch from ill-conditioned
geometry.

**Witness contract (sharpened at M2, PR 7 exit sweep; found by PR 3's
review as finding S2).** "Selected by the witness point" is verifiable
only if the witness is *pinned*: the stored witness IS the edge's
mid-parameter point (witness = carrier(mid)), enforced by certification
(`WitnessMidpoint`). The witness still selects the connected component;
pinning removes the aliasing freedom that let any point on the component
certify, including points encoding a wrong winding. Residual freedom is documented
where it is geometrically invisible (circles: joint whole-period
translation). Construction obligation on every op that mints an
`Intersection`: compute the witness as carrier(mid) with the certification
schedule's own association order.

**Prefer-intrinsic is tier-3-enforced (ratified 2026-07-19 with Evan;
landed in M2 PR 4's fix pass).** The prefer-intrinsic rule above is not
advisory: at rest, every *definitely-transverse* edge must carry
`Intersection` (`TransverseNotIntrinsic` otherwise); a definitely-smooth
join descends one order — a jet-determinate tangency must carry
`TangentIntersection` (`TangentNotIntrinsic` otherwise, and only where
`geom_brep::tangent_certificate_lane` admits the class), while a
second-order-underdetermined join keeps its conventional `MappedCurve` by
the predicate itself; escalated dihedrals, `Seam` edges and
NURBS/`Approx`-adjacent edges are exempt — so ε-tightening
can escalate but never flip a valid body to invalid. Mixed per-sample
classifications are conservatively unenforced (documented). Rationale: an
unenforced preference drifts silently — exactly the failure shape this
document exists to kill; the check is nearly free because tier 3 already
samples dihedrals per edge.

This makes D5's provenance load-bearing rather than bookkeeping: the
intensional description largely *is* the provenance.

### D3 (agreed): Analytic surfaces special-cased; NURBS as the general fallback

Plane / cylinder / cone / sphere / torus are first-class variants alongside
NURBS (as in Parasolid), not converted to NURBS. Most mechanical geometry is
analytic; analytic×analytic intersections have closed forms (exact,
robust), while NURBS×NURBS intersection is a numerical marching problem we
defer as long as possible. A seventh variant, `Approx`, is the certified
approximating class (`docs/OFFSET-DESIGN.md` O2): a fitted NURBS carrying
the intensional description of what it approximates plus a certificate
bounding the distance, re-derived per face and never trusted from storage.

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
   **The two-tolerance principle (RATIFIED 2026-07-29, Evan's lgtm on
   #129; born in the #124 thread).** Two roles
   that D7 already separates at the import boundary are adopted as
   kernel-wide vocabulary: **ε_precision** (this section's ε — "the
   precision we represent": certification residuals, D4 ¶2, what
   gets built) and **ε_input** — "the least precision the user might
   care about": what counts as too-close-to-a-coincidence when
   interpreting input, and the threshold below which user-facing
   distinctions are noise. ε_input > ε_precision always (differences below ε_precision are
   not even representable claims), and **ε_input IS K·ε — a synonym,
   not a third dial** (simplified per Evan's #129 review: K stays
   the one knob, `Tolerance.k`; the vocabulary contribution is the
   ROLE NAMES, not new machinery). The Q1 escalation band remains
   precision machinery (escalate-never-guess) as ever. Consequences,
   binding once ratified:
   (i) **User-facing messages and recourse never fork on exactly-on
   vs in-band below ε_input** — both are "coincident at any
   precision you could care about"; ONE message, ONE recourse
   (declare the coincidence / move the geometry / lower the
   tolerance — the three-arm sentence at every site whose question is
   "is this margin decidable"; a contact site, whose question is "did
   anyone declare this", drops the third arm per SELECT-DESIGN §3d),
   with the margin riding the error payload as data.
   Kernel SEMANTICS keep the distinction (ON-set classification,
   escalation, declared-verification are unchanged — this is
   message policy, not predicate policy).
   (ii) The error taxonomy carries the message-level rework (M5 S6):
   the collapsed pairs are profile UndeclaredTangency/TangentialContact vs
   Escalated, boolean UndeclaredCoincidence vs Escalated, census
   UndeclaredContact vs CensusEscalated, split_edge
   SplitParamNotInterior vs SplitParamEscalated, split-join
   DegenerateSection vs Escalated, sweep Degenerate*/
   VertexCrossesAxis vs *Escalated/SliverRadius, certify
   NotTransverse vs Escalated, props DegenerateFace vs Escalated —
   with `merge_coplanar_faces`' already-collapsed error as the
   in-repo precedent and the shared `Indeterminate` Display string
   as the natural carrier of the unified recourse. Variants may
   stay distinct as DATA; their user stories converge.
   (iii) D7's ε_in is an instance of ε_input (per-import override
   of the interpretation threshold), not a separate concept —
   adoption re-runs classification at a different ε_input, exactly
   as CURVED-DESIGN's D7 leave-room obligation already requires.
   (iv) **The rule binds a predicate's DEFINITE arms too, not only
   its indeterminate one** (RATIFIED at the M5 exit sweep, Evan on
   PR #169 comment 5171303851, 2026-08-03; the S9 lesson — the
   chord_spec azimuth-window repair introduced new definite arms
   that silently missed the two-tolerance shape,
   caught as review MIN-1 at #145). When a predicate grows an arm,
   the arm inherits the obligation: a NEW definite outcome that a
   user could reach by moving geometry must tell the same one story
   with the same one recourse as its in-band sibling. The failure
   mode this closes is subtle and was observed: the indeterminate
   arm gets the careful unified message because that is where the
   principle was written, while a freshly-added definite arm ships a
   `{:?}` payload or a second, differently-worded recourse — and the
   user's experience of "the same situation" forks after all. Review
   checklist form: for every arm added to a decision, name which
   ε_input story it belongs to, or say why it belongs to none.
   **Chordal tolerance δ is not a tolerance in this sense (ratified at
   M2, PR 6).** Tessellation/export take a per-call *display parameter*
   δ (chordal deviation), deliberately distinct from ε: δ is chosen per
   export, varies freely, and participates in no kernel validity
   decision. The tessellation promise is **certified-conservative** —
   closed-form sagitta/deviation bounds guarantee the mesh lies within
   δ of the true surface (honestly δ+ε, since mesh vertices sit on
   carriers only up to the certified residual; STL's f32 narrowing adds
   ≤1 ulp per coordinate on top, documented in the writer) — but this
   is an *export promise*, explicitly not a kernel invariant. The mesh
   layer never reads ε for sizing; display-layer comparisons are
   deliberately not Q1 predicates (none decide kernel topology).

   > **RULED 2026-08-21 — the stronger reading, and the pole
   > classification owes a GUARD rather than a qualifier here.**
   >
   > **The count that used to stand here was false** (#872): the mesh
   > layer's ε inventory is a computed pin
   > (`mesh/tests/all.rs::the_eps_inventory_is_pinned`), not a sentence,
   > so it cannot drift again. *"Never for sizing"* stands and is
   > checkable from `sizing`'s signatures.
   >
   > **The question underneath was the real one.** One of those reads is a
   > **classification**, not a bar: pole identification substitutes the
   > chart's exact `v` and emits two polygon entries instead of one, so an ε
   > that flipped it would **move emitted coordinates with δ held fixed**.
   > That makes the sentence above true of every body this build can mint
   > and **not a theorem** — nothing in the tree flips it, but no argument
   > establishes that nothing can, and a STEP import is the plausible route
   > in.
   >
   > **The ruling: this paragraph keeps its promise unqualified, and the
   > classification is guarded so the promise stays true** — filed as
   > **#896**. Weakening a ratified promise to accommodate a state that
   > could only arise from **value coincidence** would make this document
   > quieter about exactly the case worth hearing about, and this project
   > does not read intent into numerical coincidence. The guard says the
   > same thing honestly and fails loudly if the belief is wrong.
   >
   > **#895's junction guard does not discharge #896**: it compares
   > declared vertex against declared vertex, and this case is a junction
   > against an **undeclared** analytic chart pole. Where the pole is itself
   > declared the two overlap; where it is not, nothing looks.

   **The margin dimensional convention (RATIFIED 2026-08-05, Evan 👍 on PR #205 comment 5195787412; shaped
   in-chat with Evan — non-generic erased annotations, his call —
   from the du_of_rims / F3 / F4 defect family and the
   predicate-dimension audit).** ε is a length: the maximum
   deviation from specified geometry at a single point. Four
   clauses make that semantics structural instead of conventional:
   (i) **Margins are lengths, by signature.** The `classify`/Band
   seam takes a `#[repr(transparent)]` `Margin<T>` newtype (erased
   at compile time; NO dimension algebra, no generic dimension
   parameter — most kernel functions are single-kind per argument,
   so the annotation is a signature fact, not a genericity layer).
   The only constructors are blessed doors that make the dimension
   argument explicit at the call site: a coordinate/parameter
   difference that IS a length; a dimensionless quantity levered by
   an arm; a norm; a volume defect over its perturbable boundary
   area (mean displacement). Three derived doors are named special
   cases, not new kinds (fix-pass adjudication of the PR #213
   review's Y2 derivations): the sagitta κ·L²/2 is the levered door
   applied twice; the reciprocal form x/κ_rel is the levered door at
   D4 ¶1's own tangency lever 1/κ; a parameter span through its
   per-kind metric rate is clause (iii) surfacing at the seam. The
   quotient door is the measure-over-lever form (an area or signed
   volume over the boundary length / lever / surface area that
   scales it to the point displacement it subtends: 2A/P, V/A, the
   chart-orientation a×b·n̂/r). The CONSISTENCY BACKSTOPS are
   excluded from the seam entirely (Evan's #213 layering ruling):
   inequalities between integral results — the volume_backstop
   family — decide on bare T through the invariant lane
   (`k_stats::decide_invariant`), no Margin minted, and a certified
   violation is a Corrupt-class kernel-invariant error, never a
   validity refusal. A
   site where no door honestly fits is
   a finding, not a cast. The vector/linalg interior stays bare `T`
   (annotating `Vec3` ops would recreate the algebra problem); the
   typed surface is where contracts are single-kind — which is
   where every observed defect lived.
   (ii) **No dimensionally-heterogeneous uniform payloads.** A
   field whose dimension depends on a runtime kind tag (the
   `Rim.level` shape that hid the ×arm defect) is illegal; kind-
   dependent data lives in per-kind enum variants with honestly-
   typed fields (`du_of_rims`' `RimLevel { Length(v), Unit(s,c) }`
   is the pattern, #197).
   (iii) **Parameter-space values cross to model space only through
   per-kind metric doors.** Parameters are irreducibly kind-
   dependent (axial length on a cylinder, latitude on a sphere) and
   that is fine while arithmetic stays in parameter space; the
   defect class is a parameter-space quantity reaching a model-
   space comparison without the per-kind conversion. A struct
   carrying parameter-space values ACROSS kind boundaries for
   uniform consumption is the named smell.
   (iv) **Inequality gates split sign from magnitude** (the F3/F4
   fix-pass lesson, #200): a certified sign-certain violation of an
   inequality is a dimension-free fact and refuses with no ε
   involved; the banded, ε-scaled comparison governs only the
   near-zero region where sign is uncertain — and both arms consume
   the SAME metered comparand, since dividing by a certainly-
   positive lever cannot move a sign but keeps the recorded margin
   a length (K-telemetry attribution stays dimensionally honest).
   Rollout: the classify seam first (every recorded margin becomes
   `Margin<T>` by signature); extension is opportunistic as
   signatures get touched — no big-bang sweep. The migration ledger
   is `docs/predicate-dimension-audit.md` (the audited family is
   already clean by measurement; F12's expression-layer row is the
   first out-of-family site the newtype would catch at compile
   time). *Clause (i) is EXECUTED (the margin-migrate unit):
   `geom_core::k_stats::decide` takes `Margin<T>` by signature, the
   blessed doors live beside `Band` in `geom-core::predicate`, and
   the ledger's flagged rows ride the row-keyed
   `k_stats::decide_flagged` finding lane — visible typed debt, not
   casts (there is no raw construction door). K-telemetry byte
   identity over the probe census is the executed acceptance.*
   **The tessellation criterion is DISTANCE-ONLY (ruled in session
   2026-08-02/03, Evan + orchestrator concur; RATIFIED at the M5 PR
   14 exit sweep — Evan, PR #169 comment 5171303851,
   2026-08-03).** The ruling, scoped verbatim: *"NO
   angular-deflection criterion in the certified tessellator.
   Grounds: every contracted consumer is manufacturing-shaped (STL
   chordal semantics, admesh, props now quadrature-based); a
   certified angular bound would cost new normal-variation enclosure
   machinery purchased only for visual smoothness δ already buys; the
   OCC-norm expectation belongs to display+manufacture combined
   engines. The angular/screen-space criterion is the future
   DISPLAY-MESH lane's (GUI milestone), honestly uncertified there.
   δ-vs-angle arithmetic recorded: θ≈√(8δ/R) — distance-only
   under-refines small radii in angle; sizing δ to r_min
   over-refines by ~R/r_min in facet count; acceptable for current
   consumers."* The consequence to hold onto: when a display lane is
   built it gets its own, separately-honest criterion, and it must
   not be mistaken for — or quietly promoted into — the certified
   export promise above.
2. **Every derived cache carries a certified residual bound** against its
   intensional description (D2): fitted intersection curves, projected
   pcurves, refit 3-D curves. Kernel invariant: `residual ≤ ε` for every
   derived item in a valid body; the `topo` validator checks it.
   "Certified" is initially a conservative numerical estimate, upgraded to
   an interval-verified bound when Q1's machinery lands.
3. **Failure is a typed, actionable error naming the failing check and
   the entity** — consumable by humans and by the error-propagation
   machinery. The carrier is `CertifyError::ResidualExceeded { check,
   sample }`, wrapped by the attachment gates and by the validator's
   `ValidationError::EdgeCertification { edge, error }`; the residual
   MAGNITUDE rides the escalated arm's `Indeterminate` rather than the
   definite one, because no `f64` projection of a generic `T` exists on
   every scalar lane. Geometry that can't meet ε
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
4. **The shared at-rest gate** (ratified 2026-08-08, issue #260 option
   (a)): steps 1–3 certify each *entity's* description; the *body* is
   then handed to the kernel's own at-rest validator — the tier-3
   battery, or the tier-3′ form where declared contacts exist — and
   only a body it passes ships from import. Same function, same tiers,
   as a native body's caller runs; import holds no idea of validity of
   its own, so there is nothing to drift (D9 engineering convention 2
   — structural sharing, one validator). A body that fails is a typed
   *validity* refusal naming the failing check and its entities, an
   escalated verdict included (escalate-never-guess: an undecidable
   verdict is a refusal, not a pass).

   *Per solid, not merely per file.* Several tier-3 invariants are
   whole-body sums — the +V check is boundary flux over every shell —
   so in a multi-solid file an inside-out solid cancels against a
   right-side-out neighbour and the aggregate reads Zero, which is
   exempt. "Every imported solid passes the gate" therefore means each
   solid is asked on its own body, before aggregation, with the
   refusal naming which one; the aggregate pass remains for the
   cross-solid structure no per-solid view can see.

   *Scope, named.* On a body with no declared contacts, 3′ is tier 3
   **plus the coincidence census actually run** — strictly stronger.
   A file carries no arena keys, so the import-side declaration channel
   is POSITION-anchored and belongs to the adopting CALLER
   (`ImportOptions::declared_contacts`): declarations resolve against the
   assembled body and are certified by the SAME tier-3′ gate a native
   declared-contact body runs, and an anchor that does not resolve
   refuses typed rather than being dropped. An imported assembly whose
   parts *touch* therefore refuses UNDECLARED at the gate and certifies
   WITH the declaration — the equivalence with a natively built twin,
   not a residue (executed at M9-2).

   Making un-gated bodies *unrepresentable* at every kernel door — a
   currency type only the gate can mint — is the structural
   completion, banked at #250 with #260 as its design record.

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
validity; it would add only *editability*, and adoption does not do it.
Consistently: imported bodies carry no parameters, so error propagation
(M10) has nothing to vary over them.

Adoption reuses the kernel's own certification machinery — "is this curve
within ε of the described locus" is exactly the check the `topo` validator
already runs on derived caches. Note this is strictly *stronger* than
industry "shape healing" (which only patches data into self-consistency):
adoption must *explain* the data. Export is the easy direction (projection
from intensional to extensional); import is the inverse problem, and was
built as one at M7.

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
- **The kernel never panics on any INPUT** — every failure that an input
  can reach is a typed error. **A panic is therefore never a refusal: it
  reports that a bug has already happened.** Read it as *a firing panic
  is evidence of a bug*, never as *a panic in the source is a defect to
  remove* — which is the opposite of this rule.
  **The converse is a positive obligation, not a tolerance: a state that
  can only be a kernel bug MUST panic** — as loudly and as early as it is
  detectable (`unreachable!` or `debug_assert`, the D2 addendum's rows 4
  and 5 below). The whole value of such a check is catching the defect at
  the first moment it is observable, so downgrading one to silence, or to
  a typed error, launders a bug into a supported outcome. The two halves
  are **separate rules over disjoint state classes** — inputs never
  panic; bug states always do — and neither licenses the other's
  territory. *(Honest M1 footnote: operator debug postconditions
  are `debug_assert`s, and they are unreachable by input through the
  public API at every door but one — raw insertion is crate-internal,
  and the public mutation paths preserve tier 1: the Euler operators by
  the soundness theorem; the non-operator structural mutators
  (`ring_move`, `split_edge`, `movefac`, `merge_coplanar_faces`) by
  declaring the same debug postcondition or by being composed of
  operators that do; and the attach/metadata setters by re-certifying
  under their own tier-1 assertion or by writing fields tier 1 does not
  constrain. The claim is that closure property, deliberately not a
  count of the doors — a frozen count is what rots as doors are added,
  and `topo`'s
  `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`
  checks it against the real surface. `ring_move` remains the least
  obvious of the asserting doors, by the separating-curve
  argument documented on the method
  (a ring on a genus-0 component is a Jordan curve, so cross-component
  moves re-partition into legal pieces; non-separating rings force
  g ≥ 1).
  **The one door outside the property is `instance`'s graft**, which is
  a raw transplant rather than an operator run: it mints empty
  destination solids before transplanting, and its own docs state that
  a refusal raised mid-transplant leaves the destination partially
  written and *spent, never resumable* — an empty solid being the
  tier-1 error `SolidWithoutShells`. (The refusal that can actually be
  raised there is `JoinDesync`, from the reference remap; the doc named
  `GraftRecertify`, which that door's bridge never reaches — corrected
  in `topo` by #740, which found it while the argument that cited it
  collapsed.) A caller that discards a graft's
  `Err` and keeps the body can therefore fire a later operator's
  postcondition from **API misuse rather than a kernel bug**. That
  state class is not among the D2 addendum's five and is open as
  **S14** in `docs/SMELL-SCAN-2026-08.md`; this footnote records the
  door, not a disposition. Everywhere else a firing postcondition is a
  kernel bug by definition. What the kernel then DOES about such a state is the D2
  addendum below — which supersedes this footnote's original
  "typed errors where cheaply detectable, or documented garbage-out in
  release". The bounded-traversal half stands: never a hang; every
  traversal is bounded.)*
- Essentially no unsafe Rust outside vetted dependencies.

**D2 addendum — the bug-vs-invalid-state taxonomy (ratified 2026-08-19,
Evan's sign-off; Wave 0 decision D2 of `SMELL-SCAN-2026-08.md` §D,
raised by S43).**

*Why:* the kernel had **five** answers to "this state can only be a
bug", two of them mutual negations — `crates/topo` discards a missed
Euler precondition silently ~60 times (blessed by the footnote above),
while `geom` argues in its own prose that silent discard is the
wrong direction and a bare index panic is the right one (PR #447, never
brought back to D9). Both cited "fail loud". The rule below picks one.

**Silent discard is never an answer.** A state that cannot occur is
announced, not swallowed.

| # | State class | Mechanism |
|---|---|---|
| **0** | **Can this state be made unrepresentable?** — asked of every state, before the rows below | **change the type.** Preferred over every row below whenever it is available |
| 1 | Reachable by input, **invalid** | typed error |
| 2 | Reachable by input, **valid but unbuilt** | typed `Unsupported*` error |
| 3 | **Value-domain degeneracy** | poison — NaN / empty |
| 4 | **Kernel bug**, observable in a branch | `unreachable!` |
| 5 | **Kernel bug**, detectable only by re-derivation | `debug_assert` |

**Row 0 (ratified 2026-08-20, Evan's sign-off; raised by D27).
Representability comes before classification.** Rows 1–5 classify a
state that exists. Row 0 asks whether it should exist at all, and it is
answered **first**, before the classification begins. **When the answer
is yes, that is the answer** — not one disposition among six, but the
preferred one wherever it is available: a state that cannot be spelled
needs no error variant, no `Display` arm, no recourse row, no test
seed, and no row of this table.

*It is a question, not a class, which is why it is row 0 and not row
6.* It adds no bucket and renumbers nothing. What it adds is a step to
the procedure: **a lane that files a state under any row owes the
reason row 0 did not apply**, and a lane that reaches row 1 has already
answered row 0. Without that step the procedure has no place for *"this
state should not exist"* to be the answer, so a state fitting no row
reads as a gap in the taxonomy — which is exactly how
`FilletError::EmptyChain` came to sit under a row whose definition it
failed, and how a sixth row came to look like the fix.

*What "if possible" excludes, because a preference that outranks the
alternatives is otherwise a licence.* Row 0 is answered against the
cost of the type change, and the two ends of that scale are both in the
tree:

- **`EmptyChain` — yes.** The emptiness was an artefact of `Chain`
  holding its links in a `Vec` when the walk mints every chain from a
  seed link. Moving the first link into its own private field deleted
  the state, both its refusal sites and the pin guarding it: **a
  private field and a constructor signature, no public API change**
  (D27, PR #768).
- **`Live`'s generative brand — no, and it was already answered.**
  Making a stale certificate unrepresentable needs a brand lifetime on
  `Body`, which infects every signature in the workspace that names a
  body, the public API included. #755 weighed exactly that and rejected
  it, before row 0 existed — which is the evidence that this rule
  describes what careful lanes already do rather than inventing an
  obligation. **Row 0 must be able to say no out loud, and that is the
  precedent for where the line falls.**

So: yes when the change is local to the type and its constructors; no
when it propagates into signatures that do not otherwise care. A "no"
is a complete answer and is recorded as the reason a row below applies,
not as a defeat.

*Row 4's message convention stays prose, and D35 is the decision not to
gate it (PR #809, 2026-08-20).* The shape the conversion passes applied
— **the message states WHY the state cannot occur, not merely WHAT was
violated, and carries the values a reader debugging it would want** —
was settled by ruling across #740 and #744 and is recorded here rather
than as a rule anyone checks. **No gate was built, and the population is
the reason.** Re-derived at `25175838` over `crates/*/src`, an
`unreachable!` in macro-call position stands at **103** kernel sites
(plus 2 in `#[cfg(test)]` modules and 29 prose mentions a bare grep
conflates with them). **76 of the 103 are one state, not 76** — an arena
key proven live earlier in the same call did not resolve — whose row 0
is the `Live` brand and was answered *no* above; their messages are one
template, written by three conversion passes under one ruling, and read
uniformly. Only **three** messages in the whole population stated the
what and not the why, all three outside those passes, and all three are
fixed in the same PR. **A shape gate cannot separate the two**, and the
tree already shows both halves of why: `topo`'s
`d18_no_unreachable_message_can_impersonate_the_postcondition` is a
source walk over these messages that works *because* it forbids one
spelling, and `quantity`'s `row_index` is a message-**less** site that a
required-message rule could not satisfy at all — `unreachable!` routes
every message through `format_args!`, which is not const-callable, and
`panic!` is lint-banned. **What the population wants is not a message
rule.** It is row 0 asked at the sites where the answer might be yes:
the non-empty-by-construction sequences and the small-domain indices,
which are where a converted arm should have been no arm — thirteen of
them, enumerated as `SMELL-SCAN-2026-08.md`'s **D96**.

*Row 5's boundary (ratified in-chat 2026-08-29, at S-CERT's Q1):
`debug_assert` also serves the expensive check whose failure PROBABLY
indicates a bug.* Row 5 as written covers states that can only be a
bug; a debug assertion is additionally the right instrument for an
expensive re-derivation check where a failure probably indicates a
kernel bug but input-reachability cannot be excluded — a tripwire,
not a proof. The class's contract: (i) the assertion's absence never
changes shipped semantics — no typed behavior rides on one, so
release may compile them out (today `[profile.release]
debug-assertions = true` keeps them on everywhere; the eventual state
is debug/CI-only); (ii) an input-reachable failure that release must
handle still gets its row-1/2/3 disposition — the tripwire
supplements, never replaces it; (iii) each such assertion documents
its calibration in-file — the population measured and the margin
observed — so a firing one reads as evidence to investigate.
`mesh::walk`'s `closing_column` is the precedent, in both directions:
its firing is how #723's wrong certificate announced itself in a
debug build, and its recorded estimate being off by nine orders on
that input is what an uncalibrated ceiling costs.

*Row 1 absorbs the terminal indeterminates.* An `Indeterminate` whose
`MarginDiag` is `Value` (f64 margin in the ambiguity band) or an
`Enclosure` lying wholly inside a sliver band is a statement about the
input, and reaches the user through `COINCIDENCE_RECOURSE`. **But the
axis is curable-vs-terminal, not bug-vs-invalid**: `MarginDiag::Enclosure`'s
own docs record that a straddling `Enclosure` is generally *curable* by
subdivision, and `MarginDiag::Invalid` splits again (a `Trv`
domain-clamp may cure as the violating sub-box shrinks; a NaI never
does). Q1's subdivision driver is **not built** — every reference to it
is a doc comment — so today every indeterminate is terminal and row 1
is complete. **When that driver lands, a curable indeterminate must
unwind to it and must not be reported as invalid input.** This sentence
exists so that arrival does not reopen the question.

*Row 2 is a naming rule, not a new mechanism.* `Unsupported*` means
"valid input, the kernel has not built this yet" and nothing else —
which makes the frontier inventory grep-able. The convention is already
dominant across the tree); `AssemblyUnsupported`
was renamed to match (D2, PR #740 — into four variants that each name
the refused class, each carrying the offending entity where it has one
(`EntityId`, or the chain's own `EdgeKey`); the fourth reports the body's
solid and shell counts,
which is what its refusal is about). A macro (`not_implemented!`) was considered and
**rejected**: these refusals are reachable by valid user input and must
stay recoverable, so a panicking macro would convert a user-facing
frontier into a crash. Where a frontier branch genuinely cannot be
reached it is row 4, with a message.

*Row 3 is unchanged and is stated here only because it is neither a
typed error nor a panic:* poison flows through **values**, never
through decisions (Q1 residue, M0 close). `sup_norm_bound` returning
NaN on every poison path is the pattern.

*Rows 4 and 5 split on **re-derivation**, not on cost.* `unreachable!`
is for an invariant the code can simply *observe* — the Euler surgery's
failed-key arms, whose own comments already read "the lookups cannot
fail" (`euler.rs`, the W2c census below). `debug_assert` is for a check
that *re-derives*
the invariant: `assert_euler_postcondition` runs arena deltas plus a
full tier-1 validate, O(body). Cost correlates, but re-derivation is
the line that does not wobble.

*The headline bullet survives untouched.* "The kernel never panics on
any input" stays literally true: `unreachable!` is by construction not
input-reachable, which is exactly what the M1 soundness argument above
establishes.

*Boundary rule.* `pncad-py` re-types at the FFI edge — anything the
Python layer can trigger is validated into a typed error before the
kernel call, so an `unreachable!` never crosses into a
`PanicException`.

*Lint state, applied with this addendum:* `unreachable` is out of the
banned clippy family in both `Cargo.toml` and its hand-mirrored copy in
`crates/pncad-py/Cargo.toml` (kept in step by that crate's
`crate_lints_match_the_workspace_minus_unsafe_code` test, whose only
sanctioned deviation is `unsafe_code` — so the two move together).
`panic`, `todo` and `unimplemented` stay banned.

*Conversion work this licenses.* Opening the lint permitted the work;
it did not perform it. **W2c is done**, and W2c is narrower than
`crates/topo`: what is discharged is the **three-module census** — the
Euler surgery modules `euler.rs` / `euler_ring.rs` / `euler_kill.rs`,
which now discard nothing — not the crate. The
census re-derived to **58** sites and **all 58 are now row 4**: 56
converted in PR #720, each carrying its own per-site
not-input-reachable proof, and the last 2 — the shared write helper
`link_half_edges`' — once its two unproven callers gained the missing
plan-phase link check (`split_edge`'s `prev(he_minus)` and `kef`'s `prev(he)`;
each operator already proved the symmetric `next`). **The `kfmrh` pair
is not a third bucket**: two of the 56 became provable only because
that operator's plan phase gained *new* row-1 `StaleKey` checks on
`s2_data.faces` / `s2_data.solid` — those checks are added
preconditions, not discard sites converting to row 1, and the two
discards they license are among the 56. **Those last two
arms take a proven-live key rather than a precondition in prose**: a
shared helper does not know its caller, so the proof is the argument
type — `topo`'s `Live`, obtainable only through doors that perform the
lookup — and the arms announce a proof that outlived its key,
`#[track_caller]` so a panic reports the call site. **No site was row
5** — rows 4/5 split on re-derivation, and a failed key lookup is
observed rather than re-derived. The standard the conversion holds to,
and the reason it survives a corrupt body: **every converted key is
minted in the same call or proven live by a check in the same call,
never by the body's tier-1 validity** — which is a whole-body property
no single call establishes, and which would have been falsified across
roughly half the sites.

*The `crates/sweep/src/fillet` half is also done* (D2, PR #740).
`AssemblyUnsupported`'s **103** construction sites re-derived to **108**
— five refusals that conflated two of these rows behind one test split
in two — partitioned **41 row 2**, **49 row 1** and **18 row 4**. Row 2
is four variants that each name the class they refuse (chain, run-out,
stored geometry, body), plus the corner CONFIGURATION refusals routed
into the existing `FilletCornerUnsupported`; row 1 is
`BodyNotIntact`/`RepeatedEdge` (and `EmptyChain`, until D27 dissolved
it — below), and every payload that names an entity is
`topo::EntityId`, not a second spelling of it.

**The 18 is a bounded claim, and the bound is the interesting part.**
Every key an `unreachable!` there dereferences is minted by an operator
in that call, returned by a walk that succeeded in that call, or proven
present by a check in that call — and three sites were **made** provable
by adding those checks rather than converted on a proof borrowed from
one frame up. The other ~46 lookups stay row 1. **No demonstration
exists that any input reaches them** (an adversarial search reached none
of them and no panic — 1,842 requests at the shipped effort, 12,210 at
`CAD_FUZZ_EFFORT=10`, over a corpus of primitives, revolves, booleans,
transplants and the surgery's own output re-filleted), and equally **no demonstration
exists that none can**: the standard cannot discharge them locally,
because their keys arrive from outside the call. Converting on an
unproved negative is the direction the headline bullet forbids, so row 1
is the safe disposition on an open question rather than a settled
classification. The open question is **S14** — a public door that can
leave a body tier-1-invalid, and slotmap keys that resolve to *live but
wrong* entities rather than dangling.

*The state that produced row 0 is gone.* `FilletError::EmptyChain` was
neither row 1 nor row 4 and sat under row 1 failing that row's own
definition; D27 (PR #768) removed the representation rather than the
classification. **Rows 1–5
stand unamended** — nothing was added to the classification and nothing
in it was reclassified; what this case produced is **row 0** above, the
question that comes before them. The same unit retired the front-door invariants
the surgery was carrying as prose: `crates/sweep/src/fillet/admit.rs`
mints one value per admitted clause, and the helpers that used to
re-refuse a state their caller had already excluded now take the value
and have no branch to write. **Nothing there became an `unreachable!`**
— each refusal moved to the door that decides it rather than becoming a
panic.

*This is row 0's first application, and row 0 is the rule it produced.*
The disposition above is not special-cased to the fillet: it is what
row 0 says to do, and it is written into the table rather than left as
a story about one variant.

**What row 0 changes about S14, and what it deliberately does not.**
S14 asks whether the no-panic principle should be amended for a state
`topo::instance`'s graft genuinely produces — a mid-transplant refusal
leaving `dst` partially written, spent, and tier-1-invalid, which a
caller may keep. Under row 0 the first question about that state is no
longer *which row does it fall under* but **can
`graft_disjoint_all_keyed` be restructured so that a partially-written
destination is not representable?** — staging into a fresh body and
committing on success, which is the shape `merge_coplanar_faces`
already uses in this crate (`merge_faces.rs:472`, `let mut work =
self.clone()`, under its own *"Never a partial commit: each sub-stage
is tier-2-gated before adoption"*) and the shape D27 used. **That
reframes S14; it does not answer it.** Whether the restructuring is
affordable is precisely the "if possible" judgement above, and
**S14 stays open and stays Evan's** — #740 left 46 lookup sites typed
rather than converted because it is open, so anything that moves S14
moves them.

*The `crates/topo` sites outside W2c's three modules are done* (D21,
PR #773) — **the sites, not the class**, and the difference is the
part worth ratifying. The census re-derived to **17** under the stated
reading *a lookup whose `None` is discarded at a write in a mutation
phase*, and it found a **seventh** file the earlier floor of 14 did
not name: `merge_faces.rs`, whose two sites spelled the discard
`else { return Ok(()) }` under a comment that already said
*unreachable*. The disposition is **16 row 4 + 1 row 0** — the odd one
being `revert`'s edge loop, which carried no per-key value and so was
rewritten to walk the arena directly and look nothing up, the shape
this taxonomy should always prefer to a converted arm. Every converted
key is minted in the same call or proven live by a check in the same
call, **never** by tier-1 validity, and every arm was demonstrated
live by poisoning its key and watching it fire with its own message.

*Three things that closure does NOT cover, stated so no reader infers
them.* **(a)** One `crates/topo` site cannot meet the standard and is
deliberately unconverted: `merge_coplanar_faces`' ring re-homing reads
its face key out of a loop's back-pointer, so its disposition is a
typed error rather than a panic — `SMELL-SCAN-2026-08.md`'s **D88**,
and the named exception to the enumeration in `topo::euler`'s module
docs. **(b)** The **class is not confined to `crates/topo`**, and the
crate clause is a scope of work rather than a claim about the class:
verified instances live in `step-import`, `bvh` and `profile`, one of
them five lines from a panicking `Index` on the same key — **D94**.
**(c)** `boolean/combine.rs` answers one proof two ways — six
minted-in-call keys refuse `row 1` where two identical ones now
announce `row 4` — which is **D95**. And, outside `crates/topo`,
idiom 2's `MissingEntity` router defects.

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

**D9 addendum (ratified via PERF-PLAN's Q-P1, Evan's sign-off #49,
2026-07-21; folded in at the M3 exit sweep. PERF-PLAN itself stays
merged-and-advisory; this addendum is the contract.)**

*Deterministic parallelism — the project's two sanctioned idioms
(PERF-PLAN §2.2); every future use cites these instead of
re-deriving:*

1. **Indexed parallel map**: results written to slot *i* of a
   pre-sized buffer (indexed `par_iter().map().collect()`).
   Schedule-invariant by construction — combination is positional,
   not arithmetic. Bit-deterministic at any thread count.
2. **Fixed-shape reduction**: FP sums/mins are **never**
   `par_iter().reduce()` (rayon's reduction tree is
   schedule-dependent; FP non-associativity leaks the schedule into
   bits). Instead: idiom 1, then a *sequential* fold in arena order —
   or, if that fold profiles hot, a fixed-arity block tree (chunk
   size a named constant, combine order documented). Same bits every
   run, any thread count.

Targets in value order (advisory detail in PERF-PLAN): the M10
subdivision driver, per-face tessellation, certification sampling,
mass properties (the canonical idiom-2 example), independent M4 DAG
nodes. Euler-op sequences stay serial — shared arena mutation,
already cheap.

*GPU boundary (PERF-PLAN §3.3), ratified:*

| Work | Home | Why |
|---|---|---|
| Rendering, LOD, ID-buffer picking | GPU, GUI milestone | ratified direction (GUI-DESIGN); no kernel coupling |
| Preview (uncertified) surface evaluation | GPU-eligible, GUI milestone experiment | display lane; never re-enters kernel |
| Certified tessellation, export meshes | CPU forever* | export promise needs certified bounds |
| Booleans, splitting, SSI, predicates | CPU forever* | D9 + certification; GPU pre-filter not worth the audit |
| Euler ops, validators, arena surgery | CPU forever | pointer-chasing, serial by nature, already cheap |
| Interval lane / subdivision driver | CPU (rayon) | embarrassingly parallel on CPU already (PERF-PLAN §3.2) |

\* "forever" = for this project's plannable horizon; PERF-PLAN §3.2's
grounds (rounding control, f64, portability) are re-checkable facts,
and the table is revisited only if they change materially.

**Engineering conventions RATIFIED at the M4 exit sweep (the 8c PR,
#119, merged 2026-07-27 with Evan's sign-off — M4-LOG: "THE M4 EXIT
SWEEP IS RATIFIED", convention 2 sharpened at ratification to his
structural-shared-validator form; each earned by a concrete M4
incident):**

1. **Sentinel-free tagged encodings.** Internal byte/key encodings
   never use in-band magic values (sentinel indices, marker floats);
   any stream mixing kinds is TAGGED TOKENS — tag byte + typed
   payload — so collisions are unrepresentable by construction.
   Earned twice in one milestone from the same root (`float_bits`'
   in-band delimiters): #101's `usize::MAX` key alias and PR 6's
   NaN-marker alias + save-door blind spot; ruled structurally at
   the PR 6 fix pass (Evan: "deserves proper types") and landed as
   the tagged-token key-encoder retype (#112).
2. **Save/load validation is ONE shared validator, not two mirrored
   door sets** (sharpened at ratification per Evan: structural
   sharing beats a sweep — code that is literally the same cannot
   drift). Every direction-independent document check lives in a
   single validator invoked by BOTH doors: at save on the in-memory
   doc before bytes are written, at load after parse; a document
   that would refuse to load is therefore impossible to save by
   construction. The symmetry SWEEP survives only as the audit for
   the genuinely asymmetric residue (parse/position errors are
   load-only by nature; byte-level corruption has no save-side
   analogue). Earned: PR 6 review MAJ-1 — a NaN with all-ones bits
   walked past the save doors and produced an unloadable file; the
   fix-pass sweep then found two MORE save-side holes beyond the
   reported one (#112). Migration note DISCHARGED (M5 S4): both
   doors now invoke the ONE shared validator
   (`persist::check::validate_document` — float walk, joint walk,
   structural invariants); the wire keeps only the genuinely
   load-only residue (parse/position errors, the canonical-set
   rule), and the save-refuses-what-load-refuses closure is pinned
   at the unit level (a structurally invalid in-memory document
   refuses at save with the load door's own arm).
3. **Full-matrix watcher floors.** Any merge-gating checks watcher
   asserts a MINIMUM green-row count equal to the current full CI
   matrix, and the floor is bumped in the same PR that grows the
   matrix — a stale shorter matrix can never gate a merge.
   *Change-filter rider (2026-07-29, Evan's ask post-Actions-budget;
   made dependency-aware 2026-07-28):* CI carries a three-tier change
   filter, implemented once in `scripts/ci-filter.py` and called by
   both `ci.yml`'s filter job and `local-scripts/ci-local.sh`, so hosted and
   local gating cannot drift. Tier `docs` — only `*.md`/`memories/` —
   skips every build row and gates on the `docs-only` marker job.
   Tier `all` — any workspace-level file, which includes the root
   manifest, `Cargo.lock`, the toolchain file, `.cargo/`, `.github/`,
   `scripts/`, the excluded workspaces, the k-lint input data, ANY
   member `Cargo.toml` (feature unification is workspace-wide), and
   anything the allowlist does not recognise — runs the whole matrix
   unscoped. Tier `closure` — crate sources only — scopes the cargo
   rows to the changed members plus every member that transitively
   depends on them (dev-dependencies included), and runs each
   pipeline row iff its root package is in that closure. Filtering is
   never per-crate in the naive sense: the closure is what keeps
   partial green from becoming the trap this convention kills, and
   classification fails CLOSED — any uncertainty is tier `all`.
   Floors apply to CODE change sets, and to the rows a tier actually
   selects. `ci-local.sh` is filtered by default for equivalence with
   hosted, with `--full` forcing tier `all` (suspect environments,
   post-crash verification, torn caches, full-battery obligations).
   Earned:
   the #113 stale-10-row-green trap (branch predating new
   persistence rows showed green on the old matrix); floors then
   tracked the matrix 13 → 14 → 16 through #116/#118.

**Convention RATIFIED at the M5 exit sweep (Evan, PR #169 comment
5171303851, 2026-08-03: "the three amendments (two-tolerance
principle, equivariance, distance-only tesselation) sound good to me
also"):**

4. **Semantic equivariance where it is free — with the premise
   explicitly UNAUDITED.** Kernel constructions and selection rules
   should commute with rigid motions *and reflections* at the
   semantic level (in ℝ), unless equivariance is provably impossible
   for the case or costs something real. This concerns DESIGNED
   rules — no left-hand rules, no absolute-orientation tie-breaks —
   and explicitly NOT bitwise f64 equivariance, which D9's fixed
   evaluation orders already forgo. Rationale: user geometry has no
   preferred handedness, so a mirrored design should behave as the
   mirror of the original. How to apply: when specifying a
   selection/tie-break/ordering rule, prefer intrinsic quantities
   (arc lengths, distances, angles) over enumeration or construction
   order; where a candidate-swapping symmetry makes equivariance
   impossible, fall back deterministically and DOCUMENT the residual.
   Origin: Evan, 2026-07-30, during the S8 fillet-branch ruling —
   *"everything is equivariant right now, so maintain that if it's
   free (if that is indeed true)."* **The parenthetical is load-
   bearing and is carried into this convention: the "the kernel is
   currently equivariant" premise is UNVERIFIED. An audit is banked,
   not assumed. Do not cite the kernel as equivariant in docs or
   review without checking the claim at the site in question.**
   Precedent for the documented-residual escape: M5 S8's selection
   ladder, rung 3 — the first knowingly-designed residual.

## Layering

Each layer depends only on the layers below it.

| Crate | Contents |
|---|---|
| `geom-core` | Scalar trait (`f64`, intervals, duals), 2-D/3-D points/vectors/transforms (hand-rolled, small, fixed-dim — we control the scalar trait), robust predicates, root finding |
| `interval-transcendentals` | *(M5 PR 1, #127)* The `interval` feature's backend beneath `geom-core`: proven per-function libm error pads, MPFR-differential-certified. A separate workspace root on purpose (root `Cargo.toml`'s `exclude`), so its gmp-backed oracle dev-dependency never enters the kernel's graph |
| `bvh` | *(added M5 PR 8, C10)* Deterministic AABB tree: arena-order build, fixed split rule with total tie-breaks, conservative-superset contract — the tree prunes, exact predicates decide (D9). Deliberately BELOW the geometry crates (only `geom-core` under it) so SSI subdivision can consume it; certified box constructors live beside their invariants in `geom` |
| `geom` | Analytic + NURBS types, evaluators, closest-point, curve×curve and curve×surface intersection. Curves and surfaces are two modules of one crate, so what they share is stated once: the parameterization conventions and the totality/poison policy in the crate docs, the §6.1 projection constants and the azimuthal frame in interior modules |
| `geom-brep` | The B-rep geometry layer: D2's intensional edge descriptions, certified carrier caches, the dihedral classification predicate, Newell face equations, pcurve caches |
| `profile` | 2-D sketch profiles: the PATHS authoring algebra and the profile-program it records (PATHS-DESIGN, PROFILES-V2-DESIGN), lowering to the bulge-chain `Profile` and its trilean validation |
| `topo` | Arenas, entities, Euler operators, validation (watertightness, orientation, Euler characteristic); the boolean engine and its splitting/census machinery, which sit as sibling modules at the crate root rather than underneath `boolean` |
| `sweep` | Solids from validated profiles: extrude, revolve, loft/skin; fillets |
| `mesh` / `stl` | Tessellation (watertight triangle meshes from B-rep bodies); STL export (binary + ASCII) |
| `step-export` / `step-import` | STEP (AP214) analytic-subset export, and import of that subset — import is LIVE as of M7 (own-corpus byte-identical round-trip, FreeCAD foreign corpus, wild corpus) |
| `editor-core` | Headless document/editor layer AND the parametric layer: document-as-value (recipe + metadata), typed edit vocabulary (`DocEdit` + pure `apply`), parameter expressions, feature DAG evaluation, persistent naming, stable-reference/selection model, incremental evaluation service (preview/commit, epochs, cancelation). No rendering dependency — most of "the GUI project" is library work that ships and tests before a pixel exists. See `docs/GUI-DESIGN.md` |
| `quantity` | Typed quantities at the API boundary (D6): `Length`, `Angle`, and the unit constants the façade re-exports |
| `test-utils` | The workspace's shared fuzz/property harness (seed + effort dial), taken as a dev-dependency. ZERO dependencies by design — a leaf below every other crate, which is what lets any of them, and the excluded `interval-transcendentals` workspace, depend on it without inverting the layering |
| `pncad` / `pncad-py` | The authoring façade (LIBRARY-DESIGN U1 — one crate to depend on, a prelude, f64-first signatures) and its PyO3 bindings, which speak the document layer (L3) |
| `viewer` | Layer 3, the interaction layer over `editor-core` (GUI v1, closed 2026-08-28): `Camera`/`CameraOp` and `DocSession`/`SessionOp` as values with one `apply`/`perform` each, feature tree, property panel, selection, open/save, scene extraction — all renderer-free and headless-tested; the eframe/wgpu application lives behind the non-default `app` feature. Architecture: `docs/GUI-DESIGN.md` (G1 three-layer split; every operation the GUI performs — select, hide, free-move, camera — is itself API on `editor-core`/layer-3 state, testable with no renderer present) |

The API-first discipline falls out of this: every layer below `viewer` *is* the product,
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
- **M4** — Parametric model layer: parameter vector → feature DAG →
  solid; provenance-based naming; replay. STEP export. *(Complete
  2026-07-27.)* Standing design outcome
  stated here because it still binds: **production bit-identity
  coincidence checking is RETIRED** (Evan, #53; executed M4 PR 5,
  #102). The ratified mechanism is NAMING-DESIGN N6 recipe-source
  identity — `GeomSource`: same source ⇒ same bits by D9, converse
  deliberately unclaimed. `geom_core::bit_identity` is debug-only
  with an EMPTY production allowlist (CI tripwires stay armed; a new
  consumer must be allowlisted and carry a retirement-scheduled doc
  note; memo.rs retained on its bit-hashing non-consumer
  justification). Designed consequence, stated honestly: undeclared
  value-equal flush booleans refuse typed at the coincidence door —
  declared intent is the supported road.
- **M5** — NURBS depth (sweeps/lofts); first SSI marching;
  constant-radius fillets. Design record ratified:
  `docs/CURVED-DESIGN.md` (#85, 2026-07-24). *(Complete 2026-08-03;
  the done-state of record is `docs/M5-EXIT-WALK.md`; the
  shipped-unit list and the acceptance-shape/banked-openers
  narrative were relocated to that walk's appendix.)* Standing
  outcomes that still bind: **seven frontier units were BANKED by
  name with typed, pinned doors** — M6 then closed composition
  surgery, the SSI generic-`T` lift, and loft/sweep body assembly,
  and curved REST contact is M9. Still banked: the canal-surface
  blend, cyl×sphere germ chords, the NURBS extent lift, and arc-leg
  fillet sugar (#104). Acceptance shape (v) was recorded met
  piecewise at M5 and
  CLOSED at M6 unit 1 (the composed die is one tier-3 body; the M5
  pin flipped with its history). The #89 K-revisit was TAKEN at the
  M5 exit and the outcome is **#89 CLOSED, K = 10 permanent** (Evan,
  PR #169; K-REPORT M5 addendum — and see K-REPORT's M7 addendum for
  the fired-and-retired landing). The in-house
  `interval-transcendentals` crate was adopted as the kernel's
  interval backend at M5 PR 1 (#127; see the crate table).
- **M6** — the main-path completions: M5 shipped its curved kernel
  with seven frontiers banked by name, and M6 closed the main-path
  ones — the **SSI generic-`T` lift** (which gated the rest), the
  **loft/sweep body assembly** (owning **pcurve certification on the
  analytic charts**), and the **in-place edge-blend composition
  surgery** that makes M5's acceptance shape (v) one body instead of
  two — plus the fillet-selection vocabulary and the curved
  sense-flip tier gate. Design-only alongside them: the **census /
  declared-contact design doc** (`docs/CONTACT-DESIGN.md`), because
  curved REST contact is core kernel work whose design belongs with
  the main path even though its implementation does not. *(Complete
  2026-08-08; the done-state of record is `docs/M6-EXIT-WALK.md`.)*
- **M7** — STEP import as adoption (D7), **and nothing else**:
  analytic surface recognition, edge adoption, healing. Core kernel
  work that import happens to *want* belongs to M6, not here. It is
  the inverse problem of everything above it, and where the
  foreign-geometry corpus finally arrives (see #89's re-open trigger
  in `docs/K-REPORT.md`). *(Complete 2026-08-09; the done-state of
  record is `docs/M7-EXIT-WALK.md`. Import is LIVE — own-corpus
  byte-identical round-trip, the FreeCAD dialect, the wild corpus,
  NURBS faces.)*
- **M8** — the kernel residuals the demos raised: the Newell
  chart-frame re-anchor, the rational-carrier span meter, rational-
  patch-flux quadrature, and the `nurbs_iso_derive` Intersection
  arm. *(Complete 2026-08-15; the done-state of record is
  `docs/M8-EXIT-WALK.md`.)*
- **M9** — the declared-contact join lane: CONTACT-DESIGN C7, the
  at-rest census door ASSEMBLY-DESIGN A5 that binds to it, and the
  lily FULL rebuild. *(Complete 2026-08-27; the done-state of record
  is `docs/M9-EXIT-WALK.md`.)*
- **M10 (OPEN — running as the M10 program, `docs/M10-PLAN.md` /
  `docs/M10-LOG.md`)** — Error-propagation MVP: distributions over
  parameters; dual-number sensitivities of measurements (tolerance
  stackups); interval-based self-intersection / minimum-clearance
  checks over the parameter box. Design record:
  `docs/ERROR-DESIGN.md`. The sketch solver is NOT in the M10
  slate (plan Q1): it re-opens as its own design pass when
  constraint-driven sketches have a consumer.
  The carried Dual question (*what does a `Dual` actually have to
  do*) is **ANSWERED — ratified as `docs/DUAL-DESIGN.md` DL1–DL6
  (#1146, 2026-08-29)**: a Dual is tangent transport and never
  certifies; D1's *"at least for now"* hedge is closed.
- **The usability program** — see
  [Beyond the kernel](#beyond-the-kernel-the-usability-gap) below.
  Its library half is designed and RATIFIED as
  `docs/LIBRARY-DESIGN.md` and is RUNNING (`docs/LIB-LOG.md`), at
  Evan's per-unit discretion. Its GUI half ran as the v1 GUI program
  (`docs/GUI-PLAN.md`, RATIFIED 2026-08-27): units GUI-0…GUI-4 are
  merged and `docs/GUI-EXIT-WALK.md` is the proposed exit walk.
  Licensing-hygiene work with
  no usability payoff is deliberately *not* sequenced here — it is
  [Tabled](#tabled-far-future) until a trigger pulls it forward.
- **Assemblies** — Band 3, designed as `docs/ASSEMBLY-DESIGN.md`
  and executed by the ASM program. *(CLOSED at v1 scope 2026-08-23; the
  done-state of record is `docs/ASM-EXIT-WALK.md`. Banked
  successors: ASM-XSPLIT and #945.)*
- **Modeling-verb breadth** — the missing modeling verbs whose
  prerequisites are already ratified, registered in
  `docs/KERNEL-VERBS.md` and RUNNING as the VERBS program
  (`docs/VERBS-PLAN.md` / `docs/VERBS-LOG.md`), concurrently with
  the above.
- **Edge-description unification** — the #427 pcurve migration
  ratified as PCURVE-UNIFY-DESIGN U2, RUNNING as the PCURVE program
  (`docs/PCURVE-PLAN.md` / `docs/PCURVE-LOG.md`) since M9's close.

## Beyond the kernel: the usability gap

*(Added 2026-07-19, from the usability-scoping conversation with Evan.
This is a **scoping section, not a milestone plan** — it names the
work between "the M0–M10 kernel exists" and "a person can actually use
this," so that none of it gets invented ad hoc or discovered late.
Items marked **(design-now)** are cheap at design time and expensive
to retrofit; each gets folded into the existing plans rather than
waiting for a usability milestone. Several items below need their own
design documents with D1–D9 rigor before they are plannable —
flagged individually.)*

**Sequencing stance (agreed 2026-07-19, DISCHARGED 2026-08-27):
"usable as a library" ships before any GUI work begins.** The kernel
has parametric models, mass properties, and STEP in both directions;
adding language bindings (Python — the CadQuery/build123d audience),
documentation, and feature breadth yields a genuinely usable
code-first tool without waiting on an interactive application. The GUI is a separate
layer and effectively a second project of comparable size to the
kernel (Fornjot's postmortem and Zoo's app-team scale are the
evidence); its architecture lives in **`docs/GUI-DESIGN.md`**: the
G1 three-layer split (kernel / headless `editor-core` / interaction)
and GQ1–GQ5 are ratified — GQ1's mechanism subsequently ratified in
full as `docs/SOLVER-DESIGN.md` (#79), the selection-stability/
naming doc as `docs/NAMING-DESIGN.md` (#74) — with GQ6 and a
SLIMMED GQ7 deferred to GUI time (GQ7's selection-filter,
heterogeneous-set and vanishing-entity clauses were re-homed to
`docs/SELECT-DESIGN.md` at #286; what stays deferred is multi-select
UX and filter presentation), save GQ6's toolkit row: its mandated
toolkit/viewport/picking/wasm re-survey was refreshed 2026-08-16 in
`docs/GQ6-RESURVEY.md`, and on it the toolkit is ratified as
**egui** (the iced fallback closed unexercised at the GUI v1 exit
walk). The survey's viewport and picking rows were recommendations
and GUI-0…GUI-2 decided them in the building — §3's ID-buffer/ray
roles came out INVERTED, the ray path authoritative and the id pass
advisory (`docs/GUI-LOG.md`). Both layers below the pixels are real —
`editor-core` and now `viewer` ship, and GUI-DESIGN's freshness note
carries the verified shipped-vs-absent inventory. (GQ4's
assemblies-are-recipes-of-the-same-formalism commitment is restated
at Band 3, where it binds.) **The library program itself is designed
and ratified: `docs/LIBRARY-DESIGN.md`** — OPEN, currently resting
with no active lanes; the dispatchable column is the `LIB-LOG.md`
tail (L1–L7
— façade, document-layer Python bindings, v2-fronted PATHS, the
authoring-ergonomics unit ladder; per its LQ5 ruling its units run
in parallel with kernel milestones where footprints are
independent).

### Band 1 — kernel-side services an interactive client requires

The "any GUI is a thin client" claim (Vision) is true only if the
kernel exports these. None are research; all are load-bearing.
*(Status 2026-08-28, verified against the code: most of this band
SHIPPED with `editor-core` (M4) and the milestones since; each item
below records what shipped and what remains.)*

- **Incremental recompute — SHIPPED** (M4, `editor-core::eval`):
  memoized per-node evaluation keyed on 128-bit content/naming keys
  (op kind, structural params, evaluated expression bits, upstream
  keys, ambient ε/K, witness), evaluation epochs, deterministic
  level-parallel scheduling; a targeted mid-DAG edit recomputes only
  its downstream cone (pinned: 2 recomputed / 75 reused on the
  77-node corpus doc). D9 determinism is what makes the memo keys
  well-defined, as designed. Remaining: partial re-tessellation, and
  a resident cache service — today the memo is the caller-threaded
  prior `Evaluation`.
- **Picking back-references — SHIPPED end to end.** Tessellation
  output carries per-patch source-`Face` keys and per-polyline
  source-`Edge` keys (M2 PR 6, `mesh::FacePatch`/`BoundaryPolyline`),
  and `editor-core::resolve::pick::pick_face` is the G1 `ray →
  StableName` service: `bvh::Bvh::ray` over per-triangle boxes, exact
  ray/triangle tests in plain `f64`, a total documented tie-break,
  then the `resolve::hit` inversion. `NodePick` is the typed door
  that pairs a mesh with its node by construction. The GUI is a
  consumer (`viewer::pick`), with a GPU id pass beside it as the
  advisory cross-check.
- **Cancelation and progress — cancelation SHIPPED, progress
  remains.** `CancelToken` yields between nodes/levels; a canceled
  run returns the completed prefix as a typed outcome. Remaining:
  progress reporting (nothing exists), and in-op yield points — the
  granularity is whole-node, so a long single boolean or fillet is
  still uninterruptible.
- **Selection stability across edits — SHIPPED** — the user face of
  D5/M4's persistent naming, and the single most usability-
  determining piece of parametric CAD. Design ratified as
  `docs/NAMING-DESIGN.md` (#74; N1–N7 — names are derivation paths
  resolved by a replay-emitted table, no matching heuristics);
  shipped in `editor-core` as ONE `StableName` type used by both
  recipe references and selections (the naming problem solved once,
  per G1), with resolution, the diagnosis ladder, tombstones, and
  `Rebind` with suggestion affordances (suggestions are offers,
  never auto-repair). Founding pillar ratified 2026-07-19: naming is
  localized to reified predicate flips (see Banked principles
  below).
- **Appearance attributes — SHIPPED as contracted** (#92): per-face/
  body display attributes live in the document layer keyed by stable
  names — never arena keys — survive recompute via post-pass
  resolution against the evaluation's name tables, and report losses
  loudly (N3/N5 semantics) instead of dropping silently;
  appearance-only edits recompute zero nodes.

### Band 2 — the interactive application (a second, kernel-sized project)

Named here so its cost is never underestimated. Architecture is
ratified (`docs/GUI-DESIGN.md` G1–G5) and v1 is built:
`docs/GUI-PLAN.md` (RATIFIED 2026-08-27), units GUI-0…GUI-4 merged,
done-state of record `docs/GUI-EXIT-WALK.md`. What v1 delivered
against the bullets below is partial and is stated there; the items
keep their full-size framing because that is what they still cost.

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
  and brutal in a GUI if presented raw; `CertifyError::ResidualExceeded
  { check, sample }` must become "this fillet fails *here*" with the
  entity highlighted. The typed-error discipline is what makes this
  *possible*; the presentation layer is real work.
- **Direct manipulation** (drag a face → parameter change) is an
  inverse problem on top of everything above; optional for v1 except
  dragged sketch dimensions, which users assume.

### Band 3 — the subsystems beyond the kernel proper

- **Assemblies — DESIGNED AND SHIPPED at v1 scope.** Multi-part documents,
  mates (a rigid-body-DOF constraint problem, distinct from the 2-D
  sketch solver), cross-document references, interference checks
  (the latter falls out of M3 booleans / M10 clearance). Even
  hobbyist use wants this. Architecture ratified 2026-07-19 as
  GUI-DESIGN GQ4 and designed in full as `docs/ASSEMBLY-DESIGN.md`
  (A1–A13): an assembly document is a recipe DAG of the same
  formalism — instantiate-part (via the doc-identity × local-ref
  wrapper), mates, and patterns are ordinary feature nodes, so the
  editor and solver machinery (incl. mate witnesses per GQ1)
  transfers unchanged; binding is pinned-with-explicit-update, the
  Cargo.lock model. Implementation ran as its own program and CLOSED
  at v1 scope 2026-08-23; the done-state of record is
  `docs/ASM-EXIT-WALK.md`.
- **Engineering drawings.** Dimensioned 2-D drawings require
  projection plus **hidden-line removal**; HLR on curved B-reps is
  SSI-grade (silhouette curves) and belongs on the difficulty
  ranking near fillets. Explicit near-term dodge: export STEP, make
  drawings elsewhere.
- **Feature breadth.** The kernel has extrude/revolve/sweep/loft,
  booleans, constant-radius fillets (nine analytic support-pair
  arms), symmetric-setback chamfers on plane–plane supports,
  shell/hollow (sealed and opened), and linear/circular/explicit
  patterns. Daily use still assumes: variable-radius fillets, draft,
  hole features (counterbore/countersink/tapped), mirror (D8's
  structural parameters are the substrate), helixes, rib/text
  features. Individually small; the long tail
  dominates "why can't I model my part." The kernel-side view with
  dependencies is `docs/KERNEL-VERBS.md`; the program executing its
  scheduled rows is `docs/VERBS-PLAN.md` / `docs/VERBS-LOG.md`.
- **Interchange breadth**: 3MF (supersedes STL for printing), DXF
  in/out (profiles, drawings), OBJ. Each small; STEP remains the
  only hard one.

### Banked principles (ratified 2026-07-19, rounds 6–9 of the usability conversation)

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
- **Content-keyed cache transfer** *(key shape SHIPPED — M2 PR 6
  mesh back-references, editor-core's 128-bit content/naming keys;
  a finer-grained per-artifact transfer service remains future)*.
  D9 bit-determinism makes any derived
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
  day one — M10's error-propagation UI rides the same memoization /
  cancelation / per-node-result machinery as f64 rebuilds; no
  parallel path, no retrofit.
- **ε and persistence** *(rules for the first persisted document —
  SHIPPED in editor-core: the document carries its ε, `SetTolerance`
  is a recorded `DocEdit`, and the verdict-vector diff engine
  reports exactly which predicates flipped; the assembly
  ε-disagreement seam awaits assemblies)*:
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
- **Fillet/blend validity is reified predicates, not try-and-fail**
  *(pre-M5; shapes the M5 feature API)*. The industry's fillet
  misery is mostly validity discovered by construction failure;
  every classic failure is a margined predicate over the inputs —
  r vs. 1/κ_max of the support (self-intersecting blend), r vs.
  adjacent-face extent (face consumption), spine regularity, blend-
  corner configuration — stated in the feature definition. Payoffs:
  typed, diagnosable pre-construction errors; the predicates are Q1
  predicates, so M10 can certify **fillet validity over a parameter
  box** ("cannot break for r ∈ [2,5]") — a direct error-propagation
  payoff no commercial kernel offers; corner reconfigurations become
  enumerated predicate flips, extending the naming pillar to
  fillets automatically. Same principle applies to shell/offset.
- **SSI completeness is an interval obligation, not a marching
  property** *(pre-M5)*. Residual certification audits only *found*
  branches; the missed small loop is the classic silent disaster.
  Contract: **marching finds, subdivision certifies exhaustiveness**
  (interval exclusion proves each domain region intersection-free or
  accounted for); the outcome is "every branch found" or a typed
  failure, never silence. The cost knob slots into existing
  structure: certification is an at-rest/tier obligation; preview
  may march uncertified (parallel to preview's degraded chordal
  tolerance).
- **Non-manifold boolean results are typed errors** *(M3)*.
  Legitimate booleans can produce vertex- or edge-touching results,
  unrepresentable under D1 (the lower-dimensional sibling of the
  coincidence principle's face case). Ratified: typed error naming
  the touching entities. Silent splitting into separate manifold
  bodies is rejected as inexplicit (changes body count without the
  recipe saying so); any future split behavior is an explicit,
  ratified operation the user invokes, never a fallback.
  *Sharpened at the M3 exit sweep (F2, ratified — see the tier-3′
  entry under D1's validity tiers)*: "non-manifold" means
  **non-representable** (a single edge with >2 faces, a shared-entity
  wedge fan) — those stay typed errors. Touching via *distinct*
  entities is representable, is a typed success carrying its 3′
  declared-contact records, and validates at tier 3′.
- **The expression sublanguage is total and finite by charter**
  *(M4; the anti-OpenSCAD guardrail)*. No recursion, no unbounded
  iteration, no user-defined functions — anything Turing-ish lives
  in the host-language generator layer (D8's split). Keeps interval/
  dual replay trivial and schema versioning tractable; nearly
  impossible to claw back once one persisted model uses a loop.
- **Sketch DOF diagnosis is two named layers, never conflated**
  *(M10; bounds the ezpz boundary — numbers only)*. The **structural
  layer** (DOF counting, graph decomposition — exact, combinatorial,
  float-free, deterministic) diagnoses over/under-constraint; the
  known residue — generically-well-constrained but configuration-
  degenerate sketches — is a Jacobian-rank fact at the witness,
  caught by GQ1's bifurcation-margin predicate with its own honest
  vocabulary ("degenerate configuration" ≠ "over-constrained").
  "Solver didn't converge" is never reported as a diagnosis.
- **Persisted floats round-trip bit-exactly** *(first persisted
  file)*. Witnesses/parameters/caches are f64 under D9 replay;
  standard shortest-round-trip formatting (Ryu — Rust serde default)
  satisfies this for finite values; NaN/inf policy explicit (JSON
  has neither); lossy formatters banned; enforced by a
  save/load/replay-identity test in CI.
- **Flags banked for later milestones**: mate solving at assemblies
  needs witnesses/interval contraction on SE(3), not ℝⁿ — budget
  for it, don't assume the sketch machinery drops in; recipe-level
  provenance must carry **pattern indices** explicitly so references
  into indexed families never degrade to positional guessing (naming
  doc requirement); the Band 4 model corpus comes online **at M4**,
  not with the GUI — rebuild latency is an architectural property
  and must be measured while the architecture is still cheap to
  change *(landed as scheduled: editor-core carries a shape corpus
  plus latency and determinism suites)*.

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

- *(The list is currently empty. Its one entry — in-house rigorous
  interval transcendentals — was done early at M5 PR 1, #127; the
  crate table has what shipped.)*

## Open questions

### Q1: Scalar genericity — **settled in full** (direction and all residue at M0 close, 2026-07-16; K CLOSED at #89). Retained here as the ratified record.

Settled direction — **reified trilean predicates + a subdivision driver; no
persisted decision log**:

- Evaluation code (evaluators, derivatives, transforms, measurements) is
  fully generic over a `Real` trait we define. Instantiations: `f64`,
  `Interval` (the in-house `interval-transcendentals` backend, behind
  the `interval` feature),
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
  even under exact arithmetic, K = 10 (permanent ratified default, #89
  closed); K is a policy dial —
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
- **Interval scalar** (PR #7; the M5 PR 1 backend swap preserved this
  contract verbatim): the *decoration
  as the poison channel* (`decoration < Def ⇒ Indeterminate(Invalid)` —
  silent domain clamps never decide); `Bounds` certification trait with
  poison-visible NaN brackets for empty AND NaI (failing certification
  outranks 1788 representational honesty) — *`Bounds` was split at #643:
  it now means only "carries a bracket", and the certification half is
  `CertifiedEnclosure`, which is what D1 (2026-08-19) leaves refusing a
  `Dual`*; tight `pown` powi override
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
- **K's numeric value: CLOSED — K = 10 is the permanent ratified
  default (#89 closed, PR #169; evidence trail in docs/K-REPORT.md
  and its milestone addenda).** K remains a policy dial, not a
  correctness parameter, and (Evan, #41, 2026-07-20) is ε-style
  per-run configuration (`Tolerance.k`, env-overridable, one value
  per run, never changed mid-run) — expected to join ε under the
  banked change-ε/`SetTolerance` principle (per-model persisted,
  recorded change op) at the document layer.

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

Leading answer: adopt **ezpz** at M10, with "roll our own LM solver on
`levenberg-marquardt`/`faer` using ISOtope's math as tutorial" as the
fallback if ezpz's product-driven roadmap diverges from our needs.

### Q4: Units and model scale — **resolved**, folded into D4 (¶4) and D6.

### Q5: Depend on, vendor, or merely study `curvo` for NURBS algorithms — **resolved (M5 audit, 2026-07-27)**

**Study + dev-dependency test oracle; vendoring REJECTED by the M5
S3 audit** (`docs/CURVO-AUDIT.md`, curvo @ 47d19d5, 2026-06-25).
The standing rationale held and the audit closed the vendoring
half: the core invariants (certified residuals, trilean predicates,
generic `T`, no hidden tolerance decisions) live *in the
algorithms*, and in every candidate routine the invariant-relevant
surface (equality/acceptance gates, scalar trait, termination,
error reporting) is exactly the part a retrofit would rewrite —
while the two biggest hoped-for targets (an A9.10 fitting stack,
SSI) do not exist in curvo at all (its `marching` module is an
empty placeholder). Oracle scope, pinned at the audited commit:
evaluation/derivatives/basis/degree-elevation/interpolation only;
not bit-exact (std-routed math), not SSI/booleans (absent —
opencascade-rs/truck remain those oracles). Contrast ezpz, which
sits *upstream* of the certified core (its output is just numbers
that then pass through our construction and checks), so
arm's-length dependency stays principled.

### Q6: Recipe representation — **resolved**, promoted to D8.

### Q7: Determinism policy — **resolved**, promoted to D9.

### Q8: Definitional vs. approximating surfaces — **resolved**, ratified in full as `docs/OFFSET-DESIGN.md` (O1–O6, #907).

### Q9: Project license and name

License **resolved**: dual MIT OR Apache-2.0. Name: still pending —
placeholder workspace acceptable; pre-publish renames are cheap. The
rename is one entry on the **Before publishing** list below; the others
are not name questions and are not filed here.

### Before publishing (listed so they don't get lost)

Not a design question — the set of things that are deliberately in a
pre-publication state and have to be put back before the project ships.
The list exists because each entry is individually invisible: nothing
goes red when the project publishes with one of them still in the
shipped state.

- **Roll the version numbers.** No member manifest carries a `version`
  field today, so every crate is cargo's default `0.0.0`, and
  `[workspace.package]` says `publish = false` — *"nothing is
  publishable until the project has its name (Q9)"*. Publishing means
  setting real versions and dropping that line; **rolling them back is
  what un-does a premature publish**, and the next entry rides along
  with it.
- **Turn release debug-assertions back off.** The root `Cargo.toml`'s
  `[profile.release]` sets `debug-assertions = true`, so a release build
  runs every `debug_assert` in the workspace — the D2-addendum row-5
  postconditions, which cargo's release default would compile out. That
  is the right posture for a kernel nobody depends on yet: D9's converse
  half says a bug state must panic *as early as it is detectable*, and a
  row-5 assert meeting a real part is information nothing else produces.
  **Deleting the stanza is a real reduction in what a release build
  checks, so it is a decision to take at publish rather than a chore to
  tick off** — `SMELL-SCAN-2026-08.md`'s **S65** is the worked example
  (the #678 watertightness backstop, ruled row 5 in **#884**: the
  `debug_assert` is the settled mechanism, and only its release REACH
  was ever in question — which is exactly what this stanza sets).
- **The name (Q9).** Above.

### Deferred to their milestones (listed so they don't get lost)

Vertex-geometry taxonomy (M3, when intersections exist); profile/sketch
input format (M2); body-level
serialization beyond the recipe (post-STEP-export). *(Discharged:
the ambiguity constant K's numeric value — CLOSED, K = 10 permanent,
#89/docs/K-REPORT.md; εₐ was eliminated by the D4 ¶1 revision of
2026-07-16 — angular thresholds are derived per predicate.)* *(Discharged at
M1: orientation/sense conventions and the validator's concrete
invariant checklist — both ratified into D1.)*

## Crate landscape (surveyed 2026-07)

Since the kernel itself is greenfield, dependencies are for the *substrate*,
not the modeling core. Candidates, all verified active unless noted:

| Area | Crate | License | Notes |
|---|---|---|---|
| ID arenas | `slotmap` | Zlib | **Adopted** (M0+). typed keys per entity kind, `SecondaryMap` for attributes — exactly the B-rep store shape |
| Persistent collections | `imbl` (or `rpds` for MIT-only) | MPL-2.0 / MIT | still a candidate — NOT yet a dependency (nothing has needed it through M9, LIB, ASM or GUI v1). `im` is unmaintained with an open soundness advisory — use the `imbl` fork if ever adopted |
| Interval arithmetic | `interval-transcendentals` (in-house, in-repo) | MIT/Apache | **Adopted as the kernel `T = Interval` backend at M5 PR 1 (#127, 2026-07-28)** — proven per-function libm error pads (4-ulp transcendental, 1-ulp arithmetic with exactness witnesses for sqrt/mul/div), MPFR-differential-certified (~4M cases via the optional `oracle-inari` dev feature), libm-only, D9-clean; the crate keeps its own workspace, kernel crates path-depend on it; its fast suites run gmp-free in the hosted `interval-backend` CI row. **The kernel is copyleft-free in every build configuration**: the M5 PR 1 swap removed `inari` and its gmp/MPFR LGPL-3.0+ stack from the tree entirely (Cargo.lock zero hits, dev-deps included), meeting issue #4's exit condition by removal; inari survives only as the optional differential oracle inside the excluded crate's own workspace. No target-cpu floor: mul_add witnesses are correctly-rounded regardless (Evan's #127 retroactive review, 2026-07-29) |
| Robust predicates | `robust` (georust) | MIT/Apache | not a DIRECT dependency and nothing of ours calls it, though it rides in transitively under `spade`; Shewchuk adaptive predicates, battle-tested via `geo`/`spade` |
| Dual numbers / forward AD | `num-dual` (dev-only) | MIT/Apache | **Demoted at M0** (PR #10): its transcendentals route through std, not libm, so it cannot satisfy the value-channel bit-identity contract — duals are one in-house generic `Dual<T>` (f64 and Interval from the same code); num-dual serves as a dev-dependency derivative oracle in tests |
| CDT / mesh refinement | `spade` | MIT/Apache | **Adopted** (M2, `mesh` crate). Delaunay + constrained + Ruppert refinement; meshing happens in UV space (our code). CDT insertion is quadratic for faces bounded by nested near-cocircular loops (a planar face with a hole) and near-linear otherwise; the cost is the legalization cascade, not point location (`mesh` §Performance, PERF-PLAN §2.1); exterior classification is OURS since #116 (even-odd flood fill), spade supplies the CDT only |
| Serialization | `serde` + `serde_json` | MIT/Apache | **Adopted at M4 PR 6 (#112)** for persistence schema v1; the `float_roundtrip` feature is LOAD-BEARING (last-ulp parse drift caught day one); kernel crates stay serde-free (`scripts/gates/kernel-serde-free.sh` parses every crate manifest and fails on a serde dependency entry; it checks the DEPENDENCY EDGE only, not transitive reach and not whether a kernel type is persisted, and `profile` is additionally sealed from inside by `profile/tests/seal.rs`). Where a kernel type must persist, its bytes are described above the boundary rather than by a mirror enum — ruled in `M9-1-SPEC.md:22` (CONTACT-DESIGN C4) and shipped in #552 |
| 2-D polygon booleans | `i_overlay` | MIT/Apache | candidate only — not a dependency; robust integer-snapping booleans (now inside georust `geo`); useful for trim-loop ops in UV |
| Display triangulation | `earcut` (georust) | MIT/Apache | candidate only — not a dependency; cheap ear-clipping for viz only |
| Sketch constraints | `ezpz` (Zoo) | MIT | see Q3 |
| STEP | `truck-stepio`/`ruststep` | Apache | **Evaluated at M4 (F6 spike, 2026-07-23): adopt nothing at runtime.** ruststep cannot write STEP at all; truck-stepio's writer ships unfixable conformance defects. Both are DEV-DEPENDENCY parse-back oracles for the in-house AP214 analytic-subset writer (`crates/step-export`, #88) |
| GUI toolkit | `egui`/`eframe`/`egui-wgpu` 0.36, `winit`, `egui_tiles`, `bytemuck`, `rfd` | MIT/Apache | **Adopted at GUI-0…GUI-4 (`crates/viewer`)**, every entry optional behind the crate's non-default `app` feature so the toolkit graph never enters a kernel PR's compile closure. egui is the GQ6-ratified toolkit; the iced fallback closed unexercised at the GUI v1 exit walk |
| Python bindings | `pyo3` (optional) | MIT/Apache | **Adopted at LIB-U9S (`crates/pncad-py`)**, `abi3-py38`, behind an optional feature so a kernel build never links Python |
| Hashing | `sha2` | MIT/Apache | **Adopted at ASM-1**: content pins are SHA-256 over the canonical semantic bytes (ASSEMBLY-DESIGN A4) — the pin IS version identity, so collision resistance is required; the in-process FNV `ContentKey` stays a deliberately separate, weaker vocabulary |
| OS randomness | `getrandom` | MIT/Apache | **Adopted at ASM-1** for interactively-authored document ids. Document layer (`pncad`) ONLY — `editor-core` stays deterministic by construction |
| NURBS oracle | `curvo` (dev-only, git-pinned) + `nalgebra` | MIT / Apache | **Adopted as a `geom` DEV-dependency** at Q5's resolution, pinned at the audited commit `47d19d5`; `nalgebra` rides in on curvo's own major. Oracle scope per Q5 / `docs/CURVO-AUDIT.md`; never a runtime dependency |

Reference-only (read, don't depend): **truck** (only living Rust B-rep
kernel; active on git but crates.io releases stale; booleans demo-grade),
**vcad** (new Apache-2.0 half-edge B-rep kernel with booleans/fillets,
too young to depend on but the most interesting recent effort),
**Fornjot** (archived June 2026 — see below), **opencascade-rs**
(the only production-grade-boolean route in Rust today; LGPL + C++ build
tax; useful as a *test oracle* for comparing our boolean results).
**curvo** left this list at Q5's resolution: it is a pinned `geom`
dev-dependency oracle rather than reference-only (audited at M5 — NO SSI
(empty placeholder module) and demo-grade 2-D clipping only; scope in
`docs/CURVO-AUDIT.md`).

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
M10's stackup design should treat kinks/subdifferentials),
`vida-martin-varady-1994-survey-of-blending-methods-parametric-surfaces.pdf`
(Computer-Aided Design 26(5) — the canonical blending survey, supplied
by Evan 2026-07-16; primary source for M5's fillet scope-boxing:
terminology/classification of blends, rolling-ball and trimline
methods, the open problems that motivated D2's `TangentIntersection`
treatment), and `hoffmann/` (Hoffmann,
*Geometric and Solid Modeling*, complete: front + chapters 1–7 + bib,
recovered via the Internet Archive — the Purdue page is gone).

- **Mäntylä, *An Introduction to Solid Modeling*** — the Euler-operator
  B-rep reference; the `topo` layer is essentially this book. One
  erratum on record: our reading notes carry a dated erratum for
  Program 11.6 — `lmev`'s printed `addhe` order (PLUS-half first)
  breaks both `he1 == he2` cases; MINUS-first is coherent — found by
  hand-trace during M1 PR 2 and verified against the scan.
- **Hoffmann, *Geometric and Solid Modeling*** —
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
