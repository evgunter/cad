# FORK-DM4 — what keys a union member's names

## For Ev

**Recommendation (sure):** a member's names are keyed by the member's
**read**: the `VarId` of the output variable the slot holds, the id the
document already stores in `members`. One segment, `From { read: VarId,
of }`, for every operation that names an entity by the input it came
through; `FromMember`, `FromA`, `FromB` retire into it. `Union` and
`Intersect` are lists; `Subtract { from, tool }` is a pair; `Boolean { op,
a, b }` leaves the document. `MembersShareAnOperation` retires and
`Union[split.above, split.below]` builds.

**The premise, widened.** The fork asks what keys a union's names. What it
is evidence of: the document addresses an input by its read (D10, "reading
is the only dependency"; `members: Vec<VarId>`), but four places still
address it by its *operation* — the union's name key, a declared pair's
site (`SitedRef { at: RecipeNodeId }`), the coincidence walk's entry
(`coincide::construction(read: RecipeNodeId, …)`), and the evaluator's
results map, which is why a split's ports are "projected" per operand at
all. An operation standing in for a read is naming by position one level
up (port 0 of a node is the node). The answer is the same at all four:
the read. This report settles the first two and names the other two.

**Ev's words over the ratified text.** D10 already says it: "A recorded
coincidence names its cells as the reads they entered the deciding
operation through and their names there" — the shape `From { read, of }`
exactly. DM4's "the key is the edge" (agent text, DOCM-3, PR 1803; no
words of Ev's behind it that a shallow checkout reaches) was written when
the edge *was* a node; B (PR 4342) made the edge a read and moved DM5 with
it, and the key did not follow. DM5's parenthetical ("the union then
refuses … `MembersShareAnOperation`") is PR 4455's description of its own
stopgap, as its reviewer noted. Both change.

### 1. The key

| what a name must survive | by operation + refuse | by `(operation, port)` | **by the read** |
|---|---|---|---|
| member removed / reordered | survives | survives | survives |
| member re-pointed `split.above → split.below` (`SetMembers`) | **key unchanged**: only the tables' disagreement keeps a frozen selection from re-pointing silently (DM6, N5) | changes, `RecipeEdit` | changes, `RecipeEdit` |
| the split's plane edited | survives | survives | survives |
| an operation gains an output (D10 gave revolve its axis) | ambiguous | unchanged | unchanged |
| what DM5 is over | **not the key**: that mismatch is this fork | the key, spelled twice | the key, once |
| two halves of one split | refused; the pair boolean is a second way to rejoin them | admitted | admitted |

The operation key keeps a refusal and a workaround alive because a
sentence says "member id": machinery the D10 conversation set out to
remove. `(operation, port)` is `Operand::Output`, *authored* sugar the slot
door lowers; a stored name stores what the document stores. The read is
minted identity like a node id (VR1: same mint chain, never reused, never
positional); B's content key already feeds the port (PR 4342, fix 2), so
the name key is the one that lagged; and FORK-PAT's `Member { (i, …), of }`
keys by variable ids too. **Sure.**

### 2. One node, one rule (Ev: "do we want these as different things?")

No. `Boolean(Union){a, b}` and `Union[a, b]` are two spellings of one
node, and the list survives: a two-member list *is* the pair, and under the
read key its names are the pair's (a `Seam`'s sides in name order, which
the union already does). The vocabulary becomes three nodes:

- **`Union { members }` and `Intersect { members }`**, the same shape:
  two or more reads, `SetMembers`, DM4's pairwise judgement before the
  fold, a fold of the kernel's pair verb in list order, `From { read, of }`
  names. Intersect is associative and commutative set-wise, so the result
  is order-free; what the fold keeps is the material in every member, and
  a result face is a piece of one member's face, or a glue of coincident
  faces of several, named by its parent and `Borders` as the union's are;
  a glue keeps the *earliest* member's description, list order being the
  author's statement, as DM4 says for union. Two members whose boxes are
  disjoint make the whole intersect the typed empty at the pairwise
  judgement, naming the pair; an empty fold step is the typed empty too
  (for union it is a kernel bug, `UNION_STEP_EMPTY`). `A ∩ A` is `A`
  (DM5, unchanged). **Sure** on the shape; **likely** that nothing in the
  kernel's intersect lane resists the fold (not traced below the verb).
