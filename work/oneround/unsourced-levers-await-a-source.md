---
id: unsourced-levers-await-a-source
kind: issue
title: 74 lever arms fit none of the three sources and are pinned unsourced, awaiting Ev's ruling or a re-posing
status: open
opened: 2026-11-02
priority: P1
cost: H
pr: 7141
---


Found by the `onehome/one-extent` unit (2026-11-02), which typed every
lever arm (`geom_core::Lever`) and built each from one of the three
sources Ev ruled on (PR 6329, `crates/geom-core/README.md` "Float
rounding"): a read of the consumed extent, a curvature arm, the session
box; rates moved to the metric doors. These arms fit none. Each carries
a `// Unsourced lever: …` line at its call and is pinned per crate by
`scripts/gates/lever-sources.sh` (`lever-sources.counts`, pinned per
file: 63 references at the unit's merge). A fourth source is Ev's
call; otherwise each family is re-posed.

**1. A source exists, but reading it would move verdicts by rounding.**
The arm is the norm of a vector scaled to an extent, or a sum of
reads, where the honest read is the extent itself or their union:
- the sector and germ arms, read through one home,
  `topo::boolean::sectors::metering_arm` (and `departure_arm_lever`,
  which `sector_shape`, `splitting::insert`, `splitting::neighborhood`
  and `splitting::rules` call), and `leaned_lever` (an arm times a
  lean's sine); `census` wedge arm. The vector is a unit tangent times
  `edge_extent_inf`; its norm differs from the extent in the last bits;
- `boolean::vtxfac` `held_reads` (a ball about the runs' shortest arm,
  which can under-state the region) and `unglued_flat_sector`'s ball;
- sums of radii or reaches, whose union read is their `max`:
  `rim_wedge` `same_circle` (2), editor-core
  `checks::at_rest` `constant_pairing`, `eval::measure` `parallel` and
  `plane_round`, `mate::coset` and `mate::value` (2) (`Arm`, an f64
  sum), `geom_brep::curve_pair::pose` `route` (`length + e + e`);
- arc lengths, where the extent reads are diameters:
  `offset_axial` `corner_arms` circle case, `ring_path`
  `spline_crossings` arc piece, `curve_pair::pose` `line_line` and
  `line_conic`; `chord_join` `SectionOval.extent` (the larger half-width,
  short of the oval's ball); `chart_region` `hyp` (2,
  `√(reach² + r_max²)`).

- lengths read off a line's parameter span where the edge extent reads
  the end points' distance: `offset_axial` `corner_arms`' line,
  `boolean::reduce` `line_cone_roots` (`dir.norm()·|Δt|`).

**2. A tolerance or a rounding standing in for an arm** (re-pose, not
a lever): `validate` `resolved`, `boolean::join` `off_conic_slack`,
`dihedral` `wedge_reads_zero_at_the_arm` / `_offer` (`band.escalate()`),
`pcurves` joint room (`d − 2ε`), `intersect` `pn_apex_tangent_reach`,
`pn_apex_point_reach` and `pair_bound` far/near/offset (the plane's
apex distance plus its rounding; `levered_inv` differs in the last bit),
`fate` `sign` (a unit arm on a unit vector's length or a turn count).

**3. A length a caller hands in with no geometry behind it**:
`geom_brep::ssi` `SsiDomain::extent` (a `pub f64` field; its producers
in `pair_section` are geometric, its ~80 test literals are not):
`ssi::boundary`, `ssi::certify::tube_transversality` and its test twin,
`ssi::march::decide_transversality`, `ssi::section::decide_crossing`;
`ssi::march` `ssi_closure_tangent` (the marched polyline's length);
`boolean::trace` `ray_crossings` and its line piece (a ray length) and
`plane_misses` (distance from a plane's stored origin);
`census` `parallel_reading` (a line's parameter interval);
`ring_path` lead `Piece::Segment` (zero, a placeholder); editor-core
`trace` (2: a coordinate gap; a carrier size plus an offset);
`splitting::containment` `extent_from`'s edge balls (topo's own
`step_ball`/`carrier_ball`/`conic_arc_ball`), `solid_contain`
`carrier_roots` (`RuledFaceRegion::reach`, a topo boundary read),
`boolean::recl` corner ball, sweep `blend::open::planar` mitre ball
(sampled chord diameters).

**4. A carrier's stored term or a coefficient**: `pcurve_cache` focal
`beta` (a map coefficient), Villarceau `tilt` (its honest arm, the
Villarceau circle's radius, equals `R` only to the band), `unit_sign`
(2, a speed with no span), `spiric_chart_axis` (the chart's `R`);
`props::curved` `require_cone_circle` `off_axis` (a residual).

**5. `profile` (6)**: `seg` `sagitta`, `junction_reverses`,
`junction_turn`; `path` seam side and `resolve_fillet`; `sugar`
`arc_fillet_trims`. Each is a length of the consumed path (a chord, a
leg), an extent read in kind, but `profile` does not depend on
`geom_brep`, so it has no extent home to read through: whether it gets
one is a design call.

**2026-11-02 (orchestrator).** PR 6410 adds one pin while fixing main after PR 6387 (reads/holdings) collided with this change. In `topo::boolean::overlap::one_sense` (`bool_overlap_senses`), the cosine of two unit normals is levered at a unit arm. Its source is the faces' extent (the sine × extent reading), and the holdings lane owns that choice.

## Slate triage (2026-11-08, ONEHOME split)

- Moved to ONEROUND (`work/oneround/`) by `git mv`, id and body unchanged. ONEHOME measured 78 budget points against 35 after PRs 6371 and 6380 landed, and split along its seams (`work/README.md` Track size): the rows on f64 rounding, levers and the consumed extent that PR 6380 and PR 6391 filed went to ONEROUND.

## Read against main at pickup (2026-11-19, ONEROUND orchestrator)

From the two designers weighing this row for Ev, each item checked in the tree before it was written here.

- **The site list above is stale.** `rg 'Unsourced lever' crates` gives 66 markers, matching `lever-sources.counts`; the title's 74 and the body's 63 are both old. Gone: `splitting::containment` `extent_from`, `boolean::recl`'s corner ball, `checks::at_rest` `constant_pairing`, `unglued_flat_sector`. New and not listed: `star::order` (2), `star::cut`, `coincide::replay`, `link::chord_rows`, `boolean::fan` (2).
- **About seven of the pins are test code**, which the five families have no place for: `star::cut`, `star::order`'s probe, `boolean::fan` (2), a `join` fixture, `link::chord_rows` (`#[cfg(test)] mod`), `ssi::certify`'s test twin. They belong on the fixture doors and need no ruling.
- **`geom_brep::sector_shape::metering_arm(arm: T) -> Lever<T>` is a `pub fn` over one pinned reference**, so every caller that hands it a length is invisible to the per-file pin. The count understates the exposure.
- **`topo::boolean::trace::plane_misses` levers a tilt at the plane carrier's stored origin** (`(origin − point).norm()`), so one plane stored with two origins gives two verdicts. That is a defect whatever the lever's source turns out to be.
- **`editor_core::mate::coset::Arm(f64)` is a second lever type**, whose doc calls it "the one door a lever is formed through".
- **`Lever::curvature` has no caller** under `crates`; `curvature_radius` and `levered_inv` carry its cases.
- **"A sum of radii whose union read is a max" holds only from a shared pivot.** Where each region is read from its own reference point the honest read is across both, and a max under-states it.

## Reconciled for Ev (2026-11-19, ONEROUND orchestrator)

Two designers agree after four rounds (`docs/DESIGN-FORK-LOG.md` row 290): no fourth source, both `unsourced` doors and the pin deleted, each site re-posed by the role its length plays. The question to Ev is the `[ev]` PR this note rides.

- **More exposure the pin cannot see, checked in the tree:** `geom_core::Margin::sagitta(curvature_rel, arm: Lever<T>)` took a bare `arm: T` until PR 7143 typed it (11 production calls), and `Margin::over_lever(measure, lever: T)` with `over_lever_down` (43 calls outside `geom-core`) still take a bare lever.
- **Not needing a ruling, by both designers' read:** the seven test sites go to the fixture doors, and the arms whose honest read moves only last bits are re-read (`metering_arm`'s callers, the line parameter spans in `offset_axial`, `reduce` and `census`, `leaned_lever`, `intersect`'s quotient forms). `[simple.conditioned]` already covers them.
- **For the unit that builds the answer:** classify each of about 34 `Lever`/`LeverBounds` struct fields outside the extent home (a regex count, a heuristic) by two questions: does the struct already hold what the lever is read from (then it is a method, never a field beside); if not, do all its readers pin their data at one point (then it may hold the lever; else it holds its region). `BoundSupport::Bisector.arm`, `BoundSupport::Conic.extent` and `star::walk::Sector.arm` are the three the designers left to it. `Lever::max`'s doc ("a read of the union of their extents") holds only from a shared pivot.
- **Unsure, per both:** `pcurve_cache`'s focal β and `unit_sign`, `coincide::replay`'s power-of-two arm. About 35 to 40 of the 66 sites were classified from their comments, not read; nothing was built or measured, and "about 30 sites move verdicts by more than bits" is an estimate.

## Ev's answer (2026-11-19, PR 7141)

"sounds great! i am less sure about the records thing but it seems fine unless there's some kind of weird information flow that results". So: no fourth source; `Lever::unsourced`, `ExtentBall::unsourced` and `lever-sources.counts` go; each site is re-posed by the role its length plays and the moved verdicts are accepted; the point-level reads move to `geom-core` beside `Lever`; the README's text is as merged.

**The records rule carries Ev's condition.** A unit that changes what a record holds (`SsiDomain`'s ball, a sector's `arm`, `SectionOval::extent`, `Piece::Segment`'s `lever`, editor-core's `Arm`, and whichever of the roughly 34 lever fields the two questions reach) says in its PR what now flows where: which readers gain a view of geometry they did not hold before, and whether any crate or module comes to read a region it had no business knowing. A flow that looks odd goes back to Ev before it lands, on that unit's PR.

The design question is closed; what is left on this row is the build, cut into units at spec time.

## Added 2026-11-20 (`oneround/lever-doors`, the lever-fields pass)

PR 7143 mints curvature arms from carriers, and where the record or function it reaches also needs the radius as geometry it holds the two side by side, paired by whoever builds the record. Each is a half-fix. Where they are headed: the record holds the carrier (a `Circle`, the sphere or the cylinder) and derives both the radius and the arm from it, as `surface_group`'s `Unmated(FaceKey, Circle)`, `census::circle_plane(&Circle)` and `solid_contain`'s rim site do.

- `geom_brep::props::curved` — `SphereArc { centre, axis, radius, arm }`, `SeamLoops { radius, arm }`, `RimArms::uniform(r, arm)`, and the sphere, cylinder and torus functions that take `radius` (or `minor`) beside `arm` (or `tube_arm`), threaded from the surface-match doors. The radius feeds areas, pole points, residuals and `IsoChart`; holding the surface means re-threading about twenty signatures.
- `geom_brep::pair_section::ruling` — `RuledCylinders { rho, r, lever }` and `RuledCylinderSphere { r, lever }`. The first's lever is the lesser of two cylinders' arms; `rho` and `r` feed the discriminant, the meeting point and its velocity.
- `topo::ring_path` — `Quadric::Sphere { centre, radius, arm }`, where `Quadric::Cylinder` holds `radius` alone and reads its arm from the surface handed to `Quadric::lever`.
- `topo::ring_path` — `Quadric::lever(&self, surface, p)`: the record and the surface it is of are passed side by side, so a sphere's stored `arm` and the `surface` argument are two sources of one arm.
- `geom_brep::props::curved::cone_boundary` — `RimArms::uniform(rim_radius, Lever::curvature_radius(rim_radius))`: the one pair whose arm is minted from its own scalar (`cone_arm`'s `|v|·sin α`, or `1`), a pinned bare door.
- `topo::census::point` — the module's own `Surface::Sphere { center, radius, arm }` and the `(c, r, arm)` tuples of `on_sphere` and `sphere_sphere`.
- `topo::solid_contain` — `(radius, arm)` in `latitude_after` and `latitude_extremes`; `sphere_chart_trim` is handed the sphere and its destructured centre, radius and axis together.
- `topo::boolean::ops` — `SphereRecut` holds `arm` alone and reads no geometry from it.

## Read against the tree (2026-11-21, `oneround/unsourced-bits`)

Corrections to the site list above, each checked in the tree at that branch's base (main with PR 7143):

- **`topo::boolean::sectors::metering_arm` and `departure_arm_lever` do not exist.** The one home is `geom_brep::sector_shape::metering_arm(arm: T)`, with nine callers: `geom_brep::sector_shape::sector_shape` (2, reflex and straight), `topo::star::walk` (3), `topo::sector_shape` (`ruling_chord`, `apex_extrema_inside`) and `topo::splitting::insert` (2). The arm reaches every caller as a bare `T` held in `SectorShape::arm`, `FullTurn::arm` and `topo::sector_shape::ApexShape::arm`, the shorter of two departure vectors' norms, so reading the edges' extents there changes what those records hold.
- **`line_cone_roots` is in `geom_brep::curve_face::line_quadric`**, not `boolean::reduce`.
- **`leaned_lever` was `topo::star::order`'s**, re-exported through `boolean::sectors`; it is now `star::leaned`, a reading foreshortened by the smaller lean, levered at the sector's arm, which is still the metering arm's unsourced arm.
- **`offset_axial::corner_arms`** reads its line and fallback arms through `edge_extent_inf`; its circle arm stays the arc length, unsourced. Its verdict acts on every sign (a Zero at `offset_axial_corner` serves `tangent_foot`), so the circle arm takes both bounds, with `ring_path`'s twin, in the arc-length family.
- **`intersect`'s quotient forms** (`pn_apex_tangent_reach`, `pn_apex_point_reach`, `pair_bound`'s far, near and offset) are family 2's (a stand-off plus its rounding over a sine), already listed there.
- **The `metering_arm` family waits whole for the records unit** (ONEROUND orchestrator, 2026-11-21): the row's "bit-only, needs no ruling" was wrong for it, because the arm is not in hand where it is levered. Reading the edges' extents means changing what `SectorShape::arm`, `FullTurn::arm` and `ApexShape::arm` hold (Ev's condition on records). A `BoundSupport::extent()` read would convert only `star::walk` and `splitting::insert`, leaving the family read two ways, and `BoundSupport` is itself one of the records that unit settles. Where the extent actually is: each conic bound's `ConicDeparture::extent` (`edge_extent_inf`), held as `BoundSupport::Conic.extent`; a line bound's chord is its `BoundSupport::Chord` end points. `census`'s wedge arm is the same arm (`Wedge::arm` is `SectorShape::arm`), read twice in `edge_in_curved_face` (its wall sides and `strictly_inside`), so it waits with the family.
- **Converted by that unit (PR 7227)**: six test sites, `leaned_lever`, the line spans (`corner_arms`' line arm, `curve_face::pose::line_cone_read`, `census::parallel_reading`), and `corner_arms`' fallback arm (chord → the extent from below, until `oneround/unsourced-unions` reads both bounds). Moved at its review: `corner_arms`' circle arm to `oneround/unsourced-unions`, `intersect`'s five quotients to `oneround/unsourced-stand-ins`, and `census`'s wedge sides and `ssi::certify`'s test twin (it mirrors `SsiDomain::extent`) to the records unit.

## Read against the tree (2026-11-21, `oneround/unsourced-unions`)

Corrections to family 1's sums and arc lengths and to family 5, each checked in the tree at that branch's base (PR 7227 with main):

- **editor-core `eval::measure`'s two sites read a record.** `arm` sums `Carrier::Plane`/`Cylinder`/`Sphere`/`Pose`'s stored `reach: T`, each a face's cover read once from that carrier's own reference point, and the carrier keeps no cover or face key to read again. The union's honest read is across both faces from one pivot, so it needs `Carrier` to hold its region (an `ExtentBall` from `ExtentBall::of_cover_from`, moved with the seat) in place of the number: the records rule's change, left pinned for that unit. `PlanePair::arm` in `geom_brep::carrier_pose` then holds the arm it was handed.
- **`rim_wedge` `same_circle` was one site, not two**; it now reads both circles' covers from `ra`'s centre.
- **`ring_path`'s arc piece**, `chart_region`'s `hyp` (2), `curve_pair::pose`'s `route`, `line_line` and `line_conic`, and `overlap::one_sense` are converted by that unit.
- **`profile`'s six are not one kind.** `sugar::arc_fillet_trims` levers `Leg::arm`, a field `Leg::at_corner` fills: a record. `seg::junction_reverses` and `junction_turn` and `path`'s seam side take `Seg::arm()`, for an arc `min(radius, chord)`: a curvature arm folded with a two-point read, so the conversion needs a 2D curvature door as well as the 2D point reads. `seg::sagitta` (the chord) and `path::resolve_fillet` (the anchors' separation) are two-point reads alone. The point-level reads (`points_lever_from`, `points_diameter`) are `lever-doors.sh` doors whose pins only fall, and they read `Point3`.
