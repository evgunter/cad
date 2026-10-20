---
id: a-split-seams-kfmrh-onto-a-curved-host-is-unexercised
kind: issue
title: zip_split's kfmrh of a split seam's faces onto a curved host face is unexercised
status: open
opened: 2026-10-12
priority: P3
cost: M
refs: [4777]
---


## What

Found by PR 4777's dual review. `crates/topo/src/boolean/zip.rs`
`zip_split` makes every other face of a split seam a ring of its first
A face (`kfmrh_minting`), and `glue` may `kfmrh` again where a joint's
two half-edges lie on different faces. `kfmrh_minting` re-walks the
moved loop in the host's chart (`plan_moved_rows`). Every split seam
any battery reaches is planar: the 36 `dbl_battery` `yx S` lines, in
both roles. No curved line reaches `zip_split`
(`curved_notch_pinch`'s 15 new bodies go through face-pair zips).

The risk is a spurious refusal (a pcurve mint the moved loop cannot
make in the host's chart, typed `Euler`), not a wrong body: the loops
are scaffolding the zip consumes.

## Open

A pose whose split seam lies on a curved face: a pinched operand
against a cylinder or a sphere whose face holds `v`, as `dbl` does
with the cube's face.
