# META — the tracker and the process instruments (plan)

**STATUS: OPEN (2026-09-04).** Live state is `log.md`'s tail and the
item files beside this plan, never this file.

Branch prefix: **`meta/`**. Away-channel tag `(META orchestrator)`.
A/B band **2800–2899**, claimed for bookkeeping — this program runs no
duals (its units are infra and prose, the CIW/CHROME posture).

## Why this program exists

The 2026-09-03 cut divided the tree by file territory and gave eleven
tracks an owner each. It did not divide **the instruments it used to do
that**. Measured 2026-09-04 against every open program's `paths`:

- `scripts/work.py` — the tracker's one tool — is in no program's
  territory. CIW's `keep_out` says so explicitly ("`scripts/work.py`
  is the tracker's own and changes only with `work/README.md`"), which
  names an owner that did not exist.
- `docs/prompts/` (the implementer brief, the reviewer-style brief) is
  in no program's territory, and is edited by whoever last needed a
  rule changed.
- `docs/MODEL-AB-LOG.md` is written by every program and owned by none.
  Its rules — the band allocation, the protocol, the **stopping rule** —
  have no reader.
- `docs/DOC-LEDGER.md` is the done-state of record for every closed
  program and has no owner between exits.

The cost is not hypothetical and it is on file three times over: a
territory check blind to the exact collision it exists to catch, a
pre-registered stopping rule passed about nine times without anyone
noticing, and two acceptance clauses in live specs that instruct a
coverage reduction and red the gate if obeyed.

## The fence, and why it does not collide with CIW

CIW owns `.github/workflows/*`, `local-scripts/*`, the demo shell and
Python, and an enumerated list of `scripts/`. It owns **the runs**.
This program owns **the tracker and the briefs**. The two fences were
already written against each other from CIW's side before this program
existed — its `keep_out` cedes `scripts/work.py` — and the only place
they touch is `docs/prompts/implementer-discipline.md` §2, which
describes what a CI run gates. That paragraph is **CIW's to amend
whenever its runs change, without waiting on this program**; the rest
of the file is this program's. Written on both sides, per the rule the
CURVED/S-BOOL fence set.

`scripts/gates/*` is code-quality Track K's and `scripts/ci-filter.py`
is S-TCOST's. Neither is touched here.

## The slate

**Three items, re-homed at opening** (header edit and `git mv`, ids
unchanged):

1. **`territory-cannot-see-a-path-two-programs-both-claim`** (E, the
   opener) — `territory` reports a changed path another program owns
   only when the branch's own program does *not* claim it, so the one
   case that matters — two programs claiming one path — is the one case
   it is silent on. Found by FIX tripping it on `transform.rs` against
   SHELL, and it went unnoticed the same way for the whole day. Two
   candidate fixes are on the item; **(1), a lint rule, is the one that
   would have caught the instance**, and it fires on the day the second
   program opens rather than on the day a lane happens to run
   `territory`. It needs one decision — whether a deliberate overlap
   may exist and whether `keep_out` prose is the sanctioned way to
   record one — and BOOL/CURVED and MSOLVE/DOCM are both live instances
   that say yes. So: a lint rule that errors on an unrecorded double
   claim and passes one recorded in `keep_out`.

   This is the opener because **every other program's fence depends on
   it** and because TOPO, opened the same day, had to enumerate twenty
   file paths instead of writing one glob for exactly this reason.

2. **`ab-log-v6-stream-is-past-its-stopping-rule-unadjudicated`**
   (`[ev]`, and the one urgent thing here) — Protocol v6 pre-registers
   a stop at eight adjudicated unilateral-MAJORs or twelve new pairs,
   whichever first. **109 v6 dual rows are recorded**; the twelfth fair
   pair was passed around 2026-08-29/30. No row records the rule
   firing, no notification is recorded, there is no Protocol v7, and
   the running tally is stated inconsistently by four different rows
   and then abandoned. Ev's call in two parts: run the blinded
   adjudication over the candidate list, and then either STOP per the
   pre-registered rule or write v7 and say why. Every program is
   spending two reviews per unit at ~250–320k tokens each against a
   rule that said to stop.

   **The item does not carry `needs_ev: true` yet** — the flag goes on
   in the commit that opens the `[ev]` PR, per `work/README.md`
   ("whoever opens an `[ev]` PR arranges to be woken by comments on
   it"). Until then it is invisible on the board's "Waiting on Ev"
   section, which is exactly the shape of the failure it describes, so
   that PR is this program's first action and not its second.

3. **`stale-track-t-citations-in-fillet-and-cert`** — three sentences
   in FILLET's and CERT's slates describing a Track T arrangement that
   has since changed. Neither is this program's to edit (one file, one
   item), and that is exactly the point: the item is the routing
   record, and the class it names — a citation across a fence going
   stale with nothing that reads it — is what this program is for.

