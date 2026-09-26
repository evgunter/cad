//! **The must-carry rule over an edge, on both sweep verbs.**
//!
//! `geom_brep::must_carry_over_edge` is the one home of the rule a
//! constructor applies to a definitely-smooth join: gate on the jet
//! certificate's lane, read the certification schedule's interior
//! stations — each first-order before second-order — and answer
//! jet-determinate (store the intrinsic `TangentIntersection`),
//! under-determined (store the conventional chart image), in-band
//! (refuse TYPED) or transverse (the join is a corner, not the smooth
//! join the caller took it for). These rows hold
//! the extrude strut arm and the revolve latitude join to that ONE
//! answer, on the same one-parameter family of geometry, so a verb
//! that grows its own policy again shows up here as a disagreement
//! between two otherwise identical rows.
//!
//! # The family, and why its parameter is derived from the band
//!
//! Both fixtures are a tangent join between a surface whose meridian
//! is straight (a plane wall, a cylinder bore) and one whose meridian
//! is a circle of radius `r`, so the relative transverse normal
//! curvature is `|κ_rel| = 1/r` exactly. The rule's margin is the
//! sagitta that curvature subtends over the pair's folded lever arm,
//! `|κ_rel|·arm²/2`, classified against the run's linear band
//! `(ε, K·ε)`.
//!
//! A row cannot pin a radius: CI gates three ε rows (the default,
//! `1e-6` and `1e-12`), and a margin that is in-band at one of them is
//! definite at the others. So each row names the MARGIN it wants, and
//! the fixture's free length is derived from the resolved band — the
//! inverse of the sagitta formula. What is fixed in both fixtures is
//! `r = 0.25`; the free length is the one that reaches the fold as the
//! arm, so `arm = free` and `margin = free²/(2r) = 2·free²`, giving
//! `free = √(margin/2)`:
//!
//! - **extrude** — a unit block with `r = 0.25` fillets, extruded to a
//!   height `h`. The strut's extent IS `h`, and the fold is
//!   `min(∞, r, h)`, so `h < r` makes the height the arm.
//! - **revolve** — a bore cylinder of radius `R` meeting, tangentially
//!   at its inner equator, a torus of minor radius `r` and major
//!   radius `R + r`. The fold is `min(R, r, 2R)`, so `R < r` makes the
//!   bore radius the arm.
//!
//! The three rows per verb are then one geometry at three lengths:
//! `margin = 0.125` (definitely positive at every ε the matrix runs),
//! `margin = √(ε·Kε)` (the band's geometric mean — in-band at every ε),
//! `margin = ε/100` (definitely zero at every ε).
//!
//! The stored-description differential these rows are read beside lives
//! in `bitdump.rs`, which is the one home for a dump taken at two SHAs
//! and diffed; nothing here writes one.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{EdgeDescription, MustCarryVerdict, SurfaceKind, must_carry_over_edge};
use geom_core::{Band, MarginDiag, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, RawLoop, SketchPlane, test_support::bulge_loop};
use sweep::{ExtrudeError, Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::Body;

/// The run's resolved linear band — the same one both verbs classify
/// against, so a row's derived length is the one the kernel will read.
fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's linear band")
}

/// The margin every row in this file is built around: the meridian
/// circle's radius, shared by both fixtures so the two verbs' rows are
/// the same geometry twice.
const MERIDIAN_R: f64 = 0.25;

/// The free length that subtends `margin` as a sagitta over itself:
/// `margin = |κ_rel|·arm²/2` with `|κ_rel| = 1/MERIDIAN_R` and
/// `arm = free`, inverted. Valid while `free < MERIDIAN_R`, which is
/// what keeps the free length the smallest term of the fold.
fn free_length_for(margin: f64) -> f64 {
    (margin * MERIDIAN_R * 2.0).sqrt()
}

