//! In-crate tier-3 tests (M2 PR 3): the corruption directions only the
//! raw arenas can reach — a certified body whose stored geometry is
//! then made wrong must be caught by [`crate::validate_geometric`]'s
//! re-checks at rest (the other half of the attachment/at-rest pair;
//! the attachment-side rejections live in `tests/geometric_cube.rs`).
//!
//! The corruptions here replace whole arena entries (an [`EdgeCurve`]
//! is only constructible certified, so "a wrong cache" means "a
//! certified cache for *different* data sitting at this edge's slot" —
//! exactly what re-certification against the edge's own endpoints
//! exists to catch).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NetState, NurbsSurface, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

use crate::euler::FaceSurface;
use crate::fixtures::{refile_shells, test_curve};
use crate::validate::{
    MaterialArmOutcome, ValidationError, material_arm_error, material_arm_outcome, validate,
    validate_geometric,
};
use crate::{Body, MefSite, MevSite};
use geom_brep::MaterialWedge;
use geom_core::Indeterminate;
use geom_core::Tol;

/// A geometric digon pillow: two vertices, two chord edges, two
/// coplanar faces (the z = 0 plane on both sides) — the minimal
/// tier-3-clean body, and the coplanar-split smooth-dihedral case.
fn coplanar_pillow(tol: Tol) -> (Body<f64>, crate::MefCreated) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(1.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let split = body
        .mef(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            EdgeCurveSpec::line_between(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)),
            FaceSurface::New {
                surface: plane.clone(),
                sense: true,
            },
            tol,
        )
        .unwrap();
    // The seed face shares the same geometric plane under its own key
    // (identical-by-construction would share the KEY in a sweep; here
    // the point is the smooth-dihedral classification, which compares
    // the surfaces' tangent planes, not their keys).
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: plane,
            sense: true,
        },
    )
    .unwrap();
    // Both chords were built through the SCAFFOLDING door, because
    // neither face's surface existed when its `mev`/`mef` ran. The
    // body is at rest now and both faces have charts, so both edges
    // are re-described where they rest (D3's transience fence — tier
    // 3 refuses a scaffold on a body with faces).
    let chart = body.get_face(split.face).unwrap().surface;
    for e in [seg.edge, split.edge] {
        let spec =
            EdgeCurveSpec::line_between(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0))
                .at_rest_in_chart(chart, false);
        body.set_edge_curve(e, spec, tol).unwrap();
    }
    (body, split)
}

#[test]
fn coplanar_split_is_smooth_and_tier3_clean() {
    let tol = Tol::witness();
    let (body, _) = coplanar_pillow(tol);
    assert_eq!(validate_geometric(&body, tol), Ok(()));
}

/// **The transience fence** (U2's Q2 as corrected, 2026-08-27), red
/// then green on ONE edge of one body.
///
/// RED: the scaffolding door describes a locus for an edge whose
/// surfaces do not exist yet. Put that description back on an edge of
/// a body AT REST — two faces, two charts — and tier 3 names it.
///
/// GREEN: the same edge, same carrier, same interval, described where
/// it rests (an image in the chart it lies in) validates clean. Only
/// the description moves, which is the whole content of the fence.
#[test]
fn a_scaffold_at_rest_is_refused_and_the_chart_description_is_not() {
    let tol = Tol::witness();
    let (mut body, split) = coplanar_pillow(tol);
    assert_eq!(validate_geometric(&body, tol), Ok(()));

    // RED — back through the scaffolding door.
    let scaffolded =
        EdgeCurveSpec::line_between(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    body.set_edge_curve(split.edge, scaffolded.clone(), tol)
        .expect("the door itself is legal: certification is not where the fence lives");
    let errors = validate_geometric(&body, tol).expect_err("a scaffold at rest is refused");
    assert!(
        errors.contains(&ValidationError::ScaffoldAtRest { edge: split.edge }),
        "tier 3 must name the scaffolded edge, got {errors:?}",
    );

    // GREEN — the same edge described where it rests.
    let chart = body.get_face(split.face).unwrap().surface;
    body.set_edge_curve(split.edge, scaffolded.at_rest_in_chart(chart, false), tol)
        .unwrap();
    assert_eq!(validate_geometric(&body, tol), Ok(()));
}

/// The other half of the fence: the door it exists to keep open. An
/// edge whose surfaces genuinely do not exist yet — a `mev` chord in a
/// half-built ring — carries a scaffolding description and is NOT
/// refused, because the fence is TRANSIENCE and this edge is transient.
#[test]
fn the_scaffolding_door_still_passes_mid_construction() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
    body.mev_line(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        Point3::new(1.0, 0.0, 0.0),
        tol,
    )
    .unwrap();
    // No surface anywhere yet, so the chord could not name a chart
    // even in principle — and tier 3 says nothing about it.
    assert_eq!(validate(&body), Ok(()));
}

// ----------------------------------------------------------------------
// Tier 3, check 1: a `Nurbs` face surface's `NetState`. The
// placeholder's own verdict row is `tests/geometric_cube.rs`'s
// `without_the_top_cap_tier3_rejects_the_nurbs_seed`; the ladder below
// covers all three states and where their boundaries lie.
// ----------------------------------------------------------------------

/// A bilinear net over the given control points, on the pillow's face.
fn bilinear_net(control: Vec<Point3<f64>>) -> Surface<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap(),
    ))
}

/// A finite control point of the unit-square net.
fn finite_point(i: i32) -> Point3<f64> {
    Point3::new(f64::from(i % 2), f64::from(i / 2), 0.0)
}

/// `NetState::Poisoned`: every control point carries poison in `x` over
/// finite `y`/`z`.
fn poisoned_net() -> Surface<f64> {
    bilinear_net(
        (0..4)
            .map(|i| Point3::new(f64::NAN, f64::from(i), 2.0))
            .collect(),
    )
}

/// `NetState::Described`: the same knots and weights over finite
/// control points.
fn finite_net() -> Surface<f64> {
    bilinear_net((0..4).map(finite_point).collect())
}

/// The refusals a body draws once the check-1 surface verdicts are
/// taken out of it — what the REST of tier 3 says about the same face.
fn without_surface_verdicts(errs: &[ValidationError]) -> Vec<ValidationError> {
    errs.iter()
        .filter(|e| {
            !matches!(
                e,
                ValidationError::UncertifiableSurface { .. }
                    | ValidationError::PoisonedSurfaceDescription { .. }
            )
        })
        .cloned()
        .collect()
}

fn pillow_on(surface: Surface<f64>, tol: Tol) -> (Vec<ValidationError>, crate::entity::FaceKey) {
    let (mut body, split) = coplanar_pillow(tol);
    assert_eq!(validate_geometric(&body, tol), Ok(()));
    // Lifts both refusals: tier 3's surface verdicts on the swapped face are the row's, whatever it strands.
    body.set_face_surface_stranding_for_tests(
        split.face,
        FaceSurface::New {
            surface,
            sense: true,
        },
    )
    .unwrap();
    assert_eq!(validate(&body), Ok(()), "structurally still coherent");
    (
        validate_geometric(&body, tol).expect_err("the swapped chart is refused at rest"),
        split.face,
    )
}

/// A face carrying a described net that is poisoned in one channel is
/// named by check 1 ITSELF, and named as the state it is in: not the
/// placeholder's verdict, which is the benign "no description yet".
#[test]
fn a_described_net_carrying_poison_is_named_by_the_surface_check() {
    let tol = Tol::witness();
    let (errs, face) = pillow_on(poisoned_net(), tol);
    assert!(
        errs.contains(&ValidationError::PoisonedSurfaceDescription { face }),
        "check 1 must name the corrupt described surface: {errs:?}",
    );
    assert!(
        !errs
            .iter()
            .any(|e| matches!(e, ValidationError::UncertifiableSurface { .. })),
        "the placeholder's verdict is a different state's answer: {errs:?}",
    );
}

/// The control: the same face, the same knots and weights, FINITE
/// control points. Real geometry earns no surface verdict of either
/// kind, whatever else the body reports.
#[test]
fn a_finite_described_net_draws_no_surface_verdict() {
    let tol = Tol::witness();
    let (finite, _) = pillow_on(finite_net(), tol);
    assert_eq!(
        without_surface_verdicts(&finite),
        finite,
        "real geometry earns no verdict from the surface check: {finite:?}",
    );
}

/// **Where the state boundaries lie**, on one body per rung: the state
/// `geom` reports for a net decides the verdict check 1 gives it — with
/// one further read inside `Described` — each face draws at most one
/// surface verdict, and the face named is the swapped one.
///
/// The rungs that carry the argument are the near misses. Poison in
/// EVERY channel of every point is the placeholder however the net was
/// built, so a hand-built all-poison net is `Placeholder` and not
/// `Poisoned`; poison in every channel of ONE point is not, because the
/// width rule quantifies over points as well as channels; and `±∞` is
/// not `f64` poison at all, so a net carrying an infinity is
/// `Described` — and check 1 refuses it anyway, as the poisoned net it
/// is, because an infinite control point describes no locus exactly as
/// an infinite radius does not.
#[test]
fn the_net_state_ladder_decides_check_1s_verdict() {
    let tol = Tol::witness();
    let cases: Vec<(&str, Surface<f64>)> = vec![
        ("the mvfs placeholder", Surface::nurbs_placeholder()),
        (
            "all channels of all points, hand-built",
            bilinear_net(
                (0..4)
                    .map(|_| Point3::new(f64::NAN, f64::NAN, f64::NAN))
                    .collect(),
            ),
        ),
        ("finite", finite_net()),
        ("poison x at every point", poisoned_net()),
        (
            "poison y at every point",
            bilinear_net(
                (0..4)
                    .map(|i| Point3::new(f64::from(i), f64::NAN, 2.0))
                    .collect(),
            ),
        ),
        (
            "poison z at ONE point",
            bilinear_net(
                (0..4)
                    .map(|i| {
                        let p = finite_point(i);
                        if i == 2 {
                            Point3::new(p.x, p.y, f64::NAN)
                        } else {
                            p
                        }
                    })
                    .collect(),
            ),
        ),
        (
            "all channels of ONE point",
            bilinear_net(
                (0..4)
                    .map(|i| {
                        if i == 3 {
                            Point3::new(f64::NAN, f64::NAN, f64::NAN)
                        } else {
                            finite_point(i)
                        }
                    })
                    .collect(),
            ),
        ),
        (
            "negative infinity at one point",
            bilinear_net(
                (0..4)
                    .map(|i| {
                        if i == 1 {
                            Point3::new(0.0, f64::NEG_INFINITY, 0.0)
                        } else {
                            finite_point(i)
                        }
                    })
                    .collect(),
            ),
        ),
        (
            "infinite x at every point",
            bilinear_net(
                (0..4)
                    .map(|i| Point3::new(f64::INFINITY, f64::from(i), 2.0))
                    .collect(),
            ),
        ),
    ];

    for (name, surface) in cases {
        let Surface::Nurbs(payload) = &surface else {
            unreachable!("every rung is a Nurbs surface")
        };
        let state = payload.net_state();
        let finite = payload
            .control()
            .iter()
            .all(|p| p.x.is_finite() && p.y.is_finite() && p.z.is_finite());
        let (errs, face) = pillow_on(surface.clone(), tol);
        let placeholder = errs.contains(&ValidationError::UncertifiableSurface { face });
        let poisoned = errs.contains(&ValidationError::PoisonedSurfaceDescription { face });
        assert_eq!(
            (placeholder, poisoned),
            match state {
                NetState::Placeholder => (true, false),
                NetState::Poisoned => (false, true),
                NetState::Described => (false, !finite),
            },
            "{name}: the state is {state:?} (finite: {finite}) and check 1 answered {errs:?}",
        );
        // No other face is named by a surface verdict, ever.
        for e in &errs {
            match e {
                ValidationError::UncertifiableSurface { face: f }
                | ValidationError::PoisonedSurfaceDescription { face: f } => {
                    assert_eq!(*f, face, "{name}: a surface verdict names the wrong face");
                }
                _ => {}
            }
        }
    }
}

