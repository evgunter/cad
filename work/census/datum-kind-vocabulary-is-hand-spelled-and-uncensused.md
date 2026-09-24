---
id: datum-kind-vocabulary-is-hand-spelled-and-uncensused
kind: issue
title: Datum.kind's five words are struct-field literals in py/value.rs and no roster covers them
status: open
opened: 2026-09-15
priority: P3
cost: D
---



Found by CENSUS-TAG-REACH's sweep (2026-09-15). It is the same class
as that unit's row and a different KEY, which is why the `reason` and
`variant` sweep did not reach it.

## The population

`crates/pncad-py/src/py/value.rs`, `Value::datum`, builds the `Datum`
`#[pyclass]` with a `kind: &'static str` field spelled as a literal at
each of five arms: `"plane"`, `"axis"`, `"point"`, `"frame"`,
`"axis_in_plane"`. They reach a Python caller as `Datum.kind`
(`crates/pncad-py/pncad.pyi`, `Datum.kind -> str`) and a caller
branches on them exactly as on an exception's `reason`.

**No census covers them.** `TAG_INVENTORY`
(`crates/pncad-py/src/tests.rs`) lexes `src/tags.rs`; `NODE_KIND_ROSTER`
beside it re-derives `crate::node_kind`'s vocabulary and is a worked
example of the shape this wants. `Datum.kind` has neither. `"point"`,
`"frame"` and `"axis_in_plane"` are named in no Rust test at all —
their only pins are scattered assertions in `crates/pncad-py/tests/`
that happen to read the attribute.

## Why the sibling sweep missed it

A `reason` or `variant` word is minted as an argument beside a key of
that name, which is greppable. This one is a STRUCT FIELD on a value
class, so the same pattern finds nothing. That is the blind spot to
state rather than the five words: every Python-visible `&'static str`
field on a `#[pyclass]` is a vocabulary of this shape, and this row
records one instance and no claim that it is the only one.

## The shape, enumerated (CENSUS-PY-GETTERS, 2026-09-15)

That unit's sweep was keyed on the WORD rather than on any syntax, so
it saw the struct-field shape this row records. **Four
Python-visible `&'static str` fields on a `#[pyclass]` in this crate,
which is the whole population** — this row's "no claim that it is the
only one" now has a number behind it:

* `Datum.kind` — five literals, no roster. This row.
* `Measurement.dimension` — four words (`Length`, `Angle`, `Count`,
  `Scalar`), which were `py/value.rs`'s `dimension_name`. **Closed by
  CENSUS-PY-GETTERS**: the map is now
  `crate::errors::measurement_dimension_tag` and
  `the_two_dimension_alphabets_are_one_list_in_two_cases` derives its
  words from `crate::errors::dimension_tag` over `Dimension::ALL`.
* `Verdict.status` — three words (`Holds`, `Violated`,
  `Unevaluated`), minted KERNEL-side by `editor-core`'s
  `AssertionVerdict::label`, so no instrument in this crate reaches
  them. Their only pins are nine assertions on the attribute in
  `crates/pncad-py/tests/test_measures.py` and `test_north_star.py`,
  which do red on a rename — the same "covered only by accident"
  standing that
  `src/tests.rs`'s tag-table header records for three other words.
* `Resolution.status` — clean: `py/resolve.rs` fills it from
  `crate::tags::resolution_status_tag`, which the inventory lexes.

So the shape has one uncovered instance (this row's) and one that is
kernel-minted and outside this crate's reach.

## Shape of the fix, if it is taken

`crate::node_kind` plus `NODE_KIND_ROSTER` is the pattern already in
this crate: a discriminant enum with an exhaustive map, and a roster
re-derived from the map at test time. `d::DatumValue`'s arms are the
enum here, so the map can key off the kernel value directly and the
five literals become one `match`.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.
