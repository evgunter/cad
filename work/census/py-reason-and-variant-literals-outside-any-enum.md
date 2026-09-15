---
id: py-reason-and-variant-literals-outside-any-enum
kind: issue
title: three Python-visible discriminant words are still minted as literals at construction sites under src/py/
status: open
opened: 2026-09-15
---



Found by CENSUS-TAG-REACH's sweep (2026-09-15), which closed the
evaluation door's half of the same class and left this half measured
rather than repaired.

## The class, and what stays open

A Python-visible discriminant reaches a caller as an exception's
`reason` or `variant` attribute. `TAG_INVENTORY`
(`crates/pncad-py/src/tests.rs`,
`the_whole_tag_table_matches_its_committed_inventory`) lexes
`crates/pncad-py/src/tags.rs` alone, so a word minted as a string
literal at a construction site under `src/py/` is public Python
vocabulary no inventory reads.

CENSUS-TAG-REACH closed that for the EVALUATION door: `eval_err`'s
reason is now `crate::errors::EvalReason`, `crate::tags::eval_reason_tag`
is the exhaustive map, and a literal at the call site does not compile.
**Three sites in two files have no such enum and are unchanged:**

| word | site | door |
| --- | --- | --- |
| `mass_properties_failed` | `crates/pncad-py/src/py/value.rs`, `measurement_err` | `ValidationError`, not the evaluation door — two measurement doors share the one construction site, and the word is that site's literal |
| `unclassified` | `crates/pncad-py/src/py/flush.rs`, the unknown-`ContactClass` refusal | `SelectError`. It collides by hand with `select_refusal_tag`'s own `"unclassified"` (pinned in `TAG_INVENTORY`), which is a second spelling of one word rather than a reference to it |
| `wireframe` | `crates/pncad-py/src/py/value.rs`, the STEP-import arm this door does not adopt | `StepImportError`. The site's comment says the word shares a namespace with `step_import_error_tag`'s, "which contains no `wireframe`" — so the namespace is spelled in two places |

**Each is covered only by accident.** `mass_properties_failed` is
named in `crates/pncad-py/pncad.pyi` and asserted once in
`crates/pncad-py/tests/test_validate.py`; `wireframe` is named in the
stub and two Python tests; `unclassified` is in neither the stub nor
any Python test. Nothing reds if one is renamed, and nothing reds if a
fourth is added — which is the standing lesson, not the three words.

`crates/pncad-py/src/tests.rs`'s inventory doc carries this list as
"what is still outside", so the fact is stated where the guard is; a
PR body is not a slate, which is why it is also here.

## Shape of the fix, if it is taken

The evaluation door's shape transfers: give each door's reason
vocabulary an enum in `crate::errors`, an exhaustive map in
`src/tags.rs`, and a typed parameter at the construction site. The
`unclassified` case is smaller than that — `select_refusal_tag`
already mints the word, so `flush.rs` can call it.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.