/// The check-1 datum verdicts a body draws, in report order.
fn datum_verdicts(errs: &[ValidationError]) -> Vec<ValidationError> {
    errs.iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::PoisonedSurfaceDatum { .. }
                    | ValidationError::UnrepresentableSurfaceDatum { .. }
            )
        })
        .cloned()
        .collect()
}

fn cylinder(radius: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        radius,
        u_ref: Vec3::unit_x(),
    }
}

fn sphere(radius: f64) -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

fn cone(half_angle: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        half_angle,
        u_ref: Vec3::unit_x(),
    }
}

fn torus(major_radius: f64, minor_radius: f64) -> Surface<f64> {
    Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        major_radius,
        minor_radius,
        u_ref: Vec3::unit_x(),
    }
}

fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
    Surface::Plane {
        origin,
        normal,
        u_ref: Vec3::unit_x(),
    }
}

/// **Check 1 names an analytic surface's datum when it describes no
/// locus**, on the tier-3-clean pillow with one face's surface swapped:
/// the datum verdict is the FIRST finding the body draws — check 1 runs
/// before the checks that read the surface for other questions — and it
/// names the face, the kind and the datum.
///
/// Two halves. A datum that is not a number (NaN or `±∞`), or a stored
/// direction (`normal`, `axis`, `u_ref`) that is the zero vector, is
/// `PoisonedSurfaceDatum`; a finite datum outside its variant's
/// convention — a frame off unit or off `u_ref ⊥ axis` by more than ε
/// of locus movement at the kind's radius included — is
/// `UnrepresentableSurfaceDatum`. The first six rungs are the
/// measurement the carried row took before check 1 read any analytic
/// datum, when every one of them was refused only by checks 3–5's
/// escalations.
#[test]
fn check_1_names_the_analytic_datum_that_describes_no_locus() {
    use geom::ConventionEnd::{Lower, Upper};
    use geom::ConventionMeasure::{Length, Tilt, Value};
    use geom::SurfaceDatum as D;
    use geom_brep::SurfaceKind as K;
    enum Verdict {
        Poisoned,
        Unrepresentable(geom::ConventionMeasure, geom::ConventionEnd),
    }
    let tol = Tol::witness();
    let o = Point3::new(0.0, 0.0, 0.0);
    let cases: Vec<(&str, Surface<f64>, K, D, Verdict)> = vec![
        (
            "plane, NaN origin",
            plane(Point3::new(f64::NAN, 0.0, 0.0), Vec3::unit_z()),
            K::Plane,
            D::Origin,
            Verdict::Poisoned,
        ),
        (
            "plane, zero normal",
            plane(o, Vec3::new(0.0, 0.0, 0.0)),
            K::Plane,
            D::Normal,
            Verdict::Poisoned,
        ),
        (
            "plane, NaN normal",
            plane(o, Vec3::new(f64::NAN, f64::NAN, f64::NAN)),
            K::Plane,
            D::Normal,
            Verdict::Poisoned,
        ),
        (
            "cylinder, NaN radius",
            cylinder(f64::NAN),
            K::Cylinder,
            D::Radius,
            Verdict::Poisoned,
        ),
        (
            "sphere, zero radius",
            sphere(0.0),
            K::Sphere,
            D::Radius,
            Verdict::Unrepresentable(Value, Lower),
        ),
        (
            "cone, NaN half-angle",
            cone(f64::NAN),
            K::Cone,
            D::HalfAngle,
            Verdict::Poisoned,
        ),
        (
            "plane, infinite origin",
            plane(Point3::new(0.0, f64::INFINITY, 0.0), Vec3::unit_z()),
            K::Plane,
            D::Origin,
            Verdict::Poisoned,
        ),
        (
            "plane, NaN u_ref",
            Surface::Plane {
                origin: o,
                normal: Vec3::unit_z(),
                u_ref: Vec3::new(f64::NAN, 0.0, 0.0),
            },
            K::Plane,
            D::URef,
            Verdict::Poisoned,
        ),
        (
            "cylinder, negative radius",
            cylinder(-1.0),
            K::Cylinder,
            D::Radius,
            Verdict::Unrepresentable(Value, Lower),
        ),
        (
            "cylinder, infinite radius",
            cylinder(f64::INFINITY),
            K::Cylinder,
            D::Radius,
            Verdict::Poisoned,
        ),
        (
            "sphere, negative radius",
            sphere(-2.0),
            K::Sphere,
            D::Radius,
            Verdict::Unrepresentable(Value, Lower),
        ),
        (
            "cone, zero half-angle",
            cone(0.0),
            K::Cone,
            D::HalfAngle,
            Verdict::Unrepresentable(Value, Lower),
        ),
        (
            "cone, half-angle pi/2",
            cone(core::f64::consts::FRAC_PI_2),
            K::Cone,
            D::HalfAngle,
            Verdict::Unrepresentable(Value, Upper),
        ),
        (
            "cone, half-angle past pi/2",
            cone(2.0),
            K::Cone,
            D::HalfAngle,
            Verdict::Unrepresentable(Value, Upper),
        ),
        (
            "torus, zero tube",
            torus(2.0, 0.0),
            K::Torus,
            D::MinorRadius,
            Verdict::Unrepresentable(Value, Lower),
        ),
        (
            "torus, NaN tube",
            torus(2.0, f64::NAN),
            K::Torus,
            D::MinorRadius,
            Verdict::Poisoned,
        ),
        (
            "torus, infinite major radius",
            torus(f64::INFINITY, 0.5),
            K::Torus,
            D::MajorRadius,
            Verdict::Poisoned,
        ),
        (
            "torus, NaN major radius",
            torus(f64::NAN, 0.5),
            K::Torus,
            D::MajorRadius,
            Verdict::Poisoned,
        ),
        (
            "torus, infinite tube",
            torus(2.0, f64::INFINITY),
            K::Torus,
            D::MinorRadius,
            Verdict::Poisoned,
        ),
        (
            "cone, NaN apex",
            Surface::Cone {
                apex: Point3::new(0.0, f64::NAN, 0.0),
                axis: Vec3::unit_z(),
                half_angle: core::f64::consts::FRAC_PI_4,
                u_ref: Vec3::unit_x(),
            },
            K::Cone,
            D::Apex,
            Verdict::Poisoned,
        ),
        (
            "cylinder, infinite axis",
            Surface::Cylinder {
                origin: o,
                axis: Vec3::new(0.0, 0.0, f64::INFINITY),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            K::Cylinder,
            D::Axis,
            Verdict::Poisoned,
        ),
        (
            "sphere, NaN center",
            Surface::Sphere {
                center: Point3::new(f64::NAN, 0.0, 0.0),
                radius: 1.0,
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            K::Sphere,
            D::Center,
            Verdict::Poisoned,
        ),
        (
            "cylinder, zero axis",
            Surface::Cylinder {
                origin: o,
                axis: Vec3::new(0.0, 0.0, 0.0),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            K::Cylinder,
            D::Axis,
            Verdict::Poisoned,
        ),
        (
            "cylinder, zero u_ref",
            Surface::Cylinder {
                origin: o,
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::new(0.0, 0.0, 0.0),
            },
            K::Cylinder,
            D::URef,
            Verdict::Poisoned,
        ),
        (
            "plane, zero u_ref",
            Surface::Plane {
                origin: o,
                normal: Vec3::unit_z(),
                u_ref: Vec3::new(0.0, 0.0, 0.0),
            },
            K::Plane,
            D::URef,
            Verdict::Poisoned,
        ),
        (
            "cone, zero axis",
            Surface::Cone {
                apex: o,
                axis: Vec3::new(0.0, 0.0, 0.0),
                half_angle: core::f64::consts::FRAC_PI_4,
                u_ref: Vec3::unit_x(),
            },
            K::Cone,
            D::Axis,
            Verdict::Poisoned,
        ),
        (
            "sphere, zero u_ref",
            Surface::Sphere {
                center: o,
                radius: 1.0,
                axis: Vec3::unit_z(),
                u_ref: Vec3::new(0.0, 0.0, 0.0),
            },
            K::Sphere,
            D::URef,
            Verdict::Poisoned,
        ),
        (
            "cylinder, u_ref 100 eps long at r = 1",
            Surface::Cylinder {
                origin: o,
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::new(1.0 + 100.0 * tol.get().eps, 0.0, 0.0),
            },
            K::Cylinder,
            D::URef,
            Verdict::Unrepresentable(Length, Upper),
        ),
        (
            "sphere, axis of half length",
            Surface::Sphere {
                center: o,
                radius: 1.0,
                axis: Vec3::new(0.0, 0.0, 0.5),
                u_ref: Vec3::unit_x(),
            },
            K::Sphere,
            D::Axis,
            Verdict::Unrepresentable(Length, Lower),
        ),
        (
            "torus, unit u_ref tilted off the axis's normal plane",
            Surface::Torus {
                center: o,
                axis: Vec3::unit_z(),
                major_radius: 2.0,
                minor_radius: 0.5,
                u_ref: Vec3::new(0.6, 0.0, 0.8),
            },
            K::Torus,
            D::URef,
            Verdict::Unrepresentable(Tilt, Upper),
        ),
        (
            "torus, zero axis",
            Surface::Torus {
                center: o,
                axis: Vec3::new(0.0, 0.0, 0.0),
                major_radius: 2.0,
                minor_radius: 0.5,
                u_ref: Vec3::unit_x(),
            },
            K::Torus,
            D::Axis,
            Verdict::Poisoned,
        ),
    ];
    for (name, surface, kind, datum, verdict) in cases {
        let (errs, face) = pillow_on(surface, tol);
        let expected = match verdict {
            Verdict::Poisoned => ValidationError::PoisonedSurfaceDatum { face, kind, datum },
            Verdict::Unrepresentable(measure, end) => {
                ValidationError::UnrepresentableSurfaceDatum {
                    face,
                    kind,
                    datum,
                    measure,
                    end,
                }
            }
        };
        assert_eq!(
            errs.first(),
            Some(&expected),
            "{name}: check 1's datum verdict is the first finding: {errs:?}",
        );
        assert_eq!(
            datum_verdicts(&errs),
            vec![expected],
            "{name}: one datum verdict, on the swapped face: {errs:?}",
        );
    }
}

/// The control, and the boundary on the inside: surfaces whose every
/// datum is a number inside its convention draw no datum verdict,
/// whatever else the swap costs the body — a cone ONE ULP inside either
/// end of `(0, π/2)` (so a bound carrying any tolerance reds), a plane
/// whose normal underflows its length without being zero, a plane
/// whose finite normal's NORM overflows (a direction, not the zero
/// vector), a cylinder whose frame is off unit by less than ε of locus
/// movement, and a plane whose frame is not unit at all (it spans the
/// same plane) included.
#[test]
fn datums_inside_their_conventions_draw_no_datum_verdict() {
    let tol = Tol::witness();
    let cases: Vec<(&str, Surface<f64>)> = vec![
        ("unit cylinder", cylinder(1.0)),
        ("unit sphere", sphere(1.0)),
        ("cone at pi/4", cone(core::f64::consts::FRAC_PI_4)),
        ("cone one ulp above 0", cone(f64::from_bits(1))),
        (
            "cone one ulp below pi/2",
            cone(f64::from_bits(core::f64::consts::FRAC_PI_2.to_bits() - 1)),
        ),
        ("ring torus", torus(2.0, 0.5)),
        (
            "cylinder, u_ref eps/100 long at r = 1",
            Surface::Cylinder {
                origin: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::new(1.0 + 0.01 * tol.get().eps, 0.0, 0.0),
            },
        ),
        (
            "plane, normal and u_ref of length 3 (the same plane)",
            Surface::Plane {
                origin: Point3::new(0.0, 0.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 3.0),
                u_ref: Vec3::new(3.0, 0.0, 0.0),
            },
        ),
        (
            "plane, normal underflowed but not zero",
            plane(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1e-200)),
        ),
        (
            "plane, normal whose norm overflows at f64 (1e160)",
            plane(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1e160)),
        ),
        (
            "plane, normal whose norm overflows at f64 (1e200)",
            plane(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1e200)),
        ),
    ];
    for (name, surface) in cases {
        let (mut body, split) = coplanar_pillow(tol);
        // Lifts both refusals: the datum verdicts are the row's, whatever else the swap costs the body.
        body.set_face_surface_stranding_for_tests(
            split.face,
            FaceSurface::New {
                surface,
                sense: true,
            },
        )
        .unwrap();
        let errs = validate_geometric(&body, tol).err().unwrap_or_default();
        assert_eq!(
            datum_verdicts(&errs),
            vec![],
            "{name}: a datum inside its convention earns no verdict: {errs:?}",
        );
    }
}

