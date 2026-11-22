## For Ev

**(A) Lever bullet**, `crates/geom-core/README.md`, "Float rounding", after "no longer than the site's own measure of it:":

> a lever read off a consumed extent is known from both sides, each measured from the site its datum is read at: from below off points of the region and from above off its cover. A verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above, whatever acts on the verdict. Where the two disagree the reading is in band and is refused as one. Where a face is not built yet nothing proves a Zero over it. No consumer takes one bound.

**(B) Consumed-extent bullet**, replacing from "Its lever is the extent's far distance from that point" through "holds its region, not a lever":

> Its lever is the extent's far distance from the site its datum is read at: the line, for a datum that turns about one (a dihedral about a straight edge), else the point the datum is read at; the least over all sites is not owed. A tilt moves a point of the extent by the tilt times its distance from the line it turns about, never more than its distance from a point of that line. So a point is sound for a Zero, provided the datum and its lever are read at the same one, and a definite sign is levered from the line. A record that verdicts read from different sites holds its region, not a lever.

**(C) Under `## Open`**, replacing "Nothing on this page.":

> which site a dihedral along a curved edge is levered from (its station's tangent line or the edge's carrier), and the site and region of the joint turn at a pole; both are read from the span's middle.

**One split for you: when the text merges.**
- **Option 1, on your answer, before the code** (the other designer's lean). This is how the channel has worked so far, as with PR 7194.
- **Option 2, in the same `[ev]` PR, with the line-site code that makes (B) true** (my lean). The PR still states the question by editing the page and is updated in place with your answer; the code then joins it and it merges once.
  - Why: the page is present tense, and PR 7194's sentence sat on main saying something the tree did not do until PR 7258 had to revise it.
  - The line-site run (G1) is already under way, so the code is not far behind.

- **Asked now:** approve or strike (A), (B) and (C), sent once the line-site run (G1, below) has reported which tests' outcomes change. Striking (B) leaves (A) true only of distance from the span's middle, a length no verdict is owed.
- **Not yet decided:** the site on curved edges; the joint turn's site and region; any valid class that the not-built-face case newly refuses.
- **Order in the tree:**
  1. The two-sided rule, the typed not-built face and lazily read covers land with no README change. This moves no verdict where both bounds are finite.
  2. (A), (B) and (C), with the line site for straight edges. Under option 1 the text lands first, on your answer; under option 2 it lands with the code.
  3. Curved edges and the joint turn, once their truths exist.

## For the orchestrator

1. **Last clause of (A): "not built yet".** The other designer is right on the facts: `m-a.md`'s uncovered faces include planes, cylinders and cones that have a surface but no bounding loop yet.
2. **(B): the other report's text, signed word for word.** It keeps "a point is sound for a Zero", which is exactly what Ev's PR 6968 explanation supported, and the records clause from PR 7141.
3. **`## Open` line: yes.** The section already exists, and an open question there is content, not status scaffolding. It stops the middle-of-span gap on curved edges being read as ratified.
4. **When the text merges: still split**, put to Ev above with my lean. I did not concede to close. The PR 7194 to PR 7258 history you cited is evidence against text landing ahead of its code, and the channel rule does not forbid code joining the `[ev]` PR before it merges.
5. **The `[ev]` PR waits for G1: signed.** (B) binds every straight edge; on a curved face along one it rests on a first-order argument. If the extrude table in the open honeone row does not read the profile's declared tangency, the scope shrinks, and it is better that happens before Ev reads the text. It also puts the moved classes in the decision document, as Ev's PR 7140 condition asks.

Still open from earlier rounds:
- the 29,468 edge points past the bound from above;
- door against tier 3;
- the two lane notes disagreeing on step-import's snapped cylinders and the contact-edge corpus, to reconcile before the not-built-face count is read;
- whether the reach was ever the least of the folded arm;
- `Cover::distance_range` from a line, unchecked on anything but planes.
