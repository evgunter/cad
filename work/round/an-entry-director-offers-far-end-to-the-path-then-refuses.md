---
id: an-entry-director-offers-far-end-to-the-path-then-refuses
kind: issue
title: TipState::Angle is one state for an entry director and a fillet's arrival director, so after an entry Toward the lattice offers FarEndTo, which the path then refuses FarEndAnchorWithoutFillet
status: open
opened: 2026-11-06
priority: P3
cost: M
---


**Found by PR 6549's style review (ROUND, 2026-11-06); carried out of
`a-corner-behind-the-incoming-rays-start-names-no-recourse`, whose unit did
not take it.**

`TipState::Angle` (`crates/profile/src/path/program.rs`) is reached two
ways: by a director at `Entry`, and by a director on a fillet's arrival
side. Its lattice row admits `FarEndTo` (right for the second), so after
`at(p).toward(d)` the lattice refusal's recourse (PR 6549, derived from
the table) offers "At or FarEndTo", and `FarEndTo` there is refused by the
path door as `FarEndAnchorWithoutFillet` (`crates/profile/src/path.rs`).
The recourse is lattice-true and not acceptance-true. Either the two
reachings become two states (so the table says what the path accepts),
or the path's refusal moves into the lattice; which is the unit's to
decide.
