---
id: nightly-pin-reading-idiom-four-copies
kind: issue
title: nightly.yml reads ci.yml's tool pins with a sed idiom that is now in four places and breaks silently on a second match
status: review
opened: 2026-09-03
pr: 1723
branch: ciw/one-pin-reader
---


`nightly.yml` declares `ci.yml` the single source of truth for a pinned
tool version and reads it back with

    sed -n 's/^ *MATURIN_VERSION: *//p' .github/workflows/ci.yml | head -1 | tr -d "\"'"

That idiom now appears **five times in four steps** of
`.github/workflows/nightly.yml` — `:773` and `:1130` and `:1352` read
`NEXTEST_VERSION` (the last with a different quoting escape, which is
itself a small drift), and `:362-363` read `MATURIN_VERSION` and
`TY_VERSION` for the python re-take added by S-TCOST C3.

**What breaks, and it breaks quietly.** `^ *NAME:` matches at ANY
indentation, so it matches the workflow-level `env:` block it is aimed at
AND any `env:` under a job or a step that sets the same name. `head -1`
then takes whichever comes first in the file — not whichever is in
scope — so the day someone pins a tool per-job in `ci.yml` above the
workflow `env:` block, every one of these steps silently installs the
wrong version and the lane goes on reporting green. The `test -n`
guard each site carries catches an EMPTY answer; nothing catches a WRONG
one.

**Not this unit's to fix** — S-TCOST C3 added the fourth and fifth
copies and deliberately copied the established idiom rather than
inventing a sixth spelling inside a CI-posture unit. What a fix wants is
one reader (a `scripts/` helper both workflows call, which also gets the
quoting right once), anchored to the workflow-level block rather than to
"first match", and refusing on more than one match instead of picking.

Sites: `.github/workflows/nightly.yml:362`, `:363`, `:773`, `:1130`,
`:1352`.

## The class has now fired, on main (added 2026-09-04, CIW)

Filed the day before as a hazard. It is no longer hypothetical:

    c5263958  nightly: the gated-suite re-take's pin-read step had
              unbalanced quotes and never ran

Same file, same idiom, a sixth site — and the failure mode is the one
this item names, one layer down: the guard each site carries catches an
EMPTY answer, and this step did not get that far. It was found by a
person reading a log, not by anything in CI.

Sites re-checked on this tree (line numbers moved with the file):
`.github/workflows/nightly.yml:635` and `:636` (MATURIN_VERSION,
TY_VERSION), `:1064`, `:1421`, `:1643` (NEXTEST_VERSION).

## One thing the fix has to clear, and it is small

A `scripts/` helper named by `nightly.yml` trips
`scripts/check-ci-mirror-parity.py`'s claim 1, which requires a
`scripts/` path named by a workflow to be named literally by
`local-scripts/ci-local.sh` too. The mechanism for a legitimate
asymmetry already exists — `MIRROR_EXEMPT`, one entry with a sentence
saying why the local half has no pin to read (it does not install from
`ci.yml`'s pins). That is the whole cost; it is not the seam problem
`python-suite-zero-test-guard-three-copies` faces, which moves a
developer tool's contract.
