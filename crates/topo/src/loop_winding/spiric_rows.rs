//! **A spiric arc's term in the winding sum** ([`super::spiric_segment_term`]):
//! its bulge over its chord against the area a dense polygon of the arc
//! cuts off, and the bound on the rule's error.

use super::*;
use core::f64::consts::{PI, TAU};

/// The oval of the torus `(R, r)` about `axis` through `c`, cut by the
/// plane `offset` along `u_ref`.
fn oval(
    c: Point3<f64>,
    axis: Vec3<f64>,
    u_ref: Vec3<f64>,
    (big_r, r): (f64, f64),
    offset: f64,
) -> Curve3<f64> {
    Curve3::Spiric {
        center: c,
        axis,
        u_ref,
        major_radius: big_r,
        minor_radius: r,
        offset,
    }
}

/// Twice the vector area between the arc of `carrier` from `ta` to `tb`
/// and its chord, by the polygon of `n` points along it: short of the
/// arc by `O(1/n²)`.
fn polygon(carrier: &Curve3<f64>, (ta, tb): (f64, f64), n: u32) -> Vec3<f64> {
    let p0 = carrier.eval(ta);
    (0..n)
        .map(|k| {
            let t = |k: u32| ta + (tb - ta) * f64::from(k) / f64::from(n);
            (carrier.eval(t(k)) - p0).cross(carrier.eval(t(k + 1)) - p0)
        })
        .fold(Vec3::zero(), |a, b| a + b)
}

/// [`polygon`] at 2000 and 4000 points, extrapolated (Richardson): the
/// `1/n²` terms cancel.
fn polygon_bulge(carrier: &Curve3<f64>, span: (f64, f64)) -> Vec3<f64> {
    (polygon(carrier, span, 4000) * 4.0 - polygon(carrier, span, 2000)) / 3.0
}

/// The carriers every row reads: the donut `(2, 0.5)` and a fat torus
/// `(1, 0.6)`, each cut on both sides of its axis's trace, in two frames.
fn carriers() -> Vec<(String, Curve3<f64>)> {
    let frames = [
        (Point3::origin(), Vec3::unit_y(), Vec3::unit_x()),
        (
            Point3::new(0.3, -1.2, 2.5),
            Vec3::new(2.0, 2.0, 1.0) / 3.0,
            Vec3::new(1.0, -2.0, 2.0) / 3.0,
        ),
    ];
    let mut out = Vec::new();
    for (k, &(c, axis, u)) in frames.iter().enumerate() {
        for (radii, offsets) in [((2.0, 0.5), [0.3, -1.2]), ((1.0, 0.6), [0.1, -0.2])] {
            for d in offsets {
                for (u_ref, offset) in [(u, d), (-u, -d)] {
                    out.push((
                        format!("frame {k}, {radii:?}, offset {offset}"),
                        oval(c, axis, u_ref, radii, offset),
                    ));
                }
            }
        }
    }
    out
}

/// **A spiric arc's bulge is the area it cuts off its chord**: arcs
/// short and long, across the seam at `0` and the inner point at `π`, the
/// whole oval, forward and back, on every carrier; against the area a
/// dense polygon of the arc cuts off. The rule's error bound is far under
/// either's rounding, and the length bounds the polygon's.
#[test]
fn a_spiric_arcs_bulge_is_the_area_it_cuts_off_its_chord() {
    for (name, carrier) in carriers() {
        for (t0, t1) in [(0.2, 0.9), (-1.0, 2.5), (2.0, 4.0), (-PI, PI), (0.0, TAU)] {
            for forward in [true, false] {
                let (bulge, length, error) =
                    spiric_segment_term((&carrier, (t0, t1)), forward).expect("a spiric's term");
                let span = if forward { (t0, t1) } else { (t1, t0) };
                let want = polygon_bulge(&carrier, span);
                let scale = want.norm().max(1e-3);
                assert!(
                    (bulge - want).norm() <= 1e-8 * scale,
                    "{name}, {t0}..{t1}, forward {forward}: {bulge:?} against {want:?}"
                );
                assert!(
                    error <= 1e-15 * scale,
                    "{name}, {t0}..{t1}: error bound {error:e}"
                );
                let walked: f64 = (0..1000)
                    .map(|k| {
                        let t = |k: i32| t0 + (t1 - t0) * f64::from(k) / 1000.0;
                        carrier.eval(t(k)).distance(carrier.eval(t(k + 1)))
                    })
                    .sum();
                assert!(
                    length >= walked,
                    "{name}, {t0}..{t1}: length {length} under {walked}"
                );
            }
        }
    }
}