/// A margin definitely on the positive side at every ε the matrix
/// gates: the free length saturates at the meridian radius, so this is
/// the largest margin the family reaches.
fn definite_positive_margin() -> f64 {
    MERIDIAN_R / 2.0
}

/// The band's geometric mean — strictly inside `(ε, K·ε)` for every
/// `K > 1`, so this margin is in-band at every ε row.
fn in_band_margin() -> f64 {
    (band().zero() * band().escalate()).sqrt()
}

/// A margin two decades under ε: definitely coincident with zero at
/// every ε row.
fn definite_zero_margin() -> f64 {
    band().zero() / 100.0
}

// ---------------------------------------------------------------
// The extrude fixture: a filleted block, extruded to a chosen height.
// ---------------------------------------------------------------

/// A unit square with `MERIDIAN_R` fillets at all four corners,
/// extruded `h` along `+z`. Every fillet contributes two tangent
/// line–arc joins, so the body has eight smooth struts and nothing
/// else that is smooth; each wall pair is plane–cylinder on a `Line`
/// carrier, which is inside the jet certificate's lane.
fn filleted_block(h: f64) -> Result<Body<f64>, ExtrudeError> {
    let p2 = Point2::<f64>::new;
    let q = MERIDIAN_R;
    // A quarter arc: bulge = tan(θ/4) at θ = π/2.
    let b = (core::f64::consts::FRAC_PI_8).tan();
    let lp = bulge_loop(vec![
        (p2(q, 0.0), 0.0),
        (p2(1.0 - q, 0.0), b),
        (p2(1.0, q), 0.0),
        (p2(1.0, 1.0 - q), b),
        (p2(1.0 - q, 1.0), 0.0),
        (p2(q, 1.0), b),
        (p2(0.0, 1.0 - q), 0.0),
        (p2(0.0, q), b),
    ])
    .with_tangent_joints(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the filleted block is a valid profile");
    extrude(&profile, Extrusion::Distance(h), Tol::witness()).map(|e| e.body)
}

/// The eight smooth struts of [`filleted_block`] — the count is the
/// fixture's own structure, asserted rather than assumed so a profile
/// change that loses a fillet cannot quietly empty a row.
const SMOOTH_STRUTS: usize = 8;

// ---------------------------------------------------------------
// The revolve fixture: a bore cylinder tangent to a torus's inner
// equator.
// ---------------------------------------------------------------

/// A ring revolved about the sketch y-axis whose bore cylinder of
/// radius `r_bore` meets a torus of minor radius `MERIDIAN_R` — major
/// radius `r_bore + MERIDIAN_R` — tangentially at the torus's INNER
/// equator, the one latitude circle of the body where two walls join
/// smoothly.
///
/// The profile runs counterclockwise: the base annulus, the outer
/// cylinder, a 135° arc over the tube, then straight down the bore to
/// the start. Only the arc's far end is a tangent joint; the other
/// three vertices are corners, so the body has exactly one smooth
/// latitude join.
fn bored_ring(r_bore: f64) -> Result<Body<f64>, sweep::RevolveError> {
    let r = MERIDIAN_R;
    let h = 0.3;
    // The arc runs from 45° to 180° about the tube centre `(r_bore + r, 0)`.
    let shoulder = Point2::new(
        r_bore + r + r * core::f64::consts::FRAC_1_SQRT_2,
        r * core::f64::consts::FRAC_1_SQRT_2,
    );
    let outer = shoulder.x;
    let bulge = (3.0 * core::f64::consts::FRAC_PI_4 / 4.0).tan();
    let lp = bulge_loop(vec![
        (Point2::new(r_bore, -h), 0.0),
        (Point2::new(outer, -h), 0.0),
        (shoulder, bulge),
        (Point2::new(r_bore, 0.0), 0.0),
    ])
    .with_tangent_joints(vec![3]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the bored ring is a valid profile");
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&profile, axis, Revolution::Full, Tol::witness()).map(|r| r.body)
}

/// The one smooth latitude join of [`bored_ring`].
const SMOOTH_LATITUDE_JOINS: usize = 1;

// ---------------------------------------------------------------
// Reading a body back.
// ---------------------------------------------------------------

/// How many of `body`'s edges store the intrinsic tangency.
fn tangent_intersections(body: &Body<f64>) -> usize {
    body.edges()
        .filter(|(_, e)| {
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(geom_brep::EdgeCurve::description),
                Some(EdgeDescription::TangentIntersection { .. })
            )
        })
        .count()
}

