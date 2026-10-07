# FORK-3 — What a selection is in the document (designer A)

## For Ev

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
three with arity and order as the slot's own fact, which is where they are
today. A set-valued kind would be a fourth variable kind serving one consumer
family, would still need an ordered twin for the shell, and would make "this
one edge" unshareable with a measure. The fillet's canonical form becomes
"sorted and deduplicated by the selects' definitions", asserted at the door
as today (`SelectionNotCanonical`), so two recipes selecting the same edges
stay bit-identical in their content keys.

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
- **Vertex kind.** `MeasurePrimitive` docs refuse a vertex reference typed, so
  no `Vertex` kind is needed for E; D10's trio stands. If FORK-1 adds one,
  nothing here changes.
- **Canonical form of a `Vec<Edge>` slot** is over the selects' definitions,
  not their ids (mint order). The door check and the content key read through
  `Doc::vars`; `feed_scalar_join` hashes names today, so the key is unchanged.
- **Not checked:** whether `Shell` with an empty `open` is admitted today (no
  refusal found by grep; the variant docs imply a closed cavity is legal).
  If it is refused, the "drop `target`" alternative gains and the lean flips.
- **Ev's verbatim transcripts** (commits 5f7a1c71e3, 3d70e5de72, fae23dbc71,
  c4158a079c) say nothing on selections, tree rows or sharing; the premise
  check rests on that absence and on PR 3990's drafting commit 84404cdbf8.
- No code changed. Corpus selection sizes: fillets of 12, 12 and 42 edges,
  chamfers of 12, one shell face, measures of one or two refs.
