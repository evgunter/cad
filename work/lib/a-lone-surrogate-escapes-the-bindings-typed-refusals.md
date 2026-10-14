---
id: a-lone-surrogate-escapes-the-bindings-typed-refusals
kind: issue
title: A lone surrogate in a Python str raises UnicodeEncodeError at extraction, not the door's typed refusal
status: open
opened: 2026-09-29
priority: P4
cost: M
---

Found by the review of `edit/param-name-door` (PR #3164), probe
`test_a_lone_surrogate_refuses_as_the_name_door` on
`review/paramname-rev`.

## The finding

A Python `str` may hold a lone surrogate (`"\ud800"`); a Rust `&str`
or `String` cannot. pyo3's argument extraction encodes the `str` as
UTF-8 before the method body runs, so the call raises
`UnicodeEncodeError` ("surrogates not allowed") and the door's own
typed refusal is never asked.

Measured on the PR's wheel:

- `ParamName("\ud800")` raises `UnicodeEncodeError`, not `EditError`
  with `variant == "param_name_not_an_identifier"`
  (`crates/pncad-py/src/py/doc.rs`, `ParamName::new`, which takes
  `name: &str`).
- `Doc("\ud800")` raises `UnicodeEncodeError` too
  (`crates/pncad-py/src/py/doc.rs`, `Doc::new`, `label: Option<&str>`).

It is binding-wide, not `ParamName`'s: every `#[pymethods]` or
`#[pyfunction]` parameter typed `&str` or `String` has it (`load`'s
`text: &str` in the same file, and the other `&str` parameters across
`crates/pncad-py/src/py/`).

## What a fix has to decide

Whether an unencodable `str` is a boundary refusal of its own (one
typed arm at the extraction, e.g. by taking `&Bound<PyString>` and
converting with a mapped error, shared by every text-taking door), or
whether `UnicodeEncodeError` is the accepted Python-side answer and the
stub says so once. The binding's census
(`crates/pncad-py/tests/test_binding_census.py`) is where a typed
answer would be pinned per door.
