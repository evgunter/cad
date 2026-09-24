# What a recipe reference may be

This page is the ratified design of record for the editor-core
recipe's reference vocabulary: what a node may point at, and what each
kind of pointer means. It answers three of DOCM's questions — derived
frames, operand selectors, deleting from the middle of a chain — as
one conversation, because each asks for a new combination of the
reference shapes the recipe already admits (§0). Ev ratified DM1–DM6
in chat on 2026-09-04; DM4 gained its member-space declaration channel
on 2026-09-06 and was bounded at DOCM-8's review (2026-09-07); DM7 and
DM8 were ruled on EDIT's `[ev]` PR of 2026-09-16. The page
was `docs/DOCM-REFERENCES-DESIGN.md` until DOCM's exit on 2026-09-13,
when it moved beside the code it governs. Identity across time is the
companion page
`crates/editor-core/IDENTITY.md`. Mechanics here are measured, not
assumed; where a file:line has drifted, the name beside it is the
stable half.

## 0. Grounding (committed elsewhere; not re-litigated here)

The recipe admits three reference shapes, and every node is built
from them:

- **A DAG edge**: a `RecipeNodeId` in a node's inputs, structural,
  liveness- and cycle-checked at the edit door (`edit.rs`,
  `InsertNode`: `UnresolvedInput`, `WouldCycle`), enumerated by
  `Node::inputs`. Ids are minted by the document's monotone counter
  and never reused (D3; `doc.rs`).
