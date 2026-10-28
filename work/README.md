# `work/` — the tracker

Every stream of work in this repo is a file here. What is open and who
owns it are recorded here and nowhere else.

- **The board** is `work/STATUS.md`, generated on main by CI (The
  board, below). For a live view in any checkout, run
  `python3 scripts/work.py status`.
- **A program's story** is the tail of its `log.md`.
- **The rules** are this page. `scripts/work.py lint` enforces every
  rule here that a script can check, on every CI tier; an item that
  lint rejects does not merge.

The tracker is version control and nothing else: no GitHub issues,
projects, labels or milestones. Filing, claiming, blocking and closing
are all commits. GitHub PRs are the review and merge vehicle, and the
one channel to Ev (Ev's channel, below).

## Layout

```
work/
  README.md                this contract
  STATUS.md                GENERATED on main by CI — never hand-edited
  issues/README.md         that directory's signpost
  issues/<id>.md           items no program owns yet
  <program>/
    program.md             the program: charter, prefix, territory
    plan.md                the plan (narrative, present state only)
    log.md                 the log (append-only narrative)
    process-observations.md  optional: what the program has learned
                           about how its own work goes wrong
    <id>.md                one file per item: unit, issue or ruling
```

A program's directory name is its id, and an item's file name is its
id. A new item's id is a descriptive kebab-case name; older rows keep
the ids they were filed under (Rules: *Ids are stable*). The directory
a file sits in is its owner.

`work.py new` creates items, not programs. A program is opened by
hand: write `work/<id>/program.md`, `plan.md` and `log.md`.

## The header

Every file except `README.md`, `STATUS.md`, `plan.md`, `log.md` and
`process-observations.md` opens with a YAML front-matter block. The
parser accepts a deliberately small subset — `key: scalar` and
`key: [a, b, c]` only: no nesting, no multi-line values, no anchors.
Anything else is a lint error, and so is an unknown key: to add a key,
add it to the schema in `scripts/work.py` in the PR that first uses it.

An item (`unit`, `issue` or `ruling`):

```yaml
---
id: the-saturated-span-refuses-at-the-parse
kind: issue                # unit | issue | ruling
title: the saturated span refuses at the parse
status: open               # Vocabularies, below
opened: 2026-10-02
closed:                    # a date; set exactly when status is closed
priority: P1               # P0..P4 (Priority, below)
cost: H                    # C | E | M | H | X: the effort (Vocabularies)
points: 30                 # only with cost: X, priced by hand
design: true               # present only while a design question is open
parent: some-carrying-unit # optional: another item's id
blocked_on: [a-blocker, 1601]   # item ids, or PR/issue numbers as ints
rides_with: its-carrier    # optional: the item this one travels with
refs: [S330, 1588]         # related items or numbers; no semantics
pr: 1605                   # the PR carrying the item
branch: mesh/saturated-span
---
```

`id`, `kind`, `title`, `status` and `opened` are required.

A program (`program.md`):

```yaml
---
id: mesh
kind: program
title: MESH — the tessellation kernel
status: ready              # ready | active | blocked (Vocabularies)
opened: 2026-09-02
group: kernel/mesh          # where the board files it (below)
prefix: mesh/              # the program's branch prefix; ends in `/`
tag: (MESH orchestrator)   # the away-channel role tag
ab_band: 1200-1299         # the program's band in docs/MODEL-AB-LOG.md
paths: [crates/mesh/*]     # territory globs (Rules, below)
keep_out: [prose pointers, one string each]
budget: 40                 # optional; the reason goes in the body
---
```

The prefix and tag follow the branch-prefix and away-channel
conventions in `memories/orchestration-model.md`: unit branches are
`<prefix><unit>`, the orchestrator's branch is `<prefix>orchestrator`,
and every comment the orchestrator posts leads with its tag. A program
carries no priority (Priority, below).

**`group`** places the program in the hierarchy the board shows:
lowercase segments joined by `/`, top level first (`kernel`, `api`,
`gui`, `infra`), then a subsystem or two (`kernel/boolean/split`). The
board names each program by its group and id together,
`kernel/mesh/tess`. Only the header carries the hierarchy: the
directories stay flat at `work/<id>/`, so re-filing a program edits one
line and no citation of `work/<id>/` moves. File a program beside the
programs whose work it most resembles; a split's successors usually
share their parent's group.

## Vocabularies

**kind**

- `program` — a track of work, in `program.md` only.
- `unit` — a dispatchable piece of work with a spec and a PR.
- `issue` — a defect or finding, not yet a unit.
- `ruling` — a question only Ev answers; never work.

**status of an item**: `open` → `spec` → `dispatched` → `review` →
`closed`, plus two ways of being not-now. A ruling is only ever
`open` or `closed`.

- `parked` — **waits on a named trigger.** `blocked_on` must be
  non-empty, and every id in it must resolve. A trigger is an item or
  a PR: something that can fire, and that lint can see has fired.
- `deferred` — **ratified as not-now**, with the ratification cited in
  the body. A deferred row is not waiting for anything; it has been
  decided against for now, so lint refuses a `blocked_on` on it (a row
  that waits on a trigger is `parked`). The citation is prose — a
  README clause or a design doc, not an item id — so lint cannot check
  it; a reviewer reads the row and judges whether the ratification
  says what the row claims.

Neither not-now status counts as available work. `STATUS.md` gives
each its own column, so a not-now row is never read off the board as
dispatchable, and neither is listed as stale for going untouched.

**cost** is the row's effort and nothing else (Ev, in chat,
2026-09-27):

