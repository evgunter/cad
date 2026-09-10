---
id: pipestatus-after-assignment-in-ci-yml
kind: issue
title: a status capture that cannot fail: PIPESTATUS read after the assignment that clobbers it
status: review
opened: 2026-09-04
branch: ciw/pipestatus-sweep
pr: 2298
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

## Disposition (2026-09-10)

**The sweep is empty and the guard is built.**

1. **Sweep — no hits.** `scripts/check-status-capture.py`, added by this
   unit, is the instrument: 7 `PIPESTATUS` reads across 87 shell files,
   workflows and composite actions, every one taken on the command
   immediately after its pipeline. Line numbers ON THIS BRANCH'S HEAD,
   which inserts 25 lines into `ci.yml` above them: `ci.yml` `:666`,
   `:3106`, `:3597`, `:3632`, `:3642`, `:4806`; `render.yml` `:1183`.
   (On `main` they are `:3081`, `:3572`, `:3607`, `:3617`, `:4781`.)
   The tombstone at `ci.yml:4771-4777` is a comment and is correctly not
   counted.

2. **A mirror check does not gain an arm for it.** Not because the shell
   inside a `run:` is outside `scripts/check-ci-mirror-parity.py` — it is
   not: that file's claim 10 already reads argv out of `run:` bodies, and
   its own header marks that boundary ("CLAIM 10 IS WHERE THE ROSTER
   STOPS AND THE COMMANDS START"). The reason is size and blast radius.
   That file is 4004 lines and has taken a claim in each of the last four
   units; every one of its claims shares one tokenizer, so a change made
   for this property can move the answer of any other. A separate script
   fails alone. It gains one `TIER_BLIND` membership entry (6 lines with
   its comment) and nothing else.

3. **The local half's `klint_gate` is unchanged**, as asked. `ci-local.sh`
   gains only the mirror row for the new check (7 lines).

Residue: `shellcheck-is-not-run` — nothing runs a shell linter, so the
sibling `$?` class (SC2319/SC2320, which shellcheck *does* catch and this
guard deliberately does not) is unguarded, and ~20 `# shellcheck disable=`
markers are unverifiable claims.
