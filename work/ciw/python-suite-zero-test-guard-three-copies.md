---
id: python-suite-zero-test-guard-three-copies
kind: issue
title: The python suite's zero-test guard exists in three places because no shared runner does
status: closed
opened: 2026-09-03
closed: 2026-09-04
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

## Closed (2026-09-04): Ev's call, and the cost/benefit was already the finding

**Ev, 2026-09-04 (in chat, on CIW's opening slate):** close it — the
failure is rare and not worth the special effort.

This item was filed as a finding rather than as work, and it explains
its own closure: the silent-green it guards against has **never been
observed on this repository**. It is derived from a property of
`unittest discover` (a directory whose modules do not match the pattern
prints `Ran 0 tests ... OK` and exits 0, verified on CPython 3.12), not
from an incident.

And the fix is the expensive kind. The three copies are not laziness —
the body above establishes that the hosted jobs *cannot* call
`crates/pncad-py/run-python-tests.sh` (it builds through
`local-scripts/with-build-slot.sh`, which every hosted job deletes at
checkout by design, and stages a cdylib rather than installing the
wheel the hosted rows are about). Lifting a shared runner into
`scripts/` then trips `scripts/check-ci-mirror-parity.py`'s claim 1 and
requires moving the seam: either `local-scripts/ci-local.sh` calls the
shared runner directly and `run-python-tests.sh` becomes staging only,
or the parity claim learns to resolve one hop through a
non-`scripts/` script. A developer tool's contract and a parity gate,
for a duplication of about six lines guarding a thing that has not
happened.

**What stays true and needs no action:** the guard itself is present and
correct at all three sites — `.github/workflows/ci.yml:3276`,
`.github/workflows/nightly.yml:683`,
`crates/pncad-py/run-python-tests.sh:39` — so the protection this item
is about is in place three times over. Only the duplication closes.

**Reopen trigger:** a fourth copy, or any one of the three drifting from
the others. The drift risk is the real cost of three copies, and it is
the same risk `nightly-pin-reading-idiom-four-copies` is open on — that
item stays open, and it is the one where a copy has already been
observed silently broken (`c5263958`).
