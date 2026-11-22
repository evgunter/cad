---
id: the-coincidence-on-face-bound-reads-a-profile-at-the-current-values
kind: issue
title: coincide's on-face bound reads a profile's pieces at the current values, so a profile family answers false and a Zero fit moves it
status: open
opened: 2026-11-18
---


Found by the DOCTAIL value-free-door build (after #7075 ruled
`[recipe.doors]`: a door judges the document, never one evaluation of
it), as one of the readers that read a profile at `doc.var_env`.

`lies_on` (`crates/editor-core/src/coincide.rs`) decides whether a
cell named by a profile vertex lies on the wall an extrude sweeps over a
run, through the `bounds` closure: it replays the extrude's profile
with `ProfileProgram::pieces(&doc.var_env::<f64>(), tol)` and asks
`ProfilePieces::bounds`. That is one evaluation of the profile, outside
any member:

- a profile whose step reads a family index (now admitted at the door)
  does not resolve there (`IndexOutsideMember`), so `bounds` answers
  `false` for every member, and the coincidence the construction
  states is not found;
- a profile whose values refuse at the current point answers `false`
  too, and a run that fits `Zero` there moves which vertices bound
  which wall.

The answer is a construction fact about the names, so it plausibly
belongs to the program's structure: the piece order a program's
structure draws is `profile::chain_shape` (`ProfileProgram::structure`
reads it), from which a piece's end vertex is the next piece's start.
Which one applies (structural adjacency, or per-member pieces) is this
row's question; the 7082 strand-report widening (runs and corners at
generic values) grows the same structural record.

**Done when** the bound reads no single evaluation of the profile, and
a coincidence on a profile family's wall is found in every member.
