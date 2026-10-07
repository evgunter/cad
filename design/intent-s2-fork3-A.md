# FORK-3 — What a selection is in the document (designer A)

## For Ev — round 3

**The question beneath the fork is the unit of reference: an entity, or a
body's selection.** The two designs each make one property true that the
other cannot, and the two properties cannot both hold. Everything else has
converged, and one earlier recommendation (interning) is withdrawn by both
lines of argument below.

**What each makes true.** *Singletons* (`Edge`/`Face` variables, a blend
reads a list of them): every referenced entity is a variable, so it can be
named ("mouth"), read by a fillet, a measure, a frame or a mate alike, and is
one thing in the panel wherever it appears. *Sets* (`Edges`/`Faces`
variables, a blend reads one): the body a blend or shell acts on is stated
exactly once, so a selection off the acted-on body cannot be written and the
verbs carry no `target`; and the value the GUI and every materializer already
produce, a body's `Vec<StableName>`, is the stored thing. The conservation law
behind the tension: N individually addressable entities are N statements of
a body, and their agreement is a door check, never a representation fact.
Keeping `target` makes it N+1; dropping it for the blends makes it N and
gives the blends and the shell different rules. Only the set makes it one.

**Body-scoped `Rebind` fixes the defect without interning, and interning then
earns nothing.** The defect is that a name denotes one entity per body, and
today's rebind rewrites the name in every body. `Rebind { body, from, to }`
rewrites `from` only in selects of that `Body` variable, where every holder of
`from` is stranded together, so one edit repairs them all, set members and
singletons alike. That is exactly what interning was for. What interning adds
beyond it is a uniqueness invariant, a merge rule, and anonymous variables
that are silently shared between readers, which is the state VR1 exists to
rule out: stage 1 mints two variables for two slots showing `w * 2`, and a
select has no stronger claim to be inferred from its definition than a
formula has. So, under either unit: a selection authored twice is two
variables, the GUI offers the existing one, naming it is how two readers share
one, and `Rebind` is addressed by body and name. The other report's claim that
a per-site selection "has to fall back to rewriting names, which is the
defective form" is not so: it is the body scope, not the address kind, that
the defect turns on. Confidence: sure on the fix, likely on the withdrawal.

**My round-2 claim about sharing was too strong.** "A measure wants the
blend face afterwards, a different body" is true of a measure of the blend,
and false of a reader placed before the fillet on the same body: a frame on
the top face that a later shell opens, or an assertion that an edge is long
enough to fillet, read the same `(body, name)` the set reads. Under sets they
are two selects of one body, repaired together by the body-scoped rebind, and
the one thing lost is that the set's member cannot carry the frame's name.
Under singletons they are one variable. The need is real and occasional.

**Lean.** Body-once, so sets, by a small margin: the mismatch it removes is a
representable wrong document that three verbs would otherwise police at every
door, while what singletons add is a name on a member of a blend's set, which
a user can still read by its spoken entity name. The margin is small because
your own words, "the role played by edges is replaced by sharing variables",
read most literally as singletons, and because sets add a kind family D10
does not list. If you value entity-level sharing over the unrepresentable
mismatch, singletons without `target` on the blends, with `body` kept on the
shell, is the consistent form; keep `target` on all three only if one rule for
the verbs outweighs the extra statement. Confidence on the lean: unsure.
Confidence that the tension is the whole remaining fork: sure.


## For Ev — round 2

**Both designers agree a selection is a variable definition, not a node.** On
the three differences I move to the other report on two and offer a synthesis
on the third.

**1. Set kinds: moved.** A fillet reads one variable of a list kind, `Edges`
(an ordered, duplicate-free list of names, of one body); a shell reads one
`Faces`; `FaceFrame`, `Measure` and a mate side read one `Face`/`Edge`. The
argument that moves me is representability: under singletons the body is
stated once per edge and once more in the verb's `target`, so a fillet of body
X targeting body Y is a document that exists and has to be refused at every
door; and the sealed hollow (an empty `open`, legal by the shell's own
contract) means `target` cannot be dropped, so the duplication stays for every
non-empty shell. A set select carries its body even when empty, so the verbs
lose `target` and the mismatch cannot be written. What singletons bought was
sharing one edge between a fillet and a measure, and that need is narrow: a
fillet consumes its edge, so what a measure wants afterwards is the blend face
of the fillet's output, a different body and a different select. The
one-type rule of SELECT-DESIGN §4 then holds word for word: the GUI's pick set
`(body, Vec<StableName>)` is the authored form of one set select, and a
materializer's `Vec<StableName>` is stored as one definition's names. The
blend's canonical form (sorted, deduplicated) lives in its content-key
preimage, since only the shell reads order. Confidence: likely.

