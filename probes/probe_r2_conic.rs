//! Reviewer probe (reach-dual3805-r2): the tilted-cut drum against balls
//! and rods, widened past the PR's poses — scales, rigid re-poses, both
//! operand orders, results reused — every built body checked against an
//! oracle written here (closed-form volume, analytic membership sampled
//! through `point_in_solid`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp};

const TILT: f64 = 0.3;

type Member = Box<dyn Fn(Point3<f64>) -> f64>; // signed distance-ish: < 0 inside

fn drum(s: f64, tilt: f64) -> (Body<f64>, Member, f64) {
    drum_n(s, Vec3::new(tilt.sin(), 0.0, tilt.cos()), 0.5)
}

/// The drum's part below the plane through `(0, 0, zc·s)` with normal `n`.
fn drum_n(s: f64, n: Vec3<f64>, zc: f64) -> (Body<f64>, Member, f64) {
    let tol = Tol::witness();
    let r = 0.5 * s;
    let cyl = sweep::test_support::prism(
        vec![(Point2::new(-r, 0.0), 1.0), (Point2::new(r, 0.0), 1.0)],
        s,
        tol,
    );
    let n = n.normalize();
    let o = Point3::new(0.0, 0.0, zc * s);
    let plane = topo::splitting::SplitPlane { origin: o, normal: n };
    let res = topo::splitting::split(&cyl, &plane, tol).expect("split");
    let topo::splitting::SplitPart::Body(below) = res.below else {
        panic!("below")
    };
    let m: Member = Box::new(move |p: Point3<f64>| {
        let rad = p.x.hypot(p.y) - r;
        let fl = (-p.z).max(p.z - s);
        let top = (p - o).dot(n);
        rad.max(fl).max(top)
    });
    (below, m, PI * r * r * zc * s)
}

fn ball(r: f64, c: [f64; 3]) -> (Body<f64>, Member, f64) {
    let at0 = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    let to = Affine3::translation(Vec3::new(c[0], c[1], c[2]));
    let b = topo::transform_rigid(&at0, &to, Tol::witness()).unwrap();
    let cc = Point3::from_array(c);
    (b, Box::new(move |p| (p - cc).norm() - r), 4.0 / 3.0 * PI * r.powi(3))
}

fn rod(r: f64, x: f64, y: f64, z0: f64, h: f64) -> (Body<f64>, Member, f64) {
    let b = sweep::test_support::prism_at(
        vec![(Point2::new(x - r, y), 1.0), (Point2::new(x + r, y), 1.0)],
        z0,
        h,
        Tol::witness(),
    );
    (
        b,
        Box::new(move |p: Point3<f64>| {
            ((p.x - x).hypot(p.y - y) - r).max(z0 - p.z).max(p.z - z0 - h)
        }),
        PI * r * r * h,
    )
}

fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<topo::BooleanResult<f64>, topo::BooleanError> {
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

/// Check one result body: tiers, volume against `expected` (if known),
/// and membership against `inside` at random points in `lo..hi`.
fn check(
    label: &str,
    body: &Body<f64>,
    expected: Option<f64>,
    inside: &dyn Fn(Point3<f64>) -> f64,
    lo: [f64; 3],
    hi: [f64; 3],
    scale: f64,
    report: &mut Vec<String>,
) {
    let tol = Tol::witness();
    if topo::validate(body).is_err() || topo::validate_closed(body).is_err() {
        report.push(format!("FAIL {label}: tier 1/2"));
        return;
    }
    if let Err(e) = topo::validate_geometric(body, tol) {
        report.push(format!("FAIL {label}: tier 3 {e:?}"));
    }
    match topo::mass_properties(body, tol) {
        Ok(p) => {
            if let Some(v) = expected {
                let err = (p.volume - v).abs();
                if err > p.volume_pad + 1e-12 * scale.powi(3) {
                    report.push(format!("FAIL {label}: volume {} vs {v} (pad {})", p.volume, p.volume_pad));
                } else {
                    report.push(format!("ok {label}: volume err {err:.2e} pad {:.2e}", p.volume_pad));
                }
            }
        }
        Err(e) => report.push(format!("NOTE {label}: mass properties refused {e:?}")),
    }
    let band = geom_core::Band::linear(tol).unwrap();
    let mut rng = Lcg(0x5eed ^ label.len() as u64);
    let (mut agree, mut skip, mut bad, mut refused) = (0, 0, 0, 0);
    for _ in 0..150 {
        let p = Point3::new(
            lo[0] + (hi[0] - lo[0]) * rng.next(),
            lo[1] + (hi[1] - lo[1]) * rng.next(),
            lo[2] + (hi[2] - lo[2]) * rng.next(),
        );
        let d = inside(p);
        if d.abs() < (4.0 * Tol::witness().eps()).max(1e-6 * scale) {
            skip += 1;
            continue;
        }
        match topo::point_in_solid(body, p, band, tol) {
            Ok(topo::SolidContainment::In) if d < 0.0 => agree += 1,
            Ok(topo::SolidContainment::Out) if d > 0.0 => agree += 1,
            Ok(got) => {
                bad += 1;
                if bad <= 3 {
                    report.push(format!("FAIL {label}: point {p:?} oracle d={d:.3e} kernel {got:?}"));
                }
            }
            Err(_) => refused += 1,
        }
    }
    report.push(format!("   {label}: membership agree {agree} bad {bad} refused {refused} skip {skip}"));
}

#[derive(Clone, Copy)]
enum Rel {
    /// B strictly inside A.
    Held,
    /// Disjoint.
    Apart,
    /// A strictly inside B.
    Wraps,
}

/// Every op in both orders against the closed form the relation gives.
fn exercise(
    label: &str,
    a: &(Body<f64>, Member, f64),
    b: &(Body<f64>, Member, f64),
    rel: Rel,
    scale: f64,
    pose: Option<Affine3<f64>>,
    report: &mut Vec<String>,
) {
    let (va, vb) = (a.2, b.2);
    let (ab, bb) = match pose {
        Some(m) => (
            topo::transform_rigid(&a.0, &m, Tol::witness()).unwrap(),
            topo::transform_rigid(&b.0, &m, Tol::witness()).unwrap(),
        ),
        None => (a.0.clone(), b.0.clone()),
    };
    let back = pose.map(|m| m.inverse());
    let ma = |p: Point3<f64>| (a.1)(back.map_or(p, |m| m.transform_point(p)));
    let mb = |p: Point3<f64>| (b.1)(back.map_or(p, |m| m.transform_point(p)));
    let (u, i, amb, bma) = match rel {
        Rel::Held => (Some(va), Some(vb), Some(va - vb), None),
        Rel::Apart => (Some(va + vb), None, Some(va), Some(vb)),
        Rel::Wraps => (Some(vb), Some(va), None, Some(vb - va)),
    };
    // Sampling box: around both, in the posed frame.
    let c = Point3::new(0.0, 0.0, 0.5 * scale);
    let c = pose.map_or(c, |m| m.transform_point(c));
    let h = 1.0 * scale;
    let lo = [c.x - h, c.y - h, c.z - h];
    let hi = [c.x + h, c.y + h, c.z + h];
    let cases: [(&str, BooleanOp, &Body<f64>, &Body<f64>, Option<f64>, Box<dyn Fn(Point3<f64>) -> f64 + '_>); 6] = [
        ("A∪B", BooleanOp::Union, &ab, &bb, u, Box::new(|p| ma(p).min(mb(p)))),
        ("B∪A", BooleanOp::Union, &bb, &ab, u, Box::new(|p| ma(p).min(mb(p)))),
        ("A∩B", BooleanOp::Intersect, &ab, &bb, i, Box::new(|p| ma(p).max(mb(p)))),
        ("B∩A", BooleanOp::Intersect, &bb, &ab, i, Box::new(|p| ma(p).max(mb(p)))),
        ("A∖B", BooleanOp::Subtract, &ab, &bb, amb, Box::new(|p| ma(p).max(-mb(p)))),
        ("B∖A", BooleanOp::Subtract, &bb, &ab, bma, Box::new(|p| mb(p).max(-ma(p)))),
    ];
    for (opl, op, x, y, want, member) in cases {
        let l = format!("{label} {opl}");
        match run(op, x, y) {
            Err(e) => report.push(format!("REFUSE {l}: {e:?}")),
            Ok(out) => match (out.body(), want) {
                (Some(bd), Some(v)) => check(&l, &bd.body, Some(v), &*member, lo, hi, scale, report),
                (None, None) => report.push(format!("ok {l}: empty")),
                (Some(_), None) => report.push(format!("FAIL {l}: body where empty expected")),
                (None, Some(v)) => report.push(format!("FAIL {l}: empty where {v} expected")),
            },
        }
    }
}

fn poses(s: f64) -> Vec<(&'static str, Option<Affine3<f64>>)> {
    vec![
        ("id", None),
        (
            "rot",
            Some(Affine3::rotation_about_axis(
                Point3::new(0.1 * s, -0.2 * s, 0.3 * s),
                Vec3::new(0.3, -0.7, 0.648).normalize(),
                1.1,
            )),
        ),
    ]
}

#[test]
fn probe_r2_widened_poses() {
    let mut report = Vec::new();
    for s in [1.0, 1e-3, 1e3] {
        let a = drum(s, TILT);
        for (pl, pose) in poses(s) {
            let tag = |x: &str| format!("[s {s:e} {pl}] {x}");
            // Held inside, PR poses and new ones.
            for (r, c) in [(0.18, [0.0, 0.0, 0.3]), (0.12, [-0.3, 0.1, 0.4]), (0.1, [0.25, -0.25, 0.3])] {
                let b = ball(r * s, [c[0] * s, c[1] * s, c[2] * s]);
                exercise(&tag(&format!("held ball r{r} {c:?}")), &a, &b, Rel::Held, s, pose, &mut report);
            }
            // Ball inside the wall's carrier but ABOVE the cut: disjoint.
            let b = ball(0.15 * s, [0.3 * s, 0.0, 0.62 * s]);
            exercise(&tag("ball above cut, in carrier"), &a, &b, Rel::Apart, s, pose, &mut report);
            // Ball inside the carrier below the floor: disjoint.
            let b = ball(0.15 * s, [0.0, 0.1 * s, -0.2 * s]);
            exercise(&tag("ball below floor"), &a, &b, Rel::Apart, s, pose, &mut report);
            // Ball outside the carrier, near the rim: disjoint.
            let b = ball(0.2 * s, [0.6 * s, 0.4 * s, 0.5 * s]);
            exercise(&tag("ball outside near rim"), &a, &b, Rel::Apart, s, pose, &mut report);
            // Held rod.
            let b = rod(0.3 * s, 0.15 * s, 0.05 * s, 0.05 * s, 0.3 * s);
            exercise(&tag("held rod"), &a, &b, Rel::Held, s, pose, &mut report);
            // Coaxial wider rod wrapping the drum.
            let b = rod(0.7 * s, 0.0, 0.0, -0.1 * s, 1.2 * s);
            exercise(&tag("coaxial wrapping rod"), &a, &b, Rel::Wraps, s, pose, &mut report);
            // Off-axis wrapping rod.
            let b = rod(0.8 * s, 0.1 * s, -0.15 * s, -0.2 * s, 1.3 * s);
            exercise(&tag("offset wrapping rod"), &a, &b, Rel::Wraps, s, pose, &mut report);
            // Rod above the cut inside the carrier: disjoint.
            let b = rod(0.2 * s, 0.0, 0.0, 0.75 * s, 0.2 * s);
            exercise(&tag("rod above cut"), &a, &b, Rel::Apart, s, pose, &mut report);
        }
    }
    let fails: Vec<_> = report.iter().filter(|l| l.starts_with("FAIL")).collect();
    for l in &report {
        println!("{l}");
    }
    assert!(fails.is_empty(), "{} failures", fails.len());
}

/// Results reused as operands: (drum ∖ ball) ∖ ball2, (drum ∖ ball) ∪ ball.
#[test]
fn probe_r2_reuse() {
    let mut report = Vec::new();
    let a = drum(1.0, TILT);
    let b1 = ball(0.15, [0.0, 0.0, 0.25]);
    let b2 = ball(0.1, [-0.25, 0.1, 0.35]);
    let step = run(BooleanOp::Subtract, &a.0, &b1.0).unwrap();
    let c = step.body().unwrap().body.clone();
    let mc = |p: Point3<f64>| (a.1)(p).max(-(b1.1)(p));
    let lo = [-0.6, -0.6, -0.1];
    let hi = [0.6, 0.6, 0.8];
    match run(BooleanOp::Subtract, &c, &b2.0) {
        Ok(r) => check("(A∖B1)∖B2", &r.body().unwrap().body, Some(a.2 - b1.2 - b2.2), &|p| mc(p).max(-(b2.1)(p)), lo, hi, 1.0, &mut report),
        Err(e) => report.push(format!("REFUSE (A∖B1)∖B2: {e:?}")),
    }
    match run(BooleanOp::Union, &c, &b1.0) {
        Ok(r) => match r.body() {
            Some(bd) => check("(A∖B1)∪B1", &bd.body, Some(a.2), &|p| (a.1)(p), lo, hi, 1.0, &mut report),
            None => report.push("FAIL (A∖B1)∪B1 empty".into()),
        },
        Err(e) => report.push(format!("REFUSE (A∖B1)∪B1: {e:?}")),
    }
    match run(BooleanOp::Intersect, &c, &b2.0) {
        Ok(r) => match r.body() {
            Some(bd) => check("(A∖B1)∩B2", &bd.body, Some(b2.2), &|p| (b2.1)(p), lo, hi, 1.0, &mut report),
            None => report.push("FAIL (A∖B1)∩B2 empty".into()),
        },
        Err(e) => report.push(format!("REFUSE (A∖B1)∩B2: {e:?}")),
    }
    for l in &report {
        println!("{l}");
    }
    assert!(!report.iter().any(|l| l.starts_with("FAIL")));
}

/// Wall placement on cuts whose extremes fall inside wall faces (the
/// plane turned about z), steep cuts, and points within 1e-7 of the rim.
#[test]
fn probe_r2_wall_placement() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let mut fails = Vec::new();
    let mut tally = [0usize; 3];
    for (tilt, psi, zc) in [(0.3_f64, 0.7_f64, 0.5_f64), (0.3, 2.0, 0.5), (0.9, 0.4, 0.6), (1.1, -0.9, 0.7), (0.5, 0.0, 0.45)] {
        let n = Vec3::new(tilt.sin() * psi.cos(), tilt.sin() * psi.sin(), tilt.cos());
        let (a, _, _) = drum_n(1.0, n, zc);
        let walls: Vec<_> = a
            .faces()
            .filter(|(_, f)| matches!(a.get_surface(f.surface), Some(geom::Surface::Cylinder { .. })))
            .map(|(k, _)| k)
            .collect();
        let cut = |x: f64, y: f64| zc - (n.x * x + n.y * y) / n.z;
        for k in 0..60 {
            let az = (f64::from(k) + 0.37) * core::f64::consts::TAU / 60.0;
            let (x, y) = (0.5 * az.cos(), 0.5 * az.sin());
            let c = cut(x, y);
            let mut zs: Vec<f64> = (0..12).map(|j| -0.1 + 1.2 * f64::from(j) / 11.0).collect();
            zs.extend([c - 1e-7, c + 1e-7, c - 1e-4, c + 1e-4, 1e-7, -1e-7]);
            for z in zs {
                let inside = z < c && z > 0.0 && z < 1.0;
                let q = Point3::new(x, y, z);
                let mut ins = 0;
                let mut none = 0;
                for &f in &walls {
                    match topo::curved_face_containment(&a, f, q, band) {
                        Ok(Some(topo::FaceContainment::In)) => ins += 1,
                        Ok(Some(topo::FaceContainment::Out)) => {}
                        Ok(None) | Ok(Some(_)) => none += 1,
                        Err(_) => none += 1,
                    }
                }
                if none > 0 {
                    tally[2] += 1;
                    continue;
                }
                if ins != usize::from(inside) {
                    fails.push(format!("tilt {tilt} psi {psi}: ({x:.4},{y:.4},{z}) cut {c}: ins {ins} want {inside}"));
                } else {
                    tally[usize::from(inside)] += 1;
                }
            }
        }
        // Held ball on that drum, every op.
        let mut report = Vec::new();
        let b = ball(0.1, [0.1, -0.1, 0.2]);
        let am = drum_n(1.0, n, zc);
        let reach = 0.5 * tilt.tan();
        if zc - reach > 0.0 && zc + reach < 1.0 && (am.1)(Point3::new(0.1, -0.1, 0.2)) < -0.1 {
            exercise(&format!("tilt {tilt} psi {psi} held ball"), &am, &b, Rel::Held, 1.0, None, &mut report);
        }
        for l in &report {
            println!("{l}");
            if l.starts_with("FAIL") {
                fails.push(l.clone());
            }
        }
    }
    println!("placement tally out/in/none: {tally:?}");
    for f in fails.iter().take(20) {
        println!("{f}");
    }
    assert!(fails.is_empty(), "{} fails", fails.len());
}

#[test]
fn probe_r2_reuse_refusal_edge() {
    let a = drum(1.0, TILT);
    let b1 = ball(0.15, [0.0, 0.0, 0.25]);
    let c = run(BooleanOp::Subtract, &a.0, &b1.0).unwrap().body().unwrap().body.clone();
    if let Err(topo::BooleanError::CurvedPierceUnsupported { edge, face, .. }) = run(BooleanOp::Union, &c, &b1.0) {
        let e = c.get_edge(edge).unwrap();
        println!("edge carrier {:?}", c.get_curve_geom(e.curve).and_then(topo::CurveGeom::certified).map(|g| g.carrier().clone()));
        println!("face surface {:?}", b1.0.get_face(face).and_then(|f| b1.0.get_surface(f.surface)));
    }
}