/// The conventional chart images among the edges `pick` selects — a
/// COUNT OVER A NAMED SET, never over the whole body: a body's cap
/// rims are chart images too, so `chart_images(body) >= n` is met by
/// edges that have nothing to do with the rule and rises on its own
/// whenever a fixture grows.
fn chart_images_among(body: &Body<f64>, pick: impl Fn(&Curve3<f64>) -> bool) -> (usize, usize) {
    let mut selected = 0usize;
    let mut charts = 0usize;
    for (_, e) in body.edges() {
        let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        if !pick(c.carrier()) {
            continue;
        }
        selected += 1;
        if matches!(c.description(), EdgeDescription::Chart(_)) {
            charts += 1;
        }
    }
    (selected, charts)
}

/// The filleted block's STRUTS: the only edges whose carrier is a line
/// along the extrusion direction (`+z`). Its cap rims are the profile
/// segments, which lie in the sketch plane.
fn is_strut(c: &Curve3<f64>) -> bool {
    matches!(c, Curve3::Line { dir, .. } if dir.z.abs() > 0.5)
}

/// The bored ring's smooth latitude join: the one circle at the bore
/// radius. Every other latitude circle of the fixture is wider.
fn is_bore_circle(r_bore: f64) -> impl Fn(&Curve3<f64>) -> bool {
    move |c| matches!(c, Curve3::Circle { radius, .. } if (radius - r_bore).abs() < r_bore * 1e-9)
}

/// The typed escalation an in-band row must carry: the predicate name
/// the rule meters under, and a margin that really is inside the band
/// the row derived its geometry from.
fn assert_in_band_payload(source: geom_core::Indeterminate) {
    assert_eq!(
        source.predicate,
        Some("tangent_second_order"),
        "the escalation must name the must-carry rule's one metered predicate"
    );
    let b = band();
    match source.margin {
        MarginDiag::Value(m) => assert!(
            m.abs() > b.zero() && m.abs() < b.escalate(),
            "the row's derived geometry must put the margin strictly inside \
             the band: margin {m:e} against ({:e}, {:e})",
            b.zero(),
            b.escalate()
        ),
        other => panic!("the in-band row's margin should be a value, not {other:?}"),
    }
}

// ---------------------------------------------------------------
// The trio, on the extrude strut.
// ---------------------------------------------------------------

/// Definitely positive: the surfaces determine the locus along the
/// whole strut, so prefer-intrinsic demands `TangentIntersection` on
/// every one of the block's eight smooth struts.
#[test]
fn an_extrude_strut_with_a_definite_positive_margin_stores_the_intrinsic_tangency() {
    let body = filleted_block(free_length_for(definite_positive_margin()))
        .expect("a definitely-determinate join builds");
    assert_eq!(
        tangent_intersections(&body),
        SMOOTH_STRUTS,
        "every smooth strut stores the intrinsic tangency"
    );
}

/// Definitely zero: the surfaces under-determine the locus, so each
/// strut keeps the conventional description — an image at rest in the
/// previous wall's chart — and NO strut stores the intrinsic tangency.
#[test]
fn an_extrude_strut_with_a_definite_zero_margin_stores_the_conventional_description() {
    let body = filleted_block(free_length_for(definite_zero_margin()))
        .expect("an under-determined join builds");
    assert_eq!(
        tangent_intersections(&body),
        0,
        "an under-determined join stores no intrinsic tangency"
    );
    let (struts, charts) = chart_images_among(&body, is_strut);
    assert_eq!(
        struts, SMOOTH_STRUTS,
        "the fixture's eight struts are the edges this row is about"
    );
    assert_eq!(
        charts, SMOOTH_STRUTS,
        "every under-determined STRUT rests in a chart — not merely some edge of the body"
    );
}

