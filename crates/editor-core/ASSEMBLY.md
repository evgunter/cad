# Assemblies and mates

An assembly is an ordinary document whose leaves instantiate other
documents. Its recipe DAG uses the same node, edit, naming, undo and
memo machinery as a part; its evaluation is one kernel `Body`, generally
multi-solid and non-connected; the kernel has no assembly type.
Cross-document references are `(document id, content pin)` pairs with
Cargo.lock semantics. A mate is a declaration: one node carries both the
placement constraint and the contact declaration, placement is solved
constructively (frame composition over decided predicates), and
validity is the kernel's at-rest gate over the gathered product with
the mates' declarations minted into it. The census and declared-contact
vocabulary is documented in `crates/topo/README.md`; the user-facing
walk is `docs/guide/assembly.md`.

## Where in the code

| Decisions | Modules |
|---|---|
| A2 evaluation seam, memo | `src/part.rs` (`PartResolver`, `ResolveFault`), `src/eval/parts.rs` (`PartCache`, `PartFault`); `transform_rigid`, `graft_disjoint_all_keyed` in `crates/topo/src/instance.rs` |
| A2a pairing doors | `mispaired`, `Mispaired` in `src/ident.rs`; the three doors in `src/product.rs`, `src/assembly.rs`, `src/mate/solve.rs`; the memo's drop in `src/eval/mod.rs` |
| A3, A11, A12 mates, solve | `src/mate.rs` (`class_admission`, `MateFault`), `src/mate/coset.rs`, `src/mate/solve.rs` |
| A4, A13 identity, pins, update | `src/ident.rs`, `src/update.rs`, `DocEdit::UpdateReference` in `src/edit.rs` |
| A4 split and inline | `src/refactor.rs`; `InterfaceRecord` in `src/node.rs` |
| A5 at-rest gate | `src/assembly.rs` (`assemble`, `AssemblyError`) |
| A6 improper frames | `src/placement.rs` (`Frame`), `EditError::ImproperPlacement` |
| A7, A8 interchange | `PlacedInstance` in `crates/step-import/src/lib.rs` |
| A9, A11 partitions | `relative_freedom_components`, `clusters`, `gauge_of` in `src/mate/solve.rs` |
| A10 roots and gather | `src/roots.rs`, `src/product.rs`, `DocEdit::SetRoots` |
| Store (AQ1) | `Workspace` in `crates/pncad/src/workspace.rs` |

## Scope

**A1 — The scope ladder.** Four rungs in sequence: (a) the body graph
(instances of pinned part documents, rigid frames, patterns); (b)
declared contacts between instances, solved constructively; (c)
numerically solved mates (SE(3) witnesses under the witness contract in
`crates/editor-core/README.md`); (d) kinematics. (a) and (b) are built;
(c) and (d) are not, and A11's refusals point at (c).

## Evaluation

**A2 — An assembly is a document; its evaluation is a body.** An
instantiate node resolves its `DocRef` through the evaluation's
`PartResolver` (`EvalOptions::resolver`; none means instantiate nodes
refuse typed), evaluates the pinned document at the ambient ε, takes its
A10 product, and materializes it through `topo::transform_rigid`
(rigidity re-decided, every carrier re-certified) and the disjoint
graft. `PartCache` memoizes per `(DocRef, ε)` within one evaluation; a
reference already on the descent chain is a cycle and refuses naming
the loop. Which solids are one part and which mates hold is recipe
structure and provenance, never body state. Fusing is an explicit
cross-instance boolean node, never implied. A resolved document whose
recorded ε disagrees refuses `ResolveFault::EpsilonSeam`.

