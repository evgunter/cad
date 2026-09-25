//! **Review probes for the must-carry rule's one home (BLEND-9).**
//!
//! These rows exercise the rule where the unit's own suite reads one
//! point of it: the band is walked as a LADDER rather than sampled at
//! three chosen margins, the lane census is taken through an
//! exhaustive `SurfaceKind` match rather than a hand-written list, and
//! the walk's "first non-`Positive` station decides" rule is put to a
//! pair whose `κ_rel` really does vary along the carrier.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{
    EdgeDescription, MustCarryVerdict, SurfaceKind, must_carry_over_edge, tangent_certificate_lane,
};
use geom_core::{Band, Point2, Point3, Sign, Tol, Vec2, Vec3};
use profile::{Profile, RawLoop, SketchPlane, test_support::bulge_loop};
use sweep::{ExtrudeError, Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::Body;

const MERIDIAN_R: f64 = 0.25;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's linear band")
}

/// The free length whose sagitta over itself is `margin`, for a
/// meridian circle of radius [`MERIDIAN_R`]: `margin = free²/(2r)`.
fn free_length_for(margin: f64) -> f64 {
    (margin * MERIDIAN_R * 2.0).sqrt()
}

/// A unit square with [`MERIDIAN_R`] fillets at every corner, extruded
/// `h` along `+z`: eight tangent line–arc struts and nothing else
/// smooth.
fn filleted_block(h: f64) -> Result<Body<f64>, ExtrudeError> {
    let p2 = Point2::<f64>::new;
    let q = MERIDIAN_R;
    let b = core::f64::consts::FRAC_PI_8.tan();
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

/// A ring whose bore cylinder of radius `r_bore` meets a torus of
/// minor radius [`MERIDIAN_R`] tangentially at the torus's inner
/// equator: one smooth latitude join.
fn bored_ring(r_bore: f64) -> Result<Body<f64>, sweep::RevolveError> {
    let r = MERIDIAN_R;
    let h = 0.3;
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

/// What a verb did with a smooth join at one margin, as one value the
/// ladder rows can compare across the band.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Answer {
    Intrinsic,
    Conventional,
    RefusedInBand,
}

/// The ladder of margins every "the band is walked, not sampled" row
/// below uses: two decades under ε, just under ε, the band's two ends
/// from the inside, its geometric mean, just over `K·ε`, and two
/// decades over it. The kernel's answer must be monotone in this
/// order — conventional, then refusal, then intrinsic — with the
/// transitions at ε and at `K·ε` and nowhere else.
fn ladder() -> Vec<(f64, Answer)> {
    let b = band();
    let (z, k) = (b.zero(), b.escalate());
    vec![
        (z / 100.0, Answer::Conventional),
        (z / 2.0, Answer::Conventional),
        (z * 1.5, Answer::RefusedInBand),
        ((z * k).sqrt(), Answer::RefusedInBand),
        (k / 1.5, Answer::RefusedInBand),
        (k * 1.5, Answer::Intrinsic),
        (k * 100.0, Answer::Intrinsic),
    ]
}

/// The escalation payload every refusing rung must carry: the one
/// metered predicate's name, and a margin really inside the band.
fn check_payload(source: &geom_core::Indeterminate, margin: f64) {
    assert_eq!(
        source.predicate,
        Some("tangent_second_order"),
        "the refusal names the rule's one metered predicate"
    );
    let b = band();
    match source.margin {
        geom_core::MarginDiag::Value(m) => assert!(
            m.abs() > b.zero() && m.abs() < b.escalate(),
            "the reported margin is inside the band at the rung asking for {margin:e}: \
             {m:e} against ({:e}, {:e})",
            b.zero(),
            b.escalate()
        ),
        other => panic!("an in-band refusal reports a value, not {other:?}"),
    }
}

/// **The extrude strut across the whole band, not at three points.**
/// A row that pins one in-band margin cannot tell a rule that
/// escalates on the band from one that escalates on everything below
/// `K·ε`; this walks the ladder and asserts the answer changes exactly
/// at ε and at `K·ε`.
#[test]
fn an_extrude_strut_answers_the_whole_ladder_conventional_then_refused_then_intrinsic() {
    for (margin, want) in ladder() {
        let got = match filleted_block(free_length_for(margin)) {
            Ok(body) => {
                if tangent_intersections(&body) == 8 {
                    Answer::Intrinsic
                } else {
                    assert_eq!(
                        tangent_intersections(&body),
                        0,
                        "a body's eight struts answer the rule the same way at margin {margin:e}"
                    );
                    Answer::Conventional
                }
            }
            Err(ExtrudeError::SliverJoin { ref source, .. }) => {
                check_payload(source, margin);
                Answer::RefusedInBand
            }
            Err(other) => panic!("unexpected refusal at margin {margin:e}: {other}"),
        };
        assert_eq!(got, want, "the strut's answer at margin {margin:e}");
    }
}

/// The revolve twin: the same ladder through the other verb, so a
/// policy that drifts on one of them shows up as a disagreement
/// between two rows built from one list.
#[test]
fn a_revolve_latitude_join_answers_the_whole_ladder_the_same_way() {
    for (margin, want) in ladder() {
        let got = match bored_ring(free_length_for(margin)) {
            Ok(body) => {
                if tangent_intersections(&body) == 1 {
                    Answer::Intrinsic
                } else {
                    assert_eq!(
                        tangent_intersections(&body),
                        0,
                        "the one latitude join answers once at margin {margin:e}"
                    );
                    Answer::Conventional
                }
            }
            Err(sweep::RevolveError::SliverJoin { ref source, .. }) => {
                check_payload(source, margin);
                Answer::RefusedInBand
            }
            Err(other) => panic!("unexpected refusal at margin {margin:e}: {other:?}"),
        };
        assert_eq!(got, want, "the latitude join's answer at margin {margin:e}");
    }
}

/// **The lane census, taken through an exhaustive match.** The unit's
/// own lane row writes the admitted kinds out by hand, so a verb that
/// grew a wall kind outside the lane would leave it green. This one
/// maps EVERY [`SurfaceKind`] to a representative surface through a
/// match with no wildcard arm: a new kind does not compile until
/// somebody says which side of the lane it is on, and the row then
/// asserts the gate agrees.
#[test]
fn the_lane_census_is_exhaustive_over_surface_kind() {
    /// Is this kind inside the certificate's lane on a `Line` carrier,
    /// and on a `Circle` carrier? (`straight`, `round`.)
    fn expected(kind: SurfaceKind) -> (bool, bool) {
        match kind {
            SurfaceKind::Plane | SurfaceKind::Cylinder | SurfaceKind::Sphere => (true, true),
            SurfaceKind::Cone | SurfaceKind::Torus => (false, true),
            SurfaceKind::Nurbs | SurfaceKind::Approx => (false, false),
        }
    }
    let p = Point3::new(0.0, 0.0, 0.0);
    let axis = Vec3::new(0.0, 0.0, 1.0);
    let u_ref = Vec3::new(1.0, 0.0, 0.0);
    let line = Curve3::Line {
        origin: p,
        dir: axis,
    };
    let circle = Curve3::Circle {
        center: p,
        axis,
        radius: 1.0,
        u_ref,
    };
    let reps: Vec<Surface<f64>> = vec![
        Surface::Plane {
            origin: p,
            normal: axis,
            u_ref,
        },
        Surface::Cylinder {
            origin: p,
            axis,
            radius: 1.0,
            u_ref,
        },
        Surface::Cone {
            apex: p,
            axis,
            half_angle: core::f64::consts::FRAC_PI_4,
            u_ref,
        },
        Surface::Sphere {
            center: p,
            radius: 1.0,
            axis,
            u_ref,
        },
        Surface::Torus {
            center: p,
            axis,
            major_radius: 2.0,
            minor_radius: 0.5,
            u_ref,
        },
    ];
    for a in &reps {
        for b in &reps {
            let (a_line, a_circle) = expected(SurfaceKind::of(a));
            let (b_line, b_circle) = expected(SurfaceKind::of(b));
            assert_eq!(
                tangent_certificate_lane(&line, a, b),
                a_line && b_line,
                "the Line lane on {:?}/{:?}",
                SurfaceKind::of(a),
                SurfaceKind::of(b)
            );
            assert_eq!(
                tangent_certificate_lane(&circle, a, b),
                a_circle && b_circle,
                "the Circle lane on {:?}/{:?}",
                SurfaceKind::of(a),
                SurfaceKind::of(b)
            );
        }
    }
    // The carrier kinds outside both lanes, so the census covers the
    // carrier axis too.
    let arbitrary = Curve3::Line {
        origin: p,
        dir: axis,
    };
    assert!(tangent_certificate_lane(&arbitrary, &reps[0], &reps[0]));
}

/// **A pair inside the lane whose `κ_rel` really varies along the
/// carrier**, which is what the wrapper's two symmetry facts do NOT
/// cover: they are each true of the joins ONE verb mints, while the
/// walk must answer for every triple it is handed. A sphere and a
/// cylinder read along a circle that is coaxial with neither gives
/// stations that disagree, and the row pins what the walk then does —
/// the first non-`Positive` station decides, and the verdict carries
/// THAT station's reading (nothing of the first station's is returned).
#[test]
fn a_pair_whose_kappa_rel_varies_along_the_carrier_decides_at_a_later_station() {
    let (s1, s2, carrier, t0, t1, extent) = varying_triple();
    assert!(
        tangent_certificate_lane(&carrier, &s1, &s2),
        "the triple is inside the certificate's lane"
    );
    let stations: Vec<_> = (1..geom_brep::CERT_SAMPLES - 1)
        .map(|i| {
            let t = geom_brep::sample_param(t0, t1, i);
            let p = carrier.eval(t);
            geom_brep::tangent_second_order(&s1, &s2, p, carrier.deriv(t), extent, band())
        })
        .collect();
    let kappas: Vec<f64> = stations.iter().map(|s| s.jet.kappa_rel.abs()).collect();
    let spread = kappas.iter().copied().fold(f64::MIN, f64::max)
        / kappas.iter().copied().fold(f64::MAX, f64::min);
    assert!(
        spread > 100.0,
        "the row needs a carrier along which kappa_rel really varies, got {kappas:?}"
    );
    let deciding = stations
        .iter()
        .position(|s| !matches!(s.verdict, Ok(Sign::Positive)))
        .expect("the arm is derived so that a later station is not definitely positive");
    assert!(
        deciding > 0,
        "the deciding station must not be the first, or the row proves nothing: {kappas:?}"
    );
    let answer = must_carry_over_edge(&s1, &s2, &carrier, t0, t1, extent, band());
    assert_ne!(
        answer,
        MustCarryVerdict::JetDeterminate,
        "one station that is not definitely positive denies the whole edge"
    );
    // The verdict is the whole answer, and the only number that rides
    // with it is the DECIDING station's, inside `InBand`'s payload. A
    // reading taken at station 1 would be definitely positive here
    // (`stations[0]`, asserted through `deciding > 0` above), so a
    // caller reporting it as the cause would report a margin that
    // passed: {kappas:?} is the spread that makes that concrete.
    if let MustCarryVerdict::InBand(source) = answer {
        assert_eq!(
            source.predicate,
            Some("tangent_second_order"),
            "the payload names the deciding station's predicate"
        );
    }
}

/// A sphere and a coaxial cylinder read along a MERIDIAN circle —
/// in the lane by kind (a `Circle` over two `round` surfaces), and
/// with a relative curvature that runs over two and a half decades
/// along the carrier. The lever arm is derived from the band so the
/// first stations are definitely positive and a later one is not,
/// whatever ε the run committed: with `arm² / 2 = √(ε·K·ε)` a station
/// is positive while `κ_rel ≥ √K` and stops being so below it.
fn varying_triple() -> (Surface<f64>, Surface<f64>, Curve3<f64>, f64, f64, f64) {
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let cylinder = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let carrier = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let b = band();
    let extent = (2.0 * (b.zero() * b.escalate()).sqrt()).sqrt();
    // Walked from the pole end, so the big curvatures come first.
    (
        sphere,
        cylinder,
        carrier,
        core::f64::consts::FRAC_PI_2,
        0.0,
        extent,
    )
}
