//! Second-review probes for PR 4252 (lane `band-review-4252-second`).
//! Not for merge: each row falsifies or confirms one claim by execution.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Point2, Point3, Tol};
use sweep::Revolution;
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::test_support::{revolved_about_y, rim_arcs_at};
use topo::{AtRestBody, Body, EdgeKey, FaceKey, MefSite, MevSite, mass_properties};

fn tol() -> Tol {
    Tol::witness()
}

fn faces_of(body: &Body<f64>, e: EdgeKey) -> (FaceKey, FaceKey) {
    let ed = body.get_edge(e).unwrap();
    let f = |he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    (f(ed.he_plus), f(ed.he_minus))
}

fn is_plane(body: &Body<f64>, f: FaceKey) -> bool {
    matches!(
        body.get_surface(body.get_face(f).unwrap().surface).unwrap(),
        Surface::Plane { .. }
    )
}

fn pole_cylinder() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

fn repaired(mut body: Body<f64>) -> Body<f64> {
    body.merge_coplanar_faces(tol()).unwrap();
    body
}

fn wall_meridians(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges()
        .map(|(e, _)| e)
        .filter(|&e| {
            let (fa, fb) = faces_of(body, e);
            fa != fb
                && !is_plane(body, fa)
                && !is_plane(body, fb)
                && body.get_face(fa).unwrap().surface == body.get_face(fb).unwrap().surface
        })
        .collect()
}

