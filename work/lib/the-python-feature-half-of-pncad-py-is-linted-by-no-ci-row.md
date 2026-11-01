---
id: the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row
kind: issue
title: the python-feature half of pncad-py is linted by no CI row
status: closed
opened: 2026-09-03
refs: [1668]
closed: 2026-09-03
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

## Closed

2026-09-03. The row exists and the one standing finding is cleared.
Duplicate of `pncad-py-python-feature-clippy-lane-is-red`, filed from
the other lane; both close together.

**The lint.** `crates/pncad-py/src/py/value.rs`'s `Datum::axes` keeps
its tuple and carries `#[allow(clippy::type_complexity)]` with the
argument at the site. The `type` alias was the other option and was not
taken: the field is a `#[pyo3(get)]` projection, so the written shape IS
what Python receives, `pncad.pyi` states it literally, and its two
neighbours (`direction`, `in_plane`) are literal tuples that stay under
clippy's threshold — naming only the third would make three projections
of one kind read as two. Nothing crossing to Python moved: the field
type is unchanged, `pncad.pyi` is untouched, and the staged-cdylib suite
passes.

**The row.** `cargo clippy -p pncad-py --features python --all-targets
-- -D warnings`, added to ci.yml's `python-suite` job and to
nightly.yml's `python suite (ungated re-take)`, and mirrored at
`local-scripts/ci-local.sh` as `clippy (pncad-py, python)`. Hosted it
rides the python job rather than `clippy` under a constraint that it not
add compile time to the critical path: that job already installs Python
3.12, already restores the `python` feature graph's own cache, and
nothing `needs` it. Measured cost: 37 s wall on a cold target directory,
and it warms nothing the wheel build reuses (maturin compiles
`extension-module`, a different feature set).

**What the siting gives up.** `python-suite` is seed-keyed and skipped
on `push`, so the new row does not run on a PR whose seeds miss
{pncad-py, pncad, editor-core}. A lint introduced by the binding code
itself, or by `pncad`/`editor-core`, still seeds the axis and reds that
PR. One reaching `src/py/` from further down — `quantity`, or the kernel
through `pncad`'s re-exports — or from a `stable` toolchain bump
shipping a new lint, seeds nothing and surfaces at the nightly re-take
instead, within a day. That is the same trade the suite itself already
makes, not a new one, but it is a real gap and is not claimed otherwise.
