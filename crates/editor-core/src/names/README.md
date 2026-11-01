# Persistent naming (NAMING-DESIGN N1–N7)

Recipe steps and GUI picks refer to boundary entities of intermediate bodies.
Arena keys cannot be those references: they are body-lineage-scoped and diverge
after any edit that changes kill history. A stable name is a *derivation path*,
the minting recipe node plus the combinatorial role the entity plays there; it
denotes a construction role, not a point set. Because replay is a specified
function of the recipe (D9) and every birth is recorded (D5), evaluation emits
the name↔entity table and re-resolution is a lookup, never a match.

## Where in the code

| Decisions | Module |
|---|---|
| N1 `StableName`, `RolePath`, `RoleSeg`, `EntityKind`; N2 `Qualifier`; N1's pass-through set as the recipe walks read it (`verbatim_edge`: split's carried-name test, the mate member walk and the coincidence door's walk to a construction) | `role.rs`; `RecipeNodeId` in `crates/editor-core/src/node.rs` |
| N4 `NameTable`, `Entry::{Unique,Tied}`, `EntityRef` | `table.rs` |
| N4 emission, `NamingError` | `emit.rs` (helpers, totality check), `emit_sweep.rs` (extrude/revolve/loft), `emit_topo.rs` (boolean, split, N3 merge), `emit_union.rs` (the n-ary union: member-keying in, collapse out), `emit_blend.rs` behind `emit_fillet.rs`/`emit_chamfer.rs` (an edge its closing join made over several trims, rim trims or survivors `Merged`), `emit_shell.rs` (the shell: survivors `From` the target's read, cavity twins `Inner`, a chart's rim `Rim` of its first designated face, a hole's promoted annulus `HoleRim`, an edge its closing join made over several input edges `Merged`) |
| N1's node and profile step ids: the mint chain and mint log (`Mint`) | `crates/editor-core/src/mint.rs`; `RecipeNodeId` and `StepId` in `crates/editor-core/src/node.rs` |
| N2 discriminators — `Borders` over the kernel's record of what a boolean discarded and over a Split's tool plane, `Keeps`, `Ends`, the crossing's sense; tie propagation | `borders.rs`, `plane_pieces.rs`, `discriminate.rs`; `defer.rs` |
| A path's canonical form: its name-ordered positions (N3 sets, `Borders` walls, `Keeps` edges, `Ends` pairs, a junction's lines, an `EdgeCrossing`'s run, a seam's sides, a `Through` sense's things) | `canonical.rs`, which the mint, the union's collapse and every rewrite of a published name end in; `seam_pair.rs` (which seam an edge lies on, and which of its faces is which side) |
| N5 `ResolveError`, `Diagnosis`, tombstones, offers; diff engine; hit-testing; `Rebind` | `crates/editor-core/src/resolve/mod.rs`; `resolve/vdiff.rs`; `resolve/hit.rs`, `resolve/pick.rs`; `edit.rs` |
| N6 a recorded cell's read and name; the walk from them to the minting node | `crates/editor-core/src/coincide.rs` (`NamedCell`, `construction`) |
| Which node minted a named entity (`NameOrigin`); name → geometry (`denotation`, `face_frame`, ...) | `attribute.rs`; `interrogate.rs` |
| Selectors, geometric filters, detect/declare | `select.rs`, `geompred.rs`, `flush.rs`; design in `docs/SELECT-DESIGN.md`, usage in `docs/guide/selecting.md` |

## Names

**N0 — What a name is.** Naming instantiates `[intrinsic.contingent]`
(`docs/DESIGN.md`): a name is decided by what matters to the entity, never by
how the operation happened to compute it. N1–N5 are instances of N0; where a
case has no rule, N0 decides it, and a lane derives the name rather than
forking a question. Each principle names the check that holds it.

- **NS1, what a name may read.** A name may read only:
  - the operation's kind and its authored structure, up to the operation's
    declared symmetries. Each operation says which inputs are a set and which
    a sequence: union and intersect members, a seam's sides, `Merged`
    constituents, `Borders` walls, `Ends` and an `EdgeCrossing`'s lines are
    sets; a subtract's seats, loft sections, profile steps and a pattern's
    index are sequences;
  - the reads it takes, by minted identity (DM4), and the names in their
    tables;
  - the entity's descent from the kernel's birth records (D5);
  - the verdicts that decided it. For a union these are the judged pairwise
    verdicts, which are order-free (DM4), never a fold step's decision.

  Nothing else: no order of any kind (fold, completion, pairing, insertion,
  arena storage), no slot beyond the declared sequences, no frame, no value.
  A *slot* is the position an input fills: a subtract's `from` and `tool`,
  or a two-member union's first and second member. A sequence's slot means
  something (what is cut from what), so a name may read it; a set's does not
  (swapping a union's two members makes the same body), so a name may not.
  List order is the same: a loft's sections are a sequence, a union's members
  a set.
  A set is spelled in name order, which is a spelling, never a choice.
  *Check:* the table is unchanged, up to ties, under each of these: the
  f64/Interval pin; a permutation of a set input; a frame change of a split
  plane; a tilt within one topology; two separately authored identical
  operands; a replay under a different edit history (up to the bijection of
  minted ids); and an operand's arena storage permuted.
- **NS2, one grammar, read off the finished body.** Every entity takes one of
  four forms over its descent, and a new entity is named by placing it in
  one:
  - **Carried**: one input entity whole, `From { read, of }` or a
    pass-through.
  - **Merged**: it covers several input entities, `Merged(flat set)`.
  - **Met**: it lies in no input entity of its own dimension, and arises
    where input entities meet (`Seam`, `Crossing`, `EdgeCrossing`), spelled as
    the set it is made of.
  - **Made**: the operation builds it under a role its kind fixes (`Cap`,
    `Band`, `Pole`, `BlendFace`, `Inner`, a profile `Piece`). The role's
    arguments are names, reads, locators, verdict enums and integers the
    author wrote, never a counter.

  A form that does not tell an entity from its siblings takes a `Fragment`
  qualifier (NS3). *Check:* one exhaustive map from each `RoleSeg` variant to
  its form; and a denotation verifier, independent of the emitters, that
  re-reads the finished body and the records and confirms what each name
  claims.
- **NS3, locality.** A name reads nothing outside the entity's closure (its
  boundary and the lines it lies on) and its parents' closures. A qualifier
  reads only the entity's own relation to its siblings: walls, kept edges or
  ends. It never
  reads how many siblings there are or which came first. *Check:* an
  unrelated feature added elsewhere leaves every surviving entity's name.
- **NS4, totality and identity.** Every live entity has exactly one name, and
  distinct entities have distinct names. Two entities nothing covariant tells
  apart are a tie (N4); two operands with identical tables are told apart by
  their reads, never ordered or collapsed. A naming refusal on a legal body is
  a defect, never a policy. *Check:* `check_total` and `NameTable::insert`,
  and a coverage fuzzer over legal arrangements that admits no `Naming(*)`
  refusal but `Escalated` and `Band`.
- **NS5, complete records.** A name reads descent from the birth records and
  never reconstructs it; a missing or wrong record is a kernel defect, fixed in
  the kernel. *Check:* the D5 record validator, and the NS4 fuzzer.

Coincidence rows are records, not names (REFERENCES DM4): their presence
follows the decisions an evaluation took, and NS1 governs only what a name
reads of them.

**N1 — A stable name is a derivation path.** `StableName { kind, node, path }`:
a runtime `EntityKind` (Body, Face, Edge, Vertex — bodies are first-class), the
minting `RecipeNodeId` (minted at insertion from the document's mint chain, as
a step id is below; never positional, never reused), and
`RolePath = Vec<RoleSeg>`. `RoleSeg` is one closed enum grouped by op: extrude
(`Cap`, `Lateral`, ...), revolve (`Band`, `Pole`, ...), the carry-through
`From { read, of }` (an entity an operation brings in from an input, keyed
by the variable the input slot reads, DM4: a union's or intersect's members,
a subtract's two seats, a shell's or blend's one input), boolean (`Seam`,
`Merged`, `Fragment`), split (`SectionFace`, `SectionEdge`, `SplitFragment`,
...), blend (shared by fillet and
chamfer, told apart by the minting node), pattern `Instance { i, of }`
with `i` recipe-structural. Role arguments are themselves names; profile locators
(`ProfileEdgeRef`, `ProfileVertexRef`) name a profile piece by the id its step
was minted with, never by its position (below). Names contain no floats and no arena keys; a pass-through op (a
split-intact entity, a read of a split's half, a `Part`'s projection of one instance, a mirror's
image) adds no segment, so `node` stays the original minter, and so does
a copy (a placement's of its source, an instance's of its part's
product), which carries its source's names verbatim. Copies are told
apart by the variable that holds each (N1), and under a `⊔` by the
`From { read, of }` that keys each member by its read. Names are
document-local; assembly's are `ASSEMBLY.md`'s.

**N1, the scope.** A name is scoped by the variable that holds its body:
there is one name table per output variable, and a name tells apart only
what that variable holds. An operation with several outputs never spells in
a name which output holds the entity, so a split's roles carry no half
(`SplitBody`, `SectionFace`, `SectionEdge`, `SplitFragment`,
`CrossingVertex`): the variable says it, through the read that carries the
entity on (`From { read: split.above, of }`) or the body a selection
states. Each output has its own table, so a split's roles say no half;
the pattern's `Instance { i }` segment still restates the output, only to
keep one node's table distinct. D10 makes the compensation fail: `P1` and
`P2` are two placements of `X`, each equally a copy of it; pinned into one
space, a later `Place [P1, P2]` defines two outputs carrying `X`'s rows
verbatim (a placement adds no name segment, above), which one table per
operation refuses as `DuplicateName`.

**N1, the revolve poles.** `Pole(v)` names the ONE body vertex an on-axis
profile vertex revolves to, looked up in the sweep's `poles` export.

- A PARTIAL revolve keeps the axis run: the rotation fixes every point of it
  and both meridian chains meet at each of its vertices, so an INTERIOR vertex
  of a subdivided axis run (an on-axis side carried by several collinear legs,
  which the continuation verbs author) is structurally a pole and takes
  `Pole(v)` like the run's tips.
- A FULL revolve deletes the axis run outright, so an interior vertex of it has
  no body entity and the export answers `None`. So does a tip where a plane
  wall meets the axis: the full revolve builds that disc whole, its centre no
  vertex. The tips a cone or a curved wall ends on are the named on-axis
  vertices. A loop meeting the axis in several runs is no exception: the
  inner run ends are the poles of the cavities the gaps close, and those
  cavities' walls, meridians and rims are named by the profile pieces that
  sweep them, as the outer shell's are, with no role of their own.

Totality checks that silence: `check_total` refuses a table leaving a LIVE body
vertex unnamed, so a `None` standing over surviving geometry cannot pass.

**N1, the profile pieces: authored things are named by minted ids.** A name
spells what the author made by the id it was minted with, and what the kernel
made by the verdicts that decided it. Nodes and reads (`From`, DM4) follow
this rule, and so do profile pieces:

- **The id.** Every step of a profile program carries a `StepId` in the recipe
  (`ProfileProgram::ids`). It is minted when the step is authored, by
  `InsertNode` or `SetProgram`, from the document's mint chain: a digest
  the document carries, which each minting edit extends by that edit's
  canonical bytes. An `InsertNode`'s bytes are its node as the edit
  states it, which holds its inputs' and names' ids but not its own; the
  insert then extends the chain once more and takes the node's
  `RecipeNodeId` from it. The steps one edit mints take the chain's next
  digests, one per step in authored order. An id is a pair: its mint
  ordinal (the mint log's length when it was drawn, plus one; a `u32`) and
  the first 64 bits of its digest. Ids order by ordinal first, so they order
  as they were minted: of two nodes or steps in one document, the one minted
  first has the lesser id, and the document keeps no separate record of the
  order its nodes were placed in.
  So a node's or a step's id is a function of the edit sequence that
  minted it:
  - the same sequence of edits from one value mints the same ids (D9);
  - two documents that branch from one value — an undo followed by a
    different edit, or two edits applied to one base — mint different ids
    for their different nodes and steps, so a name carried from one
    branch into the other spells a node or a step that branch never
    minted and denotes nothing there, rather than another one. A
    parent's name on a step one such version minted, held across a pin
    update to the other, resolves `Vanished`.

  Like a `RecipeNodeId`, a step id is never reused and is unique across the
  whole document; its order is mint order, which is the order the author
  placed things in, and nothing else. The document keeps every
  id it has minted, node and step, in one mint log, deleted nodes' and
  dropped steps' included, in mint order, so no id is minted twice. The
  load door checks what minting makes true: the log's ordinals count up
  from one, every node's id in the mint log, one id per authored step,
  every step's id in the mint log, and no id standing for two steps. A name may spell only a step the
  document has minted: the doors that write a name (`InsertNode`, `Rebind`,
  `SetAppearance`, `SetAppearanceMeta`) refuse one the mint log does not
  hold, and so does the load door.
- **The address.** An author reaches a step's id through an authored address,
  not a name: the step's index in its loop and its program's shape up to that
  step, values erased, read against the loop the author states. The profile's
  program maps the address to the id it minted for that placement, so one
  authored loop placed twice resolves to two ids, a value edit leaves the
  address valid, and a reshape that changes the prefix refuses it. An address
  is valid for the program it was authored for; across a `SetProgram` an
  author holds the `StepId`. The check is the prefix alone, so a wrong loop of
  the same shape, or a stale address whose prefix the program still has, binds
  without error; and equal addresses are one key in a keep map.
- **The role.** A step draws its pieces from a fixed list of roles, one list
  per verb. The piece door (`ProfileProgram::piece`) checks the list, and the
  authoring surfaces' role accessors are generated from it:
  - every verb that draws one segment has one role, `Leg`: `line`,
    `line_to`, `continue_to`, `arc_to`, `tangent_arc_to`, `arc_onto`
    (whose leg the director after it lands, but which it names), `to`
    (the far end), and the close to `Start`, whose leg is the seam;
  - a verb that only binds a point, a direction or a junction (`at`,
    `angle`, `toward`, `tangent`, `cusp`, `turn`) draws nothing and has no
    role;
  - a fillet has three roles, `RunIn`, `Arc` and `RunOut`: the run into it,
    its arc, and the run out of it. The fused verbs have the same three,
    and their authored arc carriers are their runs: `arc_fillet`'s incoming
    arc is its `RunIn`, `fillet_arc`'s arrival arc its `RunOut`, and
    `arc_fillet_arc` has both;
  - a `circle_split` has `Piece(k)` for each `k < n` (the `n` is
    structural). `Piece(k)` is the arc from the split's vertex `k`, at
    `phase + 2πk/n`, to vertex `k + 1`. The phase is a value, so a phase
    edit moves every piece along the carrier, and whatever hangs on it,
    and renames none: a shift by a whole number of arcs draws the same
    circle with each piece some places along it;
  - a `circle` has `Piece(0)` and `Piece(1)`. It is drawn as the
    `circle_split` with `n = 2` and phase 0: two semicircles, split at the
    circle's `±x` points.

  A fillet's pieces belong to the step that authors its radius (the replay's
  `RadiusEmission.step`), not to the later step whose arrival resolves the
  corner.
- **The locator.** `ProfileEdgeRef::Piece { step, role }` names the piece that
  role of that step drew. `ProfileVertexRef::Piece { step, role }` names the
  vertex where that piece starts, in authored order: a loop's vertex `k` is
  where its piece `k` starts. A locator holds no loop index and no segment
  index.
- **A kernel-built section.** A section a kernel door builds rather than an
  author — a tube's circle and a hollow tube's bore — has no authored step. It
  is named structurally under the node that builds it:
  `ProfileEdgeRef::Section { circle, role }`, the circle `Outer` or `Bore` and
  the role `Piece(k)`. The node kind fixes the section's shape, as it fixes an
  extrude's two caps, so nothing can renumber it.
- **What cannot move it.** None of these changes what a locator denotes:
  - a value edit;
  - a change in which loop is the outer one, or in which way a loop runs;
  - a document parameter passing through a state that does not replay;
  - a `SetProgram` that keeps the step;
  - a pin update over a child document that kept the step.
- **Undrawn pieces vanish rather than alias.**
  - A role the current values do not draw has no piece. An example is a run
    that a `Zero` fit suppresses. A locator on it resolves `Vanished` until
    the values draw it again.
  - Two pieces on one carrier can be drawn as a single segment. That segment
    answers to the earlier piece in authored order, and the later piece's
    locator resolves `Vanished`. A fillet's run and an authored leg on the
    same carrier are such a pair. A leg into a fillet is authored before
    it, so the leg keeps the segment and the fillet's `RunIn` vanishes. The
    fillet is authored before the leg that follows it, so its `RunOut`
    keeps the segment and that leg's `Leg` vanishes. In the chain `at`,
    `toward(+x)`, `fillet(r)`, `toward(+y)`, `to (2, 2)`, `line_to(0, 2)`,
    `line_to(Start)`, the right wall is `{fillet, RunOut}`, and the far-end
    step's `Leg` denotes nothing. At a radius whose run out fits `Zero`
    the right wall is not drawn, and `{fillet, RunOut}` resolves `Vanished`
    until a radius draws it again.
  - A step that `SetProgram` drops takes its id with it. A name on that step
    keeps its spelling and resolves `Vanished`, and DM7's report names it.
    Because the id is never minted again, no later program can draw it.
  - A `SetProgram` that keeps a step can still leave one of its pieces
    undrawn, when a piece it adds or moves earlier on the same carrier
    takes the segment: a fillet inserted before a leg takes the leg's.
    A name on that piece keeps its spelling and resolves `Vanished`, and
    DM7's report names it, as it does for a dropped step. It comes back
    only when a later program draws it again.
- **The canonical numbering is not a name.** It is still the order in which
  the emitters, the loft's correspondence and the viewer's per-segment marks
  iterate (V3, DM8). When a profile's value is built, the naming anchor
  (`eval/anchor.rs`, `ProfilePieces`) pairs each canonical segment and vertex
  with the step and role that drew it, read off the replay record's
  per-segment piece (`profile::ReplayStructure::pieces`), and the sweep
  emitters name what they iterate by it.
- **Loft walls.** A loft pairs canonical segment `k` of every section into one
  wall (DM8). The wall's role-path segment (`LoftWall`) holds one locator per
  section, naming the pieces that pairing joined, and so does a seam edge
  between two walls (`LoftSeam`): one vertex locator per section. A cap and its
  rims and vertices belong to one section, and are named by that section's own
  locators. If a value edit changes which pieces pair, the old wall's name
  vanishes; it does not follow `k` to the new pairing.
- **Swept walls over a run.** Extrude and revolve build one wall per run of
  profile pieces on one carrier (`crates/sweep/README.md`, "Walls"). The
  wall's role-path segment (`Lateral` for extrude, `Band` for revolve) holds
  the run: its piece locators in authored order, a one-piece run spelled as
  one locator; a run that wraps through the loop's start begins at its first
  piece after the start vertex. `LateralEdge` and `BandRim` are minted only
  where an entity exists, so a station inside a run has none; a cap's rim
  is one edge per run too, and holds the run as the wall does, as do a
  partial revolve's meridians and its axis edge. A run wall
  is not a merge and never `Merged`.
  Covers and offers (N3) read one constituents view shared by every row
  that holds a set of names — a `Merged` face, and a run held by
  `Lateral`, `RimEdge(end, ·)`, `Band`, `BandPi`, `Meridian(end, ·)` or
  `AxisEdge`: `Lateral([p0, p1])` covers `Lateral([p0])`, so a selection made before a station was inserted
  is offered the run wall, and an edit that breaks a run offers its pieces'
  walls. A `LoftWall` holds one locator per section of ONE wall, not a set
  of walls, so it has no constituents in that view.

**N2 — Split discriminators are covariant margined predicates.** When one source
yields n fragments, `Fragment(Qualifier)` follows the parent-bearing segment.
The qualifier depends on what was split:

- **A face piece of a boolean or union** takes `Qualifier::Borders`: the sorted
  set of the piece's divider walls, the faces across its seam edges where it
  meets an obstacle that divides the parent, each cited by its parent's name.
  An obstacle is a connected part of the parent's region that no piece holds,
  connected through its closure: two lost regions that meet only at a point
  are one obstacle, whether that point survives as a vertex or the output
  stage joins it away, so a name reads nothing off where a dead vertex sat;
  it divides the parent when it borders two or more pieces, so a boss or notch
  on one piece is never cited unless a point of it touches a divider, which
  makes it part of that divider, and nothing in a piece's name lies beyond
  its own boundary. Pieces with equal sets are N4's tie.
- **The Split op's pieces** are told apart by their tool plane's side, which
  is the output variable that holds them (N1, the scope), so
  `SplitFragment` spells no side: there the plane is what the author drew. A
  piece whose side flips when the plane moves leaves its variable's table
  rather than renaming, and N5 diagnoses `PredicateFlip` from the split's
  recorded classification, not from the name. Several pieces of one parent
  in one half are further qualified by `Qualifier::Keeps`, the sorted set
  of the parent's boundary edges each holds a stretch of, cited by their lines
  (below); equal sets tie.
- **A Split's section faces** are pieces of one parent, the tool plane.
  - One plane piece in a half is the bare `SectionFace`.
  - Several are each `SectionFace` + `Fragment(Borders)`, read over the
    plane's complement in that half. An obstacle is a connected part of
    the plane the half's pieces do not hold, and it divides where it
    borders two or more pieces. A piece's walls are the operand faces
    across its edges along a divider, each cited by its parent's name, as
    its `SectionEdge` chords cite them. So two sections never cite each
    other, and a hole that no other piece sits in renames nothing.
  - Equal sets tie (N4); nothing breaks the tie by an order, since every
    order the join has reads the plane.
  - A parent that holds the plane qualifies its pieces by `Borders`, since
    the plane has no boundary edges for `Keeps` to cite.
- **Edge pieces** take `Qualifier::Ends`: the sorted pair of a piece's two end
  vertices' names as the node publishes them. A piece is spelled on its parent
  edge's *line*: the edge's name with every piece qualifier removed, at every
  depth of wrapping. So a piece of an earlier piece is a piece of that line,
  told from its siblings by its two ends, and nothing in its name records a cut
  made elsewhere on the edge. This covers every piece of a
  parent edge, a lone piece on its side of a cut included, so no piece's name
  says how many siblings it has: a seam chain's pieces, pieces of an operand
  edge, pieces of an earlier seam, and a union's pieces of a member edge
  (`From(m, e)` + `Ends` over the union's published vertex names, read
  off the finished body).
  Section chords are the same case: a section line that re-enters one operand
  face (an inner loop, a non-convex face) cuts several chords that
  `SectionEdge{face}` spells alike, and each takes `Ends` like any other
  edge piece (Ev, PR 3553). Pieces with equal pairs are N4's tie.
  An operand edge lying in the tool plane is read as a union reads a flush
  edge (below): an edge of a half along it is that edge, or a piece of it,
  only when both its faces descend from that edge's two faces. One with a
  section face beside it is a chord of the operand face on its other side,
  `SectionEdge{face}`. A split finishes each half with maximal faces (N3),
  so no edge of a half lies between two section faces.
- **Vertices** cite the edges they lie on by their lines, never by a piece's
  qualifier at any depth, so vertices are named before edge pieces are
  qualified, and
  nothing in a piece's name lies beyond its own boundary. A vertex of either
  operand is named as that vertex, wherever the other operand meets it: an
  edge that ends on a face of the other operand ends at its own vertex, and
  that vertex is the name. Every other vertex where a face of one operand
  meets an edge of the other is a crossing, named by its sense: where the
  crossed edge's two sides at the vertex lie against the closed body the
  crossing face belongs to. An orientation is read, never chosen (N0 NS1):
  a line of one edge is read as the operand body stores the edge, and a line
  of several things (a `Merged` line of flush edges, or a seam, whose two
  sides' loops run opposite ways) is read along each of them. A transversal
  crossing of such a line is `Through { entering }`: the subset of its edges,
  or its seam's sides, in name order, whose direction runs into the body. A
  touch is `Outside` or `Inside` on any line. On a line of one edge the sense
  is `Enters` (outside, then inside), `Leaves` (inside, then outside),
  `Outside` (outside on both sides: a touch from outside) or `Inside` (inside
  on both sides: a touch from inside); a side lying on the face counts as
  inside. For the Split that body is the half the edge runs into, and a plane
  only enters or leaves. Every crossing carries it, a lone one included: a
  boolean's or union's is `Crossing { edge, face, sense }`, the Split's
  `CrossingVertex` holds it as a field.
  A vertex where two or more lines meet is an `EdgeCrossing` over the run of
  those lines, each with its sense, in name order. Its lines are the edges cut
  at the vertex, edges that run flush on one line counted once, as the
  `Merged` set of them, flat and in name order, and, at a union, each seam
  through the vertex whose two faces bound none of those edges, cited by its
  faces' parents as a seam edge is. So a union vertex no member edge reaches
  is the run of its seams, and a lone member edge through a seam is the run
  of the edge and the seam. A face through the vertex that bounds none of its
  lines is not in its name: the lines locate the vertex, as a corner resting
  on a face cites no face.
  Each edge in the name carries its sense, read against the closed bodies of
  the members the name's other entries belong to, other than the edge's own:
  the crossing face's member for a `Crossing`, the other lines' members for a
  run. A seam in a run carries its sense as any line of several things does.
  A boolean reads the sense at the step
  that mints the vertex. A union reads a member edge's from its members'
  pairwise judgements, a side being inside when any judgement classed it
  so; a side no judgement classed refuses. A union's seam in a run is read
  by which of its pieces the result keeps at the vertex: a side whose piece
  ends there runs into the member that trimmed it, and two kept pieces read
  `Outside`.
  Reversing an edge swaps `Enters` and `Leaves` and keeps the other two.
  Nothing is ranked: crossings, and vertices of one run, that their names do
  not tell apart tie (N4).
  Where the result holds an operand corner as several vertices, each is a
  fragment of it, qualified as the operation's face pieces are. At a boolean
  or a union it is `Fragment(Borders)` over the walls that divide the corner
  there, read as a face piece's walls are (`borders.rs`). At a Split, where
  one half holds the corner as several vertices, it is `Fragment(Keeps)`:
  the sorted set of the operand corner's own edges, cited by their lines,
  that the vertex is an end of a piece of in that half. Equal sets tie, and a corner the result holds as one
  vertex (in a Split, one per half) is its bare name.

No rule reads a plane or a direction, and a union's reading of a seam pair in
name order changes nothing. Name order spells a set, and at a union it
chooses no entity: what several members hold at once is named for all of
them (below).

**Flush edges at a union.** Each union edge is named for the member edges it
lies on, read off the finished body and the members' own bodies
(`emit_union::Flush`), so the name is the same in every member order:

- An edge that lies within one member edge is a piece of it, whatever the
  fold named it. One that lies within several, where they run flush, is the
  same edge in each of those members, and is named for the set of them:
  `Merged` of those member edges, flat and in name order, as a face two
  members hold coincidently is `Merged` of both (N3). No member's edge is
  chosen over another's, so the name is the same whichever member was
  authored first, and two members with identical tables (a body and its
  exact copy) are told apart by their reads. An edge lies within a member edge when it lies within that
  edge along its carrier and both of its faces descend from that member edge's
  two faces. An edge one of whose member faces was glued away, another
  member's face lying over it, does not: it is the seam of the faces it
  separates, as a subtract's pocket rim is, and it vanishes when the junction
  does.
- An edge that lies within none, but runs along several member edges that
  together cover it, is named for that set: `Merged` of those member edges,
  flat and in name order, the shape faces have (N3). Such an edge is a flush
  rim the output stage joined across members (maximal edges, `docs/DESIGN.md`).
  "Along" means on the edge's carrier and overlapping it over a length, so a rim
  that only partly overlaps an edge is in its set. Several pieces of one set
  are told apart by their ends (N2).
- An edge that runs along member edges for part of its length and along a seam
  for the rest is named from its two faces, as a seam is.

The set does not retire its constituents the way N3 retires a merged face's.
A member edge a set lists can also hold a stretch outside it, and that
stretch keeps its piece name, the member edge qualified by its ends (N2),
never the bare member edge. Unlike two coplanar faces, the stretch and the set
are different cells on the line, and a name cites only what its cell lies on.

A subtract names a joined edge by the same reading over its two operands'
edges. Both read along the edge's carrier: its line, or the curve a curved edge
is carried by, where "along" is overlapping it over a length of that curve. A
closed edge, one the output stage joined round a closed carrier, lies within a
closed operand edge alone and is covered over its whole period, so its name
reads nothing off where its vertex sits.

A split and a shell name an edge their closing join made by the same reading
over their one operand's edges, read off the joins' records rather than along
a carrier: each edge the join took is chased to the operand edge it came from.
An edge that covers one operand edge whole, the cut having separated nothing
of it, takes that edge's own name. One that lies along a part of one operand
edge whose other parts live on is a piece of it, named as the split names a
piece. One that covers several operand edges whole is the `Merged` set of
their names, flat and in name order: the shell's survivors as `From` of
their names and its cavity twins as `Inner` of theirs, a set among them listed by its
constituents. A split's section chords, joined, stay a chord of the face they
cross, and a join over section chords and whole operand edges, across a
merged face, is named from its two faces, as a seam is. A split's join that
lies along several operand edges, holding part of one or more of them, is a
piece of their `Merged` set, flat and in name order, several of one set in a
half told apart by their ends, as a union names a flush rim along several
member edges. A join with no such reading, a chord with an operand edge's
piece, is refused, not named.

A blend names an edge its closing join made the same way. Each trim the blend
mints is the image of one input edge on one support, as a cavity twin is, so
an edge its join made over several of them is the `Merged` set of their
images: `TrimEdge` for a trimline, `BandTrim` for a rim trim, `From`
for a surviving input edge, kinds mixed as they come. An edge the blend
minted outright, an end arc, a mitre, a band's slit or a remnant (`BandCut`),
is the image of no input edge, and a join that takes one is refused. The
split, the shell and the blend read their joins through one helper
(`join_names.rs`), each supplying only which input edge a covered edge is
the image of.

A seam vertex cites an input edge whole, `From(m, e)`, never a piece: the
one it lies on. Where several of one input's edges lie flush on its line, a
union and a pair boolean cite the `Merged` set of them, read along each
(`work/wire/a-pair-booleans-seam-vertex-cites-the-least-of-several-flush-operand-edges.md`).
A vertex at a member vertex is that vertex. At several members' vertices (a
body glued to its copy) it is `Merged` of them, flat and in name order. A
vertex where a single face crosses or touches a member edge, or a line of
member edges flush on one line, is the `Crossing` of that line and that face.
Every other is the `EdgeCrossing` run of the lines and seams through it (N2).
Each is read from the finished body, and its senses from the members'
pairwise judgements, never from the fold, so every member order spells it
alike. A reference to a member edge, or to a piece of one, that no longer
resolves is offered every set listing that member edge.

**A union's face is named for its PARENT.** Two member faces are linked when
the pairwise judgement of their two members (DM4) merged or covered them:
its two-member union merged the faces or held either face's region through
the other (`BooleanNaming::merge_groups`, `BooleanNaming::substituted`). A pair
the judgement never brought together links nothing. Linking is
transitive. The member faces so linked are one parent, named `Merged` of all of
them, and a member face nothing links is its own parent. A parent is a set of
member-face entities, never a name: the candidates of a tied row are separate
parents that happen to be spelled alike.

A finished face takes the parent of the member faces it descends from; which
faces a fold step merged, kept or discarded does not enter. So a member face
another member's coplanar face covers, and one a later member swallows after
it merged, are cited alike in every member order. A parent the finished body
holds as one face is that face's name. A parent it holds as several faces
qualifies each with one `Fragment(Borders)` over its divider walls, the rule
above, which the subtract reads too. So a face merged and then cut, and a
face cut and then merged, are both `Merged(set)` + `Borders`; a face cut by two
members is one `Borders` over both members' walls, whether one fold step cut
it or two. The fold's spellings are replaced, not refined: which step cut a
face, and whether it met the cut or the merge first, depend on member order,
whereas the parent and its seam partners depend only on the members' consumed
pairs and the finished body.

A two-member union or intersect is the union with one step: its pairwise
judgement is its one fold step, and its table is the union's. The pair
boolean's own table is published only by the subtract, which tells its
operands apart and so links nothing: a tool face lying over a face of `from`
is cited nowhere, and the face is named for the `from` face it descends from.

**Verdicts and ties.** The verdicts run through `k_stats`, so fragment
identity changes only at a recorded flip. An in-band margin refuses
(`NamingError::Escalated`), never a silent pick. An ambient tolerance that
forms no classification band at all refuses (`NamingError::Band`) carrying the
band constructor's own diagnostic: the overflow and the collapse want opposite
repairs, so the refusal says which one it caught. Where nothing covariant
discriminates (congruent candidates, pieces with equal `Borders`, `Keeps` or
`Ends`) the table records one `Entry::Tied` row: naming a tie succeeds,
referencing it is `ResolveError::Ambiguous`. A tie is repaired by a
discriminator in the recipe (a feature whose recorded verdict tells the
candidates apart) or by a `Rebind` to a name that already distinguishes them.
There is no per-candidate choice, because a candidate number is storage order,
not something the author wrote, so a tie the recipe cannot tell apart stays
unreferenceable until it does. Ties propagate downstream as tied (`defer.rs`);
`select_where` filters a tied name all-or-nothing
(`SelectRefusal::TiedDisagrees`), with no per-candidate narrowing.

**N3 — Merge policy: names retire into the merge, loudly.** Coplanar-face
merging (F7) merges faces whose margins decide them one carrier, declared or
not (D10); the merged face is `Merged(sorted, deduped, flat constituents)`,
and a constituent is never itself a bare merged face: whatever mints a `Merged`
mints it flat — a merge of a merged face lists the faces, never the merge — and
a nested `Merged` is an emission bug, refused at the mint and again at the
union's collapse rather than flattened (the fragment carve-out is stated once,
at `RoleSeg::Merged`). A split finishes each half with maximal faces, and
a group that holds section faces is `Merged` of the plane and the operand
faces it absorbed: the plane is a parent linked as an operand face is,
cited once and bare, whatever number of sections the join completed, and
every plane piece of the half is a piece of that parent. A split's group that holds no section
face is named as a boolean names it: `Merged` of the operand faces its faces
descend from, the faces of one such parent in one half told apart by `Keeps`
(N2). A `Merged` whose set has one element is that
element: a face glued to its twin, the same read spelled twice (REFERENCES
DM5), is named as the face. A merged row COVERS a name when the name is a constituent
or is a merged face all of whose faces are (`names/merged.rs`), which is how the
offers and the union's look-through read a flat set. The
constituents retire. In a union no face publishes under a constituent's
name: every piece of a member face a merge links is a piece of its parent
(N2), including a piece that never itself merged. At every boolean, a
pair as much as a union, a seam cites, as each side, the parent of the face
beside it as this boolean's own table names it (a merged face's `Merged`
set; an unmerged face's own name, with the read it came in through), and a
`Borders` wall cites its parent: a seam never descends to a constituent, so
a seam along a merged face is named the same wherever the cut crosses it.
A seam's two sides are in name order, at a pair as at a union: the order
spells a set, and which input a side came from is in the side's own name. A
chord between two merged faces that lies within one input's rim is a piece
of that rim; one that lies within no rim is the seam of the two parents,
as a crossing between them is.
Two edges of one seam between the same two faces are two pieces of that
seam, told apart by their ends (N2), never by citing a retired
constituent. Referencing a constituent fails with the merged name
offered, and when an edit removes the coincidence the merged name vanishes with
its constituents offered. Numeric coplanarity never merges, so merges change
only at recipe edits, structural parameters, or recorded flips; nothing rebinds
itself.

