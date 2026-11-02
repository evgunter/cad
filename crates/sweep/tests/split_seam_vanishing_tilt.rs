//! A plane through a cylinder's seam vertex, tilted `ψ` off the wall's
//! tangent plane there: the split returns no side its own validator
//! refuses.
//!
//! The unit cylinder (`0 ≤ z ≤ 1`) turned `φ` about its axis has a seam
//! vertex at `v = (cos φ, sin φ, 0)`. The plane through `v` with normal
//! `n = cos ψ·w + sin ψ·ẑ` (`w` the wall's normal at `v`), its stored
//! origin moved `L` along the rim's tangent, cuts the wall in an
//! ellipse whose principal vertex is `v` and whose semi-major axis is
//! `1/sin ψ`. At small `ψ` the material is right, but the ellipse's
//! chart image at `v` is read through its centre, about `1/sin ψ` away,
//! so tier 3 cannot close the wall's loop there
//! (`props_chart_junction_closed`). The split refuses such a side at its
//! at-rest gate, `SplitFinishError::ResultNotAtRest`, with the
//! validator's own finding and no tolerance lever: the input is
//! `ψ·1 m` off tangency, far outside the band, and the miss is the
//! carrier's.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::splitting::{SplitError, SplitFinishError, SplitPart, split};
use topo::{AtRestBody, ValidationError, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn cylinder(phi: f64) -> AtRestBody<f64> {
    let circle = vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)];
    let body = AtRestBody::validate(sweep::test_support::prism(circle, 1.0, tol()), tol())
        .expect("the unit cylinder is finished");
    let turn =
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), phi);
    let turned = topo::transform_rigid(&body, &geom_core::test_support::rigid(&turn, tol()), tol())
        .expect("a rigid turn");
    AtRestBody::validate(turned, tol()).expect("the turned cylinder is finished")
}

/// The findings the ellipse's centre-form rounding, amplified by its
/// semi-major axis `1/sin ψ`, reaches at rest, each ending as the
/// carrier's own limit and never with a lever:
///
/// - check 7, the wall's loop read not closed within the band at `v`
///   (`PropsCheck::Inventory`): "no way through yet";
/// - check 7, the wall's quadrature budget spent: loosening as the last
///   resort;
/// - check 1, a frame whose rounding the arm lifts past ε
///   (`UnrepresentableCurveDatum` on the ellipse's `u_ref`), which still
///   ends as a defect
///   (`work/restfront/tier-3s-frame-check-reads-a-section-ellipses-rounding-amplified-by-its-semi-axis.md`);
/// - check 9's pinch corner: the side ends with maximal faces, so its
///   two half-walls merge across the seam and the wall's loop passes
///   `v` twice, between the rim and the ellipse leaving it at a turn of
///   about `ψ`; read in the band it is undecided, and past it at
///   ε = 1e-12 the rounding reads it crossed, both ending as a defect
///   (`work/wirerest/a-split-merging-a-two-piece-wall-reads-its-seam-vertex-as-a-pinch-corner.md`).
pub(crate) fn carrier_resolution(e: &ValidationError) -> bool {
    use geom_brep::props::{PropsCheck, PropsError};
    use topo::MassPropsError;
    let msg = e.to_string();
    let ending = match e {
        ValidationError::VolumeUncomputable {
            source:
                MassPropsError::Face {
                    source:
                        PropsError::Escalated {
                            check: PropsCheck::Inventory,
                            ..
                        },
                    ..
                },
            ..
        } => msg.contains(geom_core::NOT_YET_ENDING),
        ValidationError::UnrepresentableCurveDatum {
            kind: geom::CurveKind::Ellipse,
            datum: geom::CurveDatum::URef,
            ..
        }
        | ValidationError::PinchCornerEscalated { .. }
        | ValidationError::PinchCornerCrossed { .. } => true,
        _ => false,
    };
    ending && !msg.contains("tighten") && !msg.contains("move the")
}