/// The check-1 carrier-datum verdicts a body draws, in report order.
fn curve_datum_verdicts(errs: &[ValidationError]) -> Vec<ValidationError> {
    errs.iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::PoisonedCurveDatum { .. }
                    | ValidationError::UnrepresentableCurveDatum { .. }
            )
        })
        .cloned()
        .collect()
}

/// The pillow with one chord's carrier re-minted through the public
/// attach door as `carrier` over `(t0, t1)`, described in the face's
/// plane chart; `Err` is the mint's refusal.
fn pillow_with_carrier(
    carrier: geom::Curve3<f64>,
    t0: f64,
    t1: f64,
    tol: Tol,
) -> Result<(Body<f64>, crate::entity::EdgeKey), crate::EulerOpError> {
    let (mut body, split) = coplanar_pillow(tol);
    let chart = body.get_face(split.face).unwrap().surface;
    body.set_edge_curve(
        split.edge,
        EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::chart(chart),
            carrier,
            param_start: t0,
            param_end: t1,
        },
        tol,
    )?;
    Ok((body, split.edge))
}

/// **Check 1 names an edge carrier's datum when it describes no curve
/// of its kind**, on the pillow with one chord re-minted through the
/// public attach door as a half-arc from `(0,0,0)` to `(1,0,0)`.
///
/// These are the carriers the mint CERTIFIES: certification meters the
/// carrier's residuals against its endpoints and its chart, and a
/// circle or ellipse whose `axis` is zero traces the diameter between
/// the two vertices — which lies in the chart — while an ellipse whose
/// `major` is negative and whose `u_ref` is flipped traces the ellipse
/// it would with both signs righted; a circle whose `axis` has length 2
/// traces an ellipse through the same two vertices. Before check 1 read
/// carrier datums all four minted and nothing at rest named the datum
/// (the carried row's measurement); each is now refused, first, by
/// name.
///
/// The honest twins — the same arcs with their datums righted — mint
/// and draw no carrier-datum verdict, so the refusal is the datum's and
/// not the fixture's. (They are not clean: a bulged chord leaves one of
/// the pillow's two coplanar faces wound against its bit, which check 6
/// refuses on either bulge side; that verdict is the fixture's and
/// rides after the datum's.)
#[test]
fn check_1_names_the_carrier_datum_that_describes_no_curve() {
    use crate::query::CurveKind as K;
    use geom::ConventionEnd::{Lower, Upper};
    use geom::ConventionMeasure::{Length, Value};
    use geom::Curve3;
    use geom::CurveDatum as D;
    let tol = Tol::witness();
    let pi = core::f64::consts::PI;
    let c = Point3::new(0.5, 0.0, 0.0);
    let zero = Vec3::new(0.0, 0.0, 0.0);
    let x = Vec3::unit_x();
    let circle = |axis: Vec3<f64>| Curve3::Circle {
        center: c,
        axis,
        radius: 0.5,
        u_ref: -x,
    };
    let ellipse = |axis: Vec3<f64>, major: f64, u_ref: Vec3<f64>| Curve3::Ellipse {
        center: c,
        axis,
        major,
        minor: 0.3,
        u_ref,
    };
    let axis = Vec3::unit_z();
    for (name, honest) in [
        ("circle", circle(axis)),
        ("ellipse", ellipse(axis, 0.5, -x)),
    ] {
        let (body, _) = pillow_with_carrier(honest, 0.0, pi, tol).unwrap();
        let errs = validate_geometric(&body, tol).err().unwrap_or_default();
        assert_eq!(
            curve_datum_verdicts(&errs),
            vec![],
            "the honest {name} draws no carrier-datum verdict: {errs:?}"
        );
    }
    enum Verdict {
        Poisoned,
        Unrepresentable(geom::ConventionMeasure, geom::ConventionEnd),
    }
    let cases: Vec<(&str, Curve3<f64>, K, D, Verdict)> = vec![
        (
            "circle, zero axis",
            circle(zero),
            K::Circle,
            D::Axis,
            Verdict::Poisoned,
        ),
        (
            "ellipse, zero axis",
            ellipse(zero, 0.5, -x),
            K::Ellipse,
            D::Axis,
            Verdict::Poisoned,
        ),
        (
            "ellipse, negative major with u_ref flipped",
            ellipse(axis, -0.5, x),
            K::Ellipse,
            D::Major,
            Verdict::Unrepresentable(Value, Lower),
        ),
        (
            "circle, axis of length 2 (it evaluates an ellipse)",
            circle(axis * 2.0),
            K::Circle,
            D::Axis,
            Verdict::Unrepresentable(Length, Upper),
        ),
    ];
    for (name, carrier, kind, datum, verdict) in cases {
        let (body, edge) = pillow_with_carrier(carrier, 0.0, pi, tol)
            .unwrap_or_else(|e| panic!("{name}: the mint certifies this carrier: {e:?}"));
        let errs = validate_geometric(&body, tol).expect_err("the carrier datum is refused");
        let expected = match verdict {
            Verdict::Poisoned => ValidationError::PoisonedCurveDatum { edge, kind, datum },
            Verdict::Unrepresentable(measure, end) => ValidationError::UnrepresentableCurveDatum {
                edge,
                kind,
                datum,
                measure,
                end,
            },
        };
        assert_eq!(
            errs.first(),
            Some(&expected),
            "{name}: the carrier datum verdict is the first finding: {errs:?}",
        );
        assert_eq!(
            curve_datum_verdicts(&errs),
            vec![expected],
            "{name}: one carrier datum verdict, on the re-minted edge: {errs:?}",
        );
    }
}

/// **The carrier-datum read over the datums no mint lets through**:
/// certification refuses a carrier whose datum is not a number, or
/// whose radius or semi-minor axis is not positive, before it reaches
/// an arena (the carried row's measurement: `IntervalNotForward` or an
/// escalated endpoint or span predicate), so no body can carry one to
/// check 1. The read is asked of the carrier directly, so that it names
/// each such datum the day a door stops refusing it — and so that the
/// kinds the body row cannot mint (a line, a spiric) are covered.
#[test]
fn the_carrier_datum_read_names_every_datum_that_describes_no_curve() {
    use crate::validate::{DatumVerdict as V, analytic_datum_verdicts, poisoned_curve_datums};
    use geom::ConventionEnd::Lower;
    use geom::ConventionMeasure::Value;
    use geom::Curve3;
    use geom::CurveDatum as D;
    let nan = f64::NAN;
    let inf = f64::INFINITY;
    let o = Point3::new(0.0, 0.0, 0.0);
    let z = Vec3::unit_z();
    let x = Vec3::unit_x();
    let zero = Vec3::new(0.0, 0.0, 0.0);
    let circle = |center, axis, radius, u_ref| Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    };
    let spiric = |minor_radius: f64, offset: f64| Curve3::Spiric {
        center: o,
        axis: z,
        u_ref: x,
        major_radius: 2.0,
        minor_radius,
        offset,
    };
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let verdict = |c: &Curve3<f64>| {
        analytic_datum_verdicts(poisoned_curve_datums(c), c.representability_margins(band))
    };
    type Row = (&'static str, Curve3<f64>, Option<V<D>>);
    let cases: Vec<Row> = vec![
        ("honest line", Curve3::Line { origin: o, dir: x }, None),
        (
            "line, zero dir",
            Curve3::Line {
                origin: o,
                dir: zero,
            },
            Some(V::Poisoned(vec![D::Dir])),
        ),
        (
            "line, NaN origin and infinite dir",
            Curve3::Line {
                origin: Point3::new(nan, 0.0, 0.0),
                dir: Vec3::new(inf, 0.0, 0.0),
            },
            Some(V::Poisoned(vec![D::Origin, D::Dir])),
        ),
        ("honest circle", circle(o, z, 1.0, x), None),
        (
            "circle, NaN radius",
            circle(o, z, nan, x),
            Some(V::Poisoned(vec![D::Radius])),
        ),
        (
            "circle, infinite center",
            circle(Point3::new(0.0, inf, 0.0), z, 1.0, x),
            Some(V::Poisoned(vec![D::Center])),
        ),
        (
            "circle, zero u_ref",
            circle(o, z, 1.0, zero),
            Some(V::Poisoned(vec![D::URef])),
        ),
        (
            "circle, zero radius",
            circle(o, z, 0.0, x),
            Some(V::Unrepresentable(D::Radius, Value, Lower)),
        ),
        (
            "circle, negative radius",
            circle(o, z, -1.0, x),
            Some(V::Unrepresentable(D::Radius, Value, Lower)),
        ),
        (
            "ellipse, zero minor",
            Curve3::Ellipse {
                center: o,
                axis: z,
                major: 1.0,
                minor: 0.0,
                u_ref: x,
            },
            Some(V::Unrepresentable(D::Minor, Value, Lower)),
        ),
        ("honest spiric", spiric(0.5, 0.3), None),
        (
            "spiric, NaN offset",
            spiric(0.5, nan),
            Some(V::Poisoned(vec![D::Offset])),
        ),
        (
            "spiric, zero tube",
            spiric(0.0, 0.3),
            Some(V::Unrepresentable(D::MinorRadius, Value, Lower)),
        ),
        (
            "finite NURBS carrier",
            line_net(Point3::new(1.0, 0.0, 0.0)),
            None,
        ),
        (
            "NURBS carrier, infinite control point",
            line_net(Point3::new(inf, 0.0, 0.0)),
            Some(V::Poisoned(vec![D::Control])),
        ),
        (
            "NURBS carrier, NaN control point",
            line_net(Point3::new(0.0, nan, 0.0)),
            Some(V::Poisoned(vec![D::Control])),
        ),
    ];
    for (name, carrier, expected) in cases {
        assert_eq!(verdict(&carrier), expected, "{name}, at f64");
        let lifted: Curve3<geom_core::Interval> =
            carrier.map_scalar(<geom_core::Interval as geom_core::Real>::from_f64);
        assert_eq!(
            analytic_datum_verdicts(
                poisoned_curve_datums(&lifted),
                lifted.representability_margins(band)
            ),
            expected,
            "{name}, at Interval"
        );
    }
}

/// A two-point linear NURBS carrier from the origin to `end`.
fn line_net(end: Point3<f64>) -> geom::Curve3<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    geom::Curve3::Nurbs(std::sync::Arc::new(
        geom::NurbsCurve3::new(kv, vec![Point3::new(0.0, 0.0, 0.0), end], vec![1.0; 2]).unwrap(),
    ))
}