/// In-band: certifiable as neither, so the strut arm refuses typed.
/// This is the contract half the revolve arm used to fold away.
#[test]
fn an_extrude_strut_with_an_in_band_margin_refuses_typed() {
    match filleted_block(free_length_for(in_band_margin())) {
        Err(ExtrudeError::SliverJoin { source, .. }) => assert_in_band_payload(source),
        Err(other) => panic!("the in-band strut must refuse as a sliver JOIN, not {other}"),
        Ok(_) => panic!("an in-band second-order margin was built silently"),
    }
}

// ---------------------------------------------------------------
// The trio, on the revolve latitude join.
// ---------------------------------------------------------------

/// The revolve twin of the positive row: the bore–torus join is
/// jet-determinate, so the latitude circle stores `TangentIntersection`.
#[test]
fn a_revolve_latitude_join_with_a_definite_positive_margin_stores_the_intrinsic_tangency() {
    let body = bored_ring(free_length_for(definite_positive_margin()))
        .expect("a definitely-determinate join builds");
    assert_eq!(
        tangent_intersections(&body),
        SMOOTH_LATITUDE_JOINS,
        "the smooth latitude join stores the intrinsic tangency"
    );
}

/// The revolve twin of the zero row.
#[test]
fn a_revolve_latitude_join_with_a_definite_zero_margin_stores_the_conventional_description() {
    let r_bore = free_length_for(definite_zero_margin());
    let body = bored_ring(r_bore).expect("an under-determined join builds");
    assert_eq!(
        tangent_intersections(&body),
        0,
        "an under-determined join stores no intrinsic tangency"
    );
    let (bore_circles, charts) = chart_images_among(&body, is_bore_circle(r_bore));
    assert_eq!(
        bore_circles, 2,
        "the bore cylinder's two latitude circles — the smooth join and the base rim"
    );
    assert_eq!(
        charts, SMOOTH_LATITUDE_JOINS,
        "the smooth latitude JOIN rests in a chart; the base rim is transverse and \
         stores the plain intersection"
    );
}

/// **The unit's behaviour change.** An in-band latitude join refuses
/// `RevolveError::SliverJoin` where the verb used to keep the
/// conventional description and build — the same answer the extrude
/// strut gives on the same geometry.
#[test]
fn a_revolve_latitude_join_with_an_in_band_margin_refuses_typed() {
    match bored_ring(free_length_for(in_band_margin())) {
        Err(sweep::RevolveError::SliverJoin { source, .. }) => assert_in_band_payload(source),
        Err(other) => panic!("the in-band latitude join must refuse as a sliver JOIN, not {other}"),
        Ok(_) => panic!("an in-band second-order margin was built silently"),
    }
}

// ---------------------------------------------------------------
// The lane gate.
// ---------------------------------------------------------------

/// An out-of-lane pair answers "conventional" WITHOUT metering: the
/// certificate cannot store an intrinsic tangency on a carrier/surface
/// triple it refuses to bound, so a jet reading there decides nothing.
/// The pair below is second-order definite if it is metered — a cone
/// against its tangent plane along a ruling — and the gate is what
/// keeps the answer under-determined anyway.
#[test]
fn an_out_of_lane_pair_is_under_determined() {
    let (plane, cone, carrier) = out_of_lane_triple();
    assert!(
        !geom_brep::tangent_certificate_lane(&carrier, &plane, &cone),
        "a Line carrier on a cone pair is outside the certificate's lane"
    );
    let answer = must_carry_over_edge(&plane, &cone, &carrier, 0.0, 1.0, 1.0, band());
    assert_eq!(
        answer,
        MustCarryVerdict::UnderDetermined,
        "an out-of-lane pair answers conventional"
    );
    // That nothing was METERED is the K-stream row's claim, which
    // counts samples rather than reading a field the answer no longer
    // carries.
}

