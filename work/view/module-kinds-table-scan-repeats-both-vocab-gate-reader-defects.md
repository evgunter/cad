---
id: module-kinds-table-scan-repeats-both-vocab-gate-reader-defects
kind: issue
title: viewer-module-kinds.sh's README table scan has both defects the vocab gate just fixed: a bare ^# section end that silently SHORTENS a roster, and two unguarded reader stages whose death reds naming the README
status: open
opened: 2026-09-10
---

Found by the §5 sweep for #2282, which fixed both of these classes in
`scripts/gates/viewer-vocab-declared-once.sh`. The sweep was over the
SHAPE rather than the symbol — every gate that reads a markdown section,
and every `|| status=$?` / `|| reader_failed` sitting on a multi-stage
pipeline — and `viewer-module-kinds.sh` is the one hit outside the file
that unit owned. **Not that unit's to fix**: the brief fenced it to one
gate, and a repair here wants its own negative controls.

Both defects live in ONE function,
`scripts/gates/viewer-module-kinds.sh:220-227`:

```sh
readme_table_modules() {
  awk -v want="$1" '
    $0 == want { inside = 1; next }
    inside && /^#/ { inside = 0 }
    inside && /^\|/ { print }
  ' "$README" |
    sed -nE 's/^\|[[:space:]]*`([A-Za-z0-9_:]+)`[[:space:]]*\|.*/\1/p'
}
```

## Defect 1: the bare `^#`, and here it SHORTENS a roster rather than emptying it

`:223` ends the table region at any column-zero `#`, on the assumption
that such a line is a heading. Inside a fenced code block it is not —
`#[derive(Debug)]`, `#!/bin/sh`, `# a comment` are content. This is
`gate-section-scans-end-on-any-column-zero-hash` exactly, and
`:224`'s `inside && /^\|/` carries the second half of it too: a worked
example of a table ROW written in a fence is read as a roster row.

**The direction that matters is different here, and it is worse.** In
the vocab gate a truncated section produced a loud red. Here the callers
(`:271-274`, `:278-282`) red only on **zero** rows — *"yielded no module
rows … a roster that scans nothing is not a pass"*. They do not check
the roster's LENGTH. So a fence landing part-way down a table
truncates the region there, the rows below it vanish, the roster is
merely SHORT, and the cross-check silently stops covering those
modules. A gate quietly enforcing its rule over fewer modules than the
README lists is the `#1953`/`#2106` shape again: a gate that still
prints OK while deciding less than it claims.

## Defect 2: two unguarded reader stages, and the red names the README

`awk … | sed …` is two stages, both reading `$README`, both inside a
process substitution (`mapfile … < <(readme_table_modules …)` at
`:270` and `:277`) — so both statuses are discarded. **This gate has no
reader guard at all**: `grep -nE 'status=\$\?|reader_failed|gate_reader_died|abort_if'`
over the file returns nothing. That is
`gate-reader-guards-count-six-where-the-stated-rule-yields-nine`'s
class, at a gate that never had the apparatus rather than one that had
it on the wrong granularity.

**The mitigation is real and is why this is not a false green**, and it
should be said plainly: the empty-roster checks above mean a dead
reader DOES red. What it reds with is the defect — *"Either the heading
was renamed or the table was reshaped"* — about a README that is
perfectly fine, sending its reader to edit the one thing that is not
wrong. Misdiagnosis, not silence.

## Where the real tree stands: latent, both halves

Every fence line in `crates/viewer/README.md` is at `:3-262`. The
scanned regions open at `:290` (`### The drivers`) and lower
(`### The session's vocabularies`, `### The app's vocabularies`), so no
fence sits inside any of them today. The drivers table opens 28 lines
below the last fenced block, which is how it becomes live: a fenced
example added to the prose that introduces the drivers is the natural
next edit.

## What a fix owes

- `FENCE_AWK`/`md_fenced` already exists at
  `scripts/gates/viewer-vocab-declared-once.sh:614-637` and is the
  helper to reuse rather than re-derive — **including its
  `mawk` constraint**: no `(` immediately after an interval, argued at
  `:567-587`, because the natural spelling of the fence pattern aborts
  `mawk` 1.3.4's regex compiler while `gawk` accepts it. A third copy
  of this tracker is the thing to avoid; where the shared home should
  be is `lib.sh`'s question and `lib.sh` is not VIEW's.
- **A length check, not only an emptiness check**, or defect 1's real
  consequence survives the fence fix: nothing holds the roster's row
  count against the README's table.
- Per-stage guards on the `awk` and the `sed`, each named for its own
  stage.
- **Negative controls per #2106**: each new case run against the
  unfixed reader and recorded red. Note what the vocab gate learned
  about this — a case can assert a reader's NAME (`gate_selftest_case`
  matches a substring, `--also` requires several) but cannot assert a
  string is ABSENT, so a fix whose effect is to REMOVE a wrong name has
  no expressible control
  (`work/issues/gate-selftest-cannot-observe-the-identity-a-gate-names`).

## Confidence

`sure` on both mechanisms, on the no-guard claim and on the
latency — all read off the file and the README rather than inferred.
`sure` that the empty-roster checks mitigate defect 2 into a
misdiagnosis rather than a green. `likely` that the silent-shortening
half of defect 1 is the most valuable thing here, since it is the one
direction neither gate's existing guards can see.
