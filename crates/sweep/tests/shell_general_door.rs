//! **A solid that is neither planar nor axial, offset every chart at
//! once.** The unit box under a spline cap
//! (`common::approx::box_with_bumped_cap`): five planes and one
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

use crate::common::approx::box_with_bumped_cap;

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
    let (mut body, cap) = box_with_bumped_cap(0.0);
    let moves = inward(&body, T);
    topo::offset_surfaces_together(
        &mut body,
        &moves,
        crate::common::approx::band(),
        Tol::witness(),
    )
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
                geom_brep::EdgeDescription::Intersection { pair, .. }
                    if pair.other(cap_key).is_some_and(|side| {
                        matches!(body.get_surface(side), Some(geom::Surface::Plane { .. }))
                    })
            ) && matches!(c.carrier(), geom::Curve3::Nurbs(_))
        })
        .count();
    assert_eq!(
        sections, 4,
        "eps {eps:e}: each cap edge is a moved side's section of the fit"
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
/// ε = 1e-6 every chart moves: every corner lies on the moved side
/// planes, the four bottom corners on the moved floor, and the four top
/// corners `T` from the cap, within the fit's certified ε. Tighter, the cap's offset fit certifies at 1e-9, and the side
/// plane's section of it, which is no row of the fit, refuses at the
/// plane × NURBS certificate's limb 2: on its bound, or, once the
/// subdivision reaches a break past the band, on that break's value,
/// where the section's own foot is on the wall (3.5e-16 m there), so the
/// miss is its pcurve's (`ImageMiss`, about 4.4e-7 m at 1e-9)
/// (`work/ssiedge/plane-nurbs-limb-two-refuses-a-non-row-section.md`).
/// At 1e-12 the fit stops short, at the sample cap: 3.2795e-12 m on a
/// 45 x 48 grid.
#[test]
fn a_curved_spline_cap_moves_with_its_sides_where_its_certificates_hold() {
    // Pinned red since PR 4551: at ε = 1e-12 the cap's fit stops on (48, 48)
    // without a refusal the row lists
    // (`work/shell/a-curved-fitted-cap-rows-sample-cap-grid-pin-is-red-at-1e-12-on-main`).
    test_utils::pinned_red::row(
        geom_core::Tol::witness().eps(),
        &[1e-12],
        "the curved spline cap",
        &["eps 1e-12: the cap's fit stopped at 3.239736902471488e-12 on (48, 48)"],
        a_curved_spline_cap_moves_with_its_sides_where_its_certificates_hold_row,
    );
}

/// The row [`a_curved_spline_cap_moves_with_its_sides_where_its_certificates_hold`] runs, under its pin.
fn a_curved_spline_cap_moves_with_its_sides_where_its_certificates_hold_row() {
    let eps = Tol::witness().eps();
    let (mut body, cap) = box_with_bumped_cap(0.01);
    let Some(geom::Surface::Nurbs(bump)) = body
        .get_face(cap)
        .and_then(|f| body.get_surface(f.surface))
        .cloned()
    else {
        panic!("the cap wears its spline chart");
    };
    let moves = inward(&body, T);
    let moved = topo::offset_surfaces_together(
        &mut body,
        &moves,
        crate::common::approx::band(),
        Tol::witness(),
    );
    if eps < 3.2795e-12 {
        match &moved {
            Err(ReplaceFaceError::Fit {
                error:
                    geom_brep::OffsetFitError::WallReached {
                        wall: geom_brep::OffsetFitWall::SampleCap { .. },
                        achieved,
                        grid,
                        ..
                    },
                ..
            }) => assert!(
                *grid == (45, 48) && (achieved - 3.2795e-12).abs() < 1e-15,
                "eps {eps:e}: the cap's fit stopped at {achieved:e} on {grid:?}"
            ),
            other => {
                panic!("eps {eps:e}: expected the cap's fit to reach the sample cap, got {other:?}")
            }
        }
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
                                limb: geom_brep::ssi::SsiLimb::HullSupChart
                                    | geom_brep::ssi::SsiLimb::ImageMiss,
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
        let height = if p.z > 0.5 {
            // The fit stands within ε of the offset surface, and the
            // corner within ε of the fit.
            let foot = bump.project(p).expect("the corner projects onto the cap");
            (foot.distance - T).abs() - eps
        } else {
            (p.z - T).abs()
        };
        assert!(
            height <= eps,
            "eps {eps:e}: a corner at {p:?} is {height:e} off its moved cap or floor"
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
    let (body, cap) = box_with_bumped_cap(0.0);
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

/// **Bodies whose charts meet tangentially shell onto their closed
/// forms.** A stadium prism (two planes tangent to two half
/// cylinders), a box with one vertical and one top edge filleted, and a
/// box with all twelve edges rounded (eight sphere corners): each is
/// neither planar nor axial, so every chart moves at once. A tangent
/// edge moves where both of its charts' offset actions put it, and at
/// each corner the surfaces a tangent edge grazes contribute no root;
/// the corner stands on the transversal roots, which every surface
/// there lies under. Each thin solid's volume is its closed form: the
/// operand less the inward offset, which is the same shape at
/// dimensions `2t` smaller and blend radius `r − t`.
#[test]
fn tangent_charts_shell_onto_their_closed_forms() {
    use core::f64::consts::PI;
    let eps = Tol::witness().eps();
    let stadium = |l: f64, r: f64, h: f64| (2.0 * l * r + PI * r * r) * h;
    let filleted = |w: f64, d: f64, h: f64, r: f64| w * d * h - (1.0 - PI / 4.0) * r * r * (h + w);
    let rounded = |w: f64, d: f64, h: f64, r: f64| {
        let (a, b, c) = (w - 2.0 * r, d - 2.0 * r, h - 2.0 * r);
        a * b * c
            + 2.0 * r * (a * b + b * c + c * a)
            + PI * r * r * (a + b + c)
            + 4.0 / 3.0 * PI * r * r * r
    };
    const R: f64 = 0.3;
    for (name, body, closed) in [
        (
            "stadium prism",
            stadium_prism(),
            &(|t: f64| stadium(2.0, 0.5, 1.0) - stadium(2.0, 0.5 - t, 1.0 - 2.0 * t))
                as &dyn Fn(f64) -> f64,
        ),
        ("two filleted edges", filleted_box(false), &|t: f64| {
            filleted(2.0, 2.0, 1.0, R)
                - filleted(2.0 - 2.0 * t, 2.0 - 2.0 * t, 1.0 - 2.0 * t, R - t)
        }),
        ("rounded box", filleted_box(true), &|t: f64| {
            rounded(2.0, 2.0, 1.0, R) - rounded(2.0 - 2.0 * t, 2.0 - 2.0 * t, 1.0 - 2.0 * t, R - t)
        }),
    ] {
        let at_rest = finished(name, body, Tol::witness());
        for t in [0.1, 0.05] {
            let shelled = topo::shell(&at_rest, t, Tol::witness())
                .unwrap_or_else(|e| panic!("eps {eps:e}, {name} at {t}: the shell builds: {e}"));
            let props = topo::mass_properties(
                &topo::test_support::finished(
                    "the measured body",
                    topo::Body::clone(&shelled.body),
                    Tol::witness(),
                ),
                Tol::witness(),
                topo::test_support::pinned_resolution(Tol::witness()),
            )
            .unwrap_or_else(|e| panic!("eps {eps:e}, {name} at {t}: the thin solid weighs: {e}"));
            let want = closed(t);
            assert!(
                (props.enclosure.volume - want).abs() <= props.enclosure.volume_pad + 10.0 * eps,
                "eps {eps:e}, {name} at {t}: volume {} against the closed form {want}",
                props.enclosure.volume
            );
        }
    }
}

/// **A collapse names the move that made it.** The spline-capped box
/// with its cap moved by −0.05 and its `x = 0` side moved 2.5 into the
/// box, past the far side, every other side moved by −0.001: the side
/// plane's root along the cap's rim with a neighbour runs off the rim's
/// far end, and the refusal carries the side's distance, not the cap's,
/// which is the first chart moving that corner. Moved by 1 instead, the
/// side stops inside the box and every chart moves.
#[test]
fn a_collapse_names_the_move_that_made_it() {
    let eps = Tol::witness().eps();
    let (body, cap) = box_with_bumped_cap(0.0);
    let side = body
        .faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.x.abs() > 0.5 && origin.x.abs() < 0.5
            )
        })
        .map(|(k, _)| k)
        .expect("the box has an x = 0 side");
    let moves = |far: f64| {
        let mut moves = vec![
            topo::ChartMove {
                faces: vec![cap],
                distance: -0.05,
            },
            topo::ChartMove {
                faces: vec![side],
                distance: far,
            },
        ];
        moves.extend(
            body.faces()
                .filter(|(k, _)| *k != cap && *k != side)
                .map(|(k, _)| topo::ChartMove {
                    faces: vec![k],
                    distance: -0.001,
                }),
        );
        moves
    };
    let band = crate::common::approx::band();
    let mut through = body.clone();
    let e = topo::offset_surfaces_together(&mut through, &moves(-2.5), band, Tol::witness())
        .expect_err("a side moved past the far side collapses the rim");
    assert!(
        matches!(e, ReplaceFaceError::ReanchorCollapse { offset, .. } if offset == -2.5),
        "eps {eps:e}: expected the collapse at the side's distance, got {e:?}"
    );
    let mut inside = body.clone();
    topo::offset_surfaces_together(&mut inside, &moves(-1.0), band, Tol::witness())
        .unwrap_or_else(|e| panic!("eps {eps:e}: a side moved inside the box moves: {e}"));
}

/// A stadium prism: a 2 × 1 rectangle with half discs on its short
/// sides, extruded 1.
fn stadium_prism() -> Body<f64> {
    let tol = Tol::witness();
    let v = |x: f64, y: f64, bulge: f64| (geom_core::Point2::new(x, y), bulge);
    let profile = profile::Profile::new(
        profile::SketchPlane::xy(),
        vec![profile::test_support::bulge_loop(vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, 1.0),
            v(2.0, 1.0, 0.0),
            v(0.0, 1.0, 1.0),
        ])],
    )
    .validate(tol)
    .expect("the stadium validates");
    sweep::extrude(
        &profile,
        sweep::Extrusion::Distance {
            depth: 1.0,
            side: sweep::ExtrudeSide::Along,
        },
        tol,
    )
    .expect("the stadium extrudes")
    .body
}

