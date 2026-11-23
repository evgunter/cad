# EMBEDWALK — plan

**Dispatchable now** (none has a lane):

1. `share-nothing-face-pairs-are-halved-over-their-carriers-crossing-off-the-faces`
   (P0, M). It is on `origin/embed/gate` (7951bfdfb6) and arrives in
   `work/embed/` when that branch lands; move it here then, or cut the
   unit from the branch's copy now. Measured by EMBED's gate lane: the
   faces are 1.4 to 5.5 cm apart; the sub-millimetre points are carrier
   points in cells holding no point of a face; `sphere_face_region` and
   `torus_face_region` answer nothing for a face with an edge that is
   not a circle (or a spiric), so no cell is dropped. The owed reading
   is the sphere and torus regions for faces bounded by spline section
   edges, or a cell clipped to its trim by another route
   (`cell_region_lifted` already holds the arcs). The at-rest gate's
   landing waits on it.
2. `a-spindle-sheet-or-an-approximated-face-has-no-window-in-the-injectivity-engine`
   (P0, H). 27 census tests.
3. `a-failed-local-certificate-answers-split-with-no-ground` (P1, H).
   Ev's ruling on PR 7309 (an ask stops at its first finding) is built
   by EMBED's PR 7319; this row is what makes the later step (stop only
   at a crossing) possible. Read fork-log row 296 first.

**Arriving from EMBED's lanes when their PRs land** (filed on lane
branches, in `work/embed/` until moved here):

- from `embed/nested-tori` (PR 7119): `coaxial-cone-walls-a-thin-gap-apart-have-no-depth-that-parts-them`,
  `the-depth-gap-is-asked-of-whole-faces-not-of-cells`,
  `the-depth-gap-lives-in-the-census-and-its-pad-is-not-derived-for-the-clearance-engine`,
  `the-depth-gaps-f64-rounding-is-of-the-pads-order-near-the-least-tolerance`,
  `a-side-read-through-a-motion-is-not-asked-its-depths`;
- from `embed/first-finding` (PR 7319): `three-refusal-sites-of-the-embedding-ask-are-reached-by-no-built-body`;
- from `embed/lune` (PR 7113): the `unplaced_line` reading it adds bears
  on `a-coordinate-line-on-a-face-of-unknown-extent-is-unplaced`.

EMBED's orchestrator moves each when its PR merges and notes it on this
log.

**The tail** (P2, P3): the extent and sheet rows moved with the cut.
