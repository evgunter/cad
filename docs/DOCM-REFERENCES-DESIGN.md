# DOCM-REFERENCES-DESIGN: what a recipe reference may be

Status: **RATIFIED in-chat (Ev, 2026-09-04): DM1–DM6.** The PR that
carries this doc is the record; it asks nothing further. Companion-table
row at `docs/DESIGN.md`. This doc answers three of DOCM's questions
(`work/docm/plan.md`: frames, operand selectors, deleting from a
chain) as one conversation, because each asks for a new combination
of the reference shapes the recipe already admits (§0). Mechanics
are measured (file:line at the time of writing; cited by name where a
number would drift), not assumed.

## 0. Grounding (committed; this doc does not re-litigate)

The recipe admits three reference shapes, and every node is built
from them:

- **A DAG edge**: a `RecipeNodeId` in a node's inputs, structural,
  liveness- and cycle-checked at the edit door (`edit.rs`,
  `InsertNode`: `UnresolvedInput`, `WouldCycle`), enumerated by
  `Node::inputs` (`node.rs:1640`). Ids are minted by the document's
  monotone counter and never reused (D3; `doc.rs:313`).
- **A frozen `StableName`**: `{ kind, node, path }` (N1,
  `names/role.rs:86`) stored at authoring and resolved at evaluation
  through a name table under the N5 ladder — `NodeGone`, then
  `Ambiguous`, then `Vanished` — never silently shrunk
  (`eval/wire.rs`, `ladder` and `resolve_selection`). Carriers today:
  `Fillet`/`Chamfer` selections, `Declare` pairs, `Mate` heads,
  `Measure` refs (`Node::payload_names`, `node.rs:1985`). A name is
  not a DAG edge: the only door that checks it is `InsertNode`'s
  liveness check, and a later delete may strand it (N5).
- **An `Expr` literal** in a slot, bit-pinned (D7).

Two precedents this doc extends. `MeasureRef { at, name }`
(`node.rs:871`) pairs a DAG edge with a frozen name: `at` says which
evaluated value to read, `name` says which entity. `Datum::AxisInPlane
{ plane, .. }` (`node.rs:611`) is the one datum with a DAG input: its
meaning comes from another node, and the design note there says why —
the check is not made cheaper, the error is made unrepresentable.

Ruled elsewhere and kept: a selection FREEZES (#217, `node.rs:1266`);
selectors MATERIALIZE and are never stored (`names/select.rs:15`);
`PlacedUnion` is a node beside `Pattern`, not a flag on it, because
forking a result type on a variant is the silent-dispatch trap D3
forbids (`node.rs:1325`); `Datum::Frame` carries nine `Expr`s and no
reference, orthonormalized at evaluation (`node.rs:576`).

## DM1 — A derived frame is a datum carrying a face name

`Datum::FaceFrame { at: RecipeNodeId, face: StableName, spin: Expr }`
is a `Datum::Frame` whose pose is computed at evaluation from a
named face: `at` is a DAG edge to the body node whose value the face
is read out of, `face` is a frozen face name resolved through that
value's name table under the N5 ladder, and `spin` is the authored
rotation of sketch +x about the normal. It is the `MeasureRef` shape
applied to a datum. The frame it yields: origin at the carrier's
origin projected to the face's plane (the carrier's own distinguished
point, `readback.rs` rule 2), normal the face's OUTWARD normal (DM1a),
sketch +x the carrier's u-reference rotated by `spin`.

- **Why derived, not frozen.** The profile-plane migration deleted
  the twelve-float snapshot a sketch used to carry
  (`program.rs:250`); a frame read off a face and written into nine
  literals reintroduces that snapshot one node out, and lies about
  why it sits where it sits. A derived frame is a DAG input: the
  face's body is upstream, the frame moves when the face moves, and
  it participates in the memo and content key like every node.
- **The failure mode is the fillet's.** A face name that stops
  resolving fails the frame typed and poisons the sketch above it,
  exactly as a fillet's selection does (`BlendSelectionResolve`); the
  repair is `Rebind`. It is the first datum with an N5 failure mode,
  and that is the honest behaviour.
- **DM1a — the read-back grows a sense, it does not fold one in.**
  `Pose.axis` is the CHART's direction, deliberately uncorrected by
  the face's orientation sense (`readback.rs:69`), and stays so: two
  facts, two answers. What is missing is that the sense is not
  readable at all — `Face` carries it (`entity.rs`, `sense_sign`),
  no door returns it. `face_pose` and `names::interrogate::face_frame`
  return the sense beside the pose, one more stored value copied out,
  and DM1's datum states in its own vocabulary that its normal is
  sense times chart axis. The mate tool's frozen frames are
  unaffected: A11 keeps the solve over authored numbers, and that
  asymmetry is principled — a sketch frame is consumed by evaluation,
  which reads geometry constantly; a mate frame by a solve that must
  not.
- **DM1b — a non-planar carrier refuses typed at evaluation**
  (`NodeErrorKind`, a new arm naming the carrier kind found), so a
  headless author gets the same answer the chrome pre-empts under
  DM2.
- The chrome consequences are CHROME's builds
  (`add-profile-mints-no-frame`,
  `add-profile-placement-on-picked-face-frame`): "on a new XY frame"
  is two inserts in one committed action (`commit_action` exists);
  "on this face" mints one `FaceFrame` and one profile the same way.

## DM2 — A carrier-kind read is a value, not a verdict

