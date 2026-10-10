# What a recipe reference may be

This page decides the editor-core recipe's reference vocabulary: what a node
may point at, and what each kind of pointer means. Its clauses — derived
frames, part selection, flat operators, deleting from the middle of a chain,
stranded names, the step-to-segment map — each ask for a new combination of
the reference shapes the recipe already admits (§0). Identity across time is
the companion page `crates/editor-core/IDENTITY.md`. File citations name the
symbol beside them; the symbol is the stable half.

## 0. Grounding (committed elsewhere)

The recipe admits three reference shapes, and every node is built from them:

- **A DAG edge**: an operand's read of an output variable (D10), typed
  by the kind its slot admits, liveness- and cycle-checked at the edit
  door (`edit.rs`, `lower_operand`: `OperandUnresolved`,
  `SlotVarKind`, `WouldCycle`), enumerated by `Doc::upstream`. Ids are
  minted from the document's mint chain and never reused (D3, N1; `mint.rs`).
- **A frozen `StableName`**: `{ kind, node, path }` (N1, `names/role.rs`),
  stored at authoring and resolved at evaluation through a name table under
  the N5 ladder — `NodeGone`, then `Ambiguous`, then `Vanished` — never
  silently shrunk (`eval/wire.rs`, `ladder` and `resolve_selection`).
  Carriers: `Fillet`/`Chamfer` selections, `Shell` open lists, a
  `Datum::FaceFrame`'s face, a `Boolean`'s or `Union`'s declared pairs,
  `Mate` heads, `Measure` refs, an `InstantiatePart`'s interface crossings'
  `outer`s (a crossing's `inner` is not a name of this document; the list's
  arm says why). The list has two homes, this clause and
  `Node::payload_names`' own doc; every other site points at the latter
  rather than restating it. A name is not a DAG edge, and two doors refuse
  on one: `InsertNode`'s liveness check, and `split`'s
  `PartNameReachesRemainder` precondition, which refuses a cut whose taken
  node carries a name reaching the kept remainder (a declared pair's, a
  `Mate` head's, an instance's crossing `outer`). A later delete strands a
  name (N5) and says so (DM7).
- **An `Expr` literal** in a slot, bit-pinned (D7).

Two precedents these clauses extend. `SitedRef { at, name }` (`node.rs`)
pairs a DAG edge with a frozen name: `at` says which evaluated value to read,
`name` says which entity. `Datum::AxisInPlane { frame, .. }` (`node.rs`) is
the one datum with a DAG input: its meaning comes from another node, which
does not make the check cheaper but makes the error unrepresentable.

Also kept from elsewhere: a selection freezes (#217, `node.rs`); selectors
materialize and are never stored (`names/select.rs`); `PlacedUnion` is a node
beside `Pattern`, not a flag on it, because forking a result type on a variant
is the silent-dispatch trap D3 forbids (`node.rs`); `Datum::Frame` carries
nine `Expr`s and no reference, orthonormalized at evaluation (`node.rs`).

## DM1 — A derived frame is a datum carrying a face name

`Datum::FaceFrame { at: RecipeNodeId, face: StableName, spin: Expr }` is a
`Datum::Frame` whose pose is computed at evaluation from a named face. `at`
is a DAG edge to the body node whose value the face is read out of; `face` is
a frozen face name resolved through that value's name table under the N5
ladder; `spin` is the authored rotation of sketch +x about the normal. It is
the `SitedRef` shape applied to a datum. The frame it yields has its origin
at the carrier's origin projected to the face's plane (the carrier's own
distinguished point, `readback.rs` rule 2), its normal along the face's
outward normal (DM1a), and sketch +x along the carrier's u-reference rotated
by `spin`.

- **Why derived, not frozen.** A sketch carries no twelve-float plane snapshot
  (`program.rs`); a frame read off a face and written into nine literals
  would reintroduce that snapshot one node out, and lie about why it sits
  where it sits. A derived frame is a DAG input: the face's body is upstream,
  the frame moves when the face moves, and it participates in the memo and
  content key like every node.
- **The failure mode is the fillet's.** A face name that stops resolving
  fails the frame typed and poisons the sketch above it, exactly as a
  fillet's selection does (`BlendSelectionResolve`); the repair is `Rebind`.
  It is the first datum with an N5 failure mode, and that is the honest
  behaviour.
- **DM1a — the read-back grows a sense, it does not fold one in.**
  `Pose.axis` is the chart's direction, deliberately uncorrected by the face's
  orientation sense (`readback.rs`): two facts, two answers. The sense
  travels beside the pose as `Pose::sense`, one more stored value copied out
  of the face record (`entity.rs`, `Face::sense`), returned by `face_pose`
  and `names::interrogate::face_frame`; DM1's datum states in its own
  vocabulary that its normal is sense times chart axis. The mate tool's
  frozen frames are unaffected: A11 keeps the solve over authored numbers.
  The asymmetry is principled — a sketch frame is consumed by evaluation,
  which reads geometry constantly; a mate frame by a solve that must not.
- **DM1b — a non-planar carrier refuses typed at evaluation** (a
  `NodeErrorKind` arm naming the carrier kind found), so a headless author
  gets the same answer the chrome pre-empts under DM2.
- **DM1c — the lanes.** A derived frame has no document elaboration, so the
  profile lift's "the sketch plane stays f64" fence (PP6,
  `crates/editor-core/README.md`, which carries the same rule) does not apply
  to it. A profile on a derived frame is placed at the lane scalar through
  the by-value reader (`frame_plane_lane`) under every lift, its 2-D
  structure record staying f64-pinned as PP1 says; a loft or sweep section on
  a derived frame refuses typed at any scalar but f64, since a section's
  geometry stays f64. An authored frame's profile is unchanged. The two
  placement paths are keyed by frame kind because the two kinds differ in
  where their numbers come from.
