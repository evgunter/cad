---
id: datum-kind-vocabulary-is-hand-spelled-and-uncensused
kind: issue
title: Datum.kind's five words are struct-field literals in py/value.rs and no roster covers them
status: open
opened: 2026-09-15
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

## Shape of the fix, if it is taken

`crate::node_kind` plus `NODE_KIND_ROSTER` is the pattern already in
this crate: a discriminant enum with an exhaustive map, and a roster
re-derived from the map at test time. `d::DatumValue`'s arms are the
enum here, so the map can key off the kernel value directly and the
five literals become one `match`.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.
