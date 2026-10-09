# FORK-DM4 — what keys a union member's names (designer B)

## For Ev

**Recommendation (likely).** A name is scoped by the variable that holds
its body. An operation that brings entities in from several shape inputs
names each one by **the read it came through**: the variable's minted id,
the same `VarId` the operand stores. A union and a pair boolean then share
one segment, `From { read: VarId, of }`. That replaces `FromMember` and
`FromA`/`FromB`. Since the variable already says which port, an operation
with several outputs stops putting the port in its names: a split's names
lose the half. `MembersShareAnOperation` goes, and `Union[split.0, split.1]`
builds.

**Premise check.** The issue is not specific to unions. It is evidence
that names are scoped by *operation* (one name table per node, entities
addressed by output index inside it), while D10 scopes everything by
*variable*: a selection is "a selection of a `Body` variable by
`StableName`", and a recorded coincidence names "its cells as the reads they
entered the deciding operation through and their names there" (D10,
Coincidence; N6). The union key is one of three places still keyed by
node. The other two are `coincide::NamedCell::Entity { input: RecipeNodeId }`,
which assumes "its output body 0", and the per-node table that makes a
split repeat its half in every role. Ev's own words point the same way:
nodes become "a fancy operation on these variables (one that may produce
many more variables, not just one)". So a member is a variable, and a key
that names the operation and not the variable is naming one level too high.

**1. The union's key.** Four final states:

- **(R) Key by the read's `VarId`.** This is DM4's own argument ("the key
  is the edge") carried to the edge D10 made: the operand *is* a `VarId`.
  It is unique within a union by construction, because DM5 is over
  variables. It survives every edit N5 cares about:
  - a member removed or reordered: the other keys are untouched;
  - the port's operation edited (`SetParam` on the split): the `VarId`
    lives as long as the node, so the key holds and the inner names
    follow the split's own covariance;
  - a re-point by `SetMembers` (split.0 → split.1): that member's names
    strand, as DM6/DM7 say a re-point must, and they are reported.

  It holds no position and no port spelling. *Sure* on the uniqueness and
  survival claims.
- **(O) Keep keying by the operation, and read only the port's rows.**
  This works today for the split, whose one table already gives each half
  distinct names. It is unique only if *every* multi-output operation names
  its outputs apart, and pass-through operations do not. D10's placement
  "reading a list of shapes … defining a copy of each" over `[X, copy(X)]`,
  or over two copies of one body (DM5 admits both, being distinct
  variables), defines two outputs with identical tables (copies keep their
  rows, N1). `Union[P.0, P.1]` then collides again, and the failure would
  move from a refusal to a naming error. Rejected (*likely*: the multi-shape
  `Place` is not built yet, but D10 states it).
- **(D) DM5 over operations, for unions only.** This makes the most natural
  way to rejoin halves unwritable, and keeps the pair `Boolean` as a
  second way to say "union of two" for exactly that case. It also makes
  distinctness mean two things in two node kinds. Rejected (*sure*).
- **(P) Key by `(node, port)` spelled out.** This is a second spelling of
  the variable the operand already stores, and it literally repeats the
  half that the split's names carry. Rejected (*sure*).

**2. The pair boolean.** Under (R), `FromA`/`FromB` are the same rule
spelled by seat. A seat is a named field, not a position, so it is not
wrong in the way depth was. But it is a second way to say "the input this
came through", and nothing in a name needs the seat:
- which operand is the target is read off the node (`b == read`);
- the asymmetry of the pair verb (whose carrier a merge keeps) is geometry,
  not naming;
- `Seam`/`Crossing`/`EdgeCrossing` order their sides by name, as a union's
  already do.

One rule then covers every operation that names entities by an input among
several: **wrap in `From { read, of }`**. A one-input operation (shell,
blend: `FromTarget`) has nothing to tell apart and keeps its marker. There
is one gain beyond tidiness: swapping a commutative boolean's operands no
longer renames anything. *Likely.* Related, but outside this fork: a pair
`Boolean(Union)` beside the n-ary `Union` is two ways to say one thing, and
should retire into `Union`. I suggest filing that as an issue.

**3. Composition, and saying each thing once.** The principle: *within a
variable, a name tells apart only what that variable holds.*
- **A split.** Its two outputs are two variables, so its roles drop the
  half: `SplitBody(half)` becomes a plain body role,
  `SectionFace { side, section }` becomes `SectionFace { section }`, and
  likewise `SectionEdge` and `SplitFragment`. The half is then said once,
  by the read (`From { read: split.0, of: [split, SectionFace{0}] }`), or by
  the body variable a selection states. `node` in N1 stays the minting
  node. The member and the inner minter coincide for a direct read, as
  DM4's do today. That coincidence is not a repetition: a copy of the half
  separates them, which is DM4's reason for keying by the edge.
- **A family.** A family is one variable holding N bodies, so *there* the
  index is a segment: `From { read: xs, of: Member { (k, i), of } }`. The
  family variable appears once and the index once. An indexed read `xs[i]`
  keys by the family variable `xs` (the output it reaches), not by a
  variable of its own. So `Union[xs]` and `Union[xs[0], xs[1], …]` name
  alike, and `Union[xs, xs[0]]` is a duplicate read that DM5 refuses.
- The same rule takes the per-output qualifier off a part instance's
  outputs (stage 2: "qualified by the copy"): each output is its own
  variable. *Unsure*: I did not read that code.

**Smaller alternative (R-only).** Key the union by `VarId` and change
nothing else. The split keeps the half in its roles, but no name spells it
twice in its bytes, because the key is an opaque id. The boolean keeps its
seats. This is defensible and cheaper to build, but it leaves two keying
rules, and leaves per-node tables that the multi-shape `Place` will collide
in. I lean to the full answer above.

**4. What moves.**
- Every union name (node id → `VarId`; the ids differ), every name with a
  boolean in its ancestry (`FromA`/`FromB` → `From`), and every name with
  a split in its ancestry (the half dropped). In practice that is the whole
  corpus.
- What gets re-baselined: the corpus name digests, `perf2`,
  `name_words_corpus` ("the body above" changes its words), the die's
  tables and goldens, and every stored document holding names
  (selections, `Rebind`s, appearance, declared pairs while they last),
  migrated once by a map applied at load.
- One spelling would keep single-output members' bits: the node id when
  the operation has one output, and the `VarId` otherwise. That is two ways
  to say one key. It is not worth having (*sure*); nothing is released.
- Reversible: yes. It is a renaming over one total map.

**Ratified text this changes.**
- REFERENCES DM4 "Naming keys by member": key = the read, with the
  sentence "DM5 makes it unique" kept.
- DM3 "a split half keeps its `SplitBody(half)` names".
- DM5's parenthesis on `MembersShareAnOperation`.
- names README N1 (the boolean group's `FromA`, `FromB`, `FromMember` →
  `From`), N2 (`FromMember(m, e)`, and "in a pair boolean, where an A edge
  and a B edge both hold it, A's is cited" → least in name order), and
  N6's `NamedCell` (read, not input node).

**Sites.**
- `names/role.rs` (`RoleSeg`, `member_edge`, the rewriters);
- `emit_union.rs` (`member_view`: restrict to the read's rows, key by
  `VarId`);
- `emit_topo.rs` (boolean and split roles);
- `eval/wire.rs` (`wire_union`'s members as reads; retire
  `MembersShareAnOperation` with its class, Python tag and pins; rename
  `a_pair_declared_across_one_splits_halves…`);
- `coincide.rs` `NamedCell`;
- `select.rs`/`words.rs` (`SplitBody`);
- `nest*.rs`;
- persist (one load-time migration).

## For the orchestrator

- Assumed: a `Body` variable is always a `VarDef::Output` (D10: "the
  shapes … which only an operation defines"), so "the read's output
  variable" is well defined. `xs[i]`, if DM3 builds it as a defined
  variable, must be keyed through to the family output: state this in the
  unit.
- `NodeValue.name_table` is one table per node, with output index in the
  `EntityRef`. Scoping by variable means one table per output. That is
  sizable machinery, but it is cost, not ranking.
- `coincide::NamedCell::Entity { input: RecipeNodeId }` ("its output body
  0") contradicts N6's "read it entered through" for a split half, today.
  It is the same defect. Either file it, or fold it into the unit.
- Provenance of DM4's "the key is the edge … DM5 makes it unique": my
  `git log -S` runs timed out or hit only merges, so I could not establish
  whether Ev wrote or approved it. Treat it as agent text unless shown
  otherwise.
- Not checked: how instance outputs are qualified today (`eval/parts.rs`).
- Off-question: the pair `Boolean(Union)` duplicating `Union` (D10's "two
  ways to say one thing"). Suggest an issue under INTENT.
