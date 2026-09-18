---
id: run-pncad-py-is-computed-and-gates-nothing
kind: issue
title: RUN_PNCAD_PY survives the axis it keyed: computed on every run, read by no job
status: open
opened: 2026-09-12
---


Disclosed by the C1/C2/C3 restoration (branch `tcost/c1-c3-restore`),
which is what made it true, and filed in the same PR rather than left as
a sentence in one.

## The state

`python suite (wheel + guide + north-star)` runs on every code-tier run
of `ci.yml` and unconditionally in `local-scripts/ci-local.sh`. So:

- `scripts/ci-filter.py` still computes `RUN_PNCAD_PY` in `decorate`,
  off `pncad_py_seeds`' `cargo metadata` derivation of the members a
  build of the wheel compiles, with its fail-closed arms and about ten
  `--selftest` cases and a whole fixture graph (`_PY_FIXTURE_PKGS`)
  behind it.
- **No job reads it.** `ci.yml`'s `filter` no longer publishes a
  `run_pncad_py` output; `ci-local.sh` never consulted it.

The restoration kept it deliberately and said so at the site: the value
is echoed with the seeds it came from, so the filter's log answers "did
this change reach the bindings" for someone triaging a python-suite red.
That is a real use and a thin one, and it is not what the machinery was
built for.

## Why it is not just left alone

The sibling case was decided the other way in the same PR.
`scripts/doc-gate.sh`'s `--pr`/`--scope` mode had no caller left once
the rustdoc gate went back to running whole, and it was DELETED — a mode
no workflow invokes is the shape `docs/prompts/implementer-discipline.md`
names as the floor defect, and every sentence of its prose had become a
statement about a job that no longer exists. `RUN_PNCAD_PY` is the same
shape with a smaller blast radius: it is not a mode, nothing can invoke
it by mistake, and its arms are exercised by the filter's own selftest
rather than rotting. That is why it survived the same diff, not an
argument that it should survive the next one.

## What to decide

Either give it a consumer or delete the axis — the key, the
`pncad_py_seeds` call in `decorate`, `_PY_FIXTURE_PKGS` and the selftest
cases that pin it. `pncad_py_seeds` itself stays either way: the
`--selftest` banner prints the wheel's member set, which is how a reader
checks what the wheel compiles.

**Do not delete it as dead weight without reading the selftest first.**
Those cases are the only executable statement in the tree of which
members a `maturin build` compiles and which edges it does not follow
(above the wheel, dev-only), and that fact outlives the axis that used
it.
