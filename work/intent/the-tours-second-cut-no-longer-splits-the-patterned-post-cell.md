---
id: the-tours-second-cut-no-longer-splits-the-patterned-post-cell
kind: issue
title: The tour's second cut no longer splits the patterned-post cell: the cut reads the layout's new_space() master, which split refuses as UnresolvedInput
status: open
opened: 2026-10-23
priority: P2
cost: S
---


Found by D′2's fix pass (PR 5613), when the gui render lane ran on its
branch for the first time and its re-baseline refused: the lane no
longer produced the cell `demos/renders-gui/instantiatepart-part2-pattern-place2.png`.

**What.** `demos/tour/src/assembly.rs` `refactorings` cuts a second
cell out of the layout document: the post instance `post_i`, the
pattern and each patterned copy with its placement. Since the layout's
post instance is placed into a `new_space()` master (`layout_doc`'s
`Node::Space`, D′1), the cut reads that `Space` node, which the cut set
does not hold, and `split` refuses
`PartEdit { UnresolvedInput { Space } }`. The tour prints this as a
"second cut (gap)" and carries on, so nothing goes red, and the posts
cell is never written to the workspace. The gui lane photographs the
workspace, so the cell disappears.

**Evidence.** Main's nightly `be9b907694` (render.yml's scene-inputs
job, `120258751743`) prints the same refusal, digest for digest, so the
gap predates D′2. PR 5613 retires the committed cell rather than
keeping a picture no run produces.

**To decide.** Either the tour's cut takes the master space with the
instance placed into it (a `Space` node is a construction with no
reads, so a closed cut can take it), or `split` carries a cut node's
target space across the seam the way it carries the world. The first
is a demo fix; the second is A4's, and is the general answer for any
placement into a `new_space()`.

