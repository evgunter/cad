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
conversations per docs/M0-PLAN.md — wait for Evan's sign-off before
merging; routine implementation self-merges. See [[cad-working-style]].