/// **The carrier poison read reads every scalar**: each scalar of each
/// analytic kind ([`geom::test_support::analytic_curves`]), made NaN
/// alone, is named by [`crate::validate::poisoned_curve_datums`] as the
/// datum the walk ([`geom::Curve3::data`]) says owns it. The walk's
/// scalars must be exactly the builder's, so a walk that drops a field
/// reds on the count, and a fold that skips a datum reds on that
/// datum's rows.
#[test]
fn the_carrier_poison_read_reads_every_scalar() {
    use crate::validate::poisoned_curve_datums;
    let base = geom::test_support::scalar_base();
    for kind in geom::test_support::analytic_curves() {
        let at_rest = (kind.build)(&base);
        assert!(
            poisoned_curve_datums(&at_rest).is_empty(),
            "{at_rest:?}: a finite carrier has no poisoned datum"
        );
        let geom::CurveData::Analytic(data) = at_rest.data() else {
            panic!("{at_rest:?}: an analytic kind reads as analytic data")
        };
        let walked: Vec<(geom::CurveDatum, f64)> = data
            .into_iter()
            .flat_map(|(datum, value)| value.scalars().map(move |x| (datum, x)))
            .collect();
        assert_eq!(
            walked.iter().map(|&(_, x)| x).collect::<Vec<_>>(),
            base[..kind.scalars],
            "{at_rest:?}: the walk yields the builder's scalars, in its order"
        );
        for (i, &(datum, _)) in walked.iter().enumerate() {
            let mut poisoned = base.clone();
            poisoned[i] = f64::NAN;
            assert_eq!(
                poisoned_curve_datums(&(kind.build)(&poisoned)),
                vec![datum],
                "{at_rest:?}: check 1 missed a NaN in {} (scalar {i})",
                datum.name()
            );
        }
    }
}

/// **The frame margins' lever is the kind's radius — not 1, not its
/// square, and for an ellipse the LARGER semi-axis magnitude,
/// whichever field stores it.** Every row sits a frame deviation `δ`
/// on one side of the line at the true lever and on the other side at
/// a wrong one, so each wrong lever reddens a row:
///
/// - at `r = 4`, `δ = ε/2` refuses (`δ·r = 2ε`) and would pass at a
///   lever of 1 (`ε/2`);
/// - at `r = 4`, `δ = ε/8` passes (`δ·r = ε/2`) and would refuse at a
///   lever of `r²` (`2ε`);
/// - an ellipse stored `major = 1, minor = 4`: `δ = ε/2` refuses at
///   the lever 4 and would pass at the stored `major`; and stored
///   `major = −4, minor = 1` (a negative semi-axis) the same, where a
///   signed `max` would read the lever as 1.
///
/// Surfaces (a cylinder's `u_ref` length) and carriers (a circle's and
/// an ellipse's `axis` length), each at `f64` and at the interval
/// scalar.
#[test]
fn the_frame_lever_is_the_kinds_radius() {
    use geom::ConventionEnd::Upper;
    use geom::ConventionMeasure::Length;
    use geom::Curve3;
    use geom::CurveDatum as CD;
    use geom::SurfaceDatum as SD;
    use geom_core::Interval;
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let eps = band.zero();
    let o = Point3::new(0.0, 0.0, 0.0);
    let long = |d: f64| 1.0 + d;
    let cylinder = |r: f64, d: f64| Surface::Cylinder {
        origin: o,
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::new(long(d), 0.0, 0.0),
    };
    let circle = |r: f64, d: f64| Curve3::Circle {
        center: o,
        axis: Vec3::new(0.0, 0.0, long(d)),
        radius: r,
        u_ref: Vec3::unit_x(),
    };
    let ellipse = |major: f64, minor: f64, d: f64| Curve3::Ellipse {
        center: o,
        axis: Vec3::new(0.0, 0.0, long(d)),
        major,
        minor,
        u_ref: Vec3::unit_x(),
    };
    let surface_verdict = |s: &Surface<f64>| {
        let lifted: Surface<Interval> = s.map_scalar(<Interval as geom_core::Real>::from_f64);
        let at = |v: Option<crate::validate::DatumVerdict<SD>>| v;
        (
            at(crate::validate::analytic_datum_verdicts(
                crate::validate::poisoned_datums(s),
                s.representability_margins(band),
            )),
            at(crate::validate::analytic_datum_verdicts(
                crate::validate::poisoned_datums(&lifted),
                lifted.representability_margins(band),
            )),
        )
    };
    let curve_verdict = |c: &Curve3<f64>| {
        let lifted: Curve3<Interval> = c.map_scalar(<Interval as geom_core::Real>::from_f64);
        (
            crate::validate::analytic_datum_verdicts(
                crate::validate::poisoned_curve_datums(c),
                c.representability_margins(band),
            ),
            crate::validate::analytic_datum_verdicts(
                crate::validate::poisoned_curve_datums(&lifted),
                lifted.representability_margins(band),
            ),
        )
    };
    use crate::validate::DatumVerdict as V;
    let s_out = Some(V::Unrepresentable(SD::URef, Length, Upper));
    let c_out = Some(V::Unrepresentable(CD::Axis, Length, Upper));
    for (name, got, want) in [
        (
            "cylinder r = 4, u_ref eps/2 long",
            surface_verdict(&cylinder(4.0, eps / 2.0)),
            s_out,
        ),
        (
            "cylinder r = 4, u_ref eps/8 long",
            surface_verdict(&cylinder(4.0, eps / 8.0)),
            None,
        ),
    ] {
        assert_eq!(got, (want.clone(), want), "{name}: (f64, Interval)");
    }
    for (name, got, want) in [
        (
            "circle r = 4, axis eps/2 long",
            curve_verdict(&circle(4.0, eps / 2.0)),
            c_out.clone(),
        ),
        (
            "circle r = 4, axis eps/8 long",
            curve_verdict(&circle(4.0, eps / 8.0)),
            None,
        ),
        (
            "ellipse major 1 < minor 4, axis eps/2 long",
            curve_verdict(&ellipse(1.0, 4.0, eps / 2.0)),
            c_out.clone(),
        ),
        (
            "ellipse major 1 < minor 4, axis eps/8 long",
            curve_verdict(&ellipse(1.0, 4.0, eps / 8.0)),
            None,
        ),
    ] {
        assert_eq!(got, (want.clone(), want), "{name}: (f64, Interval)");
    }
    // A negative semi-axis is refused on its VALUE first, whatever the
    // frame; the frame's lever for it is asked directly.
    let neg = ellipse(-4.0, 1.0, eps / 2.0);
    let margins = neg.representability_margins(band);
    assert!(
        margins.iter().any(|m| m.datum == CD::Axis
            && m.measure == Length
            && m.end == Upper
            && m.margin < 0.0),
        "the axis at |major| = 4 is off by 2 eps, and a signed max would read the lever as 1: \
         {margins:?}"
    );
}

/// **An elliptic arc's perimeter lever is `|Δ|` times its larger
/// semi-axis MAGNITUDE** (`loop_winding::conic_segment_term`, read by
/// the merge's role assigner, check 6 and the boolean join's
/// `ring_run_ccw`), on the two ellipses the mint certifies with a
/// stored order the lever must not trust: `minor > major`, and a
/// negative `major` with its `u_ref` flipped (the D-2 table). A lever
/// of `|Δ|·major` reads the first too short, a signed `max` the second;
/// either is a lower bound on the arc length, which overstates the
/// metered width.
#[test]
fn the_elliptic_lever_is_the_larger_semi_axis_magnitude() {
    use geom::Curve3;
    let tol = Tol::witness();
    let pi = core::f64::consts::PI;
    let x = Vec3::unit_x();
    for (name, major, minor, u_ref, reach) in [
        ("minor > major", 0.5, 0.7, -x, 0.7),
        ("negative major, u_ref flipped", -0.5, 0.3, x, 0.5),
    ] {
        let carrier = Curve3::Ellipse {
            center: Point3::new(0.5, 0.0, 0.0),
            axis: Vec3::unit_z(),
            major,
            minor,
            u_ref,
        };
        let (body, edge) = pillow_with_carrier(carrier, 0.0, pi, tol)
            .unwrap_or_else(|e| panic!("{name}: the mint certifies it: {e:?}"));
        let curve = body
            .get_curve_geom(body.get_edge(edge).unwrap().curve)
            .and_then(crate::null::CurveGeom::certified)
            .unwrap();
        let (_, lever) = crate::loop_winding::conic_segment_term(curve, true).unwrap();
        assert_eq!(lever, pi * reach, "{name}: the lever is |Δ| times {reach}");
    }
}

#[test]
fn wrong_cache_at_rest_is_rejected_by_tier3() {
    let tol = Tol::witness();
    // Certified body; then swap one edge's stored curve for a certified
    // curve of DIFFERENT data (the scaffolding circle at a far point).
    // Attachment can't see it (raw arenas); tier 3's re-certification
    // must.
    let (mut body, split) = coplanar_pillow(tol);
    let curve_key = body.get_edge(split.edge).unwrap().curve;
    *body.curves.get_mut(curve_key).unwrap() =
        crate::null::CurveGeom::Certified(test_curve(Point3::new(50.0, 0.0, 0.0), tol));
    assert_eq!(validate(&body), Ok(()), "structurally still coherent");
    let errs = validate_geometric(&body, tol).unwrap_err();
    assert!(
        errs.iter().any(|e| matches!(
            e,
            ValidationError::EdgeCertification { edge, .. } if *edge == split.edge
        )),
        "{errs:?}"
    );
}

#[test]
fn offset_plane_at_rest_fails_planar_residuals() {
    let tol = Tol::witness();
    // Move a face's stored plane 100·ε off its vertices: the
    // Newell-cache re-check (tier 3, check 3) reports every vertex of
    // that face.
    let (mut body, split) = coplanar_pillow(tol);
    let eps = tol.eps();
    let surface_key = body.get_face(split.face).unwrap().surface;
    *body.surfaces.get_mut(surface_key).unwrap() = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 100.0 * eps),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let errs = validate_geometric(&body, tol).unwrap_err();
    let off_plane = errs
        .iter()
        .filter(|e| matches!(e, ValidationError::PlanarFaceResidual { face, .. } if *face == split.face))
        .count();
    assert_eq!(off_plane, 2, "both pillow vertices reported: {errs:?}");
}

#[test]
fn sliver_dihedral_at_rest_is_rejected() {
    let tol = Tol::witness();
    // Tilt one face's plane by the run-scaled sliver angle 3ε: both
    // MappedCurve attachments were legal (conventional descriptions
    // carry no dihedral requirement), but at rest the wedge is
    // indeterminate — the material wedge-angle predicate escalates.
    let (mut body, split) = coplanar_pillow(tol);
    let eps = tol.eps();
    let theta = 3.0 * eps;
    let surface_key = body.get_face(split.face).unwrap().surface;
    *body.surfaces.get_mut(surface_key).unwrap() = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, theta.sin(), theta.cos()),
        u_ref: Vec3::unit_x(),
    };
    let errs = validate_geometric(&body, tol).unwrap_err();
    assert!(
        errs.iter().any(|e| matches!(
            e,
            ValidationError::SliverDihedral {
                check: crate::validate::WedgeCheck::Dihedral,
                ..
            }
        )),
        "{errs:?}"
    );
}

