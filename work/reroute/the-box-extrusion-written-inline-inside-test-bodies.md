---
id: the-box-extrusion-written-inline-inside-test-bodies
kind: issue
title: Outside the named builders, the rectangle extrusion appears in at most 100 more functions, inline in their bodies
status: open
opened: 2026-09-19
priority: P4
cost: E
---

## Finding

- **Where**: at most **100** functions, measured at
  `5b4979ef2` — the merge base of
  `private-extruded-box-builders-outside-the-brick-door`'s PR.
- **Importance**: low-medium — the same shape as the builder class, one
  layer in
- **Confidence**: sure that the sites exist; **100 is a ceiling, not a
  count**, and every member needs reading before it is called one
- **Raised by**: the `dup/private-box-builders` lane, 2026-09-19

`private-extruded-box-builders-outside-the-brick-door` closed the
*named private builder* half of the class — a `fn` returning a `Body`
that builds an axis-aligned box. What it did not touch is the box
written **inline inside a `#[test]` body**: a rectangle loop, a sketch
plane and an `extrude` call in the middle of a row, with no helper to
fold.

## The instrument, and the arithmetic

Every tracked `.rs` file at `5b4979ef2`, no path argument. A `fn` is a
hit when its body contains an extrusion atom (`extrude(`, `Extrusion::`,
`extruded(`) **and**, inside it, any four CONSECUTIVE two-coordinate
expressions forming an axis-aligned rectangle — a sliding window, so
the four may be spelled as a tuple list, as `p2(..)` or
`Point2::new(..)` calls, or as a `.line_to(..)` chain.

    166  functions match, across 105 files
    -66  return a Body — the builder census that unit dispositioned
    ----
    100  remain

## What the instrument could not see

- The window matches four rectangle corners **inside a larger
  polygon**, so a letterform prism (`demos/tour/src/letterforms.rs`,
  `az.rs`) and a notched profile are hits without being boxes. That is
  why 100 is a ceiling.
- In the other direction it misses a box whose four corners are not
  consecutive in the source, and a profile assembled in a loop.

## Why this census is taken at the MERGE BASE and not at head

**The fold blinded this instrument over its own area.** It replaced 32
builders and 47 call sites with bare `brick((8.0, 28.0), …)` calls: a
site the instrument cannot see, because it has neither an extrusion
atom nor a rectangle corner list. A census taken at the fold's head is
therefore structurally blind to every site the fold itself created, and
would report the class as shrinking when what changed was the
instrument's reach.

Measured at head for the record: the same instrument matches **133**
functions, against 166 at the merge base. None of that 33 is a member
that went away.

## Why it sits here

S-DUP's charter — one thing spelled *n* times. The sites are on
S-TCOST's, S-TINT's and S-EDIT's ground and S-DUP claims no territory
(`plan.md`); a program that wants its share claims it by `git mv`.