fn midpoint(body: &Body<f64>, e: EdgeKey) -> Point3<f64> {
    let c = body
        .get_curve_geom(body.get_edge(e).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap();
    let (t0, t1) = c.params();
    c.carrier().eval((t0 + t1) * 0.5)
}

/// M1: the wrap restated on EITHER meridian finishes, meters the
/// cylinder's closed-form volume (pi r^2 h = pi), and its image is the
/// iso line at that meridian's own chart u (0 or pi).
#[test]
fn probe_wrap_on_either_meridian_is_a_legitimate_finished_wall() {
    for repair in [false, true] {
        for order in [0usize, 1] {
            let mut body = pole_cylinder();
            if repair {
                body = repaired(body);
            }
            let m = wall_meridians(&body);
            assert_eq!(m.len(), 2);
            let (dies, wraps) = (m[order], m[1 - order]);
            let (wall, _) = faces_of(&body, wraps);
            let wall = body.get_face(wall).unwrap().surface;
            let dying = body.get_edge(dies).unwrap().he_plus;
            let at = midpoint(&body, wraps);
            body.kef_describing(dying, &[(wraps, EdgeDescriptionSpec::wrap(wall))], tol())
                .expect("restates");
            let c = body
                .get_curve_geom(body.get_edge(wraps).unwrap().curve)
                .unwrap()
                .certified()
                .unwrap();
            let chart = c.description().chart().expect("a chart image");
            let u = chart.pcurve.eval(0.0).x;
            eprintln!(
                "repair={repair} order={order} wrap at {at:?}: image u = {u}, wrap = {}",
                chart.wrap
            );
            assert!(chart.wrap);
            let operand = AtRestBody::validate(body.clone(), tol())
                .unwrap_or_else(|e| panic!("repair={repair} order={order}: {e:?}"));
            let mp = mass_properties(&body, tol()).unwrap();
            eprintln!("  volume {} pad {}", mp.volume, mp.volume_pad);
            assert!(
                (mp.volume - core::f64::consts::PI).abs() < 1e-9,
                "volume {}",
                mp.volume
            );
            let arcs = rim_arcs_at(&body, 1.0, 0.0);
            let f = fillet_edges(&operand, &arcs, 0.05, tol()).map(drop);
            let ch = chamfer_edges(&operand, &arcs, 0.05, tol()).map(drop);
            eprintln!("  fillet {f:?}\n  chamfer {ch:?}");
            assert!(f.is_err() && ch.is_err());
        }
    }
}

/// Item 3: a pinched host. The repaired cylinder's base disc gets a
/// coplanar triangle cut in at a rim vertex (mev, mev, mef), so the
/// disc's one outer cycle is both rim arcs plus the triangle's path —
/// the hostless host gate's shape with no spur, no slit.
#[test]
fn probe_pinched_host_reaches_the_hostless_gate() {
    for flip in [false, true] {
        let mut body = repaired(pole_cylinder());
        let arcs = rim_arcs_at(&body, 1.0, 0.0);
        assert_eq!(arcs.len(), 2);
        let (fa, fb) = faces_of(&body, arcs[0]);
        let host = if is_plane(&body, fa) { fa } else { fb };
        let plane = body.get_face(host).unwrap().surface;
        let ed = body.get_edge(arcs[0]).unwrap();
        let he = [ed.he_plus, ed.he_minus]
            .into_iter()
            .find(|&h| {
                body.get_loop(body.get_half_edge(h).unwrap().parent_loop)
                    .unwrap()
                    .face
                    == host
            })
            .unwrap();
        let v0 = body.get_half_edge(he).unwrap().start;
        let p0 = *body.get_point(body.get_vertex(v0).unwrap().point).unwrap();
        let inward = Point3::new(p0.x * 0.5, p0.y, p0.z * 0.5);
        let side = Point3::new(-p0.z, 0.0, p0.x);
        let s = if flip { -0.2 } else { 0.2 };
        let w1 = Point3::new(inward.x + s * side.x, p0.y, inward.z + s * side.z);
        let w2 = Point3::new(inward.x - s * side.x, p0.y, inward.z - s * side.z);
        let seg = |a: Point3<f64>, b: Point3<f64>| {
            EdgeCurveSpec::line_between(a, b).at_rest_in_chart(plane, false)
        };
        let a = match body.mev(MevSite::Fan { he1: he, he2: he }, w1, seg(p0, w1), tol()) {
            Ok(a) => a,
            Err(e) => panic!("flip={flip}: mev 1 refused: {e:?}"),
        };
        let b = match body.mev(
            MevSite::Fan {
                he1: a.he_minus,
                he2: a.he_minus,
            },
            w2,
            seg(w1, w2),
            tol(),
        ) {
            Ok(b) => b,
            Err(e) => panic!("flip={flip}: mev 2 refused: {e:?}"),
        };
        // he1 starts at W2 (the second strut's return half), he2 at V0
        // (the first strut's outgoing half).
        let made = body.mef(
            MefSite::Chords {
                he1: b.he_minus,
                he2: a.he_plus,
            },
            seg(w2, p0),
            topo::FaceSurface::Inherit,
            tol(),
        );
        let made = match made {
            Ok(m) => m,
            Err(e) => {
                eprintln!("flip={flip}: mef refused: {e:?}");
                continue;
            }
        };
        let outer = body.get_face(host).unwrap().outer;
        eprintln!(
            "flip={flip}: host outer cycle {} half-edges; new face {:?}",
            body.loop_cycle(match body.get_loop(outer).unwrap().boundary {
                topo::LoopBoundary::Cycle { first } => first,
                _ => unreachable!(),
            })
            .unwrap()
            .len(),
            made.face
        );
        match AtRestBody::validate(body.clone(), tol()) {
            Err(e) => {
                eprintln!("flip={flip}: does not finish: {e:?}");
                // The mirrored triangle winds against the plane.
                assert!(flip, "the counterclockwise pinch finishes: {e:?}");
            }
            Ok(operand) => {
                eprintln!("flip={flip}: FINISHES");
                assert!(!flip);
                let f = fillet_edges(&operand, &arcs, 0.05, tol()).map(drop);
                let c = chamfer_edges(&operand, &arcs, 0.05, tol()).map(drop);
                eprintln!("  fillet {f:?}\n  chamfer {c:?}");
                assert!(matches!(&f, Err(r) if matches!(&r.error,
                    sweep::blend::BlendError::UnsupportedChain { detail, .. }
                    if detail.contains("host face carries edges outside the requested chain"))));
                assert!(c.is_err());
            }
        }
    }
}
