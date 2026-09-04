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
`closed`, plus two ways of being not-now:

- `parked` — **waits on a named trigger**, so `blocked_on` must be
  non-empty and every id in it must resolve. The trigger is an item or
  a PR: something that can fire, and that lint can see has fired.
- `deferred` — **ratified as not-now**, with the ratification cited in
  the body. No `blocked_on`: a deferred row is not waiting for anything
  to happen, it has been decided against for now, and lint refuses a
  `blocked_on` on one (a row that waits on a trigger is `parked`).
  **The citation is prose and nothing checks it.** A ratification lives
  in a README clause or a design doc, not in an item id, so there is no
  reference for `lint` to resolve and no field pretending otherwise; a
  reviewer reads the row and judges whether the ratification it names
  says what it claims. What the tracker guarantees about a deferred row
  is only this: it is not dispatchable, and it is not blocked either.

Neither counts as available work: `STATUS.md` gives each its own column
so a not-now row can never be read off the board as dispatchable, and
neither is listed as stale for going untouched. A ruling is `open` or
`closed`. A program is `open` or `closed`; a closed program may hold
only closed items.

## Rules

- **One file, one item.** Two programs editing one item is a merge
  conflict, and that is the cross-program handoff surfacing, not a
  bug. Re-parent or re-home by editing the header, never by copying.
  **An item's directory is the program that owns it** — `work.py` reads
  ownership from nowhere else — so a program claiming another's item
  MOVES the file into its own directory in the PR that claims it,
  keeping the id, and sets `parent:` to the unit that carries it. This
  is what `work/code-quality/` is for: findings wait there until a
  program claims them, and a claim empties that row out of it. A
  `keep_out` clause saying a claimed row stays where it was is the
  thing to delete.
- **Ids are stable.** An item keeps its id for life; a program keeps
  its directory for as long as it is open. Migrated code-quality rows
  keep the row ids they were cited by (`D102`, `S330`, `C15`).
- **A closed program's directory is deleted.** `work/` tracks work
  still to be done, not work that has been done, so once a program
  closes — its exit walk ratified, or Ev's ruling that it needs none —
  `program.md`, `plan.md` and `log.md` go, and so does the ratified
  exit walk; the deletion is recorded in `docs/DOC-LEDGER.md` with the
  SHA they are recoverable at, and that ledger entry is the program's
  done-state of record. Residue is re-homed to a live program or to
  `work/issues/` before the sweep, not left behind in the closed
  directory. **That sweep sees items, not sentences**: a residue a
  lane discloses inside its own item's `## Closed` prose reads as a
  record of work done, not as an open thread, so it is invisible to
  the re-homing and dies with the directory. Disclosing a residue is
  therefore not scheduling it — **give it its own file at the moment
  you disclose it**, on this program's slate or in `work/issues/`, and
  let the Closed section point at that file.
- **A rides-along is its own file** with `rides_with:` naming its
  carrier. Closing the carrier does not close the passenger; lint
  refuses a live passenger on a closed carrier.
- **References resolve.** Every id in `parent`, `blocked_on`,
  `rides_with` and `refs` names a file that exists. Ints are PR or
  issue numbers and are not checked.
- **A fired trigger is not a blocker.** A `parked` row whose
  `blocked_on` names a CLOSED item has had its trigger fire, and a
  resolving reference is no evidence the row is still blocked. Two
  cases, because the two say different things:
  - **every blocker closed — a lint ERROR.** `parked` is simply false
    of the row and the board is lying about it. Re-park it on what
    actually gates it, open it, or defer it.
  - **a fired entry beside a live one — a lint WARNING.** The row is
    genuinely still blocked, so its status is true and only the entry
    is stale; prune the fired entry.

  The cost of the error is real and was accepted deliberately (Ev,
  2026-09-04): one-file-one-item means the program closing a trigger
  cannot un-park another program's rows in the same PR, so a closing
  PR can red `main` for rows it does not own. The answer is to fix the
  stale rows, not to soften the check.
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
merges, and clears the flag. Whoever opens an `[ev]` PR arranges to be
woken by comments on it — the away-channel monitor locally, a PR
subscription on a remote box — because the answer arrives as a comment
and a question nobody is listening to has not been asked.
`STATUS.md` lists every open `needs_ev` oldest first, so the two views
(the PR list filtered on `[ev]`, and the tracker) always name the same
set.

State-sync rides the unit's PR as before (item header updates, log
entries); conversations for Ev get their own `[ev]` PR.

## The script

```
python3 scripts/work.py lint                  every rule above; CI runs it
python3 scripts/work.py status [--program P]  the render, to stdout
python3 scripts/work.py render                the render, to work/STATUS.md
python3 scripts/work.py new <id> --kind K --title T [--program P] [--set k=v]
python3 scripts/work.py set <id> key=value [key=value ...]
python3 scripts/work.py territory --base <ref> [--branch <name>]
python3 scripts/work.py --selftest
```

`set` takes `key=` to clear a field and `key=[a,b]` for lists.
