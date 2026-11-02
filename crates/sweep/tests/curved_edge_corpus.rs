//! **A corpus of curved bodies with an analytic membership truth**: the
//! revolves, booleans and prisms the curved edge-meeting ray's pins and
//! probes read every crossing of a ray on
//! (`a_ray_meets_a_curved_edge_once_for_both_faces`). PR 5697's second
//! review built it; its first review added the kinked bore and the thin
//! discs.

#![allow(clippy::expect_used, clippy::panic, clippy::type_complexity)]

use core::f64::consts::PI;

use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{
    ball_poled_z, bored_block_of_arcs, boss_of_arcs, brick, disc_of_arcs, revolved_about_y,
};
use topo::Body;

/// Inside a closed bulge loop in the plane (`b > 0` bulges to the right
/// of the chord's direction): inside the chord polygon, XOR inside an
/// odd number of the arcs' circular segments.
pub(crate) fn in_bulge_loop(verts: &[(Point2<f64>, f64)], p: (f64, f64)) -> bool {
    let n = verts.len();
    let mut inside = false;
    for i in 0..n {
        let (a, _) = verts[i];
        let (b, _) = verts[(i + 1) % n];
        if (a.y > p.1) != (b.y > p.1) {
            let x = a.x + (p.1 - a.y) * (b.x - a.x) / (b.y - a.y);
            if x > p.0 {
                inside = !inside;
            }
        }
    }
    for i in 0..n {
        let (a, bulge) = verts[i];
        if bulge == 0.0 {
            continue;
        }
        let (b, _) = verts[(i + 1) % n];
        let (dx, dy) = (b.x - a.x, b.y - a.y);
        let len = (dx * dx + dy * dy).sqrt();
        // Right normal of the chord direction.
        let (nx, ny) = (dy / len, -dx / len);
        let half = len / 2.0;
        let sag = bulge * half;
        // The arc's radius and centre: centre on the left/right normal.
        let rad = (half * half + sag * sag) / (2.0 * sag.abs());
        let off = rad - sag.abs(); // centre from chord midpoint, away from the arc
        let s = sag.signum();
        let (mx, my) = ((a.x + b.x) / 2.0, (a.y + b.y) / 2.0);
        let (cx, cy) = (mx - nx * s * off, my - ny * s * off);
        let side = ((p.0 - mx) * nx + (p.1 - my) * ny) * s; // > 0: on the arc's side
        let in_circle = (p.0 - cx).powi(2) + (p.1 - cy).powi(2) < rad * rad;
        if side > 0.0 && in_circle {
            inside = !inside;
        }
    }
    inside
}

pub(crate) type Truth = Box<dyn Fn(Point3<f64>) -> bool>;

/// A revolved profile's truth: `(r, y)` in the loop, and for a partial
/// turn the azimuth from `+x` (signed by `dir`) within `[0, angle]`.
pub(crate) fn revolved_truth(verts: Vec<(Point2<f64>, f64)>, angle: Option<(f64, f64)>) -> Truth {
    Box::new(move |p: Point3<f64>| {
        let r = (p.x * p.x + p.z * p.z).sqrt();
        if !in_bulge_loop(&verts, (r, p.y)) {
            return false;
        }
        match angle {
            None => true,
            Some((a, dir)) => {
                let mut phi = (dir * p.z).atan2(p.x);
                if phi < 0.0 {
                    phi += 2.0 * PI;
                }
                phi < a
            }
        }
    })
}

fn rot(v: &[(Point2<f64>, f64)], k: usize) -> Vec<(Point2<f64>, f64)> {
    let mut out = v.to_vec();
    out.rotate_left(k % v.len());
    out
}

fn p2(x: f64, y: f64, b: f64) -> (Point2<f64>, f64) {
    (Point2::new(x, y), b)
}

fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> impl Fn(Point3<f64>) -> bool + Clone {
    move |p: Point3<f64>| p.x > x.0 && p.x < x.1 && p.y > y.0 && p.y < y.1 && p.z > z.0 && p.z < z.1
}

