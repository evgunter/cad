//! **A definitely-smooth join imports as the must-carry rule says it
//! stores**: the rule over the two surfaces (`geom_brep::reask_pair`)
//! decides, and the adoption ladder offers what it answers. A native body
//! whose join is under-determined stores the conventional pair; written
//! by `step_string` and read back, it adopts the pair and ships tier-3
//! valid. A jet-determinate join adopts the intrinsic tangency, an
//! in-band one refuses typed, and a carrier the file puts off its
//! surfaces still refuses at certification.

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
        matches!(c, Curve3::Circle(geom::Circle { frame, radius })
            if frame.axis().y.abs() > 0.5 && (radius - r_bore).abs() <= r_bore * 1e-9)
    }
}

/// **The bore join round-trips as the rule describes it, on both sides
/// of the band.** A margin of half ε is under-determined: the
/// native ring stores the pair, and the import adopts the pair. A margin
/// of half the tube radius is jet-determinate: both store the intrinsic
/// tangency. The base rim is a corner throughout.
#[test]
fn a_bore_join_round_trips_as_the_rule_describes_it() {
    for (label, margin, join) in [
        ("under-determined", band().zero() / 2.0, "OnPair"),
        ("jet-determinate", TUBE / 2.0, "TangentIntersection"),
    ] {
        let (ring, r_bore) = ring_at(margin);
        let mut want = vec!["Intersection", join];
        want.sort_unstable();
        assert_eq!(
            kinds(&ring, bore_circles(r_bore)),
            want,
            "{label}: the native ring stores the join as the rule says"
        );
        round_trips(label, &ring, bore_circles(r_bore));
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
            let last = attempts.last().expect("the ladder tried something");
            assert_eq!(
                last.candidate,
                AdoptionCandidate::MappedCurve,
                "the conventional rung reports the rule's refusal: {attempts:?}"
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

/// **The rung only offers; certification decides.** The under-determined
/// ring's file with the torus moved outward by `100·ε`: its inner
/// equator no longer carries the join's circle, so the pair the rule
/// offers fails certification, and the import refuses at the join with
/// every candidate tried.
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
        Err(StepImportError::Adoption { attempts, .. }) => {
            let tried: Vec<_> = attempts.iter().map(|a| a.candidate).collect();
            assert_eq!(
                tried,
                [
                    AdoptionCandidate::Intersection,
                    AdoptionCandidate::TangentIntersection,
                    AdoptionCandidate::MappedCurve
                ],
                "the join's three rungs are tried and refused (r_bore {r_bore:e}): {attempts:?}"
            );
            assert!(
                matches!(
                    attempts[2].refusal,
                    topo::EulerOpError::Certification { .. }
                ),
                "the pair is refused by its certificate: {:?}",
                attempts[2].refusal
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

// ---------------------------------------------------------------
// A wild file's rulings between two records of one cylinder.
// ---------------------------------------------------------------

const TAIL_TURBINE: &str = "stepcode/TAIL_TURBINE.stp";

fn census(body: &Body<f64>) -> [usize; 5] {
    [
        body.solids().count(),
        body.shells().count(),
        body.faces().count(),
        body.edges().count(),
        body.vertices().count(),
    ]
}

/// **`TAIL_TURBINE`'s half-cylinder rulings adopt as the pair, and only
/// where the rule reads them so.** Two of its faces wear two records of
/// one cylinder (`#152`, `#563`) and meet along rulings the rule reads
/// under-determined. Pointing face `#593` at `#152` instead makes the
/// two faces one record, whose rulings are read by identity: the same
/// census, and the same volume to rounding. Moving `#563`'s axis `2ε`
/// across puts the rulings' wedge in band, so the rule refuses them,
/// where incidence alone would pass them. Pinned only where `tier_gate`
/// pins the file importing: below that its spline edge refuses first.
/// The import's pass is thin: `#544`'s residual is about `0.79ε`, so a
/// flip here reads as a change in that edge's certification.
#[test]
fn tail_turbines_rulings_adopt_as_the_pair_where_the_rule_reads_them() {
    if crate::wild::refuses_at_this_eps(TAIL_TURBINE) != Some(false) {
        test_utils::vacuity::stood_down(
            "TAIL_TURBINE at this ε",
            "its rulings' adoption: the file refuses earlier at this ε",
        );
        return;
    }
    let text = crate::wild::wild(TAIL_TURBINE);
    let pair = import(&text).expect("the file imports");
    let merged = text.replacen("(#592),#563,.T.", "(#592),#152,.T.", 1);
    assert_ne!(merged, text, "face #593 names #563");
    let one = import(&merged).expect("the one-record control imports");
    assert_eq!(census(&pair), census(&one), "one census either way");
    let volume = |b| topo::test_support::pinned_volume("the turbine", b, tol()).volume;
    let (vp, vo) = (volume(&pair), volume(&one));
    assert!(
        ((vp - vo) / vo).abs() < 1e-12,
        "one volume either way: {vp:e} against {vo:e}"
    );

    const AXIS: &str =
        "#559=CARTESIAN_POINT('Axis2P3D Location',(-7.87401574803,3.14960629921,0.))";
    let shifted = format!(
        "#559=CARTESIAN_POINT('Axis2P3D Location',(-7.87401574803,{:.15},0.))",
        3.14960629921 + 2.0 * tol().eps() / 0.0254
    );
    assert_eq!(text.matches(AXIS).count(), 1, "#563's axis is stated once");
    match import(&text.replacen(AXIS, &shifted, 1)) {
        Err(StepImportError::Adoption { attempts, .. }) => assert!(
            matches!(
                attempts.last().map(|a| &a.refusal),
                Some(topo::EulerOpError::RestJoinUndecided { .. })
            ),
            "the rule refuses the moved rulings in band: {attempts:?}"
        ),
        other => panic!("a ruling 2ε off one record refuses, got {other:?}"),
    }
}