- **In the chrome**, "on a new XY frame" is two inserts in one committed
  action (`commit_action`); "on this face" mints one `FaceFrame` and one
  profile the same way.

*Built: DOCM-1 (PR 1829), with DM1a, DM1b and DM2.*

## DM2 — A carrier-kind read is a value, not a verdict

`readback.rs` rule 1 says "values, never verdicts". "Is this face planar" and
"is this at z ≈ 1" are different kinds of question: the second is a numeric
predicate under the margins discipline; the first compares a stored tag, and
tag comparisons are allowed without restriction — they are where the intent
is stored. `select_where` filters on `SurfaceKind` exactly
(`names/geompred.rs`). So:

- `topo::readback` carries a door that returns a face's carrier kind (the
  `SurfaceKind` tag, copied out) and `names::interrogate` its `StableName`
  twin (`face_carrier_kind` in both); rule 1 restricts *numeric* predicates,
  in `readback.rs` and its mirror in `interrogate.rs`.
- The chrome offers DM1's frame only for a planar carrier; DM1b is the
  kernel's own refusal when a caller bypasses the offer.

*Built: DOCM-1 (PR 1829).*

## DM3 — One member of a family is read by index; a split's halves are ports

A `Split` defines two `Body` outputs, `above` and `below` (the port index is
`SplitHalf::output_body`'s), read like any other output; DM3's split-half
projection retires. One member of a family (D10, Repetition) is read as
`xs[i]`, or `xs[i, j]` for a family keyed by two indices: a definition
reading the family and one `Count` expression per index; an index outside
the range leaves the reader unresolved and typed at evaluation, never
re-pointed. Names pass through unchanged, as `Transform`'s do (`role.rs`):
the member keeps its `Member { (i, …), of }` names, keyed by the index
variables' ids and the integers, and a split half keeps its
`SplitBody(half)` names, so every downstream selector spells what it
already spells. A body seat reading a whole family, a profile or a split
as a whole refuses by kind at the door (`SlotVarKind`).

## DM4 — Flat operators before splice: an n-ary union

`Node::Union { members: Vec<RecipeNodeId>, declare }` fuses its members
into one body, and names each entity by the member it came from. A member
is a body or a family (D10, Repetition), whose members join in index order
at that member's place in the list; a family's length is a `Count`, so the
floor of two bodies in all is evaluation's, refused typed there.

**Why.** A multi-shell tool assembled as a chain of pairwise
`Boolean(Union)`s is an artifact of the vocabulary rather than of the model.
Boolean naming wraps every operand's names in `FromA` / `FromB` (`role.rs`,
the boolean group), so an entity's name records the depth at which it joined.
In the die (`demos/tour/src/diefillet.rs`: 21 transforms of one ball fused
into one cutting tool and subtracted, then the twelve box edges and the 21
pip rims filleted), removing one link from such a chain would change the
names of every pip that joined before it, and the rim fillet's frozen
selection would fail typed for each of them (one rim for the second pip,
twenty for the last), repairable only by a `Rebind` per name through N5's
offers. A splice edit that assumed intent about which input survives would
carry that cost on top of its own. So the chain goes, not the link:

- **The node.** An n-ary union, two or more bodies, one body out. It
  evaluates as a fold of the kernel's pair verb in member order: the order
  is the list's, the author's statement, as a pair boolean's operand order
  is. What the union decides is defined over its members, not over the
  fold: every coincidence between two members is judged once, pairwise,
  before the fold (below); the fold reads those verdicts through each
  face's member parents and decides no carrier pair again; and a name is
  defined over the finished body (N2). What follows the list says so: a
  glue keeps the earlier member's description, as a pair boolean keeps
  operand A's, and a refusal raised at a fold step names the member whose
  step refused. It sits beside `Boolean(Union)`, which
  stays for a pair. A union reads shapes already in one space and places
  nothing: a placement defines copies, and the union reads them, so the
  union of a pattern's copies is this node over the pattern's members.
  `PlacedUnion` (`node.rs`) both places copies of one prototype and
  fuses them, so it retires into those two: a placement of the copies,
  and this union reading them. When its bodies are rigid images of one
  body, visible in the reads, the union has a fast path: bodies it
  certifies disjoint are grafted without a boolean (the group boolean,
  `README.md`).
- **Naming keys by member, not by depth.** The emitter wraps a member's names
  in `FromMember { member: RecipeNodeId, of: Box<StableName> }`: `member` is
  the member's own node id (the edge in the list), `of` the entity's name in
  that member's table. The key is the edge, never the inner name's minting
  node: a copy keeps the rows of the body it copies (N1), so two members
  that are copies of one body carry identical tables, and the inner name
  alone cannot tell them apart. The member id can; it is data the node
  already carries, and DM5 makes it unique within one union. No position is
  recorded, so removing a member leaves every other member's names as they
  were. A family's members keep their `Member { (i, …), of }` segments
  (DM3), keyed by the index variables' ids, so they too are told apart by
  identity, never by position in the list.
- **`DocEdit::SetMembers { node, members: Vec<Operand> }`** is the one
  edit that changes a list input, by naming the whole new list; nothing is
  inferred. It refuses typed an unknown or non-live member, a cycle
  (`WouldCycle` through the existing check), a duplicate (DM5), or an empty
  list; fewer than two bodies in all refuses at evaluation (above).
  Deleting a pip is `SetMembers` without it, and the other
  twenty rims survive. The pip the union no longer reads is still
  defined, and out of the product because no placement names it;
  deleting it too is tidiness, not a requirement. `Loft`'s `profiles`
  list is the same shape and takes the same edit; nothing else in the
  vocabulary is a list.
- The viewer's combining doors take a union seat of N body picks (not yet
  built).

### Contact is judged pairwise, in member space, before the fold

