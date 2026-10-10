//! Design probe (not for main): spellings of a point turned about an
//! axis, f64 values dumped for an exact reference, Interval widths.
#![allow(clippy::all)]
use geom_brep::{MappedCurve, SweepRange};
use geom_core::{Affine3, Bounds, Interval, Mat3, Point2, Point3, Real, Vec3};

fn s0<T: Real>(p: Point3<T>, q: Point3<T>, n: Vec3<T>, th: T) -> Point3<T> {
    Affine3::rotation_about_axis(q, n, th).transform_point(p)
}
fn s1<T: Real>(p: Point3<T>, q: Point3<T>, n: Vec3<T>, th: T) -> Point3<T> {
    let v = p - q;
    let d = Mat3::identity_minus_rotation_about(n, th) * v;
    Point3::new(p.x - d.x, p.y - d.y, p.z - d.z)
}
fn s2<T: Real>(p: Point3<T>, q: Point3<T>, n: Vec3<T>, th: T) -> Point3<T> {
    let v = p - q;
    let r = Mat3::rotation_about(n, th) * v;
    q + r
}
// Rodrigues anchored at p, half-angle (Arc2::point_from in 3-D).
fn s3<T: Real>(p: Point3<T>, q: Point3<T>, n: Vec3<T>, th: T) -> Point3<T> {
    let n = n.normalize();
    let v = p - q;
    let half = T::from_f64(0.5);
    let two = T::from_f64(2.0);
    let sin = th.sin();
    let cm1 = -(two * (th * half).sin().powi(2));
    let nxv = n.cross(v);
    let nnv = n.cross(nxv); // = -(v_perp)
    // (R - I) v = sin·(n×v) + (1-cos)·n×(n×v) = sin·nxv - cm1·nnv
    p + (nxv * sin - nnv * cm1)
}

fn w(e: Interval) -> f64 { e.hi() - e.lo() }
fn pw(p: Point3<Interval>) -> f64 { w(p.x).max(w(p.y)).max(w(p.z)) }
fn iv(x: f64) -> Interval { Interval::from_f64(x) }
fn ip(p: Point3<f64>) -> Point3<Interval> { Point3::new(iv(p.x), iv(p.y), iv(p.z)) }
fn ivv(p: Vec3<f64>) -> Vec3<Interval> { Vec3::new(iv(p.x), iv(p.y), iv(p.z)) }