## The name table

**N4 — Eager, per-node, cache-transferable.** Every op names every boundary
entity of every output body from kernel birth data alone (`Extruded`/`Revolved`
maps, `SplitNaming`, `BooleanNaming`, `BlendNaming`, D5 provenance); an unnamed
live entity is `NamingError::Unnamed`, and `NameTable::insert` refuses aliasing
(`SeatingRefusal::Duplicate`), a second name on one entity
(`SeatingRefusal::EntityNamed`) and kind disagreement
(`SeatingRefusal::KindMismatch`). The table is bidirectional (`lookup`,
`name_of`), one per output variable (N1, the scope), read through that
variable alone (`Evaluation::table`), and rides memo hits with the geometry;
consumed entities have no row. The invariant is N0's NS1: the table reads
only what NS1 lists. CI pins one instance of it, that the same recipe and the
same verdicts give an identical table at f64 and at Interval (`tests/m4_pr3_names_ci.rs`, `tests/m4_pr3_names_interval.rs`,
`tests/lib_g16_corpus_name_digests.rs`). The kernel never sees a `StableName`;
hit-testing (`resolve/hit.rs`) reads the table backwards, so the GUI never sees
an arena key.

**A tie's candidates keep their identity.** The node that mints an
`Entry::Tied` row numbers its candidates, and the number belongs to the row: a
tied row holds (candidate, entity) pairs, and a row that narrows to one
candidate (a read of the split half that holds it, a split's
pass-through of the uncut one, a divider that crosses one candidate and leaves
the other whole) is a `Unique` row that keeps its candidate. A divider is a
discriminator among a tie's candidates like any other: the divided candidate's
pieces are named by their `Borders`, the undivided candidates stay under the
bare name, and an edit that moves the divider onto another candidate moves each
name with its role. The pass-through ops of N1 carry the candidate with the name; an op that wraps the
name numbers afresh, as it mints a fresh name. A copy is a pass-through:
each copy is its own output variable with its source's names, and a `⊔`
that gathers copies keys each member's names by the member's read
(`From { read, of }`), so a name reaches the product once per copy by
construction and the gather has no once-per-product rule to keep. The candidate is not part of the name and
reaches no name digest.

