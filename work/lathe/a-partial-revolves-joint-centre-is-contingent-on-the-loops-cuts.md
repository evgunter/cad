---
id: a-partial-revolves-joint-centre-is-contingent-on-the-loops-cuts
kind: issue
title: A partial revolve's reported joint centre is the unweighted mean of its section's cut points and apexes, so it moves with where the loop is cut
status: open
opened: 2026-11-15
priority: P3
cost: E
---

Found by BOXFINISH's `boxfinish/sweep-frames` (PR 6972), whose review
read it. `sweep::revolved_caps` reports each wedge cap's frame with the
outer section's centre carried to that station as its origin
(`crates/sweep/src/revolve/partial.rs`, `section_centre`). For a
one-segment circle that is the circle's centre. For anything else it is
the unweighted mean of the collapsed runs' start vertices and arc
apexes: the value the cap's Newell-fitted origin used to read, kept so
the reported joint did not move when the caps stopped being fitted.

That mean depends on where the loop is cut. A circle authored as two
unequal arcs reports the mean of two cut points and two apexes, not its
centre (by inspection; not built), and a polygon with one edge split in
two moves it. The only consumer today is the tour's lily, on a
one-segment circle, so nothing shipped reads the contingent case.

What closes it: either the section's own centre where it has one (a
circle in any number of arcs, by its carrier), and a typed refusal or a
stated intrinsic point (the section's area centroid, from the profile's
own segments) otherwise. The choice is the program's; a row that builds
the two-arc circle and reads its joint pins whichever it is.

