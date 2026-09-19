---
id: doc-param-refusals-keep-two-conventions-inside-one-enum
kind: issue
title: EditError's other doc-param refusals keep the old convention the four param-ref arms left
status: open
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
