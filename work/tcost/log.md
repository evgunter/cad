# S-TCOST log — test-suite cost

Narrative record; the plan is `docs/S-TCOST-PLAN.md`. Convention as in
the other programs: seam entries at pipeline seams, unit entries at
merges, the tail is the live state.

## Opening state (2026-09-02)

Opened on Ev's direction (in-chat, 2026-09-02) by a fresh
orchestrator on a remote container. Charter and the three rulings
Ev gave on the orchestrator's questions are in the plan.

**Operational facts, recorded once:**

- **Branch prefix `tcost/`**; orchestrator branch `tcost/orchestrator`;
  the harness-designated session branch
  `claude/test-suite-opus-optimization-q8u962` carries the opening PR
  and is otherwise unused.
- **A/B ordinal band: S-TCOST = 1400–1499**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry in this same PR. The band is
  used only by kernel-logic units; test-only units record no row.
- **Three censuses dispatched at opening** (Opus, read-only, reports
  under `~/tcost-work/`): CI red history, CI per-test timing history,
  local build profile. Their findings are summarised here when they
  land; the raw material stays lane-private.

**Decisions taken unilaterally:**

- The program name and prefix (`S-TCOST` / `tcost/`); the band
  1400–1499 as the next free band per the banding entry.
- The gate mechanism's shape (TCOST-1 spec): marker-at-the-suite,
  derived selection, fail open — argued from the same siting rule the
  nightly demotion marker and `scripts/nightly-only-selection.py` use
  (a central roster drifts; a marker at the test cannot).

## Tracker migration (2026-09-03)

The plan and this log moved from `docs/S-TCOST-PLAN.md` / `docs/S-TCOST-LOG.md`
to `work/tcost/plan.md` / `work/tcost/log.md`. The slate now lives in this
directory's item files and in `work/STATUS.md`; this log stays the
narrative. Items created at migration from the open unit PRs (this log had
only its opening entry): TCOST-1 (dispatched, PR 1612), TCOST-2
(dispatched, PR 1609), TCOST-3 (dispatched, PR 1614), TCOST-4 (dispatched,
PR 1608), TCOST-B1 (dispatched, PR 1616).
