---
id: polish
kind: program
title: POLISH — viewer chrome that shows the right information in a rougher shape than it could
status: ready
opened: 2026-10-01
area: gui
prefix: polish/
tag: (POLISH orchestrator)
ab_band: 10200-10299
paths: [crates/viewer/src/*, crates/viewer/tests/*, crates/viewer/README.md]
keep_out: [opened by CHROME's 2026-10-01 cut on Ev's guideline (in chat) - CHROME keeps the rows that change WHAT the chrome shows and this program holds the rows that change only HOW it shows information the reader already has - it SHARES crates/viewer with CHROME and the VIEW successors by design, shared ground is legitimate per work/README.md's 2026-09-20 rule and what is owed is awareness while a lane is live, so run scripts/work.py territory on your branch]
priority: P4
---

**Rows that change how the chrome shows information, not what it
shows.** Ev's guideline (in chat, 2026-10-01), given while ruling the
field-edit width fork: skip the "polishing" rows and work the ones
that change what information is displayed. Where a row's information
is all reachable today, by scrolling, wrapping or reading a longer
sentence, its fix is presentation and it lives here.

The line is fuzzy, and Ev drew it that way: text that wraps or lands
nowhere near what it describes, so a reader can miss it, is an
information problem and stays in CHROME. Work already implemented
under this heading before the cut is kept.

Two of these rows carry Ev's ruling on their design fork already
(PRs 3606 and 3607); they are dispatchable as written, with no
question left open. The width contract's partial work is on the
branch `chrome/width-contract`, stopped mid-lane, never PR'd.

Review posture: CHROME's (style review; a correctness arm only where
the failure mode is a confident wrong answer; no A/B duals). The band
is claimed and expected to stay empty.