**The row is a shared handle.** A table keys on `NameRef` — one `Arc<StableName>`
per row, held by both directions — and a role segment holds its argument name by
the same type, so a downstream name EMBEDS its operand's row rather than copying
the descent below it. Cloning a name costs its own path and nothing deeper. The
public doors (`insert`, `insert_tied`, `lookup`, `name_of`, `iter`) take and
return bare names and share on the way in; their `_ref` twins, which they
delegate to, take the handle a caller already holds, and an emitter reading an
operand uses those.

**The seal is an order cache.** `NameRef` carries one word beside the name.
`NameTable::seal_order` walks a finished table in its own key order and stamps
each row with its POSITION under one fresh epoch; two names of one epoch compare
by position in O(1), and any other pair — two epochs, or either name unstamped —
compares structurally. The answer is identical either way, because the walk that
minted the positions enumerated a structurally-ordered map: the stamp short-cuts
a comparison whose result it reproduces, so no output depends on whether a name
was stamped (D9). Epochs come from a process-wide counter that SATURATES rather
than wrapping — an epoch is never reused, and a seal past that point simply
leaves the table unstamped. A table is sealed the first time it is read as an
operand, by `defer::upstream_name` for a table read one entity at a time and by
`emit_union::member_view`, `emit::name_pattern`, `emit::name_placed_union` and
`emit::name_in_part` for one read whole; a row inserted after a seal is
unstamped and compares structurally, and a clone starts unsealed so what it
gains is stamped on its next operand use.

