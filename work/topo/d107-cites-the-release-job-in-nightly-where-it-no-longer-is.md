---
id: d107-cites-the-release-job-in-nightly-where-it-no-longer-is
kind: issue
title: D107's pointer and its verification recipe both say the release-profile job lives in nightly.yml; it is back in ci.yml
status: open
opened: 2026-09-12
---


Filed 2026-09-12 by the S-TCOST orchestrator at the merge of PR 2434,
which restored the job. Routed rather than fixed across the fence:
`work/topo/D107.md` is TOPO's item and one-file-one-item makes two
programs editing it a merge conflict by design.

## What changed under it

S-TCOST units C1/C2/C3 moved three checks to the nightly on 2026-09-03,
arguing them in **billed Actions minutes** — a currency that stopped
existing when `evgunter/cad` went public that same day. PR 2434 re-costed
them on wall clock, found none of the three anywhere near the critical
path, and **restored all three to the pull-request gate**. The
`release-corruption` job is back in `.github/workflows/ci.yml` and its
nightly copy is deleted, along with the tombstone comment D107 quotes.

## The three sites in D107

1. **The body's pointer** — *"That job is the `release-corruption` job,
   `name: corrupt input (release profile)`, and it lives in
   `.github/workflows/nightly.yml`"*, followed by a quotation of the
   `ci.yml` tombstone comment beginning *"THE `corrupt input (release
   profile)` JOB STOOD HERE, AND HAS MOVED TO…"*. **Both halves are now
   false**: the job is in `ci.yml` and the tombstone is deleted.
2. **A second pointer** further down, corrected once already when the job
   moved the other way, naming `nightly.yml`.
3. **A row in the item's own verification table**, whose check is
   `grep -n "corrupt input" .github/workflows/*.yml` and whose expected
   answer is `nightly.yml`. That recipe still runs and now returns the
   opposite of what the table says.

Site 3 is the one that matters: it is an executable check with a written
expectation, so it is not merely stale prose — it is a recipe that
**reports a failure** to anyone who runs it.

## What this does and does not change about D107 itself

**The finding is untouched.** D107's substance is that `kemr: 0` prints
before the floors and is not on the CI board because libtest captures a
passing test's stdout and the release-profile job passes no
`--nocapture`. **That is still exactly true** — the job's steps were
restored unchanged, and it still passes no `--nocapture`. Only the job's
address moved.

Its sentence *"That is a `nightly.yml` question and `ci.yml` is Track F's
F8 and Track G's G-a, so it is named here rather than changed"* had a
routing consequence attached to the address, and that consequence has
now inverted: it is a `ci.yml` question again. Whether that changes who
D107 hands the work to is TOPO's call, not this row's.

## Related

`work/tcost/nightly-demotions-c1-c3-were-bought-with-billed-minutes`
(closed at PR 2434) carries the re-cost and the wall-clock readings.
