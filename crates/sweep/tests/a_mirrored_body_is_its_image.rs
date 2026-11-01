//! **A mirrored body is its image** (`topo::mirror_body`,
//! `docs/MIRROR-DESIGN.md` P1–P3): over a body of every surface class
//! the sweeps build (planes, a cylinder, a sphere, a cone, a torus,
//! partial revolves with plane caps, a spline loft), mirrored across a
//! plane along an axis and an oblique one, the result is a finished
//! body at rest, of the source's volume, whose every point is the
//! reflection of the source's point under the same key, whose every
//! face's chart reads the source's through the convention (`(u, −v)`
//! on a plane, `(−u, v)` on an analytic chart, `(u, v)` on a net), and
//! whose every pcurve row is the source's row under that map; and
//! mirroring twice comes back to the source's points within the map's
//! rounding. A spiric-capped body (a torus cut by a brick off its axis)
//! carries spiric images on its caps and walls.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::Surface;
use geom_core::{Arc2, Point2, Point3, Tol, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, Reflection, mirror_body};

fn tol() -> Tol {
    Tol::witness()
}

fn finished(what: &str, body: Body<f64>) -> Body<f64> {
    (*topo::test_support::finished(what, body, tol())).clone()
}

fn prism() -> Body<f64> {
    let p = |x, y| (Point2::new(x, y), 0.0);
    let pentagon = bulge_loop(vec![
        p(0.0, 0.0),
        p(2.0, 0.0),
        p(3.0, 1.0),
        p(1.0, 2.0),
        p(0.0, 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![pentagon])
        .validate(tol())
        .unwrap();
    let along = Extrusion::Distance {
        depth: 1.5,
        side: ExtrudeSide::Along,
    };
    extrude(&profile, along, tol()).unwrap().body
}

fn cylinder() -> Body<f64> {
    let circle = RawLoop::new([(
        Point2::new(1.5, 0.25),
        Segment::Arc(Arc2 {
            centre: Point2::new(0.5, 0.25),
            radius: 1.0,
            sweep: TAU,
        }),
    )]);
    let profile = Profile::new(SketchPlane::xy(), vec![circle])
        .validate(tol())
        .unwrap();
    let along = Extrusion::Distance {
        depth: 2.0,
        side: ExtrudeSide::Along,
    };
    extrude(&profile, along, tol()).unwrap().body
}

/// The profile `points` (bulges `0`, a polygon; `1`, a semicircle) on
/// the `xz` sketch, revolved about the sketch's `y` axis through
/// `(0, 0)` by `revolution`.
fn revolved(points: Vec<(Point2<f64>, f64)>, revolution: Revolution<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![bulge_loop(points)])
        .validate(tol())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&profile, axis, revolution, tol()).unwrap().body
}

fn ball() -> Body<f64> {
    revolved(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
    )
}

fn cone() -> Body<f64> {
    let p = |x, y| (Point2::new(x, y), 0.0);
    revolved(
        vec![p(0.0, 0.0), p(1.0, 0.0), p(0.0, 1.5)],
        Revolution::Full,
    )
}

fn torus() -> Body<f64> {
    revolved(
        vec![(Point2::new(1.5, -0.5), 1.0), (Point2::new(1.5, 0.5), 1.0)],
        Revolution::Full,
    )
}

fn half_ring() -> Body<f64> {
    revolved(
        vec![(Point2::new(1.5, -0.5), 1.0), (Point2::new(1.5, 0.5), 1.0)],
        Revolution::Partial(PI * 0.75),
    )
}

fn frustum_wedge() -> Body<f64> {
    let p = |x, y| (Point2::new(x, y), 0.0);
    revolved(
        vec![p(0.5, 0.0), p(1.5, 0.0), p(1.0, 1.0), p(0.5, 1.0)],
        Revolution::Partial(-PI / 3.0),
    )
}

/// A square with one arc side, lofted through a narrower copy of
/// itself: its walls are spline nets.
fn loft() -> Body<f64> {
    let section = |h: f64, b: f64| {
        let p = |x: f64, y: f64| Point2::new(x * h, y * h);
        vec![bulge_loop(vec![
            (p(-1.0, -1.0), 0.0),
            (p(1.0, -1.0), b),
            (p(1.0, 1.0), 0.0),
            (p(-1.0, 1.0), 0.0),
        ])]
    };
    let places = [
        SketchPlane::xy(),
        SketchPlane::xy_at(Point3::new(0.25, 0.0, 1.0)),
        SketchPlane::xy_at(Point3::new(0.0, 0.0, 2.0)),
    ];
    let sections = [section(1.0, 0.3), section(0.75, 0.2), section(0.5, 0.4)];
    sweep::loft_body(&sections, &places, 2, tol())
        .expect("the arc-sided square lofts")
        .body
}

