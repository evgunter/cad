---
id: the-plate-sheet-draws-a-certifiable-box-the-tier-outgrew
kind: issue
title: The plate's CERTIFIABLE_FRACTION still reads 7.81e-7 of the study, while tolerance.rs measures the widest whole-certifying box at 0.2631 of it
status: open
opened: 2026-10-03
priority: P3
cost: E
---



Found by SHOW's `the-plate-document-never-cuts-its-holes`.

`demos/tour/src/plate.rs` `CERTIFIABLE_FRACTION` is `7.81e-7` ("`7.81e2 · ε`",
"MEASURED by `crate::tolerance`"), and `demos/tour/src/mcplate.rs`
draws it to scale and prints it on the plate-density sheet ("the
widest box that certifies THIS plate whole is 7.81e-7 of the study");
`demos/README.md`'s `renders-mc/plate-density.svg` section repeats it.
`demos/tour/src/tolerance.rs`'s module header and stop-1 narration say
the widest whole-certifying box is 0.2368 / 0.2631 / 0.2631 of the
study at ε = 1e-6 / 1e-9 / 1e-12 since M10-10's amendment A1, and that
`7.81e2·ε` was the pre-M10-10 figure. Nothing asserts the constant
against a drive, so the sheet's certified sliver is a number the tier
outgrew, and the sheet's "a picture cannot show it" caption no longer
holds at a quarter of the study.

Re-measure (the whole-box ceiling at the default ε, as the header's
numbers were), re-derive the sheet's certified panel and caption from
it, and pin the constant to a drive so it cannot drift again.
