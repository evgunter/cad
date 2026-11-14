---
id: an-unfinished-leaf-names-the-copy-that-was-not-turned
kind: issue
title: A union's UnfinishedLeaf names the copy that was not turned: the member carried through the boxed turn is the one read in the first member's frame
status: open
opened: 2026-11-14
priority: P2
cost: E
---


Found by the review of BOXFINISH's PR 6959 (2026-11-14), on that PR's
head with the box-scalar switch on; the fixture is
`crates/editor-core/tests/review_chart_ends_turned_disc.rs` on
`boxfinish/review-chart-ends`, taken into the suite by that PR's fix
pass, where it pins this behaviour and cites this row.

A disc turned about an axis by 30° ± 1°, united with a second copy
standing clear, at `Sym<Interval>`: the union refuses
`UnfinishedLeaf { input: <the clear copy>, findings:
[SurfaceDatumUndecided { kind: Cylinder, datum: Axis, … }] }`. The
union is built in its first member's frame, so the clear copy is the
one carried through the boxed turn, and it is the one whose taken image
does not finish. The person is told that the copy they did not turn
"does not finish where this operation takes it".

Likely a wording or attribution defect in what `UnfinishedLeaf` names,
not a kernel one. The refusal itself is
`check-1-reads-a-turning-frames-length-off-its-bracket`.
