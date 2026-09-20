---
id: doc-param-refusals-keep-two-conventions-inside-one-enum
kind: issue
title: EditError's other doc-param refusals keep the old convention the four param-ref arms left
status: closed
closed: 2026-09-19
branch: edit/prose-one-home
pr: 2879
opened: 2026-09-19
---

Disclosed by the style review of `edit/param-ref-one-convention` (PR
#2819, finding S1), which put the four param-ref arms onto one
convention and left their neighbours over the same subject on another.

**The convention that now exists.** `EditError`'s own enum doc
(`crates/editor-core/src/edit.rs`) states it once: for a reference to
a document parameter, the ADDRESS leads, the FACT trails, and the
parameter is one noun — `{Slot,Payload}` x
`{UnknownDocParam,DocParamDimension}`, the same four names at the load
door. Guarded by
`display_contract::the_two_doors_spell_the_four_param_ref_refusals_the_same_way_and_each_reports_its_address`
and, on the wire, by
`pncad_py::tests::the_edit_and_snapshot_maps_agree_on_the_four_param_ref_words`.

**What is beside it, in the same enum, over the same subject.** Four
`EditError` arms about a document parameter that do NOT read that way
(`crates/editor-core/src/edit.rs`, tags at
`crates/pncad-py/src/tags.rs`):

| arm | tag word | how it diverges |
| --- | --- | --- |
| `DocParamUnitMismatch` | `doc_param_unit_mismatch` | fact leads (`…Mismatch` trails), no address word |
| `DocParamValueKindMismatch` | `doc_param_value_kind_mismatch` | same |
| `DocParamCountHasNoUnit` | `doc_param_count_has_no_unit` | a sentence, not an `{address}{fact}` pair |
| `DocParamNotDeclared` | `doc_param_not_declared` | the address is a `CarryForwardDoor` payload, not a name word |

And a THIRD spelling of the dimension fact in a third enum:
`EvalError::ParamDimensionMismatch` (`crates/editor-core/src/expr.rs`,
tag `param_dimension_mismatch` at `tags.rs`) — the same "declared
against referenced" disagreement the load and edit doors now call
`{Slot,Payload}DocParamDimension`, at evaluation rather than at a
door.

**Why PR #2819 did not take it.** The row it built
(`param-ref-refusals-spell-two-facts-four-ways`) fences exactly two
facts at two addresses across two doors; these arms are a different
class — every `*Mismatch` across editor-core's refusal enums — and
each rename moves a Python tag word, which is LIB's surface and costs
a binding-census pass of its own.

**The sweep that found the class, and its blind spot.** `grep -rnoE
'\b[A-Z][A-Za-z0-9]*Mismatch\b' crates/editor-core/src/` over the
merge base returns sixteen distinct identifiers. Ten are topology or
persistence invariants with no naming question here (`PinMismatch`,
`OrderMismatch`, `NextPrevMismatch`, `IdMismatch`,
`BackPointerMismatch`, `ParentLoopMismatch`, `EmanatingStartMismatch`,
`EdgeSlotBackpointerMismatch`, `RebindKindMismatch`,
`PlacementRuleMismatch`); `SlotDimensionMismatch`,
`DisplayUnitMismatch` and `DimensionMismatch` are the adjacent
question of whether a dimension refusal names its address. **What the
pattern cannot match** is the other half of the class: an arm that
diverges WITHOUT the word `Mismatch` in it —
`DocParamCountHasNoUnit`, `DocParamCountHasNoDistribution`,
`DocParamNotDeclared`, `ContinuousParamCannotBeCount` are all in the
class and none of them is a `*Mismatch` hit. A unit taking this row
sweeps by the SUBJECT (`DocParam`, `Param`) as well as by the shape.

**What a unit taking this owes.** A ruling on whether the convention
extends past a param REFERENCE to a param DECLARATION refusal, and if
it does, the renames in one PR with the Python tag words, the
committed `TAG_INVENTORY`, `test_binding_census.py`'s name map and
every Python suite that reads one of those words — the same sweep
`edit/param-ref-one-convention` ran. If it does not, the reason
belongs on `EditError`'s enum doc beside the convention, replacing the
pointer at this row that stands there today.

## Ruled and spec'd (2026-09-19, EDIT orchestrator) — E-class, branch `edit/prose-one-home` (shared with `payload-carrier-lists-have-seven-prose-homes`)

**Ruling: the convention's scope is a param REFERENCE, and the enum
says so; nothing is renamed.** `{Slot,Payload}` × `{UnknownDocParam,
DocParamDimension}` names two facts about a reference at an address.
`DocParamUnitMismatch`, `DocParamValueKindMismatch`,
`DocParamCountHasNoUnit`, `DocParamCountHasNoDistribution`,
`DocParamNotDeclared` and `ContinuousParamCannotBeCount` are refusals
about the parameter's DECLARATION (or a carry-forward door's payload),
which has no address word to lead with — the `{address}{fact}` shape
does not apply, and forcing it would mint a fact-only convention with
a vacuous address. `EvalError::ParamDimensionMismatch` is the same
dimension fact raised at EVALUATION, where the address is the
expression path the error carries, not a door's slot/payload word; it
stays. So the change is prose: `EditError`'s enum doc, where the
convention lives, gains one paragraph stating the two families — the
reference refusals (the product) and the declaration refusals (named
by their fact, address-free) — and naming `EvalError`'s arm as the
evaluation-time spelling; the row's sweep-by-subject blind spot is
recorded there as the rule for the next arm. No tag word moves.

