---
id: the-thread-count-digests-re-cut-log-is-a-changelog
kind: issue
title: The thread-count digest's re-cut log is a changelog in a doc comment
status: closed
opened: 2026-10-12
priority: P4
cost: E
branch: nurbs/digest-log-trim
closed: 2026-10-12
pr: 4889
---


Found by the STYLE review of PR 4887.

`expected()`'s doc comment in
`crates/sweep/tests/mass_props_are_thread_count_invariant.rs` carries 29
`**Re-cut at all three ε when …**` entries after its licence paragraph
("When a lane may re-cut here instead, and what licenses it"). Each
records one past move, its numbers and its PR: history, which the
implementer discipline (§4, "comments state the invariant, not the
history") puts in PR bodies and `git log`, not in the comment that
licenses the next re-cut.

The cost is real: every re-cut appends another paragraph, and a lane
reading the licence has to scroll past the changelog to find what the
golden means today.

Fix: trim the comment to what the current golden is and what licenses a
re-cut, plus one pointer, `git log -p --
crates/sweep/tests/thread-count-digest/`, for the history. Each PR body
already carries its own re-cut's numbers and cause.

## Closed (2026-10-12)

`expected()`'s doc now says what a golden holds (both blocks' columns),
keeps the licence paragraph unchanged, and points at `git log -p --
crates/sweep/tests/thread-count-digest/` for the history. The 29 dated
entries are gone. Each one is still in its PR body and in the eps
files' history. No golden moved.

