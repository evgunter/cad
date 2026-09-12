---
id: python-suite-axis-skips-only-two-members
kind: issue
title: the python-suite axis now skips only viewer and test-utils — does the exception still earn its machinery
status: open
opened: 2026-09-06
---


Disclosed by CIW unit 2 (PR 2071), which is the change that created the
condition. Filed rather than left in that PR's body per `work/README.md`:
a disclosed residue owes a file on its own program's slate.

## What the axis is now

`RUN_PNCAD_PY` is true when the change filter's SEEDS intersect the
members a build of the wheel compiles — `pncad-py`'s non-dev dependency
closure, derived from `cargo metadata` by `scripts/ci-filter.py`'s
`pncad_py_seeds`. On this tree that is sixteen of eighteen members. The
two it excludes:

- `viewer` — above the wheel; nothing under `pncad-py` depends on it.
- `test-utils` — reaches `pncad-py` along a dev edge only, and
  `maturin build` compiles no test target.

## The question

The machinery that expresses that exception is: a second edge map in
`_member_graph`, `_dependency_closure`, `pncad_py_seeds`, a
`wheel_members` argument threaded through `decorate`, a failure arm, and
six selftest cases. What it buys is skipping a ~120 s job on the runs
whose seeds are confined to those two members.

Against that: `RUN_PNCAD_PY = "false" if tier == "docs" else "true"` —
the shape `RUN_K_LINT` already has, argued at its own site as "any
member change can break it, so it runs whenever anything builds" —
deletes all of it. The measured wall-clock cost of the job is zero
(`docs/CI-MINUTES-2026-08.md`, entry of 2026-09-06: it needs only
`filter` and finishes 692–917 s before the run ends), so what the
exception saves is billed minutes on a public repository, where they
are free.

The counter-argument is that a skip which is *sound* and *recorded* is
better than no skip, and that `viewer` is a large enough crate to be
worth not paying for. Neither side has been measured against the
population: **how many code-tier runs actually seed only `viewer` or
only `test-utils`** is the number that settles it, and it has not been
taken. Take it over the last few hundred first-parent merges before
deciding.

## Not a defect

The axis is correct as it stands; this is a question about whether the
exception earns its complexity. It is not blocking anything.

## Fence

`scripts/ci-filter.py` is S-TCOST's by this program's `keep_out`. Any
change here is an announced cross-fence change, as PR 2071 was.