#[test]
fn dangling_description_is_a_tier1_error() {
    let tol = Tol::witness();
    // An Intersection description whose surface key is ripped out of
    // the arena: the geometry-to-geometry reference check (tier 1,
    // pass 1) fires — alongside the face's own dangling reference.
    let (mut body, split) = coplanar_pillow(tol);
    let s_plus = body.get_face(split.face).unwrap().surface;
    let seed_face = body.face_of_half_edge(split.he_plus).unwrap();
    let s_seed = body.get_face(seed_face).unwrap().surface;
    let mut spec =
        EdgeCurveSpec::line_between(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    spec.description = geom_brep::EdgeDescriptionSpec::Intersection {
        s1: s_seed,
        s2: s_plus,
        witness: Point3::new(0.5, 0.0, 0.0),
    };
    // NB: coincident planes would fail transversality — so tilt the
    // split face definitively first (a legal corner), then upgrade.
    *body.surfaces.get_mut(s_plus).unwrap() = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::unit_y(),
        u_ref: Vec3::unit_x(),
    };
    body.set_edge_curve(split.edge, spec, tol).unwrap();
    // Rip the referenced surface out (raw removal past the hygiene
    // guard).
    body.surfaces.remove(s_plus);
    let errs = validate(&body).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::DanglingDescription { .. })),
        "{errs:?}"
    );
}

#[test]
fn description_references_keep_a_surface_alive() {
    let tol = Tol::witness();
    // A surface referenced ONLY by an edge description is not orphaned:
    // the hygiene guard refuses to remove it and tier 1 does not report
    // OrphanGeometry.
    let (mut body, split) = coplanar_pillow(tol);
    let s_plus = body.get_face(split.face).unwrap().surface;
    let seed_face = body.face_of_half_edge(split.he_plus).unwrap();
    let s_seed = body.get_face(seed_face).unwrap().surface;
    // Make the corner genuine, then describe the edge intrinsically.
    *body.surfaces.get_mut(s_plus).unwrap() = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::unit_y(),
        u_ref: Vec3::unit_x(),
    };
    let mut spec =
        EdgeCurveSpec::line_between(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    spec.description = geom_brep::EdgeDescriptionSpec::Intersection {
        s1: s_seed,
        s2: s_plus,
        witness: Point3::new(0.5, 0.0, 0.0),
    };
    body.set_edge_curve(split.edge, spec, tol).unwrap();
    // Repoint the split face to a NEW surface: the old one is now
    // referenced only by the description — and must survive.
    // Lifts both refusals: the stranded description keeping the old surface alive is the row.
    body.set_face_surface_stranding_for_tests(
        split.face,
        FaceSurface::New {
            surface: Surface::Plane {
                origin: Point3::new(0.0, 0.0, 0.0),
                normal: Vec3::unit_y(),
                u_ref: Vec3::unit_x(),
            },
            sense: true,
        },
    )
    .unwrap();
    assert!(body.get_surface(s_plus).is_some(), "kept alive");
    assert_eq!(validate(&body), Ok(()), "no OrphanGeometry");
    // (Tier 3 now reports the adjacency incoherence — the description
    // names a surface that is no longer the face's — which is exactly
    // the loud trail this state should leave.)
    let errs = validate_geometric(&body, tol).unwrap_err();
    assert!(
        errs.iter().any(|e| matches!(
            e,
            ValidationError::DescriptionNotAdjacent { edge } if *edge == split.edge
        )),
        "{errs:?}"
    );
}

/// **The ruling's own figure**: two kissing cylinders with one side cut
/// away, the smallest SOLID that carries a cusp edge.
///
/// Cross-section in `z = const`: the crescent between two internally
/// tangent circles — inner centre `(0, 1)` radius 1, outer centre
/// `(0, 2)` radius 2, kissing at the origin — cut by the plane `x = 0`
/// so the material is the `x ≥ 0` lip only and the body stays a
/// manifold (the two-lipped form is the doubled cusp, which is F2's
/// coincident-distinct-edges class, not one edge). Extruded along `z`
/// from 0 to 1, that is a triangular prism's topology exactly: three
/// walls (inner cylinder, the flat cut, outer cylinder) and two caps.
///
/// The cusp edge is the vertical line at the kissing point, `ev[0]`:
/// its two faces are tangent there, and their material sides oppose —
/// the inner wall's material is OUTSIDE its cylinder (`sense: false`),
/// the outer wall's INSIDE its own. Every other edge is a definite
/// corner.
pub(crate) fn cusp_prism(tol: Tol) -> crate::fixtures::RawPrism {
    let mut p = crate::fixtures::raw_prism(3, tol);
    // Cross-section corners, in the winding the fixture's caps expect
    // (counterclockwise from +z): the kiss, the outer circle's far
    // point, the inner circle's far point.
    let xy = [(0.0, 0.0), (0.0, 4.0), (0.0, 2.0)];
    for (i, (x, y)) in xy.into_iter().enumerate() {
        for (v, z) in [(p.t[i], 1.0), (p.u[i], 0.0)] {
            let point = p.body.get_vertex(v).unwrap().point;
            *p.body.points.get_mut(point).unwrap() = Point3::new(x, y, z);
        }
    }
    let inner = Surface::Cylinder {
        origin: Point3::new(0.0, 1.0, 0.0),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let outer = Surface::Cylinder {
        origin: Point3::new(0.0, 2.0, 0.0),
        axis: Vec3::unit_z(),
        radius: 2.0,
        u_ref: Vec3::unit_x(),
    };
    let flat = Surface::Plane {
        origin: Point3::new(0.0, 2.0, 0.0),
        normal: -Vec3::unit_x(),
        u_ref: Vec3::unit_y(),
    };
    let cap_top = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 1.0),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let cap_bottom = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: -Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    for (face, surface) in [
        (p.face_top, cap_top),
        (p.face_bottom, cap_bottom),
        (p.face_side[0], outer),
        (p.face_side[1], flat),
        (p.face_side[2], inner),
    ] {
        p.body
            .set_face_surface(
                face,
                FaceSurface::New {
                    surface,
                    sense: true,
                },
            )
            .unwrap();
    }
    // The inner wall's material is OUTSIDE its cylinder, so its outward
    // normal is the chart normal reversed — the one sense bit this
    // body needs, and the reason the kissing edge is a cusp rather
    // than a seam.
    p.body.set_face_sense(p.face_side[2], false).unwrap();
    for (e, kind) in [
        (p.et[0], Carrier::Arc(2.0)),
        (p.et[1], Carrier::Segment),
        (p.et[2], Carrier::Arc(1.0)),
        (p.eb[0], Carrier::Arc(2.0)),
        (p.eb[1], Carrier::Segment),
        (p.eb[2], Carrier::Arc(1.0)),
        (p.ev[0], Carrier::Segment),
        (p.ev[1], Carrier::Segment),
        (p.ev[2], Carrier::Segment),
    ] {
        let spec = edge_spec(&p.body, e, kind);
        p.body.set_edge_curve(e, spec, tol).unwrap();
    }
    p
}

/// Which carrier an edge of [`cusp_prism`] takes: the straight ones
/// (the three vertical meridians and the cut face's side) or an arc of
/// one of the two kissing circles, traversed through `x > 0`.
#[derive(Clone, Copy)]
enum Carrier {
    Segment,
    Arc(f64),
}

/// The carrier and description for one edge of [`cusp_prism`], read off
/// the body so the forward contract (increasing parameter runs
/// `he_plus`) holds whichever way the fixture wound it.
fn edge_spec(body: &Body<f64>, edge: crate::entity::EdgeKey, kind: Carrier) -> EdgeCurveSpec<f64> {
    let he = body.get_edge(edge).unwrap().he_plus;
    let start_v = body.get_half_edge(he).unwrap().start;
    let end_v = body.half_edge_end(he).unwrap();
    let p0 = *body
        .get_point(body.get_vertex(start_v).unwrap().point)
        .unwrap();
    let p1 = *body
        .get_point(body.get_vertex(end_v).unwrap().point)
        .unwrap();
    let (s1, s2) = adjacent_surfaces(body, edge);
    let (carrier, t0, t1) = match kind {
        Carrier::Segment => {
            let len = p0.distance(p1);
            (
                geom::Curve3::Line {
                    origin: p0,
                    dir: (p1 - p0) / len,
                },
                0.0,
                len,
            )
        }
        // θ = 0 is placed at the start point and the axis is chosen so
        // that `v_ref = axis × u_ref` points at +x: the half turn from
        // θ = 0 to θ = π is then the lip's arc, never its mirror.
        Carrier::Arc(radius) => {
            let center = Point3::new(0.0, radius, p0.z);
            let u_ref = (p0 - center) / radius;
            let axis = if u_ref.y < 0.0 {
                Vec3::unit_z()
            } else {
                -Vec3::unit_z()
            };
            (
                geom::Curve3::Circle {
                    center,
                    axis,
                    radius,
                    u_ref,
                },
                0.0,
                std::f64::consts::PI,
            )
        }
    };
    let witness = carrier.eval(0.5 * (t0 + t1));
    // The kissing edge is the one whose two faces are the two
    // cylinders; every other edge here is a definite corner.
    let description = if adjacent_are_the_two_cylinders(body, edge) {
        geom_brep::EdgeDescriptionSpec::TangentIntersection { s1, s2, witness }
    } else {
        geom_brep::EdgeDescriptionSpec::Intersection { s1, s2, witness }
    };
    EdgeCurveSpec {
        description,
        carrier,
        param_start: t0,
        param_end: t1,
    }
}

fn adjacent_surfaces(
    body: &Body<f64>,
    edge: crate::entity::EdgeKey,
) -> (geom_brep::SurfaceKey, geom_brep::SurfaceKey) {
    let e = body.get_edge(edge).unwrap();
    let face_of = |he| body.face_of_half_edge(he).unwrap();
    (
        body.get_face(face_of(e.he_plus)).unwrap().surface,
        body.get_face(face_of(e.he_minus)).unwrap().surface,
    )
}

fn adjacent_are_the_two_cylinders(body: &Body<f64>, edge: crate::entity::EdgeKey) -> bool {
    let (s1, s2) = adjacent_surfaces(body, edge);
    [s1, s2]
        .iter()
        .all(|&k| matches!(body.get_surface(k), Some(geom::Surface::Cylinder { .. })))
}

// ------------------------------------------------------------------
// D1's material-wedge verdict table (the #131 ruling), one row per
// arm. Every row that can be red-first is: the arms that refuse are
// pinned on bodies that validated clean before the arm existed, and
// the two legal arms are pinned green on the same fixtures.
// ------------------------------------------------------------------

/// **Row: transverse, legal at the θ = ε/r margin.** The cusp prism's
/// other eight edges are definite corners — cylinder against plane,
/// plane against plane, cylinder against cap — and none of them earns
/// a wedge refusal: the whole body is tier-3 clean.
#[test]
fn transverse_wedges_stay_legal_and_earn_no_wedge_verdict() {
    let tol = Tol::witness();
    let p = cusp_prism(tol);
    assert_eq!(p.body.edges().count(), 9);
    assert_eq!(validate_geometric(&p.body, tol), Ok(()));
}

/// **Row: wedge π, legal.** Two coplanar faces whose material sides
/// AGREE continue one another across the seam — the legal smooth case,
/// unchanged by the material arm.
///
/// **Row: the lamina, refused.** The same geometry with one face's
/// material side flipped is a zero-thickness sheet: the sides oppose
/// and the jets osculate exactly (one plane against another). It
/// validated clean before this arm existed — the unsigned dihedral
/// pass cannot tell it from the seam above, and that is the whole
/// content of "unsigned" — and now refuses per edge: the jets
/// osculate, so the tangency is not jet-determinate.
#[test]
fn the_seam_is_legal_and_the_same_geometry_flipped_is_a_lamina() {
    let tol = Tol::witness();
    let (body, split) = coplanar_pillow(tol);
    assert_eq!(validate_geometric(&body, tol), Ok(()));
    let flipped = body.flipped_face_sense_for_tests(split.face).unwrap();
    let errs = validate_geometric(&flipped, tol).unwrap_err();
    assert!(
        errs.iter()
            .all(|e| matches!(e, ValidationError::LaminaWedge { .. })),
        "{errs:?}"
    );
    assert_eq!(errs.len(), 2, "one per edge of the digon: {errs:?}");
}

