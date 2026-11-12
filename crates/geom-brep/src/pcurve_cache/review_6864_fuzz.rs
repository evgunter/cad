//! Review probe (PR 6864): an adversarial search for a `Parts::ByTurn`
//! envelope under the sampled displacement, and for a false Ok from
//! check 4 (a band whose zero lies under the sampled displacement).
//! Long spans (up to half a turn a span), spans by a sphere's pole and
//! a cone's apex, quarter turns round a cylinder, on a torus.

use super::*;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        #[allow(clippy::cast_precision_loss)]
        let x = (self.0 >> 11) as f64 / (1u64 << 53) as f64;
        x
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
}

fn spline(controls: &[Point3<f64>], degree: usize) -> Curve3<f64> {
    let spans = controls.len() - degree;
    let inner = (1..spans).map(|i| {
        #[allow(clippy::cast_precision_loss)]
        let t = i as f64 / spans as f64;
        t
    });
    let knots: Vec<f64> = std::iter::repeat_n(0.0, degree + 1)
        .chain(inner)
        .chain(std::iter::repeat_n(1.0, degree + 1))
        .collect();
    let knots = KnotVector::clamped(knots, degree).unwrap();
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(knots, controls.to_vec(), vec![1.0; controls.len()]).unwrap(),
    ))
}

#[derive(Clone, Copy, Debug)]
enum Shape {
    CylinderLong,
    SpherePole,
    ConeApex,
    Torus,
}

/// A surface and a carrier: controls at `(u, v)` on the chart, each
/// pushed off along the chart's normal by up to `push`.
fn draw(shape: Shape, s: &mut Rng) -> (Surface<f64>, Curve3<f64>, f64) {
    let degree = if s.next() < 0.7 { 3 } else { 5 };
    let spans = 1 + (s.next() * 4.0) as usize;
    let n = spans + degree;
    let push = 10f64.powf(s.range(-7.0, -2.5));
    let z = Vec3::new(0.0, 0.0, 1.0);
    let x = Vec3::new(1.0, 0.0, 0.0);
    let o = Point3::new(0.0, 0.0, 0.0);
    let mut jitter = |s: &mut Rng| s.range(-1.0, 1.0) * push;
    match shape {
        Shape::CylinderLong => {
            // up to half a turn a span
            let turn = s.range(0.2, 3.1) * spans as f64 / 2.0_f64.max(spans as f64 / 2.0);
            let turn = turn.min(1.4 * spans as f64);
            let z0 = s.range(-1.0, 1.0);
            let dz = s.range(-1.0, 1.0);
            let c: Vec<Point3<f64>> = (0..n)
                .map(|i| {
                    let f = i as f64 / (n - 1) as f64;
                    let a = turn * f;
                    let r = 1.0 + jitter(s);
                    Point3::new(r * a.cos(), r * a.sin(), z0 + dz * f)
                })
                .collect();
            let surf = Surface::Cylinder {
                origin: o,
                axis: z,
                radius: 1.0,
                u_ref: x,
            };
            (surf, spline(&c, degree), push)
        }
        Shape::SpherePole => {
            // polar angle small: the carrier runs by the pole
            let polar0 = s.range(0.01, 0.4);
            let sweep = s.range(0.2, 2.5);
            let c: Vec<Point3<f64>> = (0..n)
                .map(|i| {
                    let f = i as f64 / (n - 1) as f64;
                    let a = sweep * f;
                    let p = polar0 * (1.0 + 0.5 * (f * 3.0).sin());
                    let r = 1.0 + jitter(s);
                    Point3::new(r * p.sin() * a.cos(), r * p.sin() * a.sin(), r * p.cos())
                })
                .collect();
            let surf = Surface::Sphere {
                center: o,
                axis: z,
                radius: 1.0,
                u_ref: x,
            };
            (surf, spline(&c, degree), push)
        }
        Shape::ConeApex => {
            let half = core::f64::consts::FRAC_PI_6;
            let d0 = s.range(0.02, 0.4);
            let sweep = s.range(0.2, 2.5);
            let c: Vec<Point3<f64>> = (0..n)
                .map(|i| {
                    let f = i as f64 / (n - 1) as f64;
                    let a = sweep * f;
                    let d = d0 * (1.0 + f);
                    // point on the cone at slant distance d, azimuth a, pushed along the normal
                    let (rho, h) = (d * half.sin(), d * half.cos());
                    let (nr, nz) = (half.cos(), -half.sin());
                    let e = jitter(s);
                    Point3::new(
                        (rho + e * nr) * a.cos(),
                        (rho + e * nr) * a.sin(),
                        h + e * nz,
                    )
                })
                .collect();
            let surf = Surface::Cone {
                apex: o,
                axis: z,
                half_angle: half,
                u_ref: x,
            };
            (surf, spline(&c, degree), push)
        }
        Shape::Torus => {
            let (big, small) = (2.0, 0.5);
            let u0 = s.range(0.0, 1.0);
            let du = s.range(0.2, 2.0);
            let v0 = s.range(-3.0, 3.0);
            let dv = s.range(-2.0, 2.0);
            let c: Vec<Point3<f64>> = (0..n)
                .map(|i| {
                    let f = i as f64 / (n - 1) as f64;
                    let (u, v) = (u0 + du * f, v0 + dv * f);
                    let r = small + jitter(s);
                    let rho = big + r * v.cos();
                    Point3::new(rho * u.cos(), rho * u.sin(), r * v.sin())
                })
                .collect();
            let surf = Surface::Torus {
                center: o,
                axis: z,
                major_radius: big,
                minor_radius: small,
                u_ref: x,
            };
            (surf, spline(&c, degree), push)
        }
    }
}