struct Fx { name: &'static str, p: Point3<f64>, q: Point3<f64>, n: Vec3<f64>, angle: f64 }

fn fixtures() -> Vec<Fx> {
    let tilt = Vec3::new(0.3, 0.2, 1.0);
    let tn = tilt * (1.0 / tilt.norm());
    // a meridian-like circle about the origin, radius R, axis perpendicular to tilt
    let x = Vec3::new(1.0, 0.0, 0.0);
    let u0 = x - tn * tn.dot(x);
    let u = u0 * (1.0 / u0.norm());
    let v = tn.cross(u);
    let r = 0.75 * 1e-9 / 3.2e-16;
    let mer_axis = u * 0.7f64.sin() - v * 0.7f64.cos();
    let south = Point3::origin() + (tn * -1.0) * r;
    let far = Point3::new(1000.0, -700.0, 300.0);
    let tiltax = Vec3::new(0.2, 1.0, -0.4);
    let tq = Point3::new(1234.5, -987.25, 412.125);
    let tpt = tq + u * 1.5 + tn * 0.25;
    vec![
        Fx { name: "lune R=2.3e6 q=0", p: south, q: Point3::origin(), n: mer_axis, angle: core::f64::consts::PI },
        Fx { name: "origin r=1", p: Point3::new(0.6, 0.8, 0.3), q: Point3::origin(), n: tn, angle: 1.9 },
        Fx { name: "near rim", p: Point3::new(2.0, 2.0, 3.0), q: Point3::new(1.0, 2.0, 3.0), n: Vec3::new(0.0,0.0,1.0), angle: core::f64::consts::TAU },
        Fx { name: "far rim 1e3", p: far + Vec3::new(2.0, 2.0, 0.0), q: far + Vec3::new(1.0, 2.0, 0.0), n: Vec3::new(0.0,0.0,1.0), angle: core::f64::consts::TAU },
        Fx { name: "far tilted 1e3", p: tpt, q: tq, n: tiltax, angle: 1.9 },
        Fx { name: "far tilted 1e5", p: tpt + Vec3::new(1e5, -3e4, 2e4), q: tq + Vec3::new(1e5, -3e4, 2e4), n: tiltax, angle: 1.9 },
        Fx { name: "big r=1e3 near axis", p: Point3::new(1000.5, 3.0, 1.0), q: Point3::new(0.5, -0.25, 1.0), n: tn, angle: 2.5 },
    ]
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    let fx = fixtures();
    if mode == "f64" {
        // dump: name p q n theta out0..out3
        for f in &fx {
            for i in 0..=16 {
                let th = (i as f64 / 16.0) * f.angle;
                let o = [s0(f.p, f.q, f.n, th), s1(f.p, f.q, f.n, th), s2(f.p, f.q, f.n, th), s3(f.p, f.q, f.n, th)];
                print!("{}|{:?},{:?},{:?}|{:?},{:?},{:?}|{:?},{:?},{:?}|{:?}", f.name, f.p.x, f.p.y, f.p.z, f.q.x, f.q.y, f.q.z, f.n.x, f.n.y, f.n.z, th);
                for p in o { print!("|{:?},{:?},{:?}", p.x, p.y, p.z); }
                println!();
            }
        }
        return;
    }
    // Interval widths: exact inputs; then angle carrying 1 ulp; then wide axis origin.
    println!("{:<22} {:>10} {:>10} {:>10} {:>10}   (widest over s=i/16)", "fixture / case", "shipped", "I-R@p", "q+R(p-q)", "rodr@p");
    for f in &fx {
        for (case, aw, qw) in [("exact", 0.0, 0.0), ("angle±ulp", 1.0, 0.0), ("q±1e-12", 0.0, 1e-12), ("q±1e-9", 0.0, 1e-9), ("p±1e-12", 0.0, -1e-12)] {
            let mut worst = [0.0f64; 4];
            let mut at0 = [0.0f64; 4];
            for i in 0..=16 {
                let thf = (i as f64 / 16.0) * f.angle;
                let th = if aw > 0.0 { let u = f64::EPSILON * thf.abs().max(1e-300); Interval::from_bounds(thf - u, thf + u) } else { iv(thf) };
                let pwid = if qw < 0.0 { -qw } else { 0.0 }; let qw = qw.max(0.0);
                let qi = Point3::new(Interval::from_bounds(f.q.x - qw, f.q.x + qw), Interval::from_bounds(f.q.y - qw, f.q.y + qw), Interval::from_bounds(f.q.z - qw, f.q.z + qw));
                let p = Point3::new(Interval::from_bounds(f.p.x - pwid, f.p.x + pwid), Interval::from_bounds(f.p.y - pwid, f.p.y + pwid), Interval::from_bounds(f.p.z - pwid, f.p.z + pwid)); let n = ivv(f.n);
                let o = [pw(s0(p, qi, n, th)), pw(s1(p, qi, n, th)), pw(s2(p, qi, n, th)), pw(s3(p, qi, n, th))];
                for k in 0..4 { worst[k] = worst[k].max(o[k]); if i == 0 { at0[k] = o[k]; } }
            }
            println!("{:<12} {:<9} {:>10.2e} {:>10.2e} {:>10.2e} {:>10.2e}   at s=0: {:.1e} {:.1e} {:.1e} {:.1e}", f.name.split(' ').next().unwrap().to_string() + " " + f.name.split(' ').nth(1).unwrap_or(""), case, worst[0], worst[1], worst[2], worst[3], at0[0], at0[1], at0[2], at0[3]);
        }
    }
    // Restriction chain (0.3,0.7) x64, far rim and far tilted, each spelling evaluating range.at(s)*angle
    println!("\nchain (0.3,0.7)/(a,1) widest of s=0,1/2,1 at N=1/16/64");
    for f in fx.iter().filter(|f| f.name.starts_with("far") || f.name.starts_with("near")) {
        for chain in ["(0.3,0.7)", "(a,1)", "(1/2,1)"] {
            let mut r = SweepRange::<Interval>::whole();
            let mut rows = [[0.0f64; 3]; 4];
            for k in 0..64 {
                let a = 0.37 + 0.011 * (((k * 7) % 5) as f64);
                let (a0, a1) = match chain { "(0.3,0.7)" => (iv(0.3), iv(0.7)), "(a,1)" => (iv(a), iv(1.0)), _ => (iv(0.5), iv(1.0)) };
                r = r.restrict(a0, a1);
                let col = match k { 0 => Some(0), 15 => Some(1), 63 => Some(2), _ => None };
                if let Some(c) = col {
                    for s in [0.0, 0.5, 1.0] {
                        let th = r.at(iv(s)) * iv(f.angle);
                        let (p, q, n) = (ip(f.p), ip(f.q), ivv(f.n));
                        let o = [pw(s0(p, q, n, th)), pw(s1(p, q, n, th)), pw(s2(p, q, n, th)), pw(s3(p, q, n, th))];
                        for kk in 0..4 { rows[kk][c] = rows[kk][c].max(o[kk]); }
                    }
                }
            }
            let names = ["shipped", "I-R@p", "q+R(p-q)", "rodr@p"];
            for kk in 0..4 { println!("{:<16} {:<10} {:<9} {:.2e} / {:.2e} / {:.2e}", f.name, chain, names[kk], rows[kk][0], rows[kk][1], rows[kk][2]); }
        }
    }
    let _ = MappedCurve::<f64>::ExtrudedPoint { point: Point2::new(0.0,0.0), place: Affine3::identity(), vec: Vec3::new(0.0,0.0,1.0), range: SweepRange::whole() };
}
