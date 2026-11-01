---
id: the-r4-one-arc-walk-duplicates-r3s
kind: issue
title: ssi/four.rs re-states one_arc.rs's face walk, Krawczyk, end reading and graph margin in R4, and pair.rs's settle_crossing re-states the plane lane's: one parametrised walk owed
status: open
priority: P3
cost: M
opened: 2026-10-11
refs: [a-wall-seam-between-two-fits-has-no-section]
---

Found by SHELL's NURBS × NURBS section unit (`a-wall-seam-between-two-fits-has-no-section`,
PR 4597), at its dual review. The two-chart tube's one-arc proof was
written as a sibling of the ℝ³ one, not a parametrisation of it:

- `ssi/four.rs`'s `face_walk`, `krawczyk` / `krawczyk_with`,
  `mean_value_excludes`, `reaches_end` and `one_arc_pair` re-state
  `ssi/one_arc.rs`'s `face_walk`, `krawczyk`, `r3_reaches_end`, `link`
  and `one_arc_r3` on a `Box4` of three equations in three unknowns,
  where the r3 walk runs on a `Box3` of two in two;
- `four::graph_margin` re-states `enclose::graph_margin` on the 4×4
  `[J̃; ẽ]` with the columns scaled by chart speed;
- `ssi/pair.rs`'s `Section::settle_crossing` (3×3 Newton with the
  face coordinate held, clamped into the Krawczyk box, residual
  re-checked) re-states `ssi/boundary.rs`'s `settle` and
  `ssi/section.rs`'s `settle_root` for the plane lane.

The small shared pieces were folded at the review
(`four::heaviest`, `Box4::of_charts`); these are the large ones.

Owed: one walk generic over the box dimension and the equation count
(the face, its held coordinate, the Krawczyk step, the end reading),
with r3 and the pair lane as its two instances, and one settle at a
held coordinate. No certificate's arithmetic should move; the
`nurbs_nurbs_section` and `ssi_limb3_one_arc` rows are the check.
