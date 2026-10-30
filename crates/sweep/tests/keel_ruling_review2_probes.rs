//! Review-2 probes for PR 6061 (keel on a bore's ruling). Each pose
//! either refuses typed or builds a body checked by independent oracles:
//! tier 3′ on the result, the closed-form volume, and a split through the
//! trench whose sides hold their closed forms, pass tier 3′, and carry no
//! cap face covering the trench (winding over the cap's own vertices).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Point2, Point3, RigidMotion, Tol, Vec3};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::test_support::{extruded, finished};
use topo::{
    AtRestBody, Body, BooleanError, BooleanResult, FaceKey, LoopBoundary, SplitPart, split,
    subtract, validate_geometric, validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

fn prism_on(what: &str, plane: SketchPlane<f64>, pts: &[(f64, f64, f64)], h: f64) -> AtRestBody<f64> {
    finished(
        what,
        extruded(
            plane,
            vec![bulge_loop(
                pts.iter().map(|&(x, y, b)| (Point2::new(x, y), b)).collect(),
            )],
            h,
            tol(),
        ),
        tol(),
    )
}

fn block() -> AtRestBody<f64> {
    prism_on(
        "the bored block",
        SketchPlane::xy(),
        &[
            (0.0, -2.0, 0.0),
            (4.0, -2.0, 0.0),
            (4.0, 2.0, 0.0),
            (0.0, 2.0, 0.0),
            (0.0, 1.0, -1.0),
            (0.0, -1.0, 0.0),
        ],
        4.0,
    )
}

fn section() -> f64 {
    16.0 - PI / 2.0
}

/// The signed area of a bulge loop: shoelace plus each arc's segment.
fn loop_area(pts: &[(f64, f64, f64)]) -> f64 {
    let n = pts.len();
    let mut a = 0.0;
    for i in 0..n {
        let (x0, y0, b) = pts[i];
        let (x1, y1, _) = pts[(i + 1) % n];
        a += 0.5 * (x0 * y1 - x1 * y0);
        if b != 0.0 {
            let c = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
            let th = 4.0 * b.abs().atan();
            let r = c / (2.0 * (th / 2.0).sin());
            a += b.signum() * 0.5 * r * r * (th - th.sin());
        }
    }
    a
}

fn ring_pts(body: &Body<f64>, face: FaceKey) -> Vec<Vec<Point3<f64>>> {
    let f = body.get_face(face).unwrap();
    let at = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    core::iter::once(f.outer)
        .chain(f.rings.iter().copied())
        .filter_map(|l| match body.get_loop(l).unwrap().boundary {
            LoopBoundary::Cycle { first } => Some(
                body.loop_cycle(first)
                    .unwrap()
                    .into_iter()
                    .map(|h| at(body.get_half_edge(h).unwrap().start))
                    .collect(),
            ),
            LoopBoundary::Empty { .. } => None,
        })
        .collect()
}

/// Winding of the polygon through `ring` round `q`, projected on (e1, e2).
fn winds(ring: &[Point3<f64>], q: Point3<f64>, e1: Vec3<f64>, e2: Vec3<f64>) -> bool {
    let pr = |p: Point3<f64>| {
        let d = p - Point3::origin();
        (d.dot(e1), d.dot(e2))
    };
    let (qx, qy) = pr(q);
    let mut w = 0i32;
    for (a, b) in ring.iter().zip(ring.iter().cycle().skip(1)) {
        let ((ax, ay), (bx, by)) = (pr(*a), pr(*b));
        let cross = (bx - ax) * (qy - ay) - (qx - ax) * (by - ay);
        if ay <= qy && by > qy && cross > 0.0 {
            w += 1;
        } else if ay > qy && by <= qy && cross < 0.0 {
            w -= 1;
        }
    }
    w != 0
}

fn vol(b: &AtRestBody<f64>, what: &str) -> (f64, f64) {
    let mp = topo::test_support::pinned_volume(what, b, tol());
    (mp.volume, mp.volume_pad)
}

fn close(got: (f64, f64), want: f64) -> bool {
    (got.0 - want).abs() <= got.1 + 1e-9 * want.abs().max(1.0)
}

/// The oracle. `Ok(())` where the pose refused typed or built a right
/// body; `Err(why)` on a shipped wrong body (or an untyped refusal).
fn judge(
    what: &str,
    r: Result<BooleanResult<f64>, BooleanError>,
    want: f64,
    cut: Point3<f64>,
    normal: Vec3<f64>,
    e1: Vec3<f64>,
    e2: Vec3<f64>,
    want_sides: [f64; 2],
    in_trench: Point3<f64>,
) -> Result<String, String> {
    let body = match r {
        Err(e) => return Ok(format!("refused {e:?}").chars().take(160).collect()),
        Ok(BooleanResult::Body(b)) => b.body,
        Ok(other) => return Err(format!("{what}: not a body {other:?}")),
    };
    let mut bad = Vec::new();
    if let Err(e) = validate_geometric(&body, tol()) {
        bad.push(format!("tier 3: {e:?}"));
    }
    if let Err(e) = validate_pseudomanifold(&body, &topo::ContactRecords::default(), tol()) {
        bad.push(format!("tier 3′ ({} errors): {:?}", e.len(), e.first()));
    }
    let v = vol(&body, what);
    if !close(v, want) {
        bad.push(format!("volume {v:?} want {want}"));
    }
    let plane = topo::test_support::split_plane(cut, normal, tol());
    match split(&body, &plane, tol()) {
        Err(e) => bad.push(format!("split refused {e:?}")),
        Ok(sr) => {
            for (side, part, w) in [("above", &sr.above, want_sides[0]), ("below", &sr.below, want_sides[1])] {
                let SplitPart::Body(sb) = part else {
                    bad.push(format!("nothing {side}"));
                    continue;
                };
                let v = vol(sb, side);
                if !close(v, w) {
                    bad.push(format!("{side}: volume {v:?} want {w}"));
                }
                if let Err(e) = validate_pseudomanifold(sb, &topo::ContactRecords::default(), tol()) {
                    bad.push(format!("{side}: tier 3′ ({} errors): {:?}", e.len(), e.first()));
                }
                let n = normal;
                let covering = sb
                    .faces()
                    .map(|(f, _)| f)
                    .filter(|&f| {
                        let loops = ring_pts(sb, f);
                        loops.iter().flatten().all(|p| (*p - cut).dot(n).abs() < 1e-9)
                            && loops.iter().filter(|r| winds(r, in_trench, e1, e2)).count() % 2 == 1
                    })
                    .count();
                if covering != 0 {
                    bad.push(format!("{side}: {covering} cap face(s) cover the trench"));
                }
            }
        }
    }
    if bad.is_empty() {
        Ok("built, every oracle holds".into())
    } else {
        Err(format!("{what}: WRONG BODY SHIPPED: {bad:?}"))
    }
}

/// A trench over `pts` from `z0` up `h`, subtracted from the block, z-split
/// at `zc` through `in_trench`.
fn cyl_pose(what: &str, pts: &[(f64, f64, f64)], z0: f64, h: f64, zc: f64, q: (f64, f64)) -> Result<String, String> {
    let trench = prism_on("trench", SketchPlane::xy_at(Point3::new(0.0, 0.0, z0)), pts, h);
    let a = loop_area(pts);
    let tv = vol(&trench, "trench");
    assert!(close(tv, a * h), "{what}: trench area formula {tv:?} vs {}", a * h);
    let ov = |lo: f64, hi: f64| (hi.min(z0 + h) - lo.max(z0)).max(0.0);
    let want = 4.0 * section() - a * ov(0.0, 4.0);
    let sides = [
        (4.0 - zc) * section() - a * ov(zc, 4.0),
        zc * section() - a * ov(0.0, zc),
    ];
    let r = subtract(&block(), &trench, tol());
    judge(
        what,
        r,
        want,
        Point3::new(0.0, 0.0, zc),
        Vec3::unit_z(),
        Vec3::unit_x(),
        Vec3::unit_y(),
        sides,
        Point3::new(q.0, q.1, zc),
    )
}

fn tri(b_close: f64, b_first: f64) -> Vec<(f64, f64, f64)> {
    vec![(1.0, 0.0, b_first), (1.6, -0.3, 0.0), (1.6, 0.3, b_close)]
}

fn report(rows: Vec<(String, Result<String, String>)>) {
    let mut wrong = Vec::new();
    for (name, r) in &rows {
        eprintln!("PROBE {name}: {r:?}");
        if let Err(e) = r {
            wrong.push(e.clone());
        }
    }
    assert!(wrong.is_empty(), "{} wrong: {wrong:#?}", wrong.len());
}

/// Claim 1: curved closing walls, both walls curved, the keel's extent
/// along the ruling, and another ruling.
#[test]
fn review2_curved_walls_and_keel_positions() {
    let mut rows = Vec::new();
    for b in [0.05, -0.05, 0.2, -0.2, 0.5, -0.5] {
        rows.push((format!("closing bulge {b}"), cyl_pose("c", &tri(b, 0.0), 2.0, 3.0, 3.0, (1.3, 0.0))));
    }
    for (bc, bf) in [(0.2, 0.2), (-0.2, -0.2), (0.2, -0.2), (0.0, 0.2), (0.0, -0.3)] {
        rows.push((format!("both walls {bc}/{bf}"), cyl_pose("b", &tri(bc, bf), 2.0, 3.0, 3.0, (1.3, 0.0))));
    }
    for (z0, h, zc) in [(-1.0, 6.0, 3.0), (0.0, 4.0, 3.0), (1.0, 2.0, 2.0), (2.0, 3.0, 3.0), (-1.0, 3.0, 1.0)] {
        for b in [0.0, 0.2] {
            rows.push((
                format!("z0 {z0} h {h} bulge {b}"),
                cyl_pose("z", &tri(b, 0.0), z0, h, zc, (1.3, 0.0)),
            ));
        }
    }
    // Another ruling: the apex at azimuth 0.5, the triangle turned with it.
    let (c, s) = (0.5f64.cos(), 0.5f64.sin());
    let rot = |x: f64, y: f64| (c * x - s * y, s * x + c * y);
    for b in [0.0, 0.2, -0.2] {
        let pts: Vec<_> = tri(b, 0.0).into_iter().map(|(x, y, bb)| { let (u, v) = rot(x, y); (u, v, bb) }).collect();
        let q = rot(1.3, 0.0);
        rows.push((format!("azimuth 0.5 bulge {b}"), cyl_pose("az", &pts, 2.0, 3.0, 3.0, q)));
    }
    report(rows);
}

/// Claim 2: a closing wall tangent to the bore at the apex, and near it
/// (transverse at angle δ, staying outside the bore).
#[test]
fn review2_tangent_and_near_tangent_walls() {
    let alpha = 0.3f64.atan2(0.6);
    let mut rows = Vec::new();
    for d in [0.0, 1e-12, 1e-9, 1e-6, 1e-3] {
        let b = ((PI / 2.0 - alpha - d) / 2.0).tan();
        rows.push((format!("tangent δ {d:e} (bulge {b})"), cyl_pose("t", &tri(b, 0.0), 2.0, 3.0, 3.0, (1.3, 0.0))));
    }
    // The other wall tangent too (both walls tangent: a cusp each side).
    let alpha2 = (-0.3f64).atan2(0.6);
    for d in [0.0, 1e-9] {
        let b1 = ((PI / 2.0 + alpha2 - d) / 2.0).tan();
        rows.push((format!("first wall tangent δ {d:e} (bulge {b1})"), cyl_pose("t1", &tri(0.0, b1), 2.0, 3.0, 3.0, (1.3, 0.0))));
    }
    report(rows);
}

/// Claim 1, cone: the block less a frustum about y (r = 1 − y/8), the
/// trench prism extruded along a generator at azimuth `az`.
#[test]
fn review2_keel_on_a_cone_generator() {
    let r_of = |y: f64| 1.0 - y / 8.0;
    let frustum = finished(
        "frustum",
        sweep::test_support::revolved_about_y(
            vec![
                (Point2::new(0.0, -1.0), 0.0),
                (Point2::new(r_of(-1.0), -1.0), 0.0),
                (Point2::new(r_of(5.0), 5.0), 0.0),
                (Point2::new(0.0, 5.0), 0.0),
            ],
            sweep::Revolution::Full,
            tol(),
        ),
        tol(),
    );
    let box_ = prism_on(
        "box",
        SketchPlane::xy_at(Point3::new(0.0, 0.0, -2.0)),
        &[(-2.0, 0.0, 0.0), (2.0, 0.0, 0.0), (2.0, 4.0, 0.0), (-2.0, 4.0, 0.0)],
        4.0,
    );
    let cone_block = match subtract(&box_, &frustum, tol()) {
        Ok(BooleanResult::Body(b)) => b.body,
        other => {
            eprintln!("PROBE cone block refused: {other:?}");
            return;
        }
    };
    let bore_vol = PI * (0.0f64..4.0).step_by_simpson(|y| r_of(y).powi(2));
    let whole = 16.0 * 4.0 - bore_vol;
    let v = vol(&cone_block, "cone block");
    assert!(close(v, whole), "cone block volume {v:?} want {whole}");
    let mut rows = Vec::new();
    for az in [1.0f64, 2.5] {
        for b in [0.0, 0.2, -0.2] {
            let y0 = 2.0;
            let p0 = Point3::new(r_of(y0) * az.cos(), y0, r_of(y0) * az.sin());
            let g = Vec3::new(-az.cos() / 8.0, 1.0, -az.sin() / 8.0).normalize();
            let n = Vec3::new(az.cos(), 1.0 / 8.0, az.sin()).normalize();
            let tau = n.cross(g);
            // Sketch u = n × g... pick u, v so u × v = g: v = n, u = n × g.
            let band = geom_core::Band::linear(tol()).unwrap();
            let frame = RigidMotion::gram_schmidt(p0, tau, n, "probe_cone", band).unwrap();
            assert!(frame.w().get().dot(g) > 0.999, "the sketch normal runs along the generator");
            let pts = [(0.0, 0.0, 0.0), (0.3, 0.6, 0.0), (-0.3, 0.6, b)];
            let len = 3.0;
            let trench = prism_on("cone trench", SketchPlane::from_frame(frame), &pts, len);
            let a = loop_area(&pts);
            let tv = vol(&trench, "cone trench");
            assert!(close(tv, a * len), "cone trench area {tv:?} vs {}", a * len);
            let (cu, cv) = centroid(&pts);
            let cen = p0 + tau * cu + n * cv;
            let t_top = (4.0 - cen.y) / g.y;
            let want = whole - a * t_top;
            let t_cut = (3.0 - cen.y) / g.y;
            let below = 16.0 * 3.0 - PI * (0.0f64..3.0).step_by_simpson(|y| r_of(y).powi(2)) - a * t_cut;
            let above = whole - a * t_top - below;
            let q = cen + g * t_cut;
            let r = subtract(&cone_block, &trench, tol());
            rows.push((
                format!("cone az {az} bulge {b}"),
                judge("cone", r, want, Point3::new(0.0, 3.0, 0.0), Vec3::unit_y(), Vec3::unit_z(), Vec3::unit_x(), [above, below], q),
            ));
        }
    }
    report(rows);
}

/// The centroid of a bulge loop: shoelace moments plus each arc's
/// segment (its centroid 4R sin³(θ/2) / 3(θ − sin θ) from the centre).
fn centroid(pts: &[(f64, f64, f64)]) -> (f64, f64) {
    let n = pts.len();
    let (mut a, mut mx, mut my) = (0.0, 0.0, 0.0);
    for i in 0..n {
        let (x0, y0, b) = pts[i];
        let (x1, y1, _) = pts[(i + 1) % n];
        let cr = x0 * y1 - x1 * y0;
        a += 0.5 * cr;
        mx += (x0 + x1) * cr / 6.0;
        my += (y0 + y1) * cr / 6.0;
        if b != 0.0 {
            let (dx, dy) = (x1 - x0, y1 - y0);
            let c = (dx * dx + dy * dy).sqrt();
            let th = 4.0 * b.abs().atan();
            let r = c / (2.0 * (th / 2.0).sin());
            let seg = 0.5 * r * r * (th - th.sin());
            let s = b.signum();
            let (lx, ly) = (-dy / c * s, dx / c * s);
            let (cx, cy) = ((x0 + x1) / 2.0 + lx * r * (th / 2.0).cos(), (y0 + y1) / 2.0 + ly * r * (th / 2.0).cos());
            let d = 4.0 * r * (th / 2.0).sin().powi(3) / (3.0 * (th - th.sin()));
            let (gx, gy) = (cx - lx * d, cy - ly * d);
            a += s * seg;
            mx += s * seg * gx;
            my += s * seg * gy;
        }
    }
    (mx / a, my / a)
}

trait Simpson {
    fn step_by_simpson(self, f: impl Fn(f64) -> f64) -> f64;
}
impl Simpson for core::ops::Range<f64> {
    fn step_by_simpson(self, f: impl Fn(f64) -> f64) -> f64 {
        // f is a quadratic here, so Simpson's rule is exact.
        let (a, b) = (self.start, self.end);
        (b - a) / 6.0 * (f(a) + 4.0 * f((a + b) / 2.0) + f(b))
    }
}

// ---- Claim 5: the census fed a curved-wall one-edge form directly ----

use topo::construction::{Construct, MefSite};
use topo::euler::FaceSurface;
use topo::{Cell, EdgeKey, ValidationError, VertexKey, intersect, union};

const TIP: Point3<f64> = Point3::new(1.0, 0.0, 2.0);
const TOP: Point3<f64> = Point3::new(1.0, 0.0, 4.0);

fn at(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}
fn ends(body: &Body<f64>, e: EdgeKey) -> [VertexKey; 2] {
    let h = body.get_edge(e).unwrap().he_plus;
    [body.get_half_edge(h).unwrap().start, body.half_edge_end(h).unwrap()]
}
fn keel_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let on = |p: Point3<f64>| (p.x - 1.0).abs() < 1e-12 && p.y.abs() < 1e-12;
    body.edges().map(|(e, _)| e).filter(|&e| ends(body, e).iter().all(|&v| on(at(body, v)))).collect()
}
fn bore_face(body: &Body<f64>) -> FaceKey {
    let bores: Vec<FaceKey> = body
        .faces()
        .map(|(f, _)| f)
        .filter(|&f| {
            topo::query::face_surface_kind(body, f) == Some(geom::SurfaceKind::Cylinder)
                && ring_pts(body, f)[0].iter().any(|p| p.x < 0.5)
        })
        .collect();
    assert_eq!(bores.len(), 1, "one bore face: {bores:?}");
    bores[0]
}
fn faces_of(body: &Body<f64>, e: EdgeKey) -> [FaceKey; 2] {
    let h = body.get_edge(e).unwrap().he_plus;
    [h, body.mate(h).unwrap()].map(|h| body.face_of_half_edge(h).unwrap())
}
fn body_of(r: Result<BooleanResult<f64>, BooleanError>, what: &str) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("{what}: {other:?}"),
    }
}

