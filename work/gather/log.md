# GATHER — the log

## 2026-09-20 — opened

Cut out of WIRE, which was carrying 85.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed WIRE's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

13 rows arrived by `git mv` with their ids, bodies and history
unchanged. WIRE keeps its band 3700-3799; band 8400-8499 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## 2026-09-24 — an orchestrator picks the track up

Status `ready` → `active`. This orchestrator runs in a cloud session, so
PR subscriptions and scheduled check-ins stand in for the local monitors.
Unit lanes push `gather/<unit>` branches and open one PR each.

**Review posture, the question `plan.md` left open.** Each unit's tier is
set at spec time under `memories/orchestration-model.md`'s review tiers.
Every lane runs on Opus. The v7 triage stream is not re-opened for this
slate.

**First wave, run in parallel:**
- `product-refuses-naming-when-one-instance-is-placed-under-two-roots`:
  **single FULL review**. It adds an earlier refusal in the recipe's
  vocabulary and removes a late one, so correctness is at risk, but the
  design decision is already made (PR 2677).
- `member-space-look-through-stops-at-splits-containment-and-fragmented-merges`:
  **DUAL review**. It adds typed refusal vocabulary to the naming layer,
  which every member-space declaration reads and which would be hard to
  change later.
- The six `E` rows go in one batch unit, as `plan.md` suggests:
  **single STYLE review**. They are mechanical and local.

Load is 31/30. It is not split: this wave takes about 16 points off the
dispatchable count, which leaves a slate one session can hold.
`parallel-node-map-loses-the-funnel-and-the-symbolic-session` is next.

## 2026-09-24 — the parallel node map is ruled and dispatched

The orchestrator ruled the row's open question. **The recordings** are
handled by a per-node `k_stats::detached` frame, spliced back in level
order. **The session** is handled by a serial fallback while one is
installed, using the test `topo::props` already uses
(`decisions_are_thread_portable`). Per-node sessions were rejected: they
would re-scope what a session's table covers, which is a design change
this row does not need, and the fallback makes a parallel run decide
exactly as the serial one does. The lane hoists the idiom into one home
rather than minting a second copy. **Single FULL review**: concurrency
and the D9 bit claims have to be believed, not just read. It runs
alongside the first wave; its only ground in common with the others is
`editor-core`.

## 2026-09-25 — the lanes resume after the weekly limit

All four lanes stopped at the weekly usage limit on 2026-09-24 at about
05:30 UTC, each with its work pushed and its worktree clean. They were
resumed on 2026-09-25 from their transcripts. PR 3142's full review had
already been adjudicated and its fix pass was mid-flight.

Incoming from main: `product-instance-output-body-index-saturates` (P4,
E), filed by the index-narrowing sweep. It is folded into the E-row
batch, because it is a one-site change of the same shape the batch
already carries.
