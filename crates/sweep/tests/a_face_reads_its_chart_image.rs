//! **A curved face is measured over its chart image**: its loops with
//! every spur (an edge walked out and straight back) taken out and every
//! seam kept (`topo::chart_image`), and a chart image that is empty on a
//! closed surface is the whole surface, signed by the face's sense.
//!
//! - the three seamed walls a one-segment circle sweeps keep their seams
//!   in the image and read Pappus' volume and area: the extruded
//!   cylinder's strut, the part-turn torus's strut (whose two traversals,
//!   a tube period apart, carry the torus Green form's whole area term)
//!   and the full-turn torus closed on itself both ways;
//! - the one-face ball, whose loop walks one meridian up and back, has
//!   an empty chart image and reads the whole sphere;
//! - a spur built by `mev` into a lune of a ball is taken out of the
//!   lune's image, which reads the lune's closed form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};
use topo::construction::{Construct, MevSite};

use geom_brep::EdgeCurveSpec;
use geom_core::{Arc2, Point2, Point3, Real, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::test_support::revolved_about_y;
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{Body, FaceKey};

fn tol() -> Tol {
    Tol::witness()
}

/// A one-segment circle about `(cx, cy)` of radius `r`.
fn circle<T: Real>(cx: f64, cy: f64, r: f64) -> ProfileLoop<T> {
    let f = T::from_f64;
    RawLoop::new([(
        Point2::new(f(cx + r), f(cy)),
        Segment::Arc(Arc2 {
            centre: Point2::new(f(cx), f(cy)),
            radius: f(r),
            sweep: f(TAU),
        }),
    )])
}

fn y_axis<T: Real>() -> sweep::RevolveAxis<T> {
    sweep::RevolveAxis {
        origin: Point2::new(T::zero(), T::zero()),
        dir: geom_core::Vec2::new(T::zero(), T::one()),
    }
}

/// Every face's reading summed: the body's flux, a third of which is its
/// volume, and its area.
fn measured(body: &Body<f64>, what: &str) -> (f64, f64) {
    body.faces()
        .map(|(face, _)| {
            let r = topo::test_support::face_reading(body, face, tol(), 1e-6)
                .unwrap_or_else(|e| panic!("{what}: face {face:?} reads: {e}"));
            (r.flux, r.area)
        })
        .fold((0.0, 0.0), |(f, a), (g, b)| (f + g, a + b))
}

fn close(got: f64, want: f64, what: &str) {
    assert!(
        (got - want).abs() <= 1e-9 * want.abs().max(1.0),
        "{what}: {got}, closed form {want}"
    );
}

/// The edge count of each of `face`'s loops, and of its chart image.
fn counts(body: &Body<f64>, face: FaceKey) -> (Vec<usize>, Vec<usize>) {
    let f = body.get_face(face).unwrap();
    let loops: Vec<_> = core::iter::once(f.outer)
        .chain(f.rings.iter().copied())
        .collect();
    let full = loops
        .iter()
        .map(|&l| topo::props::loop_edges(body, l).unwrap().0.len())
        .collect();
    let image = loops
        .iter()
        .map(|&l| topo::props::chart_image_edges(body, l).unwrap().0.len())
        .filter(|&n| n > 0)
        .collect();
    (full, image)
}

/// The curved face of a body with one.
fn wall(body: &Body<f64>) -> FaceKey {
    let mut curved = body.faces().filter(|(_, f)| {
        !matches!(
            body.get_surface(f.surface),
            Some(geom::Surface::Plane { .. })
        )
    });
    let (face, _) = curved.next().expect("a curved face");
    assert!(curved.next().is_none(), "one curved face");
    face
}

/// **The seams a one-segment circle sweeps stay in the chart image**,
/// and each wall reads Pappus' volume and area over it.
#[test]
fn seamed_walls_keep_their_seams_and_read_pappus() {
    let (r, h, ox) = (1.5, 2.0, 0.5);
    let vp = Profile::new(SketchPlane::<f64>::xy(), vec![circle(ox, -0.25, r)])
        .validate(tol())
        .unwrap();
    let side = Extrusion::Distance {
        depth: h,
        side: ExtrudeSide::Along,
    };
    let cylinder = extrude(&vp, side, tol()).unwrap().body;
    let (cx, r_tube) = (3.0, 0.5);
    let vp = Profile::new(SketchPlane::<f64>::xy(), vec![circle(cx, 0.25, r_tube)])
        .validate(tol())
        .unwrap();
    let revolve = |turn| sweep::revolve(&vp, y_axis(), turn, tol()).unwrap().body;
    let rows = [
        (
            "the extruded circle's cylinder",
            cylinder,
            PI * r * r * h,
            TAU * r * h + 2.0 * PI * r * r,
        ),
        (
            "the part-turn torus",
            revolve(Revolution::Partial(1.25)),
            1.25 * PI * r_tube * r_tube * cx,
            1.25 * TAU * r_tube * cx + 2.0 * PI * r_tube * r_tube,
        ),
        (
            "the full-turn torus",
            revolve(Revolution::Full),
            TAU * PI * r_tube * r_tube * cx,
            TAU * TAU * r_tube * cx,
        ),
    ];
    for (what, body, volume, area) in rows {
        let face = wall(&body);
        let (full, image) = counts(&body, face);
        assert_eq!(full, vec![4], "{what}: the wall's loop holds its seams");
        assert_eq!(image, full, "{what}: the chart image keeps every seam");
        let (flux, got_area) = measured(&body, what);
        close(flux / 3.0, volume, &format!("{what}: volume"));
        close(got_area, area, &format!("{what}: area"));
    }
}

/// The unit ball revolved from a half circle: two lunes between two
/// meridians.
fn ball() -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        tol(),
    )
}