**A2a — The pairing doors.** A4's identity stamp is what these read.
THREE doors refuse a mismatched (document, evaluation) pair typed,
before reading anything of the value: `product` (with `product_named`
and `product_recorded`, `ProductError::EvaluationOfAnotherDocument`),
`assemble` (the same refusal, through `AssemblyError::Product`), and
`SolvedPoses::placement` (`MateFault::PosesOfAnotherDocument`), which
pairs a document with a solve rather than an evaluation and states the
same rule. The memo is the fourth reader and refuses differently, since
`evaluate` returns no `Result`: a prior of another document is dropped
whole before the schedule is built, and the run records the drop as
`Evaluation::prior_refused` while recomputing everything. Node ids
alone could not decide any of this — they are minted by a per-document
counter, so two documents built from one recipe carry the SAME ids for
the same nodes, and a gather over the wrong one would succeed, in
full, about other geometry.

Other doors that take such a pair — `run_checks`, `apply_with_names`,
`stackup` and `sensitivities`, `drive::certifying` — do NOT check it
today; `assembly::mint` is covered downstream by `product_recorded`.
That gap is tracked at
`work/docm/pair-doors-outside-the-three-do-not-check-document-identity`.

## Nodes and mates

**A3 — The node vocabulary; mates are declarations.**
`Node::InstantiatePart { doc_ref, interface }` has no placement field
(A11 puts it on the cluster). `Node::Mate { a, b, class, alignment }`:
`a`/`b` are `SitedRef`s — an instance-qualified stable name plus the
operand node it is read at; `class` is the kernel
`topo::ContactClass`; `Alignment` is two `MateFrame`s in each side's
part coordinates, a `MatePrimitive` (`FrameCoincidence`, `Coaxial`,
`PlanarRest { offset }`; `Clocking` exists only to be refused as a bare
primitive), an authored `AxisSense` (so no π-flip is inferred) and an
optional clocking rider. `Node::Pattern` replicates an instance by
`PatternKind::Linear`, `Circular` or `Explicit`. Evaluation mints each
mate's declaration into the product's `ContactRecords`, the same
currency as the boolean wrapper's; declarations are verified, never
trusted. `class_admission` is the one table the solve and the mint door
both read: `Rest` solves and mints; `Tangent` solves and refuses at the
mint door (`AssemblyError::NoAtRestRecord`, no witness edge at rest);
anything else, including the reserved and unbuilt `Fit { gap }`,
refuses at the solve door.

**A12 — Mate edges and roots.** A mate's two references are `SitedRef`s
— a name, and the OPERAND node it is read at — and each contributes a
*reading edge* to the member that operand resolves to, recomputed by
`reading_edges`, never stored. `inputs()` stays empty because a reading
edge is not consuming: making an operand consuming would take the mated
bodies out of A10's root set. A9's partition and A11's clusters run over
consuming ∪ reading edges; A10's invariants, maintenance and gather run
over consuming edges only, so a mate is an ordinary non-body root: an
isolated sink, listed, ignored by the gather. A dangling reference —
name or operand — contributes no edge; `Rebind` repairs a name and
carries an at-mint operand with it, and a stranded operand is re-authored.

## Identity, pins, split and inline

**A4 — Identity, pins, and the split/inline pair.** `DocumentId` answers
which part and survives every edit; `ContentPin` answers which version
and is the SHA-256 of the canonical semantic bytes
(`persist::canonical_bytes`); `DocRef` pairs them. Edits to a referenced
document never retarget a reference: the resolver returns a document
only when its bytes hash to the pin, else `ResolveFault::PinMismatch`;
moving a pin is a recorded edit (A13). That refusal is the SEAM's, and
only the seam's (DI2): an evaluation that crosses the seam refuses a
moved pin, and an evaluation served from a prior serves what the
document pins — the memo is a pure function of the document, since for
an instantiate node the pin IS the content, so store state enters no
admission decision. Whether the mounted store still holds those bytes is
the mounting session's question, not the memo's. An evaluation carries
the id of the document it is of, and `SolvedPoses` the id of the
document it solved (DI3); which doors read that identity is A2's.
`refactor::split` cuts a node set closed under the DAG in both
directions (a severed edge refuses) and a union of whole placement
clusters (else `SplitError::TornCluster`) into a new document with a
caller-supplied id, leaving one `InstantiatePart` behind; a cut of
exactly one cluster hoists its frame onto the instance, any other cut
moves placements verbatim. Remainder-side names re-anchor through the
instance qualifier by recorded `Rebind`s; `refactor::inline` is the
inverse. Both are pure, returning values plus edit lists. Acceptance:
split-then-evaluate equals unsplit evaluation at structural and
name-resolution identity, not bit identity. A mate whose two
`InstantiatePart` heads fall on opposite sides of a cut is an
`InterfaceCrossing::Mate` in the instance's `InterfaceRecord`, which
feeds the content key; evaluation refuses
`NodeErrorKind::CrossingUnverified` when a crossing's part-side name no
longer resolves in the pinned product. No split reaches that record
today (AQ8).