/// The slit form with wall a (the apex to (1.6, 0.3)) an arc of bulge
/// `b` as the curved trench's closing wall runs it.
fn curved_slit_block(b: f64) -> AtRestBody<f64> {
    let p = |z0| SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let upper = prism_on(
        "upper",
        p(-1.0),
        &[(1.0, 0.0, -b), (1.6, 0.3, 0.0), (1.6, -5.0, 0.0), (5.0, -5.0, 0.0), (5.0, 5.0, 0.0), (-5.0, 5.0, 0.0), (-5.0, 0.0, 0.0)],
        6.0,
    );
    let lower = body_of(
        union(
            &prism_on("lower", p(-1.0), &[(1.0, 0.0, 0.0), (-5.0, 0.0, 0.0), (-5.0, -5.0, 0.0), (1.6, -5.0, 0.0), (1.6, -0.3, 0.0)], 6.0),
            &prism_on("V", p(-1.0), &tri(b, 0.0), 3.0),
            tol(),
        ),
        "lower tool",
    );
    body_of(
        union(
            &body_of(intersect(&block(), &upper, tol()), "upper side"),
            &body_of(intersect(&block(), &lower, tol()), "lower side"),
            tol(),
        ),
        "two sides",
    )
}

#[test]
fn review2_census_reads_the_curved_wall_one_edge_form() {
    for b in [0.2, -0.2] {
        let slit = curved_slit_block(b);
        let want = 4.0 * section() - 2.0 * loop_area(&tri(b, 0.0));
        let v = vol(&slit, "slit");
        assert!(close(v, want), "bulge {b}: slit volume {v:?} want {want}");
        assert_eq!(validate_pseudomanifold(&slit, &topo::ContactRecords::default(), tol()), Ok(()), "bulge {b}: slit tier 3′");
        let mut body: Body<f64> = (*slit).clone();
        let bore = bore_face(&body);
        let outer = body.get_face(bore).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else { panic!() };
        let tops: Vec<_> = body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .filter(|&h| at(&body, body.get_half_edge(h).unwrap().start).distance(TOP) < 1e-12)
            .collect();
        assert_eq!(tops.len(), 2, "bulge {b}: the slit parts the top: {tops:?}");
        let joint = body
            .mef(
                MefSite::Chords { he1: tops[0], he2: tops[1] },
                geom_brep::EdgeCurveSpec::self_loop_circle_at(TOP),
                FaceSurface::Inherit,
                tol(),
            )
            .unwrap();
        let keep = body.get_half_edge(tops[0]).unwrap().start;
        let dying = if body.get_half_edge(joint.he_plus).unwrap().start == keep { joint.he_plus } else { joint.he_minus };
        body.kev_describing(dying, &[], tol()).unwrap();
        let gusset = [joint.face, bore].into_iter().find(|&f| ring_pts(&body, f)[0].len() == 2).expect("gusset");
        let go = body.get_face(gusset).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(go).unwrap().boundary else { panic!() };
        let to_wall_b = body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .find(|&h| {
                let wall = body.face_of_half_edge(body.mate(h).unwrap()).unwrap();
                ring_pts(&body, wall)[0].iter().any(|p| p.y < -0.1)
            })
            .unwrap();
        body.kef_minting(to_wall_b, tol()).unwrap();
        let keel = keel_edges(&body);
        assert_eq!(keel.len(), 1, "bulge {b}: one keel edge: {keel:?}");
        let keel = keel[0];
        let bore = bore_face(&body);
        assert!(!faces_of(&body, keel).contains(&bore), "bulge {b}: the keel runs between the walls");
        let kinds = faces_of(&body, keel).map(|f| topo::query::face_surface_kind(&body, f));
        eprintln!("PROBE c5 bulge {b}: keel walls {kinds:?}");
        assert!(kinds.contains(&Some(geom::SurfaceKind::Cylinder)), "a curved wall");
        let (s1, s2) = topo::readback::edge_sides(&body, keel).unwrap().surfaces();
        let [p0, p1] = ends(&body, keel).map(|v| at(&body, v));
        let mut spec = geom_brep::EdgeCurveSpec::line_between(p0, p1);
        spec.description = topo::EdgeDescriptionSpec::Intersection {
            pair: topo::SurfacePair::new(s1, s2),
            selector: geom_brep::CurveSelector::Witness(p0.lerp(p1, 0.5)),
        };
        body.set_edge_curve(keel, spec, tol()).unwrap();
        topo::mint_pcurves(&mut body, tol()).unwrap();
        eprintln!("PROBE c5 bulge {b}: tier 3 {:?}", validate_geometric(&body, tol()));
        let tip = body.vertices().find(|&(v, _)| at(&body, v).distance(TIP) < 1e-12).unwrap().0;
        let records = topo::test_support::records(&body, [(Cell::Vertex(tip), Cell::Face(bore))]);
        let r = validate_pseudomanifold(&body, &records, tol());
        eprintln!("PROBE c5 bulge {b}: tier 3′ {r:?}");
        let errors = r.expect_err("tier 3′ refuses the curved-wall one-edge form");
        assert!(
            errors.iter().any(|e| matches!(e, ValidationError::FaceThroughEdgeWedge { edge, face, .. } if *edge == keel && *face == bore)),
            "bulge {b}: the keel's wedge refused: {errors:?}"
        );
    }
}

