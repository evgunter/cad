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
  issues/README.md     this directory's signpost (unparsed)
  issues/<name>.md     issues no program owns yet (kind: issue)
  <program>/
    program.md         the program: charter, prefix, band, territory
    plan.md            the plan (narrative; present state only)
    log.md             the log (append-only narrative; its tail is
                       the program's story, never its slate)
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
priority: P1               # P0..P4; see Priority below
cost: H                    # E | D | H; what the row costs to do
parent: S-MESH-slate       # optional; another item's id
blocked_on: [D303, 1601]   # item ids, or PR/issue numbers as ints
rides_with: D304           # optional; the row this finding travels with
pr: 1605                   # the PR carrying the unit (int)
branch: mesh/12-saturated-span
needs_ev: true             # a question for Ev is open on an [ev] PR
opened: 2026-09-02
closed:                    # date; required once status is closed
refs: [S330, 1588]         # related items or numbers, no semantics
track: R                   # historical: the code-quality track letter,
                           # on the rows that still carry one
github: 1601               # migrated GitHub issue number, if any
---
```

Program headers carry, in addition: `area` (`kernel`, `api`, `gui`,
`infra`), `prefix` (the branch prefix, the #396 convention), `tag`
(the away-channel role tag), `ab_band` (the A/B ordinal band, claimed
in `docs/MODEL-AB-LOG.md`), `paths` (territory globs), `keep_out`
(prose pointers, one string each), and `blocks` (id blocks a program
allocates from). **No open program carries `blocks`, and none should**:
an item's id comes from its name, not from a per-track number block.
The block scheme belonged to the 2026-08 findings register and left the
tree with it (`docs/doc-ledger/code-quality-leaves-the-tracker.md`).

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
`closed`.

**status** of a program says what the TRACK is, and the fact it exists
to carry is **whether an orchestrator is on it** — the one thing the
row counts beside it cannot show, because a session with a full slate
and nothing dispatched yet looks exactly like an abandoned one:

- `ready` — **no orchestrator, and something to pick up**: at least
  one row is dispatchable (`open` or `spec`, as Track size counts
  them). This is the state a successor session scans the board for.
- `active` — **an orchestrator holds this track.** Nothing in the tree
  can confirm or refute that, so it is the program's own word and lint
  takes it as given; the orchestrator sets it when it picks the track
  up and clears it when it hands the track back.
- `blocked` — **no orchestrator, and nothing to pick up**: every live
  row is in flight, parked or deferred. Lint checks the half of that
  claim the tree can see (no dispatchable row, and at least one live
  one).

**There is no `closed`, because a program that closes is deleted**
(the closing rules below): a closed program is an ABSENT one, and a
status saying so would only ever describe the gap between the exit
walk being ratified and the sweep that removes the directory. A track
in that gap is still `active` — its orchestrator is writing the walk —
and it holds no dispatchable row, which is why nothing here requires
one of an `active` track.

**A blocked track never has an orchestrator, by construction.** An
orchestrator that has run out of non-blocked units does not sit on the
track waiting for its triggers to fire: it cuts the blocked rows into
a new program (Track size's splitting rules; the closing rules'
re-homing discipline), closes what it has finished, and leaves the new
program `blocked` with nobody on it. Otherwise the whole track's exit
walk and sweep wait on its slowest blocker, which can be indefinite.

## Priority

Every item carries a band, and so does every program. The bands (Ev,
in chat, 2026-09-20):

- **P0 — very high.** A normal verb broken on normal geometry; a live
  wrong answer; something the GUI cannot author at all; a GUI defect
  Ev reported as making the tool hard or impossible to use. The
  standing goal this serves is **authoring arbitrary geometry through
  the UI**, and most of what remains for it is kernel-side.
- **P1 — high.** Architecture that entrenches as things are built on
  it: two implementations of one underlying logic, a special case that
  should be handled uniformly, and the rest of that class, which
  mostly falls under no tidier heading than itself. Also verb breadth
  beyond the everyday shapes, and GUI defects Ev reported as annoying
  rather than blocking.
- **P2 — medium-high.** Interval and error propagation — the reach
  goal `docs/DESIGN.md` says shapes the architecture.
- **P3 — medium.** Library usability and the north-star audit;
  interop with other tools; tooling that prevents SILENT bugs; latent
  unsoundness, such as a certificate a downstream crate can forge.
- **P4 — low.** Code improvement that is not architectural; tooling
  whose payoff is CI going red less often or the suite costing less;
  prose, citation and naming hygiene.

**A band says what to do, never when.** Dispatch order is the band
together with what the row costs and with whether a design question is
open on it — a cheap P4 with the fix written in its body is often
taken ahead of a P1 that needs a ruling first, and that judgement is
the orchestrator's. It is deliberately not a field: a stored dispatch
order would go stale the first time a ruling landed.

Two things the bands do NOT do. A guard does not inherit the band of
what it guards: a dead assertion in the boolean suite is P3 for being
a guard that cannot go red, not P0 for sitting on P0 ground. And a
band is not a forecast of effort — `cost` carries that, separately,
because the two are independent and collapsing them hides both.

## Track size

**A track is sized to about one orchestrator session** (Ev, in chat,
2026-09-20). The measure is a weighted count of the rows that are
actually dispatchable:

- a row in `open` or `spec` counts; one `dispatched`, `review`,
  `parked`, `deferred` or `closed` does not, because a row in flight
  or ruled not-now is not a claim on the next sitting's attention;
- it counts **1 point at cost `E`, 2.5 at `D`, 5 at `H`** — so one
  budget of **30 points** says about 30 easy rows, about 12 design
  rows, or about 6 hard ones, and says it for a mixed slate too, which
  is nearly every slate;
- a row with no `cost` is charged 2.5, so a track cannot come in under
  budget by declining to price itself.

A program may set its own `budget` with its reason in `program.md`;
absent that it is 30. **The ceiling binds when a program opens and
again as it grows**: a program is not opened over budget, and one that
grows past it splits.

**A track splits along its priority seam, not along another territory
seam.** The seam that matters is the one already inside the slate: a
fifty-row track is usually a dozen rows on the goal and thirty-eight
behind them, and cutting it that way leaves one track a successor can
charter and dispatch. Cutting the same slate five ways by file gives
five tracks too thin to charter, and the territory rules above already
say shared ground is fine. The closing rules' re-homing discipline
applies unchanged: the rows MOVE, by `git mv`, keeping their ids.

**Over budget is a report, not a lint warning.** `STATUS.md`'s
`load` column carries every track's weight against its ceiling and
bolds the ones over, and `work.py status` prints the same. It is
deliberately not a warning on every `lint` run: splitting a track is a
sitting's work and cannot be done in the PR that files its eleventh
row, and a warning nobody can act on in the moment teaches people to
skip warnings — the same reasoning that retired the double-claim
warning above.

## The tracker is not comprehensive

**The tracker exists so work is not FORGOTTEN, not so work is
RECORDED.** It is not an inventory of everything wrong with the tree
and was never meant to be one, and its existence is not a reason to
file instead of fix.

So: **if you notice something small and you can fix it where you
stand, fix it** — in the PR you are already writing, as a drive-by,
and say so in the PR body. Do not open a file for it. A one-line
citation that has rotted, a stale count, a comment describing code
that moved: filing those costs a file, a header, a lint run, a review
and a reader's attention later, to schedule work that was cheaper than
the scheduling. Several rows on the board today are exactly this
mistake and should have been commits.

File an item when the fix is NOT yours to make where you stand:
it needs a design question settled, it crosses into ground you are not
working, it is too large for the PR in hand, or it would otherwise be
lost. That is the whole test. (Ev, in chat, 2026-09-20.)

## Rules

- **One file, one item.** Two programs editing one item is a merge
  conflict, and that is the cross-program handoff surfacing, not a
  bug. Re-parent or re-home by editing the header, never by copying.
  **An item's directory is the program that owns it** — `work.py` reads
  ownership from nowhere else — so a program claiming another's item
  MOVES the file into its own directory in the PR that claims it,
  keeping the id, and sets `parent:` to the unit that carries it. This
  is how a finding reaches its owner. `work/code-quality/` used to be
  where one waited for a claim; it left the tracker on 2026-09-11
  (`docs/doc-ledger/code-quality-leaves-the-tracker.md`) once all 110 of
  its live rows had gone to the eleven programs opened for them, so **a
  finding now goes straight onto the slate of the program whose ground
  it lands on**, and `work/issues/` is the last resort it always was. A
  `keep_out` clause saying a claimed row stays where it was is the thing
  to delete.
- **Ids are stable.** An item keeps its id for life; a program keeps
  its directory for as long as it is open. The rows migrated from the
  2026-08 findings register keep the ids they were cited by (`D102`,
  `S330`, `C15`) wherever they now live; nothing mints new ones in that
  shape.
- **A closed program's directory is deleted.** `work/` tracks work
  still to be done, not work that has been done, so once a program
  closes, `program.md`, `plan.md` and `log.md` go, and so does its
  exit walk if it had one. **A walk is owed when the plan set
  acceptance criteria** (its `## Exit criteria`): the walk is what
  checks the finished program against them, so such a program closes
  when its walk is ratified, or on Ev's ruling that it needs none. A
  plan that set no criteria leaves a walk nothing to check, and its
  program closes without one. The deletion is recorded in a note
  under `docs/doc-ledger/` with the SHA they are recoverable at, and
  that note is the program's done-state of record. Residue is re-homed before the sweep, never
  left behind in the closed directory: to a live program whose charter
  it fits, or to a new program opened for it when the residue coheres
  into a track of its own (a dozen items on one territory are a
  successor's opening slate, and the closing program opens it).
  `work/issues/` is the last resort, for residue that genuinely
  coheres with no live or new track — an unsorted pile of related
  items there is what the sweep exists to prevent. (Ev, 2026-09-06.)
  **That sweep sees items, not sentences**: a residue a
  lane discloses inside its own item's `## Closed` prose reads as a
  record of work done, not as an open thread, so it is invisible to
  the re-homing and dies with the directory. Disclosing a residue is
  therefore not scheduling it — **give it its own file at the moment
  you disclose it**, on this program's slate or in `work/issues/`, and
  let the Closed section point at that file.
- **`work/issues/` is for issues with no home yet, not a waiting room.**
  When the owning program is clear, file the item straight onto that
  program's slate — a lane does not need the owner's permission to put
  a finding where it belongs, and routing it through `issues/` only
  delays the owner seeing it. `issues/` is for the genuine case: a
  finding whose owner is undecided or disputed. Claiming one MOVES the
  file (header edit and `git mv`), never copies it. (Ev, 2026-09-04.)
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

  **A number reaches the rule too, and only ever as a warning.** Ints
  in `blocked_on` are PR or issue numbers that the tracker does not
  resolve against GitHub — but a migrated issue carries its number on
  the item that replaced it (`github:`), and that mapping is in the
  tree. A number matching exactly one such item is read as that item,
  and if it is closed the row is named. It is a **warning** in both
  shapes above, never the error, because the author wrote a number and
  the tracker matched it: a naming is a claim about a row, a match is
  an inference about one, and an inference does not get to red `main`
  for a program that cannot see it. A number matching no `github:`, or
  two, stays unchecked as every int did before. The fix a warning asks
  for is to name the item instead of the number, after which the rule
  reads it directly and the error applies.
- **Territory is a glob list** on the program, and every glob matches
  at least one tracked path. `scripts/work.py territory --base main`
  reads a branch's prefix and its diff and names every path another
  program owns, **and every path the branch's own program claims that
  another open program claims too** — those read differently ("owned
  by X" against "also claimed by X; a double claim, not a crossing")
  because they are different facts. It warns; it does not block.
- **Two open programs may claim one path.** Shared ground is
  legitimate and expected — a kernel file often has a structural
  question on it and a numeric one, and splitting it between two
  programs is usually worse than letting both claim it. Neither side
  owes the other a `keep_out` clause for the overlap to be allowed
  (Ev, in chat, 2026-09-20: *"it's ok if units have shared ground,
  they should just be aware of each other if working at the same
  time"*).

  **What is owed is awareness while a lane is LIVE**, and that is a
  per-branch question, not an at-rest one. `territory` answers it: it
  reads a branch's diff and names every path another program claims,
  so a lane learns at the moment it matters. The announced-seam
  convention carries the rest — a lane touching another program's
  ground says so in its PR, and the orchestrator posts a note on that
  program's log. Write a `keep_out` clause when the relationship is
  worth explaining to the next reader, which is often; do not write
  one to satisfy a checker.

  The at-rest map — which open programs share ground at all — is
  `python3 scripts/work.py territory --overlaps`. It is a report, run
  when you want it. It used to be a `lint` warning on every run, which
  put two dozen warnings in front of every reader for a condition none
  of them was expected to fix; a warning nobody can act on teaches
  people to skip warnings, which costs more than the census is worth.
  `work/meta/double-claim-lint-rule-waits-on-the-tests-seam.md`
  records the ruling and what was built.
- **No plan or log outside `work/`.** `docs/*-PLAN.md` and
  `docs/*-LOG.md` are lint errors, so a session writing to the old
  path fails loudly. (`docs/MODEL-AB-LOG.md` is an experiment log, not
  a program's, and is the one named exemption; it leaves `docs/` when
  the experiment concludes.)
- **Specs keep their lifecycle.** `docs/<ID>-SPEC.md` binds an
  implementer for one unit and is deleted at merge, with a note under
  `docs/doc-ledger/`; the item file is the record that survives.
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
