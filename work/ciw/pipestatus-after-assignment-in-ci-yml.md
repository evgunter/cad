---
id: pipestatus-after-assignment-in-ci-yml
kind: issue
title: "a status capture that cannot fail: PIPESTATUS read after the assignment that clobbers it"
status: open
opened: 2026-09-04
---

**Filed by M10-7 (PR 1725) for CIW, whose territory `.github/` is.** One
instance is FIXED in that PR because it disarmed a gate the unit had to
report on; the sweep for others is CIW's, and this item is the citation.

## The shape

```bash
set -uo pipefail
status=0
some_command 2>&1 | tee out.log || status=$?
status=${PIPESTATUS[0]:-$status}     # <- always 0
```

`PIPESTATUS` is rewritten by **every** command, including the assignment
`status=$?` on the line above — which succeeds — so by the time the
second line reads `PIPESTATUS[0]` it is that assignment's own `0`. The
`:-` default never fires, because `0` is a perfectly good value. `status`
is `0` no matter what the pipeline said, and every non-zero arm of the
`case` that follows is unreachable.

Reproduced in five lines:

```bash
set -uo pipefail
status=0
(exit 2) 2>&1 | tee /dev/null || status=$?
status=${PIPESTATUS[0]:-$status}
echo "$status"            # prints 0; should print 2
```

The correct spelling reads `PIPESTATUS` on the pipeline line itself,
which is what the two `test` jobs in `ci.yml` already do
(`.github/workflows/ci.yml:2359`, `:2830`):

```bash
some_command 2>&1 | tee out.log
status=${PIPESTATUS[0]}
```

## The instance found, and what it cost

`.github/workflows/ci.yml`, the `driver K-telemetry lint (E6 evidence —
rule 1 GATES, rules 2/3 advisory)` step of the `k-lint (gate)` job
(the broken line was `:3970` before M10-7's fix).

It landed inside M10-6's own PR (#1685): `eab6e3acc` added the step
WITHOUT a pipe, where the plain `|| status=$?` was exact; `eeb28648b`
added the `| tee` and the broken capture eight commits later. Both are in
that PR, so **the row has never been able to fail on `main`** — not on
findings (exit 2, the E6 re-open trigger) and not on harness breakage
(exit 1).

That was not theoretical. On run 33828394312 the step logged

```
k-lint: ../../target/k-fresh/driver/k-eps-1e-6.csv:3: malformed sweep row
  (harness breakage): driver/slab_narrow,witness_at_mid_parameter,0e0,
  1e-100,1e-50,symbolic_zero
```

and stopped — no per-file line, no TOTAL, zero driver samples linted at
any ε — and the step still reported **success**. Two defects composed:
k-lint did not know a new outcome token (fixed in the same PR), and the
gate that would have said so could not fail.

## What is asked of CIW

1. **Sweep `.github/workflows/` for the pattern.** The grep that finds it
   is `PIPESTATUS` on a line that is not the line immediately after a
   pipeline — at this writing `ci.yml:3970` was the only one, and
   M10-7 fixed it, but the sweep is what makes that a fact rather than a
   sample.
2. **Consider whether a mirror check can see this class at all.**
   `scripts/check-ci-mirror-parity.py` compares the NAMES and gate modes
   of rows, not the shell that implements them, and `ci-local.sh` says so
   at this very row's local half. A CI step whose `case` arms are
   unreachable is a gate that reports rather than gates, and nothing in
   the current instrument notices.
3. **The local half needs no change and should not get one.**
   `local-scripts/ci-local.sh`'s `klint_gate` does not `tee`, so its
   plain `$?` is exact. A comment saying so is in place, so that the two
   halves' asymmetry is a decision rather than drift.
