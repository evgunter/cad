# QA-2 — the matrix says what it did (#1128, #1122's visibility half, #1051 verification, #1204's minimum)

Unit spec, S-QA program (`docs/S-QA-PLAN.md`; charter
`docs/WORK-STREAMS-2026-08.md` §S-QA). Binding alongside
`docs/prompts/implementer-discipline.md` — read that in full first.

## Premise, and verify it before anything else

Three issues about the same instrument, in three states. **Verify
each against your merge base before implementing** — this tree moves
under specs daily.

- **#1128 (open, unfixed at dispatch)**: hosted CI's two nextest run
  steps (`ci.yml` ~:1666 default lane, ~:2061 interval lane) pass
  neither `--fail-fast` nor `--no-fail-fast`, so a red run reports
  ~1 failure per shard, not the failure surface (measured 22 vs 1-2
  in the issue). The `fail-fast: false` at the matrix level is the
  shard-cancellation setting and does not touch this.
- **#1122 (partially paid at dispatch)**: `scripts/ci-filter.py`'s
  header now says "A PIN IS ANNOUNCED ON STDERR … a pin no reader
  can see is …" — some or all of the issue's option 3 (say it
  pinned, and why) may already be landed. `_forces_interval`
  returns a reason string. Establish exactly what is landed, what
  reaches the hosted run's visible output, and what gap remains.
  The issue's option 2 (`LANE=both` on pin) is EXPLICITLY NOT in
  this unit — it awaits a ruling (plan §Rulings Q2).
- **#1051 (landed 2026-08-28, open)**: the request-a-point feature
  (`workflow_dispatch` inputs + `CI-Config:` head-commit trailer +
  `CONFIG_SOURCE` line) is in the tree —
  `docs/CI-MINUTES-2026-08.md` §"asking for a point" is the record.
  This unit VERIFIES it against the issue's three notes and reports;
  the orchestrator closes the issue on your record.

## Deliverables

1. **`--no-fail-fast` on both sharded nextest run steps** (#1128's
   option 1), with the issue's cost argument checked rather than
   inherited: a green run does identical work; only a red run does
   more. The two targeted interval rows (~:2069/:2075, single-test
   filters) get a one-line disposition (flag pointless or added —
   say which and why).
2. **The mechanism confirmed against the pinned nextest 0.9.140**,
   not trusted from the issue (its own caveat asks this): a scratch
   crate or existing suite with two planted failures, run under the
   pinned version with and without the flag; record the observed
   default. If the default is NOT fail-fast, the issue's premise is
   wrong — stop and report before changing ci.yml.
3. **The run prints its mode** (#1128's option 3): the step output
   carries the fail-fast mode in a form a reader of a red run sees
   without archaeology (the run line itself carrying the flag
   suffices if it is visible in the step's log header; otherwise an
   explicit echo).
4. **#1122's visibility half, verified end-to-end**: from a hosted
   run of a branch that trips `_forces_interval` (your own PR can
   plant a scratch `*_interval.rs`-touching commit and revert it, or
   read a recent real run), confirm the pin announcement appears in
   the filter job's visible output with the reason, and that
   `CONFIG_SOURCE` distinguishes a pin from a draw. Fix any gap
   between "announced on stderr" and "a reader of the run can see
   it". Report what remains for option 2 so the ruling lands on
   facts.
5. **#1051 verification report** against the issue's notes: (a) a
   requested point is recorded as requested, not an unbiased draw
   (`CONFIG_SOURCE` per dimension); (b) a dispatch run cannot
   silently skip the job the requester wanted (the `scope` input's
   semantics); (c) omitted inputs fall back to the draw. Exercise
   the trailer path on one of your own pushes (`CI-Config:` on the
   head commit) and cite the run. Gaps are fixed if small, filed if
   not.
6. **#1204's minimum (added at dispatch+1, from the PCURVE
   orchestrator's report on PR #1228)**: a draft PR's run rewrites
   every `RUN_*` flag to `false` and still reports success with
   `TIER`/`LANE`/`CARGO_SCOPE` left truthful — three consecutive
   greens on a 19-kernel-file branch gated nothing, and two
   experienced readers misread the same run in different
   directions. Take the issue's option (1): the draft skip prints
   an unmissable `GATE SKIPPED: draft PR — no RUN_* flags set`
   line in a step with no `if:` (the `CONFIG_SOURCE` precedent),
   and the `ready_for_review` escape is documented beside the skip
   in `ci.yml`. ASSESS option (2) (a non-success conclusion for a
   gate that did not run) and report its shape and cost in the PR
   description — do not take it unilaterally; the draft-skip
   behaviour itself (F5, drafts are cheap) stays.
7. **PR description** carries: the measured default from
   deliverable 2, the cost statement, the before/after of a red
   run's failure surface if one is cheaply constructible (a planted
   2-failure commit, red run recorded, then reverted in the same
   PR — cite both run IDs), and what remains open on #1122.

## Out of scope / fences

- `LANE=both` on pin (Q2's ruling), any change to what the sampler
  draws, any new CI job, k-lint rows, render lanes.
- `local-scripts/ci-local.sh` parity: touch only if
  `check-ci-mirror-parity.py` demands it for your ci.yml edit; keep
  the mirror honest, do not restructure it.

## Verification

- `python3 scripts/ci-filter.py`'s own self-tests (the file carries
  them — find and run the invocation its header names), plus
  `python3 scripts/check-ci-mirror-parity.py`, both clean.
- Hosted CI is the verification of record: your PR's run must show
  the printed mode and (if constructed) the planted-red
  demonstration. An instrument change verified only by reading the
  script is this program's own defect class — show it firing.
- No cargo builds expected beyond what the planted-failure
  experiment needs; keep it to one small crate and clean up.

## Lane discipline

- Branch `qa/2-matrix-speaks` (already created for you); commit and
  push after every coherent step. Open the PR non-draft when ready
  for the gate (drafts run nothing).
- **Blinding**: NO `Co-Authored-By` trailer in lane commits. If one
  lands in a pushed commit, note it in the PR body and carry on —
  never rewrite history, never stop the unit over it.
- Foreground rule, both halves: never arm background waiters or
  chains for your own runs, AND launch any job that could outlive a
  600 s foreground call `setsid`-detached, then poll it in the
  foreground. Never end a turn with background work still active.
- Working artifacts go in YOUR worktree or lane-private paths,
  never the shared session scratchpad.
- Final report ≤150 lines: what landed, the measured nextest
  default, the #1122 end-to-end finding, the #1051 verification
  verdict, and the run IDs.
