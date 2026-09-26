# CONTACT-3: the cylinder wall arm reads a tilted-section wall as its planes, not its vertex rectangle

Carries `work/contact/cylinder-wall-trim-overcovers-a-tilted-section`.

## What changes

`point_in_solid`'s ray lane used to decide whether a ray hit a cylinder
wall face by reading `cylinder_chart_trim`'s azimuth × height rectangle,
which is taken from the face's boundary VERTICES. A wall bounded by a
tilted planar section is not that rectangle. The rectangle reaches past
the section where it stands high and falls short of it where it dips, so
the certified walk answered `In` for points below the cut.

The ray lane now resolves each wall face once into a `WallOutline`
(`crates/topo/src/boolean/solid_contain.rs`), and every per-hit reader
goes through it. Both cylinder readers use it: the boundary pre-pass
and `cast_ray`, each through `point_on_wall_in_face`. There are three
classes:

- `Rectangle { h }`: every edge a rim or a meridian, with the rims on
  exactly two levels. Unchanged: the same rectangle, the same margins,
  the same predicate names.
- `Sections([WallSection; 2])`: every edge a meridian or a planar section
  (a rim, or an ellipse centred on the axis with minor semi-axis the
  radius, its plane not parallel to the axis), on exactly two planes. At
  least one plane is tilted. Membership is the azimuth window intersected
  with the face's side of each plane.
- `Unsupported { anchor, reach }`: everything else, answered by a
  confined typed refusal.

## The class predicate, and where it lives

`wall_planes` (`solid_contain.rs`) is the one predicate. Its `iso` flag
is the rectangle half. `iso_bounded_wall`, which was private to
`contain.rs`, is deleted. The face-level door
(`contain::curved_face_containment`) now asks `wall_planes` and serves
`iso` only, as before. There is no second spelling.

The predicate is also narrower than `iso_bounded_wall` in one direction.
Rims on three or more levels (a stepped, L-shaped iso outline) are
outside `Rectangle`, because the rectangle over-covers the step. Rings
are outside the class in both doors; the face door already returned
`None` for them before asking.

## Why two planes decide membership exactly

The argument is stated in full at `wall_outline`. Take a face in the
class with window `W` narrower than a period. Neither plane is parallel
to the axis, so each meets every ruling exactly once, which makes it a
graph `h = f(θ)` over the whole circle.

Fix a ruling at an azimuth strictly inside `W` that is no vertex's:

1. The loop's azimuth image is connected and equals `W`, so the ruling
   meets the boundary.
2. It meets no meridian (a meridian sits at a vertex's azimuth).
3. It meets each plane's edges at most once: two edges on one plane
   cannot overlap in a simple loop.
4. Each crossing is transversal, so the count is even. It is therefore
   exactly two, one on each plane.
5. The ruling's intersection with the face is the segment between
   `f_A(θ)` and `f_B(θ)`, which is exactly "on B's side of A and on A's
   side of B".
6. The graphs cannot cross inside `W` (that would be a boundary
   self-intersection), so the sides hold on every ruling. One reading
   at the window's middle fixes them (`bool_wall_section_order`).

Rulings at vertex azimuths are the closure of their neighbours. Each
side is decided with `decide` under the run band, as the hit's height
above the plane along its own ruling (`n̂·(p − p₀) / n̂·â`), in metres.
That is the same quantity a rim's `height − h₀` is, and it is decided
under the same `bool_wall_trim` row.

Where the argument fails, the arm does not guess:

- A window of a period or more escalates at `chart_azimuth_margin`'s
  period guard, as a full-turn iso wall does today.
- Two planes whose curves meet at the window's middle escalate `Invalid`.

## Where it refuses, and how the refusal is confined

`PointInSolidError::WallOutlineUnsupported { face }` is new. It is
confined the way `EdgeCarrierUnsupported` is. A face lies in its outer
loop's convex hull, because every point of it is on a ruling segment
whose ends are boundary points. So a hit is answered as a miss if it is
definitely outside either of:

- a ball holding the outer loop (`splitting::containment::loop_reach`,
  factored out of `point_in_carrier_loop`'s existing reach so both
  share one body);
- the face's azimuth window, when that window is narrower than a
  period.

Only a hit the face could hold refuses. The census maps the new variant
to `FaceKindUnsupported`, and that reason's text names the new case. The
two editor-core concision lists carry the new arm.

## The cut cylinder, head against base

`the_cut_cylinder_reads_its_truth` is un-ignored and green. On the same
probe loop at this merge base (six poses, 5³ grid, both halves):

| | false `In` | false `Out` | `VolumeUncertified` | `WallOutlineUnsupported` |
|---|---|---|---|---|
| base `c903cdcfe` | 52 | 0 | 654 | — |
| head | 0 | 0 | 706 | 0 |

The +52 refusals are exactly the formerly false probes. Their rays now
honestly cross nothing, and the at-infinity side needs the props lane,
which does not take this wall. The row's own figures (62 false, 282
refusals) were measured on an older base.

The iso walls are bit-identical. On a digest of every `point_in_solid`
answer over a 9³ grid at six poses (4 374 probes each), a plain cylinder
and a quarter sector give the same digest at base and head.

## Cone and sphere (S3)

A tilted section of either is unreachable through every door on this
tree. Each was measured on a frustum (radius 1 at z = 0, 0.5 at z = 1)
and a unit ball cut at tilts 0.3 and 0.6 rad:

- `split` gates both kinds (`CurvedBooleanUnsupported`).
- The boolean refuses the plane×cone pair (`CurvedPairUnsupported`) and
  the tilted sphere section (the typed `SectionInvariant` frontier).
