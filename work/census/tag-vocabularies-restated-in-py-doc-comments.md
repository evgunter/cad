---
id: tag-vocabularies-restated-in-py-doc-comments
kind: issue
title: Eight delegating getters under src/py/ restate their map's vocabulary in prose
status: open
opened: 2026-09-15
priority: P4
cost: E
---



Found by CENSUS-PY-GETTERS' sweep (2026-09-15) while dispositioning
the prose restatements that unit's spec named. That spec named one
site — `mate.rs`'s subgroup getter, which hand-listed four words four
lines above the delegation to the map that holds them. **It is a
population, and this is its measurement.**

## The population, measured

Every getter under `crates/pncad-py/src/py/` that returns
`&'static str` or `Option<&'static str>`, mints no literal of its own,
and whose own doc comment spells two or more backticked words.
**Eight**, after CENSUS-PY-GETTERS converted the six inside its fence.
Each was read against the tree; the third column is where the words
actually come from, which for four of them is a `crate::tags` map
reached through a payload record rather than called at the getter:

| site | shape | where the words come from |
| --- | --- | --- |
| `py/assembly.rs`, `RefusedRef::variant` | roster of 4 | `refused_ref_tag`, called here |
| `py/assembly.rs`, `Attribution::relation` | roster of 5 | `attribution_tag`, called here |
| `py/assembly.rs`, `MintRefusal::variant` | roster of 2 | `mint_refusal_tag`, called here |
| `py/mate.rs`, `MateFault::field` | roster of 2 | `band_field_tag`, via `crate::mate_payload` |
| `py/checks.rs`, `CheckEvidence::inner_variant` | mixed | `shell_classify_error_tag`, via `crate::check_payload` |
| `py/checks.rs`, `CheckEvidence::boolean_variant` | mixed | `boolean_error_tag`, via `crate::check_payload` |
| `py/checks.rs`, `CheckEvidence::chart_variant` | mixed | `coherence_condition_tag` and `unexaminable_tag`, via `crate::check_payload` |
| `py/mate.rs`, `ClassAdmission::why` | per-arm | `class_admission_tag`'s three words, naming arms |

**Three shapes, and the row does not flatten them.** Four are a bare
roster — "the stable tag: `a`, `b`, `c`" — a copy of a list the map
already holds, and the straightforward disposition is to delete it and
point at the map. Three mix a map's words with the names of
neighbouring attributes in one sentence, so a reader cannot tell which
half is the vocabulary. One (`ClassAdmission::why`) uses the three
variant words to say which arm carries which sentence — a statement
the map does not make, where deleting the words would lose something.

## A third disposition the sweep found

Two of the six restatements CENSUS-PY-GETTERS converted turned out to
be **a Python caller's only source**, because `pncad.pyi` names the
attribute and not its words: `RefusedRef.kind` and
`Measurement.dimension`. A pyo3 doc comment is the property's
`__doc__`, so deleting the roster there deletes it from `help()`. Both
kept their roster with the map named beside it, and the stub gap is
the thing to fix rather than the prose. **A disposition for any of the
eight above has to check the stub first**, and the stub is not
uniform: of the six that unit converted, four (`ClassAdmission`,
`ClusterMaintenance`, `InterfaceCrossing`, `Subgroup`) have their
words in `pncad.pyi` and two do not. The eight were not checked
against it.

## Why it is a row

These doc comments are Python docstrings: pyo3 puts them on the
getter, so a stale one is a stale answer to `help()`. Nothing holds
any of them to the map it copies — a word renamed in `src/tags.rs`
reds `TAG_INVENTORY` and leaves every prose copy of it green.

The general question is one end of
`work/census/evaluationerror-stub-lists-five-reasons-and-the-door-raises-six.md`,
which asks it for `pncad.pyi`'s docstrings. This row is the Rust-side
half of the same question: `pncad.pyi` has an instrument that already
reads it (`src/surface_census.rs`); `src/py/`'s doc comments have
none, and building one is a source-text reader, which is this
program's standing trap.

`pncad.pyi` carries the shape too, unmeasured here: its
`Subgroup.variant` docstring hand-lists all seven subgroup words,
which are now `crate::tags::subgroup_tag`'s and inventoried.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.
