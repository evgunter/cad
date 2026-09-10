---
id: module-kinds-table-scan-ends-at-any-column-zero-hash
kind: issue
title: viewer-module-kinds.sh's README table scan ends at any column-zero # and reads a fenced | as a roster row, so a fence part-way down a table SHORTENS the roster and the gate enforces its rule over fewer modules while printing OK
status: closed
opened: 2026-09-10
closed: 2026-09-10
pr: 2287
branch: view/module-kinds
---

Found by the §5 sweep for #2282, which fixed this class in
`scripts/gates/viewer-vocab-declared-once.sh`. **Split from a combined
row** on the test *can half of it be closed?* — this half is
`md_fence` plus a length check; the guard half is apparatus that gate
has never had, a different repair with different controls
(`module-kinds-gate-has-no-reader-guards`).

`scripts/gates/viewer-module-kinds.sh:220-227` (at `09b0ef5a8`; that function is gone, see Closed below):

```sh
readme_table_modules() {
  awk -v want="$1" '
    $0 == want { inside = 1; next }
    inside && /^#/ { inside = 0 }
    inside && /^\|/ { print }
  ' "$README" | sed -nE '…'
}
```

## The mechanism, and it has two sentinels

`:223` ends the table region at any column-zero `#`. Inside a fenced
code block that is content, not a heading. `:224` is the other half:
a worked example of a table ROW written in a fence is read as a roster
row.

## Why the consequence here is WORSE than at the gate that was fixed

The callers red on an **empty** roster (`:271-274`, `:278-282`) —
*"yielded no module rows … a roster that scans nothing is not a pass"*
— and **never on a short one**. So a fence landing part-way down a
table truncates the region there, the rows below it vanish, the roster
is merely SHORT, and the cross-check silently stops covering those
modules while the gate goes on printing OK. At the vocab gate a
truncated section was always a loud red; here it is a quiet narrowing
of what the gate enforces.

## Where the real tree stands: one edit away

Every fence line in `crates/viewer/README.md` is at `:3-262`, in seven
balanced pairs. The scanned regions open at `:290` (`### The drivers`)
and lower, so no fence sits inside any of them today. The drivers table
opens 28 lines below the last fenced block, which is how it becomes
live.

## What a fix owes

- **Reuse `md_fence`**, at
  `scripts/gates/viewer-vocab-declared-once.sh`'s `FENCE_AWK`, rather
  than re-deriving it — including its **three answers**, since a boolean
  is the wrong answer for any *did the previous line end a block*
  predicate (that gap shipped a false GREEN over an unratified fourth
  kind and was caught in #2282's fix pass), and its **mawk constraint**:
  no `(` immediately after an interval. A third copy of this tracker is
  the thing to avoid; where the shared home should be is `lib.sh`'s
  question and `lib.sh` is not VIEW's.
- **A length check, not only an emptiness check**, or the paragraph
  above survives the fence fix: nothing holds the roster's row count
  against the README's table.
- **Negative controls per #2106**, each case run against the unfixed
  reader and recorded red.

## Confidence

`sure` on the mechanism, on the short-roster consequence and on the
latency — all read off the file and the README rather than inferred.


## Closed (2026-09-10)

`scripts/gates/viewer-module-kinds.sh:502-516`'s `readme_table_block`
replaces `readme_table_modules`, and the fence tracker it asks is the
one `viewer-vocab-declared-once.sh` already used, now
`scripts/gates/viewer-readme-fence.awk` — one file both viewer gates
load rather than a copy each. **Both sentinels reproduced first**, on a
copy of the real tree: a `rust` fence carrying `#[derive(Debug)]` four
rows into `### The drivers` left the driver roster at 3 of 10 and red
against the seven modules the table does list below the fence; the same fence inside
`### The session's vocabularies` left the cross-check covering 2 of 6
and printed **OK with a byte-identical line**; and a worked example row
written inside a fence below a vocabulary table red as *"the table
outran the code"* about `session::ghost`.

**The row's central claim is right for one table and wrong for the
other, and the difference is check 3.** The driver roster is held
against the tree in BOTH directions, so a truncated driver table always
red — as a misdiagnosis naming a module the table lists three lines
further down, never as a green. Only the vocabulary tables have the
silent direction: check 4 is one-directional, so their rows simply stop
being cross-checked. The item's *"the gate goes on printing OK"* is
therefore true of two of the three tables it covers.

**The length check is not what closes the silent half.** Nothing here
knows how long the table is SUPPOSED to be, and two counts derived from
one read cannot catch a read that stopped early — a hand-kept count
would be a third hand-kept thing in a gate whose thesis is that the
README is the roster. What is checkable is that every table line in the
section is ACCOUNTED FOR, and what produces that population is the
indent strip below plus three markers: the roster is the first
contiguous run of table lines under the heading (what a renderer draws),
its header and delimiter rows are asserted by position, a line under
them that does not read as a row comes back as `!row` rather than being
dropped, a table line further down the section comes back as `!stray`,
and a line indented out of the table comes back as `!indent`. A fence
opening inside the table body is its own answer, `!fence` (`:513`), and
between them the markers red on both fence shapes — the one with no
blank line above it and the one with, which is the shape a fence tracker
alone leaves silent and the shape the reproduction above used.

`!fence` asks `md_fence` for `open` because that is the true predicate
and NOT because a boolean would fail: at that rule `close` and `inside`
are unreachable, so `fence != ""` decides identically and this gate's
self-test cannot tell them apart — mutated both ways. The tracker's
third answer is load-bearing for `viewer-vocab-declared-once.sh`'s
`opens` and for nothing else, and that gate's self-test is what holds
it.

Fifteen cases in `--selftest`, each run against the base reader with
the same planters spliced in: all fifteen red there, green here. The
table is in the PR and in `work/view/log.md`.

**The fix pass found this row's own defect in the fix.** The first
reader anchored every rule at `^`, and CommonMark puts a table row —
and its header, and its delimiter row — up to THREE spaces in, so a
two-space-indented row was read by no rule: `session::probe` dropped out
of the roster at exit 0 with the output `cmp`-identical to a clean run.
That is `plan.md`'s #2172 rule, one sentinel over. The strip
(`sub(/^ {0,3}/, "", line)`, `:505`) fixes it for the heading rule and
the `^#` region end at the same time, `!indent` (`:509`) reports the
four-space side rather than being quietly right about it, and the OK
line now carries the two README-derived counts that would have made the
original narrowing visible in a log. Four more cases, all four controls
against the pushed head; that table is in `work/view/log.md` too.
