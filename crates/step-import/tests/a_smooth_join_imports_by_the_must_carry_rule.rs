//! **A definitely-smooth join imports as the must-carry rule says it
//! stores**: the rule over the two surfaces (`geom_brep::reask_pair`)
//! decides what the build's tangency is. A jet-determinate join adopts
//! the intrinsic tangency, built as the pair's own tangent circle; an
//! under-determined one, which a native body stores as the conventional
//! pair, has no construction of its own and refuses not-yet-supported,
//! naming the lane; an in-band one refuses typed through the rule; and a
//! join the file puts off its surfaces refuses at interpretation.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom_brep::EdgeDescription;
use geom_core::{Band, Point2, Point3, Tol, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, SketchPlane};
use step_import::{AdoptionCandidate, ImportOptions, StepImport, StepImportError};
use sweep::{ExtrudeSide, Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{AtRestBody, Body};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("the run's band forms")
}

fn write(body: &AtRestBody<f64>) -> String {
    step_export::step_string(body, &step_export::StepOptions::default(), tol())
        .expect("a finished body exports")
}

fn import(text: &str) -> Result<AtRestBody<f64>, StepImportError> {
    step_import::import_step(text, &ImportOptions::default(), tol()).map(|got| match got {
        StepImport::Solid { body, .. } => body,
        StepImport::Wireframe { .. } => panic!("a solid's file imports a solid"),
    })
}

/// The description kind of every edge whose carrier `pick` selects,
/// sorted: a census of what a body stores over a named set.
fn kinds(body: &Body<f64>, pick: impl Fn(&Curve3<f64>) -> bool) -> Vec<&'static str> {
    let mut kinds: Vec<_> = body
        .edges()
        .filter_map(|(_, e)| body.get_curve_geom(e.curve).and_then(|g| g.certified()))
        .filter(|c| pick(c.carrier()))
        .map(|c| match c.description() {
            EdgeDescription::Intersection { .. } => "Intersection",
            EdgeDescription::TangentIntersection { .. } => "TangentIntersection",
            EdgeDescription::OnPair { .. } => "OnPair",
            EdgeDescription::Chart(_) => "Chart",
            EdgeDescription::Scaffold(_) => "Scaffold",
        })
        .collect();
    kinds.sort_unstable();
    kinds
}

/// Writes `body`, reads it back, and asserts the import ships tier-3
/// valid with the native body's descriptions over the edges `pick`
/// selects, which must include `want`.
fn round_trips(label: &str, body: &AtRestBody<f64>, pick: impl Fn(&Curve3<f64>) -> bool + Copy) {
    let native = kinds(body, pick);
    let imported =
        import(&write(body)).unwrap_or_else(|e| panic!("{label}: the file reads back: {e}"));
    assert_eq!(
        topo::validate_geometric(&imported, tol()),
        Ok(()),
        "{label}: the import is tier-3 valid"
    );
    assert_eq!(
        kinds(&imported, pick),
        native,
        "{label}: the import adopts what the native body stores"
    );
}

// ---------------------------------------------------------------
// An under-determined join: the bored ring's bore latitude.
// ---------------------------------------------------------------

/// The tube radius of [`bored_ring`]'s torus.
const TUBE: f64 = 0.25;

