# Assemblies and mates

How editor-core keeps DESIGN `[assemblies]`: an assembly is an
ordinary document whose leaves instantiate other documents, a reference
pins its document's content, and a held value names the document it came
from. What a placement, a mate and a value are is DESIGN `[placement]`;
what checks an assembly at rest is DESIGN `[assertions]`. The census and
contact vocabulary is `crates/topo/README.md`; the user-facing walk is
`docs/guide/assembly.md`.

## Where in the code

| Decisions | Modules |
|---|---|
| A2 evaluation seam, memo | `src/part.rs` (`PartResolver`, `ResolveFault`), `src/eval/parts.rs` (`PartStore`, `Reached`, `PartFault`); `graft_disjoint_all_keyed` in `crates/topo/src/instance.rs`; the copy's frame in `src/eval/framed.rs` |
| A2a pairing doors | `mispaired`, `Mispaired` in `src/ident.rs`; the doors in `src/product.rs`, `src/mate/solve.rs`, `src/checks.rs`, `src/resolve/mod.rs`, `src/resolve/pick.rs`, `src/stackup.rs`; the memo's drop in `src/eval/mod.rs` |
| A3, A11 placements, mates, solve | `Node::Place`, `Node::World`, `Node::Space`, `InstantiatePart`'s `target` and `constraints` in `src/node.rs`; `src/mate.rs` (`Constraint`, `Mate`, `Value`, `Coord`, `MateRef`, `MateFault`), `src/mate/orient.rs` (`Orientation`), `src/mate/coset.rs`, `src/mate/solve.rs`, `src/mate/value.rs`, `src/mate/pin.rs` (`Placed`, `stand`), `src/mate/corner.rs` (`corner_frame`), `src/mate/unobservable.rs`; the fold in `src/eval/wire.rs` (`fold_with`); `DocEdit::SetConstraints`/`AddConstraint`/`RemoveConstraint` in `src/edit.rs` |
| A4, A13 identity, pins, update | `src/ident.rs`, `src/update.rs`, `DocEdit::UpdateReference` in `src/edit.rs` |
| A4 split, inline and flatten | `src/refactor.rs`; `InterfaceRecord` in `src/node.rs` |
| A5 at-rest check | `src/checks/at_rest.rs` (`AtRestEvidence`, `InterferenceFinding`, the quieting rule), `src/checks/at_rest/carried.rs` (`Route`, `CarriedInterference`, `CarriedAtRestFinding`, `CarriedCoincidence`, `AtRestRow`); `CheckId::AtRest` in `src/checks.rs` |
| A6 improper frames | `src/placement.rs` (`Frame`); `Node::Mirror` in `src/node.rs`, `wire_mirror` in `src/eval/wire.rs`, `mirror_body` in `crates/topo/src/mirror.rs` |
| A7, A8 interchange | `PlacedInstance` in `crates/step-import/src/lib.rs` |
| A9 partitions | `relative_freedom_components` in `src/mate/solve.rs` |
| A11 (2) bundles, spaces | `solve_bundle`, `fold_bundle`, `overconstrained`, `pins` in `src/mate/solve.rs`; the space derivation (`Doc::space_of`, `node_space`, `space_fault`) in `src/doc.rs`; the one cross-space predicate `Doc::across_spaces`; an instance's copy in `src/eval/wire.rs` and `src/eval/parts.rs`; picks (`VarDef::Pick`, `pick_fault`) in `src/var.rs` and `src/edit.rs` |
| A10 the product slot and the gather | `Node::Place`, `Node::World`, `Node::Assemble` in `src/node.rs`, `Doc::product` in `src/doc.rs`, `src/product.rs` (`product_recorded`, `assemble_gathers`), `DocEdit::SetProduct` and `DocEdit::place` in `src/edit.rs`, `mate::Placed` and `mate::stand` |
| Store (AQ1) | `Workspace` in `crates/pncad/src/workspace.rs` |

## Scope