- STEP import, from generated Part 21 files, refuses both at the
  at-rest gate (`TierInvalid` / `VolumeUncomputable`). The cone fails on
  `QuadratureUnsupported` "conic trim on a cone/sphere/torus chart", the
  sphere on `NotIsoRectangle` / `props_rim_axis_parallel`.

No probe reached `point_in_solid`, so there is no count. By reading:

- **Sphere: no false answer possible.** `sphere_chart_trim` checks each
  edge's class, and a tilted circle is neither a rim nor a meridian
  great circle, so the face refuses whole (`PartialSphereFace`).
- **Cone: the same latent shape.** `cone_chart_trim` folds the slant
  window over vertices with no edge-class check. It is filed as
  `work/contact/cone-chart-trim-reads-a-tilted-section-as-its-vertex-window`,
  with the closing argument and the trigger (a door that builds the
  face).

## Rows

- `crates/sweep/tests/pis_arc_capped_poses.rs`:
  - `the_cut_cylinder_reads_its_truth`: un-ignored.
  - `a_wall_point_across_the_section_is_not_on_the_upper_half`: new.
    A point on the wall below the section read `OnBoundary` at base at
    all six poses (verified). A point above it still reads so.
  - `iso_bounded_walls_answer_through_their_rectangle`: new. A plain
    cylinder and a meridian-cut quarter sector: every answer is the
    truth and no refusal names the wall's outline.
- `crates/topo/src/boolean/wall_section_rows.rs`: new, one row per
  bounding side.
  - between the planes; past the rim plane; below the section where it
    stands high (the over-cover); above the section below every vertex
    (the under-cover); outside the window;
  - on the section, which grazes;
  - in band on both sides of the section and under the rim plane, which
    escalates naming `bool_wall_trim`;
  - the confined refusal inside and outside its reach;
  - `the_shared_window_sites_are_the_five_listed`, which counts
    `chart_azimuth_margin` call sites. The cosine-window inventory at
    `point_on_wall_in_face` asked for this check the next time its count
    went stale; this unit adds a site, so the count now has one.

## Sweep: every place a trim rectangle answers membership

The first pass matched by symbol (`*_chart_trim`, `*_chart_windows`,
`face_azimuth_window`, `point_on_*_in_face`, `iso_bounded`) and by shape
(a `(lo.min(x), hi.max(x))` fold, `Margin::of(x − lo)` against a range):

| Site | Disposition |
|---|---|
| `solid_contain::face_geo` → pre-pass cylinder arm | fixed (`WallOutline`) |
| `solid_contain::face_geo` → `cast_ray` cylinder arm | fixed (`WallOutline`) |
| `contain::curved_face_containment` cylinder arm | on the shared predicate, iso half; the stepped outline is now `None` |
| `cone_chart_trim` / `cone_slant_window` → `point_on_cone_in_face` (both lanes) | same shape, unreachable today; filed (contact) |
| `sphere_chart_trim` → `point_on_sphere_in_face` (both doors) | class-checked edge by edge; not this defect |
| `torus_chart_windows` → `point_on_torus_in_face` | class-checked (affine guard, closure, box); not this defect |
| `boolean/join.rs` → `face_azimuth_window` | picks an arc side by azimuth window only, which is the boundary's exact hull; no height rectangle; not this defect |
| `shell.rs` `planar_faces` / `footprints_may_overlap` | a vertex-only footprint box on planar faces misses an arc's bulge; not the ray lane; filed (shell) |
| `review_m0_pr7.rs` `bbox`, `mesh`, `viewer`, `geom-core` folds | bounding boxes, not membership |

The first pass is blind to rectangles spelled as iterator folds or under
other names. The second pass was shaped at that gap: `fold(…min(`,
`min_by`, and the rectangle refusals (`NotIsoRectangle`,
`require_iso_rectangle`).

| Site | Disposition |
|---|---|
| `geom-brep` props `require_iso_rectangle` | class-checked; refuses a notched domain by name |
| `chart_region` | reads trim polygons and refuses a non-straight chart edge; `window_mid` is a fold midpoint, not membership |

What neither pass can match: a membership test composed from a range
that is carried in a struct and compared far from where it was folded.
Such a test is only found by reading its readers. That was done for
every `FaceGeo` arm above.

## Territory seam

- `crates/topo/src/splitting/containment.rs` is `reach`'s. The change is
  the ball computation factored into `reach_from` (behaviour-preserving
  for `point_in_carrier_loop`) and a new `pub(crate) loop_reach` that
  the wall refusal shares.
- `crates/sweep/tests/pis_arc_capped_poses.rs` is `tcost`/`tint`'s. The
  spec pins the witness there, and the new public rows sit beside it.
- `crates/editor-core/tests/refusal_concision{,_chains}.rs`: the new arm
  is added to both enumerations.
- Everything else is contact's: `boolean/solid_contain.rs`,
  `boolean/contain.rs`, `census.rs`.

## Filed

- `work/contact/cone-chart-trim-reads-a-tilted-section-as-its-vertex-window`
- `work/shell/shell-clearance-footprint-reads-vertices-not-arcs`

## Local verification

- `cargo test -p topo --no-fail-fast` and `cargo test -p sweep
  --no-fail-fast` at `CAD_TOLERANCE_EPS` unset, 1e-6 and 1e-12: 3179
  passed, 0 failed at each row.
- `cargo clippy -p topo -p sweep --all-targets -- -D warnings`: clean.
- `scripts/doc-gate.sh --skip-viewer-toolkit`: OK (8 cargo roots).
- The editor-core concision suites were not built locally (disk). The
  combined PR's CI is their verification.
