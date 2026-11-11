//! Review probe for PR 6827 (compiles on main and on the PR head): limb 2
//! on a seeded sweep of nets near analytic charts, at `f64` and
//! `Interval`, band zeros 1e-9 and 1e-12. Each case prints one verdict
//! line (`REV6827 <case> <verdict>`) so main and head can be diffed, and
//! asserts no reading clears a net whose sampled distance from its chart
//! exceeds the band's zero.

use super::*;
use geom_core::Interval;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)]
        let x = (self.0 >> 11) as f64 / (1u64 << 53) as f64;
        x
    }
    fn sym(&mut self) -> f64 {
        2.0 * self.next() - 1.0
    }
}

fn spline(controls: &[Point3<f64>], weights: &[f64]) -> Curve3<f64> {
    let spans = controls.len() - 3;
    let inner = (1..spans).map(|i| {
        #[allow(clippy::cast_precision_loss)]
        let t = i as f64 / spans as f64;
        t
    });
    let knots: Vec<f64> = [0.0; 4].into_iter().chain(inner).chain([1.0; 4]).collect();
    let knots = KnotVector::clamped(knots, 3).unwrap();
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(knots, controls.to_vec(), weights.to_vec()).unwrap(),
    ))
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Chart {
    Plane,
    Cylinder,
    Sphere,
    Cone,
    Torus,
}

const ALPHA: f64 = 0.4;

pub(super) fn surface(c: Chart) -> Surface<f64> {
    let o = Point3::new(0.0, 0.0, 0.0);
    let z = Vec3::new(0.0, 0.0, 1.0);
    let x = Vec3::new(1.0, 0.0, 0.0);
    match c {
        Chart::Plane => Surface::Plane {
            origin: o,
            normal: z,
            u_ref: x,
        },
        Chart::Cylinder => Surface::Cylinder {
            origin: o,
            axis: z,
            radius: 1.0,
            u_ref: x,
        },
        Chart::Sphere => Surface::Sphere {
            center: o,
            axis: z,
            radius: 1.0,
            u_ref: x,
        },
        Chart::Cone => Surface::Cone {
            apex: o,
            axis: z,
            half_angle: ALPHA,
            u_ref: x,
        },
        Chart::Torus => Surface::Torus {
            center: o,
            axis: z,
            major_radius: 2.0,
            minor_radius: 0.5,
            u_ref: x,
        },
    }
}

/// Distance from `p` to the chart (the nappe or sheet the rows read).
pub(super) fn dist(c: Chart, p: Point3<f64>) -> f64 {
    let rho = p.x.hypot(p.y);
    match c {
        Chart::Plane => p.z.abs(),
        Chart::Cylinder => (rho - 1.0).abs(),
        Chart::Sphere => ((rho.hypot(p.z)) - 1.0).abs(),
        Chart::Cone => {
            // The double cone's distance (at most either nappe's).
            let (s, co) = ALPHA.sin_cos();
            (rho * co - p.z.abs() * s).abs()
        }
        Chart::Torus => ((rho - 2.0).hypot(p.z) - 0.5).abs(),
    }
}

/// A point on the chart at parameters `(a, b)` in `[0, 1]²`, pushed off
/// it along the normal by `off`; `pole` puts the sphere's and cone's
/// points by the axis.
fn on(c: Chart, a: f64, b: f64, off: f64, pole: f64) -> Point3<f64> {
    let th = a * core::f64::consts::TAU;
    let (s, co) = th.sin_cos();
    match c {
        Chart::Plane => Point3::new(3.0 * a - 1.5, 3.0 * b - 1.5, off),
        Chart::Cylinder => {
            let r = 1.0 + off;
            Point3::new(r * co, r * s, 2.0 * b)
        }
        Chart::Sphere => {
            let phi = pole + b * 0.4;
            let r = 1.0 + off;
            Point3::new(r * phi.sin() * co, r * phi.sin() * s, r * phi.cos())
        }
        Chart::Cone => {
            let (sa, ca) = ALPHA.sin_cos();
            let h = pole + 1.5 * b;
            // on the generator: rho = h tan a, z = h; normal (ca, -sa).
            let rho = h * sa / ca + off * ca;
            let z = h - off * sa;
            Point3::new(rho * co, rho * s, z)
        }
        Chart::Torus => {
            let v = b * core::f64::consts::TAU;
            let r = 0.5 + off;
            let rho = 2.0 + r * v.cos();
            Point3::new(rho * co, rho * s, r * v.sin())
        }
    }
}

pub(super) fn sampled_max(c: Chart, carrier: &Curve3<f64>) -> f64 {
    (0..=4000)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let t = f64::from(i) / 4000.0;
            dist(c, carrier.eval(t))
        })
        .fold(0.0, f64::max)
}

