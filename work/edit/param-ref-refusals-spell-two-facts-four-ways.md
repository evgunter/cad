---
id: param-ref-refusals-spell-two-facts-four-ways
kind: issue
title: The param-table rule's two facts are spelled four ways across eight refusal arms
status: review
opened: 2026-09-17
branch: edit/param-ref-one-convention
pr: 2819
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

## Ruled and spec'd (2026-09-17, EDIT orchestrator) — middle tier, branch `edit/param-ref-one-convention`

**Ruling: one convention, the load door's — the ADDRESS leads and the
FACT trails, with one noun.** At both doors the eight arms become
`{Slot,Payload}UnknownDocParam` and `{Slot,Payload}DocParamDimension`:
`EditError::UnknownDocParam` → `SlotUnknownDocParam`,
`DocParamDimensionMismatch` → `SlotDocParamDimension`,
`UnknownPayloadParam` → `PayloadUnknownDocParam`,
`PayloadParamDimensionMismatch` → `PayloadDocParamDimension`; the
four `SnapshotError` arms already read so and do not move. The load
door's spelling wins because it is the walk's — the address is what
`Walk::ORDER` iterates and what `ParamRefAddress` carries — and the
edit door's slot pair gains the `Slot` word it always meant. The
binding's tag words follow the names (`slot_unknown_doc_param`,
`slot_doc_param_dimension`, `payload_unknown_doc_param`,
`payload_doc_param_dimension` at the edit door too), because a tag
that no longer matches its arm is worse than either spelling; the
three Python tests that read `"unknown_doc_param"` move with it, and
the PR body says in one line that the Python-facing tag word changed
and why (LIB's surface, crossed by announcement — the rule is the
row's, already stated).

**What moves together, in one PR.** The four `EditError` arms and
every `match` on them; `tags.rs`'s `edit_error_tag` and the committed
tag inventory; `display_contract.rs`'s `f6_variants!` rosters and F6
cases; `test_binding_census.py`'s name map; `persist::check`'s walk
placement census where it names an edit-door arm; every doc sentence
that spells an old name (sweep by the old identifiers AND by their
tag words). `Display` sentences do not change (already one vocabulary
after #2793); the mapper shapes do not change (the row's "not in
scope" stands).

**Rows.** No new row: the F6 cases, the tag census and the binding
census ARE the rows, and each must be green with the new names and
red with a stale roster (state the census that would red on a missed
site). One guard the unit adds if it is cheap: a `display_contract`
row that the four edit-door arms and the four load-door arms carry the
same four suffixes (the convention, pinned).

**Territory.** `crates/editor-core/src/{edit.rs, persist/check.rs}`
(EDIT); `crates/editor-core/tests/display_contract.rs` (TCOST/TINT);
`crates/pncad-py/src/tags.rs`, `tests/test_binding_census.py`,
`tests/test_{document,placed_union,slot_edits}.py` (LIB, mechanical).
Middle tier rather than E-class because the Python tag words move:
one opus style review with a correctness arm, then the fix pass.

## Built (2026-09-17, PR #2819, `edit/param-ref-one-convention`)

**The ruling landed whole.** At both doors the param-table rule's two
facts are spelled `{Slot,Payload}UnknownDocParam` and
`{Slot,Payload}DocParamDimension` — the same four names, so a reader
who knows one arm can spell the other seven. The four `EditError` arms
moved (`edit.rs`: the declarations, the `Display` arms, `check_param_refs`
and the payload walk), and with them `tags.rs`'s `edit_error_tag` and
`edit_inner_variant_tag`, `edit_payload`'s arms, `TAG_INVENTORY`'s
`edit_error_tag` row (re-sorted), `test_binding_census.py`'s
`MEMBERS_BOUND_AS`, the `pncad.pyi` and `py/doc.rs` sentences that
quote a tag word, and six `editor-core` test files. The edit door's
tag words moved with the names.

**The guard.**
`display_contract::the_two_doors_spell_the_four_param_ref_refusals_with_the_same_four_names`
reads the eight variant names off `Debug` and asserts the edit door's
four are the load door's four, against the `{address} x {fact}`
product spelled once. Proved red on a one-door rename before it was
green.

**What did not move**, as spec'd: the `Display` sentences, both mapper
shapes, and the four `SnapshotError` arms.

**Corrections to the spec's premises** (all in the PR body):
`persist::check` names no edit-door arm, so nothing moved there and
the file is not in the diff; no `f6_variants!` roster changed, because
the only roster naming any of the eight is `SNAPSHOT_ERROR`; and nine
files beyond the territory paragraph's list carry the old identifiers
and were swept.

**Filed, not fixed here**: the four edit-door tag words are now also
`snapshot_error_tag`'s, which
`every_word_two_tag_maps_share_is_on_the_committed_roster` reds on.
They are pinned in `SHARED_TAG_WORDS` as one fact, and the evidence is
appended to CENSUS's open row
`work/census/sixty-one-tag-words-are-minted-by-two-or-more-maps-and-seven-are-read.md`.

**Left with the old spelling, deliberately**: the test function
`m10_2_r1_probes::r1_an_unknown_payload_param_refuses_at_the_edit_door`
(prose, not an identifier the sweep can see) and four closed
`work/edit/` rows whose bodies record a finding at the SHA they
describe.
