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

## 2026-09-25 — the parallel node map merges; the tag-vocabulary row dispatched

PR 3145 merged. Its review was a single FULL review with no MAJOR or
MINOR. The reviewer reproduced every red-without-fix row, and both
disclosed deviations proved load-bearing. The residue went to WIRE's
slate (the `PartCache` lock held across a nested rayon join, P1: a
possible deadlock that blocks turning `parallel` on) and to HELPER's
(`on_pool` has six homes, P4).

Next dispatch: `the-third-tag-vocabulary-macro-owes-a-unification-trigger`
(`gather/tag-vocabulary-trigger`). **The orchestrator's rule for the
trigger:** it counts a third macro that expands the same projections
(tag enum, `ALL`, payload→tag read-back). If it has fired, unify; if
not, write the trigger at the macros, where it can be evaluated. Tier:
single STYLE review. It is chosen now because it is the P1 row on ground
no live lane holds. `product-gate-says-verbatim-then-states-the-difference`
and `three-walks-over-the-name-carrying-edges` both touch `product.rs`,
so they wait for the split-halves lane (PR 3256).

## 2026-09-25 — the E-row batch merges

PR 3141 merged. Tier: single STYLE review, with the fix pass on the same
PR. Five rows closed: the slot label, the target roles, the sentinel
region, the refusing scan, and the instance index.
`frame-linear-generic-door-has-no-consumers` is parked on VERDICT's
`the-scalar-lift-convention-mints-doors-faster-than-consumers`.
`wire-rs-accumulation-residue-…` is now priced `D` and covers only the
comment ratio. The review's main finding was that the fix had minted a
third home for the target roles, and it was closed rather than
disclosed: the accessor macros now read `target_roles` through
`target_coord`. Filed: `step-arg-roles-are-spelled-in-three-homes` (P1,
D), for every non-target role, and LIB's
`prose-census-negative-reads-are-guarded-only-by-a-non-empty-allowlist`
(P3).

## 2026-09-25 — the member-space unit goes to its dual review

PR 3143 is green on `24c97a729`. Ev's #3200 and EMIT's #3213 took
containment out of the unit, so what is left is
`FoldConsumption { Split, FragmentedMerge }`: typed refusals with no
offer. It composes with #3213 through
`drop_consumed(look_through_merges(..)?)`. The lane found that the
"interval failures" it had stopped on did not exist. Those runs had been
cancelled by newer pushes; the real blocker was `mergeable_state: dirty`.

Dual review dispatched under `docs/DUAL-REVIEW-PROTOCOL.md` at
`c3129311b`: R1 and R2 run concurrently on the frozen head `24c97a729`
from one brief (scratchpad `dual3143-brief.md`, sha256 `ff9c2451…40c9c`),
and the two differ only in lane name and target dir. Class **M /
STRUCTURAL**. Triage reason, recorded at spec time: it adds refusal
vocabulary to the naming layer, which every member-space declaration
reads and which would be hard to change later.

## 2026-09-25 — the member-space dual adjudicated; split halves to review

**PR 3143 dual.** R1 and R2 both returned APPROVE-WITH-FIXES with no
MAJOR. R1: 180,279 tokens, 37 min (harness). R2: 162,857 tokens, 30 min
(harness). The fix pass works from the union: stale prose, including
DM4's carve-out handing "in pieces" back to this row; the "refuses" that
is false for a split whose every piece is then contained; recursion-arm
mutants that survive every real document (one survives the unit tests
as well); and two hand-kept definitions of "descends". Both reviewers
also executed a pre-existing, order-dependent `Emission` refusal on a
legal 3-member union. It is outside this unit and goes on EMIT's slate
as P0. **Process slip:** the implementer's worktree was removed before
its PR merged, so that lane could not be resumed. The fix pass runs in
a fresh Opus lane from `fix3143.md` in the scratchpad. Implementer
worktrees are now kept until their PR merges. The blinded coder is
dispatched with byte 40.

**PR 3256** (split halves) is green on `97f917982`. A narrowed tie piece
is marked on its row, the gather defers marked rows, and the two halves
merge into the split's one `Entry::Tied`. A lone `Part` publishes what it
did before. Tier: single FULL review. The lane filed
`the-gather-tie-merge-cannot-tell-a-candidate-carried-twice` (P3).