**2. Sharing: moved.** A selection authored at two sites is two variables, and
the GUI offers an existing one of equal definition, as D10 does for a typed
value. My interning argument (no free arm, so equal selects are one function)
was correct about geometry and wrong about authoring: VR1 mints identity and
never infers it from equal values, stage 1 applied that to anonymous
definitions (two slots showing `w * 2` are two variables), and the reason
carries over. An anonymous variable belongs to the site that wrote it;
interning would make it silently shared, so a definition edit at one reader
(`SetSelection { var, names }`, VR7's define door for the reference kinds)
would move another reader with no visible link between them. Coincidence is
unaffected either way: D10 reads a select as its definition, so two equal
selections are one construction. Confidence: likely.

**3. Repair: a synthesis.** The other report keeps `Rebind { from, to }`
addressed by name, rewriting every selection that lists `from`. That inherits
a defect today's rebind has: a name denotes one entity *per body*, and a
boolean that trims edge N into pieces strands N only in its own output, so a
rebind of N to a piece also rewrites a select of the upstream body, where N
still resolved and the piece does not exist. Variable addressing fixes it but
loses the one-edit repair across several selects of one body. The address
that is right is the one a name is scoped to: **`Rebind { body, from, to }`**
rewrites `from` in every select of that `Body` variable, set or singleton,
deduplicating as today; the appearance store and the declared pairs (until
stage 4) keep the name-level rewrite, as they hold bare names. The diagnostic
offer already knows the select's body, so accepting it is this edit.
Confidence: likely.

**What stands from round 1.** Definition over node, for the reasons both
reports give. The premise check: D10's selection sentence is agent-drafted on
PR 3990's draft commit, and your verbatim transcripts (which this checkout
holds) say nothing on selections, so the fork was open. One name on several
entities is `Tied`, refused as `Ambiguous` by N5, so one name is one entity.


## For Ev — round 1

**Recommendation (likely): a selection is a variable definition, not a node.**
`VarDef::Select { body: VarId, name: StableName }` is a third arm beside `Free`,
`Defined(Expr)` and `Output { node, port }`, defining one variable of kind
`Face` or `Edge`. It is to the reference kinds what `Expr` is to the scalar
kinds: the definition language. A fillet's edges are **many variables**, one
per edge, in a `Vec` slot (the general list-slot form the spec already gives
`Union.members` and `Loft.profiles`). A selection authored twice is **one
variable**: the document holds at most one `Select` per `(body, name)`, and
the door that authors a selection returns the existing one. The N5 ladder
runs when the variable is bound, so a failure is diagnosed per select and
reported at each reader. `Rebind` is addressed to the variable.

*Terms.* A **StableName** is a derivation path: the minting node plus the
role the entity plays there. A **select** reads a `Body` variable (a node's
output) and denotes the entity that name has in that body's table; one name
read in two bodies is two selects, since an edge key means nothing without
its body. A **reader** is any slot holding the variable's id.

**Premise check.** D10's sentence "a `Face` or `Edge` variable is a selection
of a `Body` variable by `StableName`" is agent-drafted (the PR 3990 draft
commit), ratified in passing; your own words in that thread say only that
edges are replaced by sharing variables and that a node is "a fancy operation
on these variables". The designers' reports said "selection node". So the
fork is real and open, and neither phrase binds. The fork's framing is right
about the thing: a selection denotes an entity of a body, and the question is
what shape in the document carries that denotation. One hidden premise is
wrong: the fork says SELECT-DESIGN §4 and D10 "state the stored form
differently". They do not conflict once the stored form is a variable whose
definition holds the name: no live query enters a recipe (the materializer
doctrine stands) and the GUI's pick is the *authored* form of a select, as a
`Formula` is of an `Expr` (VR6). §4 is restated, not changed.

