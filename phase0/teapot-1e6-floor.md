# PHASE 0 — teapot wall 3 at ε 1e-6 (`splice/teapot-1e6-floor`)

PHASE 0: waiting for the orchestrator. (This file is lane notes; it comes
out before the PR.)

Measured on main 7072c784, `demos/tour` `every_teapot_wall_is_attempted_by_the_suite`,
with a temporary probe in `spline_face::isolate`'s floor branch (reverted).

## The floored piece (ε 1e-6, band escalate 1e-5)

- Fragment walked: meridian edge 9v1 (circle, centre (0, Y_BELLY_C), r = R_BELLY,
  pace 0.078125 m/rad), span t ∈ [0, 0.285703] rad.
  C(0) = (−R_NECK, Y_MOUTH, 0), the mouth junction; C(0.285703) =
  (−0.062589, 0.109256, 0), the seam split point (PR 6441) the boundary
  pass cut the meridian at. Face 9v1's boundary pass against this
  fragment read `Boundary::Clear`, so the edge went to `spline_interior`.
- Walk window in t: [−0.131072, 0.416775]. END_REACH·escalate/pace =
  1024·1e-5/0.078125 = 0.131 rad = **10.2 mm of arc past each end**.
- **Floored box**: t ∈ [0.342944, 0.343479], u ∈ [0.998047, 1.0],
  v ∈ [0.558838, 0.559082]: **0.0572 rad = 4.47 mm of arc PAST the
  fragment's end b** (and past the seam split it ends on), 0.343 rad from a.
  It lies on the chart's domain edge u = 1 (the bore wall's z = 0 seam).
- Its enclosure of F = C − S: x [−2.49e-5, 1.33e-5], y [9.73e-6, 6.85e-5],
  z [−1.9e-8, 3.50e-5]; |F| enclosure reaches ~8e-5 m against band
  escalate 1e-5 m. F at the box's midpoint: |F| ≈ 4.3e-5 m. Floor side
  = sqrt(rho·escalate)/8 = 5.9e-5 m (rho = 0.0223 m).

## True clearance (dense sample t × 41² (u,v) grid + pattern-search refine)

- On the span [0, b]: clearance falls linearly from 35.9 mm (t = −0.131)
  to 3.9 mm at t = 0.289 (≈ b). **The pair is clear on the span by ≥ 3.9 mm.**
- Past b: the nearest point is ON u = 1 at every t (the meridian stays
  outside the face, across its seam edge), clearance falls linearly at
  ≈ 0.072 m/rad (|sin θ| ≈ 0.92 at the pace) to **zero at
  t ≈ 0.34373, point ≈ (−0.06517, 0.10558, 0), chart (1.0, 0.55913)**,
  then rises again (5.2 mm at t = 0.4168). The sampled minimum 2.1e-5 m
  is grid resolution; the shape is a V to zero: a **transverse meeting of
  the meridian with the bore wall's seam edge**, which is the next
  fragment's ground (the bore seam crossing the sphere in the symmetry
  plane), not this fragment's.

## Classification

None of (i)–(iii) as stated; nearest to (i): the floor is not at a
near-graze and not a fixture offset. It is **a box the walk had no
question about**: a meeting on the chart's domain edge (which by the
module's own contract no interior box isolates, so it floors at every ε
it is reached) that lies 4.47 mm past the span's end, read only because
the end reach `END_REACH·escalate/pace` scales linearly with ε and at
1e-6 is 10.2 mm, larger than the features (wall thickness at the root
R0·(1−BORE) = 5.86 mm; outer-seam→bore-seam gap along the meridian
4.47 mm). Not (iii): no ε-independent offset; SPOUT_BORE/SPOUT_R0 set a
millimetre geometry, and the bore chart's corner is at z = −BORE·R0, off
this piece. The clearance is exactly zero at a transverse edge meeting,
so not (ii).

**Why the other rows don't floor**: they never meet this pair here. At
1e-9 the reach is 1024·1e-8/0.078 = 1.3e-4 rad (10 µm); at 1e-12,
10 pm. The meeting 4.47 mm past b is far outside both windows, the
fragment reads clear, and both rows go on to `GermFrameUnsupported
{ a: 6v1 Sphere, b: 3v3 Nurbs }` (re-measured: default and 1e-12 both).

The reach's scaling: it is ∝ ε while the floor is ∝ √ε, so
reach/floor (in metres) = 8192·√(escalate/rho): ≈ 173 at 1e-6, ≈ 5.5 at 1e-9,
0.17 at 1e-12. "A little past each end" is millimetres at 1e-6.

## Proposed fix (walk fix)

`geom_brep::curve_face::spline_face::isolate`, the floor branch: a box
at the floor whose carrier piece lies **past an end of the span** (its
t-interval wholly beyond b, or before a) **and whose carrier enclosure
C(x.t) is decided past the band from that end's point** is no question
of the span's: by the module contract no solution past an end is
reported, and every solution in such a box is past the end and not
within the band of it. It settles instead of refusing. A box straddling
an end, or past it but within the band of the end point, still refuses
as today (the end's box or a refusal accounts for a meeting at the end).
- Cannot loop or over-run: it only turns a terminal `Stop` into
  `Settled` on a box that is at the floor already; it adds no pieces.
- ε-independent: the test is metric (|C(t) − C(end)| past the band),
  not a parameter count, so it holds at every row.
- Alternative/companion considered: cap END_REACH (e.g. to a multiple of
  the floor). It shrinks the window but leaves the class (an edge
  meeting inside whatever reach remains past an end still floors), so I
  propose the settle rule alone; the reach stays as the end meeting's
  room.
- Crease class: a box past the end and near a crease of the carrier
  beyond the end is equally no question of the span (the window already
  stops at the nearest crease past the end); the chart's creases are
  unchanged (`Unsettled::Crease` keeps its label for boxes that still
  refuse). Feet on another sheet: unaffected (a settled box claims no
  meeting).

Expected new 1e-6 stop: the fragment reads clear; the next fragment's
end at the bore seam is a boundary meeting; the wall should reach
`GermFrameUnsupported` like the other rows (to be confirmed).

Kernel witness: a `geom-brep` row in `spline_face`'s rows: a circle arc
(meridian-like) ending at t = b, a NURBS bicubic/rational wall whose
domain edge u = 1 is crossed transversally by the arc's continuation at
d = 4.47e-3·(ε/1e-6)… scaled so the meeting lies inside END_REACH's
window at every ε (d = 0.5·END_REACH·escalate), clear on the span:
red on `Unsettled::Floor` today, `Ok(no meetings)` after.

Siblings (to sweep at build): other walks reading a reach past an end
and refusing at a floor there — `composite::crossings` (spline span ×
analytic face), `NurbsLane::chart_meetings`' callers, the circle×torus
and plane-crossing arms' end windows; GROWTH (4096·|Y|·escalate) is
re-proved and refuses nothing, so not a sibling.