/// The `2 × 2 × 1` box with every edge filleted at 0.3, or only its
/// vertical edge at `x = 2, y = 0` and its top edge at `y = 2`.
fn filleted_box(every: bool) -> Body<f64> {
    let tol = Tol::witness();
    let block = sweep::test_support::block::<f64>(2.0, 2.0, 1.0, tol);
    let on = |a: geom_core::Point3<f64>,
              b: geom_core::Point3<f64>,
              f: &dyn Fn(geom_core::Point3<f64>) -> bool| f(a) && f(b);
    let edges: Vec<topo::EdgeKey> = block
        .edges()
        .filter(|(_, e)| {
            let a = block
                .half_edge_start_point(e.he_plus)
                .expect("a live edge's start");
            let b = block
                .half_edge_start_point(e.he_minus)
                .expect("a live edge's end");
            every
                || on(a, b, &|p| (p.x - 2.0).abs() < 1e-9 && p.y.abs() < 1e-9)
                || on(a, b, &|p| {
                    (p.z - 1.0).abs() < 1e-9 && (p.y - 2.0).abs() < 1e-9
                })
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(
        edges.len(),
        if every { 12 } else { 2 },
        "the edges to fillet"
    );
    sweep::blend::fillet_edges(&finished("the box", block, tol), &edges, 0.3, tol)
        .expect("the fillet builds")
        .body
}