/// **Row: wedge 0, legal because jet-determinate.** The ruling's own
/// figure — two kissing cylinders with one side cut away — meets along
/// one shared edge whose faces' outward normals oppose and whose
/// κ_rel is definite (radii 1 and 2): the tangency is determined by
/// the body, exactly as a π seam's is, and the body is tier-3 clean
/// with nothing declared. The kiss edge's contact mark is `Tangent`:
/// check 4 judged a jet-determinate tangency there rather than
/// exempting the edge. The mark alone does not say which END — a π
/// seam is marked the same — so the row reads the end off the same
/// sign chain the arm uses ([`material_end`]).
#[test]
fn a_jet_determinate_cusp_is_legal_at_rest_with_nothing_declared() {
    let tol = Tol::witness();
    let p = cusp_prism(tol);
    let kiss = kiss_edge(&p);
    assert_eq!(material_end(&p.body, kiss), MaterialWedge::Cusp);
    assert_eq!(validate_geometric(&p.body, tol), Ok(()));
    let marks = crate::validate::contact_marks(&p.body, tol).expect("the cusp prism is valid");
    assert_eq!(
        marks.get(kiss),
        Some(&crate::validate::ContactMark::Tangent)
    );
}

/// **Row: a transverse corner beside a legal cusp still owes its
/// intrinsic description.** A definitely-transverse edge of the cusp
/// prism, stored in a declared conventional form, refuses
/// `TransverseNotIntrinsic` and nothing else: the legal cusp beside it
/// contributes no verdict of its own.
#[test]
fn a_transverse_corner_beside_a_legal_cusp_still_refuses_its_conventional_form() {
    let tol = Tol::witness();
    let mut p = cusp_prism(tol);
    // ev[1]: the vertical meridian where the outer wall meets the flat
    // cut — a definite corner — re-stored as a line at rest in the flat
    // face's chart.
    let corner = p.ev[1];
    let he = p.body.get_edge(corner).unwrap().he_plus;
    let at = |v| {
        *p.body
            .get_point(p.body.get_vertex(v).unwrap().point)
            .unwrap()
    };
    let start = at(p.body.get_half_edge(he).unwrap().start);
    let end = at(p.body.half_edge_end(he).unwrap());
    let flat_chart = p.body.get_face(p.face_side[1]).unwrap().surface;
    let spec = EdgeCurveSpec::line_between(start, end).at_rest_in_chart(flat_chart, false);
    p.body.set_edge_curve(corner, spec, tol).unwrap();
    assert_eq!(
        validate_geometric(&p.body, tol).unwrap_err(),
        vec![ValidationError::TransverseNotIntrinsic { edge: corner }],
        "the corner's demand refuses; the cusp beside it adds nothing"
    );
}
/// **Rows: wedge 0 ↔ wedge 2π under `revert`.** Reverting negates
/// every face's outward normal at once, which negates the material
/// κ_rel — so the same body, same keys, reads as the knife slit, and
/// the arm's verdict is the mirror row: legal together or not at all.
///
/// **Why the reverted body is not asserted wholly green**: `revert`
/// bounds the COMPLEMENTARY volume, so tier 3's positive-volume
/// invariant refuses every reverted bounded solid. That is a fact
/// about `revert`, not about cusps, and the cube control row below is
/// the evidence. What the wedge arm owes is that it contributes
/// nothing to the reverted body's verdict — the slit is legal on the
/// cusp's terms — pinned here on arenas that are key-for-key the
/// source's, which is what "bit-faithfully" buys. The validator no
/// longer names the end it saw, so the row reads it off the arm's own
/// sign chain ([`material_end`]).
#[test]
fn revert_maps_the_legal_cusp_to_the_legal_slit() {
    let tol = Tol::witness();
    let p = cusp_prism(tol);
    let kiss = kiss_edge(&p);
    let reverted = p.body.revert().unwrap();
    assert_eq!(validate(&reverted), Ok(()));
    assert_eq!(material_end(&p.body, kiss), MaterialWedge::Cusp);
    assert_eq!(
        material_end(&reverted, kiss),
        MaterialWedge::Slit,
        "the cusp's revert image is the slit"
    );
    assert_eq!(
        validate_geometric(&reverted, tol).unwrap_err(),
        vec![ValidationError::NegativeVolume {
            solid: reverted.solids().next().expect("one solid").0
        }],
        "the wedge arm contributes nothing to the slit's verdict"
    );
    // That residue is `revert`'s own ratified posture — a reverted
    // body is tier-2 currency and never tier-3, failing exactly
    // `NegativeVolume` (`crate::revert` module docs, pinned on a real
    // cube by the M3 PR 1 acceptance row) — so it says nothing about
    // this body's wedges.
    // Bit-faithful: revert is an involution, so the pair really is one
    // body read two ways.
    let back = reverted.revert().unwrap();
    assert_eq!(
        crate::fixtures::deep_snapshot(&back),
        crate::fixtures::deep_snapshot(&p.body)
    );
}

/// **Row: the second-order band, all three outcomes**, on one family
/// where only κ_rel moves: two cylinders kissing along the y axis with
/// opposed material sides, the outer radius chosen to put the jet
/// margin definitely outside the band, exactly at zero, and inside the
/// band.
///
/// - determinate (radii 1 and 2) — the wedge is decided and legal, so
///   the only verdict left is the prefer-intrinsic demand on the
///   conventional line both edges carry;
/// - osculating (radii 1 and 1) — conformal along the locus, the
///   lamina the material arm does not admit;
/// - in-band (κ_rel = 6ε on a unit arm, so the sagitta margin is 3ε
///   at every CI ε row) — the honest escalation, naming
///   `tangent_second_order`, and NOT a refusal: ε-tightening escalates
///   an edge, it never flips a valid body to invalid.
#[test]
fn the_second_order_band_has_three_outcomes_and_they_are_three_answers() {
    let tol = Tol::witness();
    let eps = tol.get().eps;
    let (determinate, [seg, split]) = kissing_cylinder_pillow(tol, 2.0);
    assert_eq!(
        determinate,
        vec![
            ValidationError::TangentNotIntrinsic { edge: seg },
            ValidationError::TangentNotIntrinsic { edge: split },
        ]
    );
    let (osculating, _) = kissing_cylinder_pillow(tol, 1.0);
    assert!(
        osculating
            .iter()
            .all(|e| matches!(e, ValidationError::LaminaWedge { .. })),
        "{osculating:?}"
    );
    let (in_band, _) = kissing_cylinder_pillow(tol, 1.0 / (1.0 - 6.0 * eps));
    assert!(
        in_band.iter().all(|e| matches!(
            e,
            ValidationError::SliverDihedral {
                check: crate::validate::WedgeCheck::SecondOrder,
                cause: Indeterminate {
                    predicate: Some("tangent_second_order"),
                    ..
                },
                ..
            }
        )),
        "{in_band:?}"
    );
}

/// The 3′ pass judges a wedge end exactly as tier 3 does: the local
/// battery reads no contact record, so a jet-determinate cusp passes
/// 3′ with no records, and a curve record naming its edge — which the
/// census certifies on the jet schedule — changes nothing about it.
#[test]
fn the_pseudomanifold_gate_judges_a_cusp_as_tier_3_does() {
    let tol = Tol::witness();
    let p = cusp_prism(tol);
    assert_eq!(
        crate::validate::validate_pseudomanifold(
            &p.body,
            &crate::boolean::ContactRecords::default(),
            tol
        ),
        Ok(())
    );
    let mut records = crate::boolean::ContactRecords::default();
    records.curves.push(crate::boolean::CurveContact {
        face_a: p.face_side[0],
        face_b: p.face_side[2],
        witness: kiss_edge(&p),
    });
    assert_eq!(
        crate::validate::validate_pseudomanifold(&p.body, &records, tol),
        Ok(())
    );
}

/// The kissing edge of a [`cusp_prism`]: the vertical meridian at the
/// tangency, between the two cylinder walls.
fn kiss_edge(p: &crate::fixtures::RawPrism) -> crate::entity::EdgeKey {
    p.ev[0]
}

/// Which END of the wedge range `edge` sits at, read at its mid sample
/// through the sign chain check 4's material arm uses: the jet's κ_rel
/// signed into the plus face's outward frame, positive the cusp and
/// negative the slit. For an edge whose faces' material sides oppose
/// and whose κ_rel is definite — the caller's fixture guarantees both.
fn material_end(body: &Body<f64>, edge: crate::entity::EdgeKey) -> MaterialWedge {
    let e = body.get_edge(edge).unwrap();
    let curve = body
        .get_curve_geom(e.curve)
        .and_then(crate::CurveGeom::certified)
        .unwrap();
    let face = |he| body.get_face(body.face_of_half_edge(he).unwrap()).unwrap();
    let (plus, minus) = (face(e.he_plus), face(e.he_minus));
    let t = curve.sample_param(geom_brep::CERT_SAMPLES / 2);
    let (point, tau) = curve.carrier().ders1(t);
    let jet = geom_brep::tangent_jet(
        body.get_surface(plus.surface).unwrap(),
        body.get_surface(minus.surface).unwrap(),
        point,
        tau,
    );
    let signed = geom_brep::material_kappa_rel(jet.kappa_rel, plus.sense);
    assert!(signed != 0.0, "a definite κ_rel is the fixture's premise");
    if signed > 0.0 {
        MaterialWedge::Cusp
    } else {
        MaterialWedge::Slit
    }
}

/// The tier-3 verdict on a digon pillow whose two faces are cylinders
/// kissing along the shared chord — radius 1 against `r2` — with the
/// second face's material side flipped so the pair is the wedge-0/2π
/// arm. The body is deliberately degenerate (zero-area faces): what it
/// is for is the SECOND-ORDER band, which needs only two tangent
/// surfaces and an edge between them.
fn kissing_cylinder_pillow(
    tol: Tol,
    r2: f64,
) -> (Vec<ValidationError>, [crate::entity::EdgeKey; 2]) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(0.0, 1.0, 0.0),
            tol,
        )
        .unwrap();
    let cylinder = |radius: f64| Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, radius),
        axis: Vec3::unit_y(),
        radius,
        u_ref: Vec3::unit_x(),
    };
    let split = body
        .mef(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            EdgeCurveSpec::line_between(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)),
            FaceSurface::New {
                surface: cylinder(r2),
                sense: true,
            },
            tol,
        )
        .unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: cylinder(1.0),
            sense: true,
        },
    )
    .unwrap();
    let chart = body.get_face(split.face).unwrap().surface;
    for e in [seg.edge, split.edge] {
        let spec =
            EdgeCurveSpec::line_between(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0))
                .at_rest_in_chart(chart, false);
        body.set_edge_curve(e, spec, tol).unwrap();
    }
    let flipped = body.flipped_face_sense_for_tests(split.face).unwrap();
    (
        validate_geometric(&flipped, tol).unwrap_err(),
        [seg.edge, split.edge],
    )
}

