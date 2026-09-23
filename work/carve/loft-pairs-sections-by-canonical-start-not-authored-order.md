---
id: loft-pairs-sections-by-canonical-start-not-authored-order
kind: issue
title: loft_geometry pairs each section's canonical segment k, so a section rotated in-plane is twisted by the lex-min rule rather than by its authored correspondence
status: open
opened: 2026-09-23
priority: P0
cost: H
---


## Finding

`loft_geometry` (`crates/sweep/src/skin.rs`) validates each section on
its own (`validate_sections`) and skins canonical segment `j` of every
section into wall `j`. Canonicalization orients an outer loop
counterclockwise and starts it at its lexicographic-minimum vertex
(least x, then least y, in the section's OWN sketch coordinates —
`crates/profile/src/validate.rs`, `lex_min_index`). So the
correspondence between sections is decided by where each section's
lex-min vertex happens to fall, not by the order the author gave.
`loft_geometry`'s doc says the opposite: *"the correspondence is BY
INDEX, and there is no honest way to guess one that was not given"*.
The index that is used is the canonical one, which is a guess the
author did not give.

For a section authored in the same order as section 0 and only
rotated in-plane (a twisted loft), the two disagree once the rotation
moves the lex-min vertex.

## Fixture

Found by EMIT's `loft-anchors-every-section-with-section-zeros-map`,
whose measurement showed that authoring order is erased before the
skin runs. The numbers below are DERIVED from the two rules above and
not yet executed.

- Section 0 at z = 0: the centred square `(-1,-1), (1,-1), (1,1), (-1,1)`.
  It is CCW from its lex-min corner, so canonical = authored.
- Section 1 at z = 1: the same square rotated +30° about its centre,
  authored in the SAME order (the image of each section-0 vertex in
  turn): `(-0.366,-1.366), (1.366,-0.366), (0.366,1.366), (-1.366,0.366)`.
- The author means a +30° twist: section-0 vertex `i` joins its image,
  section-1 vertex `i`.
- Section 1's lex-min vertex is authored vertex 3, `(-1.366, 0.366)`,
  so canonical section 1 starts there. Canonical segment 0 pairs
  section 0's bottom edge `(-1,-1)→(1,-1)` with the image of its LEFT
  edge. The strut from `(-1,-1)` runs to `(-1.366, 0.366)`, which is a
  −60° twist. A square is symmetric under 90°, so both pairings close,
  but they are different solids (different ruled walls, different
  volume).

## What the answer has to settle

Whether a loft's section correspondence is the authored order, a
kernel rule stated as the contract (with the minimal-twist or lex-min
choice documented at `loft_geometry` / `loft_body`), or an explicit
per-section offset the recipe carries. It sits beside
`loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body`
(same root: which vertex is first is load-bearing and not the author's).
A fix that changes the pairing also changes which program segment each
loft wall's published name refers to. The per-section
translation (`SectionAnchors`, `ProfileProgram::profile_edges_of`)
reads the canonical correspondence, so it follows whatever the skin
pairs.
