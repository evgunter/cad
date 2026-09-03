---
id: the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row
kind: issue
title: the python-feature half of pncad-py is linted by no CI row
status: open
opened: 2026-09-03
refs: [1668]
---

Found while closing census family B-CANCEL (LIB-B-CANCEL). Not a
defect in that unit's diff; a hole in the guard set the unit ran into
and could not have found any other way, because the clippy invocation
that finds it is not one CI makes.

## The measurement

`crates/pncad-py/src/py/` is 12621 lines across 18 files — the whole
PyO3 binding surface — and it compiles only under the crate's
`python` feature (`crates/pncad-py/src/lib.rs:33`, `#[cfg(feature =
"python")] mod py;`). Two CI rows touch this crate and neither lints
that half:

* **the clippy row** — `.github/workflows/ci.yml`, mirrored at
  `local-scripts/ci-local.sh:1019`, runs `cargo clippy $SCOPE
  --all-targets -- -D warnings` with DEFAULT features. `python` is
  off by default (`crates/pncad-py/Cargo.toml:99`, `pyo3` optional),
  so the `#[cfg]` excludes every file under `src/py/` before clippy
  sees it;
* **the python-suite row** — `ci.yml`'s `python suite (wheel + guide
  + north-star)` — DOES compile that half, via `maturin build -m
  crates/pncad-py/Cargo.toml`, but runs no clippy at all: it builds a
  wheel, installs it into a venv, and runs `unittest discover`.

Measured in the LIB-B-CANCEL lane, 2026-09-03:

```
$ cargo clippy -p pncad-py --all-targets -- -D warnings
    Finished `dev` profile ... (clean)

$ cargo clippy -p pncad-py --all-targets --features python -- -D warnings
error: very complex type used. Consider factoring parts into `type` definitions
   --> crates/pncad-py/src/py/value.rs:319:11
    |
319 |     axes: Option<((f64, f64, f64), (f64, f64, f64))>,
```

That finding is pre-existing on `main`
(`git show main:crates/pncad-py/src/py/value.rs`, same line, same
text) and is the ONLY one: with `-A clippy::type_complexity` the
feature-on run is clean, so the accumulated debt is one lint, not a
backlog. That is the good news and also the reason to close the hole
now rather than after it becomes a cleanup.

## Why it matters here rather than as housekeeping

Every other Rust surface in this repo is `-D warnings` on the merge
gate, and the LIB units have been writing binding code for months
under the belief that this one was too. A lane running the local
matrix sees the same green a hosted run does, because
`local-scripts/ci-local.sh` mirrors `ci.yml` row for row and the
mirror is itself guarded (`scripts/check-ci-mirror-parity.py`) — so
the parity guard is working exactly as designed, and the gap is in
what the rows ASK FOR, which no parity check can see.

## What closing it looks like

A clippy row for `-p pncad-py --features python --all-targets`, in
both `ci.yml` and the local mirror (parity requires both), plus
whatever the one standing finding wants — a `type` alias for the axis
pair, or a recorded `#[allow]` at the field with the reason, which is
a real option since the field is a `#[pyo3(get)]` projection whose
shape is the Python tuple it becomes.

Not done in LIB-B-CANCEL: the fix touches the merge gate and a field
that unit does not bind, and a mechanical census lane changing the
gate under its own PR is the wrong shape for both.
