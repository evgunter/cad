# CHROME — viewer chrome and coverage (plan)

**STATUS: OPEN; slate re-cut 2026-09-15, and cut again 2026-09-22** (see *The slate after the 2026-09-22 cut*). The opening slate of nine
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

**The three rows the spent cession alone was holding are all
disposed of**: `band-refusal-still-badges-every-row` stays here (Wave 2
priced its variant and left the row open), `gpu-index-counts-substitute-u32-max`
is closed, and `mispaired-ids-exempts-the-empty-window` went to FIT in
the 2026-09-22 cut, after VGEOM declined it on its charter test.

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

**Wave 1 LANDED 2026-09-21** — three units, five rows, PRs 3018, 3021
and 3022. No A/B duals and no row in `docs/MODEL-AB-LOG.md` (Ev, in
chat: *"no AB protocol"*); the band 1600-1699 stays claimed and empty.
Each unit's spec was deleted at its merge per `docs/DOC-LEDGER.md`, so
nothing here points at one; `work/chrome/log.md` carries what each
did and the item files are the record that survives.

**What the wave is evidence for, stated once because it is the reason
this program reviews the way it does.** Every one of the three units
was corrected by the layer below it, and the corrections changed
answers rather than details:

- **A row's premise was wrong on all three units.** `MAX_GRID_LINES`
  is 512, not the 96 its row builds a reachability argument on. The
  `datums.rs` finiteness census was eight sites, not four. And
  `ProductErrorKind::is_empty_document`, the home the whole
  empty-document unit was built around citing, **does not exist** —
  renamed `means_no_body` inside PR 2629's own lane, so the row was
  filed citing a dead name. Two were caught at dispatch by reading the
  tree; the third by the lane. **Reading the tree before writing a
  spec is now what this program does**, and it costs minutes.
- **A correctness arm blocked a merge.** The grid unit's first cut
  shrank the over-cap patch centred on the REGION, which at a grazing
  seat is not centred on what the camera is aimed at — past a pane
  about 6900 px tall it drew a complete, closing rectangle with
  nothing at the aim point, and it was a REGRESSION against the
  truncation it replaced. Style review alone would not have caught it;
  the arm was added because that unit's failure mode is a confident
  wrong answer rather than a refusal, which is the rule this program
  already had.
- **The fresh-instance trap landed on all three units, seven times.**
  A lane closing a duplication minted one, every time. One lane caught
  its own at the keyboard and another found its fourth while filing
  the row about it; the rest were caught only by a reader who did not
  write the fix. `docs/prompts/reviewer-style-lane.md` §1's last
  bullet is the single highest-yield line in the brief, and naming the
  trap in a PR body still prevents nothing.
- **Two lanes overturned an orchestrator recommendation with
  measurement, and both were right.** One was told to restate a
  constant deliberately and instead made it private and exposed the
  DERIVED bound (`placed_rung_cost`), which is `Camera::pitch_limit`'s
  precedent exactly rather than the departure the review called it.
  The other declined to derive `OVER_BUDGET_DELTA` from the starting
  δ, because a decade under a COARSER start is a δ the same file
  records as inside the budget. Sequencing and shape calls are the
  orchestrator's; both of these were the lane's, and the lane had the
  measurement.

**Wave 2 LANDED 2026-09-22** — three units, four rows (PRs 3055, 3058, 3059), no A/B duals
and no row in `docs/MODEL-AB-LOG.md` (Ev, in chat: *"no AB protocol"*);
the band 1600-1699 stays claimed and empty. Dispatched against the item
files directly rather than against `docs/<ID>-SPEC.md`: these rows carry
their own `## What a taker owes`, and a spec restating a complete row is
a second place for its premises to rot.

- **`chrome/wrap-in-region`** — `error-and-check-text-overflows-its-region`,
  the slate's only P0 and Ev's own request. The LAYOUT half only; the
  concision half reaches kernel `Display` impls and files per-crate rows
  instead. Un-investigated when filed, so investigation is the bulk.
- **`chrome/rowstatus-exhaustive`** —
  `has-faults-cannot-red-on-a-new-rowstatus` with
  `band-refusal-still-badges-every-row`, as one unit and in that order:
  the guard goes exhaustive FIRST so the Band row's variant cannot land
  silently. Both rows say to take them together.
- **`chrome/split-circle-eps`** —
  `a-split-circle-fixture-sits-inside-the-1e-6-escalation-band`, a live
  red on `main` at `1e-6` that one CI step's missing `CAD_TOLERANCE_EPS`
  hides.

