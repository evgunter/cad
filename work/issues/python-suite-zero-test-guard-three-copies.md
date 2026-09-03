---
id: python-suite-zero-test-guard-three-copies
kind: issue
title: The python suite's zero-test guard exists in three places because no shared runner does
status: open
opened: 2026-09-03
---


`python -m unittest discover` over a directory whose modules do not match
its pattern prints `Ran 0 tests in 0.000s` / `OK` and **exits 0**
(verified on CPython 3.12). So a renamed `tests/` directory, a
`--start-directory` that stops resolving, or a wheel that imports but
exports nothing the suite names leaves the row green having executed
nothing — the same silence `ci.yml`'s release-profile row keeps a count
guard for, one language over.

S-TCOST C3 added that guard: the run's output is captured, `Ran N tests`
is read back (it goes to STDERR, so the redirect is load-bearing), the
count is echoed, and a zero fails the row.

**It exists in three places**, and that is the finding rather than the
fix:

- `.github/workflows/ci.yml`, the `python suite (wheel + guide +
  north-star)` job;
- `.github/workflows/nightly.yml`, the `python suite (ungated re-take)`
  job;
- `crates/pncad-py/run-python-tests.sh`, which `local-scripts/ci-local.sh`
  runs.

**There is no one place all three call, and the reason is structural.**
The hosted jobs cannot call `run-python-tests.sh`: it builds through
`local-scripts/with-build-slot.sh`, a tree every hosted job deletes at
checkout by design, and it stages a cdylib rather than installing the
wheel the hosted rows are about. Lifting the runner into `scripts/` is
the obvious move and is blocked by a second rule rather than by
difficulty: `scripts/check-ci-mirror-parity.py`'s claim 1 requires a
`scripts/` path named by a workflow to be named literally by
`local-scripts/ci-local.sh` too, and ci-local's row names
`run-python-tests.sh` instead. A fix therefore has to move the seam —
either ci-local.sh calls the shared runner directly (and
`run-python-tests.sh` becomes staging only), or the parity claim learns
to resolve one hop through a non-`scripts/` script.

Filed rather than done because it is a change to a documented developer
tool's contract and to a parity gate, neither of which belongs inside a
CI-posture unit.