/// The sweep's cases: chart, carrier, label.
pub(super) fn cases() -> Vec<(Chart, Curve3<f64>, String)> {
    let mut rng = Lcg(0x6827);
    let mut out = Vec::new();
    for chart in [
        Chart::Plane,
        Chart::Cylinder,
        Chart::Sphere,
        Chart::Cone,
        Chart::Torus,
    ] {
        for k in 0..36 {
            let spans = [1usize, 3, 10][k % 3];
            // 0: far, 1: by the pole/apex, 2: straddling the axis (sphere),
            // 3: a long sweep.
            let shape = (k / 3) % 4;
            let rational = (k / 12) % 3; // 0 poly, 1 mild, 2 wild
            let (pole, a_span, b_span) = match shape {
                0 => (0.6, 0.08, 0.3),
                1 => (1e-4, 0.08, 0.02),
                2 => (-0.02, 0.5, 0.05),
                _ => (0.4, 0.6, 0.9),
            };
            let n = spans + 3;
            let a0 = rng.next();
            let b0 = rng.next() * 0.5;
            // The off-chart amplitude, a multiple of a 1e-9 band zero.
            let amp = [0.0, 1e-9, 4e-9, 1e-6, 1e-12][rng.next().mul_add(5.0, 0.0) as usize % 5];
            let controls: Vec<Point3<f64>> = (0..n)
                .map(|i| {
                    #[allow(clippy::cast_precision_loss)]
                    let f = i as f64 / (n - 1) as f64;
                    on(
                        chart,
                        a0 + a_span * f,
                        b0 + b_span * f,
                        amp * rng.sym(),
                        pole,
                    )
                })
                .collect();
            let weights: Vec<f64> = (0..n)
                .map(|_| match rational {
                    0 => 1.0,
                    1 => 0.5 + rng.next(),
                    _ => 10f64.powf(2.0 * rng.sym()),
                })
                .collect();
            out.push((
                chart,
                spline(&controls, &weights),
                format!("{chart:?}#{k}:spans{spans}:shape{shape}:rat{rational}:amp{amp:e}"),
            ));
        }
    }
    out
}

fn verdict(
    read: Result<projected::OffsetRead<impl geom_core::CertifiedEnclosure>, PcurveCertifyError>,
    band: Band,
) -> (String, bool) {
    match read {
        Err(e) => (format!("E:{e:?}").chars().take(60).collect(), false),
        Ok(r) => match r.offset {
            projected::Offset::Bound(_) => {
                let c = crate::ssi::certify::clears(r.upper, band);
                (format!("B:clears={c}"), c)
            }
            projected::Offset::Miss { .. } => ("M".into(), false),
        },
    }
}

#[test]
fn rev6827_sweep() {
    let mut lines = Vec::new();
    let mut bad = Vec::new();
    for (chart, carrier, label) in cases() {
        let truth = sampled_max(chart, &carrier);
        let s = surface(chart);
        for zero in [1e-9, 1e-12] {
            let band = Band::new(zero, 10.0 * zero).unwrap();
            let f = limb2_offset_sup(&carrier, (0.0, 1.0), &s, band);
            let (vf, cf) = verdict(f, band);
            let lift = Interval::from_f64;
            let i = limb2_offset_sup(
                &carrier.map_scalar(lift),
                (lift(0.0), lift(1.0)),
                &s.map_scalar(lift),
                band,
            );
            let (vi, ci) = verdict(i, band);
            for (what, c) in [("f64", cf), ("iv", ci)] {
                if c && truth > zero {
                    bad.push(format!("{label} z{zero:e} {what}: cleared, sampled {truth:e}"));
                }
            }
            lines.push(format!(
                "REV6827 {label} z{zero:e} truth{} f64:{vf} iv:{vi}",
                if truth > zero { ">" } else { "<=" }
            ));
        }
    }
    let path = std::env::var("REV6827_OUT").unwrap_or_else(|_| "/dev/null".into());
    std::fs::write(path, lines.join("\n")).unwrap();
    assert!(bad.is_empty(), "{bad:#?}");
}

