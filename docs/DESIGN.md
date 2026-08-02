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
  trip is bit-identical at every scalar backend. *This states the
  contract the bit establishes, not shipped behaviour: S10 lands the
  representation and the consumer threading only — the curved `revert`
  writer lands in the follow-on unit, and until it does `revert` still
  refuses non-planes with its typed error.*
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
3. **Tier 3 "geometric"** (implemented across M2 PRs 3–7): D4 ¶2
   residual certification, plus the **material wedge-angle
   predicate** — at every edge the material wedge ∈ (0, 2π), bounded
   away from the ends by the derived threshold θ = ε/r; wedge = π is
   the legal smooth-seam case (ratified in PR #15's conversation).
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
   coincidence census on the planar inventory; M6 interval clearance).
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
     `Plane` faces — the same F5 boundary the booleans that produce 3′
     bodies enforce; anything else refuses typed `CensusUnsupported`);
     five quadratic sweeps (vertex–vertex, vertex-on-edge,
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
   - Contact records are vertex-granularity; edge-on-face and
     coincident-edge *segments* are certified by reconstruction from
     their bounding vertex records (rule derived and pinned in
     `topo::census` module docs: between two backed bounds, two lines
     sharing two points are one line; a missing bounding record is
     `UndeclaredContact`, never inferred).
   - Census posture, stated honestly: **area-contact certification
     strength equals its vertex skeleton** (a face-on-face flush rest
     is certified via its corner/segment records, not an area test);
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

- **Sweeps emit single-shell bodies; voids are born only from booleans
  (Evan, 2026-07-20).** `FullRevolveHoles` (revolve's typed refusal of
  full-revolving holed profiles) is a standing rule, not a scope
  deferral: a full-revolved hole's swept walls touch nothing — the
  cavity boundary would be a disconnected interior shell, i.e. revolve
  emitting multi-shell bodies with internal voids, silently breaking
  machinery documented against the no-voids assumption a milestone
  before M3's boolean/void support exists. The front door is
  `revolve(outer) − revolve(hole-as-outer)` once M3 lands (the error
  text should point there); an M4 recipe-layer sugar node may wrap
  that composition — sugar above the kernel, never a new kernel
  emission mode. (`UnsupportedToroid` is likewise permanent: a D3
  ring-torus boundary — spindle tori have no representation — not a
  scope cut.)
- **The minimal sphere at rest is V2/E2/F2** (M2 PR 5): tier 2's
  valence-1 ban makes the "minimal" V2/E1/F1 sphere unrepresentable at
  rest — a one-band wire sweep leaves valence-1 poles, so axis-touching
  full revolves sweep two π-bands, giving poles valence 2 (the angle-0
  and angle-π meridians). A deliberate consequence of the tier
  definitions, not a defect.
- **Parameterization conventions (M2 PR 1, ratified-by-documentation;
  authoritative text in the geom-curves/geom-surfaces crate docs):**
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
  `LoopBuilder::fillet` constructor authors exact tangency by
  construction and declares it, with fit gating
  (`TangentJointOutOfRange` when a tangent point falls outside its
  leg); **same-carrier is identity, not tangency** — declared
  cocircular/collinear joints refuse with `same_carrier: true`
  (two-arc circles stay legal). Zero new ε: the per-junction
  classifier reuses the existing carrier predicates verbatim.
  Persistence keys the flags (`tangent_joints` in schema v1, #112).

**M3 structural conventions (ratified at the M3 exit sweep,
2026-07-23; forks resolved with Evan in #42, 2026-07-20/21):**

- **Planar-only booleans (F5).** M3 splits and booleans require
  all-planar boundaries; any non-`Plane` face refuses typed
  `CurvedBooleanUnsupported` — precise, honest, fail-loud. Curved
  defers to **M5 as a unit**: the dependency chain is fourfold and
  entirely M5-shaped — (a) intersection-locus representation (even a
  tilted plane×cylinder cut is an ellipse, outside `Line | Circle`),
  (b) general pcurves, (c) second-order sector classification (the
  `TangentIntersection` regime), (d) certified marching numerics (the SSI
  contract). The inverse commitment holds too: M3 built **no
  speculative curved-readiness abstraction** beyond the thin
  face-intersection interface (plane×plane closed-form today); M5
  refactors that boundary against real curved requirements rather
  than inheriting a guessed one.
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
  first legitimate voids, exactly as the voids-only-from-booleans
  ratification anticipated. **The sweeps-vs-voids invariant
  (ratified): sweeps produce genus, never voids; voids are
  boolean-born; the extrude/full-revolve hole asymmetry is an
  instance of the invariant, not an inconsistency** — extruded holes
  are cap-to-cap tunnels (one shell, genus); full-revolve holes would
  be closed inner shells (voids); partial revolve is extrude-shaped
  and already supports holes. A void's inner shell carries zero
  coincidences and is census-invisible at tier 3′ — a valid void, not
  an undetected contact.
- **The envelope (typed refusals on record, never silent gaps;
  M3 entries updated at the M4 8c exit sweep, 2026-07-27):**
  (i) **RETIRED (M4 PR 5, #102)** — the operand-internal-declaration
  gap: ops now consume declarations as recipe data threaded by name
  through op composition (F5/N-decisions, exactly the recorded M4
  fix direction); the closure corpus certifies that a reused 3′
  body's declared coincidences re-certify downstream. *(Historical
  entry: reusing a 3′ body as an operand 3′-refused
  `UndeclaredContact` because ops did not consume their operands'
  contact declarations.)* (ii) **the both-sided pinch split
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
`TangentIntersection`), not the need for an extensional fallback.

Validity of `Intersection` requires *transversality*: normals of S₁, S₂
linearly independent along the locus (equivalently `T_pS₁ + T_pS₂ = ℝ³`),
so the implicit function theorem makes S₁∩S₂ locally a 1-manifold. The
transversality margin (angle between normals) is a predicate-with-margin
(Q1) and governs the conditioning of every derived cache. Cases that fail
transversality get other variants: parameterization seams (`Seam`),
tangential contact such as fillet–support contact curves (a future
`TangentIntersection` variant — *named `TangencyLocus` in pre-M5
text; renamed as a ratified D2 sharpening per CURVED-DESIGN OQ7,
Evan 👍 #85 2026-07-24, applied at M5 PR 0: the variant mirrors
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
(`WitnessMidpoint`). This is a sharpening of the original text, not a
revision — the witness still selects the connected component (that
semantics activates with real SSI at M5); pinning it removes the aliasing
freedom the review exhibited (any point on the component certified,
including points encoding a wrong winding). Residual freedom is documented
where it is geometrically invisible (circles: joint whole-period
translation). Construction obligation on every op that mints an
`Intersection`: compute the witness as carrier(mid) with the certification
schedule's own association order.

**Prefer-intrinsic is tier-3-enforced (ratified 2026-07-19 with Evan;
landed in M2 PR 4's fix pass).** The prefer-intrinsic rule above is not
advisory: at rest, every *definitely-transverse* edge must carry
`Intersection` (`TransverseNotIntrinsic` otherwise); definitely-smooth
joins keep their conventional `MappedCurve` (the D2 conventional-split
story); escalated dihedrals and `Seam` edges are exempt — so ε-tightening
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
   tolerance), with the margin riding the error payload as data.
   Kernel SEMANTICS keep the distinction (ON-set classification,
   escalation, declared-verification are unchanged — this is
   message policy, not predicate policy).
   (ii) The existing error taxonomy gets a message-level rework
   sweep — scheduled as M5 side unit S6, dispatching to the first
   freed implementation lane (post the PR 4/PR 8 review cycles in
   flight at ratification): the audited candidate
   pairs are profile UndeclaredTangency/TangentialContact vs
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
   layer reads ε exactly once (pole vertex identification) and never
   for sizing; display-layer comparisons are deliberately not Q1
   predicates (none decide kernel topology).
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

Targets in value order (advisory detail in PERF-PLAN): the M6
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

**Engineering conventions PROPOSED at the M4 exit sweep
(PROPOSED-8c — awaiting Evan's sign-off on the 8c PR; NOT yet
ratified; each earned by a concrete M4 incident):**

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
   reported one (#112). Migration note: PR 6's shipped doors are
   sweep-style mirrors; consolidating them into the shared
   validator is banked M5-adjacent hygiene, not a re-open of #112.
3. **Full-matrix watcher floors.** Any merge-gating checks watcher
   asserts a MINIMUM green-row count equal to the current full CI
   matrix, and the floor is bumped in the same PR that grows the
   matrix — a stale shorter matrix can never gate a merge.
   *Change-filter rider (2026-07-29, Evan's ask post-Actions-budget;
   made dependency-aware 2026-07-28):* CI carries a three-tier change
   filter, implemented once in `scripts/ci-filter.py` and called by
   both `ci.yml`'s filter job and `scripts/ci-local.sh`, so hosted and
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
| `bvh` | *(added M5 PR 8, C10)* Deterministic AABB tree: arena-order build, fixed split rule with total tie-breaks, conservative-superset contract — the tree prunes, exact predicates decide (D9). Deliberately BELOW the geometry crates (only `geom-core` under it) so SSI subdivision can consume it; certified box constructors live beside their invariants in `geom-curves`/`geom-surfaces` |
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
  provenance-based naming; replay. STEP export. *(Done-state recorded at
  the 8c exit sweep, 2026-07-27 — shipped: `editor-core` recipe substrate
  + expression sublanguage (#81); scalar-generic evaluation service with
  memoized result DAG (#83); the naming stack end-to-end — StableName/
  RolePath + eager tables (#87), resolution + diff engine + Rebind
  (#96), GeomSource + Declare threading + the #95 recursive naming key
  (#102); in-house AP214 STEP export with FreeCAD acceptance (#88, #94);
  StableName-keyed appearance (#92); persistence schema v1 — snapshot +
  edit log, bit-exact, **frozen** (#112); declared-tangency discipline
  (#109); join-stage seam-region repairs closing a silently-wrong-volume
  bug (#108, #113) and watertight CDT tessellation (#116); the Band 4
  corpus + rebuild-latency reporting lane (#118); K-telemetry + large-K
  lint in review (8b) at the time of writing. Fork outcomes F1–F8:
  see "M4 fork outcomes" below.)* The naming layer also
  **retires production bit-identity coincidence checking** (Evan, #53,
  2026-07-21; M3 PR 4 / #57/#58): once surfaces carry global identity,
  the "declared" coincidence rung (M3's bit-fingerprint comparison of
  descriptions — `merge_coplanar_faces`' declared rung, PR 4's
  `oriented_plane_eq` via the one sanctioned `Real`-level seam,
  `geom_core::bit_identity`) becomes a provenance-record lookup; the bit
  comparison leaves production entirely, surviving at most as a debug
  assertion that the records and the bits agree. Until then the CI
  bit-identity tripwires keep **every consumer of the channel
  acknowledged**: a new consumer must be allowlisted in CI and carry its
  own retirement-scheduled doc note, and the type-punning plumbing stays
  confined to the single `bit_identity` seam. The retirement
  *mechanism* is now ratified (NAMING-DESIGN N6, #74, 2026-07-23):
  `GeomSource` syntactic recipe-source identity — same source ⇒ same
  bits by D9, converse deliberately unclaimed. **Retirement EXECUTED
  (M4 PR 5, #102, 2026-07-25): `bit_identity` is debug-only with an
  EMPTY production allowlist** (memo.rs retained on its bit-hashing
  non-consumer justification; tripwires stay armed); the designed
  consequence, stated honestly (PR 5 review R2): undeclared value-equal
  flush booleans now refuse typed at the coincidence door — the M3 bit
  rung was doing real, now-forbidden work; the whole corpus and the
  demos migrated to declared intent.
- **M5** — NURBS depth (sweeps/lofts); first SSI marching; constant-radius
  fillets. Design record ratified: `docs/CURVED-DESIGN.md` (#85,
  2026-07-24). Banked M5 openers from the M4 exit (8c, 2026-07-27):
  **curved STEP subset** (the export lane is planar-only until M5 —
  curved stops refuse typed, narrated in the demo tour); **arc-leg
  fillet sugar** (#101 R4 scoped `LoopBuilder::fillet` to line/line
  corners; arc-leg is the noted follow-up, see #104); **REST-contact
  join lane** (the crosslap mate is a pure rest contact — M3 envelope
  frontier, `crosslap_rest.rs` pins both doors; banked at #102 R7);
  **#89 K-revisit at the M5 exit**, now with its baseline: 8b's
  K-probe over corpus + demos (≈2.56M samples/ε-row) is SHARPLY
  BIMODAL — zero mode ≤ 5.33e-15, definite floor 1.689e-3, a
  12-decade empty gap, zero in-band anywhere ⇒ K = 10 is unpressured
  on the analytic kernel; **interval-crate adoption decision** — the
  in-house `interval-transcendentals` crate (adoption GREEN-LIT, see crate table) exists as
  workspace-excluded tooling (#115); adopting it in the kernel's
  interval lane is an M5-PLAN ratified decision, not a default.
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

### M4 fork outcomes (F1–F8, ratified at the 8c exit sweep, 2026-07-27)

The forks were recorded and resolved at M4 ratification
(`docs/M4-PLAN.md`, #80); this is the outcome record — each fork:
decision, where it landed, notable deviations. Full trail:
`docs/M4-LOG.md`.

- **F1 (restrictive dimension lattice)** — landed as ratified in
  `editor-core`'s expression sublanguage (#81): {Length, Angle,
  Count, Scalar}, dimension-changing products/quotients typed
  refusals, same-dimension ratios refused in v1. No deviations.
- **F2 (result-DAG shape)** — landed F2-verbatim (#83):
  `Evaluation`/`NodeResult`/`NodeValue`, descendants-only poisoning,
  scalar-generic evaluator, epochs + cancelation in the signature.
  Notable accepted deviation (Evan, #81 rulings): **`Doc<P>`
  genericity** — the document type is generic over the profile
  payload rather than concrete.
- **F3 (persistence concretes)** — landed as schema v1 (#112):
  snapshot + edit log, leading integer schema version, explicit
  migration chain, floats shortest-round-trip, NaN/inf typed refusal
  at BOTH doors (save-side walls added at the review's symmetry
  sweep). Format choice (PR-spec latitude, REPORTED): **JSON via
  serde_json** — ryu floats + tooling; the `float_roundtrip` feature
  is load-bearing (caught real last-ulp parse drift day one).
  Metadata (Evan's #92 ask) landed as the **`MetaValue` tree after
  two D7 rounds with Evan** (final: MetaValue tree, serde-native
  boundary, v-field convention) rather than opaque bytes.
  **Schema v2 (M5 PR 10)**: the recipe vocabulary grew `Loft`/`Sweep`
  and the version bumped as a ratified CLEAN BREAK (Evan, #148) — no
  migration step was written, a v1 file refuses typed
  (`PersistError::SchemaTooOld`, naming the regenerate recourse), and
  the repo's own v1 golden was regenerated once. The kernel is
  unreleased and every file it has written replays from source, so
  live compatibility code would have been carried for nobody. The
  migration MECHANISM stays (an explicit, currently empty step
  table): D6.3's forward-only rule is unchanged, and the next
  non-breaking format change adds its step there.
- **F4 (v1 node vocabulary)** — landed as ratified (#81; Declare
  live end-to-end at #102); `tangent_joints` joined schema v1 before
  the freeze (#109 → #112). Revolved-hole sugar stayed deferred.
  **`Loft`/`Sweep` joined the vocabulary at M5 PR 10** as ORDINARY
  ops under the same rules (named slots, the structural/continuous
  divide, refs to existing nodes only) — the Q8 definitional posture
  is stated in rustdoc at both the node and the surface.
  Noted gap (demo REPORT, #98): Boolean-of-Pattern is not wireable
  in F4 — a possible future vocabulary item.
- **F5 (Declare threading)** — landed at #102: declarations are
  recipe data on the consuming node, resolved by name through
  operand tables; the M3 operand-internal-declaration envelope entry
  is retired (closure corpus certifies declared). **Verified-at-use
  semantics (ratified wording, PR 5 review F5)**: a false
  declaration that never meets geometry is a silent no-op;
  contradiction fires where the lie meets an edge. The designed
  narrowing (R2) is recorded under the M4 roadmap entry above.
- **F6 (STEP export)** — decided EARLY per Evan's amendment; spike
  outcome: **in-house AP214 analytic-subset writer, adopt nothing at
  runtime** (#88); ruststep/truck-stepio survive as dev-dependency
  parse-back oracles only. Tail of the story: FreeCAD acceptance
  discharged locally then hosted (#94); the review added a
  parse-based **signed-volume text oracle** closing the OCC-healing
  blind spot (OCC silently rectifies inverted shells); the STEP lane
  then became the demo RENDER path (#98) and the watertightness
  gate's second leg alongside admesh (#116).
- **F7 (expression AST + ExprPath)** — landed at #81: no
  conditionals in v1 (held throughout); ExprPath stable under edits
  to other expressions. Known caveat carried forward as designed:
  same-slot ancestor replacement silently re-points stale paths —
  documented at PR 1, made a binding caveat in the PR 5 spec.
- **F8 (milestone boundary)** — held: the persisted file IS in M4
  (schema v1 frozen, #112); the Band 4 corpus landed (#118, 9
  documents / 174 nodes, coverage asserted) with rebuild latency
  MEASURED AND REPORTED, not gated — PERF-PLAN stays advisory; the
  latency rows joined the hosted matrix as reporting.

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
Remaining pre-M4 design work: GQ1 mechanism details — the
selection-stability/naming design doc is done and ratified,
`docs/NAMING-DESIGN.md` #74, 2026-07-23).

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
  record" sentence is months of work. **Its design doc exists and is
  ratified: `docs/NAMING-DESIGN.md` (#74, 2026-07-23; N1–N7)** —
  names are derivation paths resolved by a replay-emitted table, no
  matching heuristics — meeting the explicit goal that our
  architecture (D5 birth provenance + D8 recipe node IDs + D9 replay)
  makes correct resolution *structurally* easy — as much "automatic"
  as the design can extract. Ratified 2026-07-19 (GUI-DESIGN.md G1): the
  GUI's selection type and the recipe's entity references are **the
  same type** (a stable name), so the naming problem is solved once,
  not twice. Founding pillar ratified 2026-07-19: naming is
  localized to reified predicate flips (see Banked principles
  below).
- **Appearance attributes (contract-now, artifact-at-M4).**
  Per-face/body display attributes (color, name, visibility) must
  live somewhere that survives recompute — which means they attach
  via the same stable-naming machinery, not arena keys (an
  arena-keyed container would be fake durability: per-lineage keys
  die on rebuild, and consumers would accumulate against the wrong
  name kind). The ratified contract (final, 2026-07-20): attributes
  attach **in the document layer (`editor-core`), keyed by stable
  names, from M4 — and nowhere, in any form, before that**. No M2
  placeholder either: the type's only correct home is a crate that
  doesn't exist until M4-era work, so an early artifact would sit in
  the wrong layer and model the mistake this contract prevents.

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
- **Fillet/blend validity is reified predicates, not try-and-fail**
  *(pre-M5; shapes the M5 feature API)*. The industry's fillet
  misery is mostly validity discovered by construction failure;
  every classic failure is a margined predicate over the inputs —
  r vs. 1/κ_max of the support (self-intersecting blend), r vs.
  adjacent-face extent (face consumption), spine regularity, blend-
  corner configuration — stated in the feature definition. Payoffs:
  typed, diagnosable pre-construction errors; the predicates are Q1
  predicates, so M6 can certify **fillet validity over a parameter
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
  *(M6; bounds the ezpz boundary — numbers only)*. The **structural
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
  roadmap's post-M7 note, 2026-07-19)*: **RETIRED FROM THIS LIST —
  DONE EARLY (M5 PR 1, #127, 2026-07-28).** The entry's whole program
  completed years ahead of its own sequencing stance: the crate was
  built as an M4 side-chain (#115), and the kernel switch landed as
  M5 PR 1 — inari and its LGPL stack are gone from the tree, not
  re-quarantined. Kept as a tombstone because the entry's original
  text explicitly warned against scheduling it ahead of user-facing
  work: what actually happened is that M5's curved certification
  made interval arithmetic load-bearing on the default path
  (CURVED-DESIGN C9/T2), which converted the licensing hygiene into
  user-facing infrastructure — the sequencing stance was right and
  the trigger it waited for arrived.

## Open questions

### Q1: Scalar genericity (direction settled 2026-07-15)

Settled direction — **reified trilean predicates + a subdivision driver; no
persisted decision log**:

- Evaluation code (evaluators, derivatives, transforms, measurements) is
  fully generic over a `Real` trait we define. Instantiations: `f64`,
  `Interval` (in-house `interval-transcendentals` backend since M5 PR 1 — inari retired; behind the `interval` feature),
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
- **Interval scalar** (PR #7; backend swapped to interval-transcendentals at M5 PR 1, contract preserved): originally inari `DecInterval` with the *decoration
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
- **K's numeric value: resolved at M2 exit (docs/K-REPORT.md; the M0
  carry closed).** The M2 multi-ε telemetry (unified recorder in
  `geom_core::k_stats`, PR 7; 13k+ samples/row at ε ∈ {1e-6, 1e-9,
  1e-12} across 63 named predicates over the full acceptance pipeline)
  found margin distributions extremely bimodal — zero-side |m| at
  rounding scale (≤1e-15), definite-side |m| ≥ 10⁴·ε — with zero
  escalation-band landings; counterfactually K ∈ {3, 10, 30, 100} are
  decision-equivalent on this corpus. **K = 10 stays the default**, and
  (Evan, #41, 2026-07-20) K is now ε-style per-run configuration
  (`Tolerance.k`, env-overridable, one value per run, never changed
  mid-run) rather than a compile-time constant — expected to join ε
  under the banked change-ε/`SetTolerance` principle (per-model
  persisted, recorded change op) at the document layer. Scope honesty:
  a native-construction corpus is well-conditioned by design; the
  discriminating K evidence is expected from D7 import adoption and
  M3's boolean/SSI predicates, and the recommendation is explicitly
  revisitable then (a policy dial, not a correctness parameter).

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
| ID arenas | `slotmap` | Zlib | **Adopted** (M0+). typed keys per entity kind, `SecondaryMap` for attributes — exactly the B-rep store shape |
| Persistent collections | `imbl` (or `rpds` for MIT-only) | MPL-2.0 / MIT | still a candidate — NOT yet a dependency (nothing has needed it through M4). `im` is unmaintained with an open soundness advisory — use the `imbl` fork if ever adopted |
| Interval arithmetic | `interval-transcendentals` (in-house, in-repo) | MIT/Apache | **Adopted as the kernel `T = Interval` backend at M5 PR 1 (#127, 2026-07-28)** — proven per-function libm error pads (4-ulp transcendental, 1-ulp arithmetic with exactness witnesses for sqrt/mul/div), MPFR-differential-certified (~4M cases via the optional `oracle-inari` dev feature), libm-only, D9-clean; the crate keeps its own workspace, kernel crates path-depend on it; its fast suites run gmp-free in the hosted `interval-backend` CI row. **History**: `inari` was the M0-M4 backend (issue #4) with its gmp/MPFR LGPL-3.0+ transitive deps quarantined behind the `interval` cargo feature; the M5 PR 1 swap RETIRED inari from the tree entirely (Cargo.lock zero hits, dev-deps included), so **the kernel is copyleft-free in every build configuration and issue #4's exit condition is met by removal** — inari survives only as the optional differential oracle inside the excluded crate's own workspace. The historical AVX+FMA target-cpu floor was DROPPED post-swap (2026-07-29, Evan's #127 retroactive review — no correctness need remains; mul_add witnesses are correctly-rounded regardless) |
| Robust predicates | `robust` (georust) | MIT/Apache | candidate only — not a dependency; Shewchuk adaptive predicates, battle-tested via `geo`/`spade` |
| Dual numbers / forward AD | `num-dual` (dev-only) | MIT/Apache | **Demoted at M0** (PR #10): its transcendentals route through std, not libm, so it cannot satisfy the value-channel bit-identity contract — duals are one in-house generic `Dual<T>` (f64 and Interval from the same code); num-dual serves as a dev-dependency derivative oracle in tests |
| CDT / mesh refinement | `spade` | MIT/Apache | **Adopted** (M2, `mesh` crate). Delaunay + constrained + Ruppert refinement; meshing happens in UV space (our code). Sequential point-location insertion is the measured tessellation bottleneck (PERF-PLAN §2); exterior classification is OURS since #116 (even-odd flood fill), spade supplies the CDT only |
| Serialization | `serde` + `serde_json` | MIT/Apache | **Adopted at M4 PR 6 (#112)** for persistence schema v1; the `float_roundtrip` feature is LOAD-BEARING (last-ulp parse drift caught day one); kernel crates stay serde-free (layering enforced by CI grep) |
| 2-D polygon booleans | `i_overlay` | MIT/Apache | candidate only — not a dependency; robust integer-snapping booleans (now inside georust `geo`); useful for trim-loop ops in UV |
| Display triangulation | `earcut` (georust) | MIT/Apache | candidate only — not a dependency; cheap ear-clipping for viz only |
| Sketch constraints | `ezpz` (Zoo) | MIT | see Q3 |
| STEP | `truck-stepio`/`ruststep` | Apache | **Evaluated at M4 (F6 spike, 2026-07-23): adopt nothing at runtime.** ruststep cannot write STEP at all; truck-stepio's writer ships unfixable conformance defects. Both are DEV-DEPENDENCY parse-back oracles for the in-house AP214 analytic-subset writer (`crates/step-export`, #88) |

Reference-only (read, don't depend): **truck** (only living Rust B-rep
kernel; active on git but crates.io releases stale; booleans demo-grade),
**curvo** (active pure-Rust NURBS evaluation/fitting-interpolation;
audited at M5 — NO SSI (empty placeholder module) and demo-grade 2-D
clipping only, an earlier "incl. SSI" claim here was wrong; oracle
scope per Q5/docs/CURVO-AUDIT.md), **vcad** (new Apache-2.0 half-edge B-rep kernel with
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
