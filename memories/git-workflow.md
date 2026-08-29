---
name: git-workflow
description: The GitHub hazards around this repo's merge-only workflow — issue-closing keywords in PR bodies, stacked branches, unprotected main — plus what never goes in a public artifact
metadata:
  type: feedback
---

The workflow itself is in CLAUDE.md (merge-only, never rewritten;
commits are the honest messy record, PR descriptions carry the
documentation; agents self-merge except PRs that ratify open design
questions). What follows is what that leaves out.

**Push branches early and often** — after each meaningful commit,
before review. Evan follows work in progress remotely.

**A PR body closes an issue by DESCRIBING it.** GitHub scans PR bodies
and commit messages (never the diff) for
`close|closes|closed|fix|fixes|fixed|resolve|resolves|resolved`
immediately followed by an issue reference, with no negation, tense or
subject analysis — so *"a style track does not fix #N"* closes it, and
quoting the hazard fires it. This repo's documents park lanes AT the
issue they wait on, so the collision is structural, not a slip. Scan
every PR body and commit message before publishing, mechanically. The
only safe forms break the token adjacency: drop the `#`, or put a word
between.

**Never delete a branch another PR is stacked on** — GitHub auto-closes
the stacked PR, and a PR whose base branch was deleted can never be
reopened. Retarget to main first, or just keep branches (private
remote, cheap).

**main has no branch protection**, so `gh pr merge --auto` merges
IMMEDIATELY. Verify the checks yourself — see [[agent-lane-operations]]
on what merging destroys.

**Account identifiers stay off GitHub (Evan, #355; the repo may go
public):** no email addresses or personal identifiers beyond the
commit-signing identity `evgunter` in issues, PRs, comments, commits or
committed files. Name accounts by role; concrete addresses live only in
local cad-work logs.