/// An exact rational-quadratic arc, `spans` equal C0 spans: centre `c`,
/// in-plane orthonormal `u`, `v`, radius `r`, angles `a..b`; each control
/// pushed by `amp` along a seeded direction.
fn arc(
    c: Point3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    r: f64,
    (a, b): (f64, f64),
    spans: usize,
    amp: f64,
    lam: f64,
    rng: &mut Lcg,
) -> Curve3<f64> {
    let mut pts = Vec::new();
    let mut ws = Vec::new();
    let mut knots = vec![0.0; 3];
    for k in 0..spans {
        #[allow(clippy::cast_precision_loss)]
        let (t0, t1) = (
            a + (b - a) * k as f64 / spans as f64,
            a + (b - a) * (k + 1) as f64 / spans as f64,
        );
        let h = 0.5 * (t1 - t0);
        let m = 0.5 * (t0 + t1);
        let at = |t: f64, s: f64| c + (u * t.cos() + v * t.sin()) * (r * s);
        // Same curve, reparametrised: span k's weights (s, s·l·w, s·l²)
        // with l = lam^(±1) alternating, so the joints agree.
        let l = if k % 2 == 0 { lam } else { 1.0 / lam };
        let s0 = if k % 2 == 0 { 1.0 } else { lam * lam };
        if k == 0 {
            pts.push(at(t0, 1.0));
            ws.push(s0);
        }
        pts.push(at(m, 1.0 / h.cos()));
        ws.push(s0 * l * h.cos());
        pts.push(at(t1, 1.0));
        ws.push(s0 * l * l);
        if k + 1 < spans {
            #[allow(clippy::cast_precision_loss)]
            let kn = (k + 1) as f64 / spans as f64;
            knots.extend([kn, kn]);
        }
    }
    knots.extend([1.0; 3]);
    let _ = lam;
    let pts: Vec<Point3<f64>> = pts
        .into_iter()
        .map(|p| p + Vec3::new(rng.sym(), rng.sym(), rng.sym()) * amp)
        .collect();
    let knots = KnotVector::clamped(knots, 2).unwrap();
    Curve3::Nurbs(Arc::new(NurbsCurve3::new(knots, pts, ws).unwrap()))
}

pub(super) fn arc_cases() -> Vec<(Chart, Curve3<f64>, String)> {
    let mut rng = Lcg(0x6827_0002);
    let o = Point3::new(0.0, 0.0, 0.0);
    let (x, y, z) = (
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    let (sa, ca) = ALPHA.sin_cos();
    let mut out = Vec::new();
    // (chart, centre, u, v, radius, label)
    let mut arcs: Vec<(Chart, Point3<f64>, Vec3<f64>, Vec3<f64>, f64, String)> = vec![
        (Chart::Plane, o, x, y, 0.7, "plane circle".into()),
        (Chart::Cylinder, Point3::new(0.0, 0.0, 0.3), x, y, 1.0, "cyl latitude".into()),
        (Chart::Torus, Point3::new(0.0, 0.0, 0.5), x, y, 2.0, "torus top".into()),
        (Chart::Torus, Point3::new(2.0, 0.0, 0.0), x, z, 0.5, "torus tube".into()),
        (Chart::Sphere, o, x, z, 1.0, "sphere meridian".into()),
        (Chart::Sphere, o, x, y, 1.0, "sphere equator".into()),
    ];
    for phi in [1e-1, 1e-2, 1e-3, 1e-5] {
        arcs.push((
            Chart::Sphere,
            Point3::new(0.0, 0.0, phi.cos()),
            x,
            y,
            phi.sin(),
            format!("sphere latitude phi{phi:e}"),
        ));
    }
    for h in [1.0, 1e-2, 1e-4] {
        arcs.push((
            Chart::Cone,
            Point3::new(0.0, 0.0, h),
            x,
            y,
            h * sa / ca,
            format!("cone latitude h{h:e}"),
        ));
    }
    for (chart, c, u, v, r, what) in arcs {
        for (a, b) in [(0.1, 0.15), (0.3, 0.8), (0.2, 1.7708), (1.4, 1.4 + core::f64::consts::PI)] {
            for spans in [2usize, 4, 16] {
                for amp in [0.0, 1e-13, 1e-10, 1e-9, 3e-9] {
                    for lam in [1.0, 8.0] {
                        out.push((
                            chart,
                            arc(c, u, v, r, (a, b), spans, amp, lam, &mut rng),
                            format!("{what}:{a}-{b}:spans{spans}:amp{amp:e}:lam{lam}"),
                        ));
                    }
                }
            }
        }
    }
    out
}

#[test]
fn rev6827_arc_sweep() {
    let mut lines = Vec::new();
    let mut bad = Vec::new();
    for (chart, carrier, label) in arc_cases() {
        let truth = sampled_max(chart, &carrier);
        let s = surface(chart);
        for zero in [1e-9, 1e-12] {
            let band = Band::new(zero, 10.0 * zero).unwrap();
            let f = limb2_offset_sup(&carrier, (0.0, 1.0), &s, band);
            let (vf, cf) = verdict(f, band);
            let lift = Interval::from_f64;
            let i = limb2_offset_sup(
                &carrier.map_scalar(lift),
                (lift(0.0), lift(1.0)),
                &s.map_scalar(lift),
                band,
            );
            let (vi, ci) = verdict(i, band);
            for (what, c) in [("f64", cf), ("iv", ci)] {
                if c && truth > zero {
                    bad.push(format!("{label} z{zero:e} {what}: cleared, sampled {truth:e}"));
                }
            }
            lines.push(format!(
                "REV6827 {label} z{zero:e} truth{} f64:{vf} iv:{vi}",
                if truth > zero { ">" } else { "<=" }
            ));
        }
    }
    let path = std::env::var("REV6827_ARC_OUT").unwrap_or_else(|_| "/dev/null".into());
    std::fs::write(path, lines.join("\n")).unwrap();
    assert!(bad.is_empty(), "{bad:#?}");
}
