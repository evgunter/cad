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

## 2026-09-28 — the name-carrying edge set lands (PR 3321)

`three-walks-over-the-name-carrying-edges` and
`select-refusal-coverage-is-not-compiler-enforced-from-the-test-crate`
closed on PR 3321. `verbatim_edge` (`names/role.rs`) is the one home
both editor-core walks read; the compiler holds the three together.
`SelectRefusal` has an in-crate census beside the enum.

Single STYLE review, verdict mergeable. All four claims held. The
reviewer mutated the census and confirmed it goes red for a missing
sample. Fix pass: S1 (the doc no longer says "every walk"), S3 (the
display-contract comment says what the census covers), S4 (the
census uses `test_utils::census::set_difference`), S6 (`Intact`
carries no unused field). No change: S5, S7, S8, S10.

Filed from the review, as classes:
- `verbatim-edge-is-not-tied-to-the-evaluator` (P1, M). Nothing ties
  `verbatim_edge` to what `eval/wire.rs` passes through; a runtime
  guard over the evaluated corpus is possible. The lane's "no runtime
  test can exist" was corrected in the PR body.
- STACK's `in-crate-census-hand-writes-the-set-comparison` (P4, E).