## 2026-09-26 — step-arg roles: the review tier is raised to FULL

PR 3264 (`step-arg-roles-are-spelled-in-three-homes`) is green on
`4ba44817e`. Every `StepArg` role is declared once, in `loop_roles!`,
and one text serves both borrows. The unit was dispatched at STYLE tier
as a unification. **Raised to single FULL review** once it landed:
resolution was rewritten so a refusal finds its role by pointer
identity, with `unreachable!` behind it, and nearly every helper was
deleted. Believing that the resolver's behaviour is unchanged takes
more than reading the diff. Its new field census caught a via x/y swap
that 1606 tests had missed. The lane filed EDIT's
`node-slot-tables-are-spelled-three-times-in-node-rs` (P1) and LIB's
`polygon-door-refuses-at-point-slots-its-corners-do-not-live-at` (P3).

## 2026-09-26 — local CI authorized; GATHER's lanes gate on the local battery

Ev, in chat, 2026-09-26: *"given the severe queueing, feel free to run
ci locally."* GATHER's lanes now run `local-scripts/ci-local.sh`
(change-filtered, the same classifier as hosted CI) once a change is
final, and the orchestrator merges on a green local battery. The hosted
run is left in place. **Serialized, because the box is small** (4
cores, 15 GB RAM, about 24 GB free disk): a battery takes every build
slot (`with-build-slot.sh -x`), so only one runs machine-wide. Other
heavy cargo work also goes through the slot. Each lane deletes its
worktree `target/` after its battery, and one with under 12 GB free asks
before starting. The standing note handed to lanes is
`local-ci-note.md` in the orchestrator's scratchpad.

## 2026-09-26 — split halves merges; the per-part gate's policy is dispatched

PR 3256 merged. Two halves of one split, taken as `Part` roots, now
gather into the split's one tie. Tier: single FULL review. The fix pass
pins over-marking: three overlapping-split shapes still refuse, and each
goes red under the mark-everything mutant. Residue: `the-gather-tie-merge-cannot-tell-a-candidate-carried-twice`
(P3, ours), and REACH's `split-through-the-u-cutter-pockets-inverts-section-loop-roles`
(P0).

With `product.rs` free again,
`product-gate-says-verbatim-then-states-the-difference` is dispatched
(`gather/per-part-gate-one-home`). The row's own analysis sets the
design: the F8/D7 trigger is stated once in `topo`'s at-rest door, and
both callers and the consumer premise (`vertex_rest_contact`) cite it.
Tier: single STYLE review. `three-walks-over-the-name-carrying-edges`
also touches `product.rs`, so it follows this one.

**Correction, same day.** `ci-local.sh` and `gate.sh` are guarded by
`local-scripts/hosted-ci-guard.sh`. It refuses unless
`CAD_LOCAL_CI_OVERRIDE` certifies that hosted CI is *unavailable*.
Hosted CI is queued, not unavailable, so no GATHER lane sets that
sentence. Lanes run the targeted local battery the guard leaves open
instead: `test-fast.sh` or nextest per crate in the closure, clippy, the
demos' clippy, `doc-gate.sh` and `work.py lint`, through the shared build
slot. The orchestrator merges on that plus review and names the basis
at each merge. The one battery launched under the first note was
refused by the guard before it started. Its lane's worktree had also
been removed under it, which is a second instance of the worktree slip
logged above. Orchestrator rule: remove a lane's worktree only after the
lane has reported AND its PR has merged, and never while it may still
be acting on a message.

**Superseded again, by main's #3276** (`dbfff2e9c`). The guard's override
now certifies that "this run should not be hosted", which covers a
hosted queue deeper than the run takes. So GATHER's lanes run the full
`ci-local.sh` battery with
`CAD_LOCAL_CI_OVERRIDE=i-certify-this-run-should-not-be-hosted` once a
change is final. It self-serializes across all build slots. Per the
guard, merging on the local battery before the hosted run lands is the
owner's call. The orchestrator reads Ev's "feel free to run ci locally"
as that call, merges on a green battery, leaves the hosted run in place,
and names the basis at each merge.
