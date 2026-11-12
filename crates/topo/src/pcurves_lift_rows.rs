//! **The decided lift is the read lift.** [`Lift::decided`] runs the
//! mint's walk and stores nothing; [`Lift::read`] reads what the mint
//! stored. On one body the two are one chain, a joint through the
//! sphere's twin sheet and a joint on a pole included.
//!
//! The fixture is the unit sphere about `z`, cut into a quarter and its
//! complement by the `x z` great circle's upper half, from
//! `A = (1, 0, 0)` over the north pole to `B = (−1, 0, 0)`, and the
//! equator's half from `B` back to `A`. With no vertex at the pole the
//! half's image runs over it and ends on the chart's twin sheet, so the
//! joint at its far end closes through the twin; with a vertex there the
//! same region's loop has a pole joint instead.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Point3, Tol, Vec3};

use crate::body::Body;
use crate::construction::{FaceSurface, MefSite, MevSite};
use crate::pcurves::{Lift, loop_rows, mint_pcurves};

/// The `x z` great circle by latitude: `eval(t) = (cos t, 0, sin t)`.
fn great_xz() -> Curve3<f64> {
    Curve3::circle(
        Point3::origin(),
        Vec3::new(0.0, -1.0, 0.0),
        1.0,
        Vec3::new(1.0, 0.0, 0.0),
    )
}

/// The equator from `B`: `eval(t) = (−cos t, −sin t, 0)`.
fn equator_from_b() -> Curve3<f64> {
    Curve3::circle(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        1.0,
        Vec3::new(-1.0, 0.0, 0.0),
    )
}

fn quarter(vertex_at_pole: bool) -> Body<f64> {
    let tol = Tol::witness();
    let g = great_xz();
    let arc = |from: f64, to: f64| EdgeCurveSpec::arc_of_circle(g.clone(), from, to).unwrap();
    let mut body = Body::<f64>::new();
    let start = body.mvfs(g.eval(0.0), true).unwrap();
    body.set_face_surface(
        start.face,
        FaceSurface::New {
            surface: Surface::Sphere {
                center: Point3::origin(),
                radius: 1.0,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            sense: true,
        },
    )
    .unwrap();
    let lone = MevSite::Lone {
        r#loop: start.r#loop,
    };
    let (first_plus, last_minus) = if vertex_at_pole {
        let up = body
            .mev(lone, g.eval(FRAC_PI_2), arc(0.0, FRAC_PI_2), tol)
            .unwrap();
        let down = body
            .mev(
                MevSite::Fan {
                    he1: up.he_minus,
                    he2: up.he_minus,
                },
                g.eval(PI),
                arc(FRAC_PI_2, PI),
                tol,
            )
            .unwrap();
        (up.he_plus, down.he_minus)
    } else {
        let over = body.mev(lone, g.eval(PI), arc(0.0, PI), tol).unwrap();
        (over.he_plus, over.he_minus)
    };
    body.mef(
        MefSite::Chords {
            he1: last_minus,
            he2: first_plus,
        },
        EdgeCurveSpec::arc_of_circle(equator_from_b(), 0.0, PI).unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

#[test]
fn the_decided_lift_is_the_read_lift_through_a_twin_and_a_pole() {
    let band = Band::linear(Tol::witness()).unwrap();
    let mut twins = 0;
    let mut turns = 0;
    for vertex_at_pole in [false, true] {
        let mut body = quarter(vertex_at_pole);
        mint_pcurves(&mut body, Tol::witness()).unwrap();
        for face in body.face_views() {
            let outer = face.record().outer;
            let cycle = loop_rows(&body, outer);
            let decided = Lift::decided(&body, face.key(), face.surface(), &cycle, true, band)
                .unwrap_or_else(|refusal| {
                    panic!("vertex at the pole {vertex_at_pole}: the decided lift: {refusal:?}")
                });
            let read = Lift::read(&body, outer).unwrap();
            let chain = |lift: &Lift<f64>| -> Vec<_> {
                lift.rows()
                    .iter()
                    .map(|row| (row.half_edge, row.lift))
                    .collect()
            };
            assert_eq!(
                chain(&decided),
                chain(&read),
                "vertex at the pole {vertex_at_pole}: the walk decides the chain the body stores"
            );
            assert_eq!(decided.winding(), read.winding());
            assert_eq!(decided.window(), read.window());
            twins += read.rows().iter().filter(|row| row.lift.twin).count();
            turns += cycle
                .iter()
                .filter(|&&he| {
                    let record = body.get_half_edge(he).unwrap();
                    let at_pole = body.vertex_points().any(|(v, p)| {
                        v == record.start && (p.z - 1.0).abs() < 1e-12
                    });
                    at_pole && body.joint(he).is_some_and(|e| !e.deck().twin)
                })
                .count();
        }
    }
    assert!(twins > 0, "the over-pole half closes through the twin sheet");
    assert_eq!(turns, 2, "each face's loop turns once at the pole vertex");
}
