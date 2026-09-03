---
id: pncad-py-python-feature-clippy-lane-is-red
kind: issue
title: the pncad-py python-feature clippy lane is red on main and no CI row runs it
status: open
opened: 2026-09-03
refs: [LIB-B-RESOLVE]
---

Measured at LIB-B-RESOLVE while running the unit's own pre-push checks.
Not caused by that unit and not fixed by it: the finding is that a lint
lane over the crate's largest half is red and nothing watches it.

## The measurement

On pristine `origin/main` (e46f3cc10), with the crate's `python`
feature on:

```console
$ cargo clippy -p pncad-py --features python --all-targets -- -D warnings
error: very complex type used. Consider factoring parts into `type` definitions
   --> crates/pncad-py/src/py/value.rs:319:11
    = note: `-D clippy::type-complexity` implied by `-D warnings`
```

The offending field is `Datum`'s `axes: Option<((f64, f64, f64), (f64,
f64, f64))>` — a frame's two sketch axes as bare dimensionless triples,
which is the shape `py/place.rs`'s direction rule requires. Verified by
`git checkout origin/main -- crates/pncad-py/` and re-running, so this
is inherited and not a lane artefact.

## Why nothing catches it

`.github/workflows/ci.yml:1523` runs `cargo clippy ${cargo_scope}
--all-targets -- -D warnings` at DEFAULT features, and pncad-py's
manifest gates `pyo3` behind the non-default `python` feature
precisely so hosted CI needs no interpreter. That gating is right and
this issue does not propose changing it. The consequence, though, is
that **every `#[pyclass]` in the crate is outside every clippy row CI
runs** — `src/py/` is roughly the crate's whole PyO3 surface, and the
only lane that compiles it is `crates/pncad-py/run-python-tests.sh`,
which builds and does not lint.

So the red is not a regression anyone let through; it is a lane that
has never been green because it has never been run.

## What a fix would have to decide

Two independent questions, and they are not the same size:

1. **The lint itself.** Either factor the axis pair into a
   `type SketchAxes = ((f64, f64, f64), (f64, f64, f64));`, or allow
   `clippy::type_complexity` on that field with the reason (the triple
   IS the dimensionless-direction convention and naming it as a tuple
   alias may read worse than the tuple). A few lines either way.
2. **Whether the lane should run at all.** Adding a clippy row at
   `--features python` puts pyo3 back into a CI job, which is exactly
   what the manifest's gating note argues against for the BUILD job.
   A lint-only row does not need an interpreter to compile (pyo3
   without `extension-module` links libpython, which does need one), so
   the honest options are `--features extension-module` on a runner
   that has Python, or leaving the lane to the local runner script and
   saying so.

The second is the real question and it belongs to whoever owns the
pncad-py CI shape, not to a binding unit. Filed so the next unit that
runs the command does not spend the same twenty minutes proving it is
not theirs.
