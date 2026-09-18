---
id: critical-path-citations-name-a-job-that-is-not-the-pole
kind: issue
title: Five sites name the wrong last job on the critical path, and the run's shape has changed under all of them
status: open
opened: 2026-09-12
---


Filed 2026-09-12 by the S-TCOST orchestrator out of the style review of
S-TCOST PR 2437. That PR corrected **one** of these sites — the one in
`scripts/ci-filter.py`, which is S-TCOST's — and left the rest, which are
CIW's or `docs/`. This row is the remainder and the class.

## The corrected fact

`scripts/ci-filter.py` §WALL CLOCK IS NOT FREE named
`test (interval, eps = default, 1/2)` as the last job on the critical
path. PR 2437 measured thirty hosted runs and it is **not**: the pole is
the **ε = 1e-12** interval row, on all thirty, and *which shard* of it
varies because the count partition reads no timings.

The editor-core extra steps do ride on shard 1 of the first ε row — that
is where they are wired, not where the wall is, and that is probably how
the original claim was arrived at.

## The sites still naming the old job

Swept with `grep -rn "test (interval, eps = default, 1/2)"` over `.md`,
`.yml` and `.py`:

| site | what it says | disposition |
|---|---|---|
| `work/ciw/reinstate-full-configuration-runs.md` | the pole "is `test (interval, eps = default, 1/2)` — the first eps row's shard 1" | live claim, now false |
| `work/ciw/log.md` | same job, same framing, with a reading | narrative; dated by position, but reads present-tense |
| `docs/CI-MINUTES-2026-08.md` (two sites) | "**`test (interval, eps = default, 1/2)`** — the first ε row's…" and "three runs the last job was…" | the dead ledger; annotate, do not quote forward |
| `.github/workflows/ci.yml` | the opt-level table labels `test (interval, default, 2/2)` "the critical-path leg" | live prose beside a live table |

**One site is NOT in this row and is correct as written:**
`scripts/slowest-tests.py` quotes that job's output **VERBATIM from a
named run id**. A verbatim quote of a past run is a dated artefact, not a
claim about today, and re-pointing it would falsify it. That distinction
is the useful half of this row: the fix is per-site and depends on
whether the sentence asserts a present fact or records a past reading.

## The larger half: the run's SHAPE changed, not just the job's name

Three further figures describe a run whose proportions no longer hold,
and they are the reason this is worth a row rather than a rename:

- `.github/workflows/ci.yml` — "the interval lane's slowest leg was
  828 s"
- `.github/workflows/ci.yml` — "test EXECUTION is now ~79 % of run wall
  … ~1060 s critical path"
- `work/ciw/f3-recosting-on-a-public-repo.md` (two sites) —
  "`build + archive (interval)` alone is ~88 % of a 13.75-minute
  critical path"

None is falsified **as a dated historical reading**; all read as
present-tense facts. Against PR 2437's measurement — a single test at
346-660 s inside a 372-740 s leg — each describes the wrong shape of
run. In particular, a claim that the *build* is ~88 % of the critical
path cannot survive a test leg that is itself 372-740 s.

## What this asks for

Per site, one of three, and the sorting is most of the work:

1. **A present-tense claim that is now false** — re-point it at the
   ε = 1e-12 row, or delete it if the surrounding argument no longer
   needs it.
2. **A dated reading** — leave the figure and make the date load-bearing
   in the sentence, so the next reader cannot quote it forward. This is
   what `scripts/slowest-tests.py` already does correctly.
3. **A figure in `docs/CI-MINUTES-2026-08.md`** — that document's
   opening premise is dead and `work/ciw/plan.md` §The 2026-09-04
   re-read forbids quoting any figure from it forward. Annotate, per the
   house spelling already used at three of its headings.

## Related rows, so the next lane does not re-derive the map

- `work/ciw/billed-minute-arguments-survive-across-ci-yml` — the same
  file, the same class, a different currency.
- `work/ciw/eps-klint-and-shard-counts-are-prose` — the eps rows, k-lint
  unifications and shard counts counted in prose in eight places; its
  hit list needs extending with PR 2437's three new prose homes.
- `work/tcost/one-test-is-the-whole-ci-critical-path` — the measurement
  that falsified all of this, and the row that owns the finding.

## What the sweep could not match

One exact job-name string plus four figures read by eye. A sentence
naming the pole without spelling the job ("the last interval leg", "the
critical-path shard"), or a proportion stated without a number, matches
nothing here. `docs/CI-MINUTES-2026-08.md` is large and only its hits for
that one string were read.
