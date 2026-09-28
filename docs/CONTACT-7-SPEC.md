# CONTACT-7 — a touch is read in metres over the touch point's star

**Binds one implementer lane.** Deleted at merge; `work/contact/CONTACT-7.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `contact/7-metric-touch`, from `main`. It carries three rows; read
each in full:
- `touch-cone-readings-are-levered-directions-not-face-distances` (P1);
- `census-touch-cones-are-a-third-vertex-sector-builder` (P1);
- `zero-dihedral-conflates-flat-with-slit-in-touch-cones` (P3).

Also read `work/contact/CONTACT-1.md` and `CONTACT-5.md` (both Closed
sections), and arm 2's argument at its loop in
`sweep_cross_solid_backstop`.

## What is wrong

The census's touch analysis (`census.rs`: `TouchSite`, `Cone`,
`touch_verdict`, `touch_lever`) decides each sign as a unit-direction
reading times a lever (`Margin::levered`). A levered reading has the
right sign whenever it is definite, since `sign(x·L) = sign(x)`; the
lever moves only where Zero falls. So a lever is harmless where a Zero
abandons something, and unsound where a Zero is a verdict ("this ray is
on the plane", "this edge is flat").

A face's side of a plane is a property of its points. A ray's side is
a property of one direction. Reading a face through its bounding rays
is exact only at exact zero: under a band, a Zero ray can be tilted by
ε/lever, and the face's far points amplify that tilt by `1/sin α`. Five
review rounds each found this at a different site. The open one is
`an_obtuse_sector_is_read_through_its_rays`: a corner reads Rest while
its face's far vertex is 30× the zero threshold below the floor. A
sixth is pinned as correct today:
`a_dihedral_is_read_at_its_far_face_on_a_real_corner` reads one edge
convex at one end and flat at the other, a verdict that depends on
which way the orbit is walked. Since CONTACT-5, every meeting pair is
cleared only through this analysis.

## Settled design

Two designers weighed this independently and agreed on the reading.

**S1. Every sign is a vertex's signed distance from a plane, in metres.**
Every sign the analysis decides as a verdict is `Margin::of(n·(q − p))`:
- `q` is a boundary vertex of a star face;
- `p` is the touch point;
- `n` is a candidate plane's unit normal.

No lever remains in a verdict. Readings that are dimensionless stay
only where their magnitude is about 1 and only the sign is read: an On
face's outward normal against `n`, and aligned against opposed
normals. Directions (face normals, cross products of edge directions)
only PROPOSE candidate planes; they never decide a sign. Candidate
generation may keep a levered reading where a Zero only skips a
candidate (`census_touch_span`).

**S2. The analysis reads the touch point's finite star, from the
census snapshot.**
- For a vertex: the faces whose boundary holds it, and the line edges
  ending at it.
- For a point inside an edge: the edge's two faces.
- For a point inside a face: that face.

`FaceGeo.boundary`, `EdgeGeo.f_plus/f_minus` and `vertex_faces` already
hold these; confirm that no `Body` orbit is needed. The fan machinery
(`Cone::vertex`, `Cone::wedge`, the orbit walk, sector subdivision,
`touch_lever`) is deleted. The census stops being a third sector
builder by building no sectors. This closes
`census-touch-cones-are-a-third-vertex-sector-builder`. Its premise, that
convexity should be read through `geom_brep::classify_dihedral`, is
wrong: that classifier is unsigned (transverse or smooth), levered at
the edge's extent, and answers a different question (whether the edge
is well conditioned). Say so in its closing note. The boolean and
splitting lanes keep their own sector code, which answers a sector
question the census does not ask.

**S3. The readings.**
1. **A face against a candidate plane**, from its boundary vertices'
   distances:
   - all Zero ⇒ On;
   - all Positive or Zero, at least one Positive ⇒ Above;
   - any Negative ⇒ Below;
   - otherwise in band.

   This is exact for a planar polygon, which lies in its vertices'
   convex hull.
2. **A star lies in the closed half-space** iff no face is Below. Its
   material side is read as follows:
   - with an On face, from that face's outward normal against `n`; two
     On faces that disagree are a fold, refused;
   - else, with an edge on the plane whose two faces are Above, from
     that edge's convexity;
   - else from the star's shape class.
3. **An edge's convexity:** the far face's vertices' distances from the
   near face's plane, in both orders; a definite sign in either order
   decides. Both Zero with aligned normals is flat. Both Zero with
   opposed normals is a fold (a slit), refused; read it through
   `geom_brep::classify_material_pairing`. This closes
   `zero-dihedral-conflates-flat-with-slit-in-touch-cones`.
4. **A Zero that is a verdict input** means the smaller face lies
   within ε of the other's plane. Substituting it is then ε-true over
   that face, and every verdict is local to it. State this argument at
   the site.

**S4. A Rest is constructed only by the check.** Give the Rest verdict
(or the certificate it carries) a private constructor, reachable only
from the function that runs S3's readings against a candidate plane.
Add a source row, after the census's existing source-scanning rows,
that forbids `Margin::levered` inside the touch analysis except at the
named candidate-generation site. A sixth round of this class must fail
to compile or go red, not be found by review.

**S5. Crossing is claimed only on decided readings.** A candidate that
fails only in band, or only because a non-convex face reaches back
across the plane, refuses typed (`Unanalysed` or in band); it is not
reported as `Crossing`. Arm 2's undeclared path blocks on any
non-Rest, so this costs it nothing. The declared-only path refuses only
on `MixedTouch`; that is
`declared-only-meetings-clear-at-the-census-gate-unread`'s question,
and it is noted on that row. Do not change `blocks` here.

**S6. Arm 2's argument states its tolerance.** A Rest certifies that
every point of both stars is within ε of its correct side of one
plane. Any local overlap is therefore confined to a slab 2ε thick,
which is coincidence under D4 and not interference. Write this premise
into arm 2's argument at its loop.

## Rows

- **`an_obtuse_sector_is_read_through_its_rays`** flips from Rest to
  not-Rest. Re-sign it, and pin it at 2, 5, 30 and 500× the zero
  threshold, plus the synthetic fan.
- **`a_dihedral_is_read_at_its_far_face_on_a_real_corner`** now reads
  both ends convex. Re-sign it, and say that the old pin encoded the
  defect.
- **Every existing touch row** (CONTACT-1's kinds, CONTACT-5's edge
  cross, the census and bool4 rows) answers as before or better. List
  every row that moved, with the reason.
- **Exactness against a bound:** the obtuse-sector family and a
  non-convex face at a touch, with the exact separating plane in band.
  These refuse, and never Rest.
- **Sweeps:** re-run CONTACT-5's brick grid and CONTACT-1's reviewers'
  rotated-prism and crossed-ridge sweeps. Wrong clears must be 0.
  Report false refusals of true rests, head against base. They must not
  grow on any sweep without a named reason.
- **K and audit:** `census_touch_side` and `census_touch_dihedral`
  become `Margin::of` rows. Update `docs/predicate-dimension-audit.md`
  and `docs/K-REPORT.md` to match.

## Review

**Dual.** The feared failure is a wrong Rest, on the door every
consumer reads as proof of no interference. The second is a flood of
false refusals of ordinary rests.

## Discipline

The usual lane rules apply:
- your own clone and `CARGO_TARGET_DIR=/home/user/contact-7-target`,
  with `CARGO_INCREMENTAL=0`;
- `with-build-slot.sh` for every cargo command;
- `df` first;
- no doc-gate script;
- no pattern kills and no process listings, and never list the shared
  scratchpad;
- push the branch only, no PR.

Run the `editor-core` concision rows, the `perf12` goldens and
`-p test-utils` before you hand back: this unit adds a source-reading
row and may move refusal reasons.