**Rows.** None new (a prose ruling). If the lane finds an arm in the
declaration family that DOES carry an address word (a slot or payload
the refusal names), report it in the PR body as the one that would
need the reference convention, without renaming it.

**Territory.** `crates/editor-core/src/edit.rs` (one doc paragraph),
`crates/editor-core/src/expr.rs` only if `ParamDimensionMismatch`'s
doc should point at the paragraph (EDIT). E-class: merges on green CI
and the orchestrator's read; no review lane.

## Built (2026-09-19, `edit/prose-one-home`)

`EditError`'s enum doc states the convention's scope and its two
families; nothing is renamed and no tag word moves.

- The pointer at this row is gone from `edit.rs`, replaced by the
  reason: the eight reference arms name two facts AT an address, and
  the six declaration arms (`DocParamUnitMismatch`,
  `DocParamValueKindMismatch`, `DocParamCountHasNoUnit`,
  `DocParamCountHasNoDistribution`, `ContinuousParamCannotBeCount`,
  `DocParamNotDeclared`) have no address to lead with, so they are
  named by their fact alone.
- The sweep-by-subject blind spot is recorded there as the rule for the
  next arm: which family an arm joins is decided by what it refuses, and
  a sweep by `*Mismatch` misses half the declaration family.
- `EvalError::ParamDimensionMismatch` (`expr.rs`) gains a doc pointing
  at that paragraph.

**Correction to the ruling's premise.** The ruling says
`EvalError::ParamDimensionMismatch`'s address is "the expression path
the error carries". It carries no path: its fields are `name`,
`expected`, `found`. The address at evaluation is the WRAPPER's —
`NodeErrorKind::Expr` (node + slot) and `NodeErrorKind::PayloadExpr`
(node + payload), which forward the refusal unaltered. The conclusion
is unchanged and strengthened: at evaluation the same slot/payload
address distinction exists, one level out, so the fact arm has no
address word to lead with either. Both docs say that instead.

**The declaration-family arm with an address word**: none. All six
carry `ParamName` and facts only; `DocParamNotDeclared`'s
`CarryForwardDoor` says which edit was refused, not where a reference
sits.

## Closed (2026-09-19, EDIT orchestrator)

Built and merged as PR #2879 (E-class, with
`payload-carrier-lists-have-seven-prose-homes`). Nothing renamed and
no tag word moved: `EditError`'s enum doc states the two families —
eight REFERENCE arms naming two facts at an address, six DECLARATION
arms named by their fact alone because a declaration has no address —
and the rule for the next arm (which family it joins is decided by
what it refuses; sweep by SUBJECT as well as by shape). One premise
corrected: `EvalError::ParamDimensionMismatch` carries no expression
path; its address is the wrapper's (`NodeErrorKind::Expr` a node and
a slot, `PayloadExpr` a node and a payload), which forwards the
refusal unaltered — the same slot/payload split one level out, so the
conclusion stands and the arm's doc points at the paragraph. No
declaration-family arm carries an address word.
