//! rev5856-r1 reviewer probes (not for merge).

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use geom::Curve3;
use geom_core::Tol;

use super::*;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("band")
}

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

struct Cap {
    body: Body<f64>,
    face: FaceKey,
    outward: Vec3<f64>,
}

fn cap(center: Point3<f64>, axis: Vec3<f64>, u_ref: Vec3<f64>) -> Cap {
    let carrier = Curve3::Circle {
        center,
        axis,
        radius: 1.0,
        u_ref,
    };
    let (body, face) = crate::test_support_fixtures::arc_cap(
        carrier,
        (0.0, core::f64::consts::PI),
        Vec3::new(0.0, 0.0, 0.0),
        Tol::witness(),
    );
    Cap {
        body,
        face,
        outward: axis * -1.0,
    }
}

impl Cap {
    fn cleared(&self, at: Point3<f64>, holds: &[Hold]) -> Result<Vec<Sector<f64>>, TouchUnread> {
        let face = self.body.get_face(self.face).unwrap();
        let surface = self.body.get_surface(face.surface).cloned().unwrap();
        Sector::cleared(&self.body, face, surface, self.outward, at, holds)
    }
    fn chord(&self) -> Hold {
        let ends: Vec<VertexKey> = self.body.vertices.keys().collect();
        Hold::Edge(ends[0], ends[1])
    }
}

/// The edge's two half-disc caps about the line through `at` along `x`
/// (wedge from `alpha` ccw to `beta` about +x from +y), judged against a
/// face in z = 0, outward +z, material below, holding `at` inside it,
/// whose chord (y = at.y - d) stands `d` off `at`.
fn judged_d(alpha: f64, beta: f64, d: f64) -> TouchVerdict {
    let at = p(0.1, 0.0, 0.0);
    let (sa, ca) = alpha.sin_cos();
    let (sb, cb) = beta.sin_cos();
    let a = cap(p(0.0, 0.0, 0.0), v(0.0, -sa, ca), v(1.0, 0.0, 0.0));
    let b = cap(p(0.0, 0.0, 0.0), v(0.0, sb, -cb), v(-1.0, 0.0, 0.0));
    let face = cap(p(0.0, -d, 0.0), v(0.0, 0.0, -1.0), v(-1.0, 0.0, 0.0));
    let mut edge = a.cleared(at, &[a.chord()]).expect("a");
    edge.extend(b.cleared(at, &[b.chord()]).expect("b"));
    let edge = Link::of(at, edge).expect("edge link");
    let fs = match face.cleared(at, &[Hold::Inside]) {
        Ok(s) => s,
        Err(e) => return TouchVerdict::Unread(e),
    };
    let face = match Link::of(at, fs) {
        Ok(l) => l,
        Err(e) => return TouchVerdict::Unread(e),
    };
    rest::judge(&edge, &face, band())
}

fn kind(t: &TouchVerdict) -> &'static str {
    match t {
        TouchVerdict::Rest(_) => "Rest",
        TouchVerdict::Crossing => "Crossing",
        TouchVerdict::Unread(_) => "Unread",
    }
}

/// Counterexample search over wedges near coplanar with the face, keels
/// at tiny angles, and face discs of tiny radius. Truth: rest iff
/// 0 < alpha and beta < pi; crossing iff alpha < 0 or beta > pi.
#[test]
fn r1_cleared_sheets_never_decide_wrong() {
    use core::f64::consts::PI;
    let tiny = [1e-1, 1e-3, 1e-6, 1e-8, 1e-10, 1e-12];
    let mut alphas = vec![PI / 3.0, -PI / 3.0];
    let mut betas = vec![2.0 * PI / 3.0, 4.0 * PI / 3.0];
    for t in tiny {
        alphas.push(t);
        alphas.push(-t);
        betas.push(PI - t);
        betas.push(PI + t);
    }
    let mut wrong = Vec::new();
    let mut tally = std::collections::BTreeMap::<(String, &str), usize>::new();
    for d in [0.5, 1e-3, 1e-6, 1e-9, 1e-12] {
        for &al in &alphas {
            for &be in &betas {
                let got = judged_d(al, be, d);
                let truth = if al > 0.0 && be < PI {
                    "Rest"
                } else {
                    "Crossing"
                };
                *tally.entry((format!("d={d:e} truth={truth}"), kind(&got))).or_default() += 1;
                let bad = match (&got, truth) {
                    (TouchVerdict::Rest(_), "Crossing") => true,
                    (TouchVerdict::Crossing, "Rest") => true,
                    _ => false,
                };
                if bad {
                    wrong.push(format!("d={d:e} alpha={al:e} beta-pi={:e}: {got:?}", be - PI));
                }
            }
        }
    }
    for (k, n) in &tally {
        println!("{k:?}: {n}");
    }
    assert!(wrong.is_empty(), "wrong decisions: {wrong:#?}");
}

