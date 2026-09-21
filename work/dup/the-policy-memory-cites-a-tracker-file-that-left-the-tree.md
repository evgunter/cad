---
id: the-policy-memory-cites-a-tracker-file-that-left-the-tree
kind: issue
title: review-and-dependency-policy cites work/issues/reviewer-pair-rebuilds-two-trees-two-rules.md, which was deleted on 2026-09-11
status: open
opened: 2026-09-19
needs_ev: true
priority: P4
cost: E
---


## Finding

- **Where**: `memories/review-and-dependency-policy.md`, the last
  sentence of the *"Reviewer tests are ordinary tests"* clause:

  > … that reading was withdrawn when two test-support trees were found
  > stating opposite rules for the same class of duplicate
  > (`work/issues/reviewer-pair-rebuilds-two-trees-two-rules.md`).

- **What is wrong**: that path is not in the tree.
  `git ls-files | grep reviewer-pair` returns nothing. The file existed
  — `kind: issue`, `opened: 2026-09-03`, `closed: 2026-09-04`, raised by
  TCOST-10's style review — and was deleted by `499122b10`, *"work:
  code-quality leaves the tracker (DOC-LEDGER sweep 11)"*. Its contents
  are readable at `81b5a3cfb`.
- **Importance**: medium. The citation is the memory's only account of
  **why** Ev withdrew the protected-class reading, so a lane checking
  that account follows the pointer and finds nothing. The tracker's own
  rule is that a deleted directory's record of survival is
  `docs/DOC-LEDGER.md` with the SHA it is recoverable at
  (`work/README.md`), and this citation predates the deletion and was
  not updated with it.
- **Confidence**: sure. The path, the deletion commit and the recovery
  SHA are all measured at `5b4979ef2`.
- **Raised by**: the S-DUP lane for the citation census, 2026-09-19,
  by resolving every citation in the memory rather than every copy of
  its sentences.

## Why this is filed and not fixed

`memories/` is Ev's call under `CLAUDE.md` — that text is read at the
start of every session, so what goes in it is his. A lane does not
edit it to repair a pointer, even a dead one. `needs_ev: true`.

## What would settle it

Ev deciding whether the clause should name the SHA the issue is
recoverable at (`81b5a3cfb`), name the DOC-LEDGER sweep that removed
it, or drop the citation and keep the account in prose. Any of the
three is a one-line edit; which one it is, is his.

## The general shape

The memory's own instruction two paragraphs above is *"When you
retract one, grep for the claim, not the sentence"*. This is the
mirror case: the claim survived and its **evidence** moved. A census
over citations finds it; a census over sentences cannot, because
nothing about the sentence changed.
