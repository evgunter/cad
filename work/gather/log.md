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

## 2026-09-25 — PR 3141's style review, adjudicated

The review is the style tier, on head `f9678ce63`. Its main finding is the
trap the style brief names: row 2 closed a two-spellings finding by
minting `target_roles` as a third home, while `spec_arg_access!` and
`step_arg_access!` kept their own copy of the target twins. Sent back to
the lane to unify; if unification is not clean, the lane corrects the
prose and files a P1 row naming every home. The lane's parking of row 5
and repricing of row 6, which it had written only as prose, are now
set in their headers in the same fix pass.

**A class noted, not scheduled:** there are three functions named
`output_body` with different meanings (`product.rs`'s narrowing,
`names/interrogate.rs`'s body fetch, and `SplitHalf::output_body`). This
is naming hygiene (P4). No row is filed, because the fix is a rename
best made by whoever next edits two of the three.

**A correction carried to PR 3142:** editor-core does have panic paths.
`unreachable!` is the sanctioned mechanism for an observable kernel bug
(`Cargo.toml` `[workspace.lints.clippy]`, D9's D2 addendum). 3142's
reason for keeping the tie-flush arm typed is corrected to the true one:
a document reaches it.

## 2026-09-25 — Ev's #3200 narrows the member-space unit

EMIT's `[ev]` PR 3200 (merged by Ev on 2026-09-25) rewrites DM4. Contact
is judged pairwise in member space before the fold. A declared contact
whose face the fold consumes WHOLE by containment is satisfied, not
refused. **So containment leaves GATHER's bound.** What stays GATHER's
is a face that survives only in pieces: one split by a later member, or
one inside a merged row that a later step fragmented. The member-space
lane brought main in and dropped its containment arm to match (branch
`gather/member-space-typed-refusals`, `7271fde5e`). The pairwise
pre-pass itself is `wire_union`'s, a WIRE/EMIT unit, and not ours.

## 2026-09-25 — the two-roots unit merges; its P0 residue is dispatched

PR 3142 merged: one body placed under two roots now refuses at the
product's entry as `PlacedUnderTwoRoots`, before any root is read. It
had a single FULL review, and its fix pass landed on the same PR. The review
found a pre-existing FALSE refusal. Two halves of one split, taken as
two `Part` roots over an N2 tie the plane separates, refuse `Naming`,
because a `Part`'s projection narrows the tie to `Unique` before the
gather sees it. Filed P0 as
`product-refuses-split-halves-as-roots-when-a-tie-narrows-to-unique`
and dispatched at once (`gather/split-halves-tie-merge`).

**The orchestrator's direction:** it applies the existing rule. Rows
under one tied name merge into one `Entry::Tied`, which already holds
when the split itself is the root. So the gather has to see the
upstream tie through the projection. A lone `Part` root's published
name does not change. If the lane finds that it must change, it stops
and reports the fork. Tier: single FULL review.

Also filed from 3142: `three-walks-over-the-name-carrying-edges` (P1),
and MSOLVE's `the-solve-accepts-a-body-placed-under-two-roots`.
