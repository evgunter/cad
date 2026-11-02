//! **A section chord's refused pin is the input's only where its
//! carrier resolves the edge** ([`section_pin_refusal`]), in closed form
//! at ε = 1e-9, `K = 10`: the quarter of the unit circle about `z`, and
//! ellipses whose centre is stored `x` off the arc they serve, read
//! against the carrier's own resolution
//! ([`geom_brep::carrier_resolution::resolution`]). And the reading a
//! chord carries is the one the section table made of the plane
//! ([`section_end_reading`]).

use super::*;
use geom_brep::{CertCheck, CertifyError};
use geom_core::MarginDiag;

const EPS: f64 = 1e-9;

fn band() -> Band {
    Band::new(EPS, 10.0 * EPS).unwrap()
}

/// The quarter arc of the unit circle about `z` in `z = 0`.
fn arc() -> EdgeCurveSpec<f64> {
    EdgeCurveSpec::arc_of_circle(
        geom::Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
        0.0,
        core::f64::consts::FRAC_PI_2,
    )
    .unwrap()
}

/// The arc's ends, where its vertices stand.
fn arc_ends() -> (Point3<f64>, Point3<f64>) {
    (Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0))
}

/// An arc about the major vertex of the ellipse of semi-axes `x` along
/// `x` and 1 along `y`, its centre stored at `(−x, 0, 0)`, so the arc
/// passes the origin: the far-centre ellipse of a plane nearly along a
/// cylinder's axis, `x` about `r/θ`.
fn far_centre(x: f64) -> EdgeCurveSpec<f64> {
    EdgeCurveSpec {
        carrier: geom::Curve3::Ellipse {
            center: Point3::new(-x, 0.0, 0.0),
            axis: Vec3::unit_z(),
            semi_u: x,
            semi_v: 1.0,
            u_ref: Vec3::unit_x(),
        },
        param_start: -1e-3,
        param_end: 1e-3,
        ..arc()
    }
}

/// The far-centre arc's ends as its carrier evaluates them.
fn far_ends(spec: &EdgeCurveSpec<f64>) -> (Point3<f64>, Point3<f64>) {
    (
        spec.carrier.eval(spec.param_start),
        spec.carrier.eval(spec.param_end),
    )
}

fn cause() -> Indeterminate {
    Indeterminate {
        margin: MarginDiag::value(1.2 * EPS),
        band: band(),
        predicate: Some("carrier_endpoint_end"),
        terminal_sliver: false,
    }
}

fn escalated(check: CertCheck) -> EulerOpError {
    EulerOpError::Certification {
        error: CertifyError::Escalated {
            check,
            sample: 8,
            cause: cause(),
        },
    }
}

fn past(margin: f64) -> EulerOpError {
    EulerOpError::Certification {
        error: CertifyError::ResidualExceeded {
            check: CertCheck::EndpointEnd,
            sample: 8,
            margin: MarginDiag::value(margin),
        },
    }
}

fn residual() -> CertCheck {
    CertCheck::SurfaceResidual {
        surface: SurfaceKey::default(),
    }
}

fn read(
    reading: SectionEndReading,
    spec: &EdgeCurveSpec<f64>,
    at: (Point3<f64>, Point3<f64>),
    refusal: EulerOpError,
) -> SplitJoinError {
    section_pin_refusal(reading, FaceKey::default(), (spec, at), refusal, band())
}

/// On a carrier that resolves the edge, an escalated end pin is the
/// input's, typed by how the table read the plane, at the start and the
/// end alike; a stood-in circle's surface residual is the plane read
/// square.
#[test]
fn a_resolved_carriers_pin_is_the_inputs_by_provenance() {
    for check in [CertCheck::EndpointStart, CertCheck::EndpointEnd] {
        for reading in [SectionEndReading::OnPlane, SectionEndReading::SquareToAxis] {
            let got = read(reading, &arc(), arc_ends(), escalated(check));
            assert!(
                matches!(
                    got,
                    SplitJoinError::SectionEndEscalated { reading: r, diag, .. }
                        if r == reading && diag == cause()
                ),
                "{check:?}, {reading:?}: {got:?}"
            );
        }
    }
    let got = read(
        SectionEndReading::SquareToAxis,
        &arc(),
        arc_ends(),
        escalated(residual()),
    );
    assert!(
        matches!(
            got,
            SplitJoinError::SectionEndEscalated {
                reading: SectionEndReading::SquareToAxis,
                ..
            }
        ),
        "stood-in residual: {got:?}"
    );
}

