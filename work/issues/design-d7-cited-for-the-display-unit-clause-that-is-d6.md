---
id: design-d7-cited-for-the-display-unit-clause-that-is-d6
kind: issue
title: ten live sites cite D7 for the display-unit rule that DESIGN.md puts in D6
status: open
opened: 2026-09-16
priority: P4
cost: E
---


Found by the style review of PR #2779 (lane `notation-rv`) at
`1c7b7340d`. That PR's body is where the miscitation is FIRST caught:
it corrects the unit spec's "D7 makes a unit presentation metadata
outside `bit_eq`" to D6, quoting `docs/DESIGN.md` D6 ¶2 ("**A stored
literal always names its notation.** … presentation metadata excluded
from expression identity, keys and evaluation"). `docs/DESIGN.md` D7 is
"Import is adoption, not admission" and says nothing about notation.

The PR corrected it in the two places it wrote — `RecordedNotation`'s
rustdoc and the PR body — and left every sibling. Filed as a class, not
as an instance: the sweep obligation is the point.

## The live sites, all making the same claim

Code:

- `crates/editor-core/src/expr.rs:493` — "the unit is presentation
  metadata under D7's hard rules"
- `crates/editor-core/src/expr.rs:559`, `:567` — `bit_eq`'s exclusion
- `crates/editor-core/src/expr.rs:707` — `literal_with_unit`'s rustdoc,
  which is the door PR #2779 rides
- `crates/editor-core/src/param_source.rs:894`
- `crates/editor-core/src/eval/mod.rs:3902`
- `crates/viewer/src/props.rs:769`, `crates/viewer/src/session/op.rs:101`

Tracker:

- `work/edit/recorded-program-arguments-carry-no-notation.md:36`, `:51`,
  `:55` — the item's own finding and Spec, uncorrected
- `work/lib/path-legs-erase-the-authored-notation-one-layer-down.md:53`
  — the parked row that unparks on it
- `work/edit/plan.md:55`, `work/edit/log.md:357`
- `work/census/hand-listed-partialeq-siblings-outside-the-census-debug-fence.md:47`
  — an independent row, outside EDIT, citing D7 for the same fact

## The residue a fix has to decide, not just rename

A second family of bare `D7`s means **replay identity / bit-exactness**
— `crates/editor-core/src/expr.rs:306`, `:506`,
`crates/editor-core/src/program.rs:431`, `:437`, `:1658`,
`crates/editor-core/src/distribution.rs:245`,
`crates/pncad-py/src/py/doc.rs:2986` — which is not D6 either, and
`docs/DESIGN.md` has no D7 clause about it (D9 is the determinism
charter). A third family is a MILESTONE spec's own clause and is
correct as written, spelled "spec D7" / "M4 PR 6 spec D7"
(`crates/editor-core/src/appearance.rs:148`, `src/diff.rs:1`,
`src/expr.rs:1038`). A fourth
is genuinely D7-the-import-clause (`crates/geom-brep/src/props/loop_area.rs:25`).
So the class is really *bare `D7` is unqualified and means four
different things*; only the first family is provably wrong.

Related precedent: `crates/editor-core/src/doc.rs:1` already
disambiguates by writing "spec D2, DESIGN.md D8", which is the spelling
a fix could adopt everywhere.
