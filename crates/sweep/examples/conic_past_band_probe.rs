//! **Where a plane × quadric conic section stops certifying**: a
//! measurement probe for `conic-section-just-past-its-band-fails-certification-downstream`.
//!
//! Two families, swept toward their degenerate limit:
//!
//! - `cone`: a frustum (half-angle `α`, height `S`, top radius `S/2`,
//!   revolved about `y`) split by the plane through `(0, S/2, 0)` whose
//!   normal stands `φ = f·(π/2 − α)` off the axis (`f = 1` the parabola).
//! - `cyl`: a cylinder (radius `S`, height `S`, about `z`) split by the
//!   plane through `(0, 0, S/2)` whose normal stands `φ = f·π/2` off it.
//!
//! Every pose prints one `POSE` line: the split's outcome, and what the
//! arm's ellipse reads over the arcs the face consumes: its own residual
//! against both surfaces in double-double, the quadric chart image's map
//! residual, the image's certificate, and the candidate admission
//! quantities.
//!
//! `cargo run -p sweep --release --example conic_past_band_probe`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::intersect::{
    PlaneConeSection, PlaneCylinderSection, plane_cone_section, plane_cylinder_section,
};
use geom_brep::{FocalImage, Pcurve, PcurveCache, Reach, chart_pcurve};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use topo::splitting::split;
use topo::{AtRestBody, Body};

#[derive(Clone, Copy, Debug)]
struct Dd(f64, f64);

fn two_sum(a: f64, b: f64) -> Dd {
    let s = a + b;
    let v = s - a;
    Dd(s, (a - (s - v)) + (b - v))
}

fn quick(a: f64, b: f64) -> Dd {
    let s = a + b;
    Dd(s, b - (s - a))
}

impl Dd {
    fn of(x: f64) -> Self {
        Self(x, 0.0)
    }
    fn add(self, o: Self) -> Self {
        let s = two_sum(self.0, o.0);
        quick(s.0, s.1 + self.1 + o.1)
    }
    fn sub(self, o: Self) -> Self {
        self.add(Self(-o.0, -o.1))
    }
    fn mul(self, o: Self) -> Self {
        let p = self.0 * o.0;
        quick(p, self.0.mul_add(o.0, -p) + (self.0 * o.1 + self.1 * o.0))
    }
    fn div(self, o: Self) -> Self {
        let q1 = self.0 / o.0;
        let r = self.sub(o.mul(Self::of(q1)));
        let q2 = r.0 / o.0;
        let r = r.sub(o.mul(Self::of(q2)));
        quick(q1, q2).add(Self::of(r.0 / o.0))
    }
    fn sqrt(self) -> Self {
        if self.0 <= 0.0 {
            return Self::of(0.0);
        }
        let x = self.0.sqrt();
        let r = self.sub(Self::of(x).mul(Self::of(x)));
        quick(x, r.0 / (2.0 * x))
    }
    /// `(sin x, cos x)` by Taylor series, `|x| ≤ 2`.
    fn sin_cos(x: f64) -> (Self, Self) {
        let x = Self::of(x);
        let x2 = x.mul(x);
        let (mut s, mut c) = (Self::of(0.0), Self::of(0.0));
        let (mut ts, mut tc) = (x, Self::of(1.0));
        for k in 0..40 {
            s = s.add(ts);
            c = c.add(tc);
            let k = f64::from(k);
            ts = ts.mul(x2).div(Self::of(-(2.0 * k + 2.0) * (2.0 * k + 3.0)));
            tc = tc.mul(x2).div(Self::of(-(2.0 * k + 1.0) * (2.0 * k + 2.0)));
        }
        (s, c)
    }
}

fn dot3(a: [Dd; 3], b: [Dd; 3]) -> Dd {
    a[0].mul(b[0]).add(a[1].mul(b[1])).add(a[2].mul(b[2]))
}

fn rel(p: Point3<f64>, o: Point3<f64>) -> [Dd; 3] {
    [
        Dd::of(p.x).sub(Dd::of(o.x)),
        Dd::of(p.y).sub(Dd::of(o.y)),
        Dd::of(p.z).sub(Dd::of(o.z)),
    ]
}

