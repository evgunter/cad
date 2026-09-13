---
id: closure-tier-skips-python-suite-on-geom-core-changes
kind: issue
title: TIER=closure on a geom-core/geom public-signature change runs RUN_PNCAD_PY=false — the python wheel is never built although it compiles against those crates
status: closed
opened: 2026-09-05
branch: ciw/python-suite-closure
pr: 2071
closed: 2026-09-07
---


(PROPS orchestrator) Reported by the Span-sweep lane (PR #1952) and by
PROPS-1's lane before it (PR #1918): both changed public signatures in
`geom-core`/`geom` and both ran `TIER=closure` with `RUN_PNCAD_PY=false`
("the seeds do not reach `pncad-py`"), so the python wheel — which
compiles against those crates through the `pncad` façade — was never
built by the gate; each lane ran the suite locally instead.
`docs/prompts/implementer-discipline.md` §2 says the python suite runs
on every code-tier run; `scripts/ci-filter.py` disagrees for closure
runs seeded outside `pncad-py`'s direct dependents. One of the two is
wrong; the cheap fix is to treat `pncad-py` as a dependent of every
crate the façade re-exports whole (`geom_core`, `geom`, …), or to say in
the discipline doc that the closure tier may skip it and that a lane
changing a re-exported crate must run it locally.

## Disposition (PR 2071)

**The filter was wrong**, and the doc was stale beside it; both moved.

`RUN_PNCAD_PY` stays SEED-keyed and its set is now every member a BUILD
OF THE WHEEL compiles — `pncad-py`'s non-dev dependency closure, read
off `cargo metadata`. On this tree that is every workspace member except
`viewer` (above the wheel) and `test-utils` (a dev edge `maturin build`
does not follow). `crates/geom-core/src/lib.rs` now gives
`RUN_PNCAD_PY=true`; so do `bvh` and `verbs`.

**The first attempt derived the set from the façade's own text** — the
members named by a top-level `pub use`/`pub mod` in
`crates/pncad/src/lib.rs` — and both reviews landed the same MAJOR: that
ships the defect one re-export deeper. `bvh` is named by no line of the
façade and still reaches Python, through `editor-core`'s
`pub use bvh::Ray`, `pncad::select`'s re-export, and a `#[pyclass] Ray`
in `crates/pncad-py/src/py/pick.rs` that `tests/test_picking.py` drives
in 37 places. Top-level naming is SUFFICIENT for reach, not necessary.
The graph has no such gap and needs no regex.

**What decided the direction.** The C3 argument for the small set held
that a compile break reds the ordinary closure rows and that only
NUMBERS wait for the nightly. The first half is true — `clippy
(--all-features)` lints `-p pncad-py` at the `python` feature on every
code-tier run — but the second is not a reason to skip: the
`crates/pncad-py/tests/*.py` assertions run in no other job. And the
skip bought nothing: the job needs only `filter` and finishes far inside
the run's serial build → test chain, so it adds zero wall clock. Figures
and caveats: `docs/CI-MINUTES-2026-08.md`, entry of 2026-09-06.

This makes the axis C3's closure condition again, up to dev edges, and
that is deliberate and stated at every site that describes it.

**Residue, filed rather than disclosed**:
`work/ciw/python-suite-axis-skips-only-two-members.md` asks whether an
exception covering two members still earns its machinery. Findings
outside this unit's fence are named in the PR body: the viewer axis's
"one direct non-façade edge" sentence
(`scripts/ci-filter.py`, `RUN_VIEWER_TOOLKIT`) is false —
`crates/viewer/Cargo.toml` also has `editor-core` — and two closed LIB
items carry the moved seed-set premise in their prose.

**Retracted from the first pass**: that `work.py territory` is blind to
a `keep_out` fence. It is not. The zero it returned was read off an
UNCOMMITTED tree — `territory` diffs `origin/main...HEAD` — and on the
committed branch it names both cross-fence paths correctly,
`scripts/ci-filter.py` (tcost) and `docs/prompts/implementer-discipline.md`
(meta). The lesson is about when to run it, not about what it can see.

## Closed 2026-09-07

PR 2071. `RUN_PNCAD_PY`'s seed set is `pncad-py`'s non-dev dependency closure,
read off the member graph: a `geom-core` change runs the suite, and so does a
`bvh` change — the crate both reviews found reaching Python through
`editor-core`'s `pub use bvh::Ray`, which the first pass's façade-text
derivation missed. `viewer` (above the wheel) and `test-utils` (a dev edge)
stay out. `docs/prompts/implementer-discipline.md` §2 now says what the filter
does.

The residue is `python-suite-axis-skips-only-two-members`, open on this slate:
the axis now excludes exactly two members, and the number that would settle
whether the machinery is worth keeping has not been taken.