**A1 — The scope ladder.** Four rungs in sequence: (a) the body graph
(instances of pinned part documents, rigid frames, patterns); (b)
mates between copies, solved constructively; (c)
numerically solved mates (SE(3) witnesses under the witness contract in
`crates/editor-core/README.md`); (d) kinematics. (a) and (b) are built;
(c) and (d) are not, and A11's refusals point at (c).

## Evaluation

**A2 — An assembly is a document; its evaluation is a body.** An
instantiate node resolves its `DocRef` through the evaluation's
`PartResolver` (`EvalOptions::resolver`; with none, instantiate nodes
refuse typed), evaluates the pinned document at the ambient ε, takes its
product (A10), the one `Body` its product slot reads, a `⊔`'s pieces
side by side (no body has world coordinates: the world sets only the
coordinates export writes), and its copy is that body, shared, under the
instance's map composed onto the product's frame (D10 `[placement.frame]`):
nothing is rebuilt, and export composes the frame into the world. A resolved document whose recorded ε disagrees refuses
`ResolveFault::EpsilonSeam`. An evaluation enters each `(DocRef, ε)` it
instantiates at most once (`PartStore`), before its mate solve and node
map read it (`Reached`); a part the prior evaluation carried under the
same pin is reused rather than evaluated again, and a faulted one is
asked again. A reference already on the descent chain is a
cycle and refuses naming the loop. Which solids are one part, and that
fusing is an explicit boolean, is DESIGN `[assemblies.document]`.

**A2a — The pairing doors.** The rule is DESIGN
`[assemblies.pairing]`: a door that takes a document together with a
value that must be of it refuses a mismatch, typed, before reading
anything of the second. What it compares is the identity stamp
(IDENTITY DI3), by the one predicate `ident::mispaired`, which each
door maps into its own error vocabulary. The rule binds only where both
halves carry an identity: a `Doc`, or a value stamped with one (an
`Evaluation`, a `SolvedPoses`, a `NodePick` and the `PickTarget` it
mints). A value with no provenance of its own (`resolve::hit`'s raw-key
name doors, a bare `StableName`) is outside it because there is nothing
to compare; #1098's raw-key class closes by handing a stamped value
instead, which is what `NodePick` is for the pick doors. A later
evaluation of the same document is admitted at every door: a pairing is
about identity, never version (DI3), except at the stackup doors below.

The doors that refuse:

- `product` (with `product_named` and `product_recorded`,
  `ProductError::EvaluationOfAnotherDocument`);
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
  (`HitTestError::EvaluationOfAnotherDocument`), below;
- `stackup::sensitivities` and `stackup::stackup`
  (`BindingViolation::EvaluationOfAnotherDocument`), for the handed
  build of record, which DUAL-DESIGN DL3 binds by revision as well.

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

## Nodes and mates

**A3 — The node vocabulary; mates place copies.**
`Node::InstantiatePart { doc_ref, interface, target, constraints }`
instantiates a pinned document, and is a placement: its one output is a
copy of the part's product in its `target` space, moved by the motion
its constraints pin. A copy carries
its source's names as they are, so a selection names a face of it by the
part's own name, read at the instance. A reader of one piece of a copy
reads a pick (`VarDef::Pick { placement, of }`): the path through the
source's placements and `⊔` operands to the piece's variable, which the
door refuses through any other operator (`EditError::PickPath`).
`Node::Place { source, target, constraints }` copies the one body it
reads into `target`, a `Space` read. The world, `Node::World`, is minted
with every document at `RecipeNodeId::WORLD` and defines the world's
space (`VarId::WORLD`); `Node::Space` mints another space. A placement
reads a space only as its target, and no mate reads one. Repetition is
an index variable (D10, Repetition): a placement whose reads reach an index places one copy per
value of it, and no node is a pattern.