fn dv(v: Vec3<f64>) -> [Dd; 3] {
    [Dd::of(v.x), Dd::of(v.y), Dd::of(v.z)]
}

/// The point's distance from the plane, in double-double (the stored
/// normal read as exact and normalised).
fn plane_miss(p: Point3<f64>, q: Point3<f64>, n: Vec3<f64>) -> f64 {
    let nd = dv(n);
    let len = dot3(nd, nd).sqrt();
    dot3(rel(p, q), nd).div(len).0.abs()
}

/// The point's distance from the cone `α` about `apex + t·axis`, both
/// nappes, in double-double.
fn cone_miss(p: Point3<f64>, apex: Point3<f64>, axis: Vec3<f64>, alpha: f64) -> f64 {
    let ad = dv(axis);
    let len = dot3(ad, ad).sqrt();
    let w = rel(p, apex);
    let h = dot3(w, ad).div(len);
    let rho = dot3(w, w).sub(h.mul(h)).sqrt();
    let (s, c) = Dd::sin_cos(alpha);
    let ha = Dd::of(h.0.abs()).add(Dd::of(h.1 * h.0.signum()));
    rho.mul(c).sub(ha.mul(s)).0.abs()
}

/// The cylinder radius `r` about `o + t·axis`'s miss, in double-double.
fn cyl_miss(p: Point3<f64>, o: Point3<f64>, axis: Vec3<f64>, r: f64) -> f64 {
    let ad = dv(axis);
    let len = dot3(ad, ad).sqrt();
    let w = rel(p, o);
    let h = dot3(w, ad).div(len);
    let rho = dot3(w, w).sub(h.mul(h)).sqrt();
    rho.sub(Dd::of(r)).0.abs()
}

/// A generic rigid motion (`ROT` set), else the identity: aligned
/// fixtures hold exact zeros a skew pose rounds.
fn rot() -> Affine3<f64> {
    if std::env::var("ROT").is_ok() {
        Affine3::rotation_about_axis(
            Point3::new(0.37, -0.21, 0.13),
            Vec3::new(0.3, 0.5, 0.8).normalize(),
            0.7,
        )
    } else {
        Affine3::identity()
    }
}

fn moved(body: Body<f64>) -> Body<f64> {
    if std::env::var("ROT").is_ok() {
        topo::transform_rigid(&body, &geom_core::test_support::rigid(&rot(), tol()), tol()).unwrap()
    } else {
        body
    }
}

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

const U: f64 = f64::EPSILON / 2.0;

/// The arcs of `e` whose coordinate `coord` lies strictly in `(lo, hi)`,
/// as `(t0, t1)` with `t1 > t0`, solved in closed form (the coordinate
/// is harmonic in `t`).
fn arcs(e: &Curve3<f64>, coord: impl Fn(Point3<f64>) -> f64, lo: f64, hi: f64) -> Vec<(f64, f64)> {
    let tau = core::f64::consts::TAU;
    let Curve3::Ellipse {
        center,
        axis,
        semi_u,
        semi_v,
        u_ref,
    } = *e
    else {
        unreachable!()
    };
    let v = axis.cross(u_ref);
    let c = coord(center);
    let zero = coord(Point3::new(0.0, 0.0, 0.0));
    let a = coord(Point3::new(0.0, 0.0, 0.0) + u_ref * semi_u) - zero;
    let b = coord(Point3::new(0.0, 0.0, 0.0) + v * semi_v) - zero;
    let r = a.hypot(b);
    let th = b.atan2(a);
    let mut roots = Vec::new();
    for level in [lo, hi] {
        let x = (level - c) / r;
        if x.abs() <= 1.0 {
            let d = x.acos();
            roots.push((th + d).rem_euclid(tau));
            roots.push((th - d).rem_euclid(tau));
        }
    }
    let inside = |t: f64| {
        let y = coord(e.eval(t));
        y > lo && y < hi
    };
    if roots.is_empty() {
        return if inside(0.0) {
            vec![(0.0, tau)]
        } else {
            vec![]
        };
    }
    roots.sort_by(f64::total_cmp);
    let n = roots.len();
    let mut out = Vec::new();
    for i in 0..n {
        let t0 = roots[i];
        let t1 = if i + 1 < n {
            roots[i + 1]
        } else {
            roots[0] + tau
        };
        if t1 - t0 > 0.0 && inside(0.5 * (t0 + t1)) {
            // a hair inside each end
            let pad = (t1 - t0) * 1e-6;
            out.push((t0 + pad, t1 - pad));
        }
    }
    out
}

