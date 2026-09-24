---
id: dimension-mismatch-sentence-is-spelled-in-two-crates-and-held-equal-by-nothing
kind: issue
title: the dimension-mismatch sentence is spelled word for word in two crates while the prose beside it argues the two types are unrelated
status: open
opened: 2026-09-15
priority: P4
cost: E
---


Found by the style review of CENSUS-ERRORS-ARRIVAL (2026-09-15) and
filed by that unit's fix pass, which verified both sites.

## The two spellings

`crates/pncad-py/src/errors.rs` (`impl Display for QuantityOpMismatch`,
by `dimension_tag` on each operand):

    "cannot apply `{}` to {} and {}"

`crates/editor-core/src/expr.rs` (`impl Display for DimensionError`,
arm `Self::Mismatch { op, left, right }`):

    "cannot apply `{op}` to {left} and {right}"

Word for word, two crates, and **nothing holds them equal**. Each is
user-visible: the first through the `DimensionError` Python exception
class, the second through the document layer's own rendering.

## Why this one is sharper than an ordinary duplicate

`errors.rs` argues at length, in the doc comment two lines above the
first spelling, that the two types are NOT the same thing:

> Raised to Python as the `DimensionError` class. That class name is
> the SURFACE spelling and this is the Rust type behind it; the
> document layer's own `DimensionError` is a different type entirely
> (the expression layer's ten-arm refusal), which is why this one does
> not share its name.

**The prose asserts distinctness and the string asserts identity.** A
reader who believes the doc will not look for the second spelling; a
reader who finds the second spelling has no way to tell whether the
match is deliberate parallelism or an accident. Both readings are
live, and which one is right is the question this row asks:

* If the sentence is deliberately one sentence for one concept, it has
  one home and both sites read it from there — and the doc comment
  should say the rendering is shared even though the types are not.
* If the two are independent and may drift, that is a decision, and it
  should be written down where the second spelling is, because today
  nothing distinguishes it from the first case.

## What is NOT claimed here

Not that either message is wrong. Not that the types should merge —
`errors.rs`'s argument that they are different types is correct and is
not in question. The defect is that the relationship between the two
strings is undocumented and unenforced, in both directions.

Territory: `crates/pncad-py/*` is LIB's fence and
`crates/editor-core/src/expr.rs` is claimed by no open program (the
same observation `hand-listed-partialeq-siblings-…` records); this
program's `keep_out` announces its pncad-py rows to LIB.
