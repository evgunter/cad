# FORK-DM4 — what keys a union member's names (designer B)

## For Ev

**Recommendation (likely).** A name is scoped by the variable that holds
its body. An operation that brings entities in from several shape inputs
names each one by **the read it came through**: the variable's minted id,
the same `VarId` the operand stores. Union and intersect become n-ary
nodes, and subtract stays the one binary node (option N in section 2;
binary-only nodes with n-ary sugar are rejected). All three share one
segment, `From { read: VarId, of }`, which replaces `FromMember` and
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

**2. One union, n-ary union and intersect, binary subtract (Ev's
addenda).** Recommendation: **(N)**, *likely to sure*.

- **(N)**, recommended. The document stores `Union { members }`,
  `Intersect { members }` (each two or more reads) and `Subtract { a, b }`.
  `Boolean { op }` and `BooleanOp` retire.
- **(S)**, rejected. The document stores only binary nodes, and the
  façade/GUI writes the fold.

For each question:
- **A union of three.** (N): one node, `Union[a, b, c]`. (S): many chains
  (`(a∪b)∪c`, `a∪(b∪c)`, and every order), so many ways to say one thing,
  and the GUI's "n-ary union" is only recognised from the program, as
  D10's pattern presets are.
- **Names.** (N): `From { read, of }` once, whatever the count. (S): the
  rule is still `From { read, of }`, but a chain nests it per link:
  `From { read: ab, of: From { read: a, of } }`. Taking `b` out means
  re-pointing the outer node to `a`, which strands every name that came
  through the removed link. That is exactly the die's failure that DM4
  exists to remove. Keying by read does not cure depth; only a flat node
  does.
- **#4323's pairwise verdicts.** (N): kept as ratified. Each member pair
  is judged once before the fold, and the fold reads those verdicts, so
  the result is the same in every order. (S): each link judges only its
  two operands, so `a` and `c` meet only through `a∪b`, after `b` may have
  covered or cut the contact. The verdicts become order-dependent, which
  #4323 ruled out.
- **`SetMembers` and deleting a member.** (N): one edit naming the new
  list; the other members' names are untouched. (S): the sugar must
  rewrite a chain of nodes, and every link above the removed one renames.
- **What a glue keeps.** For union and intersect alike, names are defined
  over the finished body, through the member-face parents (N2), so they
  are order-free. A glued face keeps the *description* of the earlier
  member in the list, as #4323 ruled. Ev kept list order there because
  a mint-order sort "hides arbitrariness". So `[a, b]` and `[b, a]` are
  not two spellings of one thing: they differ in one authored choice,
  whose carrier a glue keeps, and nothing else. (My first draft proposed
  minted-first; I withdraw it.)
- **Intersect specifically.** Intersect commutes, so the same pairwise
  judgement and parent naming apply unchanged. One difference: an empty
  fold step is a legitimate result (the typed empty body), not the kernel
  bug it is for a union.
- **Subtract.** It stays binary, because difference does not commute or
  associate and nothing is a unary "not". Several tools is
  `subtract(a, union[tools])`, the one way to say it. A list seat would
  duplicate the union's pairwise machinery among the tools. A single tool
  is `subtract(a, t)`. To keep one way, D10 Repetition's "`union` and
  `subtract` read a family as their members" narrows: subtract's `b` seat
  holds one `Body`, so a family of tools is `subtract(a, union[holes])`.
  This changes D10's text.

**(F), a stored fold `fold(op, members)` over a binary op.** Rejected
against (N), *likely*.
- **Union.** `fold(∪, [a, b, c])` is `Union[a, b, c]` with the op as a
  parameter. It is not a fold: #4323's verdicts are pairwise over the
  members, and the names and the result are defined over the member set
  and the finished body. They hold only because ∪ and ∩ commute, so the
  kernel does not evaluate left to right as written. The list order
  decides only whose description a glue keeps. Calling it a fold would
  promise a sequence the node does not compute.
- **Subtract.** `fold(−, [a, b, c])` really is sequential: `(a − b) − c`,
  with an intermediate body per step and names nesting per step. It also
  says "a is what is cut" by position, the first slot of a list: the
  declare-by-position defect D10 set out to remove. Its tools commute, but
  the list states an order anyway. As point sets it equals
  `a − (b ∪ c)`, which `subtract(a, union[tools])` says with no position:
  the target in a named seat, and the tools as a set in a union whose
  order decides only descriptions. That keeps
  `subtract(a, union[tools])` as the one way. Its cost: tool–tool
  coincidences are judged even where they lie outside `a`, so a pair of
  tools in the sliver band refuses even though it never touches the cut
  (rare; D10 makes the rest of such contacts lint findings).
- **Index variables.** FORK-PAT chose index variables over a `map`
  combinator, and `union` reading a family is already the reduction over
  the index. `fold(∪, xs)` would be a second way to say that, and a
  combinator the same ruling declined. So (F) brings back what FORK-PAT
  removed.
- **Names and `SetMembers`.** For ∪ and ∩ they are exactly (N)'s. For −,
  they are either a per-step nesting (a removal re-keys the steps after
  it) or a flat `From` over a list whose first entry is special.
- **What survives of (F).** The op as a parameter. One node
  `Combine { op: Union | Intersect, members }` in place of two node kinds
  is (F) restricted to the ops that commute, with no claim to fold. It is
  as good as two kinds, since they share every line of machinery
  (*unsure* which reads better; either is one way).

**The naming rule across all three.** An operation that takes in entities
from several shape inputs wraps each in `From { read: VarId, of }`. That
covers union members, intersect members, and subtract's `a` and `b`.
Subtract's seats are not in names:
- which read is the tool is read off the node;
- the pair verb's asymmetry is geometry;
- `Seam`, `Crossing` and `EdgeCrossing` order their sides by name, as the
  union's already do.

A one-input operation (shell, blend: `FromTarget`) has nothing to tell
apart and keeps its marker. The DM4 sentence "It sits beside
`Boolean(Union)`, which stays for a pair" becomes: "A union of two is
this node; intersect is the same node over intersection; subtract is the
one pair boolean."

**Migration.**
- Each `Boolean { Union, a, b }` becomes `Union [a, b]`, `Intersect`
  likewise, and `Subtract` becomes `Subtract { a, b }`. The conversion is
  literal: a chain stays a chain, because flattening is an edit that only
  the author or the façade may make (DM6).
- Names map `FromA`/`FromB`/`FromMember` → `From`, by one total map at
  load. The goldens are re-baselined with the rest of section 4.
- Façade: `a | b` writes `union([a, b])`, `a & b` writes
  `intersect([a, b])`, and `a - b` writes `subtract`. A `|` whose left
  operand is a union the same expression just wrote, and that nothing
  else reads, appends to it, so `a | b | c` is one node. `union(*bodies)`
  is the explicit spelling.

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
  sentence "DM5 makes it unique" kept; DM4's "sits beside
  `Boolean(Union)`" (above); D10 Repetition's "and `subtract`".
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
- Folded in Ev's addenda: n-ary union and intersect, binary subtract,
  (N) vs (S), and (F) the stored fold. Withdrew the minted-first glue rule
  in favour of #4323's list order.