/// `max |S(P(t)) − C(t)|` over a whole turn of the carrier.
fn whole_turn_miss(image: &Pcurve<f64>, e: &Curve3<f64>, quadric: &Surface<f64>) -> f64 {
    (0..=2000).fold(0.0, |m: f64, i| {
        let t = core::f64::consts::TAU * f64::from(i) / 2000.0;
        let uv = image.eval(t);
        m.max(quadric.eval(uv.x, uv.y).distance(e.eval(t)))
    })
}

struct Read {
    carrier_res: f64,
    map_res: f64,
    map_res_full: f64,
    cert: String,
}

fn read(
    e: &Curve3<f64>,
    arcs: &[(f64, f64)],
    quadric: &Surface<f64>,
    miss: impl Fn(Point3<f64>) -> f64,
) -> Read {
    let mut carrier_res = 0.0_f64;
    let mut map_res = 0.0_f64;
    let mut map_res_full = 0.0;
    let mut cert = String::new();
    let image = chart_pcurve(e, quadric, band());
    for &(t0, t1) in arcs {
        for i in 0..=400 {
            let t = t0 + (t1 - t0) * f64::from(i) / 400.0;
            let p = e.eval(t);
            carrier_res = carrier_res.max(miss(p));
            if let Ok(img) = &image {
                let uv = img.eval(t);
                map_res = map_res.max(quadric.eval(uv.x, uv.y).distance(p));
            }
        }
        match &image {
            Ok(img) => match PcurveCache::certify(img.clone(), t0, t1, e, quadric, band()) {
                Ok(c) => cert += &format!("ok({:.1e}) ", c.certificate().envelope),
                Err(err) => {
                    let s = format!("{err:?}");
                    cert += &format!("ERR[{}] ", s.chars().take(110).collect::<String>());
                }
            },
            Err(err) => {
                let s = format!("{err:?}");
                cert += &format!("NOIMG[{}] ", s.chars().take(80).collect::<String>());
            }
        }
    }
    // The same image with its vertex azimuth read off the major axis'
    // radial direction alone, where `chart_pcurve` reads it off the
    // cancelling `r0 + ra` (`work/pcert/focal-section-vertex-azimuth-…`).
    if let (
        Ok(Pcurve::FocalSection(fi)),
        &Surface::Cone {
            apex, axis, u_ref, ..
        },
        &Curve3::Ellipse {
            semi_u,
            u_ref: e_u,
            center,
            ..
        },
    ) = (&image, quadric, e)
    {
        let ra = e_u * semi_u - axis * (e_u * semi_u).dot(axis);
        let start = ra * (center - apex).dot(axis).signum();
        let u0 = start.dot(axis.cross(u_ref)).atan2(start.dot(u_ref));
        let alt = Pcurve::FocalSection(FocalImage { u0, ..*fi });
        cert += &format!(" ALTU0 mapfull {:.2e}", whole_turn_miss(&alt, e, quadric));
    }
    if let Ok(img) = &image {
        map_res_full = whole_turn_miss(img, e, quadric);
    }
    Read {
        carrier_res,
        map_res,
        map_res_full,
        cert,
    }
}

fn outcome(body: &AtRestBody<f64>, origin: Point3<f64>, n: Vec3<f64>) -> String {
    let plane = topo::test_support::split_plane(origin, n / n.norm(), tol());
    match split(body, &plane, tol()) {
        Ok(_) => "SPLITS".to_owned(),
        Err(e) => {
            let s = format!("{e:?}");
            let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();
            format!("REFUSES[{}]", s.chars().take(160).collect::<String>())
        }
    }
}

