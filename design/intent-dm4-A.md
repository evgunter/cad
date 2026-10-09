# FORK-DM4 — what keys a union member's names

## For Ev

**Recommendation (sure):** a union member's names are keyed by the member's
**read**: the `VarId` of the output variable the member slot holds, the same
id the document stores in `members`. `FromMember { member: VarId, of }`.
Nothing else: not the operation, not `(operation, port)`, and no refusal.
`MembersShareAnOperation` retires, `Union[split.above, split.below]` builds.

**Why the premise is slightly off.** The fork asks what keys a union's
names. What it is evidence of is wider: the document addresses an input by
its read (D10: "reading is the only dependency"; `members: Vec<VarId>`), but
four places still address it by its *operation* — the union's name key, a
declared pair's site (`SitedRef { at: RecipeNodeId }`), the coincidence
walk's entry (`coincide::construction(read: RecipeNodeId, …)`), and the
evaluator's results map, which is why a split's ports have to be "projected"
per operand at all. Each is one defect: an operation stands in for a read,
which is the same thing as naming by position, one level up (port 0 of a
node is the node). The union's key is simply the place it bit first. The
answer is the same at all four: the read. This report settles the first
two and names the other two.

**Ev's words win over the ratified text.** D10 already says it:
"A recorded coincidence names its cells as the reads they entered the
deciding operation through and their names there". That sentence is the
shape `From { read, of }` exactly. DM4's "the key is the edge" (agent text,
DOCM-3, PR 1803; I could not find Ev's words behind it in a shallow
checkout) was written when the edge *was* a node; B (PR 4342) made the edge
a read and moved DM5 with it, and the key did not follow. DM5's
parenthetical ("the union then refuses … `MembersShareAnOperation`") is
PR 4455's own description of its stopgap, as its reviewer noted; no
ratification backs it. Both are what I would change.

### 1. The key

Three candidates, weighed on what a name must survive (N1, N2, N5) and on
one way to say each thing:

| | by operation + refuse (DM5-for-unions) | by `(operation, port)` | **by the read (`VarId`)** |
|---|---|---|---|
| member removed / reordered | survives | survives | survives |
| member re-pointed `split.above → split.below` by `SetMembers` | **key unchanged**: the name says nothing changed, and only the tables' disagreement keeps a frozen selection from re-pointing silently (DM6, N5 forbid this) | key changes, `RecipeEdit` diagnosed | key changes, `RecipeEdit` diagnosed |
| split's plane edited | survives | survives | survives |
| an operation gains an output (D10 gave revolve its axis) | keys unchanged but now ambiguous | unchanged | unchanged |
| what DM5 is over | **not the key**: the mismatch is this whole fork | the key, spelled twice | the key, spelled once |
| two halves of one split | refused; the pair `Boolean` is the second way to rejoin them | admitted | admitted |

The operation-keyed answer keeps a refusal and a workaround alive only
because a ratified sentence says "member id" — the kind of machinery the
D10 conversation set out to remove. `(operation, port)` is `Operand::Output`,
which is *authored* sugar the slot door lowers; a stored name should store
what the document stores. The read is minted identity like a node id (VR1:
same mint chain, never reused, never positional), and B's content key
already feeds the port (PR 4342, fix 2); the name key is the one that
lagged. It is also what FORK-PAT's `Member { (i, …), of }` keys by: the ids
of the variables read. **Sure.**

### 2. One rule for every "came through this input"