/// A cone tangent to a plane along one ruling, and that ruling as the
/// carrier. The tangency is exact, so a metered reading would be
/// definite; the lane gate is the only thing that answers.
fn out_of_lane_triple() -> (Surface<f64>, Surface<f64>, Curve3<f64>) {
    let half = core::f64::consts::FRAC_PI_4;
    let cone = Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        half_angle: half,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    // The ruling in the x = z half-plane, and the plane containing it
    // and the cone's tangent plane's other direction (ŷ).
    let dir = Vec3::new(half.sin(), 0.0, half.cos());
    let normal = Vec3::new(half.cos(), 0.0, -half.sin());
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let carrier = Curve3::Line {
        origin: Point3::new(dir.x, 0.0, dir.z),
        dir,
    };
    (plane, cone, carrier)
}

// ---------------------------------------------------------------
// The first-order gate, in both surface orders.
// ---------------------------------------------------------------

/// The rule's verdict on one pair asked in BOTH argument orders, with
/// the carrier, interval and extent held fixed.
fn both_orders(
    s1: &Surface<f64>,
    s2: &Surface<f64>,
    carrier: &Curve3<f64>,
    t1: f64,
    extent: f64,
) -> (MustCarryVerdict, MustCarryVerdict) {
    (
        must_carry_over_edge(s1, s2, carrier, 0.0, t1, extent, band()),
        must_carry_over_edge(s2, s1, carrier, 0.0, t1, extent, band()),
    )
}

