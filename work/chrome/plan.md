# CHROME — viewer chrome and coverage (plan)

**STATUS: OPEN; slate re-cut 2026-09-15.** The opening slate of nine
units all landed on 2026-09-04 and the program then went dormant for
eleven days. What it holds now is the residue those units filed, five
hand-offs from DOCM, and rows other programs filed onto this slate
while it was asleep. Live state is `work/chrome/log.md`'s tail and the
item files beside this plan, never this file.

Branch prefix (the #396 convention): **`chrome/`** — unit branches
`chrome/<unit>-<slug>`. Sessions whose harness pins a branch drive the
orchestrator half from that branch instead (VIEW's plan sets the
precedent); unit branches are unaffected and keep the `chrome/` prefix.
Away-channel tag `(CHROME orchestrator)`.

## Review posture

**No A/B duals, no row in `docs/MODEL-AB-LOG.md`** (Ev, in-chat,
2026-09-04, reaffirmed 2026-09-15). The band 1600-1699 stays claimed
and empty. The default is a **style review** against
`docs/prompts/reviewer-style-lane.md`; a correctness arm is added only
where a unit's failure mode is a *confident wrong answer* rather than a
refusal, and the dispatch says which it chose and why.

The 2026-09-04 slate is the evidence for this posture, not an
assumption: **on every unit that got a review, the review found
something the unit's own evidence did not** — and two of those were
correctness defects found by a reviewer refusing to accept a claim,
not by a correctness lane. A style lane that verifies claims finds
correctness defects as a side effect of checking whether the words are
true.

## Territory — the carve-out is spent (2026-09-21)

The 2026-09-15 section this replaces divided `crates/viewer` between
CHROME and VIEW file by file, because both claimed the whole crate and
both were live in it. **Two things have happened since, and together
they retire it.**

1. **VIEW re-scoped on 2026-09-17.** Eighty-six of its ninety-four
   live rows went to four successor programs (VNEWS, VGEOM, VSEAM,
   VDOC) or to seven other live programs, and its own `keep_out` now
   says plainly: *"this program does not dispatch new units — its
   remaining act is the exit walk."* Half of every cession clause
   below was *"VIEW holds an open row on the same ground"*. VIEW holds
   almost none, and will open no more.
2. **`work/README.md` ruled the general question on 2026-09-20** (Ev,
   in chat): two open programs may claim one path, shared ground is
   legitimate and expected, **neither side owes the other a
   `keep_out`**, and what IS owed is awareness while a lane is LIVE —
   a per-branch question that `work.py territory --base main` answers.

So CHROME does not cede `scene.rs`, `gpu.rs`, `theme.rs`,
`pane/features.rs`, `marks.rs`, `blend.rs`, `app.rs`, `session.rs`,
`session/*`, `pane/*`, `frame.rs`, `pickindex.rs`, `display.rs`,
`props.rs`, `forms.rs` or `sketch.rs` to anybody. It claims
`crates/viewer/src/*`, `crates/viewer/tests/*` and
`crates/viewer/README.md`, as `paths` has always said, alongside
AUTHOR, VGEOM, VSEAM, VNEWS, VIEW and FIT.

**What replaces the file list is a per-wave read of who is live.** At
dispatch the orchestrator runs `work.py territory` and the open-PR
list, and a unit's scope names the files another lane is in RIGHT NOW
as out-of-scope for this wave — a scheduling fact with a date on it,
not a fence. Every dispatch carries it; no clause here can, because it
would be false within the day.

**Three rows were held only by the spent cession and are now
available**: `gpu-index-counts-substitute-u32-max` and
`mispaired-ids-exempts-the-empty-window` (both closed their paragraph
with *"ground is ceded to VIEW under the carve-out, so CHROME does not
work it"*), and `band-refusal-still-badges-every-row`, whose blocker
was *"a new `RowStatus` variant does not land inside CHROME's fence"*
— a fence that no longer exists. The COST arguments in those rows
stand and are unaffected: the `RowStatus` row still measures 19 sites
in 9 files for the field variant, which is a reason to shape the fix
carefully, not a reason it cannot be dispatched. VGEOM now owns
`scene.rs` and `gpu.rs` beside CHROME, so the first two are a re-home
candidate as much as a dispatch candidate; that call waits on the
wave that takes them.

## What the 2026-09-15 audit found

Every open row was re-read against the tree. **Nothing was dead** —
VIEW closed none of CHROME's rows in eleven days. But it moved the
*addresses* of about twelve, and three rows existed only to report that
rot. Two findings are worth carrying forward:

- **A rotted citation is not a cosmetic defect here.** Several rows
  cited line bands that now hold unrelated code, one cited a file that
  no longer exists (`crates/viewer/src/pick.rs`), and one row's premise
  had been falsified outright by work that landed after it was filed —
  it said the add-profile tool authors on world XY only, where the op
  now carries a picked frame node. A lane taking that row on trust
  would have built on a false statement. `docs/prompts/implementer-
  discipline.md` §7 already says why: cite by name, because numbers rot.
- **The counts inside rows rot too, and more quietly.** One row's
  census (*"`ui.weak` is spelled 49 times in `app.rs`"*) is off by an
  order of magnitude after the split — `app.rs` has three. A number a
  row asserts is evidence only as of its filing.

**A shift map is not evidence, and its consistency is not corroboration**
(VIEW, 2026-09-15, measured at 2 wrong subjects in 4 handed-over maps;
CHROME's repoint unit measured the other half independently). A shift
map is arithmetic that returns an answer for every input without asking
what that answer names. CHROME's closed report had all six of its
numbers wrong by *exactly* +76 — and a uniform delta is what a
whole-file insertion above the citations produces, which is equally
what a map naming the wrong subjects produces when they all moved
together. The unit using that map then minted four fresh wrong claims,
two in the shape it existed to close. **A unit whose subject is stale
claims is the most likely to mint them, not the least.** Re-derive a
shifted citation BY SUBJECT — open the file and confirm the symbol —
or flag the row and let the owning program repoint it.

## Ev's requests — high priority

Filed 2026-09-17 from Ev's own list of UI nits, and **ahead of the
order below**: Ev asked for these directly, so they are taken before
anything else on this slate. Each row carries Ev's note verbatim.

None open: the two rows filed here landed in PR 2856 on 2026-09-19.

## Unit order

**Wave 1, dispatched 2026-09-21.** Three units, all E, all on disjoint
files, none carrying an undecided design fork that only Ev can settle.
No A/B duals and no row in `docs/MODEL-AB-LOG.md` (Ev, in chat,
2026-09-21: *"no AB protocol"*). Specs are
`docs/CHROME-<NAME>-SPEC.md`, deleted at merge per
`docs/DOC-LEDGER.md`.

1. **`chrome/datum-honesty`** (`docs/CHROME-DATUM-HONESTY-SPEC.md`) —
   `datums.rs`: `four-spellings-of-one-finiteness-predicate-in-datums-
   rs` and `max-grid-lines-truncates-a-ruling-and-calls-it-one`. One
   file, one class (a number the module could not honestly compute,
   returned in the shape of one it did).
2. **`chrome/empty-document-gate`**
   (`docs/CHROME-EMPTY-DOC-SPEC.md`) — `frame.rs` and `pickindex.rs`:
   `viewer-states-the-empty-document-rule-in-four-places-and-the-one-
   that-gates-cannot-red`. The structural half only.
3. **`chrome/one-number-one-home`**
   (`docs/CHROME-ONE-NUMBER-SPEC.md`) — `bounds.rs`, `app.rs`,
   `scene.rs`, `tests/display_budget.rs`:
   `bounds-reading-respells-the-panels-one-divide` and
   `display-budget-rows-restate-three-private-constants`.

**Two premise corrections found by reading the tree before writing the
specs**, and recorded because the program's own 2026-09-15 audit says
this is where units get lost: `MAX_GRID_LINES` is **512**, not the 96
its row asserts and builds a reachability argument on; and the
finiteness census in `datums.rs` is **eight** sites, not the four its
row lists — the row predicted exactly that growth and it happened.
Both corrections are in the dispatches, with an instruction to re-take
the census rather than trust them.

**Held out of wave 1, and why.**
`at-rest-badge-reports-an-empty-document-as-a-refusal` is the
behavioural half of unit 2's cluster and lives in `session.rs`, which
AUTHOR's live `author/profile-frame` lane (AUTH-3) has in scope this
hour — a scheduling conflict, not a fence, and it goes out in wave 2.
`a-split-circle-fixture-sits-inside-the-1e-6-escalation-band` is small
and a live red at `1e-6` on `main` today; it is wave 2's first row.
`certify-affordance-on-the-bounds-panel` is priced **E and is not** —
it is a long-running query needing progress, cancel, a panel-side
budget and eleven unrendered `RangeRefusal` arms; it wants a re-price
and a design pass, not a lane.

## Exit shape

The program does not close on this slate. `crates/viewer/tests/*` is
CHROME's and VIEW's territory both, VIEW's architecture work will keep
landing findings on this ground, and what the carve-out leaves of
`crates/viewer/src` is VIEW's the day CHROME closes. The walk
convention applies when it does; residue re-homes per
`work/README.md`, not into `work/issues/`.
