---
id: window-view-emits-a-record-for-a-comment-only-line
kind: issue
title: lib.sh's --window view emits a record for a comment-only line, so a hit is reported twice, one line early
status: review
opened: 2026-09-06
refs: [D109, S49]
branch: gates/reader-homes
pr: 2058
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

**The gate that filtered the duplicate is retired with it.** PR 2044
merged while this branch was open, so `loop-boundary-discards.sh`'s
line-view confirming filter — `if (!((file ":" line) in anch)) next` —
comes out here. Four cells over the live tree say it was doing this and
nothing else: old reader + filter, 80 sites green; fixed reader +
filter, 80 green; fixed reader, filter deleted, 80 green; old reader,
filter deleted, RED with the duplicates (`UNREG … splitting/join.rs:318`,
the site at 319 read one line early). The pinned counts are what make
that a proof.

The anchor test stays, because what it now does is name the enclosing
`fn` at each site line; the `anch` array and the confirming test go.
A window record whose start line the line view does not place is no
longer dropped — it matches no entry and is REPORTED, which is the
direction a gate should err in.

**The gate's clean fixture is the reader fix's second witness.** Every
planted site there sits under a comment-only line, so with the filter
gone a regressed reader makes every entry MISCOUNT: proved by reverting
the reader guard and removing `gate_selftest_clean`'s own window
assertion, which reds the fixture with one duplicate per site, each
carrying an empty `<fn>`.
