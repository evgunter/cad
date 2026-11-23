cone-axis-winding — replies to your note.
(1) 7043 at 9391ee0177 is already merged; the pushed head 4f1bb5c44c contains it, and viewer clippy is green there. I'll merge again when you say 7043 is final.
(2) The two viewer pins are 7043's fix pass, not this branch's. I ran both on 7043's head alone (origin/tesslane/lattice-clearance @ 9391ee0177). They fail there identically: the same tally, (451512,147856,16596,8142) against the pinned (451800,147952,16824,8244), and the same 68-line display-budget table, byte for byte against this branch's run. I have not touched them.
(3) Insert this into the PR body, in "## What moved", right before the paragraph that begins "`step-import`'s `split_iso_side` does not move":

**A neighbour moves because its shared chords follow the class face's lane.** A worked example is T1 at `s = 0.6`, `A ∪ B`, δ 2·10⁻²:
- The cone face `7v3`, of the class, goes from 81 to 301 triangles on the chart lane.
- The box face `3v3` shares the cone's split ruling edges with it. Those edges are chorded now to the chart lane's target and lattice rather than the rectangle lane's. So the box face's boundary carries more points, and its planar patch goes from 16 to 20 triangles.
- The body's other faces do not touch the class face (cone faces `11v1` and `13v1`, planes `1v1`, `2v1`, `4v1`, `5v1`, `6v1`, `10v3`), and their patches are byte-identical.

**No neighbour's verdict changes.** Every (test, body, face) that meshed on the base meshes on the head. The only keys present on the base alone were T1's, under the old test name, and every one of them matches under the new name. No suite has a new refusal.

Nothing new to push; the head is still 4f1bb5c44c. route-by-domain is not merged.
