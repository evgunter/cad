---
name: git-workflow
description: The GitHub hazards around this repo's merge-only workflow — stacked branches, unprotected main — plus what never goes in a public artifact
metadata:
  type: feedback
---

The workflow itself is in CLAUDE.md (merge-only, never rewritten;
commits are the honest messy record, PR descriptions carry the
documentation; agents self-merge except PRs that ratify open design
questions or change `memories/`). What follows is what that leaves out.

**Push branches early and often** — after each meaningful commit,
before review. Ev follows work in progress remotely.

**Never delete a branch another PR is stacked on** — GitHub auto-closes
the stacked PR, and a PR whose base branch was deleted can never be
reopened. Retarget to main first, or just keep branches (private
remote, cheap).

**main has no branch protection**, so `gh pr merge --auto` merges
IMMEDIATELY. Verify the checks yourself — see [[agent-lane-operations]]
on what merging destroys.

**Account identifiers stay off GitHub (Ev, #355):** no email addresses
or personal identifiers in issues, PRs, comments, commits or committed
files, except Ev's own — the account `evgunter` and the address
`evgunter@gmail.com`, which signs every commit anyway and is the contact
address nightly.yml publishes.
**The rule itself now lives in CLAUDE.md** — this
memory is read as relevance dictates and a publication rule cannot
depend on an agent judging it relevant, so the two are stated together
there and this entry is the pointer, not the source.