/// A cylinder resting in the plane `y = 0`, tangent along its ruling
/// through the origin, and that ruling as the carrier.
fn resting_cylinder() -> (Surface<f64>, Surface<f64>, Curve3<f64>) {
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let cylinder = Surface::Cylinder {
        origin: Point3::new(0.0, MERIDIAN_R, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: MERIDIAN_R,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let ruling = Curve3::Line {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    };
    (plane, cylinder, ruling)
}

/// **One geometric fact, one answer, whatever the argument order.**
///
/// A plane crossing a cylinder at a right angle along a circle — the
/// ruled band's cut-off arc against its cap — is a corner, and the rule
/// says so in both orders. Its second-order jet read in either order
/// is an artefact of the order (`κ_rel = 1/r` along the cylinder's own
/// normal with the plane first, `0` along the flat ruling with the
/// cylinder first), and without the first-order gate the two orders
/// answered jet-determinate and under-determined. The same holds one
/// band down: planes crossing at a sliver angle are in-band in both
/// orders, never the conventional answer.
///
/// And the gate moves nothing where the jet IS the question: a genuine
/// tangency reads jet-determinate in both orders, and the same tangency
/// over an arm too short to subtend ε reads under-determined in both.
#[test]
fn the_rule_answers_one_pair_the_same_way_in_both_surface_orders() {
    // The right-angle crossing: the plane z = 0 and the cylinder about
    // the z axis, along their quarter circle of intersection. A Circle
    // carrier on a plane/cylinder pair is inside the lane, so this pair
    // reaches the stations rather than the lane gate.
    let cap = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let band_wall = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: MERIDIAN_R,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let arc = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: MERIDIAN_R,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    assert!(
        geom_brep::tangent_certificate_lane(&arc, &cap, &band_wall)
            && geom_brep::tangent_certificate_lane(&arc, &band_wall, &cap),
        "the crossing is inside the lane in both orders, so the stations decide"
    );
    assert_eq!(
        both_orders(
            &cap,
            &band_wall,
            &arc,
            core::f64::consts::FRAC_PI_2,
            MERIDIAN_R
        ),
        (MustCarryVerdict::Transverse, MustCarryVerdict::Transverse),
        "a right-angle crossing is a corner in both surface orders"
    );

    // The sliver crossing: two planes through the z axis at an angle
    // whose wedge margin `sin θ · extent` is the band's geometric mean.
    let extent = MERIDIAN_R;
    let sin_theta = in_band_margin() / extent;
    let cos_theta = (1.0 - sin_theta * sin_theta).sqrt();
    let flat = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let tilted = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(-sin_theta, cos_theta, 0.0),
        u_ref: Vec3::new(cos_theta, sin_theta, 0.0),
    };
    let axis_line = Curve3::Line {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(0.0, 0.0, 1.0),
    };
    let (a, b) = both_orders(&flat, &tilted, &axis_line, extent, extent);
    for (order, verdict) in [("flat first", a), ("tilted first", b)] {
        match verdict {
            MustCarryVerdict::InBand(source) => assert_eq!(
                source.predicate,
                Some("dihedral_wedge"),
                "{order}: the sliver is the first-order wedge's escalation"
            ),
            other => panic!("{order}: a sliver crossing must escalate in band, not {other:?}"),
        }
    }

    // The genuine tangency, determinate and under-determined.
    let (plane, cylinder, ruling) = resting_cylinder();
    assert_eq!(
        both_orders(&plane, &cylinder, &ruling, MERIDIAN_R, MERIDIAN_R),
        (
            MustCarryVerdict::JetDeterminate,
            MustCarryVerdict::JetDeterminate
        ),
        "a cylinder on its tangent plane determines the locus in both orders"
    );
    let short = free_length_for(definite_zero_margin());
    assert_eq!(
        both_orders(&plane, &cylinder, &ruling, short, short),
        (
            MustCarryVerdict::UnderDetermined,
            MustCarryVerdict::UnderDetermined
        ),
        "the same tangency over a sub-ε sagitta is under-determined in both orders"
    );
}

/// **The lane gate over what the two verbs actually mint.** The lane
/// TABLE — which (carrier kind, surface-kind pair) triples
/// `tangent_certificate_lane` admits — is pinned exhaustively over
/// `SurfaceKind` by
/// `review_must_carry_rule_r2_probes::the_lane_census_is_exhaustive_over_surface_kind`,
/// which stops compiling when a kind is added. This row pins the other
/// half, and only that half: that every edge of the bodies these two
/// fixtures BUILD presents the rule a triple the table admits.
///
/// **What it does not pin, stated rather than implied.** It reads the
/// verbs' output, not the verbs' source, so it is evidence about these
/// fixtures and not a theorem about the verbs: a door that minted a
/// `Nurbs` wall on some OTHER profile would reach the rule out of lane
/// and leave this row green. Making that mechanical needs a hook into
/// the `FaceSurface::New(Surface::…)` mint sites that no test has.
#[test]
fn every_edge_the_two_fixtures_mint_presents_the_rule_a_lane_admitted_triple() {
    for (name, body) in [
        (
            "the filleted block",
            filleted_block(free_length_for(definite_positive_margin())).expect("the block builds"),
        ),
        (
            "the bored ring",
            bored_ring(free_length_for(definite_positive_margin())).expect("the ring builds"),
        ),
    ] {
        let surface_of = |he| {
            let face = body
                .get_loop(body.get_half_edge(he).expect("a half-edge").parent_loop)
                .expect("a loop")
                .face;
            body.get_surface(body.get_face(face).expect("a face").surface)
                .expect("a surface")
                .clone()
        };
        let mut checked = 0usize;
        for (k, e) in body.edges() {
            let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
                continue;
            };
            let (a, b) = (surface_of(e.he_plus), surface_of(e.he_minus));
            checked += 1;
            assert!(
                geom_brep::tangent_certificate_lane(c.carrier(), &a, &b),
                "{name}: edge {k:?} presents the rule a triple the certificate's lane \
                 refuses — surfaces {:?} / {:?}",
                SurfaceKind::of(&a),
                SurfaceKind::of(&b)
            );
        }
        assert!(
            checked > 0,
            "{name}: the fixture has certified edges to read"
        );
    }
}