Every coincidence between two members is a coincidence of the union,
whatever the other members are, so the union's coincidence rows are its
member pairs' rows: a contact a third member covers is still its pair's,
recorded, and a pair the sliver band leaves undecided refuses the union even
where a third member covers it. A coincidence among three members that no
pair holds (a seam of two members lying in a third's face) is the union's
row too, spelled by its member cells, so it is one row in every order. The
faces each judgement merged or covered stay the union's face links (N2).

- Before the fold, each pair of members `m`, `n` is judged by the pair verb,
  as the two-member union `m ∪ n` with the declared pairs whose sites are `m`
  and `n`. A pair carrying a declaration is judged whatever its boxes: the
  declaration is a claim to verify, so its names resolve at their sites and a
  contradicted one refuses `ContactContradicted` in every member order. Only
  an undeclared pair whose closed bounding boxes are disjoint is skipped,
  because it cannot touch. The judgement costs up to n(n−1)/2 two-member
  unions, bounded by that pruning; if it becomes a measured performance
  problem, it is raised to Ev, not optimized around the rule.
- A pair that touches with the contact undeclared refuses `UndeclaredContact`
  exactly as the two-member union's operands do, in every member order.
- A declared pair that the geometry contradicts refuses as the pair boolean
  does.
- Any other refusal the pair verb raises on a judged pair is the union's
  refusal too, and so is order-free: a pair whose contact the pair verb
  cannot decide inside the tolerance grey band refuses `Escalated` in every
  member order, even where a third member covers it.
- A declared pair that survives the judgement is certified.

A certified pair authorizes its contact wherever the fold meets it; it is not
a demand that the fold meet it. A contact another member covers, whose region
lies in a third member's interior, is still a contact of its pair and needs
the declaration; where the fold has consumed the pair's faces before the
pair's step, the declaration is satisfied, not refused. Every contact the
fold meets is between two members, because the accumulation's boundary is
made of member faces, so every contact the fold meets was judged here and the
fold mints no contact verdict of its own.

### The declaration channel, sited at the members

The union carries the pair boolean's recourse. `Node::Union { members,
declare: Vec<DeclaredPair> }` and `Node::Boolean { op, a, b, declare:
Vec<DeclaredPair> }` hold the declared pairs as the node's own payload (empty
is undeclared). Each pair names sited entities — `SitedRef { at, name }`, the
entity `name` as it stands at node `at`, where `at` is the member (for a pair
boolean, the operand) — so a declaration says "this face of member `m` meets
that face of member `n`" by naming the face in the member with the member
beside it, and never names the union.

- **Writing pairs.** Every door that writes a pair (the insert door,
  `SetDeclare`, `Rebind`, the load door) refuses, typed, a name not minted
  before the node in document order and a site that is not one of the node's
  operands, so a declaration names only what exists before the node does. The
  load door asks the site of a pair boolean only, since a union's site can be
  stranded afterwards (below). A declaration is a parameter, not an operand:
  it carries no material and mints no names, so `SetDeclare { node, pairs }`
  replaces a live boolean's or union's whole list, `SetMembers`' shape with
  nothing inferred, and moves no DAG edge.
- **Feeding pairs to the fold.** Each certified pair is fed to the fold step
  at which both its sites are in the accumulation: the later member's step in
  list order, with the earlier side as the accumulator's operand and the
  later side as the joining member's. A pair whose two sites are one member
  is that member's carried contact at its own step. No fold position is
  recorded anywhere.
- **Consumed faces.** A pair a face of which the fold consumed whole before
  that step is satisfied, not `Vanished`: no row of the accumulation descends
  from the face, because another member contains it, so the contact has
  nothing left to back. A face that survives at its step only in pieces —
  split by another member, or inside a merged row a later step fragmented —
  is not consumed whole: rows do descend from it, and which of them carry the
  contact is not decidable from the names, so the pair refuses (Merges and
  order, below).
- **Resolution.** A name not in its site's table refuses typed through the
  pair boolean's own resolver, which the union reuses (one definition of
  "resolve a declared name at its site"); the site is the side, so a name
  carried by both operands is not ambiguous. What the union publishes is
  unchanged — `FromMember { member, of }` is what it mints; a sited pair is
  what it consumes.
- **Members leaving.** `SetMembers` leaves `declare` as it was; a pair whose
  site left the list refuses at the next evaluation as a vanished name does
  (N5), never silently.
- The "disjoint-only" reading is not taken: the common modelling case (a
  boss on a plate) would keep the pairwise chain alive.

**Merges and order.** A merged face's name is a flat constituent set (N3) —
whatever mints a `Merged` mints it flat, and a nested `Merged` is an emission
bug, never something a consumer flattens — so the accumulation's rows carry
no fold tree. A member-space name resolves at its step through whatever
merges the fold has performed (the union rewrites it to the flat `Merged` row
containing it before the shared resolver), so a declaration set whose faces
are consumed by merges fuses in every member order, and one whose face a
member contains is satisfied (above). A merge is the one consumption with a
unique successor, so it is the one looked through. A face split by a later
member, or inside a merged row a later step fragmented, has none, and a pair
naming it at a later step refuses while any piece of it survives: `Vanished`,
diagnosed `ConsumedByFold` with the composition read off the rows that
descend from the face (fragments of it, or fragments of a merged row covering
it, each bare or merged again), never by measuring it again. It offers
nothing, because which fragment the pair meant is the geometric question the
routing step does not ask. A face split and then contained whole in every
piece before the pair's step has no piece left, and its pair is satisfied
like any face consumed whole. So the outcome is not monotone in what the
later members cover: with `a`'s top cap split by `s` and a pair naming that
cap fed at `p`'s step, `[a, s, half, p]` refuses the split when `half`
contains one piece, while `[a, s, big, p]` fuses when `big` contains both.

**The refusal names member faces.** A contact is judged between two members,
so an `UndeclaredContact` finding sites both of its sides at member faces,
and the caller declares exactly what the refusal names. No refusal names a
row the fold minted, such as a `Fragment` (a `Seam` mints edges and vertices,
never a face). A fold step never meets an undeclared contact: a certified
pair is fed to its step, satisfied because its face was consumed whole, or
refused at that step, as a vanished name or naming the composition that left
it in pieces (above). A fold step that refuses a contact anyway is an emission
bug, because the fold mints no contact verdict, and a contact refusal from it
would tell the user to declare a pair the judgement already passed. The
refusal menu of `docs/SELECT-DESIGN.md` §3d keeps its two arms for every
`UndeclaredContact`.

*Built: the node, its naming and `SetMembers`, DOCM-3 (PR 1803); the
member-space declaration channel, DOCM-7 (PR 2028), sited at the members
(#2795) and made the node's own payload (#3587); the flat `Merged` mint and
the look-through, DOCM-8 (PR 2073); the typed refusal past the merges,
ruled on PR 2677; the pairwise contact rule (#3200, built in PR 3213).*

## DM5 — A node's inputs are pairwise distinct

`Boolean { a: X, b: X }` and a union or loft list with a repeated member
are refused. The rule is stated
once, as a structural validity check on a node's inputs, and called by
the edit doors' per-node checks — `InsertNode`, and `SetParam` at an
operand and `SetMembers` on the rewritten node — and by the load
validator (`persist/check.rs`, `validate_document`) on every node of a
snapshot, so the doors share the logic rather than mirror it. Replayed
edits meet it through `InsertNode`; the load validator is needed because a
hand-written snapshot never passes an edit door. Refusal:
`EditError::DuplicateInput { node, input }` at the edit doors, the
validator's own `SnapshotError` arm at load.

Distinctness is over the variables read, and only those (D10: a read is
of a variable): two outputs of one operation are two variables, so a
union of a split's two halves, or a revolve's body patterned about its own
axis port, is admitted. (The union then refuses the two halves at
evaluation, `MembersShareAnOperation`: DM4 keys a member's names by the
operation it reads. A pair boolean of them builds.) Two distinct nodes that
evaluate to one body — two `Part`s selecting one half of a split, or
`Part(Instance(0))` beside its master — meet DM5, and the boolean answers
them as it answers any operands whose shells coincide by structure or by
declaration: `A ∪ A` and `A ∩ A` are `A`, and `A − A` is the typed empty
result.

*Built: DOCM-3 (PR 1803), with DM4.*

## DM6 — No edit infers a re-point

A read changes only by an edit that names its new variable in full. An
operand slot is written by the one slot door every slot has: the formula
lowers to a read of the slot's kind (`SlotVarKind` otherwise), the read is
live, and the rewritten node passes every check the insert door makes of
a node — one function both doors call (`edit.rs`, `check_written_node`:
DM5's distinctness, an assertion's bound against its measure's
dimension, the slots' reads, the alignment and frame rules) — and
acyclicity over reads. A list operand is the same door with a list. The
write reports, and never refuses, the downstream names it strands, as DM7
has a removal do; the N5 ladder diagnoses them and `Rebind` repairs them.
No door picks a survivor: a delete leaves its readers unresolved and typed
(D10), and "delete and reconnect" is a client's offer of explicit slot
writes, the person choosing each. DM4's flat operators remain the way not
to pay a pairwise chain's naming cost.

## DM7 — A stranded name is reported at the edit that removes its referent, never refused

The edit that removes a name's referent — `DeleteNode`, and `SetProgram` for
the steps it drops and the kept pieces it stops drawing — stays legal when a
payload name (`Node::payload_names`) names what is being removed: a name is
not a DAG edge, and the carve-out in §0 stands. What the door owes is a
report: every `(node, name)` pair whose referent the edit removed rides the
accepted edit's `Applied.maintenance`, typed, computed at the door by the
same payload walk the insert door checks with. The strand is loud where it
happens rather than at the next evaluation; the N5 ladder's rungs —
`NodeGone` for a deleted minting node, `Vanished` for a name that denotes
nothing — and `Rebind` remain the diagnosis and the repair.

**What a reshaping strands.** A profile piece's name spells its step's minted
id (`names/README.md`, "N1, the profile pieces"), so:

- A name on a step the reshaping keeps still denotes that step's piece
  wherever the new program draws it, and is not rewritten.
- A dropped step's id is never minted again; every name on it keeps its
  spelling, resolves `Vanished`, and is reported stranded.
- A kept step's piece the new program does not draw — another piece took its
  segment, as a fillet inserted or moved before a leg takes the leg's
  (`names/README.md`, "Undrawn pieces vanish rather than alias") — is the
  reshaping's removal too, and its names are reported the same way. The door
  compares which pieces the old and the new program draw under the current
  parameters, as the program's own piece door answers, and reports a name
  whose piece the new program does not draw and the old one drew — or every
  such name, where the old program does not replay under the current
  parameters and so cannot say what it drew.
- A value edit reports nothing: it can move which loop is outer, which way a
  loop runs or how many segments a step draws, and none of those moves a
  name.

**The appearance store is the other carrier.** The report covers every
reference the document holds under N5 semantics, not only the node payloads.
An appearance attachment is keyed by a `StableName` in the document's
appearance store (`DocEdit::SetAppearance` gives it a declared pair's
semantics, `Rebind` repairs it, evaluation reports its loss as
`AppearanceLoss`), so an edit that strands one reports it too, as its own
`Maintenance` arm (`StrandedAppearance { name }`) rather than a `Strand` with
no carrying node. `Node::payload_names` stays the one list of node carriers.

- **Why not an edge.** A full edge over payload names reverses the carve-out
  (D3: a name is a reference, not an edge). The case that argued for one, a
  union whose declaration named the union's own space — a cycle
  `cascade_delete_order` could not see — is gone with declarations sited at
  the members (DM4), so the carve-out stands on D3's own ground.
- **Why not as-is.** A legal edit whose consequence is invisible until
  evaluation is what the maintenance column exists to end.
- The chrome's cascade affordance shows the strand count beside its
  dependent count; maintenance is derived, so nothing persisted changes.

*Ruled: `deletenode-strands-a-declare-payload-name`; the appearance store,
`stranded-appearance-keys-are-not-reported-by-dm7`; the edit that removes a
referent, #2904; kept steps and value edits, #3193.*

## DM8 — The authored-step to canonical-segment map is composed in `editor-core`

The map from an authored profile step (`SlotId::Profile { loop_, step, arg }`)
to the canonical segments it became (`CanonicalSegment { loop_index,
segment }`) is a function in `program.rs` reading the records the evaluation
already produces:

- **The answer, in the program's step order.** The replay's per-step segment
  span and per-radius emission (fields of `crates/profile`'s
  `ReplayStructure`, beside its fillet decisions) give the span for a step
  and the emission for a radius — the step a radius is authored on is not
  always the step its arc is credited to.
- **Carried into canonical numbering.** The profile's naming anchor carries
  the answer into the canonical numbering every verb iterates. The canonical
  form keeps what the author wrote wherever validity allows — each loop's
  authored start and the authored hole order, with only the traversal sense
  normalized (outer counterclockwise, holes clockwise) — so the canonical
  segment is the program's own for a loop authored in its canonical sense and
  its reflection `s ↦ n − 1 − s` for one authored against it.
- **Positions, not names.** A canonical segment is a position, not a name: a
  published name spells the piece a position is (`names/README.md`, "N1, the
  profile pieces"), and the same anchor pairs each position with its piece.
  A loft pairs canonical segment `k` of every section into one wall and names
  the wall by the pieces that pairing joined, one per section, each read
  through that section's own anchor.
- **Checked, not assumed.** The anchor is checked against canonicalization's
  `reversed` on `LoopCanonical`, its own record of the same permutation,
  before it is read; a disagreement between those two records is the
  evaluation contradicting itself and asserts. The door refuses typed where a
  record is absent or of the wrong shape rather than guessing.

The map is derived from the structure record the geometry came from, so it
cannot disagree with the geometry, and it is not persisted.

- **Why not the viewer.** A second derivation from both endpoints can
  disagree with the first.
- **Why not `crates/profile` alone.** It has no vocabulary for an authored
  step and would grow one for a consumer two layers up.
- Consumers: the viewer's per-segment focus marking, and the per-edge radius
  door in `program.rs` (`segment_radii`), which pairs each authored radius
  with the edge its own arc drew and is what a sweep's per-edge
  parameter-identity attach reads.

*Ruled: `authored-step-to-canonical-segment-map-has-no-home`; the wording,
`dm8-names-canonical-segments-but-the-published-refs-are-program-anchored`;
the canonical numbering, PR 3102 (`loft-section-correspondence-is-authored`);
the loft wall's name, #3193.*

## What this doc does not touch

Identity across time (`crates/editor-core/IDENTITY.md`), the instantiation
seam, the check registry, the certified range query.