/// Claim 3: the `(_, Some(_)) => Tangent` arm. A narrow trench (wall reach
/// ≈ len/2) whose closing wall meets the bore at angle δ, over a ladder of
/// δ; prints each refusal and counts `EdgeInFaceGap::Tangent`.
#[test]
fn review2_tangent_arm_ladder() {
    let s = 0.1;
    let pts = |b: f64| vec![(1.0, 0.0, 0.0), (1.0 + 0.6 * s, -0.3 * s, 0.0), (1.0 + 0.6 * s, 0.3 * s, b)];
    let alpha = 0.3f64.atan2(0.6);
    let eps = tol().eps();
    let mut tangent = 0;
    for k in 0..40 {
        let d = eps * 0.3 * 1.25f64.powi(k);
        let b = ((PI / 2.0 - alpha - d) / 2.0).tan();
        let r = cyl_pose("arm", &pts(b), 2.0, 3.0, 3.0, (1.0 + 0.3 * s, 0.0));
        if matches!(&r, Ok(m) if m.contains("gap: Tangent")) {
            tangent += 1;
        }
        eprintln!("PROBE arm δ/ε {:.3}: {}", d / eps, match &r { Ok(m) => m.chars().take(120).collect::<String>(), Err(e) => e.chars().take(200).collect() });
    }
    eprintln!("PROBE arm: {tangent} Tangent refusals");
}
