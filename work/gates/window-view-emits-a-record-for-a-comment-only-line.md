---
id: window-view-emits-a-record-for-a-comment-only-line
kind: issue
title: lib.sh's --window view emits a record for a comment-only line, so a hit is reported twice, one line early
status: open
opened: 2026-09-06
refs: [D109, S49]
---


## Finding

Filed by the GATES orchestrator from the `gates/loop-boundary-discards`
lane (PR 2044). `scripts/gates/lib.sh`'s `--window N` mode emits a
record for a line that held only a `//` comment: the comment is
stripped but the whitespace before it survives, so the blank line
starts a window and the same site is reported once from the comment
line and once from its own line. `loop-boundary-discards.sh` filters
the duplicate by confirming each hit against the line view; a consumer
that does not will double-count. The fix is the reader's — a record
whose stripped text is whitespace-only is not a window start — with a
fixture in `gate_selftest_clean` so every caller carries it, and the
gate's confirming filter retires with it. `D109`'s class (the reader
is a lexer that says what it cannot see); `lib.sh` is under PR 2038
until it lands, so this waits behind it.