/// **An oval whose plane nears the inner equator bounds the rule's error
/// by the bulge itself**: there `f` nears zero at `t = π`, and the
/// strip it is analytic in thins past the pieces' width, so the bound
/// grows as large as the area it bounds, which the winding then reads
/// as undecided, never as a sign.
#[test]
fn an_oval_near_the_inner_equator_bounds_its_error_by_its_bulge() {
    let (big_r, r) = (2.0, 0.5);
    for gap in [1e-3, 1e-6, 1e-9] {
        let carrier = oval(
            Point3::origin(),
            Vec3::unit_y(),
            Vec3::unit_x(),
            (big_r, r),
            big_r - r - gap,
        );
        let (bulge, _, error) = spiric_segment_term((&carrier, (0.0, TAU)), true).unwrap();
        assert!(
            error >= bulge.norm(),
            "a gap of {gap}: error {error:e} under the bulge {:e}",
            bulge.norm()
        );
    }
}

/// A planar face bounded by the whole oval `x = d` cuts on the `+z`
/// side of the donut `(2, 0.5)` about `y`, in two edges, and the face
/// the closing edge mints beside it: the two loops of one curve, run
/// opposite ways.
fn oval_faces(d: f64) -> (Body<f64>, [LoopKey; 2]) {
    use crate::euler::{FaceSurface, MefSite};
    let tol = geom_core::Tol::witness();
    let carrier = oval(
        Point3::origin(),
        Vec3::unit_y(),
        Vec3::unit_x(),
        (2.0, 0.5),
        d,
    );
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(carrier.eval(0.0), true).unwrap();
    let plane = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: geom::Surface::Plane {
                    origin: Point3::new(d, 0.0, 0.0),
                    normal: Vec3::unit_x(),
                    u_ref: Vec3::unit_y(),
                },
                sense: true,
            },
        )
        .unwrap();
    let torus = body.add_surface(geom::Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_y(),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::unit_x(),
    });
    let spec = |(t0, t1): (f64, f64)| geom_brep::EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::Intersection {
            pair: geom_brep::SurfacePair::new(plane, torus),
            witness: carrier.mid_point(t0, t1),
        },
        carrier: carrier.clone(),
        param_start: t0,
        param_end: t1,
    };
    let first = body
        .mev(
            crate::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            carrier.eval(PI),
            spec((0.0, PI)),
            tol,
        )
        .unwrap();
    let made = body
        .mef(
            MefSite::Chords {
                he1: first.he_minus,
                he2: first.he_plus,
            },
            spec((PI, TAU)),
            FaceSurface::Shared {
                key: plane,
                sense: true,
            },
            tol,
        )
        .unwrap();
    let outer = |f| body.get_face(f).unwrap().outer;
    let loops = [outer(seed.face), outer(made.face)];
    (body, loops)
}

/// **A loop on a whole oval winds, one way round and the other; one
/// whose rule cannot bound its error under its area has no winding**:
/// the oval of `x = 0.3` on the donut bounds two faces, whose loops wind
/// opposite ways about the plane's normal. The oval of a plane `1e-3`
/// short of the inner equator leaves the rule's error bound over its
/// area ([`an_oval_near_the_inner_equator_bounds_its_error_by_its_bulge`]),
/// and the sum reports no winding rather than a sign.
#[test]
fn a_loop_on_an_oval_winds_and_one_near_the_inner_equator_has_none() {
    let band = geom_core::Band::linear(geom_core::Tol::witness()).unwrap();
    let (body, loops) = oval_faces(0.3);
    let wound = loops.map(|l| body.planar_loop_winding(l, Vec3::unit_x(), band));
    assert!(
        matches!(
            wound,
            [Some(Ok(Sign::Positive)), Some(Ok(Sign::Negative))]
                | [Some(Ok(Sign::Negative)), Some(Ok(Sign::Positive))]
        ),
        "the two loops of one oval: {wound:?}"
    );
    let (body, loops) = oval_faces(1.5 - 1e-3);
    for l in loops {
        assert_eq!(
            body.planar_loop_winding(l, Vec3::unit_x(), band),
            None,
            "a loop whose rule cannot bound its error"
        );
    }
}
