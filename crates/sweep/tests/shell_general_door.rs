//! **A solid that is neither planar nor axial, offset every chart at
//! once.** The unit box under a spline cap
//! (`common::approx::box_with_spline_cap`): five planes and one
//! described NURBS, so `shell`'s ladder sends it to the general
//! simultaneous door (`topo::offset_surfaces_together`). Every chart
//! moves inward together; each side's edge with the cap is the section
//! of the moved side plane with the cap's offset fit, each edge between
//! two sides the two moved planes' line, and each corner the moved
//! planes' roots, so the offset box is its closed form at every ε.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::test_support::finished;
use topo::{Body, ReplaceFaceError, ShellError};

use crate::common::approx::box_with_spline_cap;

/// The wall thickness, in metres.
const T: f64 = 0.1;

/// Every chart of `body` moved by `t` into its material: along the
/// chart normal, turned by the face's sense.
fn inward(body: &Body<f64>, t: f64) -> Vec<topo::ChartMove<f64>> {
    body.faces()
        .map(|(k, f)| topo::ChartMove {
            faces: vec![k],
            distance: if f.sense { -t } else { t },
        })
        .collect()
}

/// Tier 3 refuses the moved body on the fitted cap's quadrature alone
/// (`work/quad/a-fitted-cap-cut-by-planes-has-a-sub-range-trim-image.md`).
fn only_the_fitted_caps_quadrature(refusals: &[topo::ValidationError]) -> bool {
    matches!(
        refusals,
        [topo::ValidationError::VolumeUncomputable {
            source: topo::MassPropsError::Face {
                source: geom_brep::PropsError::QuadratureUnsupported { what },
                ..
            },
            ..
        }] if what.starts_with("a General trim image whose carrier interval is not its own knot domain")
    )
}

/// **The flat spline cap moves with every side at once onto the offset
/// box.** Every corner is a corner of `[T, 2 − T]² × [T, 1 − T]`, the
/// cap wears its offset fit, its four edges are the side planes'
/// sections of it, and tier 3 refuses only the fitted cap's quadrature.
#[test]
fn a_spline_capped_box_moves_every_chart_onto_its_closed_form() {
    let eps = Tol::witness().eps();
    let (mut body, cap) = box_with_spline_cap(0.0);
    let moves = inward(&body, T);
    topo::offset_surfaces_together(&mut body, &moves, Tol::witness())
        .unwrap_or_else(|e| panic!("eps {eps:e}: every chart moves together: {e}"));
    let mut corners = 0;
    for (_, v) in body.vertices() {
        let p = *body.get_point(v.point).expect("a live vertex's point");
        let off = |c: f64, lo: f64, hi: f64| (c - lo).abs().min((c - hi).abs());
        let gap = off(p.x, T, 2.0 - T)
            .max(off(p.y, T, 2.0 - T))
            .max(off(p.z, T, 1.0 - T));
        assert!(
            gap <= eps,
            "eps {eps:e}: a corner at {p:?} is {gap:e} off the offset box"
        );
        corners += 1;
    }
    assert_eq!(corners, 8, "eps {eps:e}: the offset box has eight corners");
    let cap_key = body.get_face(cap).expect("the cap survives").surface;
    assert!(
        matches!(body.get_surface(cap_key), Some(geom::Surface::Approx(_))),
        "eps {eps:e}: the moved cap wears its offset fit"
    );
    let sections = body
        .edges()
        .filter_map(|(_, e)| {
            body.get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
        })
        .filter(|c| {
            matches!(
                *c.description(),
                geom_brep::EdgeDescription::Intersection { s1, s2, .. }
                    if s2 == cap_key
                        && matches!(body.get_surface(s1), Some(geom::Surface::Plane { .. }))
            ) && matches!(c.carrier(), geom::Curve3::Nurbs(_))
        })
        .count();
    assert_eq!(
        sections, 4,
        "eps {eps:e}: each cap edge is a moved side's section of the fit, plane first"
    );
    let refusals = topo::validate_geometric(&body, Tol::witness())
        .expect_err("the fitted cap's quadrature is not built");
    assert!(
        only_the_fitted_caps_quadrature(&refusals),
        "eps {eps:e}: expected only the fitted cap's quadrature refusal, got {refusals:?}"
    );
}