/// **The section's own rounding is never offered as the input's.** A
/// far centre resolves its arc only to `u` times its distance, so an
/// in-band pin is a kernel limitation exactly where that resolution, `K`
/// times over, passes ε, whichever reading of the plane, and its words
/// name no lever and no defect. Both sides of the line are read:
/// `x = 1e4` resolves (`ρ ≈ 7e-11`), `x = 3e4` does not (`ρ ≈ 2e-10`).
#[test]
fn an_unresolved_carriers_pin_is_a_kernel_limitation() {
    let k = band().escalate() / band().zero();
    for x in [1e4, 3e4, 1e5, 1e6] {
        let spec = far_centre(x);
        let rho = geom_brep::carrier_resolution::resolution(
            &spec.carrier,
            spec.param_start,
            spec.param_end,
        )
        .unwrap();
        let unresolved = k * rho > EPS;
        assert_eq!(unresolved, x > 2e4, "x {x:e}: ρ {rho:e}");
        for reading in [SectionEndReading::OnPlane, SectionEndReading::SquareToAxis] {
            let got = read(
                reading,
                &spec,
                far_ends(&spec),
                escalated(CertCheck::EndpointStart),
            );
            match (&got, unresolved) {
                (
                    SplitJoinError::SectionUnresolved {
                        miss:
                            SectionPinMiss::Undecided {
                                check: CertCheck::EndpointStart,
                                diag,
                                resolution,
                            },
                        ..
                    },
                    true,
                ) => {
                    assert_eq!(*diag, cause(), "x {x:e}");
                    assert_eq!(*resolution, MarginDiag::value(k * rho), "x {x:e}");
                    let text = got.to_string();
                    assert!(
                        text.starts_with(
                            "whether a section curve's end meets its vertex is undecided"
                        ) && text.contains(SECTION_UNRESOLVED_CAUSE)
                            && text.ends_with(geom_core::KERNEL_LIMIT_RECOURSE)
                            && !text.contains("Recourse: move")
                            && !text.contains(geom_core::KERNEL_DEFECT_ENDING),
                        "x {x:e}: {text}"
                    );
                }
                (SplitJoinError::SectionEndEscalated { reading: r, .. }, false) => {
                    assert_eq!(*r, reading, "x {x:e}");
                }
                _ => panic!("x {x:e}, {reading:?}, unresolved {unresolved}: {got:?}"),
            }
        }
    }
}

/// **A definite end miss within the carrier's resolution is the
/// carrier's**, read from the end the carrier evaluates against the
/// vertex: the far centre's vertex `3ε` off the end it serves is within
/// `ρ ≈ 7e-9` at `x = 1e6`, and the miss ends as a kernel limitation in
/// exactly its words; the same vertex `100ρ` off is past it, and keeps
/// the gate's refusal, as does any definite miss on a resolved carrier.
#[test]
fn a_definite_miss_within_the_resolution_is_the_carriers() {
    let spec = far_centre(1e6);
    let (start, end) = far_ends(&spec);
    let rho =
        geom_brep::carrier_resolution::resolution(&spec.carrier, spec.param_start, spec.param_end)
            .unwrap();
    let off = |d: f64| (start, end + Vec3::unit_z() * d);
    let got = read(
        SectionEndReading::OnPlane,
        &spec,
        off(3.0 * EPS),
        past(20.0 * EPS),
    );
    let want = format!(
        "a section curve's end misses its vertex: {} at sample 8 is {:e}, definitely past the \
         tolerance. {SECTION_WITHIN_CAUSE} {}",
        CertCheck::EndpointEnd,
        MarginDiag::value(20.0 * EPS),
        geom_core::KERNEL_LIMIT_RECOURSE
    );
    assert!(
        matches!(
            got,
            SplitJoinError::SectionUnresolved {
                miss: SectionPinMiss::Past {
                    check: CertCheck::EndpointEnd,
                    sample: 8,
                    ..
                },
                ..
            }
        ) && got.to_string() == want,
        "within: {got:?}: {got}"
    );
    for (what, spec, at) in [
        ("past the resolution", spec.clone(), off(100.0 * rho)),
        (
            "a resolved carrier",
            arc(),
            (arc_ends().0, arc_ends().1 + Vec3::unit_z() * (20.0 * EPS)),
        ),
    ] {
        let got = read(SectionEndReading::OnPlane, &spec, at, past(20.0 * EPS));
        assert!(matches!(got, SplitJoinError::Euler(_)), "{what}: {got:?}");
    }
}