**Two registers, custodied not executed:**

- **`m6-carried-items-register`** — every carried item from the M6 exit
  walk, rows owned by S-MATE, VERBS, S-CERT and the code-quality tracks
  at once.
- **`decide-flagged-dimensional-debt-inventory`** — the remaining
  `decide_flagged` families (F2 ×4 in `validate.rs`, now **TOPO's**;
  F10 in `transform.rs`, **SHELL's**; F13 the cone charts; F14/F15 the
  editor-core wire), each retiring as an opportunistic rider on
  whichever unit next touches its family, with the census count as the
  tripwire.

**Custody means keeping them accurate and routing their rows to the
owners, and nothing else.** This program does not execute a register
row; it is the thing that stops a register from being a file nobody
reads. Each is re-surveyed when a named owner's program closes, and
each closes when its last non-triage row is closed or has moved onto a
program's slate.

## Not taken, and named so it is not re-derived

**`fillet-specs-require-a-narrowing-ci-config` was closed on main
before this program opened, and stays in `work/issues/`.** It was on
this slate in the sweep's first draft; merging main showed CIW's
`delete-config-trailer` unit had already swept the clause out of both
FILLET specs and three more (`PCURVE-P2`, `EXCH-H1`, `FILLET-H5`) by
deleting the `CI-Config:` trailer path itself. A closed item is not
work, so it does not go on a program's board.

**The class it was an instance of is still this program's**, and now
has a worked example to point at: a spec's `## Acceptance` clause
silently falsified by a CI change, with nothing that reads the two
against each other. Its `## Closed` section records the sharper half —
after the sweep the stale instruction does not red, it is *inert*,
"which makes it quieter, not safer". That is the argument for the
standing instruction living in `docs/prompts/implementer-discipline.md`
§2 and specs pointing at it, and it is why §2 is on this program's
fence with CIW amending it.


**`tracker-has-no-status-for-an-unscheduled-trigger` is this program's
class and stays on VIEW's board.** It asks what `status` an item whose
trigger is neither an item nor a PR should carry — "parked lies and
open overstates" — which is a question about the tracker's own
vocabulary and would otherwise be item 5 here. It is not moved because
it carries `needs_ev: true` with a live `[ev]` PR
(`view/ev-tracker-fired-trigger`), and re-homing an item out from under
the orchestrator who is waiting to be woken by comments on its PR
breaks the one channel `work/README.md` gives that conversation. When
Ev rules and VIEW clears the flag, the ruling lands in `work/README.md`
and `scripts/work.py`'s status vocabulary — **both this program's
files** — so the build comes here even though the question does not.

The same applies in general: a tracker-class item already sitting on a
program's board with an open `[ev]` PR is left there and the build is
handed over at the ruling. This program does not collect items; it
owns the instruments.

## Order

`territory-cannot-see-a-path-two-programs-both-claim` first (E, and
everyone's fence depends on it), with the `[ev]` PR for the A/B
stopping rule opened **in parallel on day one** rather than behind it —
it is a spend that is running now, and the answer is Ev's, not a lane's.
Then the two routing items, which are announcements rather than work.
Then the first register re-survey, which is the honest measure of
whether custody is worth anything.

## Review posture

CIW's, and for the same reason: infra and prose units, no A/B row. One
subagent style review per unit, plus a correctness reviewer where a
unit changes what `work.py lint` accepts or rejects — the lint rule in
item 1 is one, since a false positive there blocks every program's CI.