/// A torus (major `1.5`, minor `0.5`, about `y`) less the brick
/// `x ∈ [0.5, 3]`: the brick's face `x = 0.5` stands `0.5` off the axis,
/// inside the two-oval regime, so the cut's rims are spiric ovals.
fn torus_less_a_brick() -> Body<f64> {
    let p = |x, y| (Point2::new(x, y), 0.0);
    let section = bulge_loop(vec![p(0.5, -1.0), p(3.0, -1.0), p(3.0, 1.0), p(0.5, 1.0)]);
    let profile = Profile::new(
        SketchPlane::xy_at(Point3::new(0.0, 0.0, -3.0)),
        vec![section],
    )
    .validate(tol())
    .unwrap();
    let along = Extrusion::Distance {
        depth: 6.0,
        side: ExtrudeSide::Along,
    };
    let brick = extrude(&profile, along, tol()).unwrap().body;
    let ring = topo::test_support::finished("the torus", torus(), tol());
    let brick = topo::test_support::finished("the brick", brick, tol());
    let cut = topo::subtract(&ring, &brick, tol()).expect("the brick cuts the torus");
    cut.body()
        .expect("material is left")
        .body
        .clone()
        .into_body()
}

fn bodies() -> Vec<(&'static str, Body<f64>)> {
    vec![
        ("the prism", prism()),
        ("the cylinder", cylinder()),
        ("the ball", ball()),
        ("the cone", cone()),
        ("the torus", torus()),
        ("the partial ring", half_ring()),
        ("the frustum wedge", frustum_wedge()),
    ]
}

