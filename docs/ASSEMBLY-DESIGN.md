# Assembly Design — instances, mates, and the document seam

**Status: RATIFIED (Evan's lgtm on PR #333, 2026-08-10; design
conversation of record: #328).** This document ratifies Band 3's assembly architecture inside the frame
already settled elsewhere, and it deliberately re-litigates none of
it:

| Settled elsewhere | What it fixes here |
|---|---|
| GUI-DESIGN GQ4 (ratified 2026-07-19) | Assembly documents are recipe DAGs of the same formalism; instantiate-part via the (document identity × local ref) wrapper; mates/patterns are ordinary feature nodes; binding is pinned-with-explicit-update (the Cargo.lock model) |
| CONTACT-DESIGN C1–C8 (#178) | The contact/fit vocabulary: `Rest`/`Tangent`/`Fit` classes, the signed gap, interference semantics; C4 names "assembly-era relation/mate nodes" as the declarations' second home — this doc is that home |
| SOLVER-DESIGN W8 | The witness contract (branch selection, certified chart-box uniqueness, typed bifurcation) transfers verbatim to mates; the SE(3) *mechanism* stays undesigned until the numeric-mate era (A1 rung (c)) |
| NAMING-DESIGN N1–N7 | Names are derivation paths; this doc discharges its scope exclusion "assembly pin representation" (A4) |
| DESIGN.md banked flags | SE(3) witnesses budgeted, not assumed from sketch machinery; ε-disagreement across referenced documents is a typed error at the seam; pattern indices ride provenance explicitly; free-move/hide are display-layer state, never persisted (G3) |

Decisions are **A1–A13**; open questions **AQ1–AQ8** (AQ3
discharged by A11; AQ7 by A12; AQ2 by A13). This doc schedules nothing; the implementation
ladder below names homes, and the program that worked through it is
the ASM program, CLOSED at v1 scope — its done-state of record
is `docs/ASM-EXIT-WALK.md`. Chat/issue rulings
incorporated: the scope ladder, materialized evaluation, and the
key-identity relaxation are Evan's calls of 2026-08-09/10 (#328).

## A1 — The scope ladder

Four rungs, ratified as *sequence*, with only (a)+(b) in the v1
design's implementation scope:

- **(a) The body graph**: instances of pinned part documents with
  per-instance rigid frames; patterns and mirror as the same family.
- **(b) Declared contacts/fits between instances**: mate nodes
  carrying CONTACT-DESIGN's classes, solved **constructively**
  (frame-composition chains). A mate system that is not
  constructively solvable — genuinely simultaneous constraints —
  refuses typed, naming the cycle (A11 pins the exact boundary).
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

- **`InstantiatePart { pin, interface }`** — pin per A4;
  `interface` is the split seam's crossing declarations (A4).
  Placement is not a field of the node: A11 rule 2 carries it on
  the placement CLUSTER as document data, and a lone unmated
  instance is the singleton case of that registry. Improper
  frames (det = −1) are admitted per A6.
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
  zip.** Same substrate, different door; the at-rest door is M9-2's
  own deliverable beside the join lane, not a corner of it.
  Cross-instance census cost is quadratic in entity pairs;
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
- **Fastener relation bundles** (Evan, #356): a modeled bolt
  closes physical loops through several contacts (shank coaxials,
  head rest, thread fit) — a predefined fastener relation is a
  compound declaration TEMPLATE over the existing mate vocabulary
  (the I3 handbook-fit shape composed with SEL2's detect/declare
  precedent for candidate generation). Sugar, never semantics:
  each expanded declaration is an ordinary A3 mate, so nothing in
  A11 changes; the bundle just authors the loop's declarations in
  one gesture and names their shared provenance. The bundle is
  also the intended home of the CROSS-CHAIN COUPLING (Evan, #356):
  when two chains of parts determine the same quantity (bolt
  shoulder vs gasket stack), by-construction agreement means the
  definitions SHARE A PARAMETER — cross-document parameter
  sharing, which v1 deliberately lacks (A4: parts are
  self-contained; the seam carries pins + declarations only). So
  in v1 the agreement is verified, not constructed — the declared
  rung's trilean, consistent with the coincidence ladder — and
  the fastener relation, when designed, carries the shared
  dimension as template data rather than punching a parameter
  hole through the document seam.
- **Handbook fits** (I3): `Fit { g₀ }` with g₀ from a versioned
  data source — the declaration shape already fits; provenance of
  g₀ is the open half.
- **#317's record shape**: the import-flatten unit must record
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

## A9 — Relative freedom is component structure

**(Evan, chat 2026-08-10, post-ratification addendum; graph
simplified same day — no derived graph, the recipe DAG itself.)**
The document's ordinary recipe DAG already partitions into disjoint
connected components: a `Mate` references both its instances, a
`Pattern` references its instances, and v1 explicit frames are
literal data creating no edges. **"Relatively unconstrained" means
exactly: in different connected components of the recipe DAG** —
decidable from recipe structure alone, no solver, no geometry
inspection, no second graph to keep synced. The general principle:
*any* reference path between two instances means some authored
relationship would be silently contradicted by an independent drag
— a cross-instance boolean (a weldment's operands are not freely
orientable) and a shared driving parameter (consistent with the
ratified drag-refusal for expression-driven values) both rightly
connect. The gathering of a
document's sub-DAGs into one evaluated `Body` is A10's root
gather — document data, not a DAG node — so connectivity over the
plain recipe DAG needs no carve-out.

- **G3's free-move binds to this partition.** The one
  live-editing-ish feature the v1 GUI supports — freely orienting
  parts relative to each other — acts on whole components: between
  components, orientation is free; within one, relative pose is
  mate-derived, so a drag moves the whole component or refuses.
- **Anchor frames are never erased** (Evan's ruling). Each component
  keeps its authored explicit frames as ordinary recipe data: the
  component partition is what makes relative freedom *derivable*,
  not an absence of frames. Evaluation therefore stays A2's one
  D9-deterministic `Body`, and everything needing definite world
  coordinates — A5/F1 undeclared-contact checks, export, mass
  properties, A7's volume queries — is untouched.
- **Probe vs. commit.** The GUI's free-orient transform is
  display-layer state per G3 (never persisted, visually
  distinguishable from mated placement). Committing a probed pose,
  if ever wanted, is an ordinary frame-edit `DocEdit` through the
  existing edit door — no new machinery, and it is a *different
  act* than probing.
- **Free consequence**: the A5 connectedness lint gets its
  expectation source — the component count is the expected
  disconnection count.

## A10 — Explicit product roots (strict)

**(Evan, chat 2026-08-10.)** Shipped documents have NO product
notion: every node evaluates, and every consumer addresses a node
id per call. A10 gives documents one. The document tracks an
**ordered list of product roots** (node ids) as document data —
never a DAG node — maintained by recorded `DocEdit`s.

- **Invariants.** *Coverage*: every node is ancestor-of-or-equal-to
  some root — every connected component carries at least one root;
  no silently dead subgraphs. *Ancestor-freedom*: no root is an
  ancestor of another (listing an extrude and its fillet would
  gather the same material twice — typed refusal). Sibling roots
  in one component (extrude + revolve of a shared profile) are
  legal; they are simply not relatively unconstrained (A9).
- **Strictness is burden-free by automatic maintenance**: a node
  inserted with no consumers becomes a root; a node consuming
  existing roots replaces them in the list (tip transfer — a
  mid-authoring component carries its root until it is joined into
  another DAG); deleting a root re-roots its orphaned sinks. All
  recorded; explicit designate/undesignate edits override the
  defaults. (Maintenance details are the patchable part of this
  decision.)
- **The product** is the deterministic gather, in list order (D9
  ordering for free), of every body-denoting root: `Body` → its
  solids; `Instances` → N placed solids; `Split` → its pieces (a
  mold document wants both halves). Roots need not be body-valued
  — a WIP profile tip or a datum contributes nothing to the `Body`
  product — and a door that needs a body refuses typed when no
  root denotes one.
- **This is A2's gather and C-register disposition C1's
  resolution**: the shipped `Node::Pattern` keeps its
  `ValuePayload::Instances` semantics and its "patterns do not
  implicitly union" truth — the root gather is what materializes
  an `Instances`-valued root into placed solids of the one product
  `Body`, with no boolean implied. The single-node export door's
  multi-body refusal stays correct; the whole-document product is
  the door that accepts them.
- **A9 consequences**: free-orient units are components, now
  explicitly represented by their roots; the connectedness lint's
  expected component count reads off the root/component structure.

## A11 — The constructive-solve boundary (discharges AQ3)

**(The AQ3 working session, 2026-08-10; REVISED same day per
Evan's #356 review — placement moved to the cluster level so
anchor-count errors are unrepresentable, and mates split into
determining/declaring so redundant-consistent loops verify instead
of refusing.)** The v1 line between rung (b) and rung (c), stated
so it is **decidable purely structurally** — graph shape plus a
per-class table over authored alignment data; no geometry
inspection, no numerics beyond decided predicates. Five rules:

1. **Per-pair combination.** Each mate class + alignment datum
   pins the pair's relative pose to a coset of an SE(3) subgroup
   (frame coincidence → the identity; coaxial → rotation-about ×
   translation-along the axis; planar rest → the planar group;
   etc.). Multiple mates on ONE instance pair combine by exact
   coset intersection — a small closed-form table over the class
   pairs. Outcomes: DETERMINED (a point — coaxial-plus-flush-plus-
   clocking, A1's own example), UNDER (a positive-dimensional
   subgroup survives), CONTRADICTORY (empty — two flushes at
   different offsets) → typed refusal naming the pair and the
   clashing declarations.
2. **Placement lives on the cluster, not the instance.** Define
   PLACEMENT CLUSTERS = connected components of the instance–mate
   graph (the finer partition inside A9's DAG components). Each
   cluster's rigid frame is one entry in the component record the
   A10 registry already keeps — instances carry no explicit frame
   of their own; a lone unmated instance is the singleton case of
   the same field. Consequences: zero-anchor and multi-anchor
   states are **unrepresentable** (Evan's ask); joining two
   clusters with a new mate is ONE recorded edit (the surviving
   cluster keeps its frame, the absorbed cluster's frame is
   consumed into the edit record — undo restores it); deleting a
   mate that splits a cluster mints the new cluster's frame from
   the solved relative pose at the edit, recorded and
   deterministic — the A10 maintenance pattern applied to frames.
3. **Gauge.** The cluster frame places the cluster's GAUGE
   instance: its earliest instance in document order — a
   deterministic convention, not stored data. An edit removing the
   gauge instance rewrites the stored frame by composing with the
   already-solved relative pose, so every surviving instance's
   world pose is unchanged (recorded rewrite, same pattern).
   Pattern-placed instances are gauge-ineligible: their poses are
   the pattern's.
4. **Determining vs declaring mates.** In each cluster, take the
   deterministic spanning tree of the mate graph rooted at the
   gauge (document-order tie-breaks). TREE mates DETERMINE: each
   must be fully-determining after per-pair combination; an UNDER
   tree edge refuses typed, naming the pair and the residual
   freedom in class vocabulary ("clocking about the shared axis"),
   recourse = add the complementary mate, or delete the mate if
   free relative motion was intended (A9 is relative freedom's
   home). Pattern-placed instances are never tree CHILDREN (the
   pattern already determined them; they may be tree parents).
   NON-TREE mates DECLARE: they determine nothing and are carried
   to evaluation as pure contact declarations, verified against
   the solved geometry by the C2 tables — trilean, definite
   mismatch refusing and naming the mate AND the loop it closes.
   **No fact is ever entered twice** (the #356 conversation's
   sharpening): A11 never asks the author to restate a relation —
   every non-tree declaration corresponds to a DISTINCT physical
   contact that A5 already obligates declaring (modeled bolts, box
   corners, patterned stud stacks). When those distinct contacts
   form a loop, two chains of real part geometry determine the
   same quantity — and verifying the loop-closers is a free
   consistency check across the part documents' own dimensions,
   not a second copy of anything the author wrote.
   So: a pattern-stacked run of identical bricks mated
   stud-to-tube VERIFIES rather than over-determines; two
   explicitly-placed parts declared flush is a fit-check, not an
   error; and a redundant-but-consistent loop (A–B and B–C
   fastened, A–C also declared at the composed transform — the
   third mate adds no placement information) closes through
   verification, not solving. An INCONSISTENT loop dies at
   verification of its closing mate. No cycle is ever SOLVED;
   genuinely simultaneous systems — no spanning tree makes every
   tree edge determining — surface as the named UNDER refusals
   with rung-(c) recourse. (Tree choice is observably irrelevant
   when loops are consistent; when not, the refusal names the
   whole loop, so the citation does not depend on it.)
5. **Evaluation.** Frames compose topologically outward from the
   gauge along the tree (uniqueness structural — D9 free);
   Δc ≡ 0 by construction for tree mates (C5 transfer, A3);
   declaring mates mint their records like any declaration (A3)
   after verification.

**Member vocabulary (rider, ratified 2026-08-23 — Evan's
approval in-session; the #945 conversation)**: a mate reference
head is a live `InstantiatePart` OR a pattern-placed instance —
the Pattern node with its `Instance(i)` qualifier. A
pattern-placed member's frame is its pattern-derived pose (rule
3), and rules 3–4 bind as written: gauge-ineligible, never a
tree child, so a mate never gives a pattern instance a pose
apart from its siblings. Rule 1's combination is unchanged — the
member frame is an ordinary frame conjugated through the derived
offset. Under rule 2 a mate to `Instance(i)` joins the other
member into the pattern's cluster; a second tree mate from a
sibling instance closes a loop and is therefore non-tree —
declaring, verified (the stud-stack behavior rule 4 already
promises). Two pins: **mates never solve pattern parameters** —
a seat satisfiable only at a different spacing is CONTRADICTORY
with the measured clash, recourse = edit the parameter
(parameter back-solve is rung (c)); and **the canonical spelling
is `Instance(i)` heads** — the pattern consumed its master's
root (A10), so the master's faces are `Vanished` and a mate
naming them refuses honestly.

Named v1 losses, honest and banked by A1: cross-edge cancellation
(UNDER tree edges whose composition would determine) refuses to
rung (c); loop CERTIFICATION beyond the C2 verification tables
(e.g. in-band closures) escalates per C4 rather than resolving.

## A13 — Update granularity (discharges AQ2)

**(Sign-off 2026-08-16, PR #544.)** The Cargo.lock model's update
door, four clauses:

- **The primitive is per-reference**: `DocEdit::UpdateReference
  { node, new_pin }` — recorded, undoable, naming its node. Two
  pins of one document id in one assembly is representable and
  sometimes intended state (staged migration), and the primitive
  an update-all elaborates into must exist regardless.
- **Whole-document update is an elaboration, never a second
  primitive**: "update id X everywhere" records one
  per-reference edit per site, atomically grouped the way
  split/inline group theirs (purity = atomicity, the ASM-4
  precedent).
- **Competing pins surface as a lint, not a refusal**: mixed
  (id → {pin₁, pin₂}) state is legal at the recipe level and
  REPORTED by an expectation-check in the A5 connectedness-lint
  mold, listing each id's pin multiplicity with the referencing
  nodes. Refusal would make staged updates unauthorable;
  silence would hide the most common mistake.
- **Update triggers ordinary re-evaluation**; a pin move on an
  instance with crossing declarations additionally triggers mate
  re-verification (A4's "does it actually fit" gate — the edit's
  contract, stated once). Disk-moved-pin-held
  staleness is AQ5's capture question, out of this decision.

## Open questions

- **AQ1 — the document store.** What a stable document id anchors
  to, and where referenced documents live (filesystem paths,
  workspace manifest, registry-shaped). Gates import-as-assembly
  and any sharing story; does not gate rungs R1–R2 (single-store =
  the workspace).
- **AQ2 — DISCHARGED into A13** (sign-off 2026-08-16, PR #544).
- **AQ3 — DISCHARGED into A11** (working session 2026-08-10).
- **AQ4 — per-instance arguments.** v1 posture: an instance is a
  pin and its cluster's placement, nothing else — no per-instance
  parameters, no per-instance suppression beyond GQ2's failure
  semantics. The
  natural form when the door opens (the #356 conversation,
  replacing the spooky "override" framing): a document already IS
  a function — `build(params) → Body` — so its named parameters
  are its SIGNATURE, its authored values are DEFAULTS, and an
  instance is an APPLICATION: `InstantiatePart { pin, …, args }`
  evaluates the pinned recipe at `args` (unsupplied names take
  defaults; memo keys on (pin, args, ε)). Nothing mutates or
  shadows the part document; self-containment and pin semantics
  survive untouched because arguments are assembly-side data. The
  fastener-bundle coupling (A7) is this door's natural client —
  one template value applied as the same argument to several
  instances. Still a deliberate future door, not v1.
- **AQ5 — in-context capture semantics**: the captured-context
  object's pin/update behavior (what exactly is captured; when it
  goes stale; how staleness surfaces).
- **AQ8 — the crossing record's reachability (R2-b finding,
  2026-08-17; the A4/A11 composition gap).** Ratified A4 says
  "every mate edge crossing the cut becomes the interface
  record"; ratified A11 rule 2 makes a mate JOIN its two
  instances into one placement cluster; and R2-a's ratified
  precondition (TornCluster) refuses any cut that is not a union
  of whole clusters. Composed: a mate that would cross a cut also
  welds its endpoints into one cluster, so the cut that would
  sever it is refused — "crossing mate" and "legal split" are
  mutually exclusive, and A4's interface record is UNREACHABLE
  through split as ratified. R2-b shipped the collector as
  specified (it inhabits the record, feeds the content key, and
  re-verifies at evaluation — all machinery real and tested); the
  rows pin the actual behavior in both directions. **Proposed
  resolution (firm): amend A4's sentence to name the honest
  mechanism instead of the impossible one** — the interface
  record is populated by the DELIBERATE act: splitting a mated
  cluster is legal exactly when the caller passes the mates
  crossing the cut to be CONVERTED (each severed mate's
  declaration moves into the interface record; the remainder's
  instance re-verifies it against the new part — A4's "does it
  actually fit" gate, exactly as shipped), turning TornCluster
  from a wall into a gate with a named door. Alternative: leave
  A4 vacuous-by-composition and drop the record (rejected: the
  re-verification machinery is R2-b's most valuable artifact and
  pin-move re-verification already uses it). The conversion
  door is a small follow-on unit (ASM-XSPLIT); its spec binds the
  construction-time refusals (non-crossing passed mate,
  unresolvable reference, mate-not-in-document) so only fit
  defers to evaluation.
- **AQ6 — cross-document `Rest` verification detail**: the trilean
  shape for value-equal-by-authoring carriers (peg/bore radii),
  and the recourse text steering designed clearance to
  `Fit { g₀ }`. Belongs with the C7-era verification-table specs.
- **AQ7 — DISCHARGED into A12** (sign-off 2026-08-15, PR #522).

## A12 — Mate edges and roots (discharges AQ7)

**(Sign-off 2026-08-15, PR #522 — Option A.)** Ratified resolution
of the A3/A9/A10 composition question the R2 census recon surfaced
(a `Mate`'s stable-name references are not DAG edges under the
shipped D3 carve-out, yet A9 defines relative freedom over the
recipe DAG itself):

- **Reading edges.** A `Mate` contributes edges of a second sort —
  *reading edges* — to the instantiate nodes its references
  resolve through (the head segment of each instance-qualified
  name). Reading edges are RECOMPUTED from recipe data at need,
  never stored beside it — the DAG stays the single structure
  (A9's no-derived-graph simplification holds). A dangling head
  (N5) contributes no edge until `Rebind`.
- **Partitions.** A9's relative-freedom partition — and A11's
  mate-connected placement clusters — run over ALL edges: mates
  couple components, so G3's free-drag moves or refuses whole
  mated clusters. A10's invariants, automatic maintenance, and
  the product gather run over CONSUMING edges only: inserting a
  mate consumes no root (no tip-transfer onto a bodiless node —
  the instances' bodies keep gathering), and ancestor-freedom is
  read over consuming paths.
- **Mates are ordinary non-body roots (Option A).** Under
  consuming-edge maintenance a mate is an isolated sink, so "no
  consumers → root" applies unchanged and every A10 sentence
  stays true as written: the mate joins the root list, denotes no
  body, and the gather ignores it. The root-list noise concern is
  presentational — a filter is cheap; A4's split handles a
  crossing mate's root membership alongside the interface record
  (spec'd in R2).

## Implementation ladder

Worked through by the ASM program, CLOSED at
v1 scope; the state of record is `docs/ASM-EXIT-WALK.md`, never
this list.

- **R0 — DISCHARGED**: #317 import flattening, with the A7 record
  shape as its one new obligation.
- **R1 — DISCHARGED, the body graph**: `InstantiatePart` (pin,
  placed through A11's cluster registry) and `Pattern` nodes;
  materialized evaluation through `transform_rigid`;
  instance-qualified naming; pins and the split/inline pair;
  disjoint-assembly validity.
- **R2 — DISCHARGED, mates, constructively**: `Mate` nodes,
  frame-chain solving, declaration minting, planar contact
  verification; typed refusal on simultaneous systems. The one
  follow-on the rung banks is AQ8's conversion door (ASM-XSPLIT,
  spec unwritten); the A11 member-vocabulary rider's
  implementation is banked with it (#945).
- **R3** — the at-rest door landed with M9, and cross-instance
  PLANAR pairs certify on the verified shared carrier (#1063,
  `docs/CENSUS-REST-CLOSURE-DESIGN.md`). Cross-instance CURVED
  verification is that document's named residue; C6 interference
  gates and the mass-props refusal/subtraction are the M10-era
  half.
- **R4 (banked)** — instanced/lazy evaluation as pure optimization;
  mirror implementation (equivariance audit as prerequisite; its
  design half is ratified as `docs/MIRROR-DESIGN.md`);
  import-as-assembly-document (post-AQ1).
- **(c)/(d) eras** — numeric SE(3) mates (W8 mechanism + proof
  obligation), then kinematics. Out of v1 by A1.