**Why a definition and not a node.** A select has nothing a node has and
needs nothing a node offers.

- It makes no value and decides nothing numeric: a name-table lookup is exact
  and there is nothing to memoize. A node's value would be an entity key into
  another node's arena, a new value kind every consumer would have to skip.
- It needs no document-order position, no label and no tree row. Its human
  handle is a variable *name* (VR2): "mouth" is what a person wants to call
  the edge, and a named select is then readable from a fillet, a measure and a
  mate alike. The tree stays the operations; the die's 42-edge fillet adds no
  rows. DM3's `Part` node precedent does not transfer: it chose a node over an
  operand struct replicated in every body-consuming payload, and here the
  alternative is one arm in the one variable table every slot already reads.
- Its failure is a definition's failure: the select binds `Unresolved` with
  the N5 payload and poisons its readers, as a stranded `Defined` would. The
  diagnostic ladder, tombstone and offers key on the variable.
- It binds mid-evaluation, after its body's node runs. That is the same
  machinery stage 2's PR D needs for a definition reading a `Measure` output,
  so selects do not add a second scheduler; they are a second customer of one.

**Why many variables and not one set-valued kind.** The consumers are of
three shapes: a single entity (`FaceFrame`, `Measure`, a mate), an ordered
list (`Shell.open`, whose first face carries the rim) and a set
(`Fillet`/`Chamfer`, canonical sorted). Singletons in `Vec` slots serve all
three with arity and order as the slot's own fact, as today. A set-valued kind
would be a fourth kind serving one consumer family, would still need an
ordered twin for the shell, and would make "this one edge" unshareable with a
measure. The fillet's canonical form becomes "sorted and deduplicated by the
selects' definitions", asserted at the door as today, so two recipes selecting
the same edges stay bit-identical in their content keys.

**Why one variable per `(body, name)`.** A select has no free arm: two selects
with the same definition are the same function of the same variables, and
D10's structural identity already reads them as one. The only act that ever
changes a select is the repair, and a repair says what a *denotation* now is
("the edge that was called N in this body is N′"), which is true for every
reader or for none. Today's `Rebind` already rewrites every site; interning
keeps that meaning and gives it an address. The stage-1 precedent of two
anonymous `w * 2` definitions is not followed because a formula is an authored
object a person edits per slot, and a select is not: changing *which* edge a
fillet blends is a slot edit (set the slot to another select, minted or
found), never a redefinition. The load door checks the uniqueness, as it
checks VR2's names.

**The repair.** `Rebind { var, to }` redefines the select as `(body, to)` and
keeps its identity, so every reader follows. When `(body, to)` already exists
as another variable, the two denote one edge and are merged: the first's
readers read the second and the first is removed, which is exactly today's
"a rebind may shrink a set by one" at the variable level. The name-addressed
`Rebind { from, to }` keeps only the carriers that still hold bare names: the
appearance store, and declared pairs until stage 4. That retires a latent
defect in today's rebind: rewriting a name everywhere re-points a site in a
body where the name still resolves (a boolean that trims N into pieces
strands N only downstream of itself; today's rebind to a piece would strand
the upstream reader).

**The surgery verbs keep their body operand.** A `Fillet` reads `target:
Body` and `selection: Vec<Edge>`; each edge's body must be the target, refused
at the door (`SelectionOffTarget`). The redundancy is a consistency check, not
a second source of truth: a shell may open no face, so its body cannot be
derived from its selections, and one rule for the three verbs beats two. The
alternative (drop `target`, derive the body from the edges) is close and
reversible; I lean to keeping it. `FaceFrame { face }`, measures and mates
drop their body operand, since a singleton select carries its body.

**What the user sees.** Picking an edge authors a select's definition
`(body, name)`; the commit mints the variable or finds it. A fillet's panel
lists its edge slots, each spoken as its variable: by name when it has one,
else as the entity's spoken name in its body. A select is selectable in the
panel like a parameter and shows its readers. Python's
`Node.fillet(target, radius, names)` stays as sugar lowering to one select
per name; `Doc.select(body, name) -> Var` is how one is named and shared.