// ---------------------------------------------------------------
// The K stream.
// ---------------------------------------------------------------

/// **What the rule itself costs the K stream**, isolated from every
/// other site that meters the same predicate: a jet-determinate pair
/// spends `CERT_SAMPLES − 2` samples of `tangent_second_order`, an
/// under-determined one spends the deciding station's index — one here,
/// where the first station decides (the walk exits there) — and an
/// out-of-lane one spends none (the gate answers
/// before any station is read).
#[cfg(feature = "probe")]
#[test]
fn the_rule_meters_the_schedules_interior_stations_and_the_gate_meters_nothing() {
    use geom_core::k_stats::{self, Probe};
    let interior = usize::try_from(geom_brep::CERT_SAMPLES - 2).expect("a small count");
    let count = |samples: Vec<geom_core::k_stats::MarginSample>| {
        samples
            .iter()
            .filter(|s| s.predicate == "tangent_second_order")
            .count()
    };

    // A cylinder resting in a plane, tangent along the ruling that is
    // also the carrier: in the lane, and definitely separated.
    let pr = |x: f64| Probe(x);
    let cylinder = Surface::Cylinder {
        origin: Point3::new(pr(0.0), pr(MERIDIAN_R), pr(0.0)),
        axis: Vec3::new(pr(0.0), pr(0.0), pr(1.0)),
        radius: pr(MERIDIAN_R),
        u_ref: Vec3::new(pr(1.0), pr(0.0), pr(0.0)),
    };
    let plane = Surface::Plane {
        origin: Point3::new(pr(0.0), pr(0.0), pr(0.0)),
        normal: Vec3::new(pr(0.0), pr(1.0), pr(0.0)),
        u_ref: Vec3::new(pr(1.0), pr(0.0), pr(0.0)),
    };
    let ruling = Curve3::Line {
        origin: Point3::new(pr(0.0), pr(0.0), pr(0.0)),
        dir: Vec3::new(pr(0.0), pr(0.0), pr(1.0)),
    };
    k_stats::start_recording();
    let determinate = must_carry_over_edge(
        &plane,
        &cylinder,
        &ruling,
        pr(0.0),
        pr(MERIDIAN_R),
        pr(MERIDIAN_R),
        band(),
    );
    let spent = count(k_stats::take_samples());
    assert_eq!(
        determinate,
        MustCarryVerdict::JetDeterminate,
        "a cylinder on its tangent plane determines the locus along the ruling"
    );
    assert_eq!(
        spent, interior,
        "a jet-determinate join reads every interior station of the schedule"
    );

    // The same pair with the arm shrunk until the sagitta is
    // definitely zero: the walk decides at the first station.
    let short = free_length_for(definite_zero_margin());
    k_stats::start_recording();
    let under = must_carry_over_edge(
        &plane,
        &cylinder,
        &ruling,
        pr(0.0),
        pr(short),
        pr(short),
        band(),
    );
    let spent_under = count(k_stats::take_samples());
    assert_eq!(
        under,
        MustCarryVerdict::UnderDetermined,
        "the derived arm puts the sagitta under ε"
    );
    assert_eq!(
        spent_under, 1,
        "the first station decides an under-determined join, and the walk stops there"
    );

    // The gate spends nothing: an out-of-lane pair is answered before
    // any station is read.
    let (plane, cone, carrier) = out_of_lane_triple();
    let plane = probe_surface(&plane);
    let cone = probe_surface(&cone);
    let carrier = probe_carrier(&carrier);
    k_stats::start_recording();
    let answer = must_carry_over_edge(
        &plane,
        &cone,
        &carrier,
        Probe(0.0),
        Probe(1.0),
        Probe(1.0),
        band(),
    );
    let after = k_stats::take_samples();
    assert_eq!(
        answer,
        MustCarryVerdict::UnderDetermined,
        "the out-of-lane pair is conventional"
    );
    assert!(
        after.is_empty(),
        "an out-of-lane pair emits no K sample: {after:?}"
    );
}

