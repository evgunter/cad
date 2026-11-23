---
id: a-plane-cone-chord-reads-its-pair-beside-the-held-reading
kind: issue
title: The chord minted between two plane×cone germs reads its pair through wall_section at the face's reach, beside the glue door's one reading, with no check that they agree
status: open
opened: 2026-11-02
priority: P2
cost: M
---

Filed by SPLITDOOR on `splitdoor/section-line-components` (PR 6310),
from its second review.

## What

PR 6310 reads each plane×cone pair once per operation, at the glue
door, at the cone surface's reach (`glue::section_door`,
`SectionTable::plane_cone`). The germ mint (`join::germ_branch` →
`apex_line`) and the join's frame read (`join::germ_loops`) take that
reading, so they cannot disagree on the section's shape or its lines'
order. The pierce germ at the apex reads its ruling off the sector's
own plane (`vtxfac::pierce_apex_germ` → `chord_join::apex_section_ruling`
→ `wall_section`). A branch that names no line there is now refused as
a desync (`vtxfac::apex_germ_on_a_line`, row
`cone_frame_rows::the_mint_takes_the_pairs_held_reading`).

One reader is still outside the held reading: the chord the join mints
between two matched germs. `join.rs` `split_curve` (`JoinLane::Split`
→ `chord_join::chord_spec` → `wall_section` at `u1`, the face's reach
through `section_reach`) and `bool_planar_curve`
(`JoinLane::BoolPlanar` → `bool_planar_chord_spec`) read the pair
again. Near the band edge of the `pn_apex_*` or `pn_axis_normal`
decisions, the chord could read a conic where the held reading
matched the germs as straight, or a line pair where it matched them
on a loop. Nothing checks that the two agree.

`wall_section` is lane-neutral: the split lane, which has no section
table, reads it too. So folding it into the held reading is not local
to the Boolean.

## Done when

The Boolean's chord readers of a plane×cone pair take the glue door's
reading, or refuse typed where their own reading disagrees with it,
with a row that reaches the disagreement.
