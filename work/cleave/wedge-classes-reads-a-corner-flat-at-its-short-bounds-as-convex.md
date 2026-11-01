---
id: wedge-classes-reads-a-corner-flat-at-its-short-bounds-as-convex
kind: issue
title: wedge_classes reads a corner whose reflex edge is flat within the band at its short bounds as convex, and classes long edges wrong far out of band
status: closed
opened: 2026-10-08
priority: P1
cost: M
branch: cleave/wedge-classes-flat
closed: 2026-10-10
pr: 4564
---

Filed by PR 4289's fourth review (m1). Pre-existing: main (8d11c126)
reads every one of the witness's readings the same way.

## What

`crates/topo/src/boolean/sectors.rs` `wedge_classes` decides whether a
corner of three faces or more is convex by reading each bound against
every other face's plane (`plane_side_code`, the bound at its own
reach). A bound read `On` changes neither flag (`Ok(SideCode::On) => {}`
in the convexity loop), so a corner whose only reflex edge is flat
within the zero band at its bounds' reach reads as convex. The convex
branch then classes each partner edge by its side of every face's
plane, at that edge's own reach.

Where the bounds are short and the edges read against the corner are
long, that reading is wrong far outside the band: the reflex dent `g`
is within the band at 1 mm, but at 50 m or 1 km the region between the
reflex-adjacent planes is metres wide, and the convex reading puts it
on the wrong side. A silent wrong class: naming takes the rows as given.

## Witness

PR 4289's review-4 probes (`tang/non-convex-cone-review4-probes` @
31580144, `review4/dart_long.py`): a dart of four faces over the ring
`(1,0,0), (0,1,g), (−1,0,0), (0,−1,0)`, bounds 1 mm and 1 cm long,
`g = ±1e-5, ±1e-6, ±3e-7`, material or hollow, read by line edges 1 m,
50 m and 1 km long a hair above or below the flat.
- At `g = ±1e-6` and `±3e-7` with 1 mm bounds, `wedge_classes` reads
  133 classes the exact oracle contradicts, 678 to 233 000 ε from any
  boundary.
- Read directly, the polygon-cone reader (`cone_side`) reads those
  directions `None` or refuses. Inside `wedge_classes` it is never
  asked, because the convexity loop has already decided the corner is
  convex.

`sectors::cone_fuzz`'s long-probe family (`dart_long`) reads this
corner every run; its `wedge_classes` column there is counted as known
wrong against this item and not asserted.

## The shape to give

A bound read `On` against another face's plane leaves the corner's
convexity undecided, not convex: route such a corner to `cone_read`
(which reads it at each probe's own reach, and passes over what it
cannot decide), or refuse. Measure which cells of the suite move first:
every flat-within-the-band corner of three faces takes that branch
today.

## Built (branch cleave/wedge-classes-flat)

**Measured on main (7e607dd).** `sectors::cone_fuzz` (seed
0xf8152e99bbefedec) counted 3, 15 and 0 long-probe `wedge_classes`
readings known wrong at ε 1e-9, 1e-6 and 1e-12. Through the public
doors: a roof creased along `+x` (`z = 0` over `y < 0`, `z = g·y` over
`y > 0`, `g` = ε/2 over 1 mm) whose crease ends at a corner with 1 mm
bounds across it (its end cut back by two slanted planes), against a
pyramid standing on that corner with one edge running 0.9 m between the
two planes. Every op, both operand orders, at all three ε: the result
builds, validates, and its material is right, but
`BooleanNaming::edge_classes` reads that edge `Out` where its midpoint
lies 80 ε inside the roof. A silent wrong naming row; `editor-core`'s
senses read it as given.

**Cause.** The convexity loop let a bound read `On` against another
face's plane vote for nothing, so every flat-within-the-band corner read
convex (or, with an `In` elsewhere, convex despite a reflex edge hidden
in the band); an in-band reading refused while the corner might be
convex. Both say the same thing, that the bound does not decide the
corner at its own reach, and edges reaching farther read the gap as
metres.

**Fix.** A bound is read only against the faces it does not bound (its
own and the one across it on the shared-bound chain). An `On` or in-band
reading there votes nothing only where that plane is the bound's own
face's, or the across face's, facing the same way, as far as the reading
reaches (the classed edges and the corner's sectors,
`bool_corner_one_plane`, the chord between the unit normals): coplanar
or tangent faces, such as a sphere's faces at its pole, read alike met
or joined. Anywhere else the corner goes to `cone_read`, which reads each
edge at its own reach. A corner no bound reads off any plane is met only
where every face lies on one plane, facing one way. The two-face wedge's
bisector takes the same story, and its `Out` now sets the wedge hollow,
so an in-band edge row on a reflex wedge reads as a polygon cone rather
than refusing; an in-band edge row on a met corner does too, as on a
hollow one. Only the in-band escalation is read so; any other side-code
error propagates. After it: 0 wrong at every ε on the fuzz, and the fuzz
asserts the long-probe family again. A first cut that sent every
non-structural `On` to `cone_read` refused
`sweep::tilted_sphere_pair::a_section_passing_near_a_pole_builds_and_one_through_it_refuses_typed`
at ε 1e-6 (`bool_cone_within` in band beside a pole whose faces share
one tangent plane); the coincidence test is what keeps it.

**Pins.** `tests/a_corner_flat_at_its_short_bounds.rs` (every pose,
both dent signs, six op orders; red on main, in the fast set),
`sectors::tests::a_reflex_edge_on_or_in_band_reads_as_a_polygon_cone`,
`a_wedge_reads_each_arm_of_its_bisector` and
`a_direction_in_band_of_a_corners_face_reads_it_as_a_polygon_cone`
(each read against a closed form; the last two red on main's arms), and
the fuzz's long-probe column. Filed:
`work/honeone/boolean-reads-two-faces-coplanar-three-ways-at-three-levers.md`.

## Closed (PR 4564, 2026-10-10)

An `On` or in-band reading no longer votes convex. It votes nothing only where the plane it
reads is one plane with the bound's own face, or with the face across, facing one way, out to the
farthest reach the verdict speaks for (`bool_corner_one_plane`, `|n̂ − n̂′|`). Otherwise the corner
reads as a polygon cone. Met and hollow corners route their in-band rows alike, and the two-face
arm follows the same rule. `cone_fuzz` asserts every reading again, with 0 wrong over 9 seeds. The
three coplanarity spellings are filed on HONE.