- `C` — chore: a mechanical change (a rename, a citation or count
  fix, a header sweep) that needs no judgement. Chores are dispatched
  only in bundles, several to one lane, or ride along with a unit that
  touches the same ground; a chore never gets an implementer of its
  own.
- `E` — easy: the fix is written in the item or is obvious.
- `M` — medium: more than a written-down fix, short of hard.
- `H` — hard: the intent is clear, and getting it right is technically
  difficult (geometry, certified-interval reasoning, topology
  invariants, tests that are hard to construct).
- `X` — extra hard: larger than any one hard row, typically a feature
  that will expand into a program of its own. An `X` row is priced by
  hand in `points:`, with the reasoning in its body; no other row
  carries `points:`.

`cost: D` is legacy: before 2026-09-27 it meant either "a design
question is open" or "medium", so read such a row's body, re-price it
`C`/`E`/`M`/`H`/`X`, and set `design` if it applies, whenever you touch it.
Lint refuses `cost: D` on a row opened on or after 2026-09-27.

**design** is separate from cost, because effort and an open decision
vary independently: a hard row can be pure execution, and a cheap one
can hinge on a choice. `design: true` says a question with several
viable answers is open on the row and must be weighed before a lane
builds it (`memories/orchestration-model.md`: designers first, and an
`[ev]` PR only if the fork is Ev's). Remove the flag once the question
is decided.

**status of a program** says whether an orchestrator is on the track.
That is the one fact the row counts beside it cannot show: a session
with a full slate and nothing dispatched yet looks exactly like an
abandoned one.

- `ready` — **no orchestrator, and something to pick up**: at least
  one row is dispatchable (`open` or `spec`). This is what a successor
  scans the board for.
- `active` — **an orchestrator holds the track.** Nothing in the tree
  can confirm that, so lint takes the program's word for it. The
  orchestrator sets it on picking the track up and clears it on
  handing the track back.
- `blocked` — **no orchestrator, and nothing to pick up**: every live
  row is in flight, parked or deferred. Lint checks the half of this
  the tree can see: no dispatchable row, and at least one live one.

A blocked track never has an orchestrator. An orchestrator that runs
out of non-blocked rows does not sit on the track waiting for its
triggers: it cuts the blocked rows into a new program (Track size),
closes what it has finished, and leaves the new program `blocked` with
nobody on it. Otherwise the whole track's closing waits on its slowest
blocker. When a sitting ends is in `memories/orchestration-model.md`.

There is no `closed` program status, because a program that closes is
deleted (Rules: *A closed program's directory is deleted*). While its
exit walk is being written the track is still `active`, and nothing
requires an `active` track to hold a dispatchable row.

## Priority

Every row should carry a band. Lint does not require one; an unbanded
row sorts after P4 and does not count towards its program's place on
the board. The bands (Ev, in chat, 2026-09-20):

- **P0 — very high.** A normal verb broken on normal geometry; a live
  wrong answer; something the GUI cannot author at all; a GUI defect
  Ev reported as making the tool hard or impossible to use. The
  standing goal this serves is **authoring arbitrary geometry through
  the UI**, and most of what remains for it is kernel-side.
- **P1 — high.** Architecture that entrenches as things are built on
  it: two implementations of one underlying logic, a special case that
  should be handled uniformly, and the rest of that class. Also verb
  breadth beyond the everyday shapes, and GUI defects Ev reported as
  annoying rather than blocking.
- **P2 — medium-high.** Interval and error propagation — the reach
  goal `docs/DESIGN.md` says shapes the architecture.
- **P3 — medium.** Library usability and the north-star audit;
  interop with other tools; tooling that prevents SILENT bugs; latent
  unsoundness, such as a certificate a downstream crate can forge.
- **P4 — low.** Code improvement that is not architectural; tooling
  whose payoff is CI going red less often or the suite costing less;
  prose, citation and naming hygiene.

**A band says what to do, never when.** Dispatch order weighs the band
together with `cost` and `design`: a cheap P4 with the fix written in
its body is often taken ahead of a P1 that needs a ruling first. That
judgement is the orchestrator's, and it is deliberately not a field,
because a stored dispatch order would go stale the first time a ruling
landed.

**A band is absolute across the tracker, never relative to its
program.** The definitions say what a row IS, wherever it sits: the
most important row in a quiet program is not P0 for being that
program's most important row. Ask what band the row would carry if it
were filed in the program at the top of the board.

**A band is not a forecast of effort**; `cost` carries that, and the
two are independent.

**A guard does not inherit the band of what it guards.** A dead
assertion in the boolean suite is P3, for being a guard that cannot go
red, not P0 for sitting on P0 ground.

**A blocker does inherit the band of what waits on it.** A parked row
cannot move until its blocker closes, so a P0 parked on a P2 would be
dispatched in P2's turn. The part of the blocker that stands in the
way is therefore work of the parked row's band. (This is not the guard
case: a guard sits on P0 ground without standing between anyone and
P0 work.) Lint refuses a `parked` row that names a live blocker of a
lower band, and there are three ways to clear the error:

- **re-band the blocker**, when fixing it as described is what
  unblocks the parked row;
- **split it**, when only part of the fix does: file the unblocking
  part as its own row at the parked row's band, re-park the parked row
  on it, and leave the remainder at its own band;
- **open the parked row**, when it was waiting on the blocker for
  convenience rather than need.

A chain settles one link at a time: a blocker re-banded upward brings
its own blockers under the rule. A program carries no band, so naming
one as a blocker is not checked, and neither is a number in
`blocked_on`. Raising a parked row's band can red `main`
for a blocker on another program; the answer is to fix the blocker's
band, not to soften the check.

**A program's place on the board is computed from its rows.**
`STATUS.md` orders the Programs table by each track's dispatchable
rows (`open` or `spec`), compared band by band from P0 down: one
dispatchable P0 outranks any number of P1s, and between two tracks
whose best rows share a band, the one with more of them ranks first.
Rows in flight, parked or deferred only break ties, because nobody can
pick them up next. The `top` column is the best dispatchable band.
When a program's last P0 closes it drops down the board, and filing a
P0 on a quiet track raises it. To move a program, re-band its rows.

## Track size

**A track is sized to about one orchestrator session** (Ev, in chat,
2026-09-20). The measure is a weighted count of the dispatchable rows
(`open` or `spec`) — a row in flight, not-now or closed is not a claim
on the next sitting's attention:

| cost | points |
|---|---|
| `C` | 0 |
| `E` | 1 |
| `M` | 3 |
| `H` | 8 |
| `X` | its `points:` |
| legacy `D` | 3 |
| unpriced | 3 |

The default budget is **30 points**: about 30 easy rows, 10 medium or
4 hard, or any mix. The weights come from the measured effort of
closed rows (wall time and commits, `analysis/cost-tiers/` on the
`analysis/cost-tier-effort` branch), and `H` carries the follow-up
rows a hard row typically files as well as its own work. A chore costs
nothing because it only ever travels in a bundle. `design` adds
nothing, because the cost prices the whole row, the weighing included.
An unpriced row is charged as medium so a track cannot come in under
budget by declining to price itself.
A program may set its own `budget`, with its reason in `program.md`.

**The budget binds when a program opens and as it grows**: a program
is not opened over budget, and one that grows past it splits.

**A track splits along its semantic seams.** Cut where the rows part
by question, mechanism or charter, so each successor tells one story
that someone can charter and dispatch as a whole. A file boundary is
not by itself a seam: cutting a slate by file leaves tracks too thin
to charter, and shared files are fine anyway (Rules: territory). The
rows MOVE, by `git mv`, keeping their ids.

**Over budget is a report, not a lint warning.** `STATUS.md`'s `load`
column shows every track's weight against its budget and bolds the
ones over; `work.py status` prints the same. Splitting a track is a
sitting's work and cannot be done in the PR that files the row that
tips it over, and a warning nobody can act on in the moment teaches
people to skip warnings.

## The tracker is not comprehensive

**The tracker exists so work is not FORGOTTEN, not so work is
RECORDED.** It is not an inventory of everything wrong with the tree,
and its existence is not a reason to file instead of fix.

So: **if you notice something small and you can fix it where you
stand, fix it** — in the PR you are already writing, as a drive-by,
and say so in the PR body. Do not open a file for it. A rotted
citation, a stale count, a comment describing code that moved: filing
those costs a file, a header, a lint run, a review and a reader's
attention later, to schedule work cheaper than the scheduling.

File an item when the fix is NOT yours to make where you stand: it
needs a design question settled, it crosses into ground you are not
working, it is too large for the PR in hand, or it would otherwise be
lost. That is the whole test. (Ev, in chat, 2026-09-20.)

## Rules

### Filing and ownership

- **One file, one item.** An item's directory is the program that owns
  it; `work.py` reads ownership from nowhere else. Re-parent or
  re-home by editing the header and moving the file, never by copying.
  Two programs editing one item is a merge conflict, and that is the
  cross-program handoff surfacing, not a bug.
- **A finding goes straight onto the slate of the program whose ground
  it lands on.** When the owning program is clear, file the item in
  that program's directory: a lane does not need the owner's
  permission to put a finding where it belongs, and routing it through
  `work/issues/` only delays the owner seeing it.
  (`python3 scripts/work.py territory --files -` says who owns a path.)
- **`work/issues/` is for issues with no home yet, not a waiting
  room** (Ev, 2026-09-04): a finding whose owner is undecided or
  disputed, or that coheres with no live track.
- **Claiming moves the file.** A program claiming another's item, or
  one from `work/issues/`, MOVES it into its own directory with
  `git mv` in the PR that claims it, keeps the id, and sets `parent:`
  to the unit that carries it.
- **A rides-along is its own file**, with `rides_with:` naming its
  carrier. Closing the carrier does not close the passenger; lint
  refuses a live passenger on a closed carrier.

### Ids and references

- **Ids are stable.** An item keeps its id for life, and a program
  keeps its directory for as long as it is open. Rows migrated from
  the 2026-08 findings register keep the ids they were cited by
  (`D102`, `S330`, `C15`); nothing mints new ids in that shape.
- **References resolve.** Every id in `parent`, `blocked_on`,
  `rides_with` and `refs` names a file that exists. Ints are PR or
  issue numbers and are not checked against GitHub, so a blocker
  that is an item should be named by its id.
- **A fired trigger is not a blocker.** A `parked` row whose
  `blocked_on` names a CLOSED item has had its trigger fire, so the
  reference is no evidence the row is still blocked. Two cases:
  - **every blocker closed — a lint ERROR.** `parked` is false of the
    row. Re-park it on what actually gates it, open it, or defer it.
  - **a closed blocker beside a live one — a lint WARNING.** The row
    is still blocked, so only the closed entry is stale; prune it.

  The error can red `main` from a PR on another program: the program
  closing a trigger cannot un-park another program's rows in the same
  PR (one file, one item). That cost was accepted deliberately (Ev,
  2026-09-04): fix the stale rows, do not soften the check.

### Territory

- **Territory is a glob list** (`paths`) on the program, and every
  glob must match at least one tracked path. Globs are `fnmatch`
  patterns, where `*` crosses `/`: `crates/mesh/*` is the whole crate.
- **Two open programs may claim one path, and two lanes may edit one
  file.** What needs care is SEMANTIC conflict, not textual overlap: a
  kernel file often has a structural question on it and a numeric one,
  and splitting it between two programs is usually worse than letting
  both claim it (Ev, in chat, 2026-09-20: *"it's ok if units have
  shared ground, they should just be aware of each other if working at
  the same time"*).
- **Logically independent changes share files freely.** Two changes are
  independent when neither one's correctness depends on what the other
  changes: different functions, different invariants, or different
  arms of one match that read nothing the other writes. They need no
  announcement and no sequencing, and a textual merge conflict between
  them is routine (merge main, resolve, carry on), not a coordination
  failure.
- **What is owed is awareness where changes are coupled.** Changes are
  coupled when one alters a contract, invariant, signature or
  behaviour that the other reads or relies on, when both rework the
  same logic, or when one would make the other's tests or reasoning
  wrong. Then the lanes must know of each other while both are live:
  the lane says so in its PR, and the orchestrator posts a note on the
  other program's log. To find the candidates, fetch, then run
  `python3 scripts/work.py territory --base origin/main` on the
  branch. Use `origin/main`, never a bare `main`: in a worktree or
  remote checkout the local `main` ref is typically far behind, and
  the answer against it is wrong without looking wrong. The command
  reads the branch's prefix and diff and names every path another
  program owns, and every path the branch's own program shares with
  another ("owned by X" against "also claimed by X"). It warns; it
  does not block. A path it names is a prompt to check what the other
  program's live rows are doing there, not by itself a coupling.
- **`keep_out` is for the next reader**, not for a checker. Write one
  when the relationship between two programs' ground is worth
  explaining, which is often.
- The at-rest map of shared ground is
  `python3 scripts/work.py territory --overlaps`, a report run when
  you want it, not a lint warning
  (`work/meta/double-claim-lint-rule-waits-on-the-tests-seam.md`).

### Closing

- **A closed program's directory is deleted.** `work/` tracks work
  still to be done, so when a program closes, its `program.md`,
  `plan.md`, `log.md` and exit walk go. The deletion is recorded in a
  note under `docs/doc-ledger/` naming the SHA they are recoverable
  at, and that note is the program's done-state of record
  (`memories/docs-ledger.md`).
- **A walk is owed when the plan set exit criteria** (its
  `## Exit criteria`): the walk checks the finished program against
  them, so the program closes when its walk is ratified, or on Ev's
  ruling that it needs none. A plan that set no criteria closes
  without one.
- **Residue is re-homed before the sweep**, never left in the closed
  directory (Ev, 2026-09-06): to a live program whose charter it fits,
  or to a new program the closing program opens for it when the
  residue coheres into a track of its own (a dozen items on one
  territory are a successor's opening slate). `work/issues/` is the
  last resort, for residue that coheres with no live or new track.
- **The sweep sees items, not sentences.** A residue mentioned in an
  item's `## Closed` prose reads as a record of work done, so it dies
  with the directory. Disclosing a residue is therefore not scheduling
  it — give it its own file at the moment you disclose it, on this
  program's slate or in `work/issues/`, and let the Closed section
  point at that file.

### Where documents live

- **No plan or log outside `work/`.** `docs/*-PLAN.md` and
  `docs/*-LOG.md` are lint errors. The exemptions are the experiment
  logs `docs/MODEL-AB-LOG.md`, `docs/DUAL-REVIEW-LOG.md` and
  `docs/DESIGN-FORK-LOG.md` (`memories/experiments.md`); each leaves
  `docs/` when its experiment concludes.
- **A spec lives only as long as its unit.** `docs/<ID>-SPEC.md` binds
  an implementer for one unit and is deleted at merge, with a note
  under `docs/doc-ledger/`; the item file is the record that survives.
- **`STATUS.md` is written by CI only** (`.github/workflows/work-status.yml`,
  on every push to main that touches `work/` or `scripts/work.py`).
  Nothing else writes it, so no branch conflicts on it.

## Ev's channel

Ev does not edit files. Anything that needs Ev is a PR whose title
starts with **`[ev]`**; how to write one is "Asking Ev" in
`CLAUDE.md`. Once Ev answers, the agent updates
the item and the docs, and merges.

## The log merges by union

Every lane appends its entry at the end of its program's `log.md`, so
two branches that each add one collide at the same line, and neither
is wrong. `.gitattributes` gives `work/*/log.md` git's `union` merge
driver: a merge keeps both sides' lines, in order, with no conflict.
That is only sound because the log is append-only — an edit to an
earlier entry on one side would come out duplicated, not flagged — so
never edit a past entry.

GitHub's own merge ignores the attribute. A PR whose log has diverged
from main's shows as conflicting on GitHub, and gets no CI run, until a
local merge of main (which does honour the attribute) is pushed.

Because the union merge never conflicts, a note another program leaves
on your log arrives silently. `python3 scripts/work.py incoming` lists
every commit on `origin/main` that touches `work/<program>/` and is not
yet in the current branch. An orchestrator runs it (after fetching) at
each check-in and before bringing main into its branch.

## The board

`work/STATUS.md`, top to bottom:

- **Programs** — one row per program, in board order (Priority). The
  band columns count live rows; `load` is the track's weight against
  its budget, bold when over (Track size); the status columns count
  rows per item status; `status` is the program's own.
- **One section per program** — its live rows, by band.
- **Issues no program owns** — the live rows in `work/issues/`.
- **Blocked** — every live row with a `blocked_on`, and what it
  waits on.
- **Untouched for 14+ days** — live rows whose file has not changed in
  two weeks, parked and deferred rows excepted.

## The script

```
python3 scripts/work.py lint                  every checkable rule above; CI runs it
python3 scripts/work.py status [--program P]  the board, to stdout; P is an id, or a group for its subtree
python3 scripts/work.py render                the board, to work/STATUS.md (CI's job)
python3 scripts/work.py new <id> --kind K --title T [--program P] [--set k=v]
python3 scripts/work.py set <id> key=value [key=value ...]
python3 scripts/work.py territory (--base <ref> | --files <list>|-) [--branch <name>] [--strict]
python3 scripts/work.py territory --overlaps
python3 scripts/work.py incoming [--program P] [--base <ref>]
python3 scripts/work.py --selftest
```

`set` takes `key=` to clear a field and `key=[a,b]` for a list.
`territory --strict` exits 1 on a collision. `incoming` defaults to
`--base origin/main` and to the program whose prefix the current
branch carries.
