# CAD Kernel — Design Document

**Status: v0.** Living document, present tense only. Decisions marked
*agreed* are settled unless new evidence overturns them; items in
[Open questions](#open-questions) are under discussion and get promoted
here once ratified. The history behind a decision lives in the PR it
names, in `docs/DOC-LEDGER.md` and in git — not here.

## Companion documents

Ratified design lives in this document AND in per-topic companions.
Companions whose programs have closed live as README pages beside the
code they govern and keep their clause ids: a citation such as
`CURVED-DESIGN C3` or `ASSEMBLY-DESIGN A6` resolves to that clause in
the row whose scope names the family. The design conversations those
pages condense, and every deleted `docs/` file, are recorded in
`docs/DOC-LEDGER.md`. Live work is never listed here: `work/STATUS.md`
is the board and `work/README.md` its contract.

| Document | Status | Scope |
|---|---|---|
| `crates/geom-brep/README.md` | Ratified (#85) | Curved geometry, CURVED-DESIGN C1–C12: locus ladder, certificates, SSI, pcurves, dispatch, fillets, NURBS scope |
| `crates/geom-brep/README.md` | Ratified (#907) | Offset & shell, OFFSET-DESIGN O1–O6: analytic offsets by struct-update, the approximating-surface lift (`Surface::Approx`), the offset certificate, what shell IS (Q8's resolution) |
| `crates/editor-core/src/names/README.md` | Ratified (#74) | Persistent naming, NAMING-DESIGN N1–N7: derivation-path names, split/merge policy, name table |
| `crates/editor-core/README.md` | Ratified (#79) | GQ1 witness mechanism, SOLVER-DESIGN W1–W9 |
| `crates/editor-core/README.md` | Ratified with a recorded hedge (#1151) | The profile-parameter lift, PROFILE-LIFT-DESIGN PP1–PP6: guided replay — structure f64-once as the witness, geometry at the lane scalar with every consumed decision re-verified at `T` |
| `crates/editor-core/README.md` | Ratified (#496, option A′) | Group boolean in the recipe layer, GROUP-BOOLEAN-DESIGN: `PlacedUnion`, a Pattern that fuses — one prototype, one body out |
| `crates/editor-core/ASSEMBLY.md` | Ratified (#333); v1 shipped | Assemblies, ASSEMBLY-DESIGN A1–A13 + AQ1–AQ8: assembly-evaluates-to-a-body, mates as declarations, pins/split-inline, validity, mirror, relative freedom, product roots, the constructive-solve boundary |
| `crates/topo/README.md` | Ratified (#178, #965) | Contact census & declared contact, CONTACT-DESIGN C1–C8 (the C7 join lane is shipped); at-rest census structural identity, the CENSUS-REST-CLOSURE-DESIGN clauses |
| `crates/geom-core/README.md` | Ratified (Ev, 2026-09-05) | The spline layer's pairing rule, SPLINE-DESIGN S1: a `Span` borrows the `KnotVector` it indexes, a `CurveWindow` its curve and a `SurfaceWindow` its surface, so every span-restricted door takes one structure and reads everything from it and the mismatch is unrepresentable; the coefficient↔vector pairing at `hull`'s free functions is what stays open |
| `crates/sweep/README.md` | Ratified (#992) | ARMS-3, ARMS3-DESIGN A3-1…A3-3: the sphere×sphere fillet arm, the valence-4 seam vertex that is not a corner, what a run-out IS; the blend-vocabulary clauses V1–V4 |
| `crates/profile/README.md` | Ratified (V1–V8; enclosing tangency #1210) | Profiles as programs (PROFILES-V2-DESIGN V1–V8); the enclosing (ρ < 0) fillet tangency is permanently unreachable and a radius demanding it refuses typed (ENCLOSING-TANGENCY-DESIGN) |
| `crates/viewer/README.md` | Ratified; GUI v1 shipped | GUI architecture G1–G5, GQ1–GQ7: the three-layer split, egui as toolkit, what v1 ships |
| `docs/ERROR-DESIGN.md` | Ratified (#110); running as M10 | Error propagation E1–E12: duals, stackups, the subdivision driver, trichotomy |
| `docs/DUAL-DESIGN.md` | Ratified (#1146) | The Dual contract DL1–DL6: a Dual is tangent transport and never certifies; ContentBits feeds both channels; the delegation rule; poison-vs-widen in certified lanes |
| `docs/PATHS-DESIGN.md` | Ratified (#124) | The PartialPath authoring algebra |
| `docs/LIBRARY-DESIGN.md` | Ratified (#229); program open | Usable-as-a-library L1–L8: façade, Python bindings via the document layer, v2-fronted PATHS, the authoring-ergonomics unit ladder |
| `docs/DISCIPLINES-DESIGN.md` | WIP, provisionally accepted | Disciplines/checks registry DS1–DS9: identification criterion, severity invariant, the four grades, the recording dial, out-of-tree checks; residents live in `editor_core::checks` |
| `docs/PCURVE-UNIFY-DESIGN.md` | Ratified (#514); executed | Pcurve unification: the conventional edge variants collapse to ONE (surface, `Pcurve`) form, the exact variants kept as certification lanes; `MappedCurve` demotes to an authority record. P-2 residue stays open |
| `docs/RECIPE-DOORS-DESIGN.md` | Ratified (D2–D5) | Recipe doors for the surgery verbs: `Node::Chamfer` is `Node::Fillet`'s twin; `Node::Tube` and `Node::HollowTube` (wall REQUIRED on the hollow kind, `Option` nowhere in the recipe vocabulary); shell's door (D5) is not built — `ShellNaming` exists in `topo`, `Node::Shell` does not |
| `docs/MIRROR-DESIGN.md` | Ratified (#909); unbuilt | Patterns & mirror P1–P6: the chart-handedness convention (u ↦ −u), mirror's own door beside rigid transform, the boundary of A6's equivariance audit |
| `docs/DRAFT-DESIGN.md` | Ratified (#908); unbuilt | Draft DR1–DR6: plane walls only at v1, a certified re-geom pass, the pull-direction selector as a SELECT-DESIGN amendment, survivor naming |
| `docs/SELECT-DESIGN.md` | Ratified | Selection: filters, heterogeneous sets, vanishing entities; the contact-site recourse (§3d) |
| `docs/VERB-SEAT-DESIGN.md` | Ratified (#1388); running as SEAT | The kernel query seat, one verb vocabulary, lowered parameter identity: §1 query doors at `topo`; §2 the per-verb kernel `Verb` declaration; §3 the opaque per-field `ParamSource` channel |
| `docs/MATE-7-TANGENCY-DESIGN.md` | Ratified | Torus×torus rim tangency; the kissing arm banks on it |
| `docs/DOCM-REFERENCES-DESIGN.md` | Ratified; running as DOCM | What a recipe reference may be, DM1–DM6: `Datum::FaceFrame`, the carrier-kind read, `Node::Part`, the n-ary `Node::Union` with `DocEdit::SetMembers` (DM4 shipped; DM1–DM3 in spec) |
| `docs/DOCM-IDENTITY-DESIGN.md` | Ratified; running as DOCM | A held value names the world it came from, DI1–DI5: history-branch validity of node ids, the memo as a pure function of the document, `Evaluation` carries its document's identity, forking is its own act |
| `docs/KERNEL-VERBS.md` | Reference register | The modeling verbs the kernel does not yet have, each with prerequisites, and the "present today" inventory. The register never schedules |
| `docs/K-REPORT.md` | Reference | K-constant evidence record (K = 10 permanent) |
| `work/perf/plan.md` | Merged-and-advisory (D9 addendum) | Performance plan and Q-P answers |
| `docs/CURVO-AUDIT.md` | Reference | curvo vendor audit behind Q5 |
| `docs/LONGTERM-IDEAS.md` | Parked, non-binding | Idea bank with a graduation rule |
| `docs/MODEL-AB-LOG.md` | Experiment log | Model A/B protocol + data; process, not design |
| `docs/NAME-CANDIDATES.md` | Reference | Q9 name candidates (re-sweep before ratifying) |
| `docs/predicate-dimension-audit.md` | Live audit | Dimensional-analysis sweep of predicate comparands against D4; its *Findings* section is the live list |

## Vision

A greenfield B-rep solid-modeling kernel in Rust, built API-first: the
kernel and its programmatic modeling API are the product; any GUI is a
thin client. Code quality and functional style are explicit goals — a CAD
kernel's job is to define what a shape *is*, and the implementation
should read that way.

**Reach goal (shapes the architecture):** native error propagation —
distributions over model parameters propagated through the model to
detect self-intersection and to compute tolerance stackups.

## The central commitment

> **A model is a pure, replayable function from a parameter vector and a
> tolerance to a solid.** `fn build(params: &Params, tol: Tol) ->
> Result<Solid, ModelError>` — deterministic, no hidden state. The B-rep
> is a derived value, never a mutated-in-place object.

Determinism is over the pair: the same parameters at the same ε give the
same solid. ε is one value per run, committed once (D4 ¶1).

Everything else follows from holding this invariant from day one:

- **Error propagation** is "evaluate the same function with a different
  scalar type" (intervals, duals) instead of a rewrite.
- **Undo, caching, and diffing** are free — models are values.
- **Testing** is property-based testing over parameter space.

The data-shape consequence: the geometry *evaluation* layer is generic
over a scalar type `T` (default `f64`); topology stays concrete (Q1).

## Decisions

### D1 (agreed): ID-based arenas, immutable values, manifold-first, Euler operators

- Topology entities (`Solid / Shell / Face / Loop / HalfEdge / Edge /
  Vertex`) live in generational arenas and reference each other by
  typed IDs — never pointers. A B-rep is a plain value: cheaply
  cloneable, serializable, diffable, validatable. Realized as Mäntylä's
  half-edge structure in typed arenas: an `Edge` is two antiparallel
  half-edges, the mate computed and never stored; the empty loop is a
  typed `LoopBoundary::Empty | Cycle`, so every half-edge field is
  non-optional and the sole `Option` in topology is the vertex's
  emanating half-edge. A face's outer loop is excluded from its ring
  list (`outer ∉ rings`), so rings coincide with the Euler–Poincaré
  r-term.
- **Manifold solids only.** Non-manifold (radial-edge) topology is added
  only if sheet/wire bodies demand it.
- Topology is built **exclusively through Euler operators**: a small
  closed set of primitives that provably preserve the Euler–Poincaré
  invariant. Each operator debug-asserts its postcondition — a per-call
  instance of the soundness theorem, never a semantic gate on
  intermediate states. "Exclusively" is realized: the operator set is
  the only public construction path; raw insertion is crate-internal
  test scaffolding.
- **A `Body` is never authoritative.** It is the materialized evaluation
  of a construction (an operator sequence; above the kernel, a recipe)
  at some scalar `T`, coherent iff bit-identical replay reproduces it
  (D9). Mutation exists only as evaluator-internal linear working
  state; a body at rest is a plain value, and modification means
  deriving a successor body by further construction. Nothing about a
  body is true that is not derivable from its construction (for
  imported bodies, from the adopted descriptions plus the import
  record, D7).

**Topology conventions.** One rule, from which everything else is a
corollary: walking any loop in `next` order with the face's outward
normal toward the viewer, the face interior lies to the **left** of
every half-edge. Corollaries: outer loops run counterclockwise viewed
from outside and rings clockwise; an edge's two half-edges are
antiparallel (`end(he) = start(mate(he))`); `Edge::he_plus` defines the
edge's intrinsic direction, and curve geometry MUST agree — increasing
curve parameter runs from `start(he_plus)` to `end(he_plus)`, pcurves
and per-face traversal senses derived from that, never stored as peers;
the vertex-orbit step `next(mate(he))` visits a vertex's outgoing
half-edges **clockwise** viewed from outside. Transcription hazard:
GWB/Mäntylä's diagrams orient face boundaries clockwise viewed from
outside — mirrored relative to us — so figures, argument orders and
traversal idioms from the book are rederived from the interior-left
rule and pinned by construction tests, never transcribed. The normative
derivations live in `crates/topo/src/entity.rs`.

**Face orientation sense.** A face carries `Face::sense: bool`: `true`
iff the face's material side agrees with its surface's chart normal, so
the outward normal at a point is `sense_sign · n(u, v)`. The analytic
chart normals admit no reversal by reparameterization (cylinder, cone
and torus normals are odd in the radius; the sphere's is even and
outward under the `radius > 0` convention — a negative-radius sphere is
rejected as a representation), so the bit on the face is the
representation of reversal. Normative consequences:
- The interior-left rule is stated against the sense-signed chart
  normal.
- Orientation reversal is **exact structure**, never a numeric decide:
  `revert` flips `sense` on every face carried by a non-plane surface
  and negates the stored normal of `Plane`-carried faces. The two
  encodings are exclusive by surface kind, so every outward normal is
  negated exactly once and `revert ∘ revert` is bit-identical at every
  scalar backend.
- A face **fragment** inherits its parent's `sense`: `mef` and `mfkrh`
  mint `true` for a new or foreign surface, but a face landing on the
  old face's surface key takes that face's bit. Key equality, never a
  numeric compare.
- Every "which way is out" consumer (tier gates, mass-properties flux,
  boolean classification, tessellation and export winding) reads the
  signed normal, or documents in place why it is sense-invariant.
- The bit is exactly STEP's `advanced_face.same_sense`, so the exporter
  consumes it rather than deriving it. Bodies are not serialized (D9),
  so persistence is unaffected.

**The operator set.** Ten operators in five make/kill pairs —
`mvfs`/`kvfs`, `mev`/`kev`, `mef`/`kef`, `kemr`/`mekr`, `kfmrh`/`mfkrh`
— plus the `ring_move` reparenting helper, deliberately **not** an
Euler operator (`mef` does not reclassify rings, after GWB's `ringmv`).
Addressing is by half-edge key plus per-op **site enums** whose
variants are the degenerate cases (`MevSite::{Fan, Lone}`): degenerate
sites live in the argument types, not behind null checks. GWB's id-scan
layer is dropped; arena keys are the stable O(1) handles. The uniform
per-op contract: **atomic** (typed-error preconditions fully resolve
before an infallible mutation phase; a failed op consumes no key
slots), **deterministic minting order** (documented per op — D9
lineage replay), and a **debug-asserted tier-1 postcondition**.
Association convention: **the given/first half-edge's side is the new
or affected thing** — `mef`'s `he1` side becomes the new face's outer
loop, `kemr`'s `he1` side becomes the ring, `kef` kills the given
half-edge's face, `kev` the vertex it points at. Cross-shell `kfmrh` is
the shell-fusion form (same solid), `mfkrh` its inverse; `ring_move`
reparents only within one shell (`EulerOpError::CrossShell`).

**Validity tiers.**

1. **Tier 1 "euler-valid"** — the structural invariant of every
   Euler-reachable state, construction scaffolding included (empty
   loops, struts, self-loop edges, laminae are mandatory
   intermediates); what each operator debug-asserts. The checklist:
   referential integrity across all arenas (orphan geometry is an
   error); half-edge chain coherence; mate involution/antiparallelism;
   vertex anchoring (every vertex is referenced by ≥ 1 half-edge XOR is
   the lone vertex of exactly one Empty loop); vertex-orbit closure
   (manifoldness — watertightness is structural in the half-edge
   form); the ownership/back-pointer partition; shell-partition/
   edge-adjacency coherence; arity floors; bidirectional D5 provenance;
   and the **component-aware per-shell Euler–Poincaré**: per connected
   component of a shell's incidence complex, v − e + f − r = 2(1 − g)
   with g a non-negative integer, summing per shell to 2(c − Σgᵢ) over
   its c components. The naive per-body form is wrong for tier-1
   bodies — `mfkrh` on a detached ring disconnects a shell's surface
   while a single shell entity remains.
2. **Tier 2 "closed solid"** (`validate_closed`) — tier 1 plus: no
   empty loops, no valence-1 vertices, and c = 1 per shell (the third
   ban is independent: a promoted detached cycle ring disconnects a
   shell with neither an empty loop nor a strut). Finished bodies must
   pass tier 2; tier-1-only states are visible solely inside operation
   sequences, never across an API boundary at rest.
3. **Tier 3 "geometric"** — D4 ¶2 residual certification, plus the
   **material wedge-angle predicate**: at every edge the material wedge
   ∈ (0, 2π), bounded away from the ends by θ = ε/r; wedge = π is the
   legal smooth-seam case; and the ends carry a **declared second-order
   arm**: wedge = 0 (a cusp) and wedge = 2π (a knife slit, the cusp's
   `revert` image — legal together or not at all) are legal iff the
   tangency is **declared** (the C7 `Tangent` contact vocabulary, never
   inferred from values) and **jet-determinate**: quadratic transverse
   separation with κ_rel bounded away from zero — `TangentIntersection`'s
   own margin, so the cusp edge's honest description IS
   `TangentIntersection`. In-band κ_rel escalates; an undeclared cusp
   refuses (`UndeclaredCusp`); osculation refuses (`LaminaWedge`). The
   arm admits no laminae, so zero-volume bodies stay geometric defects.
   A doubled cusp (two material wedges on one tangent line) is the
   coincident-distinct-edges class, each edge classifying separately.
   Consumers with no wedge-0/2π answer (fillet, offset, mesh sizing,
   sector classification) refuse typed at the consumer.
   Also at tier 3: **prefer-intrinsic enforcement** (D2) and the
   **positive-volume orientation invariant** (exact-B-rep signed volume
   definitely-negative ⇒ invalid; margin V/A_total, a length; zero and
   escalated exempt — an orientation probe, not a thinness gate, so
   ε-tightening never flips valid→invalid). Laminae live here, not at
   tier 2: two faces glued along their whole shared boundary is a
   two-hemisphere ball's incidence structure, so a zero-volume lamina
   is a geometric defect, not a topological one. Global
   self-intersection / minimum clearance is the interval clearance
   engine's: its body-level half — cell subdivision over
   `Body<Interval>` — lives in `editor-core` today and moves into
   `topo` behind `interval` (SHELL-3, ruled at #1737), with the
   parameter-box outer half above it in `editor-core`, so a verb that
   must certify a boundary embedded (`shell`'s cavity clone) runs the
   same engine at a certifying scalar and refuses typed at the door.
4. **Tier 3′ "pseudomanifold"** (`validate_pseudomanifold`) — the
   honest at-rest tier for boolean results that *touch*: contacts
   limited to entirely-coincident-but-distinct edges, edge-on-face,
   vertex-on-face, vertex-on-edge/vertex — touching allowed, proper
   self-intersection not. Composition: tier 3's local battery
   verbatim (shared extraction) plus a **global coincidence census**
   plus **two-directional declared-contact certification**:
   - The census is exact on the planar inventory and admits every
     carrier kind: same-key opposed-sense curved pairs certify through
     the conformal arm, declared curve/patch records through the jet
     schedule and the patch certifier, cross-solid pairs with a curved
     side in reach refuse as undecidable, and same-solid distinct-key
     curved pairs stay undetected until C9/C6 (`topo::census` module
     docs). A record or candidate outside a certifier's lane refuses
     typed `CensusUnsupported`, never samples. Every comparison is a
     named Q1 trilean; indeterminates surface as typed
     `CensusEscalated`, never a silent skip.
   - Certification runs **both directions and never scans-to-bless in
     either**: a census finding with no backing declaration is
     `UndeclaredContact` (discovery is never declaration); a
     declaration with no geometric witness is
     `StaleContactDeclaration`. Structural sharing (same key) is the
     coincidence ladder's first rung and needs no record.
   - Contact records carry two granularities: vertex (`VvContact`,
     `VfContact`) — edge-on-face and coincident-edge *segments*
     certified by reconstruction from their bounding vertex records
     (between two backed bounds, two lines sharing two points are one
     line; a missing bounding record is `UndeclaredContact`, never
     inferred) — and face (`CurveContact`, `PatchContact`; CONTACT-DESIGN
     C3), whose rungs back a subordinate vertex event.
   - **Certification strength equals its skeleton**: a `CurveContact`
     is certified at its jet samples plus hull bounds, a `PatchContact`
     by definitely-positive region overlap in the shared chart, a
     vertex-granularity area contact via its corner/segment records.
     Nested-shell pure containment (a void, zero coincidences) is
     census-invisible and certifies — the voids story below, not a
     gap.

   **Touching is always backed by explicit intent**: (i) operand
   coincidences are only ever structural (shared key) or declared
   (recipe data) — near-coincidence NEVER silently becomes contact
   (escalated typed error instead); (ii) result-side touching arises
   only from those intentional coincidences propagated through the
   boolean node, and the result carries machine-checkable
   declared-contact records (the ON-set survivors, carried across
   seam-zip/merge mints by a descendant map, never re-derived);
   (iii) an *undeclared* contact discovered at validation is a hard
   error, never blessed.

   **Validity class rides the result wrapper, never a mutable `Body`
   field**: a boolean result is `BooleanBody` — body + contacts — whose
   non-empty contact list is the 3′-grade currency and whose at-rest
   gate is `validate_pseudomanifold(&body, &contacts)`; empty-contact
   results remain plain tier-3 currency, and the two gates agree there.

   **Representability boundary**: pseudomanifold touching via
   *distinct* entities (two vertices at one point, two edges on one
   segment) is representable in the half-edge structure and is a typed
   *success* carrying its 3′ declarations. Genuine non-manifoldness — a
   single edge with >2 faces, a shared-entity wedge fan — is
   unrepresentable and stays a typed error at the site that would have
   needed it. "Non-manifold" means non-representable.

**Structural conventions.**

- **Sweeps emit single-shell primary boundaries; every CAVITY is born
  through the shared void-insertion door.** A cavity's boundary is a
  disconnected interior shell, and its bookkeeping — orientation,
  census participation, containment evidence — has exactly one home:
  `topo::insert_void`, the door the boolean owns, callable without the
  SSI pipeline for provably-no-crossing cases with caller-certified
  containment. Three producers: boolean subtraction (its probe
  verdicts); `shell`'s sealed hollow (its offset margin, OFFSET-DESIGN
  O4); and the full revolve of a holed profile, DEFINED as
  `revolve(outer) − revolve(hole-as-outer)` and executed through the
  degenerate no-crossing arm (the profile's validated 2-D margins).
  Recipe-layer sugar may wrap any of these; the door stays the one
  birthplace. `UnsupportedToroid` is permanent: a D3 ring-torus
  boundary — spindle tori have no representation.
- **∅, disjoint, and voids are typed results.** ∅ is a typed success
  value (`BooleanResult::Empty`), not an error; disjoint unions and
  voids are tier-2-legal multi-shell bodies. The extrude/full-revolve
  hole asymmetry is structural: extruded holes are cap-to-cap tunnels
  (one shell, genus); full-revolve holes are cavities (a second shell,
  through the door); partial revolve is extrude-shaped and carries
  holes in its one shell.
- **The minimal sphere at rest is V2/E2/F2**: tier 2's valence-1 ban
  makes a one-band wire sweep unrepresentable, so axis-touching full
  revolves sweep two π-bands and poles have valence 2.
- **Parameterization conventions** (authoritative text in the `geom`
  crate docs and its `curves`/`surfaces` modules): curve entities are
  complete loci; an edge's bounds derive from its vertices via the
  `he_plus`-forward contract (certification enforces forward, nonzero,
  ≤ one-period spans, the stored interval a certified cache reconciled
  against vertex authority). Shared azimuthal frame for all revolution
  surfaces (axis = +a₃, v_ref = axis × u_ref, seam at u_ref — for
  revolved bodies u = 0 IS the profile half-plane); sphere uses
  latitude; cone v = slant length with the apex a true chart
  singularity (poison normal, never sampled); normals are the chart's
  ∂u × ∂v with no "outward" contract — topology carries sense. A seam
  is defined SPATIALLY (the u_ref half-plane meridian), which on
  mirror-nappe cones differs from chart u = 0.
- **Profile format**: a profile loop is a vertex chain with bulge
  (b = tan(θ/4) of the arc to the next vertex, DXF-compatible) — zero
  representation-consistency conditions by construction; closed
  carriers split into ≥ 2 vertices; winding is invisible to users
  (roles derive from containment). Downstream re-inspection of arc
  geometry uses the stored bulge/carrier data, never endpoint atan2.
- **Declared-tangency discipline**: profiles refuse undeclared
  definite-Zero tangency at junctions (`UndeclaredTangency`, with a
  repair menu); declarations are verified, never trusted
  (`TangencyContradicted`); the PATHS `.fillet(r)` constructor authors
  exact tangency by construction and declares it, with fit gating
  (`TangentJointOutOfRange`). Every zero-turn joint is a declared
  tangent joint. Zero new ε: the per-junction classifier reuses the
  carrier predicates verbatim. The flags persist as `tangent_joints`.
- **Curved booleans retire per arm, never wholesale.** A face kind with
  no arm refuses typed `CurvedBooleanUnsupported` /
  `CurvedPairUnsupported` naming the pair, never falling through to a
  containment verdict a curved boundary can defeat. The wired germ join
  arms are plane×cylinder and plane×sphere (`boolean::join`'s dispatch);
  sphere×sphere and the declared-coaxial cylinder×sphere have section
  frames but no join arm; cone and torus operands refuse. The curved
  extent test refuses typed `NurbsExtentUnsupported` on NURBS faces — a
  certified extent needs a foot point plus a bound on the patch's
  reach past it, a derivation not yet written (C12.1).
- **Coincidence discipline in the reduction.** Every
  reduction/classification comparison is a Q1 trilean: definitely-off
  ⇒ clean side, exactly-on ⇒ ON, in-band ⇒ escalated typed error (a
  genuine sliver: the operand pair is ill-conditioned at this ε). No
  EPS snapping anywhere in the pipeline. Booleans on independently
  modeled nearly-touching bodies fail loudly rather than guess — the
  design thesis; the resolution is an explicit D7-style
  repair/adoption op.
- **Maximal-faces precondition and the merge stage.** Booleans
  precondition no two adjacent coplanar faces (`NonMaximalFaces`); the
  explicit opt-in normalization op is `merge_coplanar_faces` (merging
  is never silent), and boolean *outputs* run it as a documented final
  stage of the op's contract — the seam zip manufactures coplanar
  pairs by construction; the recipe records one boolean node, not
  hidden healing. Merge glues on the structural and declared rungs
  only; numeric coincidence never merges. Load-bearing dependency:
  `merge_coplanar_faces` **never elides vertices**, and tier 3′'s
  strict record-drop rule (a contact record whose vertex pair fused
  into one vertex is consumed and drops) is correct *because* of that;
  any future collinear-vertex elision re-opens the record-carriage
  class.

**The frontier is typed, named and inventoried elsewhere.** Every
unbuilt case refuses with a message naming its own blocker (D9 row 2),
so the frontier is grep-able from the tree; its prose inventory is
`docs/KERNEL-VERBS.md`'s "present today" paragraph and the crate
READMEs, and its schedule is the tracker. The lettered entries below
are the ones other documents cite by letter ((a) composition surgery
and (b) the SSI generic-`T` lift are discharged and keep no entry):

- **(c) the fitted general-circle mint route** — `certify_fitted`'s
  Circle-carrier arm is reachable from no mint site, so the
  oblique-trihedron octant faces stay legally uncached; the cone/torus
  oblique classes have no ring-computable meters composite and refuse
  with the class named.
- **(d) cyl×sphere germ chords** — a fitted carrier's chart image
  exists as `Pcurve::Fitted` and certifies at rest; what is missing is
  the join window itself (`run_azimuth_window`/`chart_pcurve` have no
  cyl×sphere analog). Sphere×sphere seams, cone and torus operands
  refuse alongside it.
- **(e) the NURBS extent test** — `NurbsExtentUnsupported`, above.
- **(f) the canal-surface general blend** — an approximating surface
  for fillet chains whose rolling-ball spine is neither a line nor a
  circle (`BlendError::SpineUnsupported`). Parked, not scheduled: no
  acceptance shape consumes it, and building reviewed machinery with
  no caller is the dead-code pattern. It re-opens with its first
  consumer (variable-radius fillets).

Unlettered standing entries: the both-sided zero-area pinch split
(single-sided pinches succeed by the exact mirror identity
`split(S, n) ≡ swap(split(S, −n))`; the `BOTH_SIDED` fixture pins the
refusal); reflex-corner tilted crossings; the torus declared-`Rest`
lane (#968) and full-period walls in the cylindrical `Rest` lane
(#1415, #1416); the coincident-plane classes beyond the
declared/anchored repertoire, where the anchor-exhaustion arm is
load-bearing rather than unreachable. Two latent-and-loud limitations
are recorded so they are not rediscovered as bugs: a meridian-tangent
circle is in-lane but uncertifiable (no constructor mints one), and
genuinely-oblique trihedral corners build through tiers 1–2 and report
`VolumeUncomputable` — a gap in the props inventory (no sphere-face
quadrature lane), not in the body.

### D2 (agreed): Intensional edge descriptions; no extensional fallback

Topology and geometry live in separate arenas: faces reference
surfaces, edges reference curves, vertices reference points.

**Background: pcurves.** A surface is a map `S(u,v) → ℝ³`. A face is a
region of that surface's parameter plane, and each boundary edge is
therefore also a curve `P(t) → (u,v)` in that plane — the *pcurve*. An
edge shared by two faces classically carries three representations: a
3-D curve plus one pcurve per adjacent face, with the consistency
requirement `Sᵢ(Pᵢ(t)) ≈ C(t)`. Pcurves are not optional — trimming,
tessellation and intersection marching happen in (u,v) — and the
redundancy among peer representations is a classic bug farm.

**Our rule:** an edge's geometry is stored as an *intensional
description* of what the locus **is**; every concrete representation
(the 3-D carrier, both pcurves) is a derived cache carrying a certified
residual bound against the described locus (D4). The type is
`EdgeDescription<T>` (`crates/geom-brep/src/description.rs`):

```text
EdgeDescription =
  | Intersection        { s1, s2, witness }  -- transverse surface–surface
                                             -- intersection: the connected
                                             -- component of S₁∩S₂ selected by
                                             -- the witness (also the marching seed)
  | TangentIntersection { s1, s2, witness }  -- tangential contact locus; same
                                             -- shape, margin one order up
  | Chart(ChartCurve)                        -- a curve the surface UNDER-determines:
                                             -- (surface, Pcurve) with a `seam` flag
                                             -- (iso-lines, seams, user splits)
  | Scaffold(MappedCurve)                    -- construction-time pushforward of a
                                             -- lower-dim entity; never at rest
```

The intrinsic variants describe loci determined by their surfaces; the
conventional form carries the defining data for loci the surfaces
*under*-determine (parameterization seams — infinite-order contact, the
seam's position pure convention; face splits at smooth profile joins,
where at a G2 join even `TangentIntersection` fails its margin, and
rightly; user splits). One (surface, `Pcurve`) form covers all of them
(PCURVE-UNIFY-DESIGN); the exact classes stay as certification lanes,
not variants. A `MappedCurve` survives as the `Scaffold` payload and as
the `EdgeAuthority` record — one authoritative source, never two peer
representations needing cross-reconciliation.

**Deliberately omitted: an `Explicit` (extensional) variant.** Taken as
an unconditional challenge: every edge must have an intensional
description — there is no escape hatch, so it cannot be reached for
when not absolutely necessary. This holds for imported geometry (D7):
the intrinsic variants are checkable properties of the geometry as it
now stands, and the conventional form carries its own defining data, so
extensional input is *adopted* by reconstructing the description it
satisfies. What import pressure-tests is the completeness of the
taxonomy (imported fillets force `TangentIntersection`), not the need
for a fallback.

**Validity predicates with margins.** `Intersection` requires
*transversality*: normals of S₁, S₂ linearly independent along the
locus, so the implicit function theorem makes S₁∩S₂ locally a
1-manifold; the margin (angle between normals) governs the conditioning
of every derived cache. `TangentIntersection`'s validity sits one
differential order up: surfaces coincident within ε and normal-parallel
within the derived angular threshold ε·κ_rel (D4 ¶1: lever arm
r = 1/κ_rel) *along* the locus, separating quadratically *transverse*
to it (relative normal curvature bounded away from zero — otherwise the
surfaces osculate over a patch and the "locus" is not a curve).
Reconstructing a tangency locus is well-conditioned *despite* the
tangency because its defining system includes the first-order
normal-alignment equations: the normal angle grows linearly with
transverse distance, and the second-order margin is the
implicit-function-theorem denominator for that jet system. (Order-k
contact generalizes: defining equations from the k-jet, margin at order
k+1.) In the intrinsic variants "the locus lies on both surfaces" holds
*by definition*; only the caches need certification. Vertices
generalize the same way (three surfaces / endpoint of a locus, with a
witness). A locus in the ambiguous band — a dihedral within a few
derived angular thresholds of tangent, certifiable as neither — fails
loudly at construction exactly as at import; a conventional description
is not an escape hatch from ill-conditioned geometry.

**Prefer-intrinsic rule, tier-3-enforced.** Wherever an intrinsic
description is certifiable, it *is* the stored description — including
for native constructions: a fillet we build stores its boundary edges
as `TangentIntersection`, the rolling-ball construction demoted to
supplying the witness and initial caches. Construction history lives in
D5 provenance, never in the description, so native and imported bodies
carry identical descriptions. At rest, every *definitely-transverse*
edge must carry `Intersection` (`TransverseNotIntrinsic` otherwise); a
jet-determinate tangency must carry `TangentIntersection`
(`TangentNotIntrinsic`, where `geom_brep::tangent_certificate_lane`
admits the class); a second-order-underdetermined join keeps its
conventional form by the predicate itself. The check reads the edge's
authority record, so a declared conventional description is exempt by
its own declaration; escalated dihedrals, seam edges and
NURBS/`Approx`-adjacent edges are exempt too, so ε-tightening can
escalate but never flip a valid body to invalid. An unenforced
preference drifts silently — exactly the failure shape this document
exists to kill.

**Witness contract.** "Selected by the witness point" is verifiable only
if the witness is *pinned*: the stored witness IS the carrier's
mid-parameter point, enforced by certification (`WitnessMidpoint`).
Pinning removes the aliasing freedom that let any point on the
component certify, including points encoding a wrong winding; residual
freedom is documented where it is geometrically invisible (circles:
whole-period translation). Every op that mints an `Intersection`
computes the witness as carrier(mid) in the certification schedule's
own association order.

This makes D5's provenance load-bearing rather than bookkeeping: the
intensional description largely *is* the provenance.

### D3 (agreed): Analytic surfaces special-cased; NURBS as the general fallback

Plane / cylinder / cone / sphere / torus are first-class variants
alongside NURBS (as in Parasolid), not converted to NURBS. Most
mechanical geometry is analytic; analytic×analytic intersections have
closed forms, while NURBS×NURBS is a numerical marching problem. A
seventh variant, `Approx`, is the certified approximating class
(OFFSET-DESIGN O2): a fitted NURBS carrying the intensional description
of what it approximates plus a certificate bounding the distance,
re-derived per face and never trusted from storage.

**Extensibility:** surface kinds form a *closed enum*, not open trait
objects. Intersection requires pairwise dispatch, and a closed enum
gives compile-time exhaustiveness, so adding an analytic kind means
adding a variant and letting the compiler enumerate every dispatch
site. The `Nurbs` variant is the universal fallback: any exotic surface
is at minimum representable. Same design for curves (line / circle /
ellipse / NURBS).

### D4 (agreed): Single strict global tolerance; operations fail loudly

No per-entity tolerances that grow as operations get sloppy (the Open
CASCADE model). "Define what something is" applied to error handling.
Five commitments:

1. **One number, global per run**: a linear tolerance ε, defined once in
   `geom-core` as the `Tolerance` value — once-initialized,
   env-overridable per run (`CAD_TOLERANCE_EPS`), **one value per run,
   shared by all bodies, never loosened mid-run** (per-model ε is
   rejected: any two bodies must be boolean-combinable). Per-run
   initialization lets the test suite run at several ε values.
   **Angular thresholds are always derived, never a second global**: an
   angle only means anything through the displacement it induces at a
   lever arm (d = r·θ), so a fixed εₐ would privilege a hidden length
   scale. Every angular predicate uses θ = ε/r with its lever arm named
   at the call site — 1/κ_rel for tangency, the face extent for
   parallelism, the session-box extent as the conservative universal
   arm. ε ≈ 1e-9 m gives micron-to-kilometer coverage with ~4 orders of
   f64 headroom at km scale. Import does not motivate loosening ε (D7).

   **The two-tolerance principle.** Two roles are kernel-wide
   vocabulary: **ε_precision** (this ε — certification residuals, what
   gets built) and **ε_input** — the least precision a user might care
   about: what counts as too-close-to-a-coincidence when interpreting
   input. ε_input IS K·ε, a synonym, not a third dial; K stays the one
   knob (`Tolerance.k`, env `CAD_AMBIGUITY_K`). Binding consequences:
   (i) user-facing messages and recourse never fork on exactly-on vs
   in-band below ε_input — ONE message, ONE recourse (declare the
   coincidence / move the geometry / lower the tolerance — the
   three-arm sentence at every site whose question is "is this margin
   decidable"; a contact site, whose question is "did anyone declare
   this", drops the third arm per SELECT-DESIGN §3d), with the margin
   riding the payload as data; kernel semantics keep the distinction
   (message policy, not predicate policy). (ii) Error variants may stay
   distinct as data; their user stories converge, with the shared
   `Indeterminate` Display string (`COINCIDENCE_RECOURSE`) as the
   carrier. (iii) D7's ε_in is an instance of ε_input, not a separate
   concept. (iv) **The rule binds a predicate's DEFINITE arms too**: a
   new definite outcome a user could reach by moving geometry tells the
   same one story with the same one recourse as its in-band sibling.
   Review checklist form: for every arm added to a decision, name which
   ε_input story it belongs to, or say why it belongs to none.

   **Chordal tolerance δ is not a tolerance in this sense.**
   Tessellation/export take a per-call display parameter δ (chordal
   deviation), chosen per export, varying freely, participating in no
   kernel validity decision. The tessellation promise is
   **certified-conservative** — closed-form sagitta/deviation bounds
   guarantee the mesh lies within δ of the true surface (honestly δ+ε,
   plus ≤1 ulp per coordinate from STL's f32 narrowing) — an export
   promise, not a kernel invariant. The mesh layer never reads ε for
   sizing (the inventory is a computed pin,
   `mesh/tests/all.rs::the_eps_inventory_is_pinned`); the one ε read
   that is a classification — pole identification — owes a guard so an
   ε flip can never move emitted coordinates with δ held fixed (#896).
   **The tessellation criterion is DISTANCE-ONLY**: no angular-deflection
   criterion in the certified tessellator — every contracted consumer
   is manufacturing-shaped, and a certified angular bound would cost
   normal-variation enclosure machinery purchased only for visual
   smoothness δ already buys. A display-mesh lane, when built, gets its
   own separately-honest criterion and is never promoted into the
   certified export promise.

   **The margin dimensional convention.** ε is a length: the maximum
   deviation from specified geometry at a single point. Four clauses
   make that structural: (i) **Margins are lengths, by signature.** The
   classify/Band seam takes a `#[repr(transparent)]` `Margin<T>`
   newtype — no dimension algebra, no generic dimension parameter —
   whose only constructors are blessed doors that make the dimension
   argument explicit at the call site (a coordinate difference that IS
   a length; a dimensionless quantity levered by an arm; a norm; a
   measure over its lever — 2A/P, V/A); the sagitta and reciprocal
   forms are the levered door applied at named arms; there is no raw
   construction door. Consistency backstops between integral results
   decide on bare `T` through `k_stats::decide_invariant`, no Margin
   minted, and a certified violation is a Corrupt-class kernel-invariant
   error, never a validity refusal. A site where no door honestly fits
   is a finding, not a cast. The vector/linalg interior stays bare `T`.
   (ii) **No dimensionally-heterogeneous uniform payloads**: a field
   whose dimension depends on a runtime kind tag is illegal;
   kind-dependent data lives in per-kind enum variants. (iii)
   **Parameter-space values cross to model space only through per-kind
   metric doors.** (iv) **Inequality gates split sign from magnitude**:
   a certified sign-certain violation refuses with no ε involved; the
   banded comparison governs only the near-zero region, and both arms
   consume the same metered comparand. Extension is opportunistic as
   signatures get touched; the migration ledger is
   `docs/predicate-dimension-audit.md`, whose flagged rows ride
   `k_stats::decide_flagged` — visible typed debt, not casts.
2. **Every derived cache carries a certified residual bound** against
   its intensional description (D2). Kernel invariant: `residual ≤ ε`
   for every derived item in a valid body; the `topo` validator checks
   it, at `f64` as a conservative estimate and at `Interval` as an
   enclosure.
3. **Failure is a typed, actionable error naming the failing check and
   the entity** — consumable by humans and by the error-propagation
   machinery. The carrier is `CertifyError::ResidualExceeded { check,
   sample }`, wrapped by the attachment gates and by
   `ValidationError::EdgeCertification`; the residual MAGNITUDE rides
   the escalated arm's `Indeterminate`, because no `f64` projection of
   a generic `T` exists on every lane. Geometry that can't meet ε almost
   always indicates a modeling mistake; surfacing it beats absorbing it.
4. **Fixed internal units — meters and radians — with a documented
   model size range**; geometry outside the range is rejected at
   construction. User-facing units are typed newtypes at the API
   boundary only (D6).
5. **Strictness is enforced at the boundary, not relaxed inside**: STEP
   import is an adoption stage (D7) that brings external geometry up to
   kernel invariants *before* it becomes a kernel body; entities that
   can't be adopted fail loudly in a typed import error.

### D5 (agreed): Persistent topological identity from birth

Every topological entity carries a provenance record from the moment it
is created: a typed per-operator **birth record** — the operator plus
its argument keys — on every entity of all seven arenas. Kills remove
the record with the entity; survivors keep theirs; reparenting or
demotion (`ring_move`, `kfmrh`'s loop demotion) is not a re-birth. The
validator enforces the record bidirectionally. This does not solve the
topological naming problem — the parametric layer builds its stable
references on top of it (NAMING-DESIGN) — but recording identity at
birth is cheap and retrofitting it is nearly impossible.

### D6 (agreed): Canonical internal units; typed units at the API boundary

Kernel-internal code is raw `T` in meters/radians by convention — no
dimensional types inside. The public API uses hand-rolled newtypes
(`Length`, `Angle`, …, the `quantity` crate) that convert on entry.
Hand-rolled rather than `uom`: uom's dimensional generics fight the
scalar-type parameter and we need ~five quantities, not the SI lattice.

**A stored literal always names its notation.** Units erase at the
accessor doors because the kernel wants them gone. One consumer wants
them kept: a document records what a person *wrote*, so it can be read
back that way. Every continuous literal therefore carries a display
unit — a row of `quantity::UNITS`, presentation metadata excluded from
expression identity, keys and evaluation — and that unit is **not
optional**: the table carries a dimensionless row (`ONE`, empty symbol,
factor 1.0) so a `Scalar` literal names its notation rather than
declining to. `Count` needs no row: a count is an integer, not a
quantity. A value crossing into a document carries the unit it was
written in, never a bare number — which is why the GUI's creation ops
carry `Expr`, and what `quantity::WrittenLength`/`WrittenAngle` are the
library spelling of.

### D7 (agreed): Import is adoption, not admission

Imported geometry is not second-class. Rather than an extensional escape
hatch, import **reconstructs** the intensional description the
extensional data satisfies — possible because the intrinsic variants
are properties of the current geometry, and the conventional form
carries its own defining data, so an imported seam curve *is* the
convention, adopted directly. Pipeline:

1. **Surface recognition**: an imported NURBS within ε of an analytic
   surface is promoted to it, restoring D3's exactness to imported
   bodies.
2. **Edge adoption**: verify the imported curve lies within ε of the
   intersection of its adjacent surfaces with adequate transversality
   margin, then rebuild it as `Intersection { s1, s2, witness }` — the
   imported curve demoted to witness + initial cache. Seams and
   tangency loci are recognized likewise.
3. **Healing**: where no description is satisfied within ε, repair
   (refit/nudge) or fail loudly naming the unhealable entities.
4. **The shared at-rest gate**: steps 1–3 certify each *entity*; the
   *body* is then handed to the kernel's own at-rest validator — tier
   3, or tier 3′ where declared contacts exist — and only a body it
   passes ships from import. Same function, same tiers as a native
   caller runs; import holds no idea of validity of its own. *Per
   solid, not merely per file*: whole-body sums (the +V flux) would let
   an inside-out solid cancel against a right-side-out neighbour, so
   each solid is gated on its own body before aggregation, and the
   aggregate pass remains for the cross-solid structure. A file carries
   no arena keys, so the import-side declaration channel is
   POSITION-anchored and belongs to the adopting caller
   (`ImportOptions::declared_contacts`): declarations resolve against
   the assembled body and are certified by the same tier-3′ gate, and
   an anchor that does not resolve refuses typed. An imported assembly
   whose parts touch therefore refuses UNDECLARED and certifies WITH
   the declaration — the equivalence with a natively built twin.

**Adoption tolerance ≠ kernel tolerance.** Adoption takes a per-import
*input tolerance* ε_in — defaulted from the STEP file's declared
`uncertainty_measure_with_unit`, overridable per call. **ε_in governs
interpretation** (recognition and classification — what the data is
evidence of); **ε governs what gets built** — an adopted entity's caches
are recomputed from its description by our own algorithms and certified
at ε like native geometry. Healing may move geometry by up to O(ε_in)
to make the chosen interpretation true — a reported model change, never
a loosened certification. Data ambiguous at ε_in scale fails with a
typed ambiguity error rather than a guess.

**Non-goal: feature recognition.** Adoption recovers *what each locus
is*, not *how the body was modeled*; imported bodies carry no
parameters, so error propagation has nothing to vary over them.
Adoption is strictly stronger than industry "shape healing" (which only
patches data into self-consistency): it must *explain* the data.

### D8 (agreed): The recipe is data

A model document is an operation DAG — typed feature nodes referencing
parameters and each other — plus a small expression sublanguage for
derived quantities. The kernel interprets the recipe at any scalar `T`;
user-facing Rust is a *generator* of recipes. Consequences: the recipe
is the save format; recipe node IDs are the substrate for D5 naming;
every value-dependent branch stays inside kernel code where predicates
are reified (Q1) — user models as generic Rust functions were rejected
because `if width > 10.0` in user code would silently break interval
replay; and structural parameters (hole *count*) are explicitly
distinct from continuous ones (hole *diameter*), so parameter-driven
topology change is stated, not emergent.

### D9 (agreed): Determinism policy and engineering charter

- Same build + same inputs → bit-identical outputs. No hash-map
  iteration order may influence geometry; parallelism only in fixed
  reduction shapes (addendum below).
- Transcendentals via the pure-Rust `libm` crate: system libm differs
  across platforms in the last ulp — enough to flip a marginal
  predicate.
- **The kernel never panics on any INPUT** — every failure an input can
  reach is a typed error. **A panic is therefore never a refusal: it
  reports that a bug has already happened.** Read it as *a firing panic
  is evidence of a bug*, never as *a panic in the source is a defect to
  remove*. **The converse is a positive obligation: a state that can
  only be a kernel bug MUST panic**, as loudly and as early as it is
  detectable (`unreachable!` / `debug_assert`, the taxonomy below);
  downgrading one to silence, or to a typed error, launders a bug into
  a supported outcome. The two halves are separate rules over disjoint
  state classes. Every traversal is bounded: never a hang.
  The closure property behind the first half: every public mutation
  path preserves tier 1 — the Euler operators by the soundness theorem,
  the non-operator structural mutators by declaring the same debug
  postcondition or by being composed of operators that do, the
  attach/metadata setters by re-certifying under their own tier-1
  assertion. The claim is that property, not a count of doors;
  `topo`'s `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`
  checks it against the real surface. **The one door outside the
  property is `instance`'s graft**, a raw transplant: a `JoinDesync`
  raised mid-transplant leaves the destination partially written and
  *spent, never resumable*, so a caller that discards the `Err` and
  keeps the body can fire a later postcondition from API misuse rather
  than a kernel bug. That state class is the open ruling **S14**
  (`work/code-quality/S14.md`), Ev's; row 0 below reframes it (stage
  into a fresh body and commit on success, the shape
  `merge_coplanar_faces` already uses) without answering it.
- Essentially no unsafe Rust outside vetted dependencies.

**The bug-vs-invalid-state taxonomy (the D2 addendum).** Silent discard
is never an answer: a state that cannot occur is announced, not
swallowed.

| # | State class | Mechanism |
|---|---|---|
| **0** | **Can this state be made unrepresentable?** — asked of every state, before the rows below | **change the type.** Preferred over every row below whenever available |
| 1 | Reachable by input, **invalid** | typed error |
| 2 | Reachable by input, **valid but unbuilt** | typed `Unsupported*` error |
| 3 | **Value-domain degeneracy** | poison — NaN / empty |
| 4 | **Kernel bug**, observable in a branch | `unreachable!` |
| 5 | **Kernel bug**, detectable only by re-derivation | `debug_assert` |

- *Row 0 is a question, not a class.* A lane that files a state under
  any row owes the reason row 0 did not apply. It is answered against
  the cost of the type change: yes when the change is local to the type
  and its constructors (a private field and a constructor signature,
  no public API change); no when it propagates into signatures that do
  not otherwise care (a brand lifetime on `Body` would infect every
  signature naming a body — that no is the precedent for where the
  line falls). A "no" is a complete answer, recorded as the reason a
  row below applies.
- *Row 1 absorbs the terminal indeterminates*, but the axis is
  curable-vs-terminal, not bug-vs-invalid: an `Indeterminate` whose
  `MarginDiag` is `Value` or an `Enclosure` wholly inside a sliver band
  is a statement about the input and reaches the user through
  `COINCIDENCE_RECOURSE`; a straddling `Enclosure` is generally curable
  by subdivision, and a `MarginDiag::Invalid` from a domain clamp may
  cure as the violating sub-box shrinks (a NaI never does). The
  subdivision driver exists (`editor_core::drive`, ERROR-DESIGN E6): a
  curable indeterminate unwinds to it and is not reported as invalid
  input.
- *Row 2 is a naming rule.* `Unsupported*` means "valid input, the
  kernel has not built this yet" and nothing else, which makes the
  frontier inventory grep-able. A panicking `not_implemented!` macro is
  rejected: these refusals are reachable by valid input and must stay
  recoverable. Where a frontier branch genuinely cannot be reached it
  is row 4, with a message.
- *Row 3:* poison flows through **values**, never through decisions
  (Q1). `sup_norm_bound` returning NaN on every poison path is the
  pattern.
- *Rows 4 and 5 split on re-derivation, not on cost.* `unreachable!` is
  for an invariant the code can *observe* — a failed lookup of a key
  minted in the same call or proven live by a check in the same call
  (`topo`'s `Live` brand; a shared helper takes a proven-live key as
  its argument type and is `#[track_caller]`), **never** by the body's
  tier-1 validity, which is a whole-body property no single call
  establishes. `debug_assert` is for a check that *re-derives* the
  invariant (`assert_euler_postcondition`, O(body)). Row 4's message
  states WHY the state cannot occur, not merely what was violated, and
  carries the values a reader debugging it would want; this stays
  prose, not a gate (`topo`'s `d18_no_unreachable_message_can_impersonate_the_postcondition`
  forbids one spelling and that is all a shape gate can do).
- *Row 5's boundary:* `debug_assert` also serves the expensive check
  whose failure PROBABLY indicates a bug — a tripwire, not a proof.
  Its contract: (i) the assertion's absence never changes shipped
  semantics; (ii) an input-reachable failure still gets its row-1/2/3
  disposition; (iii) each such assertion documents its calibration
  in-file — the population measured and the margin observed — so a
  firing one reads as evidence (`geom_brep::props::quad`'s area gauge
  is the precedent; an uncalibrated ceiling was once off by nine
  orders). Where the measured condition is reachable by input, the
  preferred disposition is a reported value, not a tripwire
  (`topo::coherence`).
- *Boundary rule.* `pncad-py` re-types at the FFI edge — anything the
  Python layer can trigger is validated into a typed error before the
  kernel call, so an `unreachable!` never crosses into a
  `PanicException`.
- *Lint state.* `unreachable` is outside the banned clippy family in
  both the workspace manifest and `crates/pncad-py/Cargo.toml` (kept in
  step by that crate's `crate_lints_match_the_workspace_minus_unsafe_code`
  test); `panic`, `todo` and `unimplemented` stay banned. The
  `unreachable!` population and its per-site proofs live in the code;
  the unconverted exception (`merge_coplanar_faces`' ring re-homing,
  which reads its key from a loop back-pointer and refuses typed) is
  named in `topo::euler`'s module docs. The class is not confined to
  `crates/topo`; instances elsewhere are tracker items.

**Replay with kills.** The determinism contract holds with destructive
operators in the history: identical histories replay bit- and
key-identically, kills included; a failed operator consumes no key
slots. Convergence with a kill-free history is **per-arena**: a balanced
kill/make pair re-converges the half-edge, edge and curve arenas
immediately and the loop arena one loop-mint later; an unbalanced kill
history offsets the killed arenas' allocation cursors permanently —
untouched arenas stay aligned forever, killed arenas never re-align.

**D9 addendum — deterministic parallelism** (PERF-PLAN §2.2; every use
cites these two idioms instead of re-deriving):

1. **Indexed parallel map**: results written to slot *i* of a
   pre-sized buffer. Combination is positional, not arithmetic;
   bit-deterministic at any thread count.
2. **Fixed-shape reduction**: FP sums/mins are **never**
   `par_iter().reduce()` (rayon's reduction tree is schedule-dependent).
   Idiom 1, then a *sequential* fold in arena order — or, if that fold
   profiles hot, a fixed-arity block tree with a named chunk size and a
   documented combine order.

Targets in value order: the subdivision driver, per-face tessellation,
certification sampling, mass properties, independent DAG nodes.
Euler-op sequences stay serial.

*GPU boundary (PERF-PLAN §3.3), ratified:*

| Work | Home | Why |
|---|---|---|
| Rendering, LOD, ID-buffer picking | GPU (viewer) | no kernel coupling |
| Preview (uncertified) surface evaluation | GPU-eligible display lane | never re-enters the kernel |
| Certified tessellation, export meshes | CPU forever* | the export promise needs certified bounds |
| Booleans, splitting, SSI, predicates | CPU forever* | D9 + certification; a GPU pre-filter is not worth the audit |
| Euler ops, validators, arena surgery | CPU forever | pointer-chasing, serial, already cheap |
| Interval lane / subdivision driver | CPU (rayon) | embarrassingly parallel on CPU already |

\* "forever" = for this project's plannable horizon; PERF-PLAN §3.2's
grounds (rounding control, f64, portability) are re-checkable facts.

**Engineering conventions.**

1. **Sentinel-free tagged encodings.** Internal byte/key encodings never
   use in-band magic values (sentinel indices, marker floats); any
   stream mixing kinds is TAGGED TOKENS — tag byte + typed payload — so
   collisions are unrepresentable by construction.
2. **Save/load validation is ONE shared validator, not two mirrored door
   sets.** Every direction-independent document check lives in
   `persist::check::validate_document`, invoked by BOTH doors — at save
   on the in-memory document before bytes are written, at load after
   parse — so a document that would refuse to load is impossible to
   save by construction. The wire keeps only the genuinely load-only
   residue (parse/position errors, the canonical-set rule).
3. **A stale shorter matrix never gates a merge.** Any merge-gating
   checks watcher asserts a minimum green-row count equal to the current
   full CI matrix, bumped in the same PR that grows the matrix. CI
   carries a three-tier change filter, implemented once in
   `scripts/ci-filter.py` and called by both `ci.yml`'s filter job and
   `local-scripts/ci-local.sh` so hosted and local gating cannot drift:
   tier `docs` (only `*.md`/`memories/`) skips every build row and
   gates on the `docs-only` marker job; tier `all` (any workspace-level
   file, any member `Cargo.toml`, anything the allowlist does not
   recognise) runs the whole matrix; tier `closure` (crate sources
   only) scopes the cargo rows to the changed members plus every member
   that transitively depends on them. Classification fails CLOSED.
   `ci-local.sh --full` forces tier `all`.
4. **Semantic equivariance where it is free — with the premise
   UNAUDITED.** Kernel constructions and selection rules should commute
   with rigid motions *and reflections* at the semantic level (in ℝ),
   unless equivariance is provably impossible for the case or costs
   something real. This concerns DESIGNED rules — no left-hand rules,
   no absolute-orientation tie-breaks — not bitwise f64 equivariance,
   which D9's fixed evaluation orders forgo. Prefer intrinsic
   quantities (arc lengths, distances, angles) over enumeration or
   construction order; where a candidate-swapping symmetry makes
   equivariance impossible, fall back deterministically and DOCUMENT
   the residual. **The "the kernel is currently equivariant" premise is
   UNVERIFIED; an audit is banked, not assumed. Do not cite the kernel
   as equivariant without checking the claim at the site in question.**

## Layering

Each layer depends only on the layers below it.

| Crate | Contents |
|---|---|
| `test-utils` | The shared fuzz/property harness (seed + effort dial), a dev-dependency with ZERO dependencies — a leaf below every crate, which is what lets the excluded `interval-transcendentals` workspace depend on it too |
| `geom-core` | The `Real` scalar trait (`f64`, `Interval`, `Dual<T>`, `Sym`), points/vectors/transforms (hand-rolled, fixed-dim), the predicate vocabulary (`Decide`, `Margin<T>`, `MarginDiag`), `Tolerance`, root finding, spline hulls |
| `interval-transcendentals` | The `interval` feature's backend beneath `geom-core`: proven per-function libm error pads, MPFR-differential-certified. A separate workspace root on purpose (root `Cargo.toml`'s `exclude`), so its gmp-backed oracle never enters the kernel's graph |
| `bvh` | Deterministic AABB tree: arena-order build, fixed split rule with total tie-breaks, conservative-superset contract — the tree prunes, exact predicates decide. Below the geometry crates (only `geom-core` under it) so SSI subdivision can consume it; certified box constructors live beside their invariants in `geom` |
| `geom` | Analytic + NURBS types, evaluators, closest-point, curve×curve and curve×surface intersection. Curves and surfaces are two modules of one crate, so the parameterization conventions and the totality/poison policy are stated once |
| `geom-brep` | The B-rep geometry layer: D2's `EdgeDescription`, certified carrier caches, the dihedral classification predicate, Newell face equations, pcurve caches, SSI, the surface-pair dispatch table, certified mass properties, offset surfaces |
| `profile` | 2-D sketch profiles: the PATHS authoring algebra and the profile-program it records, lowering to the bulge-chain `Profile` and its trilean validation |
| `topo` | Arenas, entities, Euler operators, the validation tiers; plane splitting, the boolean engine and its census/declared-contact machinery (sibling modules at the crate root), shell/offset surgery, the kernel query seat |
| `sweep` | Solids from validated profiles: extrude, revolve, loft, sweep, tube; the blend family (fillets, chamfers) and its composition surgery |
| `verbs` | The kernel verb vocabulary seat (VERB-SEAT-DESIGN §2): one closed `Verb` enum reifying an operation's parameters as data, run dispatch, and the parameter→field flow; a layer guard keeps serde, `Expr`, `StableName` and recipe ids out |
| `mesh` / `stl` | Certified tessellation (watertight triangle meshes with source-`Face`/`Edge` back-references); STL export (binary + ASCII) |
| `step-export` / `step-import` | STEP (AP214) analytic-subset export, and import of that subset as adoption (D7) |
| `quantity` | Typed quantities at the API boundary (D6): `Length`, `Angle`, the unit table and the written forms |
| `editor-core` | Headless document/editor layer AND the parametric layer: document-as-value (recipe + metadata), typed edit vocabulary (`DocEdit` + pure `apply`), parameter expressions, feature DAG evaluation, persistent naming, stable-reference/selection model, incremental evaluation (preview/commit, epochs, cancelation), assemblies, distributions and the subdivision driver, the checks registry. No rendering dependency. See `crates/editor-core/README.md` |
| `pncad` / `pncad-py` | The authoring façade (LIBRARY-DESIGN U1 — one crate to depend on, a prelude, f64-first signatures) and its PyO3 bindings, which speak the document layer |
| `viewer` | The interaction layer over `editor-core`: `Camera`/`CameraOp` and `DocSession`/`SessionOp` as values with one `apply`/`perform` each, feature tree, property panel, selection, open/save, scene extraction — renderer-free and headless-tested; the eframe/wgpu application lives behind the non-default `app` feature. See `crates/viewer/README.md` |

Every layer below `viewer` *is* the product, exercised by tests and
code-driven models. The regression suite is mass-property checks
(volume/centroid vs. closed forms), watertightness validation, and
randomized parameter fuzzing — the fuzzing infrastructure is itself a
precursor of the error-propagation feature.

## Difficulty ranking (sequence around this)

1. **Fillets/blends, shelling/offsets** — hardest; even ACIS/Parasolid
   still get these wrong. Scope-boxed: constant-radius edge fillets on
   analytic geometry first.
2. **General SSI + robust NURBS booleans** — second hardest; deferred by
   D3.
3. **Booleans on analytic geometry** — hard but tractable;
   classification is the main challenge.
4. **2-D sketch constraint solver** — a real subproject but
   well-trodden; possibly bind an existing solver (Q3).
5. Everything else is careful engineering, not research.

## Roadmap

Milestones M0–M9 are complete; each exit walk is recorded in
`docs/DOC-LEDGER.md`. M0 scalar trait/arenas/harness; M1 topology +
Euler operators; M2 analytic geometry, extrude/revolve, tessellation,
STL; M3 analytic intersections, booleans, mass properties; M4 the
parametric layer, naming, STEP export; M5 NURBS depth, SSI,
constant-radius fillets; M6 the SSI generic-`T` lift, loft/sweep
assembly, composition surgery; M7 STEP import as adoption; M8 the
kernel residuals the demos raised; M9 the declared-contact join lane.

Standing outcomes that still bind:

- **Production bit-identity coincidence checking is RETIRED** (Ev, #53;
  #102). The ratified mechanism is NAMING-DESIGN N6 recipe-source
  identity — `GeomSource`: same source ⇒ same bits by D9, converse
  deliberately unclaimed. `geom_core::bit_identity`'s consumers are
  debug-only with an EMPTY production allowlist (CI tripwires stay
  armed; a new consumer must be allowlisted and carry a
  retirement-scheduled note). Undeclared value-equal flush booleans
  refuse typed at the coincidence door — declared intent is the
  supported road.
- **K = 10 is the permanent ratified default** (#89 CLOSED,
  `docs/K-REPORT.md`).

Open work is the tracker's (`work/STATUS.md`); the programs it lists
that execute ratified design here are M10 (error propagation,
`docs/ERROR-DESIGN.md`; the sketch solver is NOT in its slate and
re-opens as its own design pass when constraint-driven sketches have a
consumer), LIB (`docs/LIBRARY-DESIGN.md`), SEAT
(`docs/VERB-SEAT-DESIGN.md`) and DOCM (the two DOCM designs). The
missing modeling verbs are registered in `docs/KERNEL-VERBS.md` and
worked as tracker issues and by the kernel programs that own their
territory.

## Beyond the kernel: the usability gap

A scoping section, not a milestone plan: it names the work between "the
kernel exists" and "a person can actually use this," so that none of it
gets invented ad hoc or discovered late.

**Sequencing stance: "usable as a library" ships before GUI work.** The
kernel has parametric models, mass properties, and STEP in both
directions; language bindings (Python — the CadQuery/build123d
audience), documentation and feature breadth yield a usable code-first
tool without an interactive application. The GUI is a separate layer
and effectively a second project of comparable size to the kernel
(Fornjot's postmortem and Zoo's app-team scale are the evidence); its
architecture is `crates/viewer/README.md` — the G1 three-layer split
(kernel / headless `editor-core` / interaction), egui as the toolkit,
the ray path authoritative for picking with the id pass advisory.
Multi-select UX and filter presentation stay deferred to GUI time.

### Band 1 — kernel-side services an interactive client requires

The "any GUI is a thin client" claim is true only if the kernel exports
these. All are shipped in `editor-core` except where noted:

- **Incremental recompute**: memoized per-node evaluation keyed on
  128-bit content/naming keys (op kind, structural params, evaluated
  expression bits, upstream keys, ambient ε/K, witness), evaluation
  epochs, deterministic level-parallel scheduling; a targeted mid-DAG
  edit recomputes only its downstream cone (pinned on the corpus
  documents). Remaining: partial re-tessellation, and a resident cache
  service — today the memo is the caller-threaded prior `Evaluation`.
- **Picking back-references**: tessellation output carries per-patch
  source-`Face` and per-polyline source-`Edge` keys, and
  `editor_core::resolve::pick::pick_face` is the `ray → StableName`
  service (`bvh::Bvh::ray`, exact ray/triangle tests, a total documented
  tie-break, the `resolve::hit` inversion); `NodePick` pairs a mesh
  with its node by construction.
- **Cancelation** (`CancelToken`, yielding between nodes/levels; a
  canceled run returns the completed prefix as a typed outcome).
  Remaining: progress reporting (nothing exists) and in-op yield points
  — a long single boolean or fillet is still uninterruptible.
- **Selection stability across edits** — the user face of D5's
  persistent naming: ONE `StableName` type for recipe references and
  selections, with resolution, the diagnosis ladder, tombstones, and
  `Rebind` with suggestion affordances (offers, never auto-repair).
- **Appearance attributes**: per-face/body display attributes live in
  the document layer keyed by stable names — never arena keys — survive
  recompute via post-pass resolution and report losses loudly;
  appearance-only edits recompute zero nodes.

### Band 2 — the interactive application (a second, kernel-sized project)

Architecture ratified (`crates/viewer/README.md`) and v1 built; what v1
delivered against these is partial, and the items keep their full-size
framing because that is what they still cost.

- **Viewport**: real-time tessellation with LOD, edge/silhouette
  rendering, section views, snapping, navigation.
- **The interactive sketcher** — the largest single item: dragging,
  dimension placement, constraint inference, and visual
  over-/under-constraint feedback. Q3's ecosystem gap (no
  DOF-diagnosis solver in Rust) becomes user-facing here, converting
  that solver from optional to mandatory.
- **Feature tree UI**: rollback, reorder, suppress, edit-in-place —
  D8's recipe-as-DAG is the substrate.
- **Error UX.** D4's typed errors are correct for a kernel and brutal
  in a GUI if presented raw; `ResidualExceeded { check, sample }` must
  become "this fillet fails *here*" with the entity highlighted.
- **Direct manipulation** (drag a face → parameter change) is an
  inverse problem on top of everything above; optional except dragged
  sketch dimensions.

### Band 3 — the subsystems beyond the kernel proper

- **Assemblies — shipped at v1 scope** (`crates/editor-core/ASSEMBLY.md`):
  an assembly document is a recipe DAG of the same formalism —
  instantiate-part, mates and patterns are ordinary feature nodes, so
  the editor and solver machinery transfers unchanged; binding is
  pinned-with-explicit-update, the Cargo.lock model. Interference
  checks fall out of booleans / M10 clearance.
- **Engineering drawings.** Dimensioned 2-D drawings require projection
  plus hidden-line removal; HLR on curved B-reps is SSI-grade and
  belongs on the difficulty ranking near fillets. Near-term dodge:
  export STEP, make drawings elsewhere.
- **Feature breadth.** What the kernel has and what daily use still
  assumes (variable-radius fillets, draft, hole features, mirror,
  helixes, rib/text) is `docs/KERNEL-VERBS.md`; the long tail dominates
  "why can't I model my part."
- **Interchange breadth**: 3MF, DXF in/out, OBJ. Each small; STEP
  remains the only hard one.

### Banked principles

Cross-milestone commitments; each binds at the layer named.

- **Naming is localized to reified predicate flips.** Topology is a
  function of the recipe and can change only where a structural
  parameter (D8) changed or a trilean predicate (Q1) flipped. Within a
  flip-free parameter region, replay is history-identical and
  lineage-scoped key identity makes name resolution provably trivial;
  at a flip, the flipping predicate itself names what changed and why.
  Resolution is trivial where provable, a loud typed failure carrying
  the flip's diagnosis where not; re-binding cleverness only as
  ratified opt-in policies.
- **Content-keyed cache transfer.** D9 bit-determinism makes any
  derived artifact keyed by the bit-content of its geometric inputs
  transferable across rebuilds by equality check — the key *is* the
  correctness proof; no dirty-flag invalidation. The key shape is
  shipped (mesh back-references, the content/naming keys); a
  finer-grained per-artifact transfer service remains future.
- **Coincidence is structural or declared, never inferred from
  values.** Treating bit-equal descriptions as semantic coincidence
  would make topology hinge on an UNMARGINED predicate — a razor-thin
  equal-vs-one-ulp cliff with no escalation band, exactly what Q1
  forbids — and value equality is not evidence of intent anyway. The
  ladder: (a) **shared surface key** — coincidence explicit by
  construction; (b) equal-but-independent descriptions do **not** glue
  — if the user means flush, the recipe must say so (share the surface,
  or declare the relation); description-equality *detection* is a
  diagnostic affordance only; (c) near-coincidence between unrelated
  definitions is a typed sliver error whose resolution is an explicit
  repair/adoption operation — D7's machinery applied natively.
  Consequence: topology depends only on recipe structure and margined
  verdicts, so predicate flips remain the *only* topology-change sites.
- **The editor-core evaluation service is generic over `Real`.** M10's
  error-propagation UI rides the same memoization / cancelation /
  per-node-result machinery as f64 rebuilds; no parallel path.
- **ε and persistence.** A document records the ε it was authored
  under; the application pins the run's ε to the document's; an
  assembly whose referenced documents disagree on ε is a typed error
  (`ResolveFault::EpsilonSeam`). **Changing ε is a recorded
  `SetTolerance` document edit**: apply = replay at the new ε and
  structurally diff — the verdict-vector diff reports exactly which
  predicates changed verdict, escalations included, and any change is
  a typed error requiring explicit resolution. Same diff machinery as
  the naming pillar: ε changes and parameter changes are both "same
  recipe, different evaluation context."
- **Fillet/blend validity is reified predicates, not try-and-fail.**
  Every classic failure is a margined predicate over the inputs — r vs.
  1/κ_max of the support, r vs. adjacent-face extent, spine regularity,
  corner configuration — stated in the feature definition. Payoffs:
  typed, diagnosable pre-construction errors; the predicates are Q1
  predicates, so M10 can certify **fillet validity over a parameter
  box**; corner reconfigurations are enumerated predicate flips,
  extending the naming pillar to fillets. Same principle for
  shell/offset.
- **SSI completeness is an interval obligation, not a marching
  property.** Residual certification audits only *found* branches; the
  missed small loop is the classic silent disaster. **Marching finds,
  subdivision certifies exhaustiveness**; the outcome is "every branch
  found" or a typed failure, never silence. Certification is an
  at-rest/tier obligation; preview may march uncertified.
- **Non-manifold boolean results are typed errors** — "non-manifold"
  meaning **non-representable** (D1's representability boundary).
  Touching via distinct entities is a typed success validating at tier
  3′. Silent splitting into separate manifold bodies is rejected as
  inexplicit; any future split behavior is an explicit ratified
  operation the user invokes, never a fallback.
- **The expression sublanguage is total and finite by charter.** No
  recursion, no unbounded iteration, no user-defined functions —
  anything Turing-ish lives in the host-language generator layer (D8).
  Keeps interval/dual replay trivial; nearly impossible to claw back
  once one persisted model uses a loop.
- **Sketch DOF diagnosis is two named layers, never conflated.** The
  **structural layer** (DOF counting, graph decomposition — exact,
  combinatorial, float-free) diagnoses over/under-constraint; the
  residue — generically-well-constrained but configuration-degenerate
  sketches — is a Jacobian-rank fact at the witness, caught by GQ1's
  bifurcation-margin predicate with its own vocabulary ("degenerate
  configuration" ≠ "over-constrained"). "Solver didn't converge" is
  never reported as a diagnosis.
- **Persisted floats round-trip bit-exactly.** Shortest-round-trip
  formatting (serde_json with `float_roundtrip`) for finite values;
  NaN/inf refuse typed (`PersistError::NonFinite`); lossy formatters
  banned; enforced by a save/load/replay-identity test.
- **Flags banked**: mate solving needs witnesses/interval contraction
  on SE(3), not ℝⁿ; recipe-level provenance carries **pattern indices**
  explicitly so references into indexed families never degrade to
  positional guessing; rebuild latency is an architectural property,
  measured by the corpus and latency suites in `editor-core`.

### Band 4 — product-grade infrastructure

- **Recipe schema versioning/migration from the first RELEASED file**,
  autosave/crash recovery, and embedded derived caches so opening a
  model isn't a full rebuild. *Ruled (Ev, 2026-09-01): pre-release
  there is NO hand-maintained schema version, no migration chain and no
  bump coordination — nothing is released and every checked-in document
  is a regenerable artifact. The one door that stays: a file this build
  cannot read refuses TYPED, by the deserializer's own rejection of
  unknown or missing vocabulary wrapped in the regenerate recourse, so
  an additive vocabulary change invalidates nothing and a breaking one
  names the field. Versioning returns the day a document ships to
  someone.*
- **Performance at scale**: hundreds of features / thousands of faces;
  the parallel-evaluation story under D9's fixed reduction shapes.
- **A real-model corpus** as the usability regression suite: "these N
  parts rebuild in < T seconds with identical topology."
- **Docs and onboarding** for the API-as-product: `docs/GUIDE.md`,
  examples, and the Python bindings.

## Open questions

### Q1: Scalar genericity — settled; retained as the ratified record

- Evaluation code (evaluators, derivatives, transforms, measurements)
  is generic over a `Real` trait we define. Instantiations: `f64`,
  `Interval` (the in-house `interval-transcendentals` backend, behind
  the `interval` feature), `Dual<T>` (one in-house generic type;
  `num-dual` is a dev-only oracle because its std-backed
  transcendentals cannot satisfy the value-channel bit-identity
  contract), and `Sym`.
- Every topology-determining branch goes through a *named predicate
  function* returning a trilean sign plus margin, generic over `T`. No
  raw `<` on control-flow paths.
- At `T = f64` predicates are total: margins within K·ε escalate (the
  *sliver band* — semantically indeterminate even under exact
  arithmetic). K is a policy dial — refusal rate and f64 noise headroom
  — not a correctness parameter: soundness rests on
  escalate-never-guess, D4 ¶2 certification and interval replay, for
  any K > 1. K = 10, per-run configuration like ε (`Tolerance.k`).
- At `T = Interval` an indeterminate predicate aborts the operation:
  predicates return `Result<Sign, Indeterminate>` (the trichotomy is
  the primitive; bool predicates are projections) and construction code
  propagates with `?` to the **subdivision driver** (`editor_core::drive`,
  ERROR-DESIGN E6), which splits the parameter box and re-runs — a pure
  model makes re-running sub-boxes trivially correct and embarrassingly
  parallel. Leaves take definite branch paths; outcome probabilities
  are the distribution's measure on the sub-boxes. The sliver band is
  *terminal* for the driver: an enclosure wholly inside (ε, Kε) never
  refines.
- A persisted decision log is *dropped*: reified predicates are the
  load-bearing part.
- **`Real` trait surface**: comparison-free by construction (no
  `PartialOrd`/`PartialEq`, plus a style rule and a CI tripwire for the
  residual channels); all operations total with poison propagation
  (NaN/empty) — poison flows through *values*, never *decisions*;
  `sin_cos` is the primitive; no fused operations (`hypot`,
  `mul_add`) — cross-instantiation consistency outranks last-ulp
  accuracy; no order-implicit reductions.
- **Interval scalar**: the *decoration as the poison channel*
  (`decoration < Def ⇒ Indeterminate(Invalid)` — silent domain clamps
  never decide); `Bounds` means only "carries a bracket" and
  `CertifiedEnclosure` is the certification half, which `Dual` does not
  implement (DUAL-DESIGN: a dual may not certify — it carries a bracket
  in its value channel, but tangent transport is not enclosure);
  poison-visible NaN brackets for empty AND NaI; tight `pown`
  (containment of the true value is the interval contract).
- **`Dual<Interval>` semantics**: *value-part delegation* — `Decide`
  classifies the value only, the derivative never influences a branch
  (tangent-space data does not decide base-space topology). Kink
  conventions: f64 tangents are branch-consistent with the value
  channel (abs′(0) = +1, ties keep self); the interval instantiation
  carries the *Clarke subdifferential enclosure* (straddle hulls).
- **Genericity boundary**: `Body<T>` = scalar-free topology arenas +
  `T`-valued geometry arenas; topology contains no `T` and never
  branches on it; keys are *body-lineage-scoped* (key identity across
  same-history builds is what lets an interval replay share topology
  with the f64 build). The certified-lane pattern is **f64 structure +
  `T` payload**: candidate generation (`ssi::jet`/`march`/`system`, the
  analytic composite's implicit form) is `f64`-only and untrusted, and
  the certificate is derived and carried at `T` (`SsiCertificate<T>`,
  `PcurveFittedLane`) — so a WIDENED analytic operand refuses typed
  rather than picking a representative surface out of the family.

### Q2: Tolerance model — **resolved**, folded into D4.

### Q3: Sketch constraint solver — build vs. bind (OPEN)

No solver is in the tree. Ecosystem: **libslvs bindings** are dead and
GPLv3 (avoid); **planegcs** has no Rust bindings; **ezpz** (Zoo, MIT) is
a pure-Rust solver in production, pre-1.0 and product-driven, the
strongest option; **ISOtope** (archived, non-OSI) is the best free
writeup of the constraint-as-energy math — reference only. **Gap**: no
DCM-style graph-decomposition/DOF-diagnosis solver exists in Rust;
over/under-constrained diagnosis is ours to build regardless. Leading
answer: adopt ezpz when constraint-driven sketches have a consumer, with
"roll our own LM solver on `levenberg-marquardt`/`faer` using ISOtope's
math as tutorial" as the fallback. ezpz sits *upstream* of the certified
core (its output is numbers that then pass through our construction and
checks), so an arm's-length dependency stays principled.

### Q4: Units and model scale — **resolved**, folded into D4 (¶4) and D6.

### Q5: `curvo` — **resolved**: study + dev-dependency oracle; vendoring rejected

`docs/CURVO-AUDIT.md`. The core invariants (certified residuals,
trilean predicates, generic `T`, no hidden tolerance decisions) live
*in the algorithms*, and in every candidate routine the
invariant-relevant surface is exactly what a retrofit would rewrite;
curvo has no SSI at all. Oracle scope, pinned at the audited commit:
evaluation/derivatives/basis/degree-elevation/interpolation; not
bit-exact (std-routed math); not SSI/booleans (opencascade-rs/truck
remain those oracles).

### Q6: Recipe representation — **resolved**, promoted to D8.

### Q7: Determinism policy — **resolved**, promoted to D9.

### Q8: Definitional vs. approximating surfaces — **resolved**, OFFSET-DESIGN O1–O6.

### Q9: Project license and name

License **resolved**: dual MIT OR Apache-2.0. Name: pending
(`docs/NAME-CANDIDATES.md`); placeholder workspace acceptable,
pre-publish renames are cheap.

### Before publishing (listed so they don't get lost)

Things deliberately in a pre-publication state; nothing goes red when
the project publishes with one still in the shipped state.

- **Roll the version numbers.** No member manifest carries a `version`
  field, so every crate is cargo's default `0.0.0`, and
  `[workspace.package]` says `publish = false` until the project has
  its name (Q9). Rolling them back is what un-does a premature publish.
- **Turn release debug-assertions back off.** The root `Cargo.toml`'s
  `[profile.release]` sets `debug-assertions = true`, so a release build
  runs every row-5 postcondition. That is the right posture for a
  kernel nobody depends on yet; deleting the stanza is a real reduction
  in what a release build checks, so it is a decision to take at
  publish rather than a chore (**S65**, `work/code-quality/S65.md`, is
  the worked example).
- **The name (Q9).**

## Crate landscape

Dependencies are for the *substrate*, not the modeling core.

| Area | Crate | License | Status |
|---|---|---|---|
| ID arenas | `slotmap` | Zlib | **Adopted**: typed keys per entity kind, `SecondaryMap` for attributes |
| Persistent collections | `imbl` (or `rpds`) | MPL-2.0 / MIT | candidate only — nothing has needed it. `im` is unmaintained with an open soundness advisory; use the `imbl` fork if ever adopted |
| Interval arithmetic | `interval-transcendentals` (in-house) | MIT/Apache | **Adopted** as the `T = Interval` backend: proven per-function libm error pads, MPFR-differential-certified via the optional `oracle-inari` feature of its own workspace, libm-only, D9-clean. **The kernel is copyleft-free in every build configuration**: `inari` and its gmp/MPFR LGPL stack appear nowhere in the root lockfile, and the hosted `interval-backend` CI row runs the crate's fast suites gmp-free and tripwires the graph |
| Robust predicates | `robust` (georust) | MIT/Apache | nothing of ours calls it; rides in transitively under `spade` |
| Dual numbers | `num-dual` | MIT/Apache | dev-only derivative oracle; duals are the in-house `Dual<T>` (Q1) |
| CDT / mesh refinement | `spade` | MIT/Apache | **Adopted** (`mesh`): Delaunay + constrained + Ruppert refinement in UV space; exterior classification is ours (even-odd flood fill), spade supplies the CDT only. Insertion is quadratic for faces bounded by nested near-cocircular loops (`mesh` §Performance) |
| Serialization | `serde` + `serde_json` | MIT/Apache | **Adopted** (persistence); the `float_roundtrip` feature is LOAD-BEARING. Kernel crates stay serde-free (`scripts/gates/kernel-serde-free.sh` checks the dependency edge of every manifest; `profile` is additionally sealed by `profile/tests/seal.rs`). Where a kernel type must persist, its bytes are described above the boundary, not by a mirror enum (CONTACT-DESIGN C4) |
| 2-D polygon booleans | `i_overlay` | MIT/Apache | candidate only (present in the lockfile only transitively under the curvo oracle's `geo`) |
| Display triangulation | `earcut` (georust) | MIT/Apache | candidate only (same) |
| Sketch constraints | `ezpz` (Zoo) | MIT | not adopted; see Q3 |
| STEP | `truck-stepio`/`ruststep` | Apache | dev-dependency parse-back oracles for the in-house AP214 writer only — ruststep cannot write STEP; truck-stepio's writer has unfixable conformance defects |
| GUI toolkit | `egui`/`eframe`/`egui-wgpu`, `winit`, `egui_tiles`, `bytemuck`, `rfd` | MIT/Apache | **Adopted** (`viewer`), every entry optional behind the non-default `app` feature so the toolkit graph never enters a kernel PR's compile closure |
| Python bindings | `pyo3` | MIT/Apache | **Adopted** (`pncad-py`), `abi3-py38`, optional so a kernel build never links Python |
| Hashing | `sha2` | MIT/Apache | **Adopted** (`editor-core`): content pins are SHA-256 over canonical semantic bytes (ASSEMBLY-DESIGN A4) — the pin IS version identity; the in-process FNV `ContentKey` stays a separate, weaker vocabulary |
| OS randomness | `getrandom` | MIT/Apache | **Adopted** for interactively-authored document ids in `pncad` (and the wasm32 backend in `viewer`); `editor-core` stays deterministic by construction |
| NURBS oracle | `curvo` (git-pinned) + `nalgebra` | MIT / Apache | `geom` DEV-dependency at the audited commit; never a runtime dependency (Q5) |

Reference-only (read, don't depend): **truck** (the only living Rust
B-rep kernel; booleans demo-grade), **vcad** (young Apache-2.0
half-edge kernel), **Fornjot** (archived June 2026 after multiple
rewrites without robust booleans — required reading as a postmortem),
**opencascade-rs** (the only production-grade-boolean route in Rust;
LGPL + C++ build tax; a *test oracle* for boolean results).

## Prior art / references

Local copies live in `references/` (git-ignored): the NURBS Book (2nd
ed.), Mäntylä complete, Hoffmann complete, the GSD06 discrete
differential geometry notes, and Vida–Martin–Várady 1994 (the canonical
blending survey; source for the fillet scope-boxing and D2's
`TangentIntersection` treatment).

- **Mäntylä, *An Introduction to Solid Modeling*** — the
  Euler-operator B-rep reference; `topo` is essentially this book (with
  the mirrored orientation hazard noted under D1). One erratum on
  record: Program 11.6's `lmev` `addhe` order (PLUS-half first) breaks
  both `he1 == he2` cases; MINUS-first is coherent.
- **Hoffmann, *Geometric and Solid Modeling*** — intersections,
  robustness.
- **Piegl & Tiller, *The NURBS Book*** — canonical NURBS algorithms.
- **Grinspun–Schröder–Desbrun, DDG course notes** — the
  discrete-exactness philosophy framing how stackup design treats
  kinks/subdifferentials (Q1).
- **Fornjot** and **truck** — the two serious Rust B-rep attempts;
  study the topology/geometry split.
- **Open CASCADE** source — a catalog of what every subsystem must do,
  and a cautionary tale on tolerance philosophy.
- **Parasolid XT format spec** — the cleanest picture of a production
  kernel's data model.
- **Shewchuk's robust predicates** + CGAL literature — for the
  polyhedral/predicate corners where exactness is achievable.
