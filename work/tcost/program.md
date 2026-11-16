---
id: tcost
kind: program
title: S-TCOST — test-suite cost
status: ready
opened: 2026-09-02
group: infra/tests
prefix: tcost/
tag: (S-TCOST orchestrator)
ab_band: 1400-1499
paths: [crates/*/tests/*, crates/test-utils/*, scripts/ci-filter.py]
keep_out: [territory says whose ground a path is and never whose slate a finding goes on — crates/*/tests/* matches every test file in the tree so work.py territory names this program for any of them, a test that is red on main goes to GREEN (work/green/) or to the program whose merge moved it, a helper or fixture spelled several times goes to HELPER or FIXTURE, S-TINT shares this program's crates/*/tests/* and crates/test-utils/* territory deliberately — the fence is the QUESTION and not the path (work/tint/plan.md), a row justified by a second is this program's and a row justified by a claim that cannot fail is S-TINT's, no test is deleted for being slow alone — every deletion names the row that owns the claim, no fixed seed and no ignore on a row that gates, a row keeps its own fixture code only where its claim needs its own derivation, nothing gates on a millisecond — cost is reported and not thresholded, CI build knobs (profile/cache) are out unless a unit's measurement makes the case in its own PR]
---

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
track and levers: `work/tcost/plan.md`; narrative in `work/tcost/log.md`.