**Final states compared.** *Node:* one row per select (or a multi-port
select with positional ports over a mutable name list), a label nobody needs,
a memo entry for a lookup. *Set-valued variable:* a new kind per arity shape,
no sharing of one edge between a fillet and a measure. *Distinct variables per
site:* duplicates with no semantic difference, each repaired separately.
*Recommended:* one `VarDef` arm, no kinds beyond `Face`/`Edge`, one variable
per denotation, repair once. Reversibility: interning relaxes to the offer
model by deleting one load check; a node cannot become a definition without
moving every id.

**Confidence.** Definition over node: *likely*. Many singletons over a set
kind: *likely*. One variable per `(body, name)`: *likely*; the merge rule on
collision: *sure* it is needed under interning. Keeping the surgery verbs'
body operand: *unsure* (close call). Selects needing D's mid-evaluation
binding regardless of FORK-5: *sure*.

## For the orchestrator

- **E depends on D's binding machinery whatever FORK-5 rules.** A select is a
  definition bound after a node runs. Spec §5 scopes that machinery to
  "assertions and other measures" if FORK-5 refuses driven geometry; selects
  are a third customer and every geometric reader is downstream of one, so
  the per-lane binding is on E's path either way. Say so in the spec.
- **The spec's `Select { body: VarId, name }` `VarDef` arm and its "one
  entity per variable" rows are the ones taken.** `Rebind`'s address becomes
  a variable; `rebind_payload_names` shrinks to declared pairs and the
  appearance store.
- **The latent name-rebind defect** (rewriting a site where `from` still
  resolves) is today's; not filed since E retires the path. File it if E slips.
- **Vertex kind.** `MeasurePrimitive` refuses a vertex reference typed, so E
  needs no `Vertex` kind; D10's trio stands. If FORK-1 adds one, nothing moves.
- **Canonical form of a `Vec<Edge>` slot** is over the selects' definitions,
  not their ids (mint order). The door check and the content key read through
  `Doc::vars`; `feed_scalar_join` hashes names today, so the key is unchanged.
- **Not checked:** whether `Shell` admits an empty `open` today (no refusal
  found; the docs imply a closed cavity is legal). If refused, the lean flips.
- **Ev's verbatim transcripts** (commits 5f7a1c71e3, 3d70e5de72, fae23dbc71,
  c4158a079c) say nothing on selections, tree rows or sharing; the premise
  check rests on that absence and on PR 3990's drafting commit 84404cdbf8.
- No code changed. Corpus selection sizes: fillets of 12, 12 and 42 edges,
  chamfers of 12, one shell face, measures of one or two refs.
- **Round 2.** The empty-`open` question is settled by `Node::Shell`'s docs
  ("Empty `open` is the sealed hollow"); that is what flips the set-kind
  lean. `Node::rebind_payload_names` rewrites `name == from` at every site
  with no body test, so the latent defect is today's and survives B's version
  verbatim; `Rebind { body, from, to }` is the spec change. Spec rows to take
  from B: `Edges`/`Faces` kinds, `Fillet`/`Chamfer`/`Shell` lose `target`, the
  `Vec<StableName>` sugar lowers to one set select, test 13 holds. FORK-1 may
  reuse the list-kind form for bodies if it wants one; nothing here needs it.
- **Round 3.** Both designers now agree: definition not node; distinct by
  authoring with the GUI offer; `Rebind { body, from, to }`; `SetSelection`
  as the define door. Interning is withdrawn from both of my positions. The
  one fork left for Ev is the unit of reference (sets vs singletons), with
  the `target` question riding on it as stated above. If Ev takes sets, the
  spec's `Select { body, name }` becomes `{ body, names }` with kinds
  `Edges`/`Faces` beside `Edge`/`Face` (one name admitted), and the three
  verbs drop `target`; the blend's content key sorts the names. If Ev takes
  singletons, the spec stands with `Rebind` re-addressed and the blends'
  `target` as Ev rules.
