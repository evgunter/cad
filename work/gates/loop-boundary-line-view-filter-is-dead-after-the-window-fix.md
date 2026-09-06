---
id: loop-boundary-line-view-filter-is-dead-after-the-window-fix
kind: issue
title: loop-boundary-discards.sh's line-view confirming filter is dead once the window view stops emitting a record for a comment-only line
status: open
opened: 2026-09-06
refs: [window-view-emits-a-record-for-a-comment-only-line, S49, 2044]
---


## Finding

`scripts/gates/loop-boundary-discards.sh` (PR 2044, open at
`28a8dcc34`) confirms every window hit against the code-only LINE view
before counting it — `if (!((file ":" line) in anch)) next`, where
`anch` holds the lines whose own text starts with the enum pattern.
That filter exists for one reason: `lib.sh`'s `--window` view emitted a
record for a line that held only a `//` comment, so every discard
sitting under a comment was reported twice, once at the comment's line.

`window-view-emits-a-record-for-a-comment-only-line` fixes that at the
reader, and the filter then has nothing to do. Measured on the live
tree at that PR's head, four cells:

| reader | filter | result |
| --- | --- | --- |
| `main`'s | present | OK, 80 sites |
| fixed | present | OK, 80 sites |
| fixed | deleted | OK, 80 sites |
| `main`'s | deleted | RED — `UNREG … splitting/join.rs:318`, the duplicate of the site at 319, and its siblings |

The pinned counts are what make that a proof: the filter's removal moves
no count once the reader is fixed, and moves them the moment it is not.

**Why it is a separate row.** The reader fix landed on
`gates/reader-homes` while PR 2044 was still open, so
`loop-boundary-discards.sh` is not on `main` and the retirement could
not ride the PR that made it dead. Whoever lands next takes it: delete
the `anch` test and the `at[]`/`anch[]` bookkeeping that only it uses —
`at[]` also carries the enclosing-`fn` name, which the report needs, so
only the test and the `anch` array go — and re-run the gate to see 80
unmoved. The gate's own mutation table has a row for this filter (*"the
line-view anchor filter deleted → the clean fixture fails"*); that row
goes with it, since the fixture's comment-only lines no longer double
anything.
