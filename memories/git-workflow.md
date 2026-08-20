---
name: git-workflow
description: Merge-only workflow — messy commits fine, PR descriptions carry the documentation, agents self-merge to main
metadata:
  type: feedback
---

Two streams: **commits** are the honest record of actual work done
(frequent, potentially messy, never rewritten); **PR descriptions** are
the sanitized, logical documentation of the change. Merge-only — no
squash, no rebase, no force-push, no history rewriting. Push branches to
the private remote freely. Agents own this greenfield codebase and are
encouraged to merge their own PRs to main.

**Why:** Evan wants both an unfalsified history of what actually happened
and a clean logical narrative of what changed — keeping them in separate
streams beats compromising either.

**How to apply:** commit early and often without polishing messages; put
the careful writeup in the PR description; merge with a merge commit
(`gh pr merge --merge`, never `--squash`/`--rebase`). Exception (confirmed
by Evan): PRs that ratify open design questions in DESIGN.md are design
conversations — wait for Evan's sign-off before merging; routine
implementation self-merges. See [[cad-working-style]].

**Push early, push often (added 2026-07-16, M1):** implementer agents
push their branches to origin early and often — after each meaningful
commit, before review — because Evan follows work-in-progress remotely.
Don't sit on local commits until the work is "ready".

**Stacked-PR gotcha (learned M0, 2026-07-16):** deleting a merged PR's
branch while another open PR still targets it makes GitHub auto-CLOSE
the stacked PR, and a PR whose base branch was deleted cannot be
reopened — a fresh PR must be opened (losing thread continuity).
Retarget stacked PRs to main BEFORE deleting the base branch, or just
don't delete branches (private remote; cheap to keep).

**Merge gate = hosted Actions (2026-07-25)**: PR checks green =
mergeable. gate.sh is a billing-outage FALLBACK only (its runner
target/ is not kept warm; cold rebuild on fallback use). Agents
never run gate.sh; reviewers run targeted cargo lanes in their own
clones. Rationale: Actions runs the same matrix in parallel on
GitHub hardware, on the PR's merge ref; the local gate was
serialized (sum-of-rows), held a cache big enough to matter, and
contributed to two disk-crash incidents on the developer box. Caveat (still true 2026-07-25): main has NO branch
protection, so `gh pr merge --auto` merges IMMEDIATELY — verify
the checks are green yourself; never rely on --auto to wait.
