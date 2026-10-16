# Assemblies and mates

An assembly is an ordinary document whose leaves instantiate other
documents. Its recipe DAG uses the same node, edit, naming, undo and
memo machinery as a part; its evaluation is one kernel `Body`, generally
multi-solid and non-connected; the kernel has no assembly type.
Cross-document references are `(document id, content pin)` pairs with
Cargo.lock semantics. A copy is made by a placement — a `Place` of the
bodies it reads, or an instance of a part — and pinned by its bundle of
constraints, each a mate that is both a placement constraint and a
contact declaration. Placement is solved constructively (frame
composition over decided predicates), and validity is the kernel's
at-rest gate over the gathered product with the mates' declarations
minted into it. The census and declared-contact vocabulary
is documented in `crates/topo/README.md`; the user-facing walk is
`docs/guide/assembly.md`.

## Where in the code

| Decisions | Modules |
|---|---|
| A2 evaluation seam, memo | `src/part.rs` (`PartResolver`, `ResolveFault`), `src/eval/parts.rs` (`PartCache`, `PartFault`); `transform_rigid`, `graft_disjoint_all_keyed` in `crates/topo/src/instance.rs` |
| A2a pairing doors | `mispaired`, `Mispaired` in `src/ident.rs`; the doors in `src/product.rs`, `src/assembly.rs`, `src/mate/solve.rs`, `src/checks.rs`, `src/resolve/mod.rs`, `src/resolve/pick.rs`; the memo's drop in `src/eval/mod.rs` |
| A3, A11 placements, mates, solve | `Node::Place`, `Node::World`, `InstantiatePart`'s `constraints` in `src/node.rs`; `src/mate.rs` (`Constraint`, `Mate`, `MateRef`, `SideRead`, `class_admission`, `MateFault`), `src/mate/coset.rs`, `src/mate/solve.rs`; `DocEdit::AddConstraint`/`RemoveConstraint` in `src/edit.rs` |
| A4, A13 identity, pins, update | `src/ident.rs`, `src/update.rs`, `DocEdit::UpdateReference` in `src/edit.rs` |
| A4 split and inline | `src/refactor.rs`; `InterfaceRecord` in `src/node.rs` |
| A5 at-rest gate | `src/assembly.rs` (`assemble`, `AssemblyError`) |
| A6 improper frames | `src/placement.rs` (`Frame`), `EditError::ImproperPlacement` |
| A7, A8 interchange | `PlacedInstance` in `crates/step-import/src/lib.rs` |
| A9 partitions | `relative_freedom_components` in `src/mate/solve.rs` |
| A11 (2) bundles, spaces | `solve_placement`, `fold_bundle`, `redundant_constraints`, `Pose`, `Space`, `spaces_with` in `src/mate/solve.rs`; the per-space gather (`LooseSpace`, `loose_spaces`) in `src/product.rs` and the gate's (`AssemblyError::Space`) in `src/assembly.rs`; the one cross-space predicate `Evaluation::across_spaces`; an instance's copies (`ValuePayload::Copies`, `CopyKey`) in `src/eval/wire.rs` and `src/eval/parts.rs` |
| A10 the world and the gather | `Node::Place`, `Node::World` in `src/node.rs`, `Doc::placements`/`Doc::unplaced` in `src/doc.rs`, `src/product.rs`, `DocEdit::place`/`place_in_world` in `src/edit.rs` |
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
`PartResolver` (`EvalOptions::resolver`; with none, instantiate nodes
refuse typed), evaluates the pinned document at the ambient ε, takes its
world (A10), one `Body` per world placement, all in one space (no body
has world coordinates: the world sets only the coordinates export
writes), and materializes it through `topo::transform_rigid`
(rigidity carried by the witness or decided at the door, every
carrier re-certified) and the disjoint graft. A resolved document whose recorded ε disagrees refuses
`ResolveFault::EpsilonSeam`. `PartCache` memoizes per `(DocRef, ε)`
within one evaluation; a reference already on the descent chain is a
cycle and refuses naming the loop. Which solids are one part and which
mates hold is recipe structure and provenance, never body state. Fusing
is an explicit cross-instance boolean node, never implied.

