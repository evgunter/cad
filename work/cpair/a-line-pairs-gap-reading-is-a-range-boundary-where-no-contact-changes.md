---
id: a-line-pairs-gap-reading-is-a-range-boundary-where-no-contact-changes
kind: issue
title: profile::seg reads segment_gap before any side, so every line pair whose projections start or stop overlapping is a decision flip that bounds a certified range, though the pair's contacts are none on both sides
status: open
opened: 2026-10-28
---


Found by the GREEN lane `green/slow-set-reds` (2026-10-28) while
re-greening `editor-core`'s `docm9_range` rows.

`profile::seg`'s `line_line` (`crates/profile/src/seg.rs`, `line_line`
and `gaps_apart`) decides **`segment_gap`** on every line pair before
any `chord_side`, which is how it avoids escalating on a near-collinear
pair whose sides are in band. The cost: the gap's sign is a decision
of its own. Over a parameter range, a non-adjacent pair whose
projections go from overlapping to apart along one axis flips it, and
the range walk (`editor_core::range`) stops there as a
`DecisionFlip`, although the pair has no contact on either side and
the built product does not change.

Witness: `docm9_range`'s notch, (0,0),(0.5,h),(1,0),(1,1),(0,1). Its
only real boundary is the crossing at h = 0, but `segment_gap` flips at
h = (1 − √2)/2 (edge (0.5,h)–(1,0) against the left wall) and at h = −1/2
(edge (0,0)–(0.5,h) against the top edge). From the old nominal −0.25,
the walk up stops at offset 0.0429 instead of 0.25. The row
`docm9_range::a_gap_between_two_lines_is_a_decision_flip_below_the_crossing`
pins this. The other notch rows moved their nominal to −0.125 and their seed
to [−0.0625, 0.15], so the crossing is the one boundary inside it.

So every certified range over a profile parameter is now narrower
wherever two lines' projections meet, by a decision that changes no
contact. That is a design question for CPAIR: read the sides first and
the gap only where they do not separate (that brings back the
escalation the gap reading was added to avoid), or give the range walk
a way to tell a flip that changes nothing downstream. Not fixed here.

The ill-conditioned half is fixed (PR on `green/slow-set-reds`):
`geom_core::one_line`'s `apart()` read an overlap by its LENGTH, so a
perpendicular span, which projects to a point, sat at the gap's zero
at any depth inside the other span. It now reads the overlap's depth.
A pair that shares a vertex is no longer read (its gap is zero by
construction).
