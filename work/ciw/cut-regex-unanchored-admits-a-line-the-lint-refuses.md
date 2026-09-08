---
id: cut-regex-unanchored-admits-a-line-the-lint-refuses
kind: issue
title: The cut script's CUT_RE is anchored only at the start, so it admits a stamp line tess-lint refuses
status: open
opened: 2026-09-08
---


## Finding

`scripts/tess_budget_cut.sh:60`'s validator is

```
CUT_RE='^# tess-budget-cut: [0-9a-f]{7,40}(-dirty)? [0-9]{4}-[0-9]{2}-[0-9]{2}'
```

anchored at the start and **not at the end**, so it matches any line
that begins with a well-formed cut record — trailing junk included.
`tools/tess-lint`'s reader does not: `split_cut` splits the text after
the prefix on single spaces and takes exactly two fields, so
`# tess-budget-cut: 1a2b3c4 2026-08-30 extra` is refused as harness
breakage.

The disagreement is one-way and it is the arm that hurts, because the
script's `head -1 … | grep -Eq "$CUT_RE"` is its ALREADY STAMPED test:

- arm 1 (`:71`) reads such a line as a valid stamp and **refuses to
  re-stamp**;
- arm 4 of the selftest (`:187`) is the repair path for a stamp the
  lint cannot read — and it is unreachable for this class, because the
  refusal fires first.

So a file carrying a trailing-junk cut line leaves the gate
permanently unreadable with no in-script recourse: `tess-lint` exits
`EXIT_HARNESS` on every run and `tess_budget_cut.sh` declines to fix
it. Every other malformed shape is repaired, which is what makes this
one a hole rather than a policy.

## Fix

Anchor the regex — `…[0-9]{2}$`, or `…[0-9]{2}[^ ]*$` if a longer ISO
date must still pass (it must: the committed baseline's date is
`2026-09-04T03:16:35-07:00`). One line, in `scripts/`.

**Why it is not closed by METER unit 4.** `scripts/tess_budget_cut.sh`
is CIW's (`work/ciw/program.md`'s `paths`), and that lane's fence was
read-only there. Nothing on `tools/`'s side can close it: loosening
`tess-lint` to ignore a third field would admit junk, which is the
opposite of what the reader is for.

## Pinned meanwhile

`tools/tess-lint/tests/cut_line_pin.rs` holds the two halves to each
other over a truth table, and this line is the one row where they
disagree — `TRAILING_TEXT`, expected `(reads = false, recognises =
true)`. **The row is a pin on the defect, not on the fix**: anchoring
the regex REDS that row. That is not a regression and the row says so
in the alarm itself — its `case` string is `TRAILING_CASE`, which
names this item and the two edits that close it (set `recognises` to
`false`, delete `TRAILING_CASE`). The failure surfaces in
`tools/tess-lint`'s suite, a cargo root outside the workspace that a
`scripts/`-only change does not run locally; CI runs it.

## Was

Found by the METER unit-4 lane (`meter/cut-prefix-pin`, 2026-09-08)
while building that pin: the truth table is what made the asymmetry
visible, since it is the only place the two languages' answers are
written side by side.

Filed on CIW's slate rather than METER's because the fix is one line in
`scripts/tess_budget_cut.sh`, which is CIW's territory, and
`work/README.md` is explicit that an item goes straight onto the owning
program's slate when the owner is clear. It was minted under
`work/meter/` and moved here in the same PR (`git mv`, id unchanged);
left there it would have depended on METER's pre-close re-homing sweep
and died with that directory.