/// **The material arm's fold, state by state** — including the two
/// states no certified geometry is known to reach.
///
/// `material_arm_outcome` is the whole of check 4's material verdict:
/// the sample loop accumulates flags, and this fold turns them into the
/// ONE outcome the edge earns. Two of its states are the reason it is a
/// separate function at all:
///
/// - **the pairing SPLIT** (`aligned == opposed`): different samples
///   along one edge disagreed about which way the material faces, or
///   no sample classified;
/// - **the end SPLIT** (`side_mixed`): the pairing agreed, the jet was
///   determinate, and different samples still called different ends.
///
/// Silence there would validate a wedge end CLEAN on an edge whose own
/// samples contradicted one another. They escalate, and because no fixture can force them,
/// calling the fold directly is the only way to pin that. The row also
/// pins the exclusivity the outcome type exists to guarantee: no input
/// yields both a lamina and a wedge.
#[test]
fn material_arm_split_states_escalate_and_the_outcomes_stay_exclusive() {
    use MaterialArmOutcome as O;
    let cusp = Some(MaterialWedge::Cusp);

    // Aligned at every sample: the legal seam, whatever the jet says
    // (a seam's legality is a first-order fact).
    assert_eq!(
        material_arm_outcome(true, false, true, None, false),
        O::Wedge(MaterialWedge::Seam)
    );
    assert_eq!(
        material_arm_outcome(true, false, false, None, false),
        O::Wedge(MaterialWedge::Seam)
    );
    // Opposed at every sample, jet determinate, one end: that end.
    assert_eq!(
        material_arm_outcome(false, true, true, cusp, false),
        O::Wedge(MaterialWedge::Cusp)
    );
    assert_eq!(
        material_arm_outcome(false, true, true, Some(MaterialWedge::Slit), false),
        O::Wedge(MaterialWedge::Slit)
    );
    // Opposed with a collapsed jet: the lamina refusal — and NOT a
    // wedge, which is why no edge can earn both refusals.
    assert_eq!(
        material_arm_outcome(false, true, false, None, false),
        O::Lamina
    );
    assert_eq!(
        material_arm_outcome(false, true, false, cusp, false),
        O::Lamina
    );
    // The two split states escalate, each naming the predicate whose
    // per-sample verdicts disagreed.
    assert_eq!(
        material_arm_outcome(false, true, true, cusp, true),
        O::Split {
            predicate: "material_cusp_side"
        },
        "the end split must escalate, never validate as the end it saw first"
    );
    assert_eq!(
        material_arm_outcome(false, true, true, None, false),
        O::Split {
            predicate: "material_cusp_side"
        },
        "opposed, determinate, and no end at all is the same non-verdict"
    );
    assert_eq!(
        material_arm_outcome(false, false, true, cusp, false),
        O::Split {
            predicate: "material_wedge_side"
        },
        "a pairing that split across samples must escalate"
    );
    assert_eq!(
        material_arm_outcome(true, true, true, cusp, false),
        O::Split {
            predicate: "material_wedge_side"
        },
        "no sample classified at all: also a non-verdict, never silence"
    );

    // Exhaustive: over every flag combination, the fold is total and
    // never returns a legal-looking wedge on a split input.
    for aligned in [false, true] {
        for opposed in [false, true] {
            for determinate in [false, true] {
                for mixed in [false, true] {
                    for side in [None, cusp, Some(MaterialWedge::Slit)] {
                        let out = material_arm_outcome(aligned, opposed, determinate, side, mixed);
                        if aligned == opposed {
                            assert!(matches!(out, O::Split { .. }), "{aligned} {opposed}");
                        }
                        if mixed && aligned != opposed && opposed && determinate {
                            assert!(matches!(out, O::Split { .. }), "a mixed end never resolves");
                        }
                    }
                }
            }
        }
    }
}

/// **What each material outcome EMITS** — the fold's other half.
///
/// `material_arm_outcome` decides what the edge is; this table decides
/// what the validator says about it, and the two are separate functions
/// because both have states no fixture can reach. Pinning the fold
/// alone would leave the escalation PUSH unpinned: a mutation dropping
/// `Split` on the floor would keep every row green while restoring
/// exactly the silence item 6 removed.
///
/// Every settled wedge is legal — the two ends included, because the
/// fold hands one out only over a jet-determinate tangency — and the
/// lamina and both splits are what refuse.
#[test]
fn material_arm_error_table() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let edge = crate::fixtures::raw_prism(3, Tol::witness())
        .body
        .edges()
        .next()
        .expect("the fixture has edges")
        .0;
    let err = |outcome| material_arm_error(outcome, edge, band);

    // No outcome at all: exempt by kind, or already escalated.
    assert!(err(None).is_none());
    // Every settled wedge is legal on its own terms.
    for wedge in [
        MaterialWedge::Seam,
        MaterialWedge::Transverse,
        MaterialWedge::Cusp,
        MaterialWedge::Slit,
    ] {
        assert!(
            err(Some(MaterialArmOutcome::Wedge(wedge))).is_none(),
            "{wedge:?} is legal at rest"
        );
    }
    // The lamina refuses.
    assert!(
        matches!(
            err(Some(MaterialArmOutcome::Lamina)),
            Some(ValidationError::LaminaWedge { .. })
        ),
        "a lamina is refused"
    );
    // Both splits escalate, carrying the predicate that split.
    for predicate in ["material_wedge_side", "material_cusp_side"] {
        match err(Some(MaterialArmOutcome::Split { predicate })) {
            Some(ValidationError::SliverDihedral { check, cause, .. }) => {
                assert_eq!(check, crate::validate::WedgeCheck::MaterialSide);
                assert_eq!(cause.predicate, Some(predicate));
            }
            other => panic!("a split must escalate, got {other:?}"),
        }
    }
}

// ---------------------------------------------------------------------
// Check 7's subject — the per-SOLID volume sign, and what tier 3
// deliberately does NOT read about a solid's shells.
//
// The bodies below are hand-assembled from cubes because that is the
// shape the states have: a second solid beside the first, and several
// shells filed under one solid. Both are states the public verbs
// produce (a boolean leaves multi-solid results; graft-onto fuses two
// disjoint bodies into one solid) and neither is reachable from this
// crate's own doors, which is what the raw arenas are for here
// (module docs).
// ---------------------------------------------------------------------

/// A cube of side `s` at `origin`, into `body` as its own solid —
/// mirrored in x when `inside_out`, which is the orientation flip
/// `review_m2_pr7` pins as invisible to tiers 1 and 2.
fn cube_solid(body: &mut Body<f64>, origin: (f64, f64, f64), s: f64, inside_out: bool, tol: Tol) {
    let (ox, oy, oz) = origin;
    crate::test_support_fixtures::cube_into(
        body,
        move |x, y, z| {
            let x = if inside_out { 1.0 - x } else { x };
            Point3::new(ox + x * s, oy + y * s, oz + z * s)
        },
        tol,
    );
}

/// The solid keys of `body`, in arena order.
fn solids_of(body: &Body<f64>) -> Vec<crate::entity::SolidKey> {
    body.solids().map(|(k, _)| k).collect()
}

/// The axis-aligned extent of `shell`'s stored vertex positions.
/// A read of the geometry rather than of the fixture's literals: a
/// cube placed somewhere else moves this.
fn shell_extent(body: &Body<f64>, shell: crate::entity::ShellKey) -> (Point3<f64>, Point3<f64>) {
    let mut lo = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut hi = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for &face in &body.get_shell(shell).expect("a live shell").faces {
        let f = body.get_face(face).expect("a live face");
        for &lp in core::iter::once(&f.outer).chain(f.rings.iter()) {
            let crate::entity::LoopBoundary::Cycle { first } =
                body.get_loop(lp).expect("a live loop").boundary
            else {
                panic!("a cube's loops are cycles")
            };
            for he in body.loop_cycle(first).expect("a closed cycle") {
                let v = body.get_half_edge(he).expect("a live half-edge").start;
                let p = *body
                    .get_point(body.get_vertex(v).expect("a live vertex").point)
                    .expect("a live point");
                lo = lo.min(p);
                hi = hi.max(p);
            }
        }
    }
    (lo, hi)
}

/// `inner`'s extent lies strictly inside `outer`'s, componentwise.
fn strictly_within(inner: (Point3<f64>, Point3<f64>), outer: (Point3<f64>, Point3<f64>)) -> bool {
    let axes = Point3::to_array;
    let (ilo, ihi) = (axes(inner.0), axes(inner.1));
    let (olo, ohi) = (axes(outer.0), axes(outer.1));
    (0..3).all(|i| ilo[i] > olo[i] && ihi[i] < ohi[i])
}

/// **An inside-out part beside a larger ordinary one certifies when
/// only the body TOTAL is pinned.**
///
/// The body's total signed volume is `1 - 0.125 = +0.875`, so a check
/// reading the sum sees nothing; the small solid is inside-out and its
/// own volume is `-0.125`. Red without the per-solid subject: the
/// runtime value that makes the assertion false is `errs` coming back
/// empty, which is what a body-total read produces here.
#[test]
fn an_inside_out_part_beside_an_ordinary_one_refuses_by_name() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    cube_solid(&mut body, (0.0, 0.0, 0.0), 1.0, false, tol);
    cube_solid(&mut body, (5.0, 0.0, 0.0), 0.5, true, tol);
    let [ordinary, reverted] = solids_of(&body)[..] else {
        panic!("two cubes are two solids");
    };
    let total = crate::mass_properties(&body, tol).expect("both cubes measure");
    assert!(
        total.volume > 0.0,
        "the fixture's point is a POSITIVE total: {}",
        total.volume
    );
    let errs = validate_geometric(&body, tol).unwrap_err();
    assert_eq!(
        errs,
        vec![ValidationError::NegativeVolume { solid: reverted }],
        "check 7 names the inside-out solid and says nothing about the other \
         one, which {ordinary:?} is not in"
    );
}

/// **The false-refusal direction, on the same shape**: two ordinary
/// solids in one body certify. A per-solid check that read a
/// neighbour's faces, or that refused a body for holding two solids at
/// all, reds here.
#[test]
fn two_ordinary_solids_in_one_body_certify() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    cube_solid(&mut body, (0.0, 0.0, 0.0), 1.0, false, tol);
    cube_solid(&mut body, (5.0, 0.0, 0.0), 0.5, false, tol);
    assert_eq!(solids_of(&body).len(), 2, "two cubes are two solids");
    assert_eq!(validate_geometric(&body, tol), Ok(()));
}

/// **The number a multi-solid body's certificate door continues to is
/// the whole-body measurement, bit for bit.**
///
/// Check 7's subject is a SOLID, so the tier-3′ door's certificate is
/// the per-solid walks assembled into face-arena order, and the number
/// a caller asks of it is that assembly continued to the reporting
/// target. The continuation carries exactly one promise — that it is
/// the number [`crate::mass_properties`] would give — and this row is
/// what reds if the two ever diverge. (That no further arena-wide read
/// is taken is a COUNT, and it lives where a quadrature body can be
/// built: `sweep`'s `tcost_k3_certificate`.)
#[test]
fn a_multi_solid_certificate_is_the_whole_body_measurement() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    cube_solid(&mut body, (0.0, 0.0, 0.0), 1.0, false, tol);
    cube_solid(&mut body, (5.0, 0.0, 0.0), 0.5, false, tol);
    assert_eq!(solids_of(&body).len(), 2, "two cubes are two solids");
    let certified =
        crate::validate_pseudomanifold_certificate(&body, &crate::ContactRecords::default(), tol)
            .expect("two ordinary cubes certify")
            .refine_to_target()
            .expect("two ordinary cubes measure");
    let measured = crate::mass_properties(&body, tol).expect("the cubes measure");
    assert_eq!(
        (
            certified.volume.to_bits(),
            certified.surface_area.to_bits(),
            certified.volume_pad.to_bits(),
            certified.area_pad.to_bits()
        ),
        (
            measured.volume.to_bits(),
            measured.surface_area.to_bits(),
            measured.volume_pad.to_bits(),
            measured.area_pad.to_bits()
        ),
        "the door's certificate, continued, is the measurement door's"
    );
}

