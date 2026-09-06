---
id: marks-header-asserts-universals-its-own-module-breaks
kind: issue
title: marks.rs's new header opens with three universals the module itself falsifies
status: open
opened: 2026-09-06
refs: [2083]
---


Found by the style review of #2083. Both new headers are claimed
written from scratch, so this diff caused all three.

## The three claims

**1. `crates/viewer/src/marks.rs:3-4`** — *"Every door here takes a
[`crate::pickindex::PickIndex`] as an ARGUMENT and answers *what should
be lit*."* `cursor_projection` (`marks.rs:432`) takes
`(&[[f32;4];4], [f32;2], [f32;2])`, names no index, and answers a
projection matrix rather than anything lit. The PR's own member table
records it as *"none — a matrix"*, so the universal was known false as
it was written.

**2. `marks.rs:16`** — the section is titled *"# The four marks"* and
its fourth bullet is `cursor_projection`, described in the same bullet
as *"the id pass's 1×1 target matrix, kept out of the render module
because it is the one part of that pass a machine with no GPU can
check"*. A matrix kept somewhere for testability is not a mark; the
section title counts it as one.

**3. `marks.rs:43-45`** — *"`Theme::marks` names the same four
(selected, hovered, probe, focus)"*. `Theme::marks`
(`crates/viewer/src/theme.rs:248`) returns `selected`, `hovered`,
`probe`, `focus`; the four this header names twenty-five lines earlier
are `highlight`, `edge_overlay`, `focus`, `cursor_projection`. Only
`focus` is in both sets. `highlight` produces two of the theme's four,
`edge_overlay` produces three, `cursor_projection` none — so "the same
four" is a coincidence of cardinality presented as a correspondence,
and the sentence tells a reader the two modules can be checked against
each other when nothing lines up.

## The class

A third spelling of the same list sits one file over:
`crates/viewer/src/theme.rs:20` says the theme *"supplies every
semantic mark (selection, hover, probe, focus, unresolved)"* — five,
where `Theme::marks()` returns four. Three enumerations of one concept
in two files, none of them enforcing another. Where else to look: any
module header that says another module "names the same" set —
`crate::datums`, `crate::scene`'s flag bits, `crate::frame`'s writer
census.

## Confidence

`sure` on 1 and 3 (mechanical); `likely` on 2, which is a judgement
about what "mark" means.
