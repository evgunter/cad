//! **A section chord's escalated pin is the input's only where its
//! carrier resolves the edge** ([`SectionEnds::refusal`]), in closed
//! form at ε = 1e-9, `K = 10`: the quarter of the unit circle about `z`,
//! and ellipses whose centre is stored `x` off the arc they serve, read
//! against the carrier's own resolution ([`geom::Curve3::resolution`]).

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

fn ends(reading: SectionEndReading) -> SectionEnds {
    SectionEnds { reading }
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

fn residual() -> CertCheck {
    CertCheck::SurfaceResidual {
        surface: SurfaceKey::default(),
    }
}

/// On a carrier that resolves the edge, an escalated end pin is the
/// input's, typed by how the table read the plane, at the start and the
/// end alike; a stood-in circle's surface residual is the plane read
/// square.
#[test]
fn a_resolved_carriers_pin_is_the_inputs_by_provenance() {
    let face = FaceKey::default();
    for check in [CertCheck::EndpointStart, CertCheck::EndpointEnd] {
        for reading in [SectionEndReading::OnPlane, SectionEndReading::SquareToAxis] {
            let got = ends(reading).refusal(face, &arc(), escalated(check), band());
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
    let got = ends(SectionEndReading::SquareToAxis).refusal(
        face,
        &arc(),
        escalated(residual()),
        band(),
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
/// far centre resolves its arc only to `u` times its distance, so the
/// pin is a kernel limitation exactly where that resolution, `K` times
/// over, passes ε, whichever reading of the plane, and its words name no
/// lever and no defect. Both sides of the line are read: `x = 1e4`
/// resolves (`ρ ≈ 7e-11`), `x = 3e4` does not (`ρ ≈ 2e-10`).
#[test]
fn an_unresolved_carriers_pin_is_a_kernel_limitation() {
    let face = FaceKey::default();
    let k = band().escalate() / band().zero();
    for x in [1e4, 3e4, 1e5, 1e6] {
        let spec = far_centre(x);
        let rho = spec
            .carrier
            .resolution(spec.param_start, spec.param_end)
            .unwrap();
        let unresolved = k * rho > EPS;
        assert_eq!(unresolved, x > 2e4, "x {x:e}: ρ {rho:e}");
        for reading in [SectionEndReading::OnPlane, SectionEndReading::SquareToAxis] {
            let got = ends(reading).refusal(
                face,
                &spec,
                escalated(CertCheck::EndpointStart),
                band(),
            );
            match (&got, unresolved) {
                (
                    SplitJoinError::SectionUnresolved {
                        check: CertCheck::EndpointStart,
                        diag,
                        resolution,
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

/// What no reading of the input explains keeps the gate's refusal: a
/// residual off the surfaces on the plane's own section where the carrier
/// resolves it, a definite miss and another check on a resolved and an
/// unresolved carrier alike, and any pin on a carrier with no bound (a
/// spiric). An unresolved residual is the carrier's limitation.
#[test]
fn what_no_reading_explains_stays_the_gates() {
    let face = FaceKey::default();
    let gates = |got: &SplitJoinError| matches!(got, SplitJoinError::Euler(_));
    let got = ends(SectionEndReading::OnPlane).refusal(face, &arc(), escalated(residual()), band());
    assert!(gates(&got), "own section's residual: {got:?}");
    let got = ends(SectionEndReading::OnPlane).refusal(
        face,
        &far_centre(1e6),
        escalated(residual()),
        band(),
    );
    assert!(
        matches!(got, SplitJoinError::SectionUnresolved { .. })
            && got
                .to_string()
                .starts_with("whether a section curve lies on the surfaces it is cut from"),
        "an unresolved residual: {got:?}"
    );
    let definite = || EulerOpError::Certification {
        error: CertifyError::ResidualExceeded {
            check: CertCheck::EndpointEnd,
            sample: 8,
            margin: MarginDiag::value(20.0 * EPS),
        },
    };
    for spec in [arc(), far_centre(1e6)] {
        for refusal in [definite(), escalated(CertCheck::WitnessMidpoint)] {
            let got = ends(SectionEndReading::SquareToAxis).refusal(face, &spec, refusal, band());
            assert!(gates(&got), "{got:?}");
        }
    }
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
    let got = ends(SectionEndReading::OnPlane).refusal(
        face,
        &spiric,
        escalated(CertCheck::EndpointEnd),
        band(),
    );
    assert!(gates(&got), "spiric: {got:?}");
}
