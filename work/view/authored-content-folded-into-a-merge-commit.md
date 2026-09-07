---
id: authored-content-folded-into-a-merge-commit
kind: issue
title: authored lines land inside merge commits, where git log -p does not show them and nothing detects it
status: open
opened: 2026-09-05
---


Found by the #1908 style review, which caught a merge commit of mine
carrying 18 lines present in neither parent. Reported as a CLASS, not
as that instance: nothing in the tree forbids it, nothing detects it,
and this repo's merge-only rule is precisely what makes it invisible.

## What happens

A lane resolving a merge conflict has the whole tree writable and no
gate between it and a commit. Anything it edits while resolving —
prose it was going to write anyway, a fix it notices, a doc comment —
lands INSIDE the merge commit. The result is authored content that:

- `git log -p <branch>` does not show, because it skips merges by
  default; only `git show --cc` or `--first-parent -m` reveals it;
- a PR's file diff shows without attributing to any commit a reader
  can find;
- no reviewer reads, because a merge is read as a resolution and
  skipped.

In the instance that surfaced it (`18a5368da`), the folded lines
included a false claim about what a test file guards — prose that
would not have survived thirty seconds of review and had none.

## Why it is not the merge-only rule's fault

Merge-only is what makes merges frequent, not what makes them
authorable. The rule the tree is missing is the other half: **a merge
commit's content is the resolution and nothing else.** Content authored
during a resolution belongs in a commit before or after it.

## What resolving it looks like

Cheap and mechanical: a check that every merge commit on a branch is
"evil-free" — no hunk in `git show --cc <merge>` outside the conflicted
paths, or more strictly, `git diff <merge> $(git merge-base ...)`
against each parent showing only resolution content. `scripts/` already
hosts tripwires of exactly this shape (`check-ci-mirror-parity.py`,
the gates under `scripts/gates/`), and this one is a `git` call and a
comparison. It wants a decision first on how strict "resolution
content" is: a conflicted file's hand-merged hunks are legitimately in
neither parent, so the check has to scope to the conflicted paths
rather than to the empty set.

Until then the discipline is stateable and unenforced: **resolve, then
commit; author in a separate commit.**