/// What no reading of the input explains keeps the gate's refusal on a
/// carrier that resolves it: a residual off the surfaces on the plane's
/// own section, a witness missed, and a definite residual. On one that
/// does not, an in-band residual or witness is the carrier's limitation.
/// A pin whose margin nobody could read, one whose enclosure straddles
/// the zero band's edge (subdividing may decide it), and any pin on a
/// carrier with no bound (a spiric), keep the gate's.
#[test]
fn what_no_reading_explains_stays_the_gates() {
    let gates = |got: &SplitJoinError| matches!(got, SplitJoinError::Euler(_));
    let far = far_centre(1e6);
    for check in [residual(), CertCheck::WitnessMidpoint] {
        let got = read(
            SectionEndReading::OnPlane,
            &arc(),
            arc_ends(),
            escalated(check),
        );
        assert!(gates(&got), "{check:?} on a resolved carrier: {got:?}");
        let got = read(
            SectionEndReading::OnPlane,
            &far,
            far_ends(&far),
            escalated(check),
        );
        assert!(
            matches!(
                got,
                SplitJoinError::SectionUnresolved {
                    miss: SectionPinMiss::Undecided { check: c, .. },
                    ..
                } if c == check
            ),
            "{check:?} on an unresolved carrier: {got:?}"
        );
    }
    let definite_residual = EulerOpError::Certification {
        error: CertifyError::ResidualExceeded {
            check: residual(),
            sample: 8,
            margin: MarginDiag::value(20.0 * EPS),
        },
    };
    let got = read(
        SectionEndReading::SquareToAxis,
        &far,
        far_ends(&far),
        definite_residual,
    );
    assert!(gates(&got), "a definite residual: {got:?}");
    let unreadable = EulerOpError::Certification {
        error: CertifyError::Escalated {
            check: CertCheck::EndpointEnd,
            sample: 8,
            cause: Indeterminate {
                margin: MarginDiag::INVALID,
                ..cause()
            },
        },
    };
    let got = read(SectionEndReading::OnPlane, &far, far_ends(&far), unreadable);
    assert!(gates(&got), "an unreadable margin: {got:?}");
    let straddle = EulerOpError::Certification {
        error: CertifyError::Escalated {
            check: CertCheck::EndpointStart,
            sample: 0,
            cause: Indeterminate {
                margin: MarginDiag::enclosure(0.0, 2.0 * EPS),
                ..cause()
            },
        },
    };
    let got = read(SectionEndReading::OnPlane, &arc(), arc_ends(), straddle);
    assert!(gates(&got), "a straddling enclosure: {got:?}");
    let spiric = EdgeCurveSpec {
        carrier: geom::Curve3::Spiric {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
            major_radius: 2.0,
            minor_radius: 1.0,
            offset: 0.5,
        },
        ..arc()
    };
    let got = read(
        SectionEndReading::OnPlane,
        &spiric,
        arc_ends(),
        escalated(CertCheck::EndpointEnd),
    );
    assert!(gates(&got), "spiric: {got:?}");
}

/// **A chord carries the reading the section table made of the plane.**
/// A frustum's cone cut square to its axis is the axis-normal circle
/// stood in for the section, read square; cut clearly off square, the
/// plane's own ellipse.
#[test]
fn a_chord_carries_the_tables_reading_of_the_plane() {
    let cone = geom::Surface::Cone {
        apex: Point3::new(0.0, 0.0, 2.0),
        axis: Vec3::new(0.0, 0.0, -1.0),
        half_angle: 0.5_f64.atan(),
        u_ref: Vec3::unit_x(),
    };
    let reach = geom_brep::Reach::Measured {
        at: Point3::new(0.0, 0.0, 1.0),
        lever: 1.0,
    };
    for (tilt, want) in [
        (0.0, SectionEndReading::SquareToAxis),
        (0.3, SectionEndReading::OnPlane),
    ] {
        let n = Vec3::new(f64::sin(tilt), 0.0, f64::cos(tilt));
        let cut = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 1.0),
            normal: n,
            u_ref: Vec3::unit_y(),
        };
        let got = match section_case(
            FaceKey::default(),
            band(),
            &cut,
            &cone,
            SectionReach::Cone(reach.clone()),
        ) {
            Ok(SectionCase::Conic(c)) => section_end_reading(&SectionCurve::Conic(c)),
            other => panic!("tilt {tilt}: {:?}", other.err()),
        };
        assert_eq!(got, Some(want), "tilt {tilt}");
    }
}

/// **A torus's concentric circles are read square too.** The plane
/// square to a torus's axis is cut in two circles the table stands in for
/// it (`pt_axis_normal`), as a cylinder's rim is, so a chord on either
/// carries the square reading. The meridian circles of a plane through
/// the axis carry the plane's own: no pose reaches their pins
/// (`work/cleave/a-torus-meridian-circle-is-a-stood-in-section-read-on-its-own-plane.md`).
#[test]
fn a_torus_square_to_its_axis_is_read_square() {
    let torus = geom::Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_y(),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::unit_x(),
    };
    for (origin, normal, u_ref, want) in [
        (
            Point3::new(0.0, 0.1, 0.0),
            Vec3::unit_y(),
            Vec3::unit_z(),
            SectionEndReading::SquareToAxis,
        ),
        (
            Point3::origin(),
            Vec3::unit_z(),
            Vec3::unit_x(),
            SectionEndReading::OnPlane,
        ),
    ] {
        let cut = geom::Surface::Plane {
            origin,
            normal,
            u_ref,
        };
        match section_case(
            FaceKey::default(),
            band(),
            &cut,
            &torus,
            SectionReach::Torus(2.5),
        ) {
            Ok(SectionCase::Loops { loops, .. }) => {
                for curve in &loops {
                    assert_eq!(section_end_reading(curve), Some(want), "{normal:?}");
                }
            }
            other => panic!("{normal:?}: {:?}", other.err()),
        }
    }
}