/// **The one-face ball reads the whole sphere.** Killing one of the
/// ball's two meridians leaves one face whose loop walks the other pole
/// to pole and straight back: a spur both ways round, so its chart image
/// is empty, and an empty image on a closed surface is the whole surface.
#[test]
fn the_one_face_ball_reads_the_whole_sphere() {
    let mut body = ball();
    let meridian = body.edges().next().map(|(_, e)| e.he_plus).unwrap();
    body.kef(meridian).expect("one meridian kills");
    let face = wall(&body);
    let (full, image) = counts(&body, face);
    assert_eq!(full, vec![2], "the loop walks one meridian up and back");
    assert!(image.is_empty(), "its chart image is empty: {image:?}");
    let (flux, area) = measured(&body, "the one-face ball");
    close(flux / 3.0, 4.0 / 3.0 * PI, "the one-face ball: volume");
    close(area, 4.0 * PI, "the one-face ball: area");
    // Its sense bit is the only side it has: flipped, it is a cavity's
    // wall, and the ball's volume comes back negative.
    let inverted = body
        .flipped_face_sense_for_tests(face)
        .expect("the face resolves");
    let (flux, area) = measured(&inverted, "the inverted one-face ball");
    close(
        flux / 3.0,
        -4.0 / 3.0 * PI,
        "the inverted one-face ball: volume",
    );
    close(area, 4.0 * PI, "the inverted one-face ball: area");
}

/// **The empty rule's torus is the full torus's.** The full-turn torus
/// wall keeps its seams, so its own Green form reads it; the whole-torus
/// closed form an empty chart image takes must read that same face, sign
/// and magnitude, under the face's own bit.
#[test]
fn the_whole_torus_is_the_full_turn_torus_face() {
    let vp = Profile::new(SketchPlane::<f64>::xy(), vec![circle(3.0, 0.25, 0.5)])
        .validate(tol())
        .unwrap();
    let body = sweep::revolve(&vp, y_axis(), Revolution::Full, tol())
        .unwrap()
        .body;
    let face = wall(&body);
    let f = body.get_face(face).unwrap();
    let surface = body.get_surface(f.surface).unwrap();
    let reading = topo::test_support::face_reading(&body, face, tol(), 1e-6).unwrap();
    let whole = geom_brep::props::whole_surface(surface, f.sense).unwrap();
    close(whole.flux, reading.flux, "the whole torus's flux");
    close(whole.area, reading.area, "the whole torus's area");
}