fn envelope_in(
    image: &ProjectedImage<f64>,
    c: &Curve3<f64>,
    s: &Surface<f64>,
    band: Band,
    parts: Parts,
) -> Result<f64, PcurveCertifyError> {
    let (t0, t1) = (0.0, 1.0);
    let (mut terms, name) = projected::projected_envelope(
        image,
        t0,
        t1,
        &image.chart_box(t0, t1),
        c,
        s,
        band,
        Some(crate::FittedLane::certified()),
        parts,
    )?;
    let pcurve = Pcurve::Projected(Box::new(image.clone()));
    let (fu, fv) = placement_terms(&pcurve, name, (t0, t1), c, s, band)?;
    terms.add(EnvelopeTerm::FidelityU, fu);
    terms.add(EnvelopeTerm::FidelityV, fv);
    Ok(terms.total())
}

#[test]
fn review_6864_by_turn_dominance_and_no_false_ok() {
    let mut s = Rng(0x6864_u64.wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let rounds: usize = std::env::var("REVIEW_ROUNDS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(150);
    let band = Band::new(1e-2, 1e-1).unwrap();
    let mut worst = [(0usize, 0usize, f64::INFINITY); 4];
    let mut bad = Vec::new();
    for _ in 0..rounds {
        for (k, shape) in [
            Shape::CylinderLong,
            Shape::SpherePole,
            Shape::ConeApex,
            Shape::Torus,
        ]
        .into_iter()
        .enumerate()
        {
            let (surf, c, _push) = draw(shape, &mut s);
            let probe = Band::new(1e-3, 1e-2).unwrap();
            let Ok(image) = projected::project(&c, None, &surf, probe) else {
                continue;
            };
            let pc = Pcurve::Projected(Box::new(image.clone()));
            let sup = super::harmonic_sup_rows::sampled_residual(&pc, (0.0, 1.0), &c, &surf, 4096);
            let zero = (3.0 * sup).max(1e-9);
            let wide = Band::new(zero, 10.0 * zero).unwrap();
            let Ok(image) = projected::project(&c, None, &surf, wide) else {
                continue;
            };
            let pc = Pcurve::Projected(Box::new(image.clone()));
            let sup = super::harmonic_sup_rows::sampled_residual(&pc, (0.0, 1.0), &c, &surf, 4096);
            let by_turn = envelope_in(&image, &c, &surf, wide, Parts::ByTurn);
            let eight = envelope_in(
                &image,
                &c,
                &surf,
                wide,
                Parts::Each(projected::INCIDENCE_CUTS),
            );
            worst[k].0 += 1;
            if let Err(e) = &by_turn {
                let k = format!("{e:?}");
                println!("REFUSED {shape:?} {}", &k[..k.len().min(60)]);
            }
            if let Ok(env) = by_turn {
                worst[k].1 += 1;
                let ratio = env / sup;
                worst[k].2 = worst[k].2.min(ratio);
                if env < sup * (1.0 - 1e-9) - 1e-15 {
                    bad.push(format!("{shape:?}: by-turn {env:e} < sampled {sup:e}"));
                }
                if let Ok(e8) = eight {
                    println!("RATIO {shape:?} {:.3}", env / e8);
                    if env < e8 * (1.0 - 1e-9) {
                        println!("NOTE {shape:?}: by-turn {env:e} under eight {e8:e}");
                    }
                }
            }
            // A false Ok: check 4 at Interval against a band whose zero is
            // just under the sampled displacement.
            if sup > 1e-12 {
                let at = geom_core::Interval::from_f64;
                let tight = Band::new(0.97 * sup, 100.0 * sup).unwrap();
                let (ci, si) = (c.map_scalar(at), surf.map_scalar(at));
                if let Ok(img) = projected::project(&ci, None, &si, tight) {
                    let got = projected::run_projected_checks(
                        &Pcurve::Projected(Box::new(img)),
                        at(0.0),
                        at(1.0),
                        &ci,
                        &si,
                        tight,
                        Some(crate::FittedLane::certified()),
                    );
                    println!("FALSEOK-TRIED {shape:?} ok={}", got.is_ok());
                    if got.is_ok() {
                        bad.push(format!(
                            "{shape:?}: FALSE OK at zero {:e} < sampled {sup:e}",
                            0.97 * sup
                        ));
                    }
                }
            }
        }
    }
    for (k, w) in worst.iter().enumerate() {
        println!(
            "SHAPE {k}: drawn {} by-turn read {} min env/sampled {:.4}",
            w.0, w.1, w.2
        );
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// The PR's `bulge` fixture, verbatim (cubic, one span).
fn bulge(delta: f64) -> Curve3<f64> {
    let arc = core::f64::consts::TAU / 10.0;
    let k = 4.0 / 3.0 * (arc / 4.0).tan();
    let on = |a: f64| Point3::new(a.cos(), a.sin(), 0.0);
    let handle = |a: f64, sense: f64, push: f64| {
        let r = (1.0 + k * k).sqrt() + push;
        let turn = a + sense * k.atan();
        Point3::new(r * turn.cos(), r * turn.sin(), 0.0)
    };
    spline(
        &[
            on(0.0),
            handle(0.0, 1.0, delta),
            handle(arc, -1.0, 0.0),
            on(arc),
        ],
        3,
    )
}

/// Check 4's whole k_stats trail at `Interval`: whether it certified,
/// its verdicts `(predicate, sign)` and its escalations' predicates.
#[allow(clippy::type_complexity)]
fn trail(
    c: &Curve3<f64>,
    s: &Surface<f64>,
    band: Band,
) -> (bool, Vec<(&'static str, Sign)>, Vec<&'static str>) {
    let at = geom_core::Interval::from_f64;
    let (ci, si) = (c.map_scalar(at), s.map_scalar(at));
    let image = projected::project(&ci, None, &si, band).unwrap();
    let (read, rec) = geom_core::k_stats::detached(|| {
        projected::run_projected_checks(
            &Pcurve::Projected(Box::new(image.clone())),
            at(0.0),
            at(1.0),
            &ci,
            &si,
            band,
            Some(crate::FittedLane::certified()),
        )
    });
    let r = rec.recorded();
    (
        read.is_ok(),
        r.verdicts.iter().map(|v| (v.predicate, v.sign)).collect(),
        r.escalations.iter().map(|e| e.predicate()).collect(),
    )
}

/// **Check 4's trail is the deciding reading's alone.** The bulging
/// span certifies at its first reading against a wide band, and at
/// eight parts a span against a band whose zero lies between the two
/// readings, where the first reading escalates in band: both trails are
/// one passing reading's, verdict for verdict, with no escalation. Red
/// where a dropped first reading's recording is spliced (HM1: the
/// escalation reaches the trail) or a passing one's is dropped (HM2:
/// the trail loses the check's verdicts).
#[test]
fn review_6864_check_4s_trail_holds_the_deciding_readings_verdict_alone() {
    let (c, s) = (
        bulge(1e-4),
        Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
    );
    let wide = Band::new(1e-3, 1e-2).unwrap();
    let at = |parts| {
        let image = projected::project(&c, None, &s, wide).unwrap();
        envelope_in(&image, &c, &s, wide, parts).unwrap()
    };
    let (first, eight) = (
        at(Parts::ByTurn),
        at(Parts::Each(projected::INCIDENCE_CUTS)),
    );
    assert!(first > 1.05 * eight, "{first:e} vs {eight:e}");
    let passed = trail(&c, &s, wide);
    assert!(passed.0, "passes at the first reading");
    assert!(passed.2.is_empty(), "no escalation: {:?}", passed.2);
    assert_eq!(
        passed.1.iter().filter(|v| v.0 == "pcurve_envelope").count(),
        1,
        "the passing first reading's verdict is on the trail: {:?}",
        passed.1
    );
    let zero = (first * eight).sqrt();
    let again = trail(&c, &s, Band::new(zero, 10.0 * zero).unwrap());
    assert!(again.0, "passes at eight parts a span");
    assert!(
        again.2.is_empty(),
        "the dropped first reading's escalation: {:?}",
        again.2
    );
    assert_eq!(again.1, passed.1, "one passing reading's trail");
}
