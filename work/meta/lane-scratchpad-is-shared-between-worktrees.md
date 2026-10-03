---
id: lane-scratchpad-is-shared-between-worktrees
kind: issue
title: the per-session scratchpad is shared between concurrent lanes, and two lanes had files overwritten mid-task
status: deferred
opened: 2026-09-11
priority: P3
cost: E
---

Filed in `work/issues/` because the owner is genuinely undecided: the
subject is neither a workflow nor a script but how concurrent agent
lanes are provisioned, and the documents that govern it
(`docs/prompts/implementer-discipline.md`'s local-work bullets,
`memories/agent-lane-operations.md`) belong to two different parties.

## Deferred (Ev, 2026-09-11): shape 3, no rule

**Ratified as not-now: the script/orchestrator side only, no sentence in
`docs/prompts/` and none in `memories/`.** Ev, on the proposed rule:
*"i lean against this kind of rule because it's not at all specific to
this project."*

That is the right cut, and the comparison he asked for is why. The
`CARGO_TARGET_DIR` rule this row leans on is project-specific in
everything load-bearing about it — cargo's target-dir semantics across
git worktrees, this repo's `.gitignore` coverage of `/target` but not an
arbitrary in-tree name, merge-only history making the 114-file incident
unfixable except by abandoning the branch, and two named incidents with
named consequences. The rule proposed here has none of that: *name your
scratch files for your lane* reads identically in any repository, and
both instances behind it ended with no work lost.

So this row is **shape 3** of the three it lists, taken for the reason
shape 3 gives: recorded so the third instance is not re-derived from
scratch. It is deferred, not closed — a lane that loses work to this,
rather than catching it, is a new fact and re-opens the question.

## What happened

Two CIW lanes running concurrently in separate git worktrees on
2026-09-11 independently reported that **the session scratchpad
directory is shared between them**, and that their files were
overwritten mid-task by another lane:

- one lane had its PR-body draft overwritten between creating the PR
  and editing it — it caught the swap by diffing against the live API
  before patching, and the correct body landed;
- another lane had two scratch files overwritten and reported that
  "lanes should use unique filenames there".

Neither lost work. The first one is the shape worth recording: the
overwrite landed in a file whose next use was **a write to GitHub**,
so the failure mode available here is one lane publishing another
lane's text under its own PR number.

## Why it is not covered by what exists

`docs/prompts/implementer-discipline.md` already has the analogous
rule one level over, and it is written about a different directory:
*"Use your own `CARGO_TARGET_DIR`, never one shared with another
lane… Keep that target directory OUTSIDE the worktree."* That rule
exists because a shared target directory served one lane another
lane's binary — twice in one wave, once behind a green claim over ten
broken assertions. The scratchpad is the same hazard with a shorter
blast radius and no rule at all.

The worktree isolation that makes the lanes safe from each other in
`git` says nothing about a directory outside every worktree.

## Shapes

1. A sentence in `docs/prompts/implementer-discipline.md` beside the
   `CARGO_TARGET_DIR` rule: a lane's scratch files are named for the
   lane, or written inside its own worktree under a git-ignored path.
   CIW may amend that file per `work/meta/program.md`'s `keep_out`,
   though that cession is scoped to §2.
2. A per-lane subdirectory handed to each lane at dispatch, which is
   an orchestrator convention rather than a repo change and fixes
   nothing for a lane dispatched by someone who has not read this.
3. Nothing, on the argument that both instances were caught and the
   cost is a re-write. Recorded so the third instance is not
   re-derived.

## What is verified

The two lane reports, and that neither lost work. **Not verified:**
whether the sharing is per-session or wider, and whether two lanes in
DIFFERENT sessions collide the same way — which is the case that
would decide between shapes 1 and 2.

## Re-homed to CITE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

CITE collects the rows about the project's own text and harness rather
than its kernel: citations that rot, numbers that were reissued, and the
paperwork a lane runs on. This row is one of them.

Its class at the cut was **M** — one-sentence rule, but owner undecided,
memories needs Ev, deciding fact unverified. The class is a dispatch
estimate made by reading the row against the tree on 2026-09-11, not a
verdict on the finding, and a lane that finds it wrong says so in its
PR. The id, the `track:` letter where the row carries one, and the body
above are unchanged by the move.

## Re-homed to META (2026-09-12, by the CITE orchestrator)

Moved as residue at CITE's close. `work/README.md` says a closed
program's residue is re-homed before the sweep and never left behind,
and that a closed program may hold only closed items — this row is
`deferred`, not closed, so it had to move or be decided.

**META by subject.** The row's own opening says the owner is undecided
because the documents that govern it belong to two parties. One of those
two is now moot: Ev ruled out the `memories/` sentence (2026-09-11),
which leaves `docs/prompts/implementer-discipline.md` as the only
document in play, and that file is META's `paths`. The row's shape 1
already pointed there.

**It stays deferred, and the ratification is Ev's, quoted on the row
above:** *"I lean against this kind of rule because it's not at all
specific to this project."* No `blocked_on`, because it waits on no
trigger — it was decided against for now.

**What would re-open it**, stated so META does not have to re-derive it:
a lane that LOSES work to a scratchpad collision. Both instances behind
the row were caught — one by diffing against the live API before
patching, one by noticing the overwrite — and "caught twice" is what the
deferral rests on. A third instance that is caught changes nothing; one
that is not is a new fact.

## Third instance, 2026-09-21 (AUTHOR) — caught, so the deferral STANDS

Reported unprompted by AUTH-1's implementer lane. Two AUTHOR lanes in
separate clones, dispatched from one orchestrator session, both wrote a
CI-poll script to the session scratchpad under the same filename.
AUTH-1's was silently replaced by AUTH-2's, and **for one poll round
AUTH-1 read job counts for AUTH-2's run believing they were its own.**
It noticed, moved its poller to a lane-private path, and re-verified its
own run from scratch against its head SHA.

**No work was lost, so by this row's own re-opening rule nothing
changes**: *"A third instance that is caught changes nothing; one that
is not is a new fact."* Recorded for the reason shape 3 gives — so a
fourth is not re-derived — and because two things about it are new.

**1. The blast radius is larger than the row's two instances.** Both
earlier ones were drafts, where the failure mode was one lane
publishing another's text. This one landed in the file feeding a
lane's reading of **hosted CI, the verification of record**: the
failure available is a lane reporting green from a run that is not its
own. That is the `CARGO_TARGET_DIR` hazard exactly — *"behind a green
claim over ten broken assertions"* — reached through a different
directory. It does not re-open the row, because the deferral rests on
"caught", not on "cheap"; it does say the third shape's *"the cost is a
re-write"* is not the whole of what is at stake.

**2. One unverified fact is now half-settled.** `## What is verified`
records as NOT verified *"whether the sharing is per-session or
wider"*. Both colliding lanes here were subagents of ONE orchestrator
session, so this confirms the sharing is at least per-session across
worktrees. The cross-SESSION case — the one that would decide between
shapes 1 and 2 — is still unverified and was not exercised.

**Shape 2 is in force in AUTHOR regardless**, as the orchestrator
convention it is: every AUTHOR lane is now handed a lane-private
scratch path at dispatch. That fixes nothing for a lane dispatched by
someone who has not read this, which is the limitation shape 2 was
always recorded with, and is not an argument for shape 1 — Ev ruled
against a `memories/` sentence on 2026-09-11 and nothing here
disturbs that.
