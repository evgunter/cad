---
id: pncad-py-python-feature-clippy-lane-is-red
kind: issue
title: the pncad-py python-feature clippy lane is red on main and no CI row runs it
status: closed
opened: 2026-09-03
refs: [LIB-B-RESOLVE]
closed: 2026-09-03
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

## Closed

2026-09-03. Both questions this issue posed are answered; the lane is
green and a row runs it. Duplicate of
`the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row`, filed from
the other lane; both close together.

**Question 1, the lint.** The `#[allow]`, not the alias.
`crates/pncad-py/src/py/value.rs`'s `Datum::axes` keeps
`Option<((f64, f64, f64), (f64, f64, f64))>` and carries
`#[allow(clippy::type_complexity)]` with the reason written at the
field: it is a `#[pyo3(get)]` projection, so the tuple written there IS
the object Python receives and `pncad.pyi` states it literally, and the
sibling projections `direction` and `in_plane` are equally literal. The
`.pyi` was not touched and the field type did not move, so the runtime
tuple and the stub signature are byte-identical to before.

**Question 2, whether the lane should run.** It runs, and this issue's
"honest options" list turned out to have a third entry. The premise that
a lint row would need `--features extension-module` does not hold:
clippy is check-only, so the `cdylib` is never linked and no libpython
is needed at plain `--features python` — a cold `cargo clippy -p
pncad-py --features python --all-targets` leaves no `libpncad_py.so`
behind, only the proc-macro dylibs every clippy run builds. What it does
need is an interpreter for pyo3's build script (`error: no Python 3.x
interpreter found` with none on `PATH`, measured). So the row sits where
an interpreter already is: ci.yml's `python-suite` job and nightly.yml's
`python suite (ungated re-take)`, plus `local-scripts/ci-local.sh` as
`clippy (pncad-py, python)`. Not in the `clippy` job — that job is on
the critical path, and a second feature graph there is the compile the
seed-key work had just finished removing.

**What the siting gives up.** `python-suite` is seed-keyed and skipped
on `push`: on a PR whose seeds miss {pncad-py, pncad, editor-core} the
row does not run. A lint the binding code or `pncad`/`editor-core`
introduces still seeds the axis and reds that PR; one arriving from
`quantity`, from the kernel through `pncad`'s re-exports, or from a
`stable` toolchain bump waits for the nightly re-take, within a day.
Stated rather than papered over — the coverage this closes is "the lane
is watched daily and on every PR that touches the bindings", not "on
every PR".