`readback.rs` rule 1 says "values, never verdicts" and lists "is this
face planar" beside "is this at z ≈ 1". The two are not the same
kind of question: the second is a numeric predicate under the margins
discipline, the first is a comparison of a stored tag, and tag
comparisons are allowed without restriction — they are where the
intent is stored (Ev, 2026-09-04). `select_where` already filters on
`SurfaceKind` exactly (`names/geompred.rs:124`). So:

- `topo::readback` gains a door that returns a face's carrier kind
  (the `SurfaceKind` tag, copied out) and `names::interrogate` its
  `StableName` twin; rule 1's text is tightened to say NUMERIC
  predicates, in `readback.rs` and its mirror at `interrogate.rs:22`.
- The chrome offers DM1's frame only for a planar carrier; DM1b is
  the kernel's own refusal when a caller bypasses the offer.

## DM3 — A part of a multi-body value is selected by a projection node

`Node::Part { of: RecipeNodeId, select: PartSelect }`, with
`PartSelect::{ SplitHalf(SplitHalf), Instance(Expr) }`, evaluates to
ONE body: the named half of a `Split` value or the `i`-th body of an
`Instances` value. `Instance`'s index is a structural slot (a count,
like `Pattern::count`) and an index at or beyond the pattern's count
refuses typed at evaluation. Names pass through unchanged, as
`Transform`'s do (`role.rs:89`): the body keeps the split's
`SplitBody(half)` name or the pattern's `Instance { i, of }` names,
so every downstream selector spells what it already spells.

- **Why a node and not an operand struct.** The operand struct puts
  a projection inside every body-consuming payload (`Boolean`,
  `Split`, `Transform`, `Fillet`, `Chamfer`, `Pattern`, …) and forks
  each consumer's admission on it. The node is the `PlacedUnion`
  ruling's shape: one meaning, one node. Every consumer stays as it
  is, `eval::wire::body_operand` (`wire.rs:457`) is unchanged, and
  the viewer's `denotes_body` (`combine.rs:464`) gains one `true`
  arm, which `the_body_seat_tracks_the_evaluators_operand_door`
  then re-pins. The selection is a visible, editable tree row.
- The cost is that row. `several_bodies_are_not_one_body_at_a_seat`
  keeps asserting that a bare split or pattern is refused at a body
  seat; the Part node is how a user says which body they meant.

## DM4 — Flat operators before splice: an n-ary union

The motivating case for deleting a node from the middle of a chain
(`work/docm/no-docedit-splices-a-deleted-node`) is
`demos/tour/src/diefillet.rs`: 21 transforms of one ball chained by
20 pairwise unions into one cutting tool, one subtract, a fillet on
the twelve box edges, a fillet on the 21 pip rims. The chain is an
artifact — `diefillet.rs:288` records that a pairwise
`Boolean(Union)` is the recipe's only way to assemble a multi-shell
tool — and the chain is also what makes any restructuring expensive:
boolean naming wraps every operand's names in `FromA` / `FromB`
(`role.rs`, the boolean group), so a pip's rim name records the depth
at which it joined. Removing one link from the chain changes the
names of every pip that joined before it, and the rim fillet's frozen
selection fails typed for each of them (one rim for the second pip,
twenty for the last), repairable only by a `Rebind` per name through
N5's offers. A splice edit that assumed intent about which input
survives would carry that cost on top of its own.

So the chain goes, not the link:

- **`Node::Union { members: Vec<RecipeNodeId> }`** — an n-ary union,
  two or more members, ONE body out. It evaluates as a fold of the
  kernel's pair verb in member order (D9: the order is the list's,
  and the list is data). It sits beside `Boolean(Union)`, which stays
  for a pair, and beside `PlacedUnion`, which fuses instances of one
  prototype and is a different sentence (`node.rs:1325`).
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
  `profiles` list is the same shape and takes the same edit in the
  unit that adds it, or a later one; nothing else in the vocabulary
  is a list.
- The viewer's combining doors gain a union seat that takes N body
  picks; that build is CHROME's.

## DM5 — A node's inputs are pairwise distinct

No door refuses `Boolean { a: X, b: X }` today: `InsertNode` checks
liveness, slots and acyclicity, and `wire_boolean` runs the pair verb
on whatever it is handed (`wire.rs:1994`). `SetMembers` needs the
rule, so it is stated once, as a structural validity check on a
node's inputs — pairwise distinct, which covers the boolean, a union
or loft list with a repeated member, and a split whose target and
tool coincide — and called by `InsertNode`, by `SetMembers` on the
rewritten node, and by the load validator (`persist/check.rs`,
`validate_document`) on every node of a snapshot, so the three doors
share the logic rather than mirror it. Replayed edits meet it through
`InsertNode`; the snapshot beside the edit log is the third door, since
a hand-written snapshot never passes an edit door. Refusal:
`EditError::DuplicateInput { node, input }` at the edit doors, the
validator's own `SnapshotError` arm at load.

## DM6 — Splice is not added

No edit rewires a live node's inputs, and none is planned. Every
graph change is still `InsertNode`, `DeleteNode`, or `SetMembers` on
a list. `no-docedit-splices-a-deleted-node` is PARKED on DM4's unit:
the trigger to reopen it is a chain that a flat operator cannot
flatten and that a user needs to edit from the middle — possibly
never (Ev, 2026-09-04). Cascade delete (`cascade_delete_order`,
`edit.rs:1253`) stays the delete for a node with consumers.

## What this doc does not touch

Identity across time (`docs/DOCM-IDENTITY-DESIGN.md`), the
instantiation seam, the check registry, the certified range query.
Viewer chrome for every ruling here is CHROME's or VIEW's.