**A13 — Update granularity.** The primitive is
`DocEdit::UpdateReference { node, new_pin }`: per reference, recorded,
refusing an unchanged pin, not resolving the pin (evaluation does).
`update::update_references` elaborates "update id everywhere" into one
edit per moving site and applies none; atomicity is the caller applying
the whole list. Mixed pins for one id are legal; `update::mixed_pins`
reports them as a lint that gates nothing. Update triggers ordinary
re-evaluation, which re-verifies crossings (A4).

## Validity

**A5 — The at-rest gate.** `assembly::assemble` gathers the product
(`product::product_recorded`), mints every solved mate's declaration as
a `MintedDeclaration` (declaring mates mint like determining ones), and
runs the scalar's at-rest policy, `topo::validate_pseudomanifold`, over
body plus records. It runs no predicate of its own; kernel findings
come back as `AtRestFinding`s attributed to the mate whose declaration
they concern. Undeclared contact between instances is a hard error,
never blessed. `AssemblyError::AtRest` is a verdict against the
document; `AssemblyError::Uncertified` is the declared direction's
frontier (every finding declined, none refuted). A disjoint assembly
certifies as a multi-solid tier-3 body. A sub-assembly's declarations
ride through the seam as records (`PartValue::contacts`), but
attribution stops at the seam. Interference fits through recorded
gate-skips are not implemented.

## Mirror

**A6 — Mirror and improper frames.** `Frame` stores a general linear
part so an improper frame (det = −1) is representable, and it is
refused: `DocEdit::SetPlacement` refuses `EditError::ImproperPlacement`
for det ≤ 0 and the load validator refuses the same. Mirrored instances
are not implemented; STEP import refuses a mirroring placement.

## Interchange

**A7 — The leave-room register.** World-space geometry stays
materialized, so volume and clearance queries read it directly;
verification verdicts are never cached into pins; per-instance
arguments and fastener bundles are future doors over the existing
vocabulary (AQ4). Flattened STEP import records one `PlacedInstance` per
solid (component, solid, occurrence, transform, rigid map applied) so a
flattened import can be re-adopted as an assembly without re-parsing.

**A8 — Interchange posture.** Import flattens to the multi-solid body,
which is the evaluation product, with the A7 record;
import-as-assembly-document is not implemented. Export writes each
positive shell as its own `MANIFOLD_SOLID_BREP`; assembly structure and
declarations are not exported.

## Relative freedom and product roots

**A9 — Relative freedom is component structure.** Two instances are
relatively unconstrained exactly when they lie in different connected
components of the DAG under consuming ∪ reading edges
(`relative_freedom_components`); no solver, no geometry. Anchor frames
are never erased, so evaluation stays one deterministic body. The
viewer's free-move probe is display state, never persisted; it admits
only an instance no mate names (`crates/viewer/src/display.rs`),
stricter than the whole-component drag this decision permits.