**The guard the lint exception rests on.** The stamp is interior mutability
inside a map key, which `clippy::mutable_key_type` flags at every map keyed by a
`StableName`; the root `clippy.toml` excepts `NameRef` and states why. What makes
that a receipt rather than a promise is two-sided: `seal_order` asserts under
`debug_assertions` that its walk really is the structural order, so a lying
position cannot be minted silently, and `table.rs`'s unit tests compare the
handle's order against the name's own across sealed, post-seal, cross-epoch,
`Borrow`-probe and deep-chain cases. The check is at the minting site and not in the compare
because `[profile.release]` keeps debug assertions on, and a check inside the
compare would restore the cost the cache exists to remove.

## Resolution

**N5 — Typed resolution failure.** A reference is sited at a read and names a
cell in that read's table, so either coordinate can fail. `ResolveError` is
`ReadGone { name, read, edit }`, `NodeGone { name, edit }`, `Ambiguous { name,
candidates, tie: TieWitness }` or `Vanished { name, diagnosis, last_good:
Option<Tombstone> }`, tried in that order:

- `ReadGone`: the read the reference is sited at is no longer this document's.
  Its operation was deleted, or the read was never this document's; the
  document decides which, with no prior run. The repair is to re-site the
  reference or remove it. Rebind never moves a read, so it cannot repair this.