- **A frozen `StableName`**: `{ kind, node, path }` (N1,
  `names/role.rs`) stored at authoring and resolved at evaluation
  through a name table under the N5 ladder — `NodeGone`, then
  `Ambiguous`, then `Vanished` — never silently shrunk
  (`eval/wire.rs`, `ladder` and `resolve_selection`). Carriers:
  `Fillet`/`Chamfer` selections, `Shell` open lists, a
  `Datum::FaceFrame`'s face, `Declare` pairs, `Mate` heads,
  `Measure` refs, an `InstantiatePart`'s interface crossings' `outer`s
  (`Node::payload_names`; a crossing's `inner` is not a name of THIS
  document, and that list's arm is the one home for why). This clause
  and `Node::payload_names`' own doc are the list's TWO homes, and
  every other site in the tree points at the latter rather than
  restating it. A name is
  not a DAG edge, and TWO doors refuse on one: `InsertNode`'s liveness
  check, and `split`'s `PartNameReachesRemainder` precondition, which
  refuses a cut whose taken node carries a name reaching the kept
  remainder — a `Declare` pair's, a `Mate` head's, an instance's
  crossing `outer`. A later delete strands a name (N5) and says so
  (DM7).
- **An `Expr` literal** in a slot, bit-pinned (D7).

Two precedents these clauses extend. `SitedRef { at, name }`
(`node.rs`) pairs a DAG edge with a frozen name: `at` says which
evaluated value to read, `name` says which entity. `Datum::AxisInPlane
{ plane, .. }` (`node.rs`) is the one datum with a DAG input: its
meaning comes from another node, and the design note there says why —
the check is not made cheaper, the error is made unrepresentable.

Ruled elsewhere and kept: a selection FREEZES (#217, `node.rs`);
selectors MATERIALIZE and are never stored (`names/select.rs`);
`PlacedUnion` is a node beside `Pattern`, not a flag on it, because
forking a result type on a variant is the silent-dispatch trap D3
forbids (`node.rs`); `Datum::Frame` carries nine `Expr`s and no
reference, orthonormalized at evaluation (`node.rs`).

## DM1 — A derived frame is a datum carrying a face name

`Datum::FaceFrame { at: RecipeNodeId, face: StableName, spin: Expr }`
is a `Datum::Frame` whose pose is computed at evaluation from a
named face: `at` is a DAG edge to the body node whose value the face
is read out of, `face` is a frozen face name resolved through that
value's name table under the N5 ladder, and `spin` is the authored
rotation of sketch +x about the normal. It is the `SitedRef` shape
applied to a datum. The frame it yields: origin at the carrier's
origin projected to the face's plane (the carrier's own distinguished
point, `readback.rs` rule 2), normal the face's OUTWARD normal (DM1a),
sketch +x the carrier's u-reference rotated by `spin`.

- **Why derived, not frozen.** The profile-plane migration deleted
  the twelve-float snapshot a sketch used to carry (`program.rs`); a
  frame read off a face and written into nine literals reintroduces
  that snapshot one node out, and lies about why it sits where it sits.
  A derived frame is a DAG input: the face's body is upstream, the
  frame moves when the face moves, and it participates in the memo and
  content key like every node.
- **The failure mode is the fillet's.** A face name that stops
  resolving fails the frame typed and poisons the sketch above it,
  exactly as a fillet's selection does (`BlendSelectionResolve`); the
  repair is `Rebind`. It is the first datum with an N5 failure mode,
  and that is the honest behaviour.
- **DM1a — the read-back grows a sense, it does not fold one in.**
  `Pose.axis` is the CHART's direction, deliberately uncorrected by
  the face's orientation sense (`readback.rs`): two facts, two
  answers. The sense travels BESIDE the pose as `Pose::sense`, one
  more stored value copied out of the face record (`entity.rs`,
  `Face::sense`), returned by `face_pose` and
  `names::interrogate::face_frame`; DM1's datum states in its own
  vocabulary that its normal is sense times chart axis. The mate
  tool's frozen frames are unaffected: A11 keeps the solve over
  authored numbers, and that asymmetry is principled — a sketch frame
  is consumed by evaluation, which reads geometry constantly; a mate
  frame by a solve that must not.
- **DM1c — the lanes.** A derived frame has no document elaboration,
  so the profile-lift's "the sketch plane stays f64" fence
  (PP6, `crates/editor-core/README.md`)
  does not apply to it: a profile on a derived frame is placed at the
  lane scalar through the by-value reader (`frame_plane_lane`) under
  every lift, its 2-D structure record staying f64-pinned as PP1 says,
  and a loft or sweep section on a derived frame refuses typed at any
  scalar but f64, since a section's geometry stays f64. An authored
  frame's profile is unchanged. Two placement paths keyed by frame
  kind, because the two kinds differ in where their numbers come from.
  PP6 carries the same sentence.
- **DM1b — a non-planar carrier refuses typed at evaluation**
  (`NodeErrorKind`, an arm naming the carrier kind found), so a
  headless author gets the same answer the chrome pre-empts under
  DM2.
- The chrome consequences are CHROME's builds
  (`add-profile-mints-no-frame`,
  `add-profile-placement-on-picked-face-frame`): "on a new XY frame"
  is two inserts in one committed action (`commit_action`); "on this
  face" mints one `FaceFrame` and one profile the same way.

*Record: built by DOCM-1 (PR 1829), with DM1a, DM1b and DM2.*

## DM2 — A carrier-kind read is a value, not a verdict

`readback.rs` rule 1 says "values, never verdicts" and lists "is this
face planar" beside "is this at z ≈ 1". The two are not the same
kind of question: the second is a numeric predicate under the margins
discipline, the first is a comparison of a stored tag, and tag
comparisons are allowed without restriction — they are where the
intent is stored. `select_where` filters on `SurfaceKind` exactly
(`names/geompred.rs`). So:

- `topo::readback` carries a door that returns a face's carrier kind
  (the `SurfaceKind` tag, copied out) and `names::interrogate` its
  `StableName` twin (`face_carrier_kind` in both); rule 1's text says
  NUMERIC predicates, in `readback.rs` and its mirror at
  `interrogate.rs`.
- The chrome offers DM1's frame only for a planar carrier; DM1b is
  the kernel's own refusal when a caller bypasses the offer.

*Record: built by DOCM-1 (PR 1829).*

## DM3 — A part of a multi-body value is selected by a projection node

`Node::Part { of: RecipeNodeId, select: PartSelect }`, with
`PartSelect::{ SplitHalf(SplitHalf), Instance(Expr) }`, evaluates to
ONE body: the named half of a `Split` value or the `i`-th body of an
`Instances` value. `Instance`'s index is a structural slot (a count,
like `Pattern::count`) and an index at or beyond the pattern's count
refuses typed at evaluation. Names pass through unchanged, as
`Transform`'s do (`role.rs`): the body keeps the split's
`SplitBody(half)` name or the pattern's `Instance { i, of }` names,
so every downstream selector spells what it already spells.