/// Mirror planes: one along an axis, and one oblique to every axis.
fn planes() -> Vec<(&'static str, Reflection<f64>)> {
    vec![
        (
            "x = −0.75",
            Reflection::across(
                Point3::new(-0.75, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                tol(),
            )
            .unwrap(),
        ),
        (
            "z = 0.25",
            Reflection::across(Point3::new(0.0, 0.0, 0.25), Vec3::new(0.0, 0.0, 1.0), tol())
                .unwrap(),
        ),
        (
            "the oblique plane",
            Reflection::across(Point3::new(0.1, -0.2, 0.4), Vec3::new(1.0, 2.0, 3.0), tol())
                .unwrap(),
        ),
    ]
}

/// `body`'s volume and its certified pad, or the measurement's refusal
/// rendered: a spline face's quadrature can refuse its budget at a fine
/// ε, which is the measurement's answer and not the mirror's.
fn volume(body: &Body<f64>) -> Result<(f64, f64), String> {
    let measured = topo::test_support::finished("the measured body", body.clone(), tol());
    topo::mass_properties(
        &measured,
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .map(|m| (m.enclosure.volume, m.enclosure.volume_pad))
    .map_err(|e| format!("{e:?}"))
}

/// The image measures as the source does: both volumes within their
/// pads of each other, or both measurements refusing.
fn same_volume(label: &str, source: &Result<(f64, f64), String>, image: &Body<f64>) {
    match (source, volume(image)) {
        (Ok((v, v_pad)), Ok((w, w_pad))) => assert!(
            w > 0.0 && (w - v).abs() <= v_pad + w_pad + 1e-9 * v,
            "{label}: volume {w} ± {w_pad} against the source's {v} ± {v_pad}"
        ),
        (Err(v), Err(w)) => test_utils::vacuity::stood_down(
            label,
            &format!(
                "no volume compared — the source's measurement refuses ({v}) and so does the \
                 image's ({w})"
            ),
        ),
        (v, w) => panic!("{label}: the source measures {v:?} and the image {w:?}"),
    }
}

/// The chart point of the image that is the source's chart point
/// `(u, v)` reflected: the convention each surface class takes
/// (`topo::mirror`'s module docs).
fn chart_map(s: &Surface<f64>, (u, v): (f64, f64)) -> (f64, f64) {
    match s {
        Surface::Plane { .. } => (u, -v),
        Surface::Cylinder { .. }
        | Surface::Cone { .. }
        | Surface::Sphere { .. }
        | Surface::Torus { .. } => (-u, v),
        Surface::Nurbs(_) | Surface::Approx(_) => (u, v),
    }
}

/// `x` less the whole number of turns nearest it.
fn unwound(x: f64) -> f64 {
    x - TAU * (x / TAU).round()
}

/// How far the chart point `q` is from `p` on `s`'s chart, up to the
/// chart's own periods (and a sphere's twin `(u + π, π − v)`).
fn chart_gap(s: &Surface<f64>, p: (f64, f64), q: (f64, f64)) -> f64 {
    let gap = |(a, b): (f64, f64)| {
        let (du, dv) = (q.0 - a, q.1 - b);
        match s {
            Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => du.hypot(dv),
            Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } => {
                unwound(du).hypot(dv)
            }
            Surface::Torus { .. } => unwound(du).hypot(unwound(dv)),
        }
    };
    match s {
        Surface::Sphere { .. } => gap(p).min(gap((p.0 + PI, PI - p.1))),
        _ => gap(p),
    }
}

/// Every face's chart and every pcurve row of `image` against `M∘S`
/// through [`chart_map`]: a surface sampled at the mapped chart point
/// is the reflection of the source's point, and a row's chart point is
/// the source row's mapped, so a chart that is consistent with its body
/// but not the convention's reds.
fn charts_are_the_convention(
    label: &str,
    body: &Body<f64>,
    image: &Body<f64>,
    r: &Reflection<f64>,
) {
    let map = r.to_affine();
    let surface_of = |b: &Body<f64>, face| {
        b.get_surface(b.get_face(face).expect("a face").surface)
            .expect("a surface")
            .clone()
    };
    for (face, _) in body.faces() {
        let (s, s_image) = (surface_of(body, face), surface_of(image, face));
        let mut samples: Vec<(f64, f64)> = Vec::new();
        match s {
            Surface::Nurbs(_) | Surface::Approx(_) => {
                for (he, row) in body.pcurves() {
                    if body.face_of_half_edge(he) == Some(face) {
                        let (t0, t1) = row.params();
                        let p = row.pcurve().eval(0.5 * (t0 + t1));
                        samples.push((p.x, p.y));
                    }
                }
            }
            _ => {
                for u in [0.3, 1.7, 4.1] {
                    for v in [-0.2, 0.15, 0.35] {
                        samples.push((u, v));
                    }
                }
            }
        }
        for (u, v) in samples {
            let (mu, mv) = chart_map(&s, (u, v));
            let got = s_image.eval(mu, mv);
            let want = map.transform_point(s.eval(u, v));
            assert!(
                (got - want).norm() <= 1e-9,
                "{label}: face {face:?} at ({u}, {v}) samples {got:?}, M∘S is {want:?}"
            );
        }
    }
    for (he, row) in body.pcurves() {
        let face = body
            .face_of_half_edge(he)
            .expect("a row's half-edge bounds a face");
        let s = surface_of(body, face);
        let image_row = image
            .pcurve(he)
            .unwrap_or_else(|| panic!("{label}: {he:?} has no row on the image"));
        let (t0, t1) = row.params();
        for w in [0.25, 0.5, 0.75] {
            let t = t0 + (t1 - t0) * w;
            let p = row.pcurve().eval(t);
            let q = image_row.pcurve().eval(t);
            let gap = chart_gap(&s, chart_map(&s, (p.x, p.y)), (q.x, q.y));
            assert!(
                gap <= 1e-9,
                "{label}: {he:?}'s row at t = {t} is {q:?}, the source's mapped is {:?}",
                chart_map(&s, (p.x, p.y))
            );
        }
    }
}

/// The checks on `body` mirrored across each plane.
fn mirrors_to_its_image(what: &str, body: Body<f64>) {
    let body = finished(what, body);
    let v = volume(&body);
    for (plane, reflection) in planes() {
        let label = format!("{what} across {plane}");
        let image = mirror_body(&body, &reflection, tol())
            .unwrap_or_else(|e| panic!("{label}: the mirror refused: {e}"));
        let image = finished(&label, image);
        same_volume(&label, &v, &image);
        charts_are_the_convention(&label, &body, &image, &reflection);
        let map = reflection.to_affine();
        for (k, p) in body.points() {
            let q = image
                .points()
                .find(|(j, _)| *j == k)
                .expect("the key survives")
                .1;
            assert_eq!(
                q.to_array().map(f64::to_bits),
                map.transform_point(*p).to_array().map(f64::to_bits),
                "{label}: point {k:?} is the reflection of the source's"
            );
        }
        let back = mirror_body(&image, &reflection, tol())
            .unwrap_or_else(|e| panic!("{label}: mirroring back refused: {e}"));
        let back = finished(&format!("{label}, mirrored back"), back);
        for (k, p) in body.points() {
            let q = back
                .points()
                .find(|(j, _)| *j == k)
                .expect("the key survives")
                .1;
            assert!(
                (*q - *p).norm() <= 1e-12,
                "{label}: mirrored back, point {k:?} is {q:?}, not {p:?}"
            );
        }
    }
}

#[test]
fn every_analytic_class_mirrors_to_its_image() {
    for (what, body) in bodies() {
        mirrors_to_its_image(what, body);
    }
}

#[test]
fn a_spiric_capped_torus_mirrors_to_its_image() {
    let body = torus_less_a_brick();
    let spirics = body
        .pcurves()
        .filter(|(_, row)| matches!(row.pcurve(), geom_brep::Pcurve::Spiric { .. }))
        .count();
    assert!(spirics > 0, "the cut's rims carry spiric rows");
    mirrors_to_its_image("the torus less a brick", body);
}

#[test]
fn a_spline_loft_mirrors_to_its_image() {
    mirrors_to_its_image("the loft", loft());
}
