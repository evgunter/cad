---
id: param-ref-refusals-spell-two-facts-four-ways
kind: issue
title: The param-table rule's two facts are spelled four ways across eight refusal arms
status: open
opened: 2026-09-17
---
Disclosed by the style review of `edit/load-door-payload-refs` (PR
#2793), which asked why one rule with two answers needs eight names.

**The rule** is `Doc::param_ref_fault` (`doc.rs`), which answers with
`ParamRefFault::Unknown` or `ParamRefFault::Dimension` — TWO facts. It
has four callers (the edit door's slot and payload walks, the load
door's two), and each caller mints its own pair of names, so the two
facts are spelled EIGHT ways under FOUR conventions:

| door | address | "undeclared" | "wrong dimension" |
| --- | --- | --- | --- |
| edit | slot | `EditError::UnknownDocParam` | `DocParamDimensionMismatch` |
| edit | payload | `EditError::UnknownPayloadParam` | `PayloadParamDimensionMismatch` |
| load | slot | `SnapshotError::SlotUnknownDocParam` | `SlotDocParamDimension` |
| load | payload | `SnapshotError::PayloadUnknownDocParam` | `PayloadDocParamDimension` |

Four conventions for the same two facts: the address leads at the load
door and trails at the edit door (`SlotUnknownDocParam` against
`UnknownDocParam`); the dimension fact is `…DimensionMismatch` at one
door and `…DocParamDimension` at the other; and the edit door's payload
pair drops `Doc` from the parameter's noun while its slot pair keeps
it. A reader who knows one pair cannot predict the next.

**Why it was not fixed there.** Renaming is cheap in the compiler and
expensive in the rosters that ride the names: `crates/pncad-py`'s tag
map and its committed tag inventory (`snapshot_error_tag`,
`edit_error_tag`), the `f6_variants!` rosters and F6 cases in
`display_contract.rs`, `test_binding_census.py`'s name map, and the
walk placement census in `persist::check`. Eight renames is one sweep
across all of those, and it is not the payload walk's unit.

**What a unit taking this owes.** One convention, stated, and every
name moved to it in one PR — the binding's tag words with them, since
a tag that no longer matches its arm is worse than either spelling.
Both doors, or say why one keeps its own.

**Not in scope**: the two `Display` sentences, which are already one
vocabulary at both doors after PR #2793 ("payload expression" at both
payload arms), and the mapper shape — `persist::check`'s
`param_ref_refusal` already converts both addresses in one function
over `ParamRefAddress`, while the edit door keeps its two
destructurings because they feed a different error type with a
different subject.