/// **A curved spline cap moves with its sides where both certificates
/// it meets hold.** The cap's middle control point is raised 0.01. At
/// ε = 1e-6 every chart moves, with every corner on the moved side
/// planes. Tighter, the cap's offset fit certifies at 1e-9, and the side
/// plane's section of it, which is no row of the fit, refuses at the
/// plane × NURBS certificate's limb 2
/// (`work/ssiedge/plane-nurbs-limb-two-refuses-a-non-row-section.md`).
/// At 1e-12 the fit stops short, at about 1.7e-10 m.
#[test]
fn a_curved_spline_cap_moves_with_its_sides_where_its_certificates_hold() {
    let eps = Tol::witness().eps();
    let (mut body, _) = box_with_spline_cap(0.01);
    let moves = inward(&body, T);
    let moved = topo::offset_surfaces_together(&mut body, &moves, Tol::witness());
    if eps < 1e-10 {
        assert!(
            matches!(
                moved,
                Err(ReplaceFaceError::Fit {
                    error: geom_brep::OffsetFitError::BudgetExhausted { .. },
                    ..
                })
            ),
            "eps {eps:e}: expected the cap's fit to exhaust its budget, got {moved:?}"
        );
        return;
    }
    if eps < 1e-7 {
        assert!(
            matches!(
                &moved,
                Err(ReplaceFaceError::Op {
                    error: topo::EulerOpError::RechartFalsifies {
                        error: geom_brep::CertifyError::PlaneNurbs(
                            geom_brep::PlaneNurbsRefusal::Limb {
                                limb: geom_brep::ssi::SsiLimb::HullSup,
                                ..
                            }
                        ),
                        ..
                    },
                    ..
                })
            ),
            "eps {eps:e}: expected the rim certificate's limb-2 refusal, got {moved:?}"
        );
        return;
    }
    moved.unwrap_or_else(|e| panic!("eps {eps:e}: every chart moves together: {e}"));
    let mut corners = 0;
    for (_, v) in body.vertices() {
        let p = *body.get_point(v.point).expect("a live vertex's point");
        let off = |c: f64| (c - T).abs().min((c - (2.0 - T)).abs());
        let gap = off(p.x).max(off(p.y));
        assert!(
            gap <= eps,
            "eps {eps:e}: a corner at {p:?} is {gap:e} off the moved sides"
        );
        corners += 1;
    }
    assert_eq!(corners, 8, "eps {eps:e}: the moved box has eight corners");
}

/// **The shell gets past the door and refuses at tier 3.** The sealed
/// shell builds the cavity through the general door and validates the
/// thin solid, which refuses on the cavity's fitted cap's quadrature,
/// as the moved body above does. The opened shell lifts the cavity's
/// fitted cap back onto the designated spline cap, which is an offset
/// of a fit, and that is not built.
#[test]
fn shelling_a_spline_capped_box_refuses_past_the_door() {
    let (body, cap) = box_with_spline_cap(0.0);
    let at_rest = finished("the spline-capped box", body, Tol::witness());
    let e = topo::shell(&at_rest, T, Tol::witness())
        .expect_err("the fitted cavity cap's quadrature is not built");
    let ShellError::NotValid { errors } = &e else {
        panic!("expected the thin solid's tier-3 refusal, got {e}");
    };
    assert!(
        only_the_fitted_caps_quadrature(errors),
        "expected only the fitted cap's quadrature refusal, got {e}"
    );
    let e = topo::shell_open(&at_rest, T, &[cap], Tol::witness())
        .expect_err("an offset of a fit is not built");
    assert!(
        matches!(
            &e,
            ShellError::Lift { face, error }
                if *face == cap
                    && matches!(
                        error.as_ref(),
                        ReplaceFaceError::Offset {
                            error: geom_brep::OffsetError::ApproxNesting,
                            ..
                        }
                    )
        ),
        "expected the lift to refuse offsetting the cavity's fit, got {e:?}"
    );
}
