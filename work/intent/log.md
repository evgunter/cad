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