A pair `Boolean`'s `FromA`/`FromB` key by *seat*: a second way to say the
same fact. Under seat keys, swapping a subtract's operands vanishes every
name; under read keys the tool's surviving faces are still "from the tool".
DM5 makes the reads of one node distinct, so one segment serves every
operation: **`From { read: VarId, of: NameRef }`**, replacing `FromA`,
`FromB` and `FromMember`. A `Seam`'s two sides are then in canonical name
order in a boolean's table as in a union's (the "a union has no A and B"
special case goes). **Likely.** Consequence worth saying: a two-member
`Union` and a `Boolean(Union)` then publish the same names, which shows
they were two spellings of one node; DM4's "sits beside `Boolean(Union)`,
which stays for a pair" should go, `Union` being the one union. `FromTarget`
(a blend's one input) is the same rule with one read; folding it in too is
uniform, and I would, but nothing hinges on it (**unsure**). `InPart { of }`
stays: it crosses a document, which a read never does.

### 3. Composition: a family member, a split half

- A union reading a family whole: `From { read: xs, of: Member { (k), of } }`.
  The read once, the index once; the key never spells the index, so a
  family's members are told apart by the inner segment, as DM4 says today.
- A union reading `split.above`: `From { read: #above, of: SplitFragment {
  side: Above, parent } }`. The key is an opaque id and spells no half; the
  half is spelled once, in the inner name, where N2 needs it (the side is
  the fragment's covariant discriminator). The `(operation, port)` key is
  the one that would say it twice, and it is rejected above. **Sure.**
- There is a second-order duplication at the split itself, not the union:
  a split's ports *are* its sides (`SplitHalf::output_body`), and once each
  output variable has its own table, `side` on `SplitBody`, `SectionFace`,
  `SectionEdge`, `SplitFragment` and `CrossingVertex` restates which table
  the row is in. The final state I would want drops `side` from those
  segments (a selection "states its body once", D10) and keeps only the
  intra-table qualifiers (`Keeps`, `Ends`). That is a split-naming change,
  separable from this fork, and works with the read key either way.
  **Likely**, offered rather than recommended here.

### 4. What moves, and the special case

Every `FromMember` (and `FromA`/`FromB` under §2) carries a variable id in
place of a node id, so every name digest over a document holding a union or
boolean re-baselines: `lib_g16` corpus digests, `perf2 PINNED`, `m4_pr3`
die table and names digests, `m4_pr4 DIAGNOSIS_DIGEST`, `name_words_corpus
SAID_DIGEST`, `asm2b SINGLE_SOLID_NAMES_DIGEST`, `seat4`; stored documents
whose frozen selections or declared pairs name union rows (`golden.cad`,
`die_tool.pncad`, `die_composed_tour.pncad`, `gallery_ring.pncad`,
`plate_param.pncad`); the Python tag `members_share_an_operation`. Geometry
does not move. A spelling that keeps today's bits ("bare node id when the
operation has one output") is `Operand::Node` sugar inside a stored name:
two spellings of one key, and adding an output port to an operation would
re-key every name through it. **Not worth one. Sure.**

### Clauses changed

- REFERENCES **DM4**: "Naming keys by member" → keys by the read; the
  `FromMember` paragraph; "stays for a pair" (if §2). **DM5**: delete the
  `MembersShareAnOperation` parenthetical. **NAMES N1**: the `RoleSeg` list
  (`From` for `FromA`/`FromB`/`FromMember`). **N6**, **D10**: unchanged;
  they already say the read.
- Sites: `RoleSeg` and its rewriters (`role.rs`), `emit_union::{member_view,
  member_name, keyed}`, `name_union`'s `member_of`, `wire_union` (members
  as reads; the duplicate-operation loop goes), `lift`, `member_site` /
  `site_operand` (the "sided by table" inference goes), `SitedRef { at:
  VarId }` for the declaration channel's life until stage 4 F,
  `coincide::construction` takes the read, `eval/class.rs` and the Python
  tag, tests `dm5_is_over_the_variables_read` (builds, volume 1.0) and
  `a_pair_declared_across_one_splits_halves_is_sided_by_table`.
  Reversible: the key is one field's type.

## For the orchestrator

- **Provenance.** `git log --all -S'key is the edge'` reaches only the
  graft merges in this shallow clone; I take DM4's wording as DOCM-3's
  agent text under self-merge. The DM5 parenthetical is PR 4455's.
- **Root one level down.** The evaluator keys `Results` by node and
  projects split ports per operand (`split_ports_projected`, per-operand
  overlays in `run_op`). A value keyed by the read (port) retires that
  machinery; PR 4455 says `part-split-half-retires` reshapes it. Worth
  stating as that unit's target rather than a reshaping.
- **`coincide::construction(read: RecipeNodeId, …)`** lowers each read to
  its operation (`operation_of`), so the walk cannot tell a split's halves
  apart either. Same defect, same fix; file under INTENT if the unit that
  builds this does not take it.
- **FORK-PAT composition to check**: inside a per-`k` evaluation, a read of
  `xs` is "its member at the same value" (D10), and DM3 says the member
  keeps `Member { (k), of }`; an operation inside the loop then wraps its
  own output in `Member { (k), … }` again, so `k` is spelled at every level
  of a per-index chain. Not this fork's; the read key is the same either
  way. Worth a row before `Member` is built.
- **§3's split `side` drop** is a separate fork if taken; it moves every
  split name and touches N2's text.
- Not checked: whether anything outside editor-core reads
  `FromMember.member` as a node id (viewer, Python). Grep before building.