/// A face disc of zero radius: `at` exactly on the face's arc, held
/// Inside (a misheld tie). Does it decide?
#[test]
fn r1_zero_radius_disc() {
    let at = p(0.1, 0.0, 0.0);
    // arc of unit circle about (0.1, -1, 0) passes through `at`.
    let face = cap(p(0.1, -1.0, 0.0), v(0.0, 0.0, -1.0), v(-1.0, 0.0, 0.0));
    let s = face.cleared(at, &[Hold::Inside]);
    match &s {
        Ok(ss) => {
            let r: Vec<f64> = ss
                .iter()
                .flat_map(|s| s.wedges.iter().map(|w| (w.0 - at).norm()))
                .collect();
            println!("zero-radius disc built: radii {r:?}");
        }
        Err(e) => println!("zero-radius disc refused: {e:?}"),
    }
    let c = cap(p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0));
    let s2 = c.cleared(p(1.0, 0.0, 0.0), &[c.chord()]);
    println!(
        "chord hold at the corner point: {:?}",
        s2.as_ref().map(|v| v.iter().map(|s| s.wedges.len()).collect::<Vec<_>>())
    );
    let s3 = c.cleared(p(1.0 - 1e-13, 0.0, 0.0), &[c.chord()]);
    println!(
        "chord hold 1e-13 off the corner: {:?}",
        s3.as_ref().map(|v| v
            .iter()
            .map(|s| (s.wedges[0].0 - p(1.0 - 1e-13, 0.0, 0.0)).norm())
            .collect::<Vec<_>>())
    );
    if s.is_ok() {
        // judge the edge keel / rest against the zero disc
        for (al, be) in [(1.0, 2.0), (-1.0, 4.0)] {
            let (sa, ca) = f64::sin_cos(al);
            let (sb, cb) = f64::sin_cos(be);
            let a = cap(p(0.0, 0.0, 0.0), v(0.0, -sa, ca), v(1.0, 0.0, 0.0));
            let b = cap(p(0.0, 0.0, 0.0), v(0.0, sb, -cb), v(-1.0, 0.0, 0.0));
            let mut edge = a.cleared(at, &[a.chord()]).unwrap();
            edge.extend(b.cleared(at, &[b.chord()]).unwrap());
            let edge = Link::of(at, edge).unwrap();
            let fl = Link::of(at, face.cleared(at, &[Hold::Inside]).unwrap());
            match fl {
                Ok(fl) => println!(
                    "zero disc judged ({al},{be}): {:?}",
                    rest::judge(&edge, &fl, band())
                ),
                Err(e) => println!("zero disc link refused: {e:?}"),
            }
        }
    }
}

/// An eccentric ellipse, points near its major vertex and around it.
#[test]
fn r1_eccentric_ellipse_bound() {
    let axis = v(0.0, 0.0, 1.0);
    let u_ref = v(1.0, 0.0, 0.0);
    let w = axis.cross(u_ref);
    for (su, sv) in [(1000.0, 1e-3), (1e-3, 1000.0), (5.0, 4.999_999), (1.0, 1.0)] {
        let e = Curve3::Ellipse {
            center: Point3::origin(),
            axis,
            semi_u: su,
            semi_v: sv,
            u_ref,
        };
        let n = 2_000_000;
        let walk: Vec<Point3<f64>> = (0..n)
            .map(|k| {
                let t = core::f64::consts::TAU * f64::from(k) / f64::from(n);
                Point3::origin() + u_ref * (su * t.cos()) + w * (sv * t.sin())
            })
            .collect();
        let step = core::f64::consts::TAU * su.max(sv) / f64::from(n);
        let big = su.max(sv);
        for (a, b, h) in [
            (1.0001, 0.0, 0.0),
            (1.1, 0.0, 0.0),
            (0.999, 0.0, 0.0),
            (0.5, 0.0, 0.0),
            (0.0, 0.0, 0.0),
            (0.999, 0.0, 1e-3),
            (0.7, 0.2, 0.0),
        ] {
            let (qa, qb) = if su >= sv { (a * big, b * big) } else { (b * big, a * big) };
            let q = Point3::origin() + u_ref * qa + w * qb + axis * h;
            let bound = conic_clearance(&e, q).unwrap();
            let least = walk.iter().map(|&x| (x - q).norm()).fold(f64::MAX, f64::min);
            println!("{su}x{sv} q=({qa},{qb},{h}): bound {bound:e} least {least:e}");
            assert!(bound <= least + step, "{su}x{sv} q=({qa},{qb},{h}): {bound} > {least}");
        }
    }
    // degenerate
    let c0 = Curve3::Circle {
        center: Point3::origin(),
        axis,
        radius: 0.0,
        u_ref,
    };
    println!("zero circle at (1,0,0): {:?}", conic_clearance(&c0, p(1.0, 0.0, 0.0)));
    let e0 = Curve3::Ellipse {
        center: Point3::origin(),
        axis,
        semi_u: 0.0,
        semi_v: 1.0,
        u_ref,
    };
    println!("zero-semi ellipse at (2,0,0): {:?}", conic_clearance(&e0, p(2.0, 0.0, 0.0)));
    let cn = Curve3::Circle {
        center: Point3::origin(),
        axis,
        radius: -1.0,
        u_ref,
    };
    println!("negative circle at (1,0,0) (on it): {:?}", conic_clearance(&cn, p(1.0, 0.0, 0.0)));
    let huge = Curve3::Circle {
        center: Point3::origin(),
        axis,
        radius: 1e9,
        u_ref,
    };
    let q = p(1e9 + 1e-7, 0.0, 0.0);
    println!("huge circle 1e-7 off: {:?}", conic_clearance(&huge, q));
    let q = p((1e9f64).sqrt() * (1e9f64).sqrt() * 0.6, 1e9 * 0.8, 0.0);
    println!("huge circle on it (0.6,0.8): {:?}", conic_clearance(&huge, q));
}
