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
