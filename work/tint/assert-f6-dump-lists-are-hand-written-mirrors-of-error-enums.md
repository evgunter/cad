---
id: assert-f6-dump-lists-are-hand-written-mirrors-of-error-enums
kind: issue
title: assert_f6's dumps lists mirror whole error enums by hand, with nothing to say the enum grew
status: open
opened: 2026-09-12
---



Found by DOOR's `dimension-all-has-readers-outside-the-viewer` sweep,
which converted the one `dumps` list in this file that had an `ALL` to
read (`a_dimension_reaches_refusal_prose_as_a_word_not_as_its_variant`,
now `Dimension::ALL.iter().map(|dim| format!("{dim:?}"))`). The
neighbours have no `ALL` to read, so they are a different fix and a
different row.

## The finding

`crates/editor-core/tests/display_contract.rs`'s `assert_f6` takes a
`dumps: &[&str]` of the variant identifiers that must NOT appear in a
rendering — the F6 claim that a refusal reads as a sentence and never
as a struct dump. Every caller writes that list by hand, and each list
is a complete mirror of one error enum's variant names:

- `interrogate_error_display_names_its_content_not_its_struct` — ten
  `InterrogateError` identifiers (`:161`).
- `parse_error_display_names_its_content_not_its_struct` — ten
  `ParseError` identifiers (`:306`).
- `select_refusal_display_names_its_content_not_its_struct`
  (`SelectRefusal`, `:224`),
  `node_pick_error_display_names_its_content_not_its_struct`
  (`NodePickError`, `:75`),
  `resolve_indeterminate_display_names_its_content_not_its_struct`
  (`ResolveIndeterminate`, `:116`),
  `resolve_fault_display_names_its_content_not_its_struct`
  (`ResolveFault`, `:284`) and
  `declare_error_display_names_its_content_not_its_struct`
  (`DeclareError`, `:144`).

Nothing ties any of them to the enum. A variant added tomorrow is not
in its list, so an arm that renders it through `Debug` passes this
suite green while the header says the opposite — the same failure mode
the `Dimension` list had, one class up.

## Why it is not fixed where it was found

The `Dimension` fix was to read a published `Dimension::ALL`. These
enums publish nothing of the kind, and `assert_f6`'s `dumps` cannot be
derived from a type in safe Rust, so the answer here is a decision
(publish an `ALL` per error enum? a census row per enum, as
`m4_pr1_dims::all_is_every_dimension` is for `Dimension`? read the
identifiers out of the source, as `test_utils::source` already lets
this tree do?) rather than a substitution. That is a unit, not a
one-line conversion, and this file is S-TINT's ground.

Note the ceiling on any answer: the `cases` fixtures beside each
`dumps` are hand-written too, so a ban list that covered every variant
would still only bite on a rendering the fixture list reaches. The two
halves want deciding together.

## Re-derived (2026-09-15, lane C)

**VERDICT: REPRODUCES**, and the drift the row predicted has already
happened in **three of the seven** lists. Nothing was fixed; the row now
has live instances rather than a forecast.

**The mechanism is unchanged.** `crates/editor-core/tests/display_contract.rs`'s
`fn assert_f6<E: Debug + Display>(err, wants, dumps)` still takes a
`dumps: &[&str]`, and all seven callers still write it by hand. None of
the three shapes the row weighs (an `ALL` per error enum, a census row
per enum, reading the identifiers out of the source through
`test_utils::source`) is in the tree.

**Each list against its enum, re-derived by name.** Variants taken from
the `pub enum` declaration in each source file; `dumps` entries counted
in `display_contract.rs`:

| row | enum (and where it is declared) | variants | in `dumps` | missing |
| --- | --- | --- | --- | --- |
| `interrogate_error_display_names_its_content_not_its_struct` | `InterrogateError`, `crates/editor-core/src/names/interrogate.rs` | 10 | 10 | — |
| `parse_error_display_names_its_content_not_its_struct` | `ParseError`, `crates/editor-core/src/parse.rs` | **11** | 10 | **`Dimension`** |
| `select_refusal_display_names_its_content_not_its_struct` | `SelectRefusal`, `crates/editor-core/src/names/geompred.rs` | **8** | 8 entries, but one is `"Angle"` (a `Dimension` identifier, deliberately) | **`Band`** |
| `node_pick_error_display_names_its_content_not_its_struct` | `NodePickError`, `crates/editor-core/src/resolve/pick.rs` | 5 | 5 | — |
| `resolve_indeterminate_display_names_its_content_not_its_struct` | `ResolveIndeterminate`, `crates/editor-core/src/resolve/mod.rs` | 3 | 3 | — |
| `resolve_fault_display_names_its_content_not_its_struct` | `ResolveFault`, `crates/editor-core/src/part.rs` | 3 | 3 | — |
| `declare_error_display_names_its_content_not_its_struct` | `DeclareError`, `crates/editor-core/src/names/flush.rs` | **3** | 2 | **`Edit`** |

**Three live holes**: `ParseError::Dimension { .. }`,
`SelectRefusal::Band(BandError)` and `DeclareError::Edit(EditError)` are
each a variant whose identifier no `dumps` list bans, so an arm that
rendered one through `Debug` passes this suite green while the file's F6
claim says the opposite. Each is a newer variant than its list — exactly
the sentence *"A variant added tomorrow is not in its list"*, three times
over, with nobody noticing.

Note the second and third are payload-carrying wrappers
(`Band(BandError)`, `Edit(EditError)`), so the F6 exposure is doubled:
neither the wrapper identifier nor anything about the inner error's own
rendering is banned.

**The stated ceiling still holds and is now measurable.** The `cases`
fixtures beside each `dumps` are still hand-written, so even a complete
ban list bites only on the renderings those fixtures reach — and for the
three missing variants there is no fixture either, so completing the ban
lists alone would change nothing until a case is added for each.

**How this was derived, and its blind spot.** Variant names extracted
from each `pub enum` body at depth 1 and eyeballed against the declaration
(`sed`/`grep '^    [A-Z]'` over the enum body); `dumps` arrays read in
full. **What that could not match**: a variant declared behind a `#[cfg]`
(none of these enums has one today), a variant whose identifier is a
substring of another so that `assert_f6`'s `contains` check accidentally
covers it, and any eighth `assert_f6` caller outside
`display_contract.rs` — `grep -rn "assert_f6"` finds the helper and its
callers only in that file.

**Recommendation (orchestrator's call).** Keep open and raise its
priority: the row is no longer a forecast. The three holes are evidence
for whichever of the three shapes is chosen, and a unit that publishes an
`ALL` (or a source-read census) would have to name them as its
regression test.
