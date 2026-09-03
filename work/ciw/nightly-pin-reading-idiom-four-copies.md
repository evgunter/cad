---
id: nightly-pin-reading-idiom-four-copies
kind: issue
title: nightly.yml reads ci.yml's tool pins with a sed idiom that is now in four places and breaks silently on a second match
status: open
opened: 2026-09-03
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