- `NodeGone`: the name's minting node is gone. The repair is Rebind.
- `Ambiguous` and `Vanished`: the read stands and the minting node lives, but
  the read's table holds more than one cell for the name, or none. Rebind
  repairs both.

A read the name itself carries (`From { read }`) is a fact of the name, not a
site. A read the document holds that is no body output (a scalar, a selection,
a pose) is no question at all: the door refuses it typed (`ResolveRefusal`),
never answers it.

`Diagnosis` is one of:

- `PredicateFlip { predicate, from, to }`, `StructuralParam { node, param }`,
  `RecipeEdit { edit }`: evidence on the name's derivation path;
- `Cascade { through }`: an embedded operand name vanished first;
- `BorderDelta { node, gone, new }`: a face piece's walls changed (below);
- `GroupResized { node, was, now, cutters }`: the fragment's group changed in
  number, and `cutters` names the seams on its parent only one run spells;
- `Upstream { node, cause }`: evidence upstream of the minting node, off the
  derivation path;
- `WitnessBifurcation` (SOLVER-DESIGN W3).

Diagnosis is computable because every node evaluation records its verdict log
(`k_stats`); `resolve/vdiff.rs` diffs two runs per predicate by sign population
(permutation-invariant) and is shared with `SetTolerance`'s ε-audit.

