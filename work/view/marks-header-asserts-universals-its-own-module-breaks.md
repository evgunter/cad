---
id: marks-header-asserts-universals-its-own-module-breaks
kind: issue
title: marks.rs's new header opens with three universals the module itself falsifies
status: closed
opened: 2026-09-06
refs: [2083]
closed: 2026-09-06
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

## Closed — the header is rewritten (2026-09-06, #2083's fix pass)

All three are gone, and the citations above are left as filed: they
name the header this item found, which no longer exists.

- **1.** The universal is deleted. The opening section is now *"# The
  three marks"*, and its lead sentence scopes itself to those three —
  *"Each is a pure function of a built `PickIndex` and what is
  selected"* — rather than quantifying over every door in the file.
- **2.** `cursor_projection` is no longer counted as a mark. It has its
  own section, *"# `cursor_projection` is not a mark, and is here for
  want of a home"* (`marks.rs:31`), which says it takes no index, no
  selection and no document, that nothing is lit by it, and that the
  reason it sits here is TESTABILITY rather than subject. That section
  points at `cursor-projection-landed-in-marks-for-want-of-a-home`,
  which holds the question of whether `camera` is its home; the move
  is not made here, because it would be a third move in a PR whose
  warrant is two clean ones.
- **3.** *"the same four"* is deleted. The passage now says the two
  modules **enumerate different lists and neither checks the other**,
  names the theme's four as SEMANTIC marks, says only `focus` appears
  in both, and gives the reason they cannot be lined up: one door
  feeds several of the theme's marks, so the correspondence is
  many-to-many. The per-door tally that an earlier draft of this fix
  carried was removed as well — it was an inference over `gpu.rs`'s
  uniform block rather than something either module states, and this
  item is about headers asserting more than they can support.

The class the item names — a third spelling at `theme.rs:20` (five
where `Theme::marks` returns four) — is NOT fixed here: it is
`theme.rs`'s sentence, not this unit's, and no header written by this
unit now claims a correspondence with it.
