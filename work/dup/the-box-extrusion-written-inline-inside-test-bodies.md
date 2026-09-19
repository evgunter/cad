---
id: the-box-extrusion-written-inline-inside-test-bodies
kind: issue
title: The rectangle-profile extrusion is written inline inside test bodies at least 80 more times
status: open
opened: 2026-09-19
---

## Finding

- **Where**: 100 sites in 59 files, measured at head after the
  builder-level fold landed (below).
- **Importance**: low-medium — the same shape as the builder class, one
  layer in
- **Confidence**: sure that the sites exist; the 100 is an **upper
  bound**, for the reason under *What the instrument could not see*
- **Raised by**: the `dup/private-box-builders` lane, 2026-09-19

`private-extruded-box-builders-outside-the-brick-door` closed the
*named private builder* half of the class — a `fn` per suite that
returns a box. What it did not touch is the box written **inline
inside a `#[test]` body**: a rectangle loop, a sketch plane and an
`extrude` call, in the middle of a row, with no helper to fold.

**The instrument.** Every tracked `.rs` file, no path argument; each
`fn` body that contains an extrusion atom (`extrude(`, `Extrusion::`,
`extruded(`); inside it, any four CONSECUTIVE two-coordinate
expressions that form an axis-aligned rectangle (a sliding window, so
the four may be spelled as a tuple list, as `p2(..)` calls, as
`Point2::new(..)` or as a `.line_to(..)` chain). 133 functions match at
head; 34 are the Body-returning builders the census above dispositioned;
**100 remain**, across 59 files. `crates/sweep/tests/verbs_shell_r2_probes.rs`
(7), `review_m2_pr4.rs` (6) and
`crates/editor-core/tests/m10_5_r1_probes_interval.rs` (6) are the
heaviest.

## What the instrument could not see

The window matches four consecutive rectangle corners **inside a larger
polygon**, so a letterform prism (`demos/tour/src/letterforms.rs`,
`az.rs`) and a notched profile count as hits without being boxes. 100
is therefore an upper bound and every member needs reading before it is
called one. In the other direction it still misses a box whose four
corners are not consecutive in the source, and a profile assembled by a
loop.

## Why it sits here

S-DUP's charter — one thing spelled *n* times. The sites are on
S-TCOST's, S-TINT's and S-EDIT's ground and S-DUP claims no territory
(`plan.md`); a program that wants its share claims it by `git mv`.