/// **A `_structural` certificate is continued at a scalar that may not
/// certify, to the closed form's own number.**
///
/// `SignCertificate`'s continuation is generic over `Decide`, so the
/// certificate the no-lane tier-3′ door hands back at a
/// [`geom_core::Dual64`] can be asked for its number: it carries no lane,
/// every face is closed-form and finished at round 0, and the
/// continuation is the fold of what it holds. That fold must BE
/// [`crate::mass_properties_structural`] on the same body — value and
/// derivative, compared through `Debug` (which renders every `f64`
/// round-trip exactly) — over two solids, so the assembly re-orders
/// real parts.
#[test]
fn a_structural_certificate_continues_at_a_dual_to_the_closed_form() {
    use geom_core::Dual64;
    use geom_core::Real as _;
    let tol = Tol::witness();
    let mut body = Body::<Dual64>::new();
    for (ox, s) in [(0.0, 1.0), (5.0, 0.5)] {
        crate::test_support_fixtures::cube_into(
            &mut body,
            move |x, y, z| {
                Point3::new(
                    Dual64::from_f64(ox + x * s),
                    Dual64::from_f64(y * s),
                    Dual64::from_f64(z * s),
                )
            },
            tol,
        );
    }
    assert_eq!(body.solids().count(), 2, "two boxes are two solids");
    let continued = crate::validate_pseudomanifold_certificate_structural(
        &body,
        &crate::ContactRecords::default(),
        tol,
    )
    .expect("two closed-form boxes pass the no-lane tier-3′ door at a dual")
    .refine_to_target()
    .expect("a closed-form certificate's continuation cannot refuse");
    let measured =
        crate::mass_properties_structural(&body, tol).expect("the closed form measures two boxes");
    assert_eq!(
        format!("{continued:?}"),
        format!("{measured:?}"),
        "the continued no-lane certificate is the closed form's own measurement"
    );
}

/// **A solid holding SEVERAL outer boundaries certifies, and that is
/// the ratified posture rather than a gap** — the executable form of
/// `work/atrest/one-solid-holding-two-outer-shells-is-what-five-kernel-doors-produce`.
///
/// One solid, three shells: the outer cube, a cavity wall inside it,
/// and an island inside that cavity — the hollow-operand subtraction's
/// shape
/// (`work/fuse/subtract-of-a-hollow-operand-files-the-island-under-one-solid`).
/// Two of those shells enclose definitely-positive volume.
///
/// Four doors produce this state on purpose — `graft onto`, the
/// boolean coplanar split (which asserts three shells under one solid
/// in so many words), `subtract`, and the editor's placed union — and
/// how many material components a product should have is answered
/// one layer up, as `editor_core`'s `CheckId::Connectedness` finding
/// against an authored expectation. So tier 3 admits it, and this row
/// reds if a count-level refusal is ever put back at this tier.
///
/// The NESTING is read too, by check 10, and admits it on the merits:
/// inside the island the shells wind `+1 - 1 + 1 = 1`, so the island is
/// material and every region winds 0 or 1.
#[test]
fn a_solid_holding_several_outer_shells_still_certifies() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    cube_solid(&mut body, (0.0, 0.0, 0.0), 1.0, false, tol);
    cube_solid(&mut body, (0.2, 0.2, 0.2), 0.5, true, tol);
    cube_solid(&mut body, (0.3, 0.3, 0.3), 0.2, false, tol);
    let [keeper, cavity, island] = solids_of(&body)[..] else {
        panic!("three cubes are three solids");
    };
    let shell_of = |body: &Body<f64>, solid| {
        let shells = body.shells_of_solid(solid).expect("a live solid");
        assert_eq!(shells.len(), 1, "a cube arrives as one shell");
        shells[0]
    };
    let (wall, void, isle) = (
        shell_of(&body, keeper),
        shell_of(&body, cavity),
        shell_of(&body, island),
    );
    refile_shells(&mut body, cavity, keeper);
    refile_shells(&mut body, island, keeper);
    assert_eq!(
        body.shells_of_solid(keeper).expect("the one solid").len(),
        3,
        "one solid, three shells"
    );

    // The premise, read off the body rather than off the fixture's
    // literals: TWO of the three shells classify `Outer`, and the
    // island is nested in the cavity that is nested in the wall. Move
    // the island cube beside the others and both reads move with it,
    // which is what stops this row from passing while no longer being
    // about several outer boundaries.
    let roles: std::collections::BTreeMap<_, _> = crate::classify_shells(&body, tol)
        .expect("the cubes classify")
        .into_iter()
        .map(|c| (c.shell, c.role))
        .collect();
    assert_eq!(
        (roles[&wall], roles[&void], roles[&isle]),
        (
            crate::ShellRole::Outer,
            crate::ShellRole::Void,
            crate::ShellRole::Outer
        ),
        "two outer boundaries and one cavity, filed under one solid: {roles:?}"
    );
    let (wall_box, void_box, isle_box) = (
        shell_extent(&body, wall),
        shell_extent(&body, void),
        shell_extent(&body, isle),
    );
    assert!(
        strictly_within(isle_box, void_box) && strictly_within(void_box, wall_box),
        "the island sits inside the cavity inside the wall: \
         {isle_box:?} in {void_box:?} in {wall_box:?}"
    );

    assert!(
        crate::mass_properties(&body, tol)
            .expect("the cubes measure")
            .volume
            > 0.0,
        "check 7's subject is this solid, and its volume is positive"
    );
    assert_eq!(validate_geometric(&body, tol), Ok(()));
}

/// **Check 7 sums a solid's whole boundary, cavity included**: a solid
/// with one outer shell and one void certifies. A per-solid walk that
/// read only one of a solid's shells would red here — the outer cube
/// alone measures `+1`, the cavity wall alone `-0.125`, and only their
/// sum is the solid's `+0.875`.
#[test]
fn a_solid_with_a_genuine_cavity_certifies() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    cube_solid(&mut body, (0.0, 0.0, 0.0), 1.0, false, tol);
    cube_solid(&mut body, (0.2, 0.2, 0.2), 0.5, true, tol);
    let [keeper, cavity] = solids_of(&body)[..] else {
        panic!("two cubes are two solids");
    };
    refile_shells(&mut body, cavity, keeper);
    assert_eq!(
        body.shells_of_solid(keeper).expect("the one solid").len(),
        2,
        "one solid, two shells"
    );
    assert_eq!(validate_geometric(&body, tol), Ok(()));
}

/// **Check 10's per-shell contribution, both arms.** The point-in-solid
/// walk over ONE shell's faces answers whether a point is in the
/// material that shell alone bounds. For the outer wall that is its
/// inside; for the cavity wall, whose faces point into the cavity, it is
/// the cavity's COMPLEMENT — so a point in the cavity reads `Out` and a
/// point anywhere else reads `In`, the far one included (where the walk
/// may cross nothing and read the selection's negative volume at
/// infinity). Check 10's `[In] - [Void]` is exactly this: `-1` in the
/// cavity, `0` outside it. A walk that read a void selection as its
/// ENCLOSED region would flip both void rows below.
#[test]
fn a_shell_selection_reads_the_material_that_shell_alone_bounds() {
    use crate::boolean::SolidContainment::{In, Out};
    use crate::boolean::solid_contain::{SolidFaces, point_in_solid_faces};
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    cube_solid(&mut body, (0.0, 0.0, 0.0), 1.0, false, tol);
    cube_solid(&mut body, (0.2, 0.2, 0.2), 0.5, true, tol);
    let [keeper, cavity] = solids_of(&body)[..] else {
        panic!("two cubes are two solids");
    };
    let (wall, void) = (
        body.shells_of_solid(keeper).expect("live")[0],
        body.shells_of_solid(cavity).expect("live")[0],
    );
    refile_shells(&mut body, cavity, keeper);
    let roles: std::collections::BTreeMap<_, _> = crate::classify_shells(&body, tol)
        .expect("the cubes classify")
        .into_iter()
        .map(|c| (c.shell, c.role))
        .collect();
    assert_eq!(
        (roles[&wall], roles[&void]),
        (crate::ShellRole::Outer, crate::ShellRole::Void)
    );
    let band = geom_core::Band::linear(tol).expect("a band");
    let probe = |shell, p: Point3<f64>| {
        let sel = SolidFaces::of_shell(&body, shell).expect("a shell selection");
        point_in_solid_faces(&body, &sel, p, band, tol).expect("the walk answers")
    };
    let in_cavity = Point3::new(0.43, 0.41, 0.47);
    let in_wall = Point3::new(0.1, 0.13, 0.11);
    let beside = Point3::new(2.0, 0.37, 0.41);
    let far = Point3::new(100.0, 90.0, 80.0);
    assert_eq!(
        [
            probe(wall, in_cavity),
            probe(wall, in_wall),
            probe(wall, beside),
            probe(wall, far)
        ],
        [In, In, Out, Out],
        "the outer wall's material is its inside"
    );
    assert_eq!(
        [
            probe(void, in_cavity),
            probe(void, in_wall),
            probe(void, beside),
            probe(void, far)
        ],
        [Out, In, In, In],
        "the cavity wall's material is everything outside the cavity"
    );
    // The far point's answer is the AT-INFINITY arm's: the schedule's
    // first ray (+x) from it crosses nothing, so the walk reads the
    // selection's own signed volume — negative for the cavity wall,
    // hence `In`. Pinned by the verdict log, so a far point that a ray
    // happened to reach through a face could not pass for it.
    let bracket = geom_core::k_stats::Bracket::open();
    assert_eq!(probe(void, far), In);
    let log = bracket.finish();
    assert!(
        log.verdicts
            .iter()
            .any(|v| v.predicate == "bool_point_in_solid_infinity"),
        "the far probe must be answered at infinity, got {:?}",
        log.verdicts.iter().map(|v| v.predicate).collect::<Vec<_>>()
    );
    assert_eq!(validate_geometric(&body, tol), Ok(()));
}

/// **Check 10 skips a witness where two shells TOUCH, and reads the
/// next one.** A unit cube hangs from the ceiling of a larger cube,
/// both `Outer` and under one solid, so the space inside the unit cube
/// winds `2` — and its top lies ON the larger cube's top. The first
/// vertex the check reads is on that face (asserted below, from the
/// body), where the walk answers `OnBoundary`; a check that stopped
/// there would be silent. The cube's lower vertices touch nothing, and
/// one of them refuses the body.
#[test]
fn check_10_reads_past_a_witness_where_two_shells_touch() {
    use crate::boolean::SolidContainment;
    use crate::boolean::solid_contain::{SolidFaces, point_in_solid_faces};
    let tol = Tol::witness();
    let mut body: Body<f64> =
        crate::test_support_fixtures::brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0), tol);
    let [outer_solid] = solids_of(&body)[..] else {
        panic!("a brick is one solid");
    };
    let outer = body.shells_of_solid(outer_solid).expect("live")[0];
    let inner_body: Body<f64> =
        crate::test_support_fixtures::brick((1.0, 2.0), (1.0, 2.0), (2.0, 3.0), tol);
    crate::graft_disjoint_all_onto_keyed(&mut body, &[outer_solid], &inner_body)
        .expect("the graft");
    let inner = *body
        .shells_of_solid(outer_solid)
        .expect("live")
        .iter()
        .find(|&&s| s != outer)
        .expect("the grafted cube");

    let band = geom_core::Band::linear(tol).expect("a band");
    let sel = SolidFaces::of_shell(&body, outer).expect("a selection");
    let first = crate::validate::shell_vertices(&body, inner)
        .next()
        .expect("the cube has vertices");
    assert_eq!(
        point_in_solid_faces(&body, &sel, first, band, tol).expect("the walk answers"),
        SolidContainment::OnBoundary,
        "the premise: the first witness {first:?} touches the larger cube"
    );
    assert_eq!(
        validate_geometric(&body, tol),
        Err(vec![ValidationError::ShellWinding {
            solid: outer_solid,
            shell: inner,
            winding: 1,
            bounded: 2,
        }])
    );
}