fn finished(body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol()).expect("a valid operand")
}

fn fracs() -> Vec<f64> {
    std::env::var("FRACS").map_or_else(
        |_| {
            vec![
                0.5,
                0.8,
                0.9,
                0.95,
                0.99,
                0.999,
                0.9999,
                0.99999,
                0.999_999,
                0.999_999_9,
                0.999_999_99,
            ]
        },
        |s| s.split(',').map(|x| x.parse().unwrap()).collect(),
    )
}

fn list(var: &str, default: &[f64]) -> Vec<f64> {
    std::env::var(var).map_or_else(
        |_| default.to_vec(),
        |s| s.split(',').map(|x| x.parse().unwrap()).collect(),
    )
}

fn cone_family(no_split: bool) {
    let eps = tol().eps();
    for alpha in list("ALPHAS", &[0.2, 0.5_f64.atan(), 1.0, 1.3, 1.55]) {
        for scale in list("SCALES", &[1e-2, 1.0, 1e2]) {
            let (h, r1) = (scale, 0.5 * scale);
            let r0 = r1 + h * alpha.tan();
            let body = if no_split {
                None
            } else {
                std::panic::catch_unwind(|| {
                    finished(moved(sweep::test_support::revolved_about_y(
                        vec![
                            (Point2::new(0.0, 0.0), 0.0),
                            (Point2::new(r0, 0.0), 0.0),
                            (Point2::new(r1, h), 0.0),
                            (Point2::new(0.0, h), 0.0),
                        ],
                        sweep::Revolution::Full,
                        tol(),
                    )))
                })
                .ok()
            };
            let m = rot();
            let apex = m.transform_point(Point3::new(0.0, h + r1 / alpha.tan(), 0.0));
            let axis = m.transform_vec(Vec3::new(0.0, -1.0, 0.0));
            let cone = Surface::Cone {
                apex,
                axis,
                half_angle: alpha,
                u_ref: m.transform_vec(Vec3::unit_x()),
            };
            let (o_frame, y_frame) = (
                m.transform_point(Point3::new(0.0, 0.0, 0.0)),
                m.transform_vec(Vec3::unit_y()),
            );
            // the face's own reach: its farthest chord
            let across = (2.0 * r0).max((h * h + (r0 + r1).powi(2)).sqrt());
            let reach = Reach::Measured {
                at: apex,
                lever: across,
            };
            let phi_p = core::f64::consts::FRAC_PI_2 - alpha;
            for f in fracs() {
                let phi = f * phi_p;
                let n = m.transform_vec(Vec3::new(phi.sin(), phi.cos(), 0.0));
                let q = m.transform_point(Point3::new(0.0, 0.5 * h, 0.0));
                let plane = Surface::Plane {
                    origin: q,
                    normal: n,
                    u_ref: m.transform_vec(Vec3::unit_z()),
                };
                let ep = alpha.tan() * phi.tan();
                let k = (1.0 - ep * ep).max(0.0).sqrt();
                let out = match &body {
                    _ if no_split => "-".to_owned(),
                    Some(b) => outcome(b, q, n),
                    // the frustum itself refuses to build at this ε
                    None => "NOBODY".to_owned(),
                };
                let tag = format!("cone a{alpha:.4} S{scale:e} f{f}");
                match plane_cone_section(&plane, &cone, &reach, band()) {
                    Ok(PlaneConeSection::TiltedEllipse(e)) => {
                        let Curve3::Ellipse { semi_u, semi_v, .. } = e else {
                            unreachable!()
                        };
                        let arcs = arcs(&e, |p| (p - o_frame).dot(y_frame), 0.0, h);
                        let r = read(&e, &arcs, &cone, |p| {
                            cone_miss(p, apex, axis, alpha).max(plane_miss(p, q, n))
                        });
                        println!(
                            "POSE {tag} eps{eps:e} | {out} | A {semi_u:.3e} B {semi_v:.3e} ext {across:.3e} k {k:.3e} \
                             | carrier {:.2e} map {:.2e} mapfull {:.2e} | uA/k2 {:.2e} | cert {}",
                            r.carrier_res,
                            r.map_res,
                            r.map_res_full,
                            U * semi_u / (k * k),
                            r.cert
                        );
                    }
                    Ok(other) => println!("POSE {tag} eps{eps:e} | {out} | arm {other:?}"),
                    Err(err) => {
                        let s = format!("{err:?}");
                        println!(
                            "POSE {tag} eps{eps:e} | {out} | ARM[{}] k {k:.3e}",
                            s.chars().take(120).collect::<String>()
                        );
                    }
                }
            }
        }
    }
}

