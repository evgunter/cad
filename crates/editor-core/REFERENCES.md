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
  silently shrunk (`eval/wire.rs`, `ladder` and `select`). Two kinds of
  carrier hold them. A **selection** is a definition, `Select { body,
  names }` (`var.rs`, `Select`): its body read is a DAG edge, its names
  are frozen, and it defines a set (`Faces`, `Edges`) or a singleton
  (`Face`, `Edge`, `Vertex`) that a node reads like any variable — a
  `Fillet`'s or `Chamfer`'s edges, a `Shell`'s open faces, a
  `Plane { face }` pose's face, a
  `Measure`'s refs, a `Mate`'s two sides. The N5 ladder runs in the
  selection's evaluation and nowhere else for these. A **payload** is a
  name a node holds itself: an `InstantiatePart`'s interface crossings'
  `outer`s (a crossing's `inner` is not a name of this document; the
  list's arm says why). The payload list has two homes, this clause and
  `Node::payload_names`' own doc; every other site points at the latter
  rather than restating it. A name is not a DAG edge, and two doors refuse
  on one: `InsertNode`'s liveness check (the selection's door,
  `mint_selection`, runs the same check on its names), and `split`'s
  `PartNameReachesRemainder` precondition, which refuses a cut whose taken
  node carries a name reaching the kept remainder (a selection's or
  an instance's crossing `outer`). A
  later delete strands a name (N5) and says so (DM7).
- **An `Expr` literal** in a slot, bit-pinned (D7).

Two precedents these clauses extend. `SitedRef { at, name }` (`node.rs`)
pairs a DAG edge with a frozen name: `at` says which evaluated value to read,
`name` says which entity. A selection is the same pairing as a definition,
its `body` the `at`. A `Revolve`'s axis (`node.rs`, `axis_through` and
`axis_heading`) is a point and a heading in its profile's own
coordinates: its meaning comes from the profile's plane, which does not
make the check cheaper but makes an axis out of that plane, and a zero
direction, unrepresentable. A pose definition (`VarDef::Pose`, `pose.rs`)
is a variable read off geometry, moved from another by a slide or a spin,
or constructed from other poses (D10), each of its reads a DAG edge (a
finer pose read through the role it is read through), bound at the
reader's seat.