**Two scopes.** The with-history lanes (recorded flips, structural parameters,
recipe edits) read two scopes, in order (`resolve::upstream_nodes` states the
rule): first the reference's path, which is the nodes the name mentions (N1)
and the node that defines the read it is sited at, because that node decides
which table holds the body; this scope answers `PredicateFlip`,
`StructuralParam` or `RecipeEdit`; then the nodes
that were strict ancestors of the minting node in the last-good document or in
the current one, each walked within its own document, minus the path,
answering `Upstream { node, cause }`, a candidate cause that fed the name
without deciding it. A node in neither set is never read.

**The ladder.** When the path is silent, the rungs run in this order:

1. `Cascade`;
2. the qualifier delta: for a face piece, the BORDER delta, the divider walls a
   vanished piece bordered last run and does not now, and the reverse;
3. the upstream scope;
4. the GROUP-SIZE rung;
5. the fallback `Diagnosis::cause_not_in_evidence` = `RecipeEdit {
   NodeChanged(minting node) }`, a site rather than a claim that an edit
   happened. It is reached in particular when the evidence lived on a pair the
   boolean's BVH sweep pruned; results are unaffected, only diagnosis richness
   degrades.

The ladder orders cause before effect: the flips, the qualifier delta, the
doc-diff lanes and `Upstream` name a cause, while the group-size change is an
effect whose cause the evidence does not hold, so it runs after every
cause-naming rung and before the fallback.

