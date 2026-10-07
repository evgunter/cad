# INTENT — log

## 2026-10-03 — opened

Chartered by the session that ran the D10 ruling (PR 3990), to build
it. Opening slate: stage 1 (variables). The representation row goes to
a designer pair before any lane builds; the build, derived parameters
and the GUI row are parked on it.

Moved here from RECIPE: `d10-one-way-to-say-intent-is-unbuilt` (the
hold's trigger; its id is unchanged, so every parked row still
resolves), `equal-literals-lower-to-one-identity-token` and
`mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion`.

## 2026-10-03 — the representation is settled (VARIABLES-DESIGN VR1–VR9)

A designer pair weighed `variables-are-identities-with-labels`
(blinding: `analysis/design-fork/intent-variables-2026-10-03`). They
converged on everything but where a written quantity becomes a
variable; the orchestrator took the edit-door lowering of an authored
`Formula` (the type then cannot hold a literal) and folded in the other
report's findings. An elaboration of D10 that changes no ratified text,
so it did not go to Ev; the design is `docs/VARIABLES-DESIGN.md`, with
a companion-table row. Stage 1 is re-sliced: the build starts with
`variables-replace-the-parameter-table`; the literal retirement, derived
variables and the GUI row are parked behind it.

## 2026-10-03 — VARS PR 1: the renames (orchestrator's read)

`docs/INTENT-VARS-1-SPEC.md` sliced `variables-replace-the-parameter-table`
into three PRs, each green. PR 1 renames `ParamName`→`VarName`,
`DocParam`→`FreeVar`, `DocParamValue`→`FreeValue` and the name-fault
types, with no behaviour or wire change (golden fixtures untouched).
Review tier: the orchestrator's read — a mechanical rename whose only
risk, the wire, is pinned by the golden tests. The Python classes keep
their names until PR 3 reshapes that surface (the binding census now
argues the three types by `BOUND_AS`). PRs 2 and 3 are H and draw a
concurrent dual.

## 2026-10-04 — VARS PR 2 to review (dual, concurrent)

PR 4011, frozen head `42e2330ddf`. Pre-draw fields: difficulty **H**,
task class **STRUCTURAL**; arm CONCURRENT (rule 1). Both reviewers run
in their own cloud sessions, since this container's disk cannot hold
two more build directories; the method is identical for both. Rulings
made at hand-off: the Monte-Carlo sheets move because draws follow
`VarId` order (by design: name order would let a rename move them) and
are re-rendered; the symbolic tier's two lost registrations on
r2_filleted_bracket are a claim for the reviewers, not an accepted
cost.

## 2026-10-04 — VARS PR 2 merged (PR 4011)

Both reviews delivered at frozen head `42e2330ddf`. Each ran in its own
cloud session from one brief (sha256 prefixes `ed0ba53e17eaf67c` and
`fe6b38288dbaa08d`, identical modulo the lane label). R1 returned
APPROVE-WITH-FIXES. R2 returned NOT-MERGEABLE-AS-IS: four editor-core
rows were red at the head, all in the slow set the PR gate skips, and
three of them came from the declare preimage holding the definition.
Rulings at adjudication:

- The declare preimage is the variable's kind only. A value, a law or
  a display unit is not identity, so two documents differing only in a
  nominal keep one id chain.
- Variables are ordered by declaration (`Doc.var_order`, saved and
  checked at load), never by id. An id is digest output and gives an
  order no author can see.
- Load refuses an unnamed variable until PR 3's walk 6 relaxes it to
  "anonymous ⇒ read".

The fix pass was large and run by the implementer lane. It also
re-blessed the sym_9 bracket's pins (154/160): the tier's reach depends
on symbol order, filed as `work/sym/sym-tier-reach-depends-on-symbol-order.md`.
The row is in `docs/DUAL-REVIEW-LOG.md`. Next is PR 3: readers read ids.
## 2026-10-04 — open PRs triaged against D10

At Ev's request a lane read every open PR against D10. Closed as fully
superseded: #3929 (`[ev]`, a measured part stays a product root: the
consume/read typing is moot once nothing consumes). Its row
`a-measured-part-is-not-a-product-root` moved here from RECIPE, parked on
the build (REACH's plate row still blocks on it by id); its one unfiled
finding is `a-failed-requirement-refuses-the-whole-product`. Partly
superseded and left open: FUSE's #3955 (note on FUSE's log). 22 others
are not superseded.

## 2026-10-04 — VARS PR 3 merged (PR 4027): readers read ids

The implementer ran in its own cloud session, and the PR went to a dual
review at frozen head `b07891634f`. Each reviewer ran in its own cloud
session from one brief, identical modulo the lane label (sha256 prefixes
`fe4eec8319d1d0a0` for r1 and `78488713d2fd5f35` for r2). Both returned
APPROVE-WITH-FIXES with no MAJOR, so the tally is 0.

The union and the rulings made at adjudication:

- A variable is spoken from the document wherever one is at hand,
  including a refusal re-spoken after a rename and an analysis box taken
  before one. Memoized pattern refusals hold their formula as an `Expr`;
  the `#hex` text scan is gone.
- An unnamed variable has one spelling, `#<16 hex>`.
- Split or inline across an unresolved reader refuses typed
  (`UnresolvedVarCrossesCut`).
- The door checks a post-condition: no stored node holds a `Name` leaf
  (`NameLeafWritten`).
- The id-free corpus digest is a committed guard.
- One expression walker and one lowering rule.
- Every surviving mutant has a row.

The viewer's "param" vocabulary keeps its names until it is moved in one
piece (`viewer-param-vocabulary-names-a-variable`). Two more items were
filed: the Python classes, and `range.rs`'s synthetic name. An
independent verifier session checked the fix pass: 8 of 9 mutants red and every probe held; the one survivor, the door post-condition, got its row in a second pass.

`variables-replace-the-parameter-table` is closed. The spec is in
`docs/doc-ledger/intent-vars-1-spec.md`. `no-dimensioned-literal-in-a-slot`
and `parameters-defined-by-formulas` are unparked; the literal retirement
is next.

## 2026-10-05 — the literal retirement's spec (`docs/INTENT-LITERALS-SPEC.md`)

A spec writer read the merged code and sized every literal site. VR4
cannot land before a slot can hold an anonymous *defined* variable, so
`parameters-defined-by-formulas` merges into this unit as its first PR.
The unit lands in four PRs, each green:

- A: definitions.
- B: authored `Formula` and stored `Expr`, with byte-identical goldens.
- C: a slot holds a `VarId`.
- D: `Expr` holds no float.

All nine of the spec's recommendations were accepted (§11). Two were
written into VARIABLES-DESIGN:

- A lone number at a slot's root is a typed value (VR6).
- A `Measure`'s arithmetic stays a formula in the node until stage 2
  (VR4).

Both elaborate D10 and change no ratified text. The spec found that C
makes separately typed equal radii lose their declared evidence; a
fixture that relied on it shares one variable instead. Ev has said to
proceed through the plan without asking.

## 2026-10-05 — INTENT-LITERALS PR A, definitions (`intent/literals-a`)

A variable may be defined by an `Expr` over other variables
(`VarDef::Defined`), authored as `VarDecl` and lowered at the edit door.
The doors refuse a definition cycle, a read the document does not
answer, and an expansion past `DEFINITION_NODE_BOUND` (4096 nodes); the
load door refuses the same in two new walks. The environments bind
definitions over their inputs in definition order, so a defined
variable carries its inputs' enclosure, tangent or symbol; the seed door
refuses one. Coincidence tokens and content keys expand definitions,
`DocDiff::vars` closes over them, and the anonymous lifecycle cascades
through them. The viewer's parameter text door defines a variable, and
retires `Refusal::ParamNotANumber`.

Filed: `viewer-value-doors-read-a-defined-variable-as-absent`.

## 2026-10-05 — INTENT-LITERALS PR B, authored Formula and stored Expr (`intent/literals-b`, PR 4072)

One tree, `ExprTree<L>`, serves two forms: the stored `Expr` reads
every variable by id, and the authored `Formula` may also write a name.
Nodes, programs, placements and measures are generic over the slot
form; a `DocEdit` carries the authored form, and every door lowers it
before anything else reads it, refusing an unlowerable name in the
existing vocabulary. The type now carries what `NameLeafWritten` and
load walk 4 (`NamedReaderInSnapshot`, with its definition twin)
checked, so all three are deleted. `parse_expr` is `parse_formula`.
Python's `Expr` class is `Formula`, a stored definition reads back as a
read-only `Expr`, and `ParamName`, `DocParam` and `DocParamValue` are
`VarName`, `FreeVar` and `FreeValue`. Goldens, corpus files and digests
are byte-identical.

Ruled by the lane: an edit refused for two faults at once may now name
its unlowerable name first (InsertNode, SetProgram, SetOffset; SetParam
and SetExpression keep their order); a name read at another kind is
`var_kind_mismatch` in Python rather than `unlowered_name`;
`GeomPred.datum_distance`'s comparand stays a stored `Expr`, so a
written name refuses where the predicate is built; the error tag words
(`unknown_param`, …) keep their spelling.

Closed: `python-param-classes-name-a-variable`.
### PR A review pass (both reviewers APPROVE-WITH-FIXES, no MAJOR)

- The viewer's text door folds constant text, continuous or count, to a
  value (`SetVarValue`), so a toleranced parameter keeps its tolerance.
  Only text that reads a variable defines one. A value typed over a
  definition frees the variable and writes through the value door in one
  action, so `3` over a length refuses `VarValueKindMismatch` whether the
  variable is free or defined.
- A defined parameter's row has the free row's field, showing its
  formula. A formula typed into it redefines the variable; a number
  frees it.
- The order memo is gone. `definition_order` is Kahn's algorithm over
  one adjacency, and the other definition walks read the same index.