- **Why a node and not an operand struct.** The operand struct would
  put a projection inside every body-consuming payload (`Boolean`,
  `Split`, `Transform`, `Fillet`, `Chamfer`, `Pattern`, …) and fork
  each consumer's admission on it. The node is the `PlacedUnion`
  ruling's shape: one meaning, one node. Every consumer stays as it
  is, `eval::wire::body_operand` is unchanged, and the viewer's
  `denotes_body` (`combine.rs`) carries one more `true` arm, which
  `the_body_seat_tracks_the_evaluators_operand_door` pins. The
  selection is a visible, editable tree row.
- The cost is that row. `several_bodies_are_not_one_body_at_a_seat`
  keeps asserting that a bare split or pattern is refused at a body
  seat; the Part node is how a user says which body they meant.

*Record: built by DOCM-2 (PR 1860).*

## DM4 — Flat operators before splice: an n-ary union

The case is the die's. `demos/tour/src/diefillet.rs` assembles 21
transforms of one ball into one cutting tool, subtracts it, and
fillets the twelve box edges and the 21 pip rims. The other way to
assemble a multi-shell tool is a chain of pairwise `Boolean(Union)`s,
and that chain is an artifact of the vocabulary rather than of the
model: boolean naming wraps every operand's names in `FromA` / `FromB`
(`role.rs`, the boolean group), so a pip's rim name records the depth
at which it joined. Removing one link from such a chain changes the
names of every pip that joined before it, and the rim fillet's frozen
selection fails typed for each of them (one rim for the second pip,
twenty for the last), repairable only by a `Rebind` per name through
N5's offers. A splice edit that assumed intent about which input
survives would carry that cost on top of its own.

So the chain goes, not the link:

- **`Node::Union { members: Vec<RecipeNodeId>, declare }`** — an
  n-ary union, two or more members, ONE body out. It evaluates as a
  fold of the kernel's pair verb in member order (D9: the order is the
  list's, and the list is data). It sits beside `Boolean(Union)`,
  which stays for a pair, and beside `PlacedUnion`, which fuses
  instances of one prototype and is a different sentence (`node.rs`).