**The live-ground map each dispatch carried, and the fact that it was
partly WRONG.** `plan.md`'s territory section promises a per-wave read
of who is live rather than a file list, and this was the first wave to
owe one. I built it from `git diff --name-only origin/main...origin/<branch>`
per open PR — and a **three-dot diff takes the merge base**, which for a
week-stale branch is far behind `main`, so `main`'s own changes to a file
are attributed to the branch. The map therefore named files as live that
were not, and missed at least one that was.

Measured, after PR 3055's reviewer caught it: **#2929 does not touch
`crates/viewer/tests/tree_badges.rs` at all** — its diff is 40 files and
that is not among them; its branch merely carries pre-`main` content
there. **#2934 (`msolve/9-from-face`) touches both that file AND
`crates/viewer/src/tree.rs`**, and `tree.rs` is what I told the lane was
clear ground and is where its whole change lives.

**The authoritative source is the PR's own file list**, which GitHub
computes against the real merge base; `git merge-base` is not a
sufficient substitute, because a branch that has merged `main` has
several merge bases and `git` picks one. `git merge-tree --write-tree A B`
answers the question that actually matters — will these two conflict —
and is what this program should use from here.

This is the same root cause as
`work/meta/territory-base-main-is-stale-in-every-agent-checkout`, filed
this sitting from the other direction: a stale base ref silently
answering a question about the wrong tree. Two independent instances in
one wave, one of which reached a merged artifact.

## The slate after the 2026-09-22 cut

The cut (`work/chrome/log.md`, same date) left **28 points** against
the 30-point ceiling, in two families and the class behind the second.

- **The text the viewer lays out** — Ev's P0 overflow report and its
  residue. The concision half (`error-and-check-text-overflows-its-region`),
  the fifty unconverted sentences
  (`messages-in-the-creation-and-properties-panes-still-draw-past-their-row`)
  and the toolbar's own canceled line are P0. Before them sits a fork
  nobody has answered: `messages-wrapped-at-a-region-and-numbers-bounded-by-characters-are-two-answers`
  and `wrapping-at-a-region-with-no-floor-produces-a-four-character-ribbon`
  ask what a too-narrow region owes a sentence, and the fifty-site
  conversion waits on that answer so it is done once.
  `an-auto-sized-window-makes-available-width-last-frames-content`,
  `feature-tree-row-labels-draw-an-unbounded-pose-in-an-extend-row` and
  `message-resolves-its-text-style-differently-from-ui-label` ride with
  whichever unit next edits `widgets::message` or its callers.
- **The badges it draws** — `at-rest-badge-reports-an-empty-document-as-a-refusal`,
  `band-refusal-still-badges-every-row`,
  `blamed-mates-sends-the-eye-past-the-node-the-fault-says-to-fix`,
  `viewer-panels-disagree-on-a-poisoned-node`,
  `two-is-this-broken-readings-argue-opposite-on-poisoned`,
  `six-viewer-sites-restate-the-empty-document-rule-and-its-badge-policy`,
  `downstream-wording-spells-node-where-node-number-forbids-it` and
  `chrome-weight-is-outside-the-palette`.
- **The subset-policy class** Wave 2's exhaustive guard exposed:
  `a-wildcard-match-decides-viewer-policy-in-five-places`,
  `matches-subset-policy-survives-in-four-viewer-modules` and
  `the-exhaustive-on-purpose-argument-is-restated-twenty-times` — one
  class, one unit.

**Held back, and why.**
`at-rest-badge-reports-an-empty-document-as-a-refusal` is the next row
and did not go this wave: its whole subject is `session.rs`, which is
live in two open PRs. That is the same reason it was held last wave, and
it is a scheduling fact, not a fence — it goes the moment 3052 and 2960
land.

**The board's in-flight column is a claim, not a fact** — see
`work/chrome/log.md`, 2026-09-22. A row is marked `dispatched` here only
after its branch exists on the remote, which is the cheapest thing that
would have caught the phantom this sitting repaired.

## Exit shape

The program does not close on this slate. `crates/viewer/tests/*` is
CHROME's and VIEW's territory both, VIEW's architecture work will keep
landing findings on this ground, and what the carve-out leaves of
`crates/viewer/src` is VIEW's the day CHROME closes. The walk
convention applies when it does; residue re-homes per
`work/README.md`, not into `work/issues/`.