**A10 — Explicit product roots.** `Doc::roots` is an ordered list of
node ids, document data. Invariants (`roots::check`): coverage (every
live node is ancestor-of-or-equal-to some root) and ancestor-freedom
(no root is a strict ancestor of another); together the root set is
exactly the DAG's sink set and the list adds only the solid order.
Maintenance: a new sink appends, a node consuming roots replaces them,
deleting a root re-roots its orphaned inputs; `DocEdit::SetRoots` states
the list outright. `product::product` gathers, in list order, every
body-denoting root (`Body`/`Boolean` solids, `Instances` as placed
solids with no boolean implied, `Split` as both pieces); non-body roots
contribute nothing, and a door needing a body refuses
`ProductError::NoBodyRoots`.

## The constructive-solve boundary

**A11 — Five rules, decided structurally.** (1) Each primitive pins the
pair's relative pose to a coset of an SE(3) subgroup; the closure is
`Subgroup::{Se3, Planar, Cylindrical, Prismatic, Revolute, Trivial,
Empty}` and several mates on one pair fold by exact coset intersection
(`mate/coset.rs`): DETERMINED, UNDER or CONTRADICTORY, the last refusing
with the added mate's measured clash. (2) Placement lives on the
cluster: clusters are connected components of the instance–mate graph
(`clusters`); `Doc::placements` holds at most one `Frame` per cluster,
keyed by its gauge, a missing entry being the identity, so zero- and
multi-anchor states are unrepresentable; `reconcile` re-keys records
when an edit joins or splits clusters
(`ClusterMaintenance::{Join, Split, GaugeRewrite}`, gauge-exact in
bits). (3) The gauge is the cluster's earliest instance in document
order, a convention, not data; pattern-placed instances are
gauge-ineligible. (4) `solve_document` takes the deterministic spanning
tree rooted at the gauge: tree mates DETERMINE and must fold to
`Trivial` (an UNDER tree edge refuses naming the residual subgroup,
recourse `UNDER_RECOURSE`); non-tree mates DECLARE and are only
verified by the gate. No cycle is ever solved; an inconsistent loop
dies at its closing mate's verification. The solve is total and
per-node: a refusing cluster faults its own mate and instances
(`SolvedPoses::fault`), nothing else. (5) `SolvedPoses::placement`
composes the cluster frame onto the solved relative pose; a singleton
cluster returns its recorded frame bit for bit. It is one of A2a's
pairing doors: the document it is handed must be the one solved, else
`MateFault::PosesOfAnotherDocument` before any frame is read. A
reference resolves by walking from its OPERAND down to a live
`InstantiatePart`, through any number of `Transform`s and at most one
`Pattern` level (which the name qualifies `Instance(i)`); the member's
frame is the composed static offset of every node that walk passed, on
that instance's pose, so mates never solve pattern or transform
parameters or give one placed body its own pose. Two references to one
instance read at different operands are two members.

## Open questions

- **AQ1 — the document store.** Built: `pncad::Workspace`, a directory
  of `*.pncad` files scanned by id header, one file per id, resolving
  `DocRef`s with the pin gate. Open: what an id anchors to beyond one
  directory, and how a one-version-per-id store serves mixed-pin state.
- **AQ2, AQ3, AQ7** — discharged into A13, A11, A12.
- **AQ4 — per-instance arguments.** Not implemented. Intended form:
  `InstantiatePart { …, args }` applying the pinned recipe at `args`.
- **AQ5 — in-context capture semantics.** Open.
- **AQ6 — cross-document `Rest` verification.** Answered in the kernel:
  the trilean verdict and the designed-clearance steer toward `Fit`
  (`fit_steer` in `crates/topo/src/boolean/contact_verify.rs`).
- **AQ8 — the crossing record's reachability.** A crossing mate welds
  its ends into one cluster and `TornCluster` refuses cutting through
  one, so split never populates `InterfaceRecord`. The conversion door
  (crossing mates passed at split and converted) is not implemented.