/// One boolean of two bodies through the public door, `None` on a refusal.
fn op(which: &str, a: &Body<f64>, b: &Body<f64>, tol: Tol) -> Result<Option<Body<f64>>, String> {
    let rest =
        |x: &Body<f64>| topo::AtRestBody::validate(x.clone(), tol).map_err(|e| format!("{e:?}"));
    let (a, b) = (rest(a)?, rest(b)?);
    let r = match which {
        "union" => topo::union(&a, &b, tol),
        "intersect" => topo::intersect(&a, &b, tol),
        _ => topo::subtract(&a, &b, tol),
    }
    .map_err(|e| format!("{e:?}").chars().take(200).collect::<String>())?;
    Ok(r.body().map(|b| b.body.clone().into_body()))
}

/// A profile revolved about `y` through the public door, with its
/// truth: a partial turn's direction is read off one deep point of the
/// missing wedge.
pub(crate) fn revolved(
    name: &str,
    verts: Vec<(Point2<f64>, f64)>,
    rev: Revolution<f64>,
    tol: Tol,
) -> (Body<f64>, Truth) {
    let v2 = verts.clone();
    let Ok(body) = std::panic::catch_unwind(|| revolved_about_y(v2, rev, tol)) else {
        panic!("{name}: the revolve refuses")
    };
    let truth = match rev {
        Revolution::Full => revolved_truth(verts, None),
        Revolution::Partial(a) => {
            let (cr, cy) = deep_point(&verts);
            let m = a + (2.0 * PI - a) / 2.0;
            let probe = Point3::new(cr * m.cos(), cy, cr * m.sin());
            let band = geom_core::Band::linear(tol).expect("band");
            let dir = if topo::point_in_solid(&body, probe, band, tol)
                == Ok(topo::SolidContainment::In)
            {
                -1.0
            } else {
                1.0
            };
            revolved_truth(verts, Some((a, dir)))
        }
    };
    (body, truth)
}