/// A ring revolved about `y` whose torus meets the bore cylinder
/// smoothly along one latitude circle: the profile's 135° arc over the
/// tube ends tangent to the straight run down the bore. The join's
/// sagitta is `r_bore²/(2·TUBE)`, so `r_bore` sets the margin the rule
/// reads; every other profile vertex is a corner.
fn bored_ring(r_bore: f64) -> AtRestBody<f64> {
    let h = 0.3;
    let shoulder = Point2::new(
        r_bore + TUBE + TUBE * core::f64::consts::FRAC_1_SQRT_2,
        TUBE * core::f64::consts::FRAC_1_SQRT_2,
    );
    let bulge = (3.0 * core::f64::consts::FRAC_PI_4 / 4.0).tan();
    let outline = bulge_loop(vec![
        (Point2::new(r_bore, -h), 0.0),
        (Point2::new(shoulder.x, -h), 0.0),
        (shoulder, bulge),
        (Point2::new(r_bore, 0.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![outline])
        .validate(tol())
        .expect("the ring's profile validates");
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&profile, axis, Revolution::Full, tol())
        .expect("the ring revolves")
        .body
}

/// The ring whose join reads the sagitta `margin`.
fn ring_at(margin: f64) -> (AtRestBody<f64>, f64) {
    let r_bore = (2.0 * TUBE * margin).sqrt();
    (bored_ring(r_bore), r_bore)
}

/// The latitude circles (about the revolve axis, `y`) at the bore
/// radius: the smooth join and the base rim.
fn bore_circles(r_bore: f64) -> impl Fn(&Curve3<f64>) -> bool + Copy {
    move |c| {
        matches!(c, Curve3::Circle(geom::Circle { frame, radius, .. })
            if frame.axis().y.abs() > 0.5 && (radius - r_bore).abs() <= r_bore * 1e-9)
    }
}

/// **The bore join round-trips as the rule describes it, on both sides
/// of the band.** A margin of half the tube radius is jet-determinate:
/// the native ring and the import both store the intrinsic tangency. A
/// margin of half ε is under-determined: the native ring stores the
/// pair, and the import, which builds every carrier from its
/// description, has no construction for a join its surfaces
/// under-determine, so it refuses naming the lane. The base rim is a
/// corner throughout.
#[test]
fn a_bore_join_round_trips_as_the_rule_describes_it() {
    let (ring, r_bore) = ring_at(TUBE / 2.0);
    assert_eq!(
        kinds(&ring, bore_circles(r_bore)),
        ["Intersection", "TangentIntersection"],
        "jet-determinate: the native ring stores the join as the rule says"
    );
    round_trips("jet-determinate", &ring, bore_circles(r_bore));

    let (ring, r_bore) = ring_at(band().zero() / 2.0);
    assert_eq!(
        kinds(&ring, bore_circles(r_bore)),
        ["Intersection", "OnPair"],
        "under-determined: the native ring stores the pair"
    );
    match import(&write(&ring)) {
        Err(StepImportError::NotYetSupported { lane, .. }) => assert!(
            lane.contains("under-determined smooth join"),
            "under-determined: the refusal names the lane: {lane}"
        ),
        other => panic!("under-determined: the join refuses not-yet-supported, got {other:?}"),
    }
}

/// The ring's file with its length unit made hectometres: every length
/// the file states reads a hundred times larger, so the join's sagitta
/// does too.
fn in_hectometres(text: &str) -> String {
    const METRE: &str = "( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT($, .METRE.) )";
    assert_eq!(
        text.matches(METRE).count(),
        1,
        "the file states metres once"
    );
    text.replace(
        METRE,
        "( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.HECTO., .METRE.) )",
    )
}

/// The ring whose join reads `margin` in metres, written, its file read
/// in hectometres, with the bore radius in metres as the importer reads it.
fn hecto_ring(margin: f64) -> (String, f64) {
    let (ring, r_bore) = ring_at(margin);
    assert_eq!(
        kinds(&ring, bore_circles(r_bore)),
        ["Intersection", "OnPair"],
        "the native join at {margin:e} is under-determined"
    );
    (in_hectometres(&write(&ring)), 1e2 * r_bore)
}

/// **The rule is read on the file as the import reads it, not on what
/// the writer stored.** The same under-determined ring read a hundred
/// times larger: from a native margin of `ε/2`, the scaled join's
/// sagitta is definitely positive, so the rule demands the intrinsic
/// tangency and the pair is not offered; from a native margin a
/// hundredth of the band's middle, the scaled sagitta is in band, and
/// the import refuses the join typed through the rule rather than
/// adopting either description.
#[test]
fn a_scaled_join_is_read_by_the_rule_at_its_new_size() {
    let (zero, escalate) = (band().zero(), band().escalate());
    if 1e2 * zero / 2.0 <= 2.0 * escalate {
        test_utils::vacuity::stood_down(
            "a hundredfold scale",
            "that a scaled join the rule reads determinate adopts the intrinsic tangency, and \
             an in-band one refuses: ε/2 scaled does not clear the band at this K",
        );
        return;
    }
    let (text, r_bore) = hecto_ring(zero / 2.0);
    let imported = import(&text).unwrap_or_else(|e| panic!("the scaled ring reads back: {e}"));
    assert_eq!(
        topo::validate_geometric(&imported, tol()),
        Ok(()),
        "the scaled ring is tier-3 valid"
    );
    assert_eq!(
        kinds(&imported, bore_circles(r_bore)),
        ["Intersection", "TangentIntersection"],
        "the scaled join is jet-determinate and adopts the intrinsic tangency"
    );

    let middle = (zero * escalate).sqrt();
    let (text, _) = hecto_ring(middle / 1e2);
    match import(&text) {
        Err(StepImportError::Adoption { attempts, .. }) => {
            let last = attempts.last().expect("the build's description was tried");
            assert_eq!(
                last.candidate,
                AdoptionCandidate::TangentIntersection,
                "the tangency reports the rule's refusal: {attempts:?}"
            );
            assert!(
                matches!(last.refusal, topo::EulerOpError::RestJoinUndecided { .. }),
                "the rule reads the scaled join in band: {:?}",
                last.refusal
            );
        }
        other => panic!("an in-band join refuses its adoption, got {other:?}"),
    }
}

/// **The file's join is evidence, never truth.** The under-determined
/// ring's file with the torus moved outward by `100·ε`: its inner
/// equator no longer carries the join's circle, which lies on neither of
/// the two parallels the moved pair cuts at the file's band, so the
/// import refuses at interpretation, naming the miss.
#[test]
fn a_join_off_its_surfaces_still_refuses() {
    let (ring, r_bore) = ring_at(band().zero() / 2.0);
    let text = write(&ring);
    let torus = text
        .lines()
        .find(|l| l.contains("TOROIDAL_SURFACE("))
        .expect("the ring has a torus");
    let fields: Vec<&str> = torus.trim_end_matches(");").rsplitn(3, ", ").collect();
    let major: f64 = fields[1].parse().expect("the major radius parses");
    let moved = torus.replacen(fields[1], &format!("{:?}", major + 100.0 * tol().eps()), 1);
    let text = text.replacen(torus, &moved, 1);
    match import(&text) {
        Err(StepImportError::Interpretation {
            refusal: step_import::InterpretationRefusal::EvidenceMiss { reading, miss },
            ..
        }) => {
            assert_eq!(
                reading, "step_edge_on_description",
                "the join's own reading"
            );
            assert!(
                miss > tol().eps(),
                "the join misses the moved pair's section past ε (r_bore {r_bore:e}): {miss:e}"
            );
        }
        other => panic!("a join off its torus refuses, got {other:?}"),
    }
}

// ---------------------------------------------------------------
// The producers the row was measured on.
// ---------------------------------------------------------------

/// `loops` extruded `h` along `+z` from the `xy` plane.
fn prism(loops: Vec<ProfileLoop<f64>>, h: f64) -> AtRestBody<f64> {
    let profile = Profile::new(SketchPlane::xy(), loops)
        .validate(tol())
        .expect("the profile validates");
    let extrusion = Extrusion::Distance {
        depth: h,
        side: ExtrudeSide::Along,
    };
    extrude(&profile, extrusion, tol())
        .expect("the profile extrudes")
        .body
}

fn is_line(c: &Curve3<f64>) -> bool {
    matches!(c, Curve3::Line(_))
}

/// **The declared-tangent stadium and the split shoulder round-trip at
/// thin depths**: the stadium's line–arc joints and the shoulder's seam
/// along its tangent ruling, each written and read back with the native
/// body's descriptions over every line edge, tier-3 valid.
#[test]
fn thin_tangent_joins_round_trip() {
    let thin = 0.5 * (2.0 * tol().eps()).sqrt();
    for h in [thin, 1e-3] {
        let stadium = bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 1.0),
            (Point2::new(2.0, 2.0), 0.0),
            (Point2::new(0.0, 2.0), 1.0),
        ]);
        round_trips(
            &format!("the stadium {h:e} deep"),
            &prism(vec![stadium], h),
            is_line,
        );
    }
    let cut = topo::test_support::split_plane(
        Point3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        tol(),
    );
    for h in [4e-5, 1e-5] {
        let bulge = (std::f64::consts::PI / 8.0).tan();
        let shoulder = prism(
            vec![bulge_loop(vec![
                (Point2::new(-1.0, 0.0), 0.0),
                (Point2::new(2.0, 0.0), 0.0),
                (Point2::new(2.0, 2.0), 0.0),
                (Point2::new(0.0, 2.0), 0.0),
                (Point2::new(0.0, 1.0), bulge),
            ])],
            h,
        );
        let halves = match topo::split(&shoulder, &cut, tol()) {
            Ok(halves) => halves,
            Err(e) => {
                test_utils::vacuity::stood_down(
                    &format!("the shoulder {h:e} deep"),
                    &format!("its pieces' round trip: the split refuses at this ε: {e:?}"),
                );
                continue;
            }
        };
        for (side, piece) in [("above", &halves.above), ("below", &halves.below)] {
            let piece = piece.body().expect("material on both sides");
            round_trips(&format!("the shoulder {h:e} deep, {side}"), piece, is_line);
        }
    }
}