- The edit and load doors ask one `Doc::expansion_fault`, and the
  definition sentences render once for both.
- `Doc.definitions` lists the named defined variables in Python.
- Each mutant the review found surviving has a row, and there are MC and
  symbolic-tier rows.

Filed: `definition-node-bound-is-re-measured-against-the-corpus-after-d`.

## 2026-10-06 — INTENT-LITERALS PR C, a slot holds a VarId (`intent/literals-c`)

A stored slot holds the id of the variable it reads. The edit door
lowers each authored slot in one walk that returns every fault at its
address: a lone variable, name or fresh entry is itself, a value mints
an anonymous free variable, and a formula mints an anonymous defined
one. `Node::authored` re-authors a node by reading its variables, and
`Node::written` gives the formulas the slots were written as. Equal
typed values are two variables, so their tokens and content keys
differ; sharing is said by reading one variable.

Ev revised Q3 and Q4 mid-PR (spec §11, VARIABLES-DESIGN VR8): only a
toleranced variable is an analysis axis. An untoleranced one is a
constant in every lane, and Sym binds it as its exact nominal. Measured
against B's head: the interval suite's slowest five are unchanged
(427/209/115/95/87 s against 422/213/114/90/87 s), the stackups on
`tolerance`, `mcplate` and `chaintol` list the same three rows, and the
Monte Carlo sheet is bit-identical. Sym theorems rise by one per written
formula (`work/sym/symbolic-reach-at-slot-variables-c.md`).

Ruled by the lane:
- An anonymous variable's mint reads what it holds, so sibling inserts
  that differ only in a typed value still mint different nodes.
- An anonymous definition's refusal is reported as the slot's own
  (`VarEnv::written`), not as a refusal of `#id`.
- Names refuse before dimensions, B's order.
- The path editor loads and compares the program as written, so a value
  moved since the load refuses `ProfileEditStale`.
- Retiring anonymous variables is no loss the person is shown.

Closed: `range-synthetic-name-mints-a-name`,
`symbolic-reach-at-slot-variables-c`. Filed:
`unproven-coincidence-lint-binds-every-variable-as-a-symbol`.
- 2026-10-06 — Note from ZIP: filed `the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms` (P0/H) on this slate as stage-4 input, on Ev's direction in chat. Two designers converged on retiring the declared-REST zip (`boolean/rest.rs`) with declared pairs. Before then the join gains a partner-edge chord, ring re-homing on a curved chart in aligned contact, and the `mekr` `NotSameFace` cause. The row carries the measurement. ZIP's REST-lane rows are parked on `d10-one-way-to-say-intent-is-unbuilt`. Units ZIP already started finish: the zip's admission check (PR 4127, a live wrong body; Ev, in chat, "finish it"), `Fusions` (PR 4116) and pins (PR 4130). (ZIP orchestrator)

## 2026-10-07 — INTENT-LITERALS PR D, Expr holds no float (`intent/literals-d`)

A stored expression's leaves are variable readers, exact rationals
(`Ratio`, reduced, numerator and denominator at most 2^53), integers
and `turn`; no float is left in a document's expressions. A written
quantity is the authored `Quantity` leaf: inside a formula it mints an
anonymous free variable of its own, in pre-order before the variable it
defines, so two typed `5 mm` in formulas are two variables and their
tokens differ. Tokens carry `T_RATIO`, `T_INTEGER` and `T_TURN`;
`T_LITERAL` and `T_COUNT_LITERAL` are retired bytes. The f64 geometry,
and the interval enclosures, of the whole corpus are byte-identical
with ids masked; the id-bearing pins moved on `kitchen_sink` alone.

Ruled by the lane:
- A dimensionless value has no notation to remember, so it is the bare
  number its text is: `Formula::literal` at `Scalar` (and
  `Formula::scalar`, `Formula::number`) is the exact constant its
  shortest decimal spells where one in range keeps its bits, and a
  written value otherwise (`0.30000000000000004`, `-0.0`). The text
  door reads digits the same way, so `unparse` round-trips with no
  exception. A distribution on a lone number makes it the written value.
- A bare integer beside an operand that is no count reads as the scalar
  it equals (`turn/4`, `w * 2`); `INT/INT` with no space is one ratio,
  the text `unparse` writes a non-terminating constant as.
- A declared definition mints its own id before its quantities, so a
  refusal speaks the id it is minted at.
- `Doc::unparse` writes an anonymous reader as what it holds; the bare
  `unparse` keeps `#<16 hex>`.
- `GeomPred::DatumDistance` holds a `Formula`, evaluated with no
  document: a name refuses `EvalError::Unlowered`.

Closed: `no-dimensioned-literal-in-a-slot`,
`equal-literals-lower-to-one-identity-token`,
`definition-node-bound-is-re-measured-against-the-corpus-after-d`
(4096 stands). Opened: `typing-a-value-mints-or-offers-a-variable`
(its trigger fired).