- **Naming keys by member, not by depth.** The emitter wraps a
  member's names in `FromMember { member: RecipeNodeId, of:
  Box<StableName> }`: `member` is the member's own node id (the edge
  in the list), `of` the entity's name in that member's table. The
  key is the edge, never the inner name's minting node, because a
  pass-through op contributes no segment (N1; `Transform` keeps the
  input's rows verbatim), so two members that are transforms of one
  body carry IDENTICAL tables — the die's 21 pips are exactly that —
  and the inner name alone cannot tell them apart. The member id
  can, it is data the node already carries, and DM5 makes it unique
  within one union. No position is recorded, so removing a member
  leaves every other member's names as they were. The `Instance { i,
  of }` segment is the precedent shape, with an identity where it has
  an index.
- **`DocEdit::SetMembers { node, members: Vec<RecipeNodeId> }`** —
  the one edit that changes a list input, by naming the whole new
  list. Unambiguous by construction: nothing is inferred. Refuses
  typed on an unknown or non-live member, a cycle (`WouldCycle`
  through the existing check), a duplicate (DM5), or fewer than two
  members. Deleting a pip is `SetMembers` without it plus a plain
  `DeleteNode` of the orphaned transform, one committed action
  (`commit_action`), and the other twenty rims survive. `Loft`'s
  `profiles` list is the same shape and takes the same edit; nothing
  else in the vocabulary is a list.
- The viewer's combining doors take a union seat of N body picks —
  CHROME's build, not in the tree today.
- **A declaration channel, sited at the members.** Two members that
  touch refuse `UndeclaredContact` exactly as a pair boolean's operands
  do, and the union carries the same recourse: `Node::Union { members,
  declare: Option<RecipeNodeId> }`, the `Declare` node's pairs naming
  SITED entities — `SitedRef { at, name }`, the entity `name` as it
  stands at node `at`, where `at` is the member (for a pair boolean,
  the operand) — so a declaration says "this face of member `m` meets
  that face of member `n`" by naming the face IN the member with the
  member beside it, and never names the union. A declaration therefore
  names only what exists before the union does, and is authored in one
  pass: the `Declare` is inserted before the union that carries it.
  Each pair is fed to the fold step at which both its sites are in the
  accumulation: the later member's step in list order, the earlier
  side as the accumulator's operand, the later as the joining member's;
  a pair whose two sites are ONE member is that member's carried
  contact at its own step; no fold position is recorded anywhere. A
  name not in its site's table refuses typed through the pair
  boolean's own resolver, which the union reuses (one definition of
  "resolve a declared name at its site"); the site IS the side, so a
  name carried by both operands is no longer ambiguous. What the
  union PUBLISHES is unchanged — `FromMember { member, of }` is what
  it mints; a sited pair is what it consumes. `SetMembers` leaves
  `declare` as it was; a pair whose site left the list refuses at the
  next evaluation as a vanished name does (N5), never silently. The
  "disjoint-only" reading is not taken: the common modelling case (a
  boss on a plate) would keep the pairwise chain alive.
  **Merges and order.** A merged face's name is a FLAT constituent
  set (N3) — whatever mints a `Merged` mints it flat, and a nested
  `Merged` is an emission bug, never something a consumer flattens —
  so the accumulation's rows carry no fold tree; and a member-space
  name resolves at its step through whatever merges the fold has
  performed (the union rewrites it to the flat `Merged` row containing
  it before the shared resolver), so a declaration set whose faces are
  consumed by MERGES fuses in every member order. A merge is the one
  consumption with a unique successor, so it is the one looked
  through. A face consumed by a split, by containment, or by a merge a
  later step fragmented has none, and a pair naming it at a later step
  refuses — `Vanished`, diagnosed `ConsumedByFold` with the
  composition read off the rows that descend from the face (fragments
  of it; fragments of a merged row covering it; none at all while its
  member still derives it), never by measuring it again — and offers
  nothing: which fragment the pair meant is the geometric question the
  routing step does not ask, and a containment leaves no candidate.
  **The refusal against a fold-minted row.** "The same recourse"
  above holds for a member's own face verbatim, and for a face the
  fold MERGED through a constituent: the refusal sites that side at
  one constituent (the first in member order — any constituent
  declares the same contact through the look-through, so the pick is
  immaterial and the finding carries the flat set beside it), and the
  caller declares what the refusal names. A row the fold minted that
  no member's entity stands for (a `Fragment` — a `Seam` mints edges
  and vertices, never a face) has no site to name and so no declare
  arm: it refuses typed, `UndeclarableContact`, rather than degrading
  into an emission bug that blames the crate for a document the user
  wrote. The refusal menu of `docs/SELECT-DESIGN.md` §3d keeps its
  two arms for every sited row; this one row has only the second.

*Record: the node, its naming and `SetMembers` are DOCM-3 (PR 1803);
the member-space declaration channel is DOCM-7 (PR 2028), re-sited at
the members by Ev on EDIT's fourth `[ev]` PR (#2795, 2026-09-17;
`a-declared-union-has-no-one-pass-authoring-path`), built by the unit
that row names; the flat
`Merged` mint and the look-through are DOCM-8 (PR 2073); the typed
refusal past the merges applies Ev's ruling on PR 2677
(`does-n3-retire-loudly-generalise-to-the-folds-other-compositions`).*

## DM5 — A node's inputs are pairwise distinct

`Boolean { a: X, b: X }`, a union or loft list with a repeated member,
and a split whose target and tool coincide are all refused. The rule
is stated once, as a structural validity check on a node's inputs —
pairwise distinct — and called by `InsertNode`, by `SetMembers` on the
rewritten node, and by the load validator (`persist/check.rs`,
`validate_document`) on every node of a snapshot, so the three doors
share the logic rather than mirror it. Replayed edits meet it through
`InsertNode`; the snapshot beside the edit log is the third door, since
a hand-written snapshot never passes an edit door. Refusal:
`EditError::DuplicateInput { node, input }` at the edit doors, the
validator's own `SnapshotError` arm at load.

*Record: built by DOCM-3 (PR 1803) with DM4.*

## DM6 — Splice is not added

No edit rewires a live node's inputs, and none is planned. Every
graph change is `InsertNode`, `DeleteNode`, or `SetMembers` on a
list. `no-docedit-splices-a-deleted-node` stays open as the record of
the one trigger that would reopen the question: a chain that a flat
operator cannot flatten and that a user needs to edit from the middle
— possibly never. Cascade delete (`cascade_delete_order`, `edit.rs`)
stays the delete for a node with consumers.

*Record: ruled with DM4's build, DOCM-3 (PR 1803), which is what makes
the die's chain unnecessary.*

## DM7 — A stranded name is reported at the delete, never refused

`DeleteNode` stays legal when a payload name (`Node::payload_names`)
names the node being deleted: a name is not a DAG edge, and the
carve-out in §0 stands. What the door owes is a report: every
`(node, name)` pair whose minting node the edit removed rides the
accepted edit's `Applied.maintenance`, typed, computed at the door by
the same payload walk the insert door checks with. The strand is loud
where it happens rather than at the next evaluation; `NodeGone` and
`Rebind` remain the diagnosis and the repair.

The report covers every reference the document holds under N5
semantics, not only the node payloads: an appearance attachment is
keyed by a `StableName` in the document's appearance store
(`DocEdit::SetAppearance` gives it Declare's semantics, `Rebind`
repairs it, evaluation reports its loss as `AppearanceLoss`), so a
delete that strands one reports it too, as its own `Maintenance` arm
(`StrandedAppearance { name }`) rather than a `Strand` with no
carrying node. `Node::payload_names` stays the one list of NODE
carriers; the store is the other carrier.

- **Why not an edge.** A full edge over payload names reverses the
  carve-out (D3: a name is a reference, not an edge). The case that
  decided it was the declared union as DOCM-7 first shaped it — the
  union's input was the `Declare` and the `Declare`'s pairs named the
  union's own space, a cycle `cascade_delete_order` could not see;
  sited declarations (DM4, as re-ruled on EDIT's fourth `[ev]` PR)
  remove that cycle, and the carve-out stands on D3's own ground.
- **Why not as-is.** A legal edit whose consequence is invisible until
  evaluation is what the maintenance column exists to end.
- The chrome's cascade affordance shows the strand count beside its
  dependent count (CHROME's build); maintenance is derived, so nothing
  persisted changes.

*Record: ruled by Ev on the `[ev]` PR of 2026-09-16
(`deletenode-strands-a-declare-payload-name`); built by the unit that
row names. The appearance-key widening was ruled by Ev on EDIT's third
`[ev]` PR of 2026-09-16
(`stranded-appearance-keys-are-not-reported-by-dm7`), which builds
it.*

## DM8 — The authored-step to canonical-segment map is composed in `editor-core`

The map from an authored profile step (`SlotId::Profile { loop_, step,
arg }`) to the profile edges it became (`ProfileEdgeRef {
loop_index, segment }`) is a function in `program.rs` reading the
records the evaluation already produces: the replay's per-step segment
span and its per-radius emission (fields of `crates/profile`'s
`ReplayStructure`, beside its fillet decisions) give the answer — the
span for a step, the emission for a radius, since the step a radius is
authored on is not always the step its arc is credited to — in the
program's own step order —
the numbering the published names carry, since `eval/anchor.rs`
renumbers every emitted ref canonical → program before the name table
is published — and canonicalization's `reversed` and `start` on
`LoopCanonical` are checked against the naming anchor's record of the
same permutation, never applied. A disagreement between those two
records is the evaluation contradicting itself and asserts. The door
refuses typed where a record is absent or of the wrong shape rather
than guessing. It is derived from the structure record the geometry
came from, so it cannot disagree with the geometry, and it is not
persisted. For a loft the published anchoring is section 0's
(`work/wire/loft-anchors-every-section-with-section-zeros-map`).

- **Why not the viewer.** A second derivation from both endpoints can
  disagree with the first.
- **Why not `crates/profile` alone.** It has no vocabulary for an
  authored step and would grow one for a consumer two layers up.
- Consumers: the viewer's per-segment focus marking (VIEW's row), and
  the per-edge radius door in `program.rs` (`segment_radii`), which
  pairs each authored radius with the edge its own arc drew and is
  what a sweep's per-edge parameter-identity attach reads.

*Record: ruled by Ev on the `[ev]` PR of 2026-09-16
(`authored-step-to-canonical-segment-map-has-no-home`); the
`ReplayStructure` field is S-BOOL's ground by announcement. The
wording — the answer in the program's numbering, the permutation
checked rather than applied, a disagreement asserting — was ruled by
Ev on EDIT's third `[ev]` PR of 2026-09-16
(`dm8-names-canonical-segments-but-the-published-refs-are-program-anchored`),
after the unit that built the door measured the original clause's
composition wrong.*

## What this doc does not touch

Identity across time (`crates/editor-core/IDENTITY.md`), the
instantiation seam, the check registry, the certified range query.
Viewer chrome for every ruling here is CHROME's or VIEW's.
