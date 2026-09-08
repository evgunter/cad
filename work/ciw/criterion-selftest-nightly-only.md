---
id: criterion-selftest-nightly-only
kind: issue
title: criterion-emit.py --selftest is invoked only from nightly.yml - a guard exercised only on a schedule
status: open
opened: 2026-09-07
refs: [2124]
---


Disclosed by the unit that wired `scripts/opt-level-calibrate.py --selftest`
into the per-PR gate (PR 2124). It is the one other hit of that unit's sweep,
and it is a live instance of the same class rather than a tidy-up.

## The finding

`scripts/criterion-emit.py --selftest` is invoked from exactly one place:
`.github/workflows/nightly.yml:1835`, immediately before the real `emit`. No
row in `.github/workflows/ci.yml` and no row in `local-scripts/ci-local.sh`
runs it.

So a PR that breaks that selftest — or breaks the parser it exercises — merges
green. The break surfaces at the next scheduled fire, to nobody, which is
`work/ciw/nightly-demotions-have-never-run`'s sentence applied one level in.
The siting is not accidental: the comment above the row cites
`scripts/opt-level-calibrate.py` as its precedent for keeping such rows in the
nightly rather than the merge gate. That precedent has now moved — the
calibrator's selftest runs per-PR in `discipline`, mirrored in `ci-local.sh` —
so the argument the comment leans on no longer says what it says.

## Why it is not a one-line move

`scripts/criterion-emit.py` is declared hosted-only in
`scripts/check-ci-mirror-parity.py`'s `MIRROR_EXEMPT`, on a reason about the
measurement harness ("what does not mirror is stapling an environment block to
a local reading and committing it"). Naming the selftest in both halves expires
that confession, exactly as it expired the calibrator's — so the fix is a row
in `ci.yml`'s `discipline`, a mirrored row in `ci-local.sh`, and the deletion
of the `MIRROR_EXEMPT` entry, with the hosted-only reason re-stated at the
local row where it belongs. Whether the nightly's own copy stays is the real
question: it guards an append to `docs/perf-data/criterion/`, but that append
runs on `main`, and `main` is reached through the gate that would now carry the
selftest.

## Not the same as claim 4's second arm

`check-ci-mirror-parity.py`'s claim 4 grew a second arm in the same unit: it
refuses a `--selftest` mode no WORKFLOW invokes. This row is invoked by
`nightly.yml`, so the arm is silent on it and correctly so: "invoked from a scheduled workflow only" is a
judgement about siting, and the checker has no siting vocabulary for it that
would not also condemn `nightly.yml`'s legitimately nightly-only guards.