Also kept from elsewhere: a selection freezes (#217, `node.rs`); selectors
materialize and are never stored (`names/select.rs`).

## DM1 — What stands on a face reads the face

What is built on a face locates itself by reading the face's geometry,
never by copying numbers off it. Two things stand on a face, and each
reads it in its own way:

- **A sketch on a face is drawn on a frame read off the face.** Its
  profile's `on` is a frame in the space of the face's body
  (`ProfileProgram.on`, `DrawnOn`): `Spin { pose: Through { axis, point
  }, about: z, by: spin }`, whose `z` is the face's outward normal, whose
  `x` points toward a cell of the body the gesture names (along an edge at
  a vertex, where the face has one), and whose `spin` turns it (the
  constant `0` unless named). The profile and every sweep of it are in
  that body's space, and nothing is placed. Extrude and Combine are the
  ordinary acts after it: a signed depth sweeps above the face (a boss) or
  into it (a pocket's tool).
- **A body on a face (a construction, a part) is placed against the face's
  plane**, by a `Plane` mate from its own start cap, flipped to look up
  its own `z` (`Flip`), to the pose `Plane { face }` (`pose.rs`), and
  values setting the freedoms the mate leaves — the slides along the
  target space's axes as the least turn carries them onto the face's
  normal, and the spin about it, under the orientation the gesture writes
  nearest the face. `face` reads a `Face` selection: its body read is the
  DAG edge to the body whose value the face is read out of, and its one
  name is frozen and resolved through that value's name table under the N5
  ladder. It is the `SitedRef` shape applied to a mate's target. The plane
  is the plane through the face with its outward normal (DM1a), and no
  u-reference, because a plane has no in-plane direction (D10: no pose
  definition takes a carrier's reference direction): where the body is
  turned on the face is the spin value's. A split's tool reads the same
  pose.

The clauses below are spelled for the placement; each holds for the
frame's reads in the same way.

- **Why derived, not frozen.** A sketch carries no twelve-float plane
  snapshot (`program.rs`); a placement whose target were nine literals
  copied off the face would reintroduce that snapshot one node out, and lie
  about why the construction sits where it sits. The face is a DAG input:
  its body is upstream, the copy moves when the face moves, and the
  placement participates in the memo and content key like every node.
- **The failure mode is the fillet's.** The plane reads its face through a
  `Face` selection, as a fillet reads its edges through an `Edges` one, so a
  face name that stops resolving fails the pose typed and poisons the
  placement above it with the same refusal (`SelectResolve`); the repair is
  `Rebind` on the selection's body.
- **DM1a — the read-back grows a sense, it does not fold one in.**
  `Pose.axis` is the chart's direction, deliberately uncorrected by the face's
  orientation sense (`readback.rs`): two facts, two answers. The sense
  travels beside the pose as `Pose::sense`, one more stored value copied out
  of the face record (`entity.rs`, `Face::sense`), returned by `face_pose`
  and `names::interrogate::face_frame`; DM1's plane states in its own
  vocabulary that its normal is sense times chart axis. The asymmetry is
  principled: a pose read off a face is consumed by evaluation, which reads
  geometry constantly.
- **DM1b — a non-planar carrier refuses typed at evaluation**
  (`PoseRead` with `PoseReadFault::NotPlanar`, naming the carrier kind
  found), so a headless author gets the same answer the chrome pre-empts
  under DM2.
- **DM1c — the lanes.** A pose read off a face has no document
  elaboration: it is read off the evaluated body at the lane scalar, so at
  `Interval` it is an enclosure, which is what a face's plane is at that
  scalar. The profile itself is 2-D and stays f64-pinned (PP1).

*Built: DOCM-1 (PR 1829), with DM1a, DM1b and DM2; the placement form,
INTENT stage 3 D.*

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
- The chrome offers DM1's sketch only on a planar carrier; DM1b is the
  kernel's own refusal when a caller bypasses the offer.

*Built: DOCM-1 (PR 1829).*

## DM3 — One member of a family is read by index; a split's halves are ports

A `Split` defines two `Body` outputs, `above` and `below` (the port index is
`SplitHalf::output_body`'s), read like any other output: a read of a port
is that half, its body and its own name table (N1: one table per output
variable). One member of a
family (D10, Repetition) is read as `xs[i]`, or `xs[i, j]` for a family
keyed by two indices: a read of the family at one `Count` expression per
index (VR4); an index outside the
range leaves the reader unresolved and typed at evaluation, never
re-pointed. `xs[i]` is not a variable: it is the read a per-`k` reader
makes implicitly, with the index written in the slot rather than supplied
by the evaluation. Names pass through unchanged (`role.rs`): the member
keeps its `Member { (i, …), of }` names, keyed by
the index variables' ids and the integers, and an operation reading it
keys them by `xs` (DM4), the index said once, in `Member`. So
`Union(xs)` and `Union([xs[0], …, xs[N−1]])` name alike, because they read
the same things, and inside a per-`k` reader `union(xs)` and
`union(xs[k])` are one read with one name. A member is not a named alias:
two readers wanting one member share the family and a named `Count`. A
name is scoped by the variable that holds its body (N1), so a split half's names do not spell the half: its variable says it,
and a selection of `split.above` states its body once. A body seat
reading a whole family, a profile or a split as a whole refuses by kind at
the door (`SlotVarKind`).

*Built: the indexed read at every body seat, rank one, FORK-DM4 unit 1
(PR 4527). Every family a node defines is one flat list, so `xs[i, j]`
waits on a family keyed by two indices
(`work/intent/an-indexed-read-of-rank-two-has-no-family.md`). Until stage
3 G, `Part { select: Instance }` stands beside it, a second way to pick a
member; new authoring writes `xs[i]`.*

## DM4 — Flat operators before splice: union and intersect are lists, subtract a pair

`Node::Union { members }` fuses its members into one body, and
names each entity by the read it came through. `Node::Intersect { members }`
keeps the material in every member, the same shape over the
kernel's intersection verb. `Node::Subtract { from, tool }` cuts
one body by another and is the one pair: difference neither commutes nor
associates, and nothing is a unary "not", so its two seats are named, and
several tools are `Subtract { from: a, tool: Union([tools…]) }`, the one
way to say it. There is no `Boolean { op, a, b }` and no `BooleanOp`.
`Node::Assemble { members }` is the fourth operator, a list like a
union's: it keeps its members as separate pieces, glues nothing,
records each touch between two members and reports an overlap as an
interference finding, and it is the at-rest census's door (DESIGN
[operations.operators], [assertions]).

A union or an intersect takes one argument of kind `Bodies` (D10). A
`Bodies` is defined by index or by enumeration. By index it is a family:
one program per value of `k`, its length a `Count`, its members named by
`Member { (k), of }`; `Union(xs)` joins them in index order. By
enumeration it is a spelled list of independent reads, `Union([a, b, c])`
for the three unrelated bodies just made, each member named by its own
read and joined in the spelled order; D10's "an irregular family is its
members written out" already says this. Every reader of a `Bodies` (union,
intersect, a placement's shapes) takes either. Today an enumeration is
spelled at the slot; naming one, a definition several readers share, is
not built, for lack of immediate need, and nothing in the design forbids
it. It is the argument a placement's
shape list takes (S3M). A mix (`[xs, a]`) puts a `Bodies` where a `Body`
goes and is ill-typed at the door (`SlotVarKind`), so a list beside an
individual cannot be written. A union or intersect of one body builds that
body (no boolean runs; names are `From { read, of }` as in any union), and
of none builds the typed empty body, so a family whose `N` is 1 or 0
builds instead of refusing.

**Why.** A multi-shell tool assembled as a chain of pairwise unions is an
artifact of the vocabulary rather than of the model. A pair node wraps every
operand's names once per link, so an entity's name records the depth at
which it joined.
In the die (`demos/tour/src/diefillet.rs`: 21 transforms of one ball fused
into one cutting tool and subtracted, then the twelve box edges and the 21
pip rims filleted), removing one link from such a chain would change the
names of every pip that joined before it, and the rim fillet's frozen
selection would fail typed for each of them (one rim for the second pip,
twenty for the last), repairable only by a `Rebind` per name through N5's
offers. A splice edit that assumed intent about which input survives would
carry that cost on top of its own. So the chain goes, not the link:

- **The node.** A union or an intersect reads its `Bodies` and defines one
  body. A union of two is `Union([a, b])`; there is no second spelling of
  it. It evaluates through the kernel's own n-ary union or intersect, which
  judges the member pairs and folds the members in order: the order is the
  list's, the author's statement. What the node decides is
  defined over its members, not over the fold: every coincidence between two members is judged once, pairwise,
  before the fold (below); the fold reads those verdicts through each
  face's member parents and decides no carrier pair again; and a name is
  defined over the finished body (N2). What follows the list says so: a
  glue keeps the earlier listed member's description (a subtract's keeps
  `from`'s), and a refusal raised at a fold step names the member whose
  step refused. Union and intersect differ in one respect beyond the verb:
  an empty fold step is the typed empty result for an intersect and a
  kernel bug for a union. A union reads shapes already in one space and
  places nothing: a placement defines copies, and the union reads them, so the
  union of a pattern's copies is this node over the family. When its
  bodies are rigid images of one body, visible in the reads, the union has
  a fast path: pairs it certifies apart are not judged (the group boolean,
  `README.md`).
- **Naming keys by the read, not by depth.** An operation that carries an
  entity in from an input wraps its name in `From { read: VarId, of:
  Box<StableName> }`: `read` is the element read the body came through (D10:
  a read is of a variable) — a seat's variable, a spelled member's read, or
  the family read for each of its members — and `of` the entity's name in
  that variable's table.
  The one segment serves a union's and an intersect's members, a
  subtract's two seats and a one-input operation's input (a shell's, a
  blend's) alike; which seat a read sits in is the node's to say, never the
  name's. The key is the read, never the inner name's minting node: a
  placement adds no name segment (N1), so two members that are placements
  of one body carry identical tables, and the inner name alone cannot tell
  them apart. Nor is it the operation the read reaches: two
  outputs of one operation are two variables, and `Union([split.above,
  split.below])` names its halves apart. The read is minted identity, as a
  node id is, and data the node already carries. A read spelled twice is
  one key, and its two members glue to each other (DM5). No position is
  recorded, so removing a member leaves
  every other member's names as they were, and re-pointing one by
  `SetMembers` changes its key, a recipe edit (N5). A family's members keep
  their `Member { (i, …), of }` segments (DM3), keyed by the index
  variables' ids, so they too are told apart by identity, never by position
  in the list.
- **`DocEdit::SetMembers { node, members: Vec<Operand> }`** is the one
  edit that changes a list input, by naming the whole new list; nothing is
  inferred. It refuses typed an unknown or non-live member, a cycle
  (`WouldCycle` through the existing check), or a mix of a family and a
  member (`SlotVarKind`).
  Deleting a pip is `SetMembers` without it, and the other
  twenty rims survive. The pip the union no longer reads is still
  defined, and out of the product because no placement names it;
  deleting it too is tidiness, not a requirement. An intersect's
  argument, a placement's shape list and `Loft`'s `profiles` are the same
  shape and take the same edit; nothing else in the vocabulary is a list.
- The viewer's combining doors take a union or intersect seat of N body
  picks, and a subtract's two (not yet built).

### Contact is judged pairwise, in member space, before the fold

Every coincidence between two members is a coincidence of the union,
whatever the other members are, so the union's coincidence rows are its
member pairs' rows: a contact a third member covers is still its pair's,
recorded, and a pair the sliver band leaves undecided refuses the union even
where a third member covers it. The union's rows are its evaluation's decisions,
complete: the judged rows and every decision a fold step takes, cited or
not. A coincidence among three or more members (a seam of two members lying
in a third's face) is the row of the step that decided it, spelled as the
set of member cells that meet, so one fact has one spelling; which steps
meet such a coincidence follows the fold order, as a step's refusal names
its step. The judged rows are the union's order-free rows, and a row says
which it is. The
faces each judgement merged or covered stay the union's face links (N2).
An intersect's members are judged the same way, over the pair verb's
intersection.

- Before the fold, each pair of members `m`, `n` whose closed bounding boxes
  meet is judged by the pair verb, as the two-member union `m ∪ n`; a pair
  whose boxes are disjoint cannot touch and is skipped. The judgement costs
  up to n(n−1)/2 two-member unions, bounded by that pruning; if it becomes a
  measured performance problem, it is raised to Ev, not optimized around the
  rule.
- A judged pair glues what its margins decide one carrier or tangent, and
  its rows, in member space, are the union's coincidence rows.
- Any refusal the pair verb raises on a judged pair is the union's refusal
  too, and so is order-free: a pair whose coincidence the pair verb cannot
  decide inside the tolerance grey band refuses `Escalated` in every member
  order, even where a third member covers it.

**One verdict per carrier pair.** The fold reads the pass's verdict for each
carrier pair through each face's member parents (its lineage, not its name)
and decides no pair again, so a piece is never judged at its own extent.
Every contact the fold meets is between two members, because the
accumulation's boundary is made of member faces, so every contact the fold
meets was judged here and the fold mints no carrier-pair verdict of its
own. The vertex and edge identities a step decides among the cells it
minted are that step's rows (above).

**A row names member cells.** A coincidence is judged between two members,
so its row sites both of its sides at member cells, and a row a fold step
decided is spelled as the set of member cells that meet: a cell the fold
minted (a seam, a crossing, a junction, a `Merged` edge) is the member cells
it is made of. Such a row's relation is `Meet`, the cells' carriers sharing
one point: a point decided on a cell keeps the two cells and `OnCarrier`
only where the point is one member vertex and the cell one member cell. No
row names a cell the fold minted, such as a `Fragment`.

*Built: the node, its naming and `SetMembers`, DOCM-3 (PR 1803); the flat
`Merged` mint, DOCM-8 (PR 2073); the pairwise contact rule (#3200, built in
PR 3213); the three nodes over one `Bodies` argument, keyed by the read,
FORK-DM4 unit 1 (PR 4527); one verdict per carrier pair, read through
lineage (#4323); the rows as the evaluation's decisions, spelled by member
cells (PR 5378).*

## DM5 — A read repeated is answered by the operation

No door refuses a repeated read. A read that appears twice among one node's
inputs — `Union([A, A])`, `Subtract { from: X, tool: X }`, a loft section
listed twice — is the same case as two distinct variables that evaluate to
one body (two `Part`s selecting one copy of a pattern, a transform of `A`
by the identity beside `A`), and the operation answers both alike:

- **Union and intersect keep both and glue.** `[A, A]` is judged like any
  pair (DM4): every cell meets its twin, one construction read twice, so
  the coincidence is structural and the glue keeps `A`'s description. `A ∪
  A` and `A ∩ A` are `A`; each glued row is `Merged` of a set whose
  elements are one name, which is that name (N3).
- **Subtract.** `A − A` is the typed empty result.
- **Assemble.** `A ⊔ A` is two coincident pieces, an interference
  finding.
- **Loft.** A section listed twice is degenerate geometry, refused by the
  loft at evaluation.

A meeting one operand already decided (two of its own cells touching)
is that operand's record, carried, and no operator above it judges it
again (DESIGN [coincidence.booleans]). Two outputs of one operation
are two variables, so a union of a split's two halves, or a revolve's
body patterned about its own axis port, reads two
things and names them apart (DM4). Two indexed reads `xs[i]` and
`xs[j]` landing on one member at the current values are the repeated read
of that member, answered the same way. The alternatives are not taken:
refusing the repeat keeps a syntactic refusal beside a semantic admission
of the identical geometry, and collapsing the list as a set rewrites what
the author wrote and is a second rule beside the glue two variables get.

*Built: FORK-DM4 unit 1 (PR 4527). The refusal this clause replaces
(`EditError::DuplicateInput`, the load validator's arm) was built in DOCM-3
(PR 1803).*

## DM6 — No edit infers a re-point

A read changes only by an edit that names its new variable in full. An
operand slot is written by the one slot door every slot has: the formula
lowers to a read of the slot's kind (`SlotVarKind` otherwise), the read is
live, and the rewritten node passes every check the insert door makes of
a node — one function both doors call (`edit.rs`, `check_written_node`:
an assertion's bound against its measure's
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
appearance store (`DocEdit::SetAppearance` gives it a payload name's
semantics, `Rebind` repairs it, evaluation reports its loss as
`AppearanceLoss`), so an edit that strands one reports it too, as its own
`Maintenance` arm (`StrandedAppearance { name }`) rather than a `Strand` with
no carrying node. `Node::payload_names` stays the one list of node carriers.

- **Why not an edge.** A full edge over payload names reverses the carve-out
  (D3: a name is a reference, not an edge), and the carve-out stands on D3's
  own ground.
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