/// **A plane face whose only loop is a lone strut bounds nothing**: its
/// chart image is empty, and a plane is no closed surface, so its
/// reading refuses it as a degenerate face rather than answering zero.
#[test]
fn a_lone_strut_plane_face_is_degenerate() {
    use topo::construction::FaceSurface;
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::origin(), true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: geom::Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            sense: true,
        },
    )
    .unwrap();
    body.mev_line(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        Point3::new(1.0, 0.0, 0.0),
        tol(),
    )
    .unwrap();
    let (full, image) = counts(&body, seed.face);
    assert_eq!(full, vec![2], "the strut walked out and back");
    assert!(image.is_empty(), "no chart image: {image:?}");
    let got = topo::test_support::face_reading(&body, seed.face, tol(), 1e-6);
    assert!(
        matches!(
            got,
            Err(topo::MassPropsError::Face {
                source: geom_brep::props::PropsError::DegenerateFace,
                ..
            })
        ),
        "{got:?}"
    );
}

/// **A spur built by `mev` is out of the face's chart image.** A
/// meridian arc from the north pole a radian into a lune, spliced into
/// the lune's loop as an edge walked out and straight back: the lune
/// still reads its own closed form (half the sphere's area, and the
/// ball's volume across both lunes), its two corners and two arcs
/// unchanged, the spur's tip contributing no turn.
#[test]
fn a_built_spur_is_out_of_the_chart_image() {
    let mut body = ball();
    let north = Point3::new(0.0, 1.0, 0.0);
    let at_north = |b: &Body<f64>, h: topo::HalfEdgeKey| {
        let v = b.get_half_edge(h).unwrap().start;
        (*b.get_point(b.get_vertex(v).unwrap().point).unwrap() - north).norm() < 1e-12
    };
    let (lune, he) = body
        .faces()
        .find_map(|(face, f)| {
            let (_, hes) = topo::props::loop_edges(&body, f.outer).unwrap();
            hes.into_iter()
                .find(|&h| at_north(&body, h))
                .map(|h| (face, h))
        })
        .expect("a lune's half-edge leaves the north pole");
    // The half-edge's direction off the pole, and the lune's interior to
    // its left under the outward normal.
    let edge = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
    let forward = he == edge.he_plus;
    let curve = body
        .get_curve_geom(edge.curve)
        .unwrap()
        .certified()
        .unwrap();
    let (t0, t1) = curve.params();
    let near = curve
        .carrier()
        .eval(if forward { t0 + 1e-3 } else { t1 - 1e-3 });
    let along = near - north;
    let along = Vec3::new(along.x, 0.0, along.z) / Vec3::new(along.x, 0.0, along.z).norm();
    let sense = body.get_face(lune).unwrap().sense;
    let outward = if sense {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(0.0, -1.0, 0.0)
    };
    let inward = outward.cross(along);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let carrier = geom::Curve3::Circle {
        center: Point3::origin(),
        axis: up.cross(inward),
        radius: 1.0,
        u_ref: up,
    };
    let tip = carrier.eval(1.0);
    let spec = EdgeCurveSpec::arc_of_circle(carrier, 0.0, 1.0).unwrap();
    body.mev(MevSite::Fan { he1: he, he2: he }, tip, spec, tol())
        .expect("the spur splices into the lune's loop");
    let (full, image) = counts(&body, lune);
    assert_eq!(full, vec![4], "the lune's loop walks the spur out and back");
    assert_eq!(
        image,
        vec![2],
        "its chart image is the lune's two meridians"
    );
    let (flux, area) = measured(&body, "the spurred ball");
    close(flux / 3.0, 4.0 / 3.0 * PI, "the spurred ball: volume");
    close(area, 4.0 * PI, "the spurred ball: area");
    let lune_area = topo::test_support::face_reading(&body, lune, tol(), 1e-6)
        .unwrap()
        .area;
    close(lune_area, TAU, "the spurred lune: area");
    // The lune's cycle read from every half-edge: from the spur's return
    // half the cycle ends on its way out, which only the pass across the
    // cycle's start takes out.
    let (_, cycle) = topo::props::loop_edges(&body, body.get_face(lune).unwrap().outer).unwrap();
    let want = topo::surface_boundary::chart_image(&body, &cycle).unwrap();
    for k in 0..cycle.len() {
        let mut rotated = cycle.clone();
        rotated.rotate_left(k);
        let mut got = topo::surface_boundary::chart_image(&body, &rotated).unwrap();
        assert_eq!(got.len(), 2, "from {k}: the two meridians");
        let at = got.iter().position(|h| *h == want[0]).unwrap();
        got.rotate_left(at);
        assert_eq!(got, want, "from {k}: the same image");
    }
}
