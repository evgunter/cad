---
id: mirror-job-is-the-whole-wall-for-tracker-prs
kind: issue
title: CI half parity is ~90% of the wall on docs/tracker PRs: the 61s step is fixed, the remaining 121s is not measured
status: open
opened: 2026-09-12
---


Filed 2026-09-12 by S-TCOST on Ev's direction in chat, out of that
program's latency work. **The job is right to run on every tier and this
row does not propose gating it** — it proposes making it cheaper, and
names the step that would pay for itself first.

## The observation

On a docs- or tracker-tier pull request every heavy job skips, so the
run's wall IS the `CI half parity + gate wiring (every tier)` job. This
repository produces those constantly — every work-tracker PR is one, and
Ev named #2414 as an instance while it was in flight.

Measured over the 32 most recent completed pull-request runs of `ci.yml`:

| figure | value |
|---|---|
| mirror job, all sampled runs | **176 s median** (n = 32), range 19-185 s |
| sampled runs at docs/tracker tier (<= 5 live jobs) | **11 of 32** |
| the job's share of the wall on those | **93-95 %** |

Eight of the eleven, run by run: 180 s of 192, 174 of 185, 176 of 189,
176 of 188, 176 of 187, 164 of 173, 180 of 192, 177 of 186.

## Where the time goes

Step medians over the same 32 runs, read from the jobs API (they sum to
172 s of the 176 s job, so this accounts for it):

| step | median |
|---|--:|
| **render lane parity (the helper knows the lanes render.yml declares)** | **61 s** |
| **apt preamble selftest (the narrowed update survives a bad index)** | **31 s** |
| probe type-check loop citations | 21 s |
| change filter selftest (the docs tier fails open) | 17 s |
| viewer module kinds (vocabulary/driver boundary) | 14 s |
| CI half parity (both halves name the same checks) | 14 s |
| everything else (12 steps incl. checkout, work tracker lint, territory) | <= 4 s each |

**The top two are 92 s — 52 % of the job.**

## The first one is FIXED IN THIS PR, and the cause was one line

`scripts/check-render-lane-parity.py` runs twice in that step: the
selftest, then the check. Locally:

- `--selftest` — **60.6 s**
- the real check — **0.09 s**

So the step is its selftest. And the selftest is **one of its 24
mutants**: at `:700-710`, *"helper hangs instead of answering"* replaces
the helper's table print with `sleep 120`, and `helper_answers`' own
`timeout=60` (`:356`) fires. **The selftest waits out a real sixty
seconds on every CI run to prove that a timeout constant fires.** The
other 23 mutants and the check together are under a second.

**The claim is real and stays**: a helper that hangs must be reported
rather than waited on forever, and that mutant is what proves it. What
was negotiable is the CONSTANT under test. `helper_answers` and `check`
now take the ceiling as a parameter — `HELPER_TIMEOUT_S = 60.0` in
production, `SELFTEST_HELPER_TIMEOUT_S = 5.0` for the mutant run — so
the same mutant proves the same arm against a ceiling it does not have
to outlast. 5 s and not 0.5: a healthy print answers in 0.07 s, so this
keeps a ~70x margin against a loaded runner rather than trading one
flake class for another.

**Measured, local:** `--selftest` **60.6 s -> 5.6 s**, 24 mutants, the
hang mutant still RED with the same message; the real check unchanged at
0.07 s. No other caller exists — `check()` and `helper_answers()` are
invoked only from this file and the two CLI entries in `ci.yml` and
`local-scripts/ci-local.sh`, neither of which changed.

Expected effect on the job: **~176 s -> ~121 s**, and on a tracker PR's
wall about the same, since the job IS that wall. A hosted before/after
is this PR's own runs.

## WHAT IS LEFT: the second step, not diagnosed

`scripts/apt-install.sh --selftest` at 31 s. Its header says it points
apt at a scratch tree with `-o Dir::*` and touches no system file, and
that the row proves a narrowed `apt-get update` survives a mirror a full
one dies on. Whether 31 s is an unavoidable apt round trip or another
constant being waited out is unmeasured, and is the next thing to look
at after the first.

## Why the job cannot simply be gated

Its own header is explicit: **it carries no `if:` deliberately, and that
is enforced** by `scripts/check-ci-mirror-parity.py`. Three of its steps
are sited there precisely because the job runs when every other job is
skipped — the change filter's own selftest most of all, since a diff that
wrongly widens the docs branch would otherwise skip the job that catches
it. So the lever is step cost, not step count, and a fix that gates the
job would be the defect the job exists to prevent.

## What remains on this row

With the 61 s step at 5.6 s, the job is still ~121 s and still ~90 % of a
tracker PR's wall. The ranked remainder, from the table above:
`apt-install.sh --selftest` at 31 s, `probe type-check loop citations` at
21 s, `change filter selftest` at 17 s, `viewer module kinds` at 14 s and
`CI half parity` itself at 14 s. Whether any of the others is another
constant being waited out, or is real work, is unmeasured — the first one
was found by timing the script's two halves separately, which is the
cheapest thing to try on each.

## Scope

`.github/workflows/*` and `scripts/check-*.py` are both this program's,
so the whole of the first fix is in territory. No S-TCOST row depends on
this and nothing is blocked on it; it is filed here because CIW owns it
and the finding came from a latency sweep next door.