/// **What the rule costs a whole extruded body**, so the PR's K-stream
/// claim is a measurement and not an extrapolation: the block's eight
/// jet-determinate struts each spend `CERT_SAMPLES − 2` in the rule,
/// and the certification door then re-asks the SAME question at the
/// SAME stations for each stored `TangentIntersection` — that second
/// half is the certificate's, unchanged by this unit, and it is what
/// keeps the stored set and the certified set one set.
#[cfg(feature = "probe")]
#[test]
fn a_filleted_block_spends_the_rules_stations_once_per_smooth_strut() {
    use geom_core::k_stats::{self, Probe};
    let interior = usize::try_from(geom_brep::CERT_SAMPLES - 2).expect("a small count");
    let zero = Probe(0.0);
    let b = Probe(core::f64::consts::FRAC_PI_8.tan());
    let p2 = |x: f64, y: f64| Point2::new(Probe(x), Probe(y));
    let q = MERIDIAN_R;
    let lp = bulge_loop::<Probe>(vec![
        (p2(q, 0.0), zero),
        (p2(1.0 - q, 0.0), b),
        (p2(1.0, q), zero),
        (p2(1.0, 1.0 - q), b),
        (p2(1.0 - q, 1.0), zero),
        (p2(q, 1.0), b),
        (p2(0.0, 1.0 - q), zero),
        (p2(0.0, q), b),
    ])
    .with_tangent_joints(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the filleted block is a valid profile");
    k_stats::start_recording();
    let body = extrude(&profile, Extrusion::Distance(Probe(q)), Tol::witness())
        .expect("the block extrudes")
        .body;
    let metered = k_stats::take_samples()
        .iter()
        .filter(|s| s.predicate == "tangent_second_order")
        .count();
    assert_eq!(
        body.edges()
            .filter(|(_, e)| matches!(
                body.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(geom_brep::EdgeCurve::description),
                Some(EdgeDescription::TangentIntersection { .. })
            ))
            .count(),
        SMOOTH_STRUTS,
        "the block's eight struts store the intrinsic tangency"
    );
    assert_eq!(
        metered,
        2 * SMOOTH_STRUTS * interior,
        "each strut spends {interior} stations in the rule and {interior} more in \
         the certificate that accepts what the rule decided"
    );
}

/// The probe twin of a surface — the K-stream rows build the same
/// geometry at the recording scalar.
#[cfg(feature = "probe")]
fn probe_surface(s: &Surface<f64>) -> Surface<geom_core::k_stats::Probe> {
    use geom_core::k_stats::Probe;
    match *s {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => Surface::Plane {
            origin: origin.map(Probe),
            normal: normal.map(Probe),
            u_ref: u_ref.map(Probe),
        },
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => Surface::Cone {
            apex: apex.map(Probe),
            axis: axis.map(Probe),
            half_angle: Probe(half_angle),
            u_ref: u_ref.map(Probe),
        },
        _ => panic!("only the out-of-lane triple's kinds are lifted here"),
    }
}

/// The probe twin of a line carrier.
#[cfg(feature = "probe")]
fn probe_carrier(c: &Curve3<f64>) -> Curve3<geom_core::k_stats::Probe> {
    use geom_core::k_stats::Probe;
    match *c {
        Curve3::Line { origin, dir } => Curve3::Line {
            origin: origin.map(Probe),
            dir: dir.map(Probe),
        },
        _ => panic!("only the out-of-lane triple's carrier is lifted here"),
    }
}
