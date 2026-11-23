# Sweep item 2 — the readers of a conic's derived centre

**How this was made.** The table below is a READING pass (a delegated
read-only audit: every verdict is from the code, nothing run), over the
63 expectations whose reason is "a conic's centre frame, derived where
an ellipse is held on its arc", plus three readers the gate does not
see. Two of its suspects I then RAN on `origin/main` (2130aa45f), public
API, and both answer wrongly; they are reported in their own message:

| suspect | probe | result on main |
|---|---|---|
| `curve_face::pose::plane_conic`'s belly graze (row 2's consumer) | `plane_crossing_lane` with the ellipse's own plane through its stored point | `Miss` at `a` = 1e9 (ε 1e-9) and `a` ≥ 1e7 (ε 1e-12); `InPlane` below, one `PlaneParallel` escalation between |
| `Curve3::clearance` through `Foot::on_carrier` (row 8, section (c)) | a point 5ε off the arc, inside the band's gap | `Off` at `a` ≥ 1e8 (ε 1e-9) and `a` ≥ 1e5 (ε 1e-12); refused below |

`line_conic`'s height and `Slant`'s `H` (rows 1, 2) and
`Window::height_jet` (row 7) are M1's measured `meet` misses. Every
other SUSPECT row is read, not run: its pose is derived from the code.
Reachability caveat from sweep item 1: `split` builds a far-section body
only to `a` ≈ 6e5 at ε 1e-9.

---

# Sweep 2: production readers of a conic's derived centre

Branch `oneround/conic-harmonic-probe`, read-only (Read/Grep only; nothing built or run).
Scope: the 63 `#[expect(clippy::disallowed_methods, reason = "a conic's centre frame, derived where an ellipse is held on its arc ...")]`
sites (the other 79 expectations say "a circle's centre frame, which a circle stores" and are out of scope), plus (a) `Conic::unit_in_plane`,
(b) `Window::holds_only`, (c) `Curve3::clearance`. `scripts/gates/conic-centre-reads.sh` reads `conic-centre-reads.counts` (per-file pins, both
reasons summed); it pins the count of listed functions only, not reads through a `Frame` handed on, nor `unit_in_plane`.

Facts the verdicts rest on:
- The ONLY production mint of an arc-held ellipse is `plane_cone_section` (`crates/geom-brep/src/intersect.rs`, `Curve3::ellipse_anchored`,
  the `TiltedEllipse` arm). `Ellipse::mapped/translated/reversed/seamed_*/map_scalar` keep it arc-held. `Curve3::ellipse_anchored` is public.
  Plane x cylinder ellipses and STEP ellipses go through `Ellipse::new` (centre-held) - not re-verified site by site.
- `Ellipse` does NOT override `unit_in_plane` (nor `center`); it overrides `frame`, `extent_along`, `cover_over`, `point_at`, `azimuth`,
  `running_point`. So `unit_in_plane` on an arc-held ellipse subtracts the derived centre.
- `rounding_charge(t)` = 16 half-ulps of `t` (~1.8e-15*t); `carrier_resolution::resolution` = u*speed_hi*(64+tau). Either covers u*a when
  the semi-axis is in its terms.
- "bare" below = `Margin::of(x)` / exact `compare` with no slack term containing a semi-axis.

Depth marks: [F] function read in full; [C] consumer of its result also read; [nc] consumer NOT read (verdict is on the function alone).

## Table (SUSPECT first)