/// Every pose: `φ` (the seam's azimuth), `ψ` (the tilt), `L` (the stored
/// origin's offset along the rim's tangent, in the plane) and the
/// normal's sign. `ψ = 10⁻³` builds at every ε row; the smaller tilts
/// put the ellipse's semi-major axis at `10⁶`–`10⁷`.
#[test]
fn a_split_through_a_seam_vertex_returns_no_side_tier_three_refuses() {
    let mut answered = 0;
    let mut refused_at_rest = 0;
    for phi in [2.2_f64, 0.9] {
        let cyl = cylinder(phi);
        let (c, s) = (phi.cos(), phi.sin());
        let v = Point3::new(c, s, 0.0);
        let along = Vec3::new(-s, c, 0.0);
        for psi in [1e-7_f64, 1.1e-7, 1e-6, 1e-3] {
            for l in [0.0, 0.02, 0.1, 0.18, 0.5] {
                for sign in [1.0, -1.0] {
                    let pose = format!("φ {phi} ψ {psi:e} L {l} sign {sign}");
                    let n = Vec3::new(c * psi.cos(), s * psi.cos(), psi.sin()) * sign;
                    let plane = topo::test_support::split_plane(v + along * l, n, tol());
                    match split(&cyl, &plane, tol()) {
                        Ok(r) => {
                            answered += 1;
                            let mut volume = 0.0;
                            for (word, part) in [("above", &r.above), ("below", &r.below)] {
                                let SplitPart::Body(b) = part else { continue };
                                if let Err(errors) = validate_geometric(b, tol()) {
                                    panic!(
                                        "{pose}: the {word} side is returned but fails tier 3: {errors:?}"
                                    );
                                }
                                volume += mass_properties(
                                    b,
                                    tol(),
                                    topo::test_support::pinned_resolution(tol()),
                                )
                                .unwrap_or_else(|e| panic!("{pose}: {word} measures: {e}"))
                                .met()
                                .expect("the pinned resolution is met")
                                .volume;
                            }
                            assert!(
                                (volume - PI).abs() < 1e-6,
                                "{pose}: the sides hold the cylinder's π, not {volume}"
                            );
                        }
                        Err(SplitError::Finish(
                            ref err @ SplitFinishError::ResultNotAtRest { ref errors, .. },
                        )) => {
                            refused_at_rest += 1;
                            assert!(
                                !errors.is_empty() && errors.iter().all(carrier_resolution),
                                "{pose}: every finding is the ellipse's own rounding, ending \
                                 with no lever: {errors:?}"
                            );
                            let msg = err.to_string();
                            assert!(
                                msg.contains("did not pass the at-rest gate")
                                    && msg.ends_with(&errors[0].to_string()),
                                "{pose}: the refusal renders its first finding whole: {msg}"
                            );
                        }
                        Err(err) => {
                            assert!(
                                psi < 1e-3,
                                "{pose}: a tilt of 10⁻³ off the wall builds at every ε row: \
                                 {err:?}"
                            );
                            assert!(
                                !matches!(
                                    err,
                                    SplitError::Finish(
                                        SplitFinishError::ResultInvalid { .. }
                                            | SplitFinishError::ResultEscalated { .. }
                                    )
                                ),
                                "{pose}: a side the at-rest gate refuses on the carrier's \
                                 findings is `ResultNotAtRest`: {err:?}"
                            );
                        }
                    }
                }
            }
        }
    }
    assert!(answered >= 20, "the ψ = 10⁻³ poses answer: {answered}");
    // Measured: 26 at ε = 1e-9, 28 at 1e-12, none at 1e-6, where
    // the semi-axis's rounding stays inside ε (re-measured with each
    // split half ending with maximal faces: the same counts, the same
    // poses).
    if tol().eps() <= 1e-9 {
        assert!(
            refused_at_rest > 0,
            "the witness poses reach the at-rest gate at ε = {:e}",
            tol().eps()
        );
    }
    eprintln!("answered {answered}, refused at the at-rest gate {refused_at_rest}");
}
