---
id: module-kinds-table-scan-ends-at-any-column-zero-hash
kind: issue
title: viewer-module-kinds.sh's README table scan ends at any column-zero # and reads a fenced | as a roster row, so a fence part-way down a table SHORTENS the roster and the gate enforces its rule over fewer modules while printing OK
status: open
opened: 2026-09-10
---

Found by the §5 sweep for #2282, which fixed this class in
`scripts/gates/viewer-vocab-declared-once.sh`. **Split from a combined
row** on the test *can half of it be closed?* — this half is
`md_fence` plus a length check; the guard half is apparatus that gate
has never had, a different repair with different controls
(`module-kinds-gate-has-no-reader-guards`).

`scripts/gates/viewer-module-kinds.sh:220-227`:

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

