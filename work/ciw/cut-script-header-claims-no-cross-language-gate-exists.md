---
id: cut-script-header-claims-no-cross-language-gate-exists
kind: issue
title: The cut script's header says its format is pinned by nothing; tools/tess-lint now pins it in two clauses
status: open
opened: 2026-09-08
---


## Finding

`scripts/tess_budget_cut.sh:54-59`, above `CUT_RE`:

```
# What counts as ALREADY STAMPED. This is the shell's reading of the
# format `tess_lint::split_cut` parses, and the two are pinned by
# nothing: there is no cross-language gate here. A drift shows up as
# this script declining to refuse, never as a wrong cut. `--selftest`
# covers this side and `tools/tess-lint`'s suite covers the other;
# neither reads the other's spelling.
```

Two of its clauses are now false, as of `tools/tess-lint/tests/cut_line_pin.rs`
(PR 2151):

- **"there is no cross-language gate here"** — there is. That file
  `include_str!`s this script, holds its four executable spellings of
  the prefix to `tess_lint::CUT_PREFIX`, extracts `CUT_RE` from the
  text and runs it under `grep -E` as the oracle for this half.
- **"neither reads the other's spelling"** — `tools/tess-lint`'s suite
  reads this one's.

The third clause still holds and should survive the edit: a drift
that merely LOOSENS `CUT_RE` still shows up as this script declining
to refuse a re-stamp rather than as a wrong cut, and the pin does not
red on it by design.

## Fix

Rewrite those two clauses in `scripts/tess_budget_cut.sh:54-59` to say
what is true: the format is read by `tess_lint::split_cut` and the two
readings are held to each other by `tools/tess-lint/tests/cut_line_pin.rs`,
which is a cargo root OUTSIDE the workspace — so an edit to `CUT_RE`,
to the prefix, or to the `echo`'s field list reds there and not in a
`cargo test` at the repo root. CI runs it.

## Why not closed by the lane that falsified it

`scripts/` is CIW's (`work/ciw/program.md`'s `paths`) and the METER
unit-4 lane's fence was read-only there. The lane quoted the sentence
in its own module header as motivation, which made a second copy of a
false sentence in a file the fix would not touch; that quotation has
since been dropped, so this item is the only place the correction is
owed.

## Was

Found by the METER unit-4 lane (`meter/cut-prefix-pin`, 2026-09-08),
which is what made it false. Filed here rather than on METER's slate
because the fix is an edit to a CIW path, and METER's directory is
deleted at close.
