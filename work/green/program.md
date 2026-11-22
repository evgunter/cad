---
id: green
kind: program
title: GREEN — a green main and a fast gate: the infra rows that matter now
status: ready
opened: 2026-10-28
group: infra/ci
prefix: green/
tag: (GREEN orchestrator)
paths: [.github/workflows/*, .config/nextest.toml, scripts/ci-filter.py, crates/*/tests/*, crates/test-utils/*]
keep_out: [opened by a priority-seam cut across the infra group (the user, in chat, 2026-10-28 - most infra tracks carried one or two rows that matter beside a long P3/P4 tail) - the rows moved here by git mv with their ids and bodies, and their old programs keep their tails, CIW owns hosted CI and the workflows as a whole and GREEN shares that ground only for the latency and main-red rows on its slate, so run scripts/work.py territory on a unit branch and announce the seam on CIW's log where a change is coupled, a main red that a kernel program's merge caused is still fixed here when it sits in a row on this slate but the kernel program is told on its log, TCOST merged in on 2026-11-20 (docs/doc-ledger/tcost-merges-into-green.md) - it held no P0 or P1 row, from TCOST: territory says whose ground a path is and never whose slate a finding goes on — crates/*/tests/* matches every test file in the tree so work.py territory names this program for any of them, from TCOST: a test that is red on main goes to GREEN (work/green/) or to the program whose merge moved it, from TCOST: a helper or fixture spelled several times goes to HELPER or FIXTURE, from TCOST: S-TINT shares this program's crates/*/tests/* and crates/test-utils/* territory deliberately — the fence is the QUESTION and not the path (work/ciw/plan.md), from TCOST: a row justified by a second is this program's and a row justified by a claim that cannot fail is S-TINT's, from TCOST: no test is deleted for being slow alone — every deletion names the row that owns the claim, from TCOST: no fixed seed and no ignore on a row that gates, from TCOST: a row keeps its own fixture code only where its claim needs its own derivation, from TCOST: nothing gates on a millisecond — cost is reported and not thresholded, from TCOST: CI build knobs (profile/cache) are out unless a unit's measurement makes the case in its own PR]
---
**Main is green and the per-PR gate is fast.** Every other track's
work goes through the PR gate and lands on main, so two things here
cost everyone: a gate that takes 23 minutes at the median against
a 15-minute bar (`memories/orchestration-model.md`, "Friction is a
finding"), with the nightly's latency alarm red; and a nightly that
has been red on main for a week, so a new regression arrives unseen
among the old ones.

The slate is the latency work (P0), the main reds on infra ground
(P1), and the cheap rows that make the gate untrustworthy: flaky
guards that turn unrelated PRs red, tests red on main under a
configuration no row runs, gate self-tests no job runs.

**A red is greened, not fixed, here** (the user, in chat, 2026-10-28:
*"as the infra orchestrator, it is more centrally your job to make
things that are currently red go back to green, though if this
requires any actual work to fix then it should be done instead by
pinning the failure or otherwise disabling the test and filing a row
to actually fix it on the appropriate program"*). An explained,
deliberate move is re-blessed. Anything else is pinned at what main
produces (or `#[ignore]`d with the row it waits on), and the fix row
goes on the program that owns the code that moved, with the bisected
merge and a reproduce command. The point is scope, not speed (the
user: *"less 'fast' and more 'without having to get into the weeds on
a bunch of different programs'"*): GREEN reads a red far enough to name
its merge and its owner, and the owner works out the cause.

Charter and order: `work/green/plan.md`; narrative in `work/green/log.md`.

**GREEN acts on the nightly triage, not beside it** (2026-10-29). A
daily nightly-triage session (`triage/nightly-<date>` PRs) attributes
each red to its merge, files or annotates the owner's row, and fixes the
trivial ones (doc links, stale counts). GREEN starts from that PR's
table and greens what the triage left red (the pins, the disables, the
re-blesses), citing the triage's rows rather than filing its own. On
2026-10-29 a GREEN lane re-did the triage's rustdoc fixes in parallel
(#6022, closed as superseded by #6018).

## From TCOST (merged 2026-11-20)

S-TCOST — test-suite cost

**A row belongs here when its subject is what the suite COSTS**: the
time a test, a suite or the test build takes, or the machinery that
decides which tests a run pays for (the change filter, `gated_to!`
markers, the EFFORT dial, the slow set's membership). That is the whole
test, and a path is not part of it. This program's `paths` match every
test file in the tree, so `work.py territory` answers "tcost" for any
finding in a test, and five main reds and several duplicated-helper
rows were filed here on that answer alone (re-homed 2026-11-14). File
by the question instead:

- a test red on main: GREEN (`work/green/`), or the program whose merge
  moved it;
- a claim that cannot fail, or a guard that cannot see: S-TINT;
- one helper, fixture or walk spelled several times: HELPER or FIXTURE;
- the kernel being slow in a way a user pays too: the kernel program
  that owns the code, unless the row's evidence is a test's time and
  its fix is sized by it.

Make the suite cheaper without losing its power to detect defects: the six
levers Ev named (CI red history, per-test timing history, gating file-specific
suites to their files, merging tests that share initialization, deleting
covered tests, simpler objects), plus build-side levers. Three read-only
censuses cut the units, largest share first; every unit's PR states its
before/after from hosted runs. The A/B protocol is off for this program
(Ev, in-chat 2026-09-12): a unit gets a style review, and a full review
where its risk of being WRONG argues for one. Charter, rulings, review
track and levers: `work/green/plan.md`; narrative in `docs/doc-ledger/tcost-merges-into-green.md`.
