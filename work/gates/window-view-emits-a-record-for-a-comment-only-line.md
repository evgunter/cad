---
id: window-view-emits-a-record-for-a-comment-only-line
kind: issue
title: lib.sh's --window view emits a record for a comment-only line, so a hit is reported twice, one line early
status: review
opened: 2026-09-06
refs: [D109, S49]
branch: gates/reader-homes
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

## Landed (2026-09-06)

On `gates/reader-homes`, with
`test-module-resolution-has-three-homes`.

**The fix is one guard, at the reader.** `lib.sh`'s emit guard was
`if (out == "") next`; a comment-only line strips to its own
indentation, which is not `""`. It is now `if (out ~ /^[ \t]*$/) next`,
so a line that carries no code is not a record in ANY of the three
shapes — which is what a blank line already got. The window view
therefore starts only at code, and joins the next N-1 CODE lines rather
than spending a slot on a comment; the reader's header said "the next
N-1 code lines" already, so the shape and its description agree now.

**Its fixture is in `gate_selftest_clean`**, so every gate carries it,
and it is an assertion about the READER rather than about the gate
around it — the defect is the reader's and every gate reads through it.
It pins the whole `--window 2` view of a four-line file whose second
line holds only a comment, so it fails in both directions: a record at
the comment's line, and a join that stops there. Mutation-proved by
restoring `out == ""`, on a gate that reads no window at all
(`no-ambient-env.sh`), which is the point of siting it in the shared
harness:

```
SELFTEST FAILED: the --window view over a file whose second line holds only a comment ...
win.rs:1: fn f() {
win.rs:2: let x = 1;        <- the record that holds none of the code it carries
win.rs:3: let x = 1; }
win.rs:4: }
```

**What moved in the views** over `crates/*/src` (433 files), before and
after, under both `mawk` and `gawk`: the line view loses 68,583
whitespace-only records and gains none (269,422 → 200,839); the
literal-keeping line view the same; `--skip-cfg-test` loses 57,967. The
two statement views are byte-identical. The window views change by
construction — at window 16, 174,506 records out and 113,610 in, the
difference being both the duplicates that leave and the joins that now
reach one code line further. **No gate's output moves**: every gate in
`scripts/gates/`, self-test and real pass, prints byte-identically
before and after, under both awks.

**The gate that filtered the duplicate.** `loop-boundary-discards.sh`
is PR 2044 and not on `main`, so its confirming filter could not be
retired in this diff. Measured out of tree at that PR's head: the fixed
reader leaves its 80 sites unmoved with the filter present AND with it
deleted, while the old reader plus a deleted filter reds with the
duplicates — so the filter did exactly this and nothing else. The
retirement is
`loop-boundary-line-view-filter-is-dead-after-the-window-fix`.
