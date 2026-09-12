---
id: tcost
kind: program
title: S-TCOST — test-suite cost
status: open
opened: 2026-09-02
area: infra
prefix: tcost/
tag: (S-TCOST orchestrator)
ab_band: 1400-1499
paths: [crates/*/tests/*, crates/test-utils/*, scripts/ci-filter.py, scripts/slowest-tests.py, scripts/base-test-listing.sh]
keep_out: [S-TINT shares this program's crates/*/tests/* and crates/test-utils/* territory deliberately — the fence is the QUESTION and not the path (work/tint/plan.md), a row justified by a second is this program's and a row justified by a claim that cannot fail is S-TINT's, no test is deleted for being slow alone — every deletion names the row that owns the claim, no fixed seed and no ignore on a row that gates, reviewer suites that pull their weight keep their independence from shipped fixtures, nothing gates on a millisecond — cost is reported and not thresholded, CI build knobs (profile/cache/sharding) are out unless a unit's measurement makes the case in its own PR]
---

Make the suite cheaper without losing its power to detect defects: the six
levers Ev named (CI red history, per-test timing history, gating file-specific
suites to their files, merging tests that share initialization, deleting
covered tests, simpler objects), plus build-side levers. Three read-only
censuses cut the units, largest share first; every unit's PR states its
before/after from hosted runs. The A/B protocol is off for this program
(Ev, in-chat 2026-09-12): a unit gets a style review, and a full review
where its risk of being WRONG argues for one. Charter, rulings, review
track and levers: `work/tcost/plan.md`; narrative in `work/tcost/log.md`.