| # | file :: function | what it decides from the derived centre (expression) | metered? | reachable by an arc-held ellipse? | verdict |
|---|---|---|---|---|---|
| 1 | geom-brep/src/curve_pair/pose.rs :: `line_conic` [F,C] | (i) APART: `c = (k.frame().origin() - o).dot(u)`, extent `(c - speed_hi, c + speed_hi)` into `OnOneLine` -> `route` row GAP; (ii) PARALLEL / level OFFSET: `height = (o + u*at.mid() - k.frame().origin()).dot(n)` -> `route` rows PARALLEL, OFFSET; (iii) crossing arm: seed `o + u*((k.frame().origin()-o).dot(n)/den)` and `height = (point - k.frame().origin()).dot(n).abs()` sets `Polish.reach` (too small/large) | NOT. `OnOneLine` carries `rounding_charge(ends(ends_a))` = the line ends' distance from the origin only; `route` reads `Margin::of(offset - rate*half)` bare | yes: `b` is any `Curve3::Ellipse` span (`conic_of`) | SUSPECT |
| 2 | pose.rs :: `height` (pub(crate)) [F,C] | computes `c0 = plane_offset(on, n, k.frame().origin())`; consumers decide on it bare: `line_conic` in-plane arm (`Range::first_harmonic(c0, ..)` -> `owed`/`harmonic_roots` -> `Harmonic::Apart`; `Slant.r`, `Slant.h.0` -> `within_zero`, `read` floor), `conic_conic` (`constant_as(PARALLEL, c0, swing)` -> Apart), `transverse` (`height_extremes`, `band_arc`), and **`curve_face/pose.rs::plane_conic`** (`decide("curve_face_pose_belly_graze", Margin::of(r - d0.abs()))` -> `Pose::Apart`; `constant_residual_as(.., (d0, r))` -> lies/apart) | NOT at any consumer read | yes: every ellipse against a plane or another conic | SUSPECT |
| 3 | pose.rs :: `conic_conic` [F] | APART: the other conic's plane stated through `on(k) = k.frame().origin()`, so `ha = height(&ka, on(&kb), kb.axis())`, `hb` likewise; `constant_as(row::PARALLEL, c0, swing, band)` -> `Pose::Apart`; also slant `c0.abs() + swing` | NOT (`arm` = ExtentBall lever is used for `sense` only) | yes | SUSPECT (c0 carries BOTH centres' rounding) |
| 4 | pose.rs :: `level_on` [F] | computes `Level { centre: kb.frame().origin(), .. }` and `w = ka.frame().origin() - kb.frame().origin()` (the latter only in the 4th-derivative bound, `up::<24>`, harmless) | NOT for `centre` | yes | SUSPECT via #5 |
| 5 | pose.rs :: `Level::jet_at` [F,C] | computes F and F', F'', F''' from `q = p - self.centre` and `ddp = self.a.frame().origin() - p`; consumed with zero error by `Window::read` (`taylor(level.jet_at(m), [z0;4], ..)` -> recedes / on / d_lo, d_hi) and by `coplanar_ellipses` (`Rounded::exact`) | NOT (doc says "uncharged at f64") | yes | SUSPECT |
| 6 | pose.rs :: `coplanar_ellipses` [F] | roots / APART / tangent of the certified subdivision: `residual = ellipse_level_clearance((d.dot(ub), d.dot(vb)), ..)`, `d = ka.point_at(theta) - kb.frame().origin()`; `SubdivisionFrame.noise = 0` ("No harmonics are read") | NOT | yes (kb arc-held) | SUSPECT |
| 7 | curve_pair/pose/window.rs :: `Window::height_jet` [F,C] | EMPTY WINDOW: see (b) | NOT | yes | SUSPECT |
| 8 | geom/src/curves/on_carrier.rs :: `Curve3::clearance` [F,C] | CLEAR/OFF: see (c) | NOT | yes (ellipse arm) | SUSPECT |
| 9 | topo/src/splitting/section_loops.rs :: `line_clears_conic` [F,C] | CLEAR (one side): `positive("split_nest_line_conic", m.dot(c.frame().origin() - p).abs() - reach, band)` | NOT (`validate::definitely_positive`, bare) | yes: `outlines_disjoint` routes every `Curve3::Ellipse` outline edge here | SUSPECT |
| 10 | section_loops.rs :: `conics_clear` [F,C] | CONTAINMENT (inside/outside): `(cx, cy) = e.unit_in_plane(h.frame().origin())`, `inside = (1 - centre - reach)*lever`, `outside = (centre - reach - 1)*lever` | NOT (only `reach` padded 8 eps) | yes; two centre reads, one hidden (`unit_in_plane` default on `e`) | SUSPECT |
| 11 | geom-brep/src/props/curved.rs :: `require_cone_ellipse` [F] | SIDE (nappe) and ON-CONE: `w = (center - apex) + am`, `center = conic_k.frame().origin()`; `classify_as("props_cone_ellipse_nappe", Margin::of(h_low))`, `require_zero("props_cone_ellipse_on_cone", Margin::of(elevation))` | NOT | yes, by construction: a cone face's ellipse edge IS the `plane_cone_section` mint | SUSPECT (most likely outcome a false refusal `OffSurface`, also a wrong nappe side) |
| 12 | topo/src/replace_face.rs :: `cone_v_range` [F,C] | computes `(c - amp, c + amp)`, `c = v_of(ellipse.frame().origin())`; `offset_charts_staged` decides `decide("offset_apex_window", Margin::of(v_near * sense))` on it (`group_cone_v_window` caller [nc]) | NOT | yes (cone face boundary edges) | SUSPECT (`c - amp` cancels to the vertex nearest the apex) |
| 13 | geom-brep/src/curve_pair/on_surface.rs :: `plane` [F,C] | computes `(frame.origin(), frame.axis(), frame.u_ref())`; `under` builds `Surface::Plane` through the derived centre and `co_surface` -> `curve_face::cross` decides InPlane / Touch / Roots | NOT | yes (`Curve3::Ellipse` arm); caller `topo/src/region.rs` `on.under(..)` [nc] | SUSPECT (plane displaced by the centre's rounding along the normal) |
| 14 | sweep/src/blend/surgery.rs :: `piece_distance` (ellipse arm) [F,C] | CLEAR: `d = (e.frame().origin() - c).norm()`, `((d - major).max(0), d + major)`; `CapSliver::whole_clearance` `near - self.reach` -> `ring_clearance_pass` `classify(.., Margin::of(margin))`; also `end_face.rs::apart` | NOT | only if an end-face / rim edge is a cone-section ellipse [reachability not traced] | SUSPECT (thin: whole-carrier bound, bites only at a vertex of the major axis) |
| 15 | surgery.rs :: `piece_along` (ellipse arm) [F, callers partly] | CLEAR: `mid = (f.origin() - c).dot(u)`, returns `(mid + low, mid + high)`; `whole_clearance` `floor - high`; other callers in surgery.rs (near `piece_along(carrier, piece, o, u)`) [nc] | NOT | as #14 | SUSPECT (`mid + high` cancels at semi-axis scale near the anchor) |
| 16 | sweep/src/blend/open/end_face.rs :: `SectionFrame::depth` [F] | CONTAINMENT: `self.scaled(k.frame().origin() - center) + k.speed_hi()/self.minor()` | NOT | as #14 | SUSPECT (thin: a whole-carrier bound never positive for a large ellipse) |
| 17 | end_face.rs :: `SectionFrame::outside` [F] | CLEAR: `self.scaled(k.frame().origin() - center) - k.speed_hi()/self.minor()` | NOT | as #14 | SUSPECT (thin) |
| 18 | end_face.rs :: `apart` [F] | CLEAR: `round = (k.frame().origin(), speed_hi, speed_lo)` -> `(near - outer).max(inner - far)` | NOT | as #14 | SUSPECT (thin) |
| 19 | geom-brep/src/pcurve_cache.rs :: `anchored` [F] | computes the moved carrier/chart and the slack for `incidence_over_span_as` (ON-CHART). `k = exact_vec(carrier.frame()?.origin() - origin)` is entered EXACT; the carrier is `translated(-k)` then `centred_at(origin)`, each rounding an arc-held anchor at the semi-axis scale; `error` sums only the offset's running bound | PARTLY: slack is u*(chart anchor - centre) scale by coincidence, the centre's own rounding and the two anchor moves are not in it | yes (ellipse on plane/cylinder/cone chart) | SUSPECT (weak) |
| 20 | geom-brep/src/implicit.rs :: `conic_torus_jet` [F] | computes `[Rounded; 4]`; `c = exact_vec(conic.frame().origin())` entered EXACT, so the running bound omits the centre's rounding (it charges `c - tc` and `+ e` at their own sizes) | PARTLY (same order, not a bound) | yes if an ellipse edge is read against a torus [consumers nc] | SUSPECT (weak) |
| 21 | geom-brep/src/carrier_net.rs :: `harmonic` [F] | computes an "enclosing" harmonic: `p0 = pt(carrier.conic()?.frame().origin())` lifts the f64-derived centre as a POINT (u, v are lifted through `map_scalar`) | NOT at f64 (an Interval body encloses it) | yes [`EnclosingNet.rounding` and consumers nc] | SUSPECT (weak: an enclosure built on an unenclosed centre) |
| 22 | topo/src/injective.rs :: `one_stated_conic` [F] | IDENTITY: `f.origin().equals_exactly(g.origin())` on two derived centres | NOT (exact compare) | yes | SUSPECT (weak: two arc-held ellipses whose anchors differ below ulp(a) derive one centre and read "one conic") [`continues` consumer nc] |
| 23 | topo/src/boolean/join.rs :: `bool_connect` [fragment] | computes `GermLane::EdgePlane { plane: (center, germ_normal(..)) }`, `center = frame.origin()` | consumer [nc] | yes (`along.carrier` any conic) | UNDECIDED - same shape as #13 (a plane stated through the derived centre); consumer of `EdgePlane.plane` not read |
| 24 | topo/src/ring_path.rs :: `misses_in_plane` [F] | SAME-CIRCLE: `apart = (p.circle.frame().origin() - conic.frame().origin()).norm()`, `decide("split_ring_path_same_circle", Margin::of(differ))` bare; then span readings charged `slack` | first decision bare, the certifying ones charged `slack` (`path_resolution` folds `carrier_resolution::resolution`) | yes | COVERED with a caveat (a Zero misread only admits the charged disjoint-arcs test) |
| 25 | ring_path.rs :: `coplanar_apart` [F] | APART: `gap` from the centres' distance | METERED: `Margin::of(charge(gap, slack))` | yes | COVERED |
| 26 | ring_path.rs :: `ConicArc::distance_floor` [F,C] | computes `Circle::new(k.frame(), b).distance_within(x, span) - (a - b).abs()`; `segment_misses` decides | METERED: `charge(.. - half, slack)` | yes | COVERED |
| 27 | ring_path.rs :: `LoopArc::plane` [F] | computes `SectionArc.origin = c.conic().frame().origin()`; `path_parity` hands it to `plane_roots(.., slack, band)` | METERED per `path_parity` (slack passed) [`plane_roots` body nc] | yes | COVERED (as far as read) |
| 28 | pose.rs :: `one_curve_gap` [F] | ONE CURVE: `(e.frame().origin() - f.frame().origin()).norm() + frame_gap` | METERED: `+ rounding_charge(reach(e) + reach(f) + gap)` | yes | COVERED |
| 29 | implicit.rs :: `sample_rounding` [F] | computes a charge: `at = |centre| + a`, `offset = |centre - o| + a` | it IS the meter | yes | COVERED |
| 30 | implicit.rs :: `quadric_harmonics` [F] | computes harmonics from `d = conic.frame().origin() - origin` | METERED: `terms = (d.norm() + speed_hi)^2 + r^2`, consumers charge `rounding_charge(h.terms)` | yes | COVERED |
| 31 | implicit.rs :: `conic_cone_harmonics` [fragment] | same, `d = centre - apex` | METERED: `terms` built from `reach_m = d.norm() + speed_hi` levers | yes | COVERED |
| 32 | implicit.rs :: `conic_axis_distance_floor` [F] | computes a floor | METERED: `- rounding_charge((d.norm() + speed_hi)^2)` | yes | COVERED |
| 33 | implicit.rs :: `conic_torus_harmonics` [fragment] | harmonics from `w = centre - t_center` | has a `terms` field (`p_abs*p_inputs + ..`); its composition not read | yes | COVERED (not fully read) |
| 34 | implicit.rs :: `conic_residual_harmonics` [F] | plane arm `c0 = d.dot(normal)` | METERED: `noise = rounding_charge(d.norm() + speed_hi)`; quadric arms `rounding_charge(h.terms)/(2r)` | yes | COVERED |
| 35 | geom-brep/src/curve_face/conic_quadric.rs :: `apex_spread` [F] | computes a charge: `direction_rounding((centre - apex).norm() + speed_hi)` | it IS the meter | yes | COVERED |
| 36 | geom-brep/src/curve_face/spline_face.rs :: `ConicPath::of` (`fn of`) [F] | computes `c = conic.frame().origin()` on the carrier lifted to `Interval` | enclosure: the centre is derived in interval arithmetic | yes | COVERED |
| 37 | topo/src/injective.rs :: `conic` [F] | computes `Trig { c: center, .. }` on `Curve3<Interval>` | enclosure | yes | COVERED |
| 38 | injective.rs :: `one_conic` [F] | IDENTITY on Interval frames via `one_plane`/`one_value` | a bracket wider than a point is "never one conic" (doc), so a derived Interval centre reads false | yes | COVERED (conservative); delegates to #22 when both are `stated` |
| 39 | injective.rs :: `planes_through` [fragment] | computes `Psi::Linear { n, q }` from an Interval frame origin | enclosure [consumer nc] | yes | COVERED (as far as read) |
| 40 | geom/src/cover.rs :: `Cover::arc_of` [F,C] | computes an `Arc` piece with `frame = Conic::frame(&ellipse)` and `on: Some(ellipse)` | `Arc::support`, `range`, `ball` return through `on` (`extent_along`, `chord_range`, `chord_ball`) before touching `frame` | yes | COVERED (other `Arc` readers of `frame`, e.g. `point`, `far`, `circle_about`, are behind those early returns as far as read) |
| 41 | pcurve_cache.rs :: `derive_class_zero` (ellipse arm) [fragment] | SIDE: `decide("pcurve_cone_chart_nappe", Margin::of(h))`, `h = (center - apex).dot(axis)`; computes `FocalImage.v0 = h/c_ha`, `r0 = radial(w)`, `ecc` | bare, but the margin is the centre's own height (semi-axis scale, not a cancellation); comment: "the envelope measures the image against the carrier" [envelope nc] | yes (cone chart = the mint) | COVERED as far as read (image constants carry u*a; envelope not verified) |
| 42 | pcurve_cache.rs :: `carrier_harmonic` [F] | computes `Harmonic3 { c: k.frame().origin(), .. }` | consumers (`chart_pcurve`, envelope) [nc] | yes | computes only - not traced |
| 43 | geom-brep/src/pcurve.rs :: `ellipse_pcurve_on_plane` [F] | computes NURBS control points from `c2 = (center - q)` in chart | none; no decision | yes | N/A (computes only; the 2D image is off the evaluator's arc by the centre's rounding) |
| 44 | topo/src/shell.rs :: `cut_counting` [fragment] | computes crossing parameter `phi +/- root`, `root = acos(-sc/|(a,b)|)`, `sc = (conic.frame().origin() - line.origin).dot(m)`; sides are decided on `conic.point_at(..)` points | sides sound; `root` bare | yes | N/A (computes only: the COUNT comes from point sides; the crossing's parameter carries u*a/amplitude) |
| 45 | topo/src/stands.rs :: `line_meetings` [F,C] | computes roots of the line/conic quadratic in unit coordinates about `ellipse.frame().origin()`; caller keeps `s` where `definitely_positive("stands_across_meeting", s)` and pushes a candidate point | bare; candidates only [their consumer nc] | yes | N/A (computes candidates) - not traced further |
| 46 | topo/src/census/depth_gap.rs :: `face_read` [F] | computes `Around::Line(conic.frame().axis_line())` as a proposed depth | consumers (`face_nearest`, pair readings) [nc] | yes | not traced |
| 47 | topo/src/boolean/join.rs :: `germ_section_loops` [F] | computes `SectionLoops::conic(frame.axis_line())` | [nc] | yes | not traced |
| 48 | join.rs :: `pk_section` [F] | computes `PkSection::Loop(frame.axis_line())` straight off `plane_cone_section`'s `TiltedEllipse` | [nc] | yes, by construction | not traced |
| 49 | step-import/src/on_built.rs :: `Conic::of` [F] | computes `center: e.frame().origin()` | [nc] | unknown (which built curves reach it not read) | not traced |
| 50 | pose.rs :: `coplanar_circles` [F] | apart/concentric from `w = ka.frame().origin() - kb.frame().origin()` | bare | NO: `Coplanar::Circles` arm, two circles, always centre-held | N/A |
| 51 | ring_path.rs :: `torus_circle_arc` [fragment] | meridian/parallel/crest from `circle.frame().origin()` | bare | NO: matches `Curve3::Circle` only | N/A |
| 52 | topo/src/offset_axial.rs :: `mint_carrier` [fragment] | circle arms read centres; the ellipse path reads `frame.axis()` only (`normal_of`) | - | centre reads are circle-only | N/A |
| 53 | section_loops.rs :: `whole_conic_principal` [F] | reads `frame.u_ref()` only | - | - | N/A (no centre) |
| 54 | topo/src/boolean/vtxfac.rs :: `read_vertex_on_face` [fragment] | reads `e.frame().axis()` only | - | - | N/A (no centre) |
| 55 | props/curved.rs :: `cone_loop_winding` [fragment] | ellipse arm reads `frame.axis()` only; points via `carrier.eval` | - | - | N/A (no centre) |
| 56 | geom/src/curves.rs :: `Curve3::representability_margins` [F] | reads `e.frame().axis()`, `.u_ref()` only | - | - | N/A (no centre) |
| 57 | geom/src/curves.rs :: `Curve3::frame` [F] | a door: hands out `e.frame()` | - | - | N/A (one of the five gated doors) |
| 58 | topo/src/splitting/section.rs :: `plane_section` [fragment] | output: `SectionEdge::Arc { center: chart(conic.frame().origin() - plane.origin), .. }` | - | yes | N/A (reported value, off by u*a) |
| 59 | topo/src/readback.rs :: `edge_pose` [F] | output `Pose { origin: center, .. }` | - | yes | N/A (reported value) |
| 60 | step-export/src/writer.rs :: `edge_curve` [fragment] | writes `axis2_placement(&conic_k.frame(), ..)` | - | yes | N/A (the file states the centre-held ellipse at the rounded centre: a curve u*a off the kernel's) |
| 61 | geom-brep/src/curve_face/conic_oracle/exact.rs :: `truth` [fragment] | the double-double oracle takes `ellipse.frame()` as its datum | - | only from tests | N/A (but the ORACLE states another curve than the evaluator for an arc-held ellipse) |
| 62 | end_face.rs :: `plan` [fragment] | reads `frame.axis()` only (`turn.dot(axis).hi() < 0.0`) | - | - | N/A (no centre) |
| 63 | end_face.rs :: `CapSliver::of` (`fn of`, ellipse arm) [fragment] | computes `center = carrier_frame.origin()` -> `CapSliver.center`, the point #14-#18 measure from | bare | only if a cut-off's round end curve is a cone-section ellipse [not traced; blend sections are expected centre-held] | N/A / not traced |

## (a) `Conic::unit_in_plane` (default: `self.unit_of(p - self.frame().origin())`)

Not overridden by `Ellipse` (the `impl Conic for Ellipse` block has no `unit_in_plane`; `&C` forwards). The module-wide
`#![allow(clippy::disallowed_methods)]` in conic.rs hides it from the gate. Three callers in `crates/`:

| caller | use | verdict |
|---|---|---|
| curve_pair/pose/window.rs `Window::foot_seed` | `(x, y) = self.b.unit_in_plane(q)` -> `Azimuth::of(y, x).principal_as(..)`: a SEED for `span_b.place(q, seed, band)` and `span_b.carrier().foot(q, seed)` (polished) | computes only - N/A (a seed u off in angle; `Ellipse::azimuth` would read it from the anchor) |
| topo/src/splitting/section_loops.rs `conics_clear` | row 10: decides inside/outside | SUSPECT |
| sweep/src/blend/open/end_face.rs `SectionFrame::unit` | `shape` is built by `geom::Ellipse::new(Frame::new(Point3::origin(), axis, u), major, minor)` in `CapSliver::of`: centre-held at the origin | N/A |

## (b) Inside `Window::holds_only` (window.rs)

`holds_only` returns true (the caller `transverse::near` then returns `Ok(None)`, i.e. drops the root: `Pose::Apart` or one crossing fewer)
when every piece of an arc no ring bounds reads `Read::Off`, and ringed arcs read off/receding. Four quantities come from a centre:

1. **`H`, a's height above b's plane** - `height_jet`: `centre = self.b.frame().origin()`, value `(p - centre).dot(n)`. Handed to
   `taylor(self.height_jet(m), [z0; 4], self.height_fourth(), w)` with ZERO error terms. `off_by` then certifies
   `positive("curve_pair_window_off", h_lo.max(clear - speed*w) - z)` - an exact `compare`, no band, `z = band.escalate()` on an unringed arc.
   For an arc-held `b` the true plane passes through the anchor; the derived centre sits off it by the component along `n` of the rounding of
   `anchor - u_ref*(a*c0) - v_ref*(b*s0)` (~u*a unless `u_ref`/`n` are coordinate axes). The window is exactly the arc where the TRUE `|H| <= e`
   (`Range::band_arc`), so once that component exceeds ~2e every piece reads `|H| > z` and the window is certified empty with the crossing in it.
   a = 1e6, eps = 1e-12 (e = 1e-11): 1e-10 > 2e-11. This is the quantity that does it.
2. **`clear`** in `off_by`: `self.span_b.carrier().clearance(self.a.point_at(m))` - (c) below; its `e.frame().height(q)` term is the same
   displaced height, so `clear >= |H_wrong|` certifies off by the same amount, independently of 1.
3. **The level `F`** (`level_on` -> `Level::jet_at`): `q = p - self.centre`, `centre = kb.frame().origin()`; `F = X^2/A^2 + Y^2/B^2 - 1` then carries
   ~2*delta/a relative, i.e. `d_lo`/`d_hi` are off by ~delta in metres. Feeds `recedes` (`Read::Recedes`) and the on reading `close`. Also zero error.
   `Level::jet_at` and `height_jet` further read a's own centre (`self.a.frame().origin() - p` as P''), which only enters the second-order term.
4. `foot_seed` (`unit_in_plane`): a seed only.

Charges carried: none for the centre. `taylor` gets `[z0; 4]`; `up::<N>` only lifts a nonnegative product past its own N roundings;
`positive` is `compare(row, x, 0)`. The module doc states the intent ("read locally ... uncharged at f64 ... each rounds as its few local terms
do"), which holds for a centre-held `b` and fails for an arc-held one because `centre` is itself a sum at the semi-axis scale.

## (c) `Curve3::clearance`, ellipse arm (on_carrier.rs)

`lower = ellipse_level_clearance(e.frame().in_plane(q), e.semi_axes()).abs(); (lower^2 + e.frame().height(q)^2).sqrt()`.
Both terms subtract the derived centre from `q`: for `q` on the arc near the anchor, `in_plane` is `(~a, small)` with error delta ~ u*a, the level
is ~2*delta/a and the bound it returns is ~delta; `height(q)` is the normal component of the same delta. So a point ON the arc can read a
"lower bound on its distance" of ~u*a (1e-10 m at a = 1e6, 1e-7 m at a = 1e9). No charge anywhere in the function.

Who decides on it (all bare `Margin::of`):
- `geom::Foot::on_carrier` (same file): `decide_reported("curve_carrier_clear", Margin::of(lower), band)` -> `OnCarrier::Off`. Reached after the
  foot's gap fails to read Zero, so it bites where the gap is in the band's gap (true distance between eps and K*eps) or the foot polish stalls:
  an escalation becomes a certified Off. Public: `Curve3::on_carrier(q, near, band)`, `Curve3::foot(..).on_carrier(band)`.
- `geom::Curve3::distance_within` reads it for circles only (N/A).
- geom-brep `curve_pair/pose/window.rs::Window::off_by` -> `positive("curve_pair_window_off", ..)` ((b) item 2).
- geom-brep `curve_pair/pose.rs::clearance` -> `Polish.clear` in `line_conic` (crossing arm) and `transverse::near` [`Polish::closest` not read].
- geom-brep `curve_pair/span.rs::past_by`: `decide("curve_pair_span_run_end", Margin::of(clear - bend - bend), band) == Positive` -> the end is skipped.
- topo `solid_contain/curved_edges.rs::passes`: `decide("curve_carrier_clear", Margin::of(lower), band) == Positive` -> `Passes::Clear`.
- topo `link.rs::clearance` returns it for a curved edge [its decider not read].
(`topo/src/link/cleared_rows.rs` and `census/pierce_rows.rs` are tests.)

## Doors and exposing poses for the SUSPECT rows

Common fixture E: the ellipse `plane_cone_section` mints for a shallow cut with a ~ 1e6 m (1e9 for the eps = 1e-9 band), in a generic
orientation (plane normal and seam not coordinate axes, so the derived centre's rounding has a component in every direction).

| # | cheapest public door | pose |
|---|---|---|
| 1 | `geom_brep::curve_pair::meet(line span, E span, band)` | (ii) a 1 m line in E's plane through a point of the arc 0.3 m from the anchor, a = 1e6, eps = 1e-12: `height` reads ~1e-10 > K*eps -> `Route::Parallel` (already measured). (i) a line along E's major axis whose span ends on the anchor vertex from inside: the GAP row reads the conic's extent `c + speed_hi` 1e-10 short -> `Route::Apart` |
| 2 | `geom_brep::curve_face::cross(E span, Surface::Plane, band)` -> `plane_conic` | E's own plane stated through its anchor (normal = E's axis), a = 1e6, eps = 1e-12: r = 0 and `d0` reads ~1e-10, so `r - |d0|` is decided negative -> `Pose::Apart` where the truth is `Lies` |
| 3 | `curve_pair::meet(E1 span, E2 span, band)` | two section ellipses of one cone in parallel planes 0 apart / a circle in E's plane crossing the arc near the anchor: `c0` reads ~1e-10 with swing ~0 -> `constant_as(PARALLEL)` true -> `Pose::Apart` |
| 4-7 | `curve_pair::meet(conic a, E as b, band)`, transverse arm | a unit circle piercing E's plane on the arc 0.3 m from the anchor, a = 1e6, eps = 1e-12: `holds_only` reads `|H|` ~1e-10 across the whole band arc and returns true (measured). For #6: an ellipse coplanar with E touching the arc near the anchor from outside by < 1e-10: the level's zero set shifts by delta, a touch reads apart or two roots |
| 8 | `geom::Curve3::on_carrier(q, near, band)` | q 5e-9 m off E's arc near the anchor, a = 1e9, eps = 1e-9 (gap in the band's gap): `clearance` reads ~1e-7 -> `OnCarrier::Off` where the answer is an escalation; at a = 1e6, eps = 1e-12 the same with q 5e-12 off |
| 9 | topo splitting, `finish.rs` -> `section_loops::outlines_disjoint` (via a plane split of a body whose section holds an ellipse loop and a polygon loop) | a segment whose line is tangent to E's carrier within 1e-8 m, a = 1e9, eps = 1e-9: `m.dot(centre - p) - reach` reads +1e-7 -> separated |
| 10 | same door | a small circle (r = 1 m) lying across E's arc near the anchor, the pair read with the circle as `e`: `h.frame().origin()` error u*a_h enters `cx` unscaled by a_h, x lever 1 m -> `outside` reads positive by ~1e-7 at a = 1e9 |
| 11 | the mass-properties door over a body with a cone face cut by a tilted plane (`require_cone_ellipse` in props/curved.rs; public entry not traced) | a = 1e9, eps = 1e-9: `elevation` reads ~1e-7 for an ellipse exactly on the cone -> `PropsError::OffSurface`; or a section whose lowest point is 5e-9 from the apex height reads `h_low` > K*eps |
| 12 | the offset / replace-face door that reaches `offset_charts_staged` (public entry not traced) | a cone face bounded by E whose nearest point to the apex is within 1e-8 m of the apex plane, a = 1e9: `v_near = c - amp` reads the wrong sign past the band |
| 13 | `geom_brep::curve_pair::on_surface::OnSurface::under(curve, span, endless, band)` | a surface curve (line) lying exactly in E's plane: read against the plane through the derived centre it is 1e-10 off and parallel -> `CoSurface::Roots([])` instead of `InPlane`, eps = 1e-12 |
| 14-18 | sweep blend surgery `ring_clearance_pass` (public blend entry not traced) | needs an end-face edge that is a cone-section ellipse whose major vertex sits on the sliver; not constructed |
| 19 | `pcurve_cache` incidence (`incidence_over_span_as`), through whatever derives E's pcurve on a plane chart | E on its own plane, plane anchor near the arc, a = 1e9: the moved carrier's anchor is displaced ~2e-7 while `error` holds only the offset's bound; not measured |
| 20-22 | not traced to a door | - |

## Not read / limits

- Consumers marked [nc]; rows 23, 42, 45-49 have no verdict beyond "not traced".
- `Polish::closest` (what `Polish.clear`/`reach` decide), `plane_roots`, `group_cone_v_window`, the cover `Arc` readers other than
  `support`/`range`/`ball`, `EnclosingNet`, the pcurve envelope, and every caller above the named function were not read.
- The gate's own stated blind spots apply: a second read inside a listed function, a centre restated from `Curve3::data()`, a read through a
  `Frame` handed on (e.g. `geom::cover::Arc.frame`, `window.rs` tests), and `Conic::center`/`unit_in_plane`/`probe_stored` defaults inside conic.rs.
- Not gate-listed but found deciding on a listed function's result: `geom-brep/src/curve_face/pose.rs::plane_conic` (row 2).