/// The corpus: `(name, body, truth)`; a body that does not build is
/// reported and left out.
pub(crate) fn corpus(tol: Tol) -> Vec<(String, Body<f64>, Truth)> {
    let mut out: Vec<(String, Body<f64>, Truth)> = Vec::new();
    let mut revolve = |name: String, verts: Vec<(Point2<f64>, f64)>, rev: Revolution<f64>| {
        let (body, truth) = revolved(&name, verts, rev, tol);
        out.push((name, body, truth));
    };
    let quarter = (PI / 2.0 / 4.0).tan();
    let rect = vec![
        p2(0.0, 0.0, 0.0),
        p2(0.5, 0.0, 0.0),
        p2(0.5, 1.0, 0.0),
        p2(0.0, 1.0, 0.0),
    ];
    let frustum = vec![
        p2(0.0, 0.0, 0.0),
        p2(1.0, 0.0, 0.0),
        p2(0.6, 1.0, 0.0),
        p2(0.0, 1.0, 0.0),
    ];
    let apex = vec![p2(0.0, 0.0, 0.0), p2(1.0, 0.0, 0.0), p2(0.0, 1.0, 0.0)];
    let near_apex = vec![
        p2(0.0, 0.0, 0.0),
        p2(1.0, 0.0, 0.0),
        p2(0.01, 0.99, 0.0),
        p2(0.0, 0.99, 0.0),
    ];
    let hemi = vec![p2(0.0, 0.0, 0.0), p2(1.0, 0.0, quarter), p2(0.0, 1.0, 0.0)];
    let lantern_b = (0.6f64.asin() / 4.0).tan();
    let lantern = vec![
        p2(0.0, 0.0, 0.0),
        p2(1.0, 0.0, lantern_b),
        p2(0.8, 0.6, 0.0),
        p2(0.2, 1.2, 0.0),
        p2(0.0, 1.2, 0.0),
    ];
    let bowl = vec![
        p2(0.0, 0.0, 0.0),
        p2(1.5, 0.0, 0.0),
        p2(1.5, 1.5, 0.0),
        p2(1.0, 1.0, 0.0),
        p2(0.0, 1.0, 0.0),
    ];
    let tube = vec![p2(2.5, 0.0, 1.0), p2(1.5, 0.0, 1.0)];
    let annulus = vec![
        p2(0.2, 0.0, 0.0),
        p2(0.5, 0.0, 0.0),
        p2(0.5, 1.0, 0.0),
        p2(0.2, 1.0, 0.0),
    ];
    // A sphere zone on a bore (sweep's `sphere_zone`'s profile).
    let (big, ylo, yhi) = (2.0f64, -0.5f64, 1.0f64);
    let (xlo, xhi) = (
        (big * big - ylo * ylo).sqrt(),
        (big * big - yhi * yhi).sqrt(),
    );
    let zb = (((yhi / big).asin() - (ylo / big).asin()) / 4.0).tan();
    let zone = vec![
        p2(0.5, ylo, 0.0),
        p2(xlo, ylo, zb),
        p2(xhi, yhi, 0.0),
        p2(0.5, yhi, 0.0),
    ];
    for k in 0..4 {
        revolve(
            format!("rev cylinder rot{k}"),
            rot(&rect, k),
            Revolution::Full,
        );
        revolve(
            format!("rev frustum rot{k}"),
            rot(&frustum, k),
            Revolution::Full,
        );
        revolve(
            format!("rev slit annulus 1.5pi rot{k}"),
            rot(&annulus, k),
            Revolution::Partial(1.5 * PI),
        );
    }
    for k in 0..3 {
        revolve(
            format!("rev cone apex rot{k}"),
            rot(&apex, k),
            Revolution::Full,
        );
        revolve(
            format!("rev hemisphere rot{k}"),
            rot(&hemi, k),
            Revolution::Full,
        );
    }
    revolve("rev cone near apex".into(), near_apex, Revolution::Full);
    revolve("rev lantern".into(), lantern.clone(), Revolution::Full);
    revolve(
        "rev lantern rot2".into(),
        rot(&lantern, 2),
        Revolution::Full,
    );
    revolve("rev bowl".into(), bowl, Revolution::Full);
    revolve("rev sphere zone".into(), zone.clone(), Revolution::Full);
    revolve("rev sphere zone half".into(), zone, Revolution::Partial(PI));
    revolve(
        "rev half cylinder".into(),
        rect.clone(),
        Revolution::Partial(PI),
    );
    revolve(
        "rev half torus".into(),
        tube.clone(),
        Revolution::Partial(PI),
    );
    revolve(
        "rev torus 1.5pi".into(),
        rot(&tube, 1),
        Revolution::Partial(1.5 * PI),
    );
    revolve(
        "rev hemisphere 1.5pi".into(),
        hemi,
        Revolution::Partial(1.5 * PI),
    );

    // Booleans, both operand orders.
    let cyl = disc_of_arcs(2, 0.5, 1.0, tol);
    let cyl_t = |p: Point3<f64>| (p.x * p.x + p.y * p.y).sqrt() < 0.5 && p.z > 0.0 && p.z < 1.0;
    let mut boolean = |name: &str, which: &str, a: &Body<f64>, b: &Body<f64>, t: Shared| {
        for (order, (x, y)) in [("ab", (a, b)), ("ba", (b, a))] {
            if which == "subtract" && order == "ba" {
                continue;
            }
            // A boolean the door refuses, or that leaves nothing, is no
            // body to read.
            if let Ok(Some(body)) = op(which, x, y, tol) {
                out.push((format!("{name} {order}"), body, clone_truth(&t)));
            }
        }
    };
    let cut = boxed((-0.3, 2.0), (-2.0, 2.0), (-2.0, 2.0));
    let cut_body = brick::<f64>((-0.3, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol);
    boolean(
        "cylinder cut by a plane",
        "intersect",
        &cyl,
        &cut_body,
        shared(move |p| cyl_t(p) && cut(p)),
    );
    let fr = revolved_about_y(
        vec![
            p2(0.0, 0.0, 0.0),
            p2(0.5, 0.0, 0.0),
            p2(0.3, 0.5, 0.0),
            p2(0.0, 0.5, 0.0),
        ],
        Revolution::Full,
        tol,
    );
    let block = brick::<f64>((-1.0, 1.0), (-1.0, 0.0), (-1.0, 1.0), tol);
    let blk = boxed((-1.0, 1.0), (-1.0, 0.0), (-1.0, 1.0));
    boolean(
        "frustum on a box",
        "union",
        &fr,
        &block,
        shared(move |p| {
            let r = (p.x * p.x + p.z * p.z).sqrt();
            blk(p) || (p.y > 0.0 && p.y < 0.5 && r < 0.5 - 0.4 * p.y)
        }),
    );
    let ball = ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol);
    let bx = boxed((-0.8, 0.8), (-0.8, 0.8), (-0.8, 0.8));
    let bxb = brick::<f64>((-0.8, 0.8), (-0.8, 0.8), (-0.8, 0.8), tol);
    boolean(
        "ball corners of a box",
        "intersect",
        &ball,
        &bxb,
        shared(move |p| bx(p) && p.x * p.x + p.y * p.y + p.z * p.z < 1.0),
    );
    let cap = ball_poled_z(0.6, Vec3::new(0.0, 0.0, 1.0), tol);
    boolean(
        "cylinder under a ball",
        "union",
        &cyl,
        &cap,
        shared(move |p| cyl_t(p) || p.x * p.x + p.y * p.y + (p.z - 1.0) * (p.z - 1.0) < 0.36),
    );
    let torus = revolved_about_y(tube.clone(), Revolution::Full, tol);
    let tb = boxed((-3.0, 3.0), (0.2, 3.0), (-3.0, 3.0));
    let tbb = brick::<f64>((-3.0, 3.0), (0.2, 3.0), (-3.0, 3.0), tol);
    boolean(
        "torus top cut",
        "intersect",
        &torus,
        &tbb,
        shared(move |p| {
            let r = (p.x * p.x + p.z * p.z).sqrt();
            tb(p) && (r - 2.0).powi(2) + p.y * p.y < 0.25
        }),
    );
    let cone = revolved_about_y(
        vec![p2(0.0, 0.0, 0.0), p2(1.0, 0.0, 0.0), p2(0.0, 1.0, 0.0)],
        Revolution::Full,
        tol,
    );
    let cb = boxed((-0.25, 2.0), (-2.0, 2.0), (-2.0, 2.0));
    let cbb = brick::<f64>((-0.25, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol);
    boolean(
        "cone cut off axis",
        "intersect",
        &cone,
        &cbb,
        shared(move |p| {
            let r = (p.x * p.x + p.z * p.z).sqrt();
            cb(p) && p.y > 0.0 && r < 1.0 - p.y
        }),
    );
    let cube = boxed((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let boss = boss_of_arcs(2, 1.0, 0.3, 0.5, 1.0, tol);
    let rr = |p: Point3<f64>| ((p.x - 0.5).powi(2) + (p.y - 0.5).powi(2)).sqrt();
    out.push((
        "boss on a box".into(),
        boss,
        Box::new({
            let cube = cube.clone();
            move |p| cube(p) || (rr(p) < 0.3 && p.z > 0.5 && p.z < 1.5)
        }),
    ));
    let bored = bored_block_of_arcs(2, 1.0, 1.0, 0.3, tol);
    out.push((
        "bored block".into(),
        bored,
        Box::new(move |p| cube(p) && rr(p) > 0.3),
    ));
    out.push((
        "cylinder 3 arcs".into(),
        disc_of_arcs(3, 0.5, 1.0, tol),
        Box::new(cyl_t),
    ));
    out.extend(fast(tol));
    out
}

/// The kinked prism's profile: a unit square whose corner at `(1, 1)` is
/// cut by an arc of bulge `bulge` from `(1, 0.8)` to `(0.8, 1)`.
pub(crate) fn kinked_profile(bulge: f64) -> Vec<(Point2<f64>, f64)> {
    vec![
        p2(0.0, 0.0, 0.0),
        p2(1.0, 0.0, 0.0),
        p2(1.0, 0.8, bulge),
        p2(0.8, 1.0, 0.0),
        p2(0.0, 1.0, 0.0),
    ]
}

/// **The kinked bored prism** (PR 5697's first review): the kinked
/// prism, its arc meeting the plane `x = 1` at a kink of about 5.7°
/// (bulge 0.357, against a tangent quarter turn's 0.414), bored along
/// `x` at radius 0.2 about `y = 0.4, z = 0.5`. A ray just inside `x = 1`
/// and shallow to it passes the bore's mouth on that face within the
/// band.
pub(crate) fn kinked_bore(tol: Tol) -> (Body<f64>, Truth) {
    let block = sweep::test_support::prism(kinked_profile(KINK), 1.0, tol);
    let plane = profile::SketchPlane::from_frame(geom_core::RigidMotion::axes_yz(Point3::new(
        -0.5, 0.0, 0.0,
    )));
    let bore =
        sweep::test_support::prism_on(plane, vec![p2(0.6, 0.5, 1.0), p2(0.2, 0.5, 1.0)], 2.0, tol);
    let body = op("subtract", &block, &bore, tol)
        .expect("the bore subtracts")
        .expect("a body");
    let verts = kinked_profile(KINK);
    (
        body,
        Box::new(move |p: Point3<f64>| {
            in_bulge_loop(&verts, (p.x, p.y))
                && p.z > 0.0
                && p.z < 1.0
                && (p.y - 0.4).powi(2) + (p.z - 0.5).powi(2) > 0.04
        }),
    )
}

/// The kinked bore's bulge.
pub(crate) const KINK: f64 = 0.357;

/// The corpus's cheap part: a cylinder, the thin discs, the filleted
/// prism and the kinked bore.
pub(crate) fn fast(tol: Tol) -> Vec<(String, Body<f64>, Truth)> {
    let mut out: Vec<(String, Body<f64>, Truth)> = Vec::new();
    let disc =
        |h: f64| move |p: Point3<f64>| (p.x * p.x + p.y * p.y).sqrt() < 0.5 && p.z > 0.0 && p.z < h;
    out.push((
        "cylinder 2 arcs".into(),
        disc_of_arcs(2, 0.5, 1.0, tol),
        Box::new(disc(1.0)),
    ));
    for m in [12.0, 21.0, 30.0] {
        let h = m * tol.eps();
        out.push((
            format!("disc {m}ε thick"),
            disc_of_arcs(2, 0.5, h, tol),
            Box::new(disc(h)),
        ));
    }
    let fillet = kinked_profile(quarter_bulge());
    out.push((
        "filleted prism".into(),
        sweep::test_support::prism(fillet.clone(), 1.0, tol),
        Box::new(move |p: Point3<f64>| {
            in_bulge_loop(&fillet, (p.x, p.y)) && p.z > 0.0 && p.z < 1.0
        }),
    ));
    let (body, truth) = kinked_bore(tol);
    out.push(("kinked bore".into(), body, truth));
    out
}

/// A quarter turn's bulge, `tan(π/8)`.
fn quarter_bulge() -> f64 {
    (PI / 8.0).tan()
}

pub(crate) type Shared = std::rc::Rc<dyn Fn(Point3<f64>) -> bool>;

fn shared(f: impl Fn(Point3<f64>) -> bool + 'static) -> Shared {
    std::rc::Rc::new(f)
}

fn clone_truth(t: &Shared) -> Truth {
    let t = t.clone();
    Box::new(move |p| t(p))
}

/// A point well inside a profile, for reading a partial turn's sense.
fn deep_point(verts: &[(Point2<f64>, f64)]) -> (f64, f64) {
    let (mut best, mut at) = (-1.0, (0.0, 0.0));
    for i in 1..40 {
        for j in 1..40 {
            let (minx, maxx) = verts.iter().fold((f64::MAX, f64::MIN), |a, v| {
                (a.0.min(v.0.x), a.1.max(v.0.x))
            });
            let (miny, maxy) = verts.iter().fold((f64::MAX, f64::MIN), |a, v| {
                (a.0.min(v.0.y), a.1.max(v.0.y))
            });
            let x = minx - 0.5 + (maxx - minx + 1.0) * f64::from(i) / 40.0;
            let y = miny - 0.5 + (maxy - miny + 1.0) * f64::from(j) / 40.0;
            if x <= 0.05 || !in_bulge_loop(verts, (x, y)) {
                continue;
            }
            // Clearance: the smallest step that leaves the loop.
            let mut c = 0.0;
            'grow: for s in 1..200 {
                let h = f64::from(s) * 0.005;
                for k in 0..16 {
                    let a = f64::from(k) * PI / 8.0;
                    if !in_bulge_loop(verts, (x + h * a.cos(), y + h * a.sin())) {
                        break 'grow;
                    }
                }
                c = h;
            }
            if c > best {
                best = c;
                at = (x, y);
            }
        }
    }
    at
}

/// Whether the truth is the same over a sphere of `r` about `p` (26
/// directions): `p` is clear of the boundary by about `r`.
pub(crate) fn robust(t: &Truth, p: Point3<f64>, r: f64) -> Option<bool> {
    let v = t(p);
    for i in -1i32..=1 {
        for j in -1i32..=1 {
            for k in -1i32..=1 {
                if i == 0 && j == 0 && k == 0 {
                    continue;
                }
                let d = Vec3::new(f64::from(i), f64::from(j), f64::from(k)).normalize();
                if t(p + d * r) != v {
                    return None;
                }
            }
        }
    }
    Some(v)
}
