# Assembly Design — instances, mates, and the document seam

**Status: DRAFT — design conversation live at #328; awaits Evan's
sign-off (design-ratification class, never self-merged).** This
document ratifies Band 3's assembly architecture inside the frame
already settled elsewhere, and it deliberately re-litigates none of
it:

| Settled elsewhere | What it fixes here |
|---|---|
| GUI-DESIGN GQ4 (ratified 2026-07-19) | Assembly documents are recipe DAGs of the same formalism; instantiate-part via the (document identity × local ref) wrapper; mates/patterns are ordinary feature nodes; binding is pinned-with-explicit-update (the Cargo.lock model) |
| CONTACT-DESIGN C1–C8 (#178) | The contact/fit vocabulary: `Rest`/`Tangent`/`Fit` classes, the signed gap, interference semantics; C4 names "assembly-era relation/mate nodes" as the declarations' second home — this doc is that home |
| SOLVER-DESIGN W8 | The witness contract (branch selection, certified chart-box uniqueness, typed bifurcation) transfers verbatim to mates; the SE(3) *mechanism* stays undesigned until the numeric-mate era (A1 rung (c)) |
| NAMING-DESIGN N1–N7 | Names are derivation paths; this doc discharges its scope exclusion "assembly pin representation" (A4) |
| DESIGN.md banked flags | SE(3) witnesses budgeted, not assumed from sketch machinery; ε-disagreement across referenced documents is a typed error at the seam; pattern indices ride provenance explicitly; free-move/hide are display-layer state, never persisted (G3) |

Decisions are **A1–A8**; open questions **AQ1–AQ6**. Design-only:
no unit here is scheduled by this doc; the implementation ladder
names natural homes (M8's #317, M9's C7, M10's clearance) without
claiming them. Chat/issue rulings incorporated: the scope ladder,
materialized evaluation, and the key-identity relaxation are Evan's
calls of 2026-08-09/10 (#328).

## A1 — The scope ladder

Four rungs, ratified as *sequence*, with only (a)+(b) in the v1
design's implementation scope:

- **(a) The body graph**: instances of pinned part documents with
  per-instance rigid frames; patterns and mirror as the same family.
- **(b) Declared contacts/fits between instances**: mate nodes
  carrying CONTACT-DESIGN's classes, solved **constructively**
  (frame-composition chains). A mate system that is not
  constructively solvable — genuinely simultaneous constraints —
  refuses typed, naming the cycle (AQ3 pins the exact boundary).
- **(c) Numerically solved mates**: the W8 mechanism (SE(3)
  witnesses, chart-centered Krawczyk and its proof obligation) —
  its own design-and-implementation era, after the sketch-solver
  era, not before.
- **(d) Kinematics/articulation** (under-constrained assemblies
  that move): beyond (c). CONTACT-DESIGN C8's deliberate exclusion
  stands until then.

The v1 line between (b) and (c) is honest for real assemblies:
most placement chains are fully determined mate-by-mate (fastened,
coaxial-plus-flush, frame coincidence), which is composition, not
root-finding. What v1 loses — simultaneous systems — refuses with
recourse text pointing at rung (c).

## A2 — An assembly is a document; its evaluation is a body

**The uniformity principle, both halves.** An assembly document is
a recipe DAG of the same formalism (GQ4 — nodes are features:
instantiate-part, mate, pattern; edits are recorded `DocEdit`s;
naming, suppression, undo transfer with zero new machinery). And
its *evaluation product* is the same kind of value as any
document's: **one kernel `Body`, generally multi-solid and
non-connected** (F8 already makes disjoint multi-shell bodies
tier-2-legal). `build(params) → Result<Body, ModelError>` extends
to assemblies verbatim; every existing consumer — tier gates, mass
properties, census, tessellation, export — takes the result with
no new kernel vocabulary. (Evan's framing, ratified in the #328
conversation: *inside the kernel, an assembly is the exact same
thing as a non-connected body; the split into files is metadata.*)

Consequences, all normative:

- **Evaluation materializes.** Instantiating a pinned part document
  evaluates it (memoized, at its own parameters) and materializes
  each instance into the assembly body via `transform_rigid` with
  full re-certification of every carrier — the same door the STEP
  importer already uses: the recipe proposes, the kernel disposes.
- **The kernel gains no assembly type.** Which solids are "the same
  part," which frames placed them, which mates hold — all of it is
  recipe structure plus D5 provenance/naming, never body state.
  This is the central commitment doing its job: nothing about a
  body is true that is not derivable from its construction.
- **Instanced/lazy evaluation is a banked optimization, not
  semantics.** O(N) materialization (the FW13 case: ~116
  placements) is accepted for v1 and named here. Because a body is
  never authoritative (D1), a shared-representation evaluation
  strategy — one certified part body + N frames, world-space caches
  content-keyed by (part content hash, frame bits) — can be
  introduced later with no observable change. It is legal precisely
  because A2 fixed the semantics first.
- **Fusing is explicit.** When one connected solid is genuinely
  wanted (export a weldment), that is a cross-instance boolean node
  in the recipe — C7's declared-contact join lane — never an
  implicit consequence of assembly evaluation. Which product you
  have is stated in the recipe, not implied by layer. This also
  retires the M5-LOG frontier note that `Boolean(Union)` is the
  recipe layer's only multi-solid expression: `InstantiatePart` is
  the intended front door for "N solids in one document," and
  disjoint union stays what it is — a boolean, with boolean
  semantics.
- **ε at the seam**: the assembly document pins its ε; an
  instantiated document whose recorded ε disagrees is the
  already-ratified typed error, enforced at instantiate-node
  evaluation.

## A3 — The node vocabulary; mates are declarations

Three node kinds, all ordinary feature nodes:

- **`InstantiatePart { pin, placement }`** — pin per A4; placement
  is either an explicit rigid frame or derived from mates (below).
  Improper frames (det = −1) are admitted per A6.
- **`Mate { a, b, class, alignment }`** — `a`/`b` are
  instance-qualified stable references (the GQ4 wrapper composed
  with NAMING-DESIGN local names); `class` is **exactly
  CONTACT-DESIGN's declaration vocabulary** — `Rest`, `Tangent`,
  `Fit { g₀ }` — plus the frame-alignment data (which frames
  coincide, axis senses, clocking). One node kind carries both the
  placement constraint and the contact declaration; there is no
  second vocabulary to keep synced. This is C4's promised second
  home, landing.
- **`Pattern`** — indices → frames at the D8 structural level
  (count is a structural parameter; indices ride provenance
  explicitly per the banked naming flag). Mirror is a pattern whose
  frame is improper (A6). Native patterns and import instancing are
  one family (KERNEL-VERBS' claim, now binding).

**Constructive solving (v1).** A mate chain defines each instance's
frame as the composition that makes the mated frames coincide.
Consequence worth stating as a derived property: the solved
placement satisfies its mate's coincidence **by construction** —
Δc ≡ 0 identically as parameters vary — which is the
cross-document analog of structural carrier sharing, so
CONTACT-DESIGN C5's differentiability-at-the-operating-point result
transfers to constructively mated assemblies for free. (Numerically
solved mates, era (c), get the witness-certified version instead.)

**Declaration minting.** Evaluation carries each mate's declaration
into the evaluated body's **contact record set** — the same
currency as the boolean 3′ wrapper's records, deliberately
identical so every downstream consumer (tier-3′ validation, C6
gates, M10 clearance) needs no adapter. Declarations are verified,
never trusted: per-class verification tables (C2) run against the
materialized geometry, and a mate whose declared class does not
hold is a typed refusal, not a repair.

**The declared rung, cross-document.** Two separately-authored
parts cannot share carriers structurally; a peg r = 5 mm in a bore
r = 5 mm is value-equal by authoring, and the coincidence ladder
forbids value-equality gluing. The mate declaration is what
licenses the coincidence — declared-and-verified, trilean (definite
mismatch refuses; in-band escalates). A *designed* clearance is
authored `Fit { g₀ }`, never a `Rest` that happens to pass (AQ6
pins the verification detail).

## A4 — Identity, pins, and the split/inline pair

- **Document identity ≠ pin.** The cross-document wrapper carries
  `(stable document id, content-hash pin)`: the id answers "which
  part," the pin answers "which version of it." D9 bit-determinism
  is what makes the pin well-defined — the hash of the referenced
  document's canonical serialized recipe bytes. Human-readable
  version labels are affordance metadata, never semantics.
- **Cargo.lock semantics** (GQ4, ratified in direction; detailed
  here): an assembly is a self-contained reproducible value. Edits
  to a referenced document never retarget an assembly; **"accept
  updated version" is a recorded `DocEdit`** that moves the pin,
  and the mate re-verification (A3) runs at that edit — the swap
  succeeds iff every crossing declaration re-verifies against the
  new geometry. That is the "does it actually fit" gate, and it is
  verified, never assumed.
- **Split and inline are first-class recorded refactorings.**
  Splitting a subtree out of an assembly cuts the recipe DAG; every
  mate edge crossing the cut becomes the interface record in the
  remainder — (pin, wrapped name, declaration) — i.e. **the seam is
  the crossing declarations**. Inline is the inverse. Acceptance
  (Evan's ruling, 2026-08-10): split-then-evaluate ≡ unsplit
  evaluation at **structural + name-resolution identity** — same
  topology and geometry semantically, every stable name resolving
  identically; arena-key/bit identity is not required (matching
  D9's per-arena convergence precedent), taken if the clean
  implementation gives it.
- **Swapping a different part into a seam** is pin-retarget +
  re-verification, same machinery as update; name resolution across
  the swap is N1–N7 unchanged — resolve, or fail loudly with the
  diagnosis ladder and `Rebind` affordances.

## A5 — Validity: what the at-rest gate means for an assembly

- **Disjoint assemblies validate today**: the evaluated body is a
  multi-shell tier-3 body; per-solid checks run per D7's
  per-solid-not-per-file note.
- **Touching assemblies are tier-3′ currency**: mate-minted records
  + census + two-directional certification, exactly the boolean 3′
  shape. The per-class predicates are shared substrate with C7
  (M9). **Named wiring so it is not lost (the #328 scoping trap):
  M9's C7 as slated is the *join lane*; an assembly at rest needs
  census + verification with no boolean — an at-rest door, not a
  zip.** Same substrate, different door; the C7 spec should adopt
  it deliberately as a sibling deliverable or it becomes a silent
  gap. Cross-instance census cost is quadratic in entity pairs;
  `bvh` pruning is the intended engineering answer (the tree
  prunes, exact predicates decide).
- **Interference fits**: representable as overlapping shells,
  valid only through C6's recorded gate-skips
  (`OverlapUncorrected`, opt-in certified lens subtraction). C6 has
  no single-body form — it is *defined over* this representation —
  so it lands as the assembly-era feature with no adapter, in the
  M9/M10 era.
- **Undeclared contact between instances is a hard error**, never
  blessed — F1's scan-to-bless ban applies across the document seam
  exactly as within it.
- **The connectedness lint** (LONGTERM-IDEAS I1(0b), Evan
  2026-08-10): warn when a body has more disconnected components
  than its structure expects; assembly/file-split structure is the
  natural expectation source. Advisory lane, not a gate; recorded
  here as a consumer this design enables.

## A6 — Mirror and improper frames

det = −1 instance frames are admitted, riding machinery that
already shipped: the M5 S12 face-orientation bit plus M6's ratified
curved sense-flip gate (#223) — every face's outward normal negates
exactly once, by the per-surface-kind encoding each already has.
The equivariance convention (D9 conv. 4) is the design frame, and
its UNAUDITED premise becomes a **named prerequisite of the mirror
implementation unit** — audited then, not assumed here. STEP
residue recorded, not solved: import refuses det = −1 by ratified
choice (M7-4); export of mirrored instances needs a policy line
when assembly-structure export exists (A8).

## A7 — The leave-room register

Cheap to honor now, expensive to retrofit; each names its source:

- **Volume queries against the assembly** (LONGTERM I1(c)'s
  explicit instruction): nothing in A2 privatizes world-space
  geometry — the materialized body is exactly what swept-volume /
  clearance-corridor queries want; keep it that way.
- **Thermal re-check** (I1(b)): per-part CTE scaling then re-run
  mate verification — possible because declarations are recipe
  data re-verifiable at any evaluation; do not cache verification
  verdicts into pins.
- **In-context modeling** (GQ4's named future consumer): a part
  referencing an assembly neighbor's face arrives as the
  captured-context object — wrapper-plus-pin again, held by the
  part document. Nothing here forecloses it (AQ5).
- **Handbook fits** (I3): `Fit { g₀ }` with g₀ from a versioned
  data source — the declaration shape already fits; provenance of
  g₀ is the open half.
- **#317's record shape**: the M8 import-flatten unit must record
  the NAUO instance structure (which product, which placement, per
  instance) in the import record, so a flattened import can later
  be re-adopted as an assembly document without re-parsing. This is
  the one demand this doc places on an already-adopted unit.

## A8 — Interchange posture

- **Import**: #317 flattens to the multi-solid body (correct under
  A2 — that *is* the evaluation product), with the A7 record shape.
  Import-as-assembly-document (NAUO → `InstantiatePart` nodes,
  pins into a generated part document) is the later, richer door;
  its natural trigger is the document-store decision (AQ1).
- **Export**: STEP AP214 as shipped carries no assembly structure;
  declarations/fits are dropped on export today (CONTACT-DESIGN's
  honest note) and solid grouping does not round-trip
  (`kiss_assembly`). Assembly-structure export (AP242 or AP214
  NAUO) is future breadth with no owner; this doc only requires
  that the recipe retains everything such an exporter would need —
  which A2/A3 give by construction.

## Open questions

- **AQ1 — the document store.** What a stable document id anchors
  to, and where referenced documents live (filesystem paths,
  workspace manifest, registry-shaped). Gates import-as-assembly
  and any sharing story; does not gate rungs R1–R2 (single-store =
  the workspace).
- **AQ2 — update granularity and conflict surfacing** under the
  Cargo.lock model: whole-document pin bump only, or
  per-reference; how competing updates in one assembly surface.
- **AQ3 — the constructive-solve boundary.** Tree-shaped mate
  chains are clearly in; the typed refusal's exact line — cycles,
  redundant-but-consistent mates, partially-constraining mates
  whose composition is still determined — needs its own working
  session, with the refusal naming the cycle.
- **AQ4 — per-instance overrides.** v1 posture: an instance is pin
  + frame, nothing else — no per-instance parameter overrides, no
  per-instance suppression beyond GQ2's failure semantics.
  Overrides are a deliberate future door, not an accident.
- **AQ5 — in-context capture semantics**: the captured-context
  object's pin/update behavior (what exactly is captured; when it
  goes stale; how staleness surfaces).
- **AQ6 — cross-document `Rest` verification detail**: the trilean
  shape for value-equal-by-authoring carriers (peg/bore radii),
  and the recourse text steering designed clearance to
  `Fit { g₀ }`. Belongs with the C7-era verification-table specs.

## Implementation ladder (homes named, nothing scheduled)

- **R0 (M8, already adopted)** — #317 import flattening, with the
  A7 record shape as its one new obligation.
- **R1 — the body graph**: `InstantiatePart` (pin + explicit
  frame) and `Pattern` nodes; materialized evaluation through
  `transform_rigid`; instance-qualified naming; pins and the
  split/inline pair; disjoint-assembly validity (all-shipped
  substrate — this rung has no kernel prerequisites).
- **R2 — mates, constructively**: `Mate` nodes, frame-chain
  solving, declaration minting, planar contact verification (the
  census inventory that exists); typed refusal on simultaneous
  systems.
- **R3 (with M9/C7)** — curved contact verification at rest (the
  A5 sibling door), then C6 interference gates and the mass-props
  refusal/subtraction (M9/M10 era).
- **R4 (banked)** — instanced/lazy evaluation as pure optimization;
  mirror implementation (equivariance audit as prerequisite);
  import-as-assembly-document (post-AQ1).
- **(c)/(d) eras** — numeric SE(3) mates (W8 mechanism + proof
  obligation), then kinematics. Out of v1 by A1.
