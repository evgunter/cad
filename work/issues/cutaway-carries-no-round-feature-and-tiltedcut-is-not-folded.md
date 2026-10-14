---
id: cutaway-carries-no-round-feature-and-tiltedcut-is-not-folded
kind: issue
title: The tour's cutaway still sections an all-planar box and tiltedcut keeps its own cell, though the split now takes a round feature
status: open
opened: 2026-10-02
---


## What

`split-refuses-cylindrical-feature-box` (PR 3768) found that the tour's
`cutaway` could not carry a round feature because `topo::split`
refused one, and fixed the cause (the split plane's normal is now a
`UnitVec3`). Its rows build both features the issue named and split
them on the cutaway plane (`crates/sweep/tests/split_cylindrical_feature_box.rs`):
the cable-gland bore along `y` and the round standoff along `z`.

The scene itself was left as it was, and PR 3768 says so in its body
("Not done"): `demos/tour/src/cutaway.rs` still sections the
all-planar project box, and `tiltedcut` still keeps its own montage
cell. Folding `tiltedcut` into `cutaway` was the point of the change
the issue was filed from (its "Meanwhile" section), and the module doc
of `demos/tour/src/projectbox.rs` still names round bosses and pilot
holes as what the chain does not attempt.

The issue that recorded this blocks nothing once it closes, so without
this row the work has no home.

## Acceptance

`cutaway` carries at least one round feature crossed by its section
plane; `tiltedcut`'s cell is folded in or kept with a stated reason;
the moved frames are re-baselined and named in the PR
(`docs/prompts/implementer-discipline.md` §3).

## Found by

Review of PR 3768 (`tquery/split-cyl-feature`), 2026-10-02.

## Note (SHOW, 2026-10-02)

`tiltedcut` now carries its own engraving (PR 3819: CUT cut into the
cylinder's cap as three blind pockets before the tilted split), so it
keeps its own cell: folding it into `cutaway` would drop the
engraving. `cutaway`'s round feature is still open.
