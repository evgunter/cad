# `work/` — the tracker

Every stream of work in this repo is a file here, and nothing about
what is open, who owns it, or what waits on Ev lives anywhere else.
Read `work/STATUS.md` (generated, see below) for the whole board;
read a program's `log.md` tail for its narrative; run
`python3 scripts/work.py status` in any checkout for the live view.

The tracker is version control and nothing else: no GitHub issues,
projects, labels or milestones. Filing, claiming, blocking and closing
are all commits. GitHub PRs remain the review and merge vehicle, and
the one channel to Ev (below).

## Layout

```
work/
  README.md            this contract
  STATUS.md            GENERATED on main by CI — never hand-edited
  issues/<name>.md     issues no program owns yet (kind: issue)
  <program>/
    program.md         the program: charter, prefix, band, territory
    plan.md            the plan (narrative; present state only)
    log.md             the log (append-only narrative; its tail is
                       the program's story, never its slate)
    process-observations.md
                       code-quality only: the C1–C27 observations
                       (narrative, unparsed)
    <ID>.md            one file per item: unit, issue or ruling
```

A program's directory name is its id. An item's file name is its id.
`scripts/work.py lint` enforces every rule on this page and runs on
every CI tier; an item that lint rejects does not merge.

## The header

Every file except `plan.md`, `log.md`, `process-observations.md`,
`README.md` and `STATUS.md` opens with a YAML front-matter block. The parser is a deliberately
small subset: `key: scalar` and `key: [a, b, c]` only, no nesting, no
multi-line values, no anchors. Anything else is a lint error.

```yaml
---
id: MESH-12
kind: unit                 # program | unit | issue | ruling
title: the saturated span refuses at the parse
status: spec               # see the vocabularies below
parent: S-MESH-slate       # optional; another item's id
blocked_on: [D303, 1601]   # item ids, or PR/issue numbers as ints
rides_with: D304           # optional; the row this finding travels with
pr: 1605                   # the PR carrying the unit (int)
branch: mesh/12-saturated-span
needs_ev: true             # a question for Ev is open on an [ev] PR
opened: 2026-09-02
closed:                    # date; required once status is closed
refs: [S330, 1588]         # related items or numbers, no semantics
track: R                   # code-quality only: the track letter
github: 1601               # migrated GitHub issue number, if any
---
```

Program headers carry, in addition: `area` (`kernel`, `api`, `gui`,
`infra`), `prefix` (the branch prefix, the #396 convention), `tag`
(the away-channel role tag), `ab_band` (the A/B ordinal band, claimed
in `docs/MODEL-AB-LOG.md`), `paths` (territory globs), `keep_out`
(prose pointers, one string each), and `blocks` (id blocks a program
allocates from, code-quality only).

Unknown keys are lint errors. Add a key by adding it to the script's
schema in the same PR that first uses it.

## Vocabularies

**kind**: `program`, `unit` (a dispatchable piece of work with a spec
and a PR), `issue` (a defect or finding, not yet a unit), `ruling` (a
question only Ev answers; never work).

**status** of an item: `open` → `spec` → `dispatched` → `review` →
`closed`, plus `parked` (waits on a named trigger, so `blocked_on`
must be non-empty). A ruling is `open` or `closed`. A program is
`open` or `closed`; a closed program may hold only closed items.

## Rules

- **One file, one item.** Two programs editing one item is a merge
  conflict, and that is the cross-program handoff surfacing, not a
  bug. Re-parent or re-home by editing the header, never by copying.
- **Ids are stable.** An item keeps its id for life; a program keeps
  its directory. Migrated code-quality rows keep the row ids they were
  cited by (`D102`, `S330`, `C15`).
- **A rides-along is its own file** with `rides_with:` naming its
  carrier. Closing the carrier does not close the passenger; lint
  refuses a live passenger on a closed carrier.
- **References resolve.** Every id in `parent`, `blocked_on`,
  `rides_with` and `refs` names a file that exists. Ints are PR or
  issue numbers and are not checked.
- **Territory is a glob list** on the program, and every glob matches
  at least one tracked path. `scripts/work.py territory --base main`
  reads a branch's prefix and its diff and names every path another
  program owns. It warns; it does not block.
- **No plan or log outside `work/`.** `docs/*-PLAN.md` and
  `docs/*-LOG.md` are lint errors, so a session writing to the old
  path fails loudly. (`docs/MODEL-AB-LOG.md` is an experiment log, not
  a program's, and is the one named exemption; it leaves `docs/` when
  the experiment concludes.)
- **Specs keep their lifecycle.** `docs/<ID>-SPEC.md` binds an
  implementer for one unit and is deleted at merge per
  `docs/DOC-LEDGER.md`; the item file is the record that survives.
- **`STATUS.md` is written by CI only.** A workflow regenerates it on
  every push to main and commits it from the Actions token. Nothing
  else writes it, so no branch conflicts on it; if you want the view
  on a branch, run `status` and read the terminal.

## Ev's channel

Ev does not edit files. Anything that needs Ev — a design fork, a
ruling, a plan ratification, a question — is a PR whose title starts
with **`[ev]`**, and the item that asked sets `needs_ev: true`. The PR
is not named in the item: which PR carries the question is one
`git log` away, and the item usually exists before the PR does. Ev
answers in the PR's comments; the agent edits the item and the docs,
merges, and clears the flag. `STATUS.md` lists every open `needs_ev`
oldest first, so the two views (the PR list filtered on `[ev]`, and
the tracker) always name the same set.

State-sync rides the unit's PR as before (item header updates, log
entries); conversations for Ev get their own `[ev]` PR.

## The script

```
python3 scripts/work.py lint                  every rule above; CI runs it
python3 scripts/work.py status [--program P]  the render, to stdout
python3 scripts/work.py render                the render, to work/STATUS.md
python3 scripts/work.py new <id> --kind K --title T [--program P]
python3 scripts/work.py set <id> key=value [key=value ...]
python3 scripts/work.py territory --base <ref> [--branch <name>]
python3 scripts/work.py --selftest
```

`set` takes `key=` to clear a field and `key=[a,b]` for lists.