**A2a — The pairing doors.** A door that takes a document, or a value of
a document, together with a second value that must be of that same
document, refuses a mismatch typed before reading anything of the
second. What it compares is A4's identity stamp; the comparison is the
one predicate `ident::mispaired`, and each door maps it into its own
error vocabulary.

The rule binds only where both halves carry an identity: a `Doc`, or a
value stamped with one (an `Evaluation`, a `SolvedPoses`, a `NodePick`
and the `PickTarget` it mints). A value with no provenance of its own is
outside the clause because there is nothing to compare, not because a
check was declined: `resolve::hit`'s `face_name` / `edge_name` /
`vertex_name` take a raw arena key beside an evaluation, and a bare
`StableName` is text. That is #1098's raw-key class, and what closes it
is a stamped value to hand instead, which is what `NodePick` is for the
pick doors.

The stamp decides the document only. A later evaluation of the same
document is admitted at every door, because a pairing is about identity,
never about version (DI3); whether anything may be reused across such a
run is the content keys' business. Node ids cannot decide the pairing:
they are minted from the edits that inserted them, not from the
document's identity, so two documents built from one recipe carry the
same ids for the same nodes, and a gather over the wrong one would
succeed, in full, about other geometry.

The doors that refuse:

- `product` (with `product_named` and `product_recorded`,
  `ProductError::EvaluationOfAnotherDocument`), and `assemble` through
  them (the same refusal, wrapped as `AssemblyError::Product`);
- `SolvedPoses::placement` (`MateFault::PosesOfAnotherDocument`), which
  pairs a document with a solve rather than an evaluation;
- `run_checks_on`, and `run_checks` as its wrapper
  (`ChecksError::EvaluationOfAnotherDocument`), which checks the
  evaluation and the document a `Subject::Product` carries, because a
  resident reading `doc.placements()` against a foreign evaluation
  finds a value for every placement;
- `resolve::apply_with_names` (`EditError::EvaluationOfAnotherDocument`),
  which reads the handed evaluation's name tables, so a foreign one
  admits a name the edited document does not carry or refuses one it
  does;
- the pick index's three doors, `NodePick::patch_names`,
  `NodePick::boundary_names` and `pick_face`
  (`HitTestError::EvaluationOfAnotherDocument`), below.

The memo reads the stamp too, and since `evaluate` returns no `Result`
it refuses by dropping: a prior of another document is dropped whole
before the schedule is built, the run records the drop as
`Evaluation::prior_refused`, and everything is recomputed.

**The pick doors** pair a value of a document with an evaluation: a
`NodePick` is built from one evaluation and handed a second at each name
door, and `pick_face` takes `PickTarget`s built from one evaluation
beside an `eval` argument. So the stamp is on the value: `NodePick` keeps
the building evaluation's `DocumentId`, and `NodePick::target` carries it
onto each target it mints. The refusal is of the whole call, outside the
per-entity vector, because a mispairing is one thing wrong with the
arguments rather than one thing wrong with each patch. `pick_face`
checks every target before it reads any target's standing: a twin
recipe's evaluation mints the same node ids, so the standing ladder
would admit it and the hit would resolve to a name from the twin's
table.

The stamp reaches every target a consumer can hold. A `PickTarget`'s
fields are private and `NodePick::target` is its only reachable mint:
the document, node, body and mesh index all come from the one
tessellation `NodePick::build` performed, so a caller declares none of
them and the door's check is a statement about the type. The
hand-assembled mint (`PickTarget::new`, and `MeshPick::build` for the
index it needs) is behind `editor-core`'s `test-support` cargo feature,
which only this crate's own dev-dependency enables;
`scripts/gates/test-features-dev-only.sh` holds across every manifest
that no consumer wires it onto an edge of its own, and its header says
why that is all a feature can claim (a build command may ask for any
feature by name). #1098's raw-assembly class therefore exists only where
that feature does: in the rows that measure it (a raw target's
declaration is taken at its word in the document half as in the node
half, which is unprovable in principle, since arena keys collide
numerically across sibling nodes of one document), and in the rows that
need an index no tessellation produced or a node `NodePick::build`
refuses.

