---
id: closure-tier-skips-python-suite-on-geom-core-changes
kind: issue
title: TIER=closure on a geom-core/geom public-signature change runs RUN_PNCAD_PY=false — the python wheel is never built although it compiles against those crates
status: review
opened: 2026-09-05
branch: ciw/python-suite-closure
pr: 2071
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

`RUN_PNCAD_PY` keeps the seed shape Ev ruled on — SEEDS, not the
closure — and its seed set is widened from `{pncad-py, pncad,
editor-core}` by every workspace member the façade names at the top
level of `crates/pncad/src/lib.rs`, derived there rather than listed.
On this tree that adds `geom, geom-brep, geom-core, mesh, profile,
quantity, step-export, step-import, stl, sweep, topo`; it leaves out
`bvh, test-utils, verbs, viewer`, the members the façade keeps
interior. `profile` is in it because the façade narrowed
`pub use profile` to a curated `pub mod profile` and the bindings still
name `pncad::profile`, so the derivation reads `pub mod` as well as
`pub use`.

**What decided the direction.** The C3 argument for the small set held
that a compile break reds the ordinary closure rows and that only
NUMBERS wait for the nightly. The first half is true — `clippy
(--all-features)` lints `-p pncad-py` at the `python` feature on every
code-tier run — but the second is not a reason to skip: the
`crates/pncad-py/tests/*.py` assertions run in no other job, and a
re-exported crate is a crate those scripts call. And the skip bought
nothing: measured over 35 code-tier runs on 2026-09-06, the `python
suite` job takes 115–125 s, needs only `filter`, and finishes 692–917 s
before the run ends (run wall clock 856–1302 s, set by the serial
build → test chain). Zero added wall clock, which is the currency on a
public repository.

**Residue, disclosed and not scheduled here**: whether the axis should
be a seed key at all now that its row is free is S-TCOST's question,
raised in the PR for the owner of `scripts/ci-filter.py`. Two findings
outside this unit's fence are reported in the PR body rather than filed:
a stale "linted by NO row" claim on ci.yml's `python-suite` clippy step,
and `work.py territory` being blind to a `keep_out` fence because it
reads `paths` only.

