---
id: criterion-lane-asymmetry-argued-in-four-prose-homes
kind: issue
title: the criterion lane's hosted-only argument now lives in four prose copies and no machine-read one
status: open
opened: 2026-09-11
refs: [criterion-selftest-nightly-only, 2330]
---


Disclosed by the unit that deleted `scripts/criterion-emit.py`'s
`MIRROR_EXEMPT` entry. Flat in count, worse in kind, and worth saying out loud
rather than leaving as an accounting the next reader has to do.

## The finding

One argument — *the criterion LANE is hosted-only because a developer box's
milliseconds committed as a trend point are not comparable, while the HARNESS
and its guard are not one-sided* — is now stated in four places:

* `local-scripts/ci-local.sh`, above the mirrored selftest row
* `.github/workflows/nightly.yml`, at the `criterion` job's `NO LOCAL MIRROR`
  header
* `docs/perf-data/criterion/README.md`, under "Running it yourself"
* `.github/workflows/ci.yml`, at the per-PR row

Before PR 2330 there were four homes too, but one of them — the
`MIRROR_EXEMPT` entry — was **machine-read**: `check-ci-mirror-parity.py`'s
claim 1 required the asymmetry it described to still exist, and reported an
error when it stopped. That entry's deletion was forced (both halves name the
path now, which is exactly the expiry the table is built around), so nothing
here is a regression. What was lost is that one of the four copies could go
stale loudly.

## Why it matters

Every one of the surviving four is prose that nothing reads. The
`demos/render-uv.sh` note still sitting in `MIRROR_EXEMPT`'s comments records
what that costs: an entry described a ci.yml row retired weeks earlier and
stayed true-sounding until a widened population expired it.

## The shapes

1. **Accept it and cut the copies to two** — the local row (which has to
   explain why it runs the guard and not the measurement) and the nightly job
   (which has to explain why it has no local half at all, for claim 9). The
   README and the ci.yml row would point at one of those rather than restate
   it. Cheapest, and it does not buy a machine reader.
2. **Give the lane-level asymmetry a checked home.** `PAIR_EXEMPT` and
   `MIRROR_EXEMPT` both key on things that exist in both halves' text; a
   "this JOB is hosted-only and here is why" assertion is what claim 9 already
   does by requiring a sentence at the job key. Extending claim 9 to require
   that the sentence NAME the local row it delegates the guard to would make
   the two-copy version above enforceable rather than tidy.
3. **Nothing.** Four prose copies of a stable argument is not a defect
   anybody is currently paying for. State that and close.

This is an argument for the orchestrator to settle, not a lane's call.