Other doors that take such a pair — `stackup` and `sensitivities`,
`drive::certifying_vector` — do not check it; that gap is the tracker
row `pair-doors-outside-the-three-do-not-check-document-identity`.
`assembly::mint` is covered downstream by `product_recorded`.

## Nodes and mates

**A3 — The node vocabulary; mates are declarations.**
`Node::InstantiatePart { doc_ref, interface, constraints }` instantiates
a pinned document, and is a placement: its one output `copies` is the
part's world copies, each keyed by its identity in the part
(`CopyKey`), moved together by the motion its constraints pin. A reader
of one copy reads it by its key (`BodyRead::key`); a selection names a
face of it by the part-local name path, wrapped at the instance.
`Node::Place { shapes, constraints }` copies the bodies it reads, one
output of their shape. The world, `Node::World`, is minted with every
document, defines no variable, and is read only by a placement's
constraints and by export. Repetition is an index variable (D10,
Repetition): a placement whose reads reach an index places one copy per
value of it, and no node is a pattern.

A placement's `constraints` are its bundle, each a `Constraint { id,
mate }` addressed by its placement and the id minted with it
(`MateRef`), never by its position. A `Mate { a, b, class, alignment }`
is one contact declaration that also places:

- `a`/`b` are each a `SideRead`: a `Face` read (`OperandSlot::Side`, a
  selection of the body the side is read in by an instance-qualified
  face name, D10; the authored spelling is a `SitedFace`), the copy
  itself (`Copy`), or the world (`World`). One side is on the copy its
  placement defines (else `MateFault::NotOnTheCopy`); the other is its
  target.
- `class` is the kernel `topo::ContactClass`.
- `alignment` is an `Alignment`: two `MateFrame`s, a `MatePrimitive`,
  and an authored `AxisSense` (so no π-flip is inferred). Each
  `MateFrame` is a base composed with an offset, a `Placement` written
  in the base's frame (the empty chain by default); the base is the
  side's part frame, the face base (`FrameBase::Face`), which names no
  face and is its side's own head face, or the world's frame
  (`FrameBase::World`). The `MatePrimitive` names only the residual
  subgroup and carries no number: `FrameCoincidence`,
  `Coaxial { roll: Free | Pinned }`, `PlanarRest`. A copy pinned to the
  world at a pose is one coincidence of the world's frame, moved by the
  pose, with the copy's own (`Mate::world_pin`).

Every number that says where the two sides meet lives in the sides'
offsets: the roll about a shared axis is a rotation in an offset, and a
pinned coaxial's roll is the angle between the two sides' resolved
references, so no primitive can be written that the table has no row
for.

Evaluation mints the declaration of each constraint whose two face
sides read copies of the space being gathered into the product's
`ContactRecords`, the same currency as the boolean wrapper's;
declarations are verified, never trusted. `class_admission` is the one
table both the solve and the mint door read: `Rest` solves and mints;
`Tangent` solves and refuses at the mint door
(`MintRefusal::NoAtRestRecord`: the census at rest decides nothing
along a tangency's locus for its face–face record to cite; recourse
`NO_AT_REST_RECORD_RECOURSE`); anything else, including the reserved
and unbuilt `Fit { gap }`, refuses at the solve door.

**A12 — Mate sides are reads.** Retired by D10: a mate side is a
read like any other, so a mate depends on the bodies its faces are read
in, A9's partition runs over reads (D10, Operations), and a cut that
separates a mate from a body it reads is D-2's ordinary severed read. The
member a side lands on is the walk from the selection's body down to its
name's minting instance, whatever the depth of the copy chain above it
(`mate::member_of`). A side whose body is deleted leaves the mate
unresolved and typed, and a name that stops resolving is the selection's
refusal, repaired by `Rebind` addressed by the body. A mate places
nothing by itself: it is a constraint of the placement that holds it,
and only that placement's copy is in the product (A10).

## Identity, pins, split and inline

**A4 — Identity, pins, and the split/inline pair.**

*Identity and pins.* `DocumentId` answers which part and survives every
edit; `ContentPin` answers which version and is the SHA-256 of the
canonical semantic bytes (`persist::canonical_bytes`); `DocRef` pairs
them. Edits to a referenced document never retarget a reference: the
resolver returns a document only when its bytes hash to the pin, else
`ResolveFault::PinMismatch`; moving a pin is a recorded edit (A13). That
refusal is the seam's, and only the seam's (DI2): an evaluation that
crosses the seam refuses a moved pin, and an evaluation served from a
prior serves what the document pins. The memo is a pure function of the
document, since for an instantiate node the pin is the content, so store
state enters no admission decision; whether the mounted store still
holds those bytes is the mounting session's question. An evaluation
carries the id of the document it is of, and `SolvedPoses` the id of the
document it solved (DI3); which doors read that identity is A2a's.

*Saving.* A save is two acts (Ev, PR 2016). Saving a document at a path
keeps its identity, and refuses typed when the target directory already
holds that id under another filename. Saving it as a new document mints
a fresh id: an explicit fork that leaves every inbound `DocRef` pointing
at the original.

*Split.* `refactor::split` cuts a node set closed under the DAG in both
directions into a new document with a caller-supplied id, leaving one
`InstantiatePart` behind, pinned to the world at the identity; a severed
edge refuses. A cut takes whole placements: a placement's constraints are
its own, so a cut placement whose bundle reads a kept copy severs that
read. The part's world is the cut's world placements, so the instance's
copies stand where the cut's did. A remainder read of the cut crosses
only where the instance carries what it reads: a selection each of whose
names one cut world placement's copy holds, re-authored on the
instance's copies; a read of one copy of a cut world placement, or of a
body one cut world `Place` copies at the identity, re-pointed to that
copy of the instance by its key in the part. A kept constraint reading a
cut copy on that copy's own part frame crosses only where the copy is
pinned to the world at the identity (`SplitError::MateFrameCrosses`).
Remainder-side names re-anchor through the instance qualifier by
recorded `Rebind`s. A variable moves with its readers: its side is the
union of its readers' — a node slot, or another variable's definition —
and one with no reader follows what it reads. One whose side is the
cut's is declared in the part and deleted from the remainder.

*Inline.* `refactor::inline` is the inverse. An instance pinned to the
world at a pose splices its part's placements, each constraint on the
part's world moved by that pose (at the identity, verbatim). An instance
placed by other constraints is inlined only when the part's one
placement on its world is an instance pinned there at the identity,
which takes the instance's bundle in place of its pin
(`InlineError::UnplaceablePart`). A host constraint side reading the
instance crosses only where its coordinates do not change: a face frame
rides its face, a part frame needs the copy it lands on to stand in the
instance's coordinates, and the instance's own side lands on that root
(`InlineError::MateFrameCrosses`).

Split and inline compute no frame. Both are pure, returning values plus
edit lists.

*Acceptance.* Split-then-evaluate equals unsplit evaluation at
structural and name-resolution identity, not bit identity, up to the
order of copies: the cut's world placements become the part's, and the
remainder holds one instance of it. Inline-of-split returns the document
split was given, up to minted ids and order.

*Crossings.* A kept constraint whose two face reads fall on opposite
sides of a cut is an `InterfaceCrossing::Mate` in the instance's
`InterfaceRecord`, which feeds the content key (AQ8). Evaluation refuses
`NodeErrorKind::CrossingUnverified` when a crossing's part-side name no
longer resolves in the pinned product.

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
(`product::product_recorded`), which mints every solved mate's
declaration as a `MintedDeclaration` and runs the scalar's at-rest policy,
`topo::validate_pseudomanifold`'s census, over body plus records, once,
keeping its findings on the product (`Product::census`); the gate reads
them and runs no census of its own. A mate places and never checks (D10):
its record cites the at-rest census's decision that its two faces rest
on one carrier (`topo::census_rest_decision`, a `CensusAtRest` row on
the product, which the `unproven-coincidence` check reads), and a pair
the census refuses mints no record, its finding raised by the gate
beside the census's own. The gather's own tier-3 verdict rides
on the product's body (`topo::AtRestBody`), so the gate reads both
verdicts rather than running either a second time. The gate
runs no predicate of its own; kernel findings come back as
`AtRestFinding`s attributed to the mate whose declaration they concern.
A touch between two copies that no record backs refuses nothing (D1
(iii), D10): the gather's census records it (`topo::record_at_rest`,
each copy its own group) as a `CensusAtRest` row of the product
(`AtRestBy::Copies`) and a record citing it, which the census's next
pass confirms, and the `unproven-coincidence` check reports each row
the coincidence door does not prove. Such a record backs no mate, so it
quiets no `Separation` finding. A touch inside one copy that no record
backs is that copy's defect, and refuses.
`AssemblyError::AtRest` is a verdict against the document;
`AssemblyError::Uncertified` is the declared direction's frontier
(every finding declined, none refuted). A disjoint assembly certifies as
a multi-solid tier-3 body.

*Where a declaration is minted.* A mate's declaration is minted on the
world copies of its members, read through those copies' placements
(#4220). Each side's face is resolved once, at its selection, where the
mate's own evaluation resolves it: its entity in the body the selection
reads. The copy is the world placement reading that body, or reading
the `Part` that picks the entity's copy out of a pattern's bodies; a
placement moves its body and re-mints nothing, so the copy holds the
entity under the same key, and the graft carries it into the product.
Nothing is resolved against the product's table. A side whose body no
placement reads has no world copy, and the mate mints nothing: such a
mate relates a boolean's operands in the workbench, which is the
boolean's coincidence door's business, not an at-rest fact of the
product. A body placed twice refuses `RefusedRef::Ambiguous`, since a
declaration names one pair of faces, and a mate with no value in the
evaluation refuses `MintRefusal::Unevaluated` rather than passing the
gate silently.

*Across the seam.* A sub-assembly's declarations ride through the seam
as records (`PartValue::contacts`), and so does the bookkeeping that
names them: `PartValue::minted`/`unminted` carry each inner mate's
declaration, and each mate the inner document could not mint, into
`Product::carried`/`carried_unminted`, each tagged with the `Route` it
arrived by (the instantiating node, the document that minted it, and the
instances below). A declaration's faces are re-keyed at every graft
through the graft's own descendant map, exactly as its record is; a mint
refusal names no entity, so it carries verbatim. Nothing is re-verified
or re-minted across the seam: verification runs once, at the outermost
gate.

*Attribution.* A finding against a carried declaration attributes
`Attribution::Carried`, naming that mate, that document and that route,
in the same `Relation` (`refuted` or `declined`) the own-minted arms
use; so a carried decline reaches `AssemblyError::Uncertified` under its
own name. A finding no declaration this document holds a row for
answers for, its own or a part's, carries no attribution
(`AtRestFinding::attribution` is `None`): a finding about no
declaration at all (a structural defect, an overlap or a touch no
record backs inside one copy, an unrecorded touch of a pair the census
could not clear, a census that could not look), never a declaration
that lost its row. Every declaration in the tree keeps its row,
because the only path a record can take without its row (a boolean over a source) is one no instance
carrying a declaration can reach: such an instance is a product of
several parts, whose part count the placers and a sub-assembly that
instantiates it carry through (`NodeValue::parts`), and every op that
fuses or reshapes one body refuses a product
(`NodeErrorKind::ProductOperand`, naming the explicit union as the
recourse).

*Carried mint refusals.* An inner mate that could not be minted refuses
the outer gate (`AssemblyError::CarriedMintRefusal`), carrying every
such row in gather order, before this document's own unminted rows and
before the at-rest gate: an outer assembly is unusable while an inner
part's contact is unverified.

*Interference.* An overlap of two copies' material is a finding of its
own, never a refusal (D10; `checks::at_rest`). The census decides that
two copies interfere: a vertex of one strictly inside the other's
material (`ValidationError::InstanceInterference`), an edge piercing a
face (`CensusContact::EdgeFacePierce`), or an in-plane crossing whose
side test reads `SameSide`. The gate takes those verdicts out of its
refusal, with every `CensusUndecidable` the census left on the same
pair of copies, and intersects the two copies. Each connected solid of
the intersection is one `InterferenceFinding` on
`Assembly::interference`, named by the faces of both copies that bound
it. An intersection the kernel cannot form leaves one
`Overlap::Unlocalized` finding for the pair, which is loud and which
nothing quiets; so does a decided overlap at an analysis scalar, which
has no localizing lane (`LocalizeLane`: `f64` builds bodies). A pair the
census decided nothing about is refused as before.

A finding is quiet when a holding assertion reads a `Gap`'s output
directly over an opposed pair of faces of the two copies, admits only
negative values (`≤ b` or `= b`, `b` decided negative), and its two
faces bound the overlap, and the overlap lies between their carriers.
The last is the kernel's own boolean: the
region past each asserted carrier, away from its copy's material, is
built as a solid (a box past a plane, the solid cylinder inside a bore,
the tube outside a pin) and intersected with the overlap, which lies
between when neither intersection keeps any of it. A containment check
that refuses leaves the finding `Unlocalized`. A quiet finding carries
no verdict nothing checked: every `CensusUndecidable` on the pair must
be a face pair bounding the quieted overlap, so an undecided pair
beside it (a pin pressed into one bore and slipped into another) keeps
it loud. Every other overlap is loud. Between two solids of one copy an overlap stays the kernel's
typed error. The partition takes interference verdicts only: a press
fit whose pin is flush with its plate also touches at the coplanar
caps, which the gather records as rows of the two copies.

## Mirror

**A6 — Mirror and improper frames.** `Frame` stores a general linear
part so an improper frame (det = −1) is representable, and it is refused
wherever a document admits a frame, by one predicate
(`Frame::admission_fault`): a literal step of a constraint's frame
offset or a transform's chain refuse
`EditError::ImproperPlacement` for det ≤ 0, naming which frame, and the
load validator refuses the same. A mirror is not a placement: a
reflection is not a pose, so a mirrored body is the construction
`Mirror { body, plane }`, which defines a new `Body` (MIRROR-DESIGN
P1–P4), and no placement needs an improper frame. STEP import refuses a
mirroring placement.

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

**A9 — Relative freedom is component structure.** Two copies are
relatively unconstrained exactly when they lie in different connected
components of the recipe under its reads (`Doc::upstream`;
`relative_freedom_components`); no solver, no geometry. Evaluation
stays deterministic, and every copy pinned against the world has a world
pose, so the world is one body. A loose copy lives in its own space (A11
(2)): nothing outside it is compared with it, and it is not part of that
body. The viewer's free-move probe is display state, never persisted
(`crates/viewer/src/display.rs`).

**A10 — The product is the world.** The product is the copies the
document's placements define whose space reaches the world, in the
document order of those placements. A placement (`Place`, or an
instance) is an operation reading the shapes it copies and defining
their copies as one output, so two placements of one body are two
copies. Its constraints relate the copy to the world, directly or
through the copies it is pinned against, and only the gather, export and
a constraint read the world. Nothing else places: no edit places or
unplaces as a side effect, and nothing derives a placement from what
reads what. An empty world is a valid document with an empty product; a
door needing a product refuses `EmptyProduct`, naming the bodies no
placement copies and the loose copies. A placement whose body is deleted
is a stranded reader (D10), and the gather refuses naming it. Split and
inline move placements with their bodies (A4). `product::product`
gathers the copies in placement order.

## The constructive-solve boundary

**A11 — Five rules, decided structurally.**

**(1) Primitives fold by coset intersection.** Each primitive pins the
pair's relative pose to a coset of an SE(3) subgroup; the closure is
the symmetry groups of the pose kinds a primitive equates (D10), and
one `Subgroup` type is both a pose's symmetry and what a mate folds:
`Subgroup::{Se3, Planar, Cylindrical, Prismatic, Revolute, Trivial,
Empty}`, which grows a `Point`'s and a `Direction`'s when a reader needs
them, and several mates on one pair fold by exact coset intersection
(`mate/coset.rs`) to DETERMINED, UNDER or CONTRADICTORY, the last
refusing with the added mate's measured clash. The edit door asks the
same per-constraint admission of a constraint being added (the walk, the
class, each frame, the table's row, the bundle before it), so a mate the
table refuses on its own is refused at the door
(`EditError::MateRefused`, carrying the solve's fault). The doors decide edits and the solve decides states: a
verdict about the pair, and a state a mate comes to hold after insert (a
head a rebind or a shrunk pattern strands, a re-pointed `Part`, a
loaded snapshot), stay the solve's.

**(2) A placement is the bundle of its constraints.** Each placement's
bundle is folded by itself, in the placements' schedule order, through
`fold_pair`'s coset intersection, and its copy's pose is its target
composed with the folded representative. A bundle's constraints all
target one other copy, or all the world
(`MateFault::TargetsDiffer` otherwise; the stage's named interim,
`work/intent/a-bundle-folds-in-the-joined-space.md`). The solve reads a
face only of an instance or a pattern copy of one: a face side, or a
needed lever, on a `Place` copy or a raw construction refuses typed
(`work/intent/the-solve-reads-a-face-only-of-an-instance.md`).

A placement is parametric (Ev, `[ev]` #3437, 2026-09-29): a pose is a
chain of steps, each either a rigid step of `Expr`s (the translation,
rotation axis and rotation angle, the same type `Node::Transform` holds)
or a literal proper matrix held to A6, so a document parameter can drive
where a copy sits.

A bundle that pins its copy puts it in its target's space; one pinned,
transitively, against the world is of the world's. A bundle that pins
less than its body needs makes a loose copy: a space of its own with
everything pinned to it, solved among itself, read at a pose by nothing
outside it, and not a refusal. Nothing outside a loose space is compared
with it, so the at-rest gate checks it by itself (`loose_spaces`,
`AssemblyError::Space`) and cross-space reads refuse
(`NodeErrorKind::SpaceMismatch`), while everything inside it solves and
checks as usual. STEP export writes the world and refuses a loose copy
beside it. The viewer draws a loose copy where it was last shown, as
display state that no logic reads.

No edit records a frame: removing a constraint, deleting an instance or
a body is never refused for the placement it removes, what is pinned is
recomputed from what remains, and replay re-applies the edits alone,
without solving.

**(3) A constraint beyond the pin refuses** (FORK-S3O, #4326). At the
door, a constraint added to a bundle whose fold is already `Trivial`
refuses `MateFault::Overconstrained`. The solve, for a state the door
did not see (a load, a rebind, a bundle each of whose prefixes pinned
less), refuses naming every constraint whose removal leaves the fold's
codimension unchanged, never "the later one": constraints have no order.

**(4) The solve is total and per-placement.** No cycle is ever solved:
a placement reading its own copy, but for an instance's copy side
(which reads the part's face, a pose the instance's motion carries, not
one it computes), is a read cycle the door refuses; an inconsistent
bundle dies at its clashing constraint (`MateFault::Contradictory`,
recourse `CONTRADICTORY_RECOURSE`). A refusing bundle faults its own
placement (`SolvedPoses::fault`), nothing else. The solve states where
each copy is and what each constraint decides, never that a product
exists: whether the document has a product is the gather's question
alone (A10), so a product refusal is not a mate fault, and a document
can solve whole and still have no product.

**(5) World poses, members, and what the solve reads.**

*World pose.* A copy pinned against the world has a world pose, its
targets' composed onto its solved relative pose; a copy pinned to the
world at a pose returns that pose's frame bit for bit. A loose copy has
no world pose, only its pose in its own space.
`SolvedPoses::placement` is one of A2a's pairing doors: the document it is handed must be the one
solved, else `MateFault::PosesOfAnotherDocument` before any frame is
read.

*The member walk.* A reference resolves by walking from its operand
down to a live `InstantiatePart`, through any number of `Transform`s and
`Part` instance selections, any number of `Pattern` levels (each of
which the name qualifies `Instance`) and any number of `Union`s (each of
which the name qualifies `From`, naming the member read the walk
continues at). The member's frame is the composed offset of every node
that walk passed, evaluated as that node is evaluated, on that
instance's pose, so mates never solve for a pattern's or transform's
parameters or give one placed body its own pose. Nothing in the walk is
evaluated, so the partitions never depend on a slot value.

*Member identity.* A member's identity is its instance and the chain of
placing nodes the walk passed, outermost first: each pattern with the
copy the name says, and each transform; the operand is not part of it.
Two references through different placements are two members, and so
are two references to sibling copies at any level. One placement spelled
two ways (read at a transform and at a union above it naming that
member, or at a `Part` selecting a copy and at the pattern naming that
copy) is one member, and its mates fold.

*What the solve reads.* The solve's inputs are the document plus its
mated parts' evaluations, at the evaluation's own scalar and over the
evaluation's own parameters. It reads no geometry except what each
mated part's own evaluation answers, through one door asked lazily per
pair (`mate::MateReach`; the evaluation answers from its own part cache,
so a mated part is evaluated once). Two answers cross that door:

- The part's EXTENT, an upper bound taken from its evaluated body,
  enters only as the lever a parallelism verdict is decided over:
  `(R_a + ‖a.origin‖) + (R_b + ‖b.origin‖) + Σ|authored lengths|`, no
  floor and no constant.
- A side on a face base takes its head face's canonical pose, read
  off its surface parameters exactly (`topo::readback::face_pose`, no
  tolerance) in the part's own coordinates and composed with the side's
  offset (an offset that does not evaluate faults
  `MateFault::FrameUnevaluated`), as the side's frame, so a side set on a face follows that face
  through any edit of the part. The offset is any rigid motion; which
  offsets a mate admits is its contact class's to say (a `Rest` side set
  back from its face declares a contact the gate refutes). A face with
  no canonical frame refuses typed and keeps taking authored vectors.

Neither read changes the solve's algorithm (coset intersection over
decided predicates, no numeric fitting, no geometry inspected inside the
fold), and nothing is stored twice: the face name is the state, the
frame is derived. At `f64` the solve is the nominal solve; in a seed
run each solved pose carries its tangent, in a box run each is an
enclosure over the box, and every branch inside the fold is a named
predicate decided at that scalar. So a face frame resolves on every
lane, and a parametric placer on a member's chain moves its member's
pose in the run that binds it.

*Faults.* A mated part that does not resolve faults its mate in the
resolver's own voice, carrying the part fault unaltered:
`MateFault::FaceUnresolved` (`FacePoseRefusal::PartUnresolved`, in
`FaceRefusal::Reach`) where a face-based side stands on it, since a
side's frame is read before the lever, else `MateFault::Unleverable`.
That fault faults its placement as any mate fault does.

Two questions do need a number: a pattern's count and a `Part`'s index,
which no box or seed binds, so they are read at the nominal in every
run. They are asked once per reference, where the solve reads it, for
every reference of every live constraint, not only the ones a bundle's
fold derives. The named copy must exist (its index against the
pattern's evaluated count, else `MateFault::DanglingHead` at the
pattern), and a `Part` directly above a pattern must select the copy the
name names (else `MateFault::PartSelectsAnotherCopy`, which reports
both). The name is the authority; the `Part` is checked against it.

A member's derived pose refuses in the placer's own voice: a pattern
copy or a transform on the chain whose pose cannot be derived refuses
`MateFault::PlacerRefused`, holding the evaluation layer's own typed
cause unaltered. Where a bundle's fold derives the offset, the fault
reaches the instance under the placer, so the placer is poisoned and
never states that cause itself: the fault carries it, drawn as its own
line under the constraint's. Where the solve reads one mate's references, the
fault reaches that mate alone; the placer fails in its own right and
states the cause on its own row, and the mate points there.

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
- **AQ8 — the crossing record's reachability.** A kept constraint whose
  target is a cut copy crosses the cut, and populates `InterfaceRecord`,
  so no conversion door is needed. **Only a mate both of whose sides
  resolve to members can cross** (ruled: option (b), SKIP): a mate a
  side of which resolves to no member — a head outside A11's member vocabulary, or a
  read of nothing live — contributes no interface crossing however its
  names fall across a cut. Such a mate never solved, and a record minted from it would
  assert a relationship evaluation never established: trusted-at-rest
  state, which this design forbids. The mate stays in the document and
  its names rebind like any other (N5); it says nothing about the seam.
  The split collector gates on `member_of` for both sides, the same
  predicate the solve's member walk asks.