- **`Subtract { from, tool }`**: binary, two named seats, two reads DM5
  keeps distinct. Several tools are **`subtract(a, union[tools])`**, the
  one way: the tools' mutual contacts have to be judged somewhere, and the
  union judges them once, pairwise, order-free; a list seat on subtract
  would be a second union, cutting in sequence and meeting those contacts
  at the fold with no judgement before it — the bug class DM4 exists to
  prevent. The façade may spell `a - [t1, t2]` and write the union, shown
  as what it is. **Likely.**
- **The one naming rule.** An entity carried from an input into an
  operation's table is `From { read, of }`: the variable it entered through
  and its name there. Which of subtract's seats a read sits in is the
  document's to say, never the name's, so nothing is named by operand
  order (`work/emit/a-pair-boolean-names-a-declared-covered-pair-by-
  operand-order` is this class). `FromTarget` (a blend's one input) is the
  same rule with one read; folding it in is uniform and I would
  (**unsure**). `InPart { of }` stays: it crosses a document.

### 3. Composition: a family member, a split half

- A union reading a family whole: `From { read: xs, of: Member { (k), of } }`.
  The read once, the index once; the key never spells the index.
- A union reading `split.above`: `From { read: #above, of: SplitFragment {
  side: Above, parent } }`. The key is an opaque id and spells no half; the
  half is spelled once, where N2 needs it (the side is the fragment's
  covariant discriminator). `(operation, port)` would say it twice. **Sure.**
- Second-order, at the split itself: its ports *are* its sides
  (`SplitHalf::output_body`), so once each output variable has its own
  table, `side` on `SplitBody`, `SectionFace`, `SectionEdge`,
  `SplitFragment` and `CrossingVertex` restates which table the row is in.
  The final state I would want drops `side` (a selection "states its body
  once", D10), keeping the intra-table qualifiers (`Keeps`, `Ends`). A
  split-naming change, separable, and the read key works either way.
  **Likely**; offered, not recommended here.

### 4. What moves, and the special case

Every `From` segment carries a variable id in place of a node id or a
seat, so every name digest over a document holding a boolean re-baselines
(`lib_g16`, `perf2 PINNED`, `m4_pr3` die digests, `m4_pr4
DIAGNOSIS_DIGEST`, `name_words_corpus SAID_DIGEST`, `asm2b`, `seat4`).
Stored `Boolean` nodes rewrite: `op: Union` to `Union { members: [a, b] }`
(`plate_param.pncad` twice, `die_composed_tour.pncad`,
`pre_b_families.json`), `Subtract` to its seats, `Intersect` to a list;
the variant feeds the node preimage, so ids re-mint and PR 4342's
id-feeding pins move again. Geometry does not move. Tests name
`BooleanOp::Union` in 190 files; demos in `heatsink`, `impeller`,
`teapot`, `checks`. Python: `Node.boolean(op, a, b)` and `BooleanOp`
retire; `Node.union(members)` stays; `Node.intersect(members)` and
`Node.subtract(a, b)` arrive; no `|` spelling exists today, and one added
later writes `union` (`a | b | c` is one list, not a nest). Viewer: the
two-pick Boolean tool becomes union and intersect with N picks (DM4's
"not yet built" door) and subtract with two. A bits-keeping spelling
("bare node id when the operation has one output") is `Operand::Node`
sugar inside a stored name: two spellings of one key, and giving an
operation a second output would re-key every name through it. **Not
worth one. Sure.**

### 5. N, S or F (Ev: binary under the hood with n-ary as sugar; or a stored fold)

**Recommend N. Sure against S; likely against F**, which is N's data under
a word that says the wrong thing, plus one temptation. "Binary under the
hood" is already N: the kernel's pair verb is its only boolean, and the
list node is the one place that folds it, after the pairwise pass.

| | **N** `Union[a,b,c]`, `Intersect[…]`, `Subtract{from,tool}` | **S** binary nodes, n-ary as sugar | **F** `fold(op, [a,b,c])` |
|---|---|---|---|
| stored; one way for a∪b∪c? | the list in the author's order; one | a chain `∪(∪(a,b),c)`: for three members, two shapes and the orders; the sugar picks one and the document shows a tree the author did not write; not one | op and list, N's data; one |
| names | `From { read, of }`, depth one whatever the position; remove or reorder re-keys nothing | DM4's original defect verbatim: `From{u2, From{u1, From{a,…}}}`, depth is position; removing `b` deletes `u1` and re-points `u2`, renaming every downstream frozen selection (the die's pips) | as N |
| #4323's one-pass verdicts, lineage | the pass over the list's pairs; the fold reads one verdict per carrier pair through lineage | each node sees one pair, accumulation against member, so `b`–`c` is judged at `u2` on pieces at their extent, "the main way refusals depended on order after E" (#4323); or the kernel recognises a chain as one union, and then what is stored and what is computed disagree | as N |
| `SetMembers`, delete one | the list without it | no node to set; delete is a splice, the inference DM6 forbids, or a façade rewrite of the chain, with each node's `declare` re-sited until stage 4 F | as N |
| what intersect's glue keeps | the earliest member's description, the list being the author's order (#4323) | the leftmost leaf's, reached through the chain's shape | as N |

**F's own questions.** `fold(∪, [a,b,c])` is `Union[a,b,c]` with the op
as a field; it differs only in what the word claims. The kernel does
accumulate the body in list order, a real fold of the pair verb, but what
it *decides* is over the members (#4323: one verdict per carrier pair
before the fold, names over the finished body, three-member rows by
member cells), and DM4 now says so in those words. Ev's ruling on #4323
was that what is order-dependent be said, not hidden; "fold" says the
opposite, making the accumulation strategy the meaning. N says both: the
list is the author's order, the meaning is the set. So F is honest about
the bits and not about the semantics; calling it a fold is not honest.
`fold(−, [a,b,c])` is `(a−b)−c = a−(b∪c)` set-wise, and its first element
is `from`: a seat named by its position, index 0, the defect class D10
removed, and a list whose order means something under `−` and nothing
under `∪`. What the kernel computes is the same three pairwise verdicts
(`b`–`c` among them, the tools' union in all but name), then two cuts;
`subtract(a, union[tools])` says that and `fold(−)` hides it. A fold over
a family is `union` reading a family with `union` demoted to sugar; one
spelling either way, and #4341's case against `map` (a domain restated at
every step, membership said twice) does not touch a fold, which has no
domain of its own. Names and `SetMembers` are N's. **If the op-as-field
shape appeals, N can be one variant `Combine { op: Union | Intersect,
members }`; not named fold, not admitting `−`.** Likely.

### 6. Round 2, against designer B's report

**1. The split's `side` drop: in this `[ev]` PR (likely).** I move to B.
The per-node table is already wrong on D10's own terms, before any union
is in the picture: a placement "reading a list of shapes … defining a
copy of each" over `[X, copy(X)]` defines two outputs with identical rows
(a copy keeps its rows, N1), which one table cannot hold. So "a name is
scoped by the variable that holds its body" is forced, not optional, and
the half in a split's roles is that principle's one visible consequence
today. Asking Ev for the key now and the principle later would put one
decision in two PRs; the PR ratifies both, the unit that builds the key
may land first, and the per-variable tables (with `SplitBody(half)`,
`SectionFace { side }`, `SectionEdge`, `SplitFragment`, `CrossingVertex`
losing the half) follow as a second unit. One check for that unit: a
piece whose side flips when the plane moves vanishes from its variable's
table rather than renaming, so N5 must still diagnose `PredicateFlip`
from the split's recorded classification, not from the name.

**2. `FromTarget`: fold it into `From` (likely).** A one-input marker is
`From` with its key omitted because derivable, which is the same sugar
§4 rejected inside a stored name (`Operand::Node`), and a second segment
kind meaning "came through an input", told from the first only by arity.
`From { read: target, of }` says the same thing and one more true thing,
and the rewriters, the lift and the words handle one segment.

**3. Two node kinds (likely).** `Union { members }` and `Intersect {
members }`. The node kind is the operation everywhere else in the
vocabulary, and `Boolean { op }`, the one variant with an op field, is
what this fork retires; a field would put two levels of discrimination
on one fact, and the two ops branch anyway (an empty step is a bug for
union and a result for intersect; the copies fast path is union's).
Shared machinery is a function over the verb, not a shared variant. To an
author the words are `union` and `intersect` either way; in the stored
document the kind alone says it.

**B's family rule: disagree (likely).** `xs[i]` is a definition (DM3: "a
definition reading the family and one `Count` expression per index"), so
it is a variable with its own id, and the key is that id, as it is for
every read: the rule is "the read", with no walk to the output it
reaches, which is the collapse this fork removes for operations. Then
`Union[xs]` and `Union[xs[0], …, xs[N−1]]` name differently, and should:
they are two things (one follows `N`, the other is fixed), and naming
them alike would hide a second spelling rather than remove it.
`Union[xs, xs[0]]` is two variables, admitted by DM5 as written, and
builds as DM5 already says of two nodes evaluating to one body (`A ∪ A`
is `A` on that member). B's rule holds only if the unit builds `xs[i]` as
an operand spelling with no id of its own; then the key would have to be
the family and the index, and the unit should say which it built.

### 7. Round 4: a family member's key, and what `copy` is

**Item 4: (i), sure.** The plain rule is "a name is keyed by the read it
came through", a read being the variable the slot holds. Exceptions each
state needs:

- **(i) `xs[i]` is a definition with its own id.** None. A definition is
  a variable (D10: "defined, by an `Expr` over other variables, by a
  selection …"; DM3: "a definition reading the family and one `Count`
  expression per index"), a read is of a variable, DM5 is over variables.
  What looked special is only the rule's output: `Union[xs]` and
  `Union[xs[0], …]` are two documents (one follows `N`), so they name
  apart; `Union[xs, xs[0]]` is two variables, admitted, and builds as DM5
  already says of any two variables holding one body (`A ∪ A` is `A`, the
  rule written for `Part(Instance(0))` beside its master). Nothing is
  added for families. Typing `xs[0]` mints an unnamed definition spoken by
  its one slot, exactly as typing a number mints a free variable.
- **(ii) key by the family plus the index.** Three. The key of a read of a
  definition is the output it *reaches*, a walk the plain rule has
  nowhere else (and the collapse this fork removes for operations). DM5
  must walk too, to refuse `Union[xs, xs[0]]`, so a structural door check
  becomes value-dependent: whether `xs[i]` duplicates `xs[0]` needs `i`
  evaluated, and an index out of range is unresolved at evaluation, not
  decidable at the door. And "duplicate" needs a new relation between a
  family and one of its members, beyond "one variable twice".
- **(iii) `xs[i]` an operand spelling with no id.** Two or three. The slot
  no longer holds a `VarId` (VR4) but a read plus an index expression, so
  the key is a compound and `From { read }` has two shapes. The index is
  then said in the key and again in the inner `Member { (i), of }` (the
  split's `(operation, port)` defect over again), or indexed reads strip
  `Member` and the index lives in the key for one spelling and in the
  inner segment for the other.

(i) has the fewest, zero, and is the only one under which the key, DM5
and VR4 keep their one sentence each. **Sure.**

**Item 3: the case holds, and its motivation is current.** `X'` is a copy
of `X` placed against `X`; under #4326 the copy takes its target's root,
so `X` and `X'` are shapes of one space, and `Place [X, X']` reads two
variables of one space and defines two copies whose rows are identical
(a copy keeps its rows, N1). A name table is keyed by the name alone
(`NameTable::insert` refuses `DuplicateName`), so one table per
operation cannot hold them. That placement is D10 text, not yet built,
so as evidence it is future. The built evidence is the same defect
already compensated: a split's two halves and a pattern's N instances
each sit in one per-node table, and the half (`SplitBody`, `SectionFace
{ side }`, …) and the `Instance { i }` segment exist to keep that one
table's names distinct, restating the output the variable already says.
The split is the case to show Ev; the placement is where the compensation
would stop working.

### 8. Round 5: B's three departures, and (iii) in B's shape

**I move to (iii), likely.** `xs[i]` is a read of the family variable at
`Count` expressions, not a variable; its key is `xs`, the index said once
in `Member { (i), of }`; `Union[xs]` and `Union[xs[0], …]` name alike.

**1. A shape defined by a definition.** True of (i): `xs[i]` would be a
`Body` variable defined by no operation, against D10's "the shapes …
which only an operation defines", and it would be the one shape with a
name table that no operation minted (the member's rows, carried through
a definition). It is a departure, and in D10's own text. DM3's "a
definition reading the family" and D10's "`xs[i, j]` reads one member"
were ratified together and disagree here; D10 governs where a companion
clause disagrees, and its two sentences are consistent with each other
under (iii) only. The final state keeps D10's wording and changes DM3's
to "a read of the family at one `Count` expression per index".

**2. Two spellings of one member read.** Yes, and Ev would see it: inside
a per-`k` reader, `union(xs)` and `union(xs[k])` read the same member,
and under (i) the second keys by a definition's id while the first keys
by `xs`, so one thing gets two names depending on whether the author
wrote the index the evaluation already supplies. Under (iii) both are the
read of `xs` at `k` and name alike. This is the departure in the layer
this fork is about, and it decides it for me.

**3. `Union[xs, xs[0]]`.** Under (i) a syntactic duplicate escapes DM5 by
being wrapped in a definition, and builds `A ∪ A`; the door's check is
dodged by a spelling. Under (iii) it is one member read twice and refuses
at the door, which needs DM5 to say one thing more: a read of a family is
a read of each member. The tail B lists is not optional: `xs[i]` and
`xs[j]` landing on one member at the current values would give two rows
one name (`From { read: xs, of: Member { (v), … } }`), so the evaluation
must refuse it typed, in the class DM3 already keeps there (an index out
of range). State it as DM5's evaluation-time arm, forced by the key.

**My objection to (iii), against B's shape.** Withdrawn but for VR4. B
keys by `xs` alone and the index lives only in `Member`, so there is no
compound key and no doubled index. What remains: a slot holds a read with
its index expressions, not a bare `VarId`, so VR4 changes by one clause
("a read of a family carries one `Count` expression per index") and the
operand machinery (`Operand`, lowering, persistence, `upstream`, the
keys) carries the index. That is cost; the per-`k` implicit read already
is "the family at an index", with the index supplied by the evaluation
instead of the slot, so (iii) gives the explicit spelling the same shape.

### 9. Round 6: one list argument, self-union, and item 3 without `copy`

**1. One argument of kind `Bodies` (sure).** Union and intersect take
one argument, and its kind is `Bodies`, the family of `Body` (D10,
FORK-PAT). There is one kind and two forms of filling the slot, because
D10 has no list literal and a family is the only `Bodies` *value*: the
slot holds one read of a `Bodies` variable (`Union(xs)`), or the member
reads spelled at the slot (`Union([a, b, c])`, the "list of reads" of
S3M, which a placement's shape list already is: the same argument type
in both nodes). A mix is ill-typed at the door (`SlotVarKind`: a `Body`
read beside a `Bodies` read), which retires the family-containment
relation §8 gave DM5. Item 4 keeps (iii), now with less to carry:
`xs[i]` is an indexed read of `xs`, so `Union(xs)` and
`Union([xs[0], xs[1]])` both name `From { read: xs, of: Member { (i),
of } }`, and `Union([a, b, c])` names `From { read: a, of }` and so on.
The spelled form is the slot's, not a value, so no later reader can read
"that list" as a `Bodies`; a reusable family is an index.

**2. Self-union: (c), likely.** Nothing is wrong with `A ∪ A`, and DM5
already says so of two variables holding one body (`Part(Instance(0))`
beside its master builds `A ∪ A = A` by the coincident-shell lane). The
same read twice should be the same case, not a second rule:

- **(a) refuse the duplicate read** (today). Keeps a syntactic refusal
  beside a semantic admission of the identical geometry: the thing Ev
  asked about. Names: none, it refuses.
- **(b) the list is a set.** The door collapses `[A, A]` to `[A]`: a
  silent rewrite of what the author wrote, and still a second rule
  beside the two-variables case, which is not collapsed but glued.
- **(c) duplicates are kept and glue.** `[A, A]` is judged like any pair:
  every cell of `A` meets itself, the same construction read twice, so
  the coincidence is structural and the glue keeps `A`'s description;
  `A ∪ A` and `A ∩ A` are `A`, `A − A` the typed empty. Names: each
  result entity is one entity of `A` glued to itself, one row,
  `From { read: A, of }`; a `Merged` of a row with itself is that row
  (N3's flat set). So one read twice and two variables holding one body
  are one case, answered by the operation, and DM5's door check retires
  (`DuplicateInput` and the validator's arm with it). Two indices
  landing on one member at the current values are the same case, so §8's
  evaluation-time arm retires too. A loft section repeated is a
  degenerate geometry refusal at evaluation, not a door rule.

**3. Item 3 without `copy` (holds, sure).** There is no copy function:
`P1` and `P2` are two placements of `X`, each defining a copy of `X` in
its targets' space, both equally copies of the original. When both are
pinned into one space, a later `Place [P1, P2]` reads two shapes of one
space and defines a copy of each; a placement adds no name segment (N1),
so both outputs carry `X`'s rows verbatim, and one per-node table cannot
hold them (`NameTable::insert` refuses `DuplicateName`). Unbuilt, as
before; the split and the pattern are the built cases of the same
compensation.

**Ratified text that changes.** DM4: `Union { members: Vec<…> }` becomes
one `Bodies` argument in the two forms above, for union and intersect;
"DM5 makes it unique" goes. DM5: the door check and `DuplicateInput`
retire; the clause becomes "a read repeated is answered by the
operation" with its three answers. D10 Repetition and FORK-PAT: "`union`
and `subtract` read a family as their members" becomes "`union` and
`intersect` take a `Bodies`: a family, or its members spelled as reads;
a subtract's tool is one `Body`". D10 Variables' `Bodies` ("an ordered
list of bodies whose length is a `Count`") already fits: the spelled
form's length is a literal count. DM3 as §8.

### Clauses changed

- REFERENCES **DM4**: "Naming keys by member" → by the read; "It sits
  beside `Boolean(Union)`, which stays for a pair" deleted; "an n-ary
  union" → union and intersect; "nothing else in the vocabulary is a
  list" → intersect too. **DM5**: `Boolean { a: X, b: X }` → `Subtract {
  from: X, tool: X }`; the `MembersShareAnOperation` parenthetical deleted.
  **D10 Repetition**: "`union` and `subtract` read a family as their
  members" → union and intersect do; a subtract's tool is one body, so
  cutting a family is `subtract(a, union(holes))`. **NAMES N1**: the
  `RoleSeg` list (`From` for `FromA`/`FromB`/`FromMember`). **N6**: unchanged.
- Sites: `RoleSeg` and its rewriters, `emit_union::{member_view,
  member_name, keyed}`, `name_union`'s `member_of`, `wire_union` (the
  duplicate-operation loop goes; an op parameter for intersect),
  `wire_boolean` → subtract only, `lift`, `member_site` / `site_operand`
  (the "sided by table" inference goes), `SitedRef { at: VarId }` until
  stage 4 F retires declarations, `coincide::construction` takes the
  read, `eval/class.rs`, the Python tags and stub, the viewer's combine
  tool, tests `dm5_is_over_the_variables_read` (builds, volume 1.0) and
  `a_pair_declared_across_one_splits_halves_is_sided_by_table`.
  Reversible: a field's type and a variant split.

## For the orchestrator

- **Provenance.** `git log --all -S'key is the edge'` reaches only the
  graft merges here; I take DM4's wording as DOCM-3's agent text under
  self-merge. The DM5 parenthetical is PR 4455's.
- **Root one level down.** `Results` is keyed by node and split ports are
  projected per operand (`split_ports_projected`, the overlays in
  `run_op`). A value keyed by the read retires that; PR 4455 says
  `part-split-half-retires` reshapes it. State it as that unit's target.
- **`coincide::construction`** lowers each read to its operation
  (`operation_of`), so the walk cannot tell a split's halves apart either.
  Same defect, same fix; file under INTENT if the building unit skips it.
- **Not traced:** the kernel's intersect lane under a fold (whether
  `BooleanNaming` and the flush rows behave for `Intersect` as for
  `Union` at every step); the reviewer of the building unit should run a
  three-member intersect with two coincident faces.
- **FORK-PAT composition to check**: inside a per-`k` evaluation a read
  of `xs` is "its member at the same value" (D10) and keeps `Member { (k),
  of }` (DM3); an operation in the loop wraps its output in `Member { (k),
  … }` again, so `k` is spelled at every level. Not this fork's.
- **§3's split `side` drop** is a separate fork if taken (every split
  name moves; N2's text).
- Not checked: readers of `FromMember.member` as a node id outside
  editor-core (viewer, Python). Grep before building.