**The border delta** (`resolve::border_delta`) reads the walls off the names:
the vanished piece's against those of the current piece of its parent nearest
it, and declines where two are equally near.

**The group-size rung** (`resolve::group_resized`, whose docs say why a
fragment name can vanish with no flip) needs a prior run: when the last-good
table at the minting node carried the name, and the group its emitter divided
the fragment's parent into held `was` entities there and holds `now ≠ was` in
the current run, the diagnosis is `GroupResized { node, was, now, cutters }`.

- *Which edge pieces it meets.* An edge piece's `Ends` holds no count and a
  crossing's sense is its own, so a cut elsewhere on its parent leaves the
  piece's name as it was, and so do the crossings it ends at. A second
  crossing with the same sense by a face that already crosses the line ties
  with the first (N4), so the first's name stays and resolves `Ambiguous`. The
  rung meets an edge piece where its own ends moved or were renamed, or its
  group stopped being divided.
- *What the group is.* The group is the one the emitter formed, read from the
  record it keeps beside the table (`names::FragmentGroups`, not persisted),
  not re-derived from the names. It counts the distinct entities of the node's
  output descended from the parent within the group by the emitter's own
  descent, however each is spelled: an undivided pass-through or an N3
  `Merged` survivor. At a union, faces, seam edges and vertices are grouped
  over the finished body: a face group per parent, a seam group per pair of
  parents, and a vertex group per member corner, holding the corner's copies.
  Any other edge group, and the group of a member corner whose copies keep
  the name a fold step tied them under, counts the published entities a fold
  step's group descends to, followed through every later step. A piece a
  later step re-mints under a seam name of its own is not a descendant by
  that descent and is not counted. An edge piece's group is its line's: every
  edge the node holds on the line from an operand edge, whichever parent edge
  on the line it descends from, whole or in pieces, counted together. Two tied parents that
  share a base are two groups, each counted on its own: the emitter groups by
  parent entity everywhere. That is a statement about two recorded groups, not
  a claimed flip.