fn cyl_family(no_split: bool) {
    let eps = tol().eps();
    for scale in list("SCALES", &[1e-2, 1.0, 1e2]) {
        let body = if no_split {
            None
        } else {
            Some(finished(moved(sweep::test_support::prism(
                vec![
                    (Point2::new(scale, 0.0), 1.0),
                    (Point2::new(-scale, 0.0), 1.0),
                ],
                scale,
                tol(),
            ))))
        };
        let m = rot();
        let axis = m.transform_vec(Vec3::unit_z());
        let o = m.transform_point(Point3::new(0.0, 0.0, 0.0));
        let cyl = Surface::Cylinder {
            origin: o,
            axis,
            radius: scale,
            u_ref: m.transform_vec(Vec3::unit_x()),
        };
        let across = (4.0 * scale * scale + scale * scale).sqrt();
        let reach = Reach::Measured {
            at: m.transform_point(Point3::new(0.0, 0.0, 0.5 * scale)),
            lever: across,
        };
        for f in fracs() {
            let phi = f * core::f64::consts::FRAC_PI_2;
            let n = m.transform_vec(Vec3::new(phi.sin(), 0.0, phi.cos()));
            let q = m.transform_point(Point3::new(0.0, 0.0, 0.5 * scale));
            let plane = Surface::Plane {
                origin: q,
                normal: n,
                u_ref: m.transform_vec(Vec3::unit_y()),
            };
            let out = body
                .as_ref()
                .map_or_else(|| "-".to_owned(), |b| outcome(b, q, n));
            let tag = format!("cyl S{scale:e} f{f}");
            let k = phi.cos();
            match plane_cylinder_section(&plane, &cyl, &reach, band()) {
                Ok(PlaneCylinderSection::TiltedEllipse(e)) => {
                    let Curve3::Ellipse { semi_u, semi_v, .. } = e else {
                        unreachable!()
                    };
                    let arcs = arcs(&e, |p| (p - o).dot(axis), 0.0, scale);
                    let r = read(&e, &arcs, &cyl, |p| {
                        cyl_miss(p, o, axis, scale).max(plane_miss(p, q, n))
                    });
                    println!(
                        "POSE {tag} eps{eps:e} | {out} | A {semi_u:.3e} B {semi_v:.3e} ext {across:.3e} k {k:.3e} \
                         | carrier {:.2e} map {:.2e} mapfull {:.2e} | uA {:.2e} | cert {}",
                        r.carrier_res,
                        r.map_res,
                        r.map_res_full,
                        U * semi_u,
                        r.cert
                    );
                }
                Ok(other) => println!("POSE {tag} eps{eps:e} | {out} | arm {other:?}"),
                Err(err) => {
                    let s = format!("{err:?}");
                    println!(
                        "POSE {tag} eps{eps:e} | {out} | ARM[{}]",
                        s.chars().take(120).collect::<String>()
                    );
                }
            }
        }
    }
}

fn main() {
    let no_split = std::env::var("NO_SPLIT").is_ok();
    match std::env::var("FAMILY").as_deref() {
        Ok("cone") => cone_family(no_split),
        Ok("cyl") => cyl_family(no_split),
        _ => {
            cone_family(no_split);
            cyl_family(no_split);
        }
    }
}