A placement's `constraints` are its bundle, each a `Constraint { id,
kind }` addressed by its placement and the id minted with it
(`MateRef`), never by its position: a mate or a value (FORK-S3M).

A `Mate { on, to }` equates two poses of one kind, and the kind
is the primitive: `Frame`–`Frame` coincides, `Axis`–`Axis` is coaxial,
`Plane`–`Plane` rests, `Point`–`Point` is a ball and
`Direction`–`Direction` is parallel, each pinning the relative pose to a
coset of the kind's `Subgroup` (A11 (1)).

- `on` is a pose read off the copied source's geometry (a `Place`'s
  source, an instance's own copy; else `MateFault::NotOnTheCopy`).
- `to` is a pose read off geometry of the space the copy joins. No
  space has roles a mate can read: there is no world frame and no part
  frame to read.
- A mate holds no number. Its sense is a `Flip` on one side: a plain
  `Plane`–`Plane` mate equates outward normals, and two faces stacked
  against each other carry a `Flip` on the target.
- A mate declares no contact. It places a copy; whether two copies
  touch is the at-rest census's to find (A5).

A `Value { coord, by }` names no mate: it is a coordinate of the
target space, setting one freedom the bundle's mates leave to a
`Length` or an `Angle` (`Coord::Slide(X | Y | Z)`, `Coord::Spin(None)`
beside a mate fixing a direction, `Coord::Spin(Some(X | Y | Z))` for a
chain): a plane mate's two slides and its spin, an axis mate's one
slide and its spin, a direction mate's three slides and its spin.
Where a mate fixes a direction `d`, the mate's `orient` (an
`Orientation`, one of the 24 ways the copy's axes lie along the target
space's, the identity unless written) puts the copy's mated direction
`d_c` at `n = O·d_c`, and the least turn `S` carries `n` onto `d`; the
copy's turn is `rot(a, θ)·S·O`, `a` the target's own direction (its
outward normal for a plane: `d` with every flip on the target's side
undone, `mate::flipped`). A slide is the copy's origin measured
from the target space's along a target axis as `S` carries it, and the
spin `θ` is the turn left about `a`, whatever the contact's sense, so a
flip written on either side of a rest turns its spin the same way
(`mate/value.rs`, `Chart`). Where
`n` is a target axis, the slide `S` carries onto `d` is the one a plane
mate fixes and an axis mate leaves; otherwise the written slides are
admitted only where they span what the mate leaves
(`NoChartWhy::SlidesDoNotSpan`). The one pole, `n = −d`, is the
orientation written backwards: inside the band about it the chart
refuses `MateFault::Backwards`, naming the orientation to write. The chart reads no body a pose is read off, so a value edit
never rides with one. Where no mate fixes a direction (no mate, or a
lone point mate), the rotation is a chain of at most three spins about
the target space's own axes, through the mated point for a point mate,
no two in a row about one axis, applied in the order written, and the
slides run along those axes. Zero says the copy's own coordinates agree
with the target space's as far as the mates allow.

The door admits a constraint on what the recipe shows: two poses of one
kind, `on` reading the copy, and of the bundle that no
constraint fixes an equation the rest already fix and every value sets
a freedom the mates leave (A11 (1)). The placement folds its bundle
where it evaluates (`eval::wire::fold_with`), reading every pose as a
value: its mates' cosets met, then its values placed inside what they
leave. A bundle is written by one pure constructor (`Placed::of`) from
the mates and moves its caller wrote, with the constant `0` on each
freedom left unnamed; it reads no geometry, so a copy stands exactly
where its author means it. A gesture names the slides from the
source's geometry before calling the constructor: it reads `n` off the
evaluated copy and writes the slides `mate::slides_left_along` leaves
(`Recording::sketch_on_face`, `Recording::constrain` and `remove_mate`,
the viewer's mate tool and snap). The raw edit doors stay structural,
so where the slides they fill do not span the solve refuses
`SlidesDoNotSpan`, naming the slides to write. A copy pinned in the world at a pose is that
pose's moves written as values on the world's axes; the "place part"
gesture writes the identity bundle (`mate::stand`). A frame read off
geometry is `mate::corner_frame`'s, which a caller asks before writing
the moves it means.

**A12 — Mate sides are reads.** A mate side is a read like any
other, so a mate depends on the bodies its faces are read
in, A9's partition runs over reads (D10, Operations), and a cut that
separates a mate from a body it reads is D-2's ordinary severed read. A
side whose body is deleted leaves the mate unresolved and typed (the
placement's `MateFault::SideUnevaluated` where it evaluates), and a name
that stops resolving is the selection's refusal, repaired by `Rebind`
addressed by the body. A mate places
nothing by itself: it is a constraint of the placement that holds it,
and only that placement's copy is in the product (A10).

## Identity, pins, split and inline

**A4 — Identity, pins, and the split/inline pair.**

*Identity, pins and saving* are DESIGN `[assemblies.pins]` and
`[assemblies.saving]`. Here: the resolver returns a document only when
its bytes hash to the pin, else `ResolveFault::PinMismatch`, and that
refusal is the seam's alone (IDENTITY DI2): an evaluation that crosses
the seam refuses a moved pin, and one served from a prior serves what
the document pins. An evaluation carries the id of the document it is
of, and `SolvedPoses` the id of the document it solved (DI3); which
doors read that identity is A2a's. The two save acts (Ev, PR 2016) are
built at the workspace door (DI4).

*Split.* `refactor::split` cuts a node set closed under the DAG in both
directions into a new document with a caller-supplied id, leaving one
`InstantiatePart` behind, standing where it stands (`mate::stand`: the
identity bundle, no mate and the constant `0` on every freedom); a severed
edge refuses. A cut takes whole placements: a placement's constraints are
its own, so a cut placement whose bundle reads a kept copy severs that
read. The part's product is the cut's one world copy, or the `⊔` of
its world copies in document order, so the instance's copy stands
where the cut's copies did. A remainder read of the cut crosses only
where the instance carries what it reads: a selection each of whose
names one cut world placement's copy holds, re-authored on the
instance's copy; a read of a cut world placement's copy, or of a body
one cut world `Place` copies at the identity, re-pointed to a pick of
the instance's copy (the member of the part's `⊔` that copy is, or the
copy whole). A kept mate whose pose reads a cut copy's geometry crosses
with its pose written afresh over the instance's copy, each selection
in it re-authored as a crossing selection is; a body's face read
through a cut placement crosses only where that copy stands at the
identity, decided by evaluation. A remainder-side name re-anchors to
the name the part's product spells it by, which the instance's copy
carries verbatim, by a recorded `Rebind`. A variable moves with its readers: its side is the
union of its readers' — a node slot, or another variable's definition —
and one with no reader follows what it reads. One whose side is the
cut's is declared in the part and deleted from the remainder.

*Inline.* `refactor::inline` is the inverse, by one rule. It mints a
`new_space()`, copies the other document's nodes into the host
verbatim with that document's world read as the new space, and
rewrites the instance as one placement of the spliced product body
into the instance's target under the instance's own bundle, verbatim
(`InlineOutcome::placement`, its new space `InlineOutcome::space`). No
motion is composed, every variable keeps driving its copy, and no
landing refuses for its numbers. A host read of the instance's copy
reads that placement's copy, and a pick of it a pick of that copy; a
host mate reading it is written afresh over the spliced copy its names
land on. It refuses a document with no product.

*Flatten.* Flattening an identity placement into its target is an edit
of its own, not a case of inline's: `refactor::flatten` takes a `Place`
whose bundle is the identity as written and whose source is in a
`new_space()` no other placement copies a source out of. Every
placement into that space is re-targeted to the placement's target,
every read of the copy re-pointed to the shape it copies (a name of
the copy re-anchored on the shape's, an appearance key re-attached
there), and the placement and the space are deleted. Anything else
refuses, typed (`FlattenError`).

Split, inline and flatten compute no frame. Each is pure, returning
values plus edit lists.

*Acceptance.* Split-then-evaluate equals unsplit evaluation at
structural and name-resolution identity, not bit identity, up to the
order of copies: the cut's world placements become the part's, its
product their `⊔`, and the remainder holds one instance of it. Inline-of-split, with the identity
placement it lands flattened into its target, returns the document
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

## Mirror

**A6 — Mirror and improper frames.** Every pose is proper. `Frame`
stores a general linear part, and a frame that is not a rigid motion, a
mirror (det = −1) among them, is refused wherever a document admits a
frame, by one predicate (`Frame::admission_fault`): a placement rule's
listed frame refuses `EditError::NonRigidPlacement`, naming which frame,
and the load validator refuses the same. A mirror is not a placement: a
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
contact records are not exported.

## Relative freedom and the product

**A9 — Relative freedom is component structure.** Two copies are
relatively unconstrained exactly when they lie in different connected
components of the recipe under its reads (`Doc::upstream`;
`relative_freedom_components`); no solver, no geometry. Evaluation
stays deterministic, and every copy pinned against the world has a world
pose, so the world is one body. A copy in another space (A11 (2)), a
`new_space()` or a construction's own, is compared with nothing outside
it, and is not part of that body. The viewer's free-move probe is display state, never in the
document: a save keeps it beside the document as viewer data (IDENTITY
DI5; `crates/viewer/src/layout.rs`).

**A10 — The product is a slot.** A document's product is the body its
product slot reads (`Doc::product`, set by `DocEdit::SetProduct`), a
body in the world: one copy, or a `⊔` (`Node::Assemble`) of several,
each operand one arrival of the census. A placement (`Place`, or an
instance) is an operation reading one source and defining one copy, so
two placements of one body are two copies; an instance's copy is a copy
of its part's product. Its constraints relate the copy to the world,
directly or through the copies it is pinned against, and only the
gather, export and a constraint read the world. Nothing else places: no
edit places or unplaces as a side effect, and nothing derives a
placement or a product from what reads what. A slot that reads nothing
is a valid document with no product; a door needing a product refuses
`EmptyProduct`. The door refuses a product read that is no body in the
world (`EditError::ProductRead`), and a product read whose body is
deleted is a stranded reader (D10): the gather refuses
`StrandedProduct`. Split and inline move placements with their bodies
and re-point the slot where they take what it reads (A4).
`product::product` gathers the slot's body, a `⊔`'s pieces each its own
source; a `⊔` the slot does not read is gathered by itself
(`product::assemble_gathers`) and checked by the at-rest check (A5).

## The constructive-solve boundary

**A11 — Five rules, decided structurally.**

**(1) A placement's mates fold by subgroup algebra, measuring nothing,
to DETERMINED or UNDER, or refuse OVERCONSTRAINED (a constraint any of
whose equations the bundle already fixes, named)** (FORK-S3O). Each
primitive pins the copy's relative pose to a coset of an SE(3)
subgroup, the target pose's symmetry; one `Subgroup` type is both a
pose's symmetry and what a mate folds:
`Subgroup::{Se3, Planar, Cylindrical, Prismatic, Revolute, Spherical,
Parallel, Translation, PlaneTranslation, Trivial}`, the closure of the
pose kinds' symmetries (D10). A constraint is admitted by the families
its definitions name (`SubgroupFamily::meet`, `transverse`): two mates
only where their families meet transversally, their codimensions
adding, which of two mates' families only a ball's and a direction's
do; a value only on a freedom its mate's kind leaves, set once, and
which every other mate leaves too (a slide beside mates freeing every
translation, a spin beside mates freeing every rotation). The solve
reads a slide as a translation equation and a spin as a rotation one
and settles each inside what the mates leave, so a slide beside a ball
refuses `MateFault::NoChart` (`SlideTurnsABall`): through the copy's
lever it would fix the ball's turn, which is no repeat, but the ball
leaves it no translation to set. Where the mates repeat an equation, the mates are named and no
value. The rule is
order-free and decides determination by codimension alone, so a copy's
space is the recipe's, never a value's (`mate::solve::overconstrained`,
`pins`). A pinned copy is the case `(Trivial, _)`. The cosets a bundle
admits are met by construction and never checked
(`mate::coset::intersect`). The edit doors (`SetConstraints`, an
inserted placement, a re-pointed side, and `AddConstraint` and
`RemoveConstraint(s)`, each of which rewrites the bundle whole: the
mates kept by id, the values the new mates still leave kept as written,
and the constant `0` on each freedom left unnamed) ask each mate's own
admission (`mate::solve::admit_mate`) and the bundle rule, refusing
`EditError::MateRefused`; the load door
asks the constraint's own admission (`SnapshotError::MateRefused`), and
the fold asks the bundle rule again for a state no edit door saw (a
loaded snapshot).

**(2) A placement is the bundle of its constraints.** Each placement's
bundle is folded inside the placement's own op, where it evaluates
(`eval::wire::fold_with`): its mates' `on` poses read off the copied
shapes before placement, their targets read in the space the copy
joins, each mate's coset intersected, then its values placed inside
what the mates leave (`mate::value::settle`). The copy's pose is the
folded representative in its target's space. A placement names its
target space (`target`, a `Space` read), and each mate's `to` reads a
body of it: the edit door refuses one that does not
(`EditError::SpaceMismatch`), so the fold reads every mate's target in
that space. Every face is a value there, so a mate reads a face of any
copy.

A placement is parametric (Ev, `[ev]` #3437, 2026-09-29): its numbers
are its values, each a `Length` or an `Angle` slot, so a document
parameter can drive where a copy sits.

Every bundle pins its copy: one that leaves a freedom unwritten, an
empty one included, refuses `MateFault::Unpinned` at the door. The copy
is in its placement's target space, the world's or another's. Nothing
outside a space is compared with it: a read across two spaces refuses
(`NodeErrorKind::SpaceMismatch`), decided from the recipe, while
everything inside a space solves as usual. STEP export reads the world;
the at-rest check reads the world and every `⊔` beside it, each
against nothing outside it. The viewer shows a set of spaces, each at a
display frame that no logic reads (IDENTITY DI5).

No edit records a frame: removing a constraint, deleting an instance or
a body is never refused for the placement it removes, what is pinned is
recomputed from what remains, and replay re-applies the edits alone,
without solving.

**(3) A constraint beyond the pin refuses** (FORK-S3O, #4326).
`MateFault::Overconstrained` names every constraint any of whose
equations the rest of the bundle already fixes, never "the later one":
constraints have no order. A mate that would take a freedom a value
sets refuses the same, and so does a value beside one of its own
freedom. Where two copies meet beyond what their mates fix is a contact
like any other, recorded and linted, and its recourse is an assertion
(`OVERCONSTRAINED_RECOURSE`). A configuration degenerate at its values
refuses as its construction does (`PoseDegenerate`).

**(4) The solve is total and per-placement.** No cycle is ever solved:
a placement reading its own copy, but for an instance's copy side
(which reads the part's face, a pose the instance's motion carries, not
one it computes), is a read cycle the door refuses. A refusing bundle
faults its own
placement (`SolvedPoses::fault`), nothing else. The solve states where
each copy is and what each constraint decides, never that a product
exists: whether the document has a product is the gather's question
alone (A10), so a product refusal is not a mate fault, and a document
can solve whole and still have no product.

**(5) World poses, members, and what the solve reads.**

*World pose.* A copy pinned against the world has a world pose, its
targets' composed onto its solved relative pose; a copy pinned to the
world at a pose returns that pose's frame bit for bit. A copy in
another space has no world pose (`PoseRefusal::OtherSpace`), only its pose in its space.
`SolvedPoses::placement` is one of A2a's pairing doors: the document it is handed must be the one
solved, else `MateFault::PosesOfAnotherDocument` before any frame is
read.

*The member walk.* A face selection resolves by walking from its
operand down to a live placement, through any number of indexed reads,
families (each of which the name qualifies `Member`, keyed by the
family's indices) and `Union`s (each of which the name qualifies
`From`). It decides what a crossing names (`mate::member_of`); the fold
reads no walk, since every face is a value where it evaluates.

*What the fold reads.* The fold's inputs are the poses its mates read,
evaluated at the evaluation's own scalar over its own parameters, and
its values' numbers. It reads one more number: the copy's REACH, an
upper bound taken from the copy's own evaluated geometry. The reach
enters as the lever a value's chart is decided over,
`R + max(‖on.origin‖ + ‖to.origin‖)`, with no floor and no constant. The
arm's one door (`Arm::of`) refuses a lever whose sixteenfold square
overflows, so
the bundle's meeting is always a finite length away. The reach is also
the turn's lever in the placed copy's at-rest verdict (`rest_verdict`).
Outside the fold, the unobservable report (`mate::unobservable`) reads
the copies' reach again, as the lever of its own verdicts. The fold
constructs its coset and places its values, deciding nothing about
whether they hold, with no numeric fitting. At `f64` it is the nominal solve; in a seed run each
solved pose carries its tangent, in a box run each is an enclosure over
the box.

*Faults.* A pose a mate reads that does not evaluate faults the
placement `MateFault::SideUnevaluated`, carrying the evaluation layer's
own refusal; a copy whose reach is not in hand faults it
`MateFault::Unleverable`. A target read off a node that failed poisons
the placement through that node, whose own row states the cause.

## At rest

**A5 — The at-rest check.** At each `⊔`, the census examines every
pair of arrivals the census's pre-filter cannot prove apart, and decides
each one apart, in contact, overlapping or undecided. A contact is
recorded at the coincidence door; unless it is structural, the
`unproven-coincidence` lint reports it, quiet under a holding `Gap = 0`
between its two cells. An overlap is an `AtRest` interference finding,
localised to the faces bounding it, or loud and unquietable when the
intersection refuses; a finding no assertion the quieting rule accepts
could quiet refuses at no severity. An
undecided pair is an `AtRest` could-not-look finding, which refuses at
no severity (CHECKS DS6). Each finding is
quiet or loud. Nothing refuses: a caller that wants a gate runs
`enforce_checks` at `Error`.

The census runs once, in the gather (`product::product_recorded`, and
each `⊔`'s own census, `product::Census`), which keeps its findings on
the product (`Product::census`); the check (`CheckId::AtRest`,
`checks::at_rest`) reads them and runs no census or predicate of its
own. Every touch between two copies that no carried record backs, a
seat a mate set up or a touch no mate relates alike, is recorded
(`topo::record_at_rest`, each arrival its own group) as a
`CensusAtRest` row of the product (`Product::coincidences`, an
`AtRestRow` naming its two copies) and a record citing it, which the
census's next pass confirms. The coincidence door proves a mated seat's
row from the mate's own statement (`[coincidence.structural]`). Two
copies' planar faces meeting over an area are one such row, the face
pair. Two faces of two copies on one carrier (a plane, a cylinder or a
sphere), facing the same way, whose boundaries share a run of an edge
and whose interiors do not meet, are a flush pair: a row of its own (`topo::AtRestPolicy::flush_at_rest`) that no
record cites, reported unless the door proves it, and never quieted by
an assertion. A pierce or a same-side crossing is no touch: it is an
overlap between two copies. The findings follow the gathers: the
product's, then each `⊔` the product does not read, in document order,
each under the `⊔` whose census decided it (`FindingSubject::Census`).
What the check could not look at names why (`AtRestEvidence`): a pair
the census had no lane for or left undecided, a body failing a check of
its own structure, a meeting inside one arrival no carried record
backs, a `⊔` whose operands did not gather, or a scalar that runs no
census. A disjoint assembly reports nothing.

*Across the seam.* A sub-assembly's records ride through the seam in
what its product body carries (`PartValue::carry`), each citing the
part (`CitedInput::Part`). An instance's copy carries them as every
copy carries its source's (`Carry::copied`, the one door every copy's
carry goes through). A
record's faces are re-keyed at every graft through the graft's own
descendant map. Nothing is re-verified across the seam.

*A part's own census findings.* A part crosses the seam as one copy,
whose meetings its own censuses decided. The part's reading of them
crosses with it, its product's and each `⊔`'s its product does not
read, as its own at-rest check reads them
(`checks::at_rest::whole_reading`, at the part store): each overlap
between two of its copies rides
`PartValue::interference`, and each pair it could not look at rides
`PartValue::at_rest`, each finding routed (`Route`: the instantiating
node, the document that found it, and the instances below). An
instance's copy is censused (`Carry::censused`): a census reading
it as one arrival discards what it finds inside it
(`product::gather_sources`), as it discards what it finds inside any
arrival a door already censused (a `⊔`, or a value that carries
one's as a copy carries it: a placement's copy, a family's members, a
pick of a piece, a union of one body, a mirror's image).
The kernel's
census has no group scope, so it still computes inside such an arrival,
and the door discards what it found. Each carried finding is reported
once per part row, by the first route in gather order, so a part placed
twice reports its overlap once, as the part's (`CarriedInterference`,
`CarriedAtRestFinding`), quiet or loud as the part's own assertions
judged it, and never refusing. The rows a part owes its own coincidence
door cross the same way, unproven (`PartValue::door_rows`, each with the
part's pinned document): the host's door judges them over that document,
as the part's own door would, and reports each unproven one once per
part row at the route's first instance, as the part's
(`CarriedCoincidence`), quiet only as the part's assertions quiet it. A
host `Gap` quiets none of them, wherever its cells lie, since a finding
belongs to the door that decided it (D10 `[assertions]`). A loud one
refuses at `Error` exactly when the part's own finding would, under the
host's `unproven-coincidence` severity. A finding that names no pair of
solids (a census escalation, a pair it had no lane for, a stale record of a
vertex or an edge) is inside no one arrival, and is kept: one the census
re-finds inside an instance's copy is reported as the host's beside the
part's carried row, until the finding names its pair and the discard
places it (`work/contacthold/an-escalated-census-finding-names-no-pair.md`).
What does not cross: the census of a part's spaces other than its
world, and the rows
of a part read one member at a time rather than whole
(`work/intent/a-parts-loose-spaces-and-member-reads-drop-its-census-rows.md`).

*Interference.* An overlap of two copies' material is an `AtRest`
finding of its own (D10; `checks::at_rest`). The census decides that
two copies interfere: a vertex of one strictly inside the other's
material (`ValidationError::InstanceInterference`), a face of each on
one plane, cylinder or sphere facing the same way and sharing interior
(`ValidationError::SameSideInterference`), a touch where the two
materials meet (`ValidationError::TouchInterference`), an edge piercing
a face (`CensusContact::EdgeFacePierce`), an in-plane crossing whose
side test reads `SameSide`, or an edge of each on one line whose wedges
overlap (`ValidationError::CoincidentWedgesOverlap`). The check takes
those verdicts, with every
`CensusUndecidable` the census left on the same pair of copies, and
intersects the two copies. Each connected solid of the intersection is
one `InterferenceFinding`, named by the faces of both copies that bound
it. An intersection the kernel cannot form leaves one
`Overlap::Unlocalized` finding for the pair, which is loud and which
nothing quiets; so does a decided overlap at an analysis scalar, which
has no localizing lane (`LocalizeLane`: `f64` builds bodies). A pair the
census decided nothing about is a could-not-look finding.

An interference finding is quiet when a holding assertion reads a
`Gap`'s output directly over an opposed pair of faces of the two copies
(the material pairing door decides opposed), admits only
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

A contact row is quiet when a holding assertion reads a `Gap`'s output
directly, relates it by `=` to a bound decided zero, and its two
selections are the row's two cells, in either order, each the read its
arrival entered the `⊔` through and the cell's name there, at whatever
dimension the cells are (`checks::at_rest::quiet_row`; the
`unproven-coincidence` lint reads it). It matches cells, so another
contact between the same two copies stays loud, and it reads verdicts
only, so a failed assertion quiets nothing.

## Open questions

- **AQ1 — the document store.** Built: `pncad::Workspace`, a directory
  of `*.pncad` files scanned by id header, one file per id, resolving
  `DocRef`s with the pin gate. Open: what an id anchors to beyond one
  directory, and how a one-version-per-id store serves mixed-pin state.
- **AQ2, AQ3, AQ7** — discharged into A13, A11, A12.
- **AQ4 — per-instance arguments.** Not implemented. Intended form:
  `InstantiatePart { …, args }` applying the pinned recipe at `args`.
- **AQ5 — in-context capture semantics.** Open.
- **AQ6 — cross-document `Rest` verification.** Not a question: a mate
  declares no contact, and a touch between copies is a census row the
  coincidence door proves. C6's interference fits are stage 5's
  question.
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