- *The cutters.* `cutters` (`GroupCutters`, `resolve::group_cutters`) reads the
  minting node's two tables for the seam rows on the group's parent (a face
  group's seam edges, an edge group's seam and crossing vertices), matched on
  the pair — a `Seam`'s two sides, an `EdgeCrossing`'s lines, a `Crossing`'s edge
  and face — with any `Fragment` tail and never on the row, and names
  every cutter whose seam with the parent only the last-good table spells
  (`gone`) or only the current one does (`new`); both empty says the two
  tables spell seams on the parent with the same cutters. A seam is a crossing,
  not a division: a contact that only trims or notches the parent is listed
  alike, so the field states which seams one run spells, not which cutters
  stopped or started dividing the group. At a union a cutter is compared
  without the fold's `Fragment` tail, which is the fold's and not the member's;
  a renamed cutter reads as one gone and one new; a cutter vertex fused onto
  the parent edge is spelled as a pass-down vertex, not a seam. It names none, and says why, where the
  tables cannot say: a group no seam rows on one parent name bound (a split's,
  a subtract's own seam chain or merged face), tied parents whose shared
  name the seams are spelled on, a prior table that spells no seam on the
  parent, and a seam on the parent spelled in a shape the reading does not
  follow.

**A cited line.** A line is not a row: no table publishes an edge's name with
its piece qualifiers removed. Where a name cites an edge by its line, N5 reads
it as the rows of that node's table whose undivided base it is. The cascade
rung counts a line as present while any row lies on it, so a crossing whose
own verdict flipped reports that flip, and a line no row lies on any longer
cascades. The offers for a vanished edge piece are the surviving pieces of its
line. A union reads a cited line as its least member row on it.

**Offers, tombstones and repair.** A collapsed fragment's undivided base, when
it resolves, rides in the offers for either qualifier kind. `Tombstone` carries
the last-good entry for ghost rendering; selection tools hold name plus
tombstone, never a key. N3's offers ride beside the verbatim error in
`ResolutionFailure::offers`. The automatic rebinding menu is empty: the only
repair is `DocEdit::Rebind { body, from, to }`, recorded once, no alias
table: with a body it rewrites that body's selections naming `from`, and
without one the names no selection holds (an instance's crossing `outer`, an
appearance key).

**N6 — A cell's construction is read from the document.** A recorded
cell is named by the read it entered the deciding operation through and its
name there. The door reads its carrier at that name from the symbolic
evaluation (D10, Coincidence); a placement or a mirror adds no name segment
(N1), so the read, not the name, carries where the cell was placed or
reflected. The kernel carries no recipe provenance of a description.

**N7 — The topology-change sites, exhaustively.** (i) structural parameter
change, (ii) reified predicate flip, N2 discriminators included, (iii) recipe
edit, which stable node ids localize to derivation paths through the edited
node. Within a flip-free, edit-free replay arena-key identity remains the proof
device; everywhere else the name table carries resolution.

## Open

- Out-of-family detection: a failure says the name broke, not that the edit
  left the design family; no membership predicate exists.

