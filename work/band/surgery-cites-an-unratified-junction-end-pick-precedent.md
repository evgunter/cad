---
id: surgery-cites-an-unratified-junction-end-pick-precedent
kind: issue
title: surgery's bounds-lane reads cite a junction-end pick precedent that no ratification records
status: open
opened: 2026-10-25
priority: P4
cost: E
---


Found building unit 2 of `period-headroom-margin-has-no-shared-home`
(branch `pred/period-turn-sites`), which was asked to check the
precedent before touching `sweep/src/blend/surgery.rs`
`split_param_in_span`.

`split_param_in_span`'s doc and its window test cite "the battery's
junction-end pick precedent" for reading a stored window through the
bounds lane rather than a `decide` row.
`git log --all -S'junction-end pick'` finds the phrase first in
`701e1b5ae5` (an M6 WIP commit, in code), then in
`work/band/S90-impl.md`, which classifies `battery.rs`'s
`d0.min(d1).lo() == d0.lo()` read as a selection. No commit in
`docs/DESIGN.md`, a design page or an `[ev]` PR writes it: the
precedent is an agent's classification, not a ruling.

That branch moved the period guard off the bounds read onto the edge
certificate's stored turn. The window test (`inside`) and the doc
paragraph still cite the precedent. Either the precedent is put to Ev
as the ruling it is cited as, or the citation goes and the window test
states its own argument.
