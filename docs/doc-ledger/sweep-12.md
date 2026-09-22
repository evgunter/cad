# Sweep 12 — 2026-09-12: CITE leaves the tracker

Sweep SHA: `116d96c01d4a03084d4701d7d58fb3b5dcf1703b` — the commit
immediately before the deletion (on the closing PR's branch, reachable
from `main` through that PR's merge commit; it is the state in which
CITE's directory is complete, `program.md` reads `status: closed`, and
every row in it is closed), so every path below is recoverable at
`git show 116d96c01d4a03084d4701d7d58fb3b5dcf1703b:work/cite/<FILE>` and
`git show 116d96c01d4a03084d4701d7d58fb3b5dcf1703b:docs/CITE-EXIT-WALK.md`.

CITE — citations, numbering and the paperwork a lane runs on — opened
2026-09-11 in the tracker cut of that day
(`docs/WORK-TRACKS-2026-09.md` addendum 3) and closed 2026-09-12 on Ev's
ratification of `docs/CITE-EXIT-WALK.md` (PR #2405). It claimed **no
paths**, by charter: its repair ground was `work/<program>/*.md`, which
is one-file-one-item ground. Per the sweep-5 rule the directory leaves
whole — `program.md`, `plan.md`, `log.md` and eight item files, all
`status: closed` at the sweep SHA (`C-namespace`, `S176`,
`build-slot-banner-leaks-the-holders-command-line`,
`code-quality-item-quotes-a-viewer-doc-string-that-was-rewritten`,
`d107-release-profile-job-lives-in-nightly`,
`doc-line-citations-rot-silently`,
`loud-skip-marker-row-cites-a-lib-paragraph-that-was-reversed`,
`tracker-file-line-citations-measured`); the other four rows the slate
held were re-homed first, below. **Two PRs** merged, #2397 (the
convention, the repairs and four rulings) and #2405 (the walk).
Infra-and-prose: **no A/B rows were written and no ordinal was spent**;
the band 4000–4099 stays claimed in `docs/MODEL-AB-LOG.md`.

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `cite` | CITE — citations, numbering and the paperwork a lane runs on | 2026-09-12 | this row and the exit-walk row below; the two PRs above; the standing rule at `docs/prompts/implementer-discipline.md` §7 |

### What survived, and where

The program's output is deliberately not in its directory:

- **The rule.** `docs/prompts/implementer-discipline.md` §7 — *"Cite by
  name; line numbers rot. A number may ride along beside the name and is
  allowed to go stale; a bare `file.rs:NNN` is not a citation."* Landed
  2026-09-12 on Ev's wording, cut down by him twice from a three-
  paragraph draft. `docs/prompts/*` is META's territory; landed on Ev's
  word rather than taken.
- **Three repaired rows**, each on its owning program's slate with a
  marked section recording what the repair put in question and leaving
  the ruling to the owner:
  `work/door/viewer-pathverb-all-hand-written-seventeen.md`,
  `work/topo/D107.md`,
  `work/tint/loud-skip-marker-is-a-hand-kept-idiom.md`.
- **One leak closed.** `local-scripts/with-build-slot.sh` no longer
  records the caller's command line in the holder file, so the banner a
  waiting caller prints cannot carry another lane's test filter or
  scratch path between blinded review lanes.

### The measurement, because nothing else now carries it

`tracker-file-line-citations-measured` is deleted with the directory and
its figures are load-bearing for §7, so they are restated here. Taken
2026-09-09 at `0762714fd` over every `<path>:<line>` in every
`status: open` row of `work/**/*.md`, bare basenames resolved through
`git ls-files`:

| | count |
| --- | --- |
| citations, in 317 open rows across 22 programs | **1,508** |
| already anchored (a backticked identifier or quoted phrase within one line) | **1,446 (96%)** |
| unanchored | 62 (4%) |
| file missing or line past EOF | 31 (2.1%) |
| basename ambiguous, not checkable that way | 204 |

Per program the anchored figure runs 90–100%. **That 96% is why §7 is a
ratification of existing practice rather than a proposal**, and why the
walk declines both a sweep and a gate: a line-range check sees only the
2.1%, while VIEW's three hand-sweeps found roughly three quarters of the
citations they touched pointing at the wrong *subject*, nearly all of it
inside the column such a check passes.

### Residue re-homed before the deletion

The moves happened in #2397 and #2405, not in the deleting commit; the
deleting commit finds the directory already emptied of live work.

| item | to |
| --- | --- |
| `S351` | `work/trim/` — the placement rule it watches is in `crates/geom-brep/src/nurbs_iso.rs`, TRIM's `paths`. Checked at the move and **not fired**: both pointers resolve and both cite `nurbs_iso`'s module docs by name |
| `d321-row-number-reissued` | `work/meta/` — overtaken on both halves by sweep 11; the one surviving thing, a `work.py` check that an id is never reissued, is META's |
| `lane-scratchpad-is-shared-between-worktrees` | `work/meta/` — `deferred` by Ev (the rule is not project-specific); with the `memories/` half ruled out, `docs/prompts/*` is the only document left in play and it is META's |
| `no-local-script-builds-all-four-cargo-workspaces` | `work/ciw/` — the fix is in `local-scripts/*`, CIW's `paths`; CITE never started it |

### The four inbound pointers this sweep rewrote

All four named `work/cite/` paths and none was a header `refs:`, so no
row's references broke. They were repointed at what survives, in the
commit at the sweep SHA:

- `work/door/viewer-pathverb-…` and `work/topo/D107.md` cited
  `work/cite/S176.md` as the argument for cite-by-name → now cite §7,
  which states the rule;
- `work/topo/D107.md` cited `work/cite/d107-release-profile-job-lives-in-nightly`
  for the two line numbers that had drifted → now names it as a record
  recoverable through this ledger;
- sweep 11's two entries cited `work/cite/plan.md` for `d321`'s
  disposition → now name `work/meta/d321-row-number-reissued.md`.

### Honesty notes

- **Four of the twelve rows were re-homed unstarted, not finished.** A
  program that opens twelve and works eight did not finish twelve; the
  four moved because they were somebody else's.
- **`S176`'s `Verdict:` was blank for three weeks and CITE filled it**,
  on the measurement rather than on a ruling from Ev. The row says so in
  its own words, and the row is now recoverable only at the sweep SHA.
- **The walk tables the plan's slate rather than quoting it.** The
  plan's table has a `where the work lands` column that is about
  dispatch and is not a criterion. The plan is recoverable at the sweep
  SHA.
- **The routing list the plan promised was never produced**, and the
  walk argues it should not have been: Ev authorised repair-in-place at
  the first ask, and the row that motivated the fence
  (`loud-skip-marker-…`) exists precisely because §6 reported the same
  rot twice and filed nothing both times. The distinction CITE leaves
  behind is that **repairing what a row POINTS AT is not the same act as
  ruling on what it CLAIMS**, and only the second needs the owner.
