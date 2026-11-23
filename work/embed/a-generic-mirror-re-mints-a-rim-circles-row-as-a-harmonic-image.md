---
id: a-generic-mirror-re-mints-a-rim-circles-row-as-a-harmonic-image
kind: issue
title: A mirror across a plane that is no axis plane re-mints a rim circle's chart row as a harmonic image, so an iso wall is read as a curved wall and a cell holding that circle beside a ruling's vertex is read at no depth
status: open
opened: 2026-11-23
priority: P1
cost: M
---



## Finding

Found by EMBED's lane `embed/clip-vertex` (2026-11-23), posing the body
rows of `crates/sweep/tests/self_intersection_proves_injectivity.rs`,
and traced on PR 7464's head `5421491599` with temporary prints.

`topo::mirror_body` across the plane through `(0.1, -0.2, 0.4)` with
normal `(1, 2, 3)`, then `topo::clearance`'s sweep at the strict bound,
`f64`:

- **After the mirror every rim circle's use on a cylinder wall reads
  `iso = None, harmonic = Some`**; the rulings still read `iso = Some`.
  The re-minted row of a rim circle is a harmonic image and no longer a
  coordinate line.
- `present_chart` hands any cell with such an edge in reach to
  `curved_walls` before it builds a clip, and `curved_walls` answers
  `Unpresented::Local` wherever the circle has a second edge or a
  vertex beside it (its multi-edge arm: `corner_of`, `vertex_corner`,
  `beside_curve`). Every smaller cell holding the vertex holds the same
  edges, so the pair ends at the depth budget.

Two bodies, each an ordinary result that holds unmirrored, turned about
a generic axis, a hundred units off, and mirrored across an axis plane:

| body | mirrored across the generic plane |
|---|---|
| a half rod united with a block under its flat face (`half_rod_on_a_block`) | the wall against one block face: `Uncertified { Departure }` at the vertex; the whole body `Uncertified { 4v1, 6v3, Edge(7v1), Departure }` at `f64`, `Budget` at `Interval` |
| a ring revolved `τ − 10⁻³` (`near_full_ring`) | `Uncertified { 1v1, 3v1, Edge(9v1), Departure }`, a plane end face against the cylinder wall, at both scalars, in all three poses |

Trace counts: the block's two wall pairs, 47 `curved_walls → Local`
exits (the walls list is always a rim circle beside two or more
rulings); the ring's refusing pair, 292 (the shared ruling and the rim
circle, both circles at coarser cells).

**What it is not.** `sharing` never fires in either trace. Every clip
that reaches `zero_of` has `Along::whole = true`. `face_side` is never
undecided. Where a mirrored cell does reach `zero_of` the wall reads
its side through the vertex and the plane signs the two faces opposite
(4 of 4 such asks certified).

**Controls**, same head, same mirror, `f64`, whole body: a unit rod, a
full ring, a 3-radian ring and a unit brick all hold mirrored. A lone
circle is read by `present_curved`; it is the circle with a second edge
or a vertex in reach that refuses. A rigid generic turn keeps the
stored rows, which is why a turn holds and a mirror does not.

**Against main** (`a3d5ce89c`'s `injective.rs`): the block and the ring
refuse in every pose, mirrored or not, so nothing that held is lost.

## Unprobed, and the first thing to test

**Why the re-minted row loses `iso`**: the mirror's re-mint writes a row
that is not a coordinate line, or the lift reads as harmonic a row whose
held coordinate is constant only to rounding. If it is the mint, this
row belongs on the pcurve certification's slate and moves there.

## Other sightings of the pose

- PR 7320's review: a filleted plate holds in six poses and refuses
  mirrored across a generic plane, cylinder against plane across an
  edge ("the mirror door re-mints the rows on turned frames").
- PR 7322's body row mirrored refuses at `Interval` in the distance
  arm: possibly related, another cause not excluded.

## A second exit on the block, expected closed elsewhere

The mirrored block's wall also fails `sides_are_the_faces` 36 times on
the clip holding its ruling's two edges, joined at a vertex: the cover
of two edges of one line, which PR 7322 reads by key. Expected closed
by PR 7322; not separated from the first exit. Re-run the mirrored
block once that PR is in the landing branch.

## Valid or in band

Valid: mirror images of valid solids.

## Evidence

- Pinned as it reads on the landing branch:
  `self_intersection_proves_injectivity::the_ring_and_the_interval_block_refuse_mirrored_across_a_generic_plane`,
  which goes red the day this is fixed.

## On the landing branch (`origin/embed/wave3-land` `8d34076a9e`, 2026-11-23)

With the cover's joint reading in (PR 7322) the same poses read:

- the mirrored block, on the dyadic grid and off it: **embedded at
  `f64`** (the whole body at the verdict door, and the wall against
  both faces at the vertex); at `Interval` the whole body refuses
  (`Budget`), one wall pair `Budget` and the other
  `Uncertified { Departure }` at the vertex;
- the mirrored ring: `Uncertified { Departure }` across an edge of a
  plane face and the cylinder wall, at both scalars.

So the block's second exit (the cover) was the `f64` blocker, as
expected, and what is left is the ring at both scalars and the block at
`Interval`. **Not re-traced on this tree**: that the remaining refusals
exit at `curved_walls` is the trace above, taken before the merge.

**Possibly the same cause as a PCERT row**:
`work/pcert/a-cylinder-rim-on-a-turned-frame-stores-a-rounding-in-its-axial-channel-at-f64.md`
(a rim on a turned frame stores a rounding in its axial channel, so it
is no coordinate line), with the review's step that `mirror_body`
clears the pcurve rows and re-mints them (`mint_pcurves`) where a rigid
pose carries its rows through. **Not checked**: the mirrored rim row's
axial channel was not printed, so whether this sighting is that row's
is open; if it is, this row folds into it.
