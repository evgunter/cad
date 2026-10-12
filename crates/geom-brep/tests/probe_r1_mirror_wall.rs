#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
//! R1 probe: the mirror-torus wall image is exact for random frames.
use geom::{Curve3, Surface};
use geom_brep::{EnvelopeStatement, Pcurve, PcurveCache, SpiricImage};
use geom_core::{Band, Interval, Point3, Real, Tol, Vec3};

struct Lcg(u64);
impl Lcg {
    fn f(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(2.0 * self.f() - 1.0, 2.0 * self.f() - 1.0, 2.0 * self.f() - 1.0);
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v * (1.0 / n);
            }
        }
    }
    fn perp(&mut self, a: Vec3<f64>) -> Vec3<f64> {
        let v = self.unit();
        let p = v - a * v.dot(a);
        p * (1.0 / p.norm())
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

#[test]
fn probe_mirror_wall_exact_random_frames() {
    let mut g = Lcg(0x5eed);
    let mut stats = [0usize; 6]; // configs, mint-ok, cert-ok f64, cert-ok iv, mint-err, cert-err
    let mut worst = 0.0f64;
    for k in 0..300 {
        let a = g.unit();
        let n = g.perp(a);
        let big = 0.05 + 2.0 * g.f();
        let r = big * (0.2 + 0.6 * g.f());
        let mut d = (big - r) * (0.02 + 0.95 * g.f());
        if k % 2 == 1 {
            d = -d;
        }
        let c = Point3::new(g.f() - 0.5, g.f() - 0.5, g.f() - 0.5);
        let carrier = Curve3::spiric(c, a, n, big, r, d, band()).unwrap();
        for (mirror, flip) in [(true, 1.0), (true, -1.0), (false, 1.0), (false, -1.0)] {
            stats[0] += 1;
            let axis = a * flip;
            let u_ref = g.perp(axis);
            let center = if mirror { (c + n * d) + n * d } else { c };
            let chart = Surface::Torus { center, axis, major_radius: big, minor_radius: r, u_ref };
            let p = match geom_brep::chart_pcurve(&carrier, &chart, band()) {
                Ok(p) => p,
                Err(e) => {
                    stats[4] += 1;
                    eprintln!("k{k} mirror={mirror} flip={flip}: mint err {e:?}");
                    continue;
                }
            };
            stats[1] += 1;
            let Pcurve::Spiric { image: SpiricImage::Wall { u_sense, v_sense, .. }, .. } = p else {
                panic!("not a wall")
            };
            assert_eq!(u_sense * v_sense, if mirror { -1.0 } else { 1.0 }, "k{k}");
            assert_eq!(v_sense, flip, "k{k}");
            for i in 0..257 {
                let t = core::f64::consts::TAU * f64::from(i) / 256.0 - 3.0;
                let uv = p.eval(t);
                let s = chart.eval(uv.x, uv.y);
                let e = (s - carrier.eval(t)).norm() / big;
                worst = worst.max(e);
                assert!(e < 1e-13, "k{k} mirror={mirror} flip={flip} t={t}: {e:e}");
            }
            // certify over a few spans covering every quadrant
            for (t0, t1) in [(-3.1, -1.0), (-1.2, 1.2), (1.0, 3.1), (2.5, 4.5)] {
                match PcurveCache::certify(p.clone(), t0, t1, &carrier, &chart, band()) {
                    Ok(cc) => {
                        stats[2] += 1;
                        assert_eq!(cc.certificate().statement, EnvelopeStatement::SpiricIdentity);
                        if cc.certificate().envelope != 0.0 { stats[4] += 1000; if stats[4] < 4000 { eprintln!("NONZERO env k{k} mirror={mirror} {:e}", cc.certificate().envelope); } }
                    }
                    Err(e) => {
                        stats[5] += 1;
                        eprintln!("k{k} m={mirror} f={flip} [{t0},{t1}] f64 cert err {e:?}");
                    }
                }
                let iv = <Interval as Real>::from_f64;
                let pi = geom_brep::chart_pcurve(&carrier.map_scalar(iv), &chart.map_scalar(iv), band());
                match pi.and_then(|pi| {
                    PcurveCache::certify(pi, iv(t0), iv(t1), &carrier.map_scalar(iv), &chart.map_scalar(iv), band())
                }) {
                    Ok(_) => stats[3] += 1,
                    Err(e) => {
                        stats[5] += 1;
                        eprintln!("k{k} m={mirror} f={flip} [{t0},{t1}] iv err {e:?}");
                    }
                }
            }
        }
    }
    eprintln!("stats {stats:?} worst {worst:e}");
}

/// d → 0: own and mirror centres coincide within the band.
#[test]
fn probe_tiny_offset() {
    let eps = Tol::witness().get().eps;
    for scale in [2.2, 2.5, 3.0, 3.5, 4.0, 5.0, 6.0, 8.0] {
        let d = eps * scale;
        let (big, r) = (0.09375, 0.0703125);
        let Ok(carrier) = Curve3::spiric(Point3::origin(), Vec3::unit_y(), Vec3::unit_x(), big, r, d, band()) else {
            eprintln!("scale {scale}: spiric refuses");
            continue;
        };
        for flip in [1.0, -1.0] {
            let chart = Surface::Torus {
                center: Point3::new(2.0 * d, 0.0, 0.0),
                axis: Vec3::unit_y() * flip,
                major_radius: big,
                minor_radius: r,
                u_ref: Vec3::unit_z(),
            };
            let res = geom_brep::chart_pcurve(&carrier, &chart, band())
                .and_then(|p| PcurveCache::certify(p, -1.0, 1.0, &carrier, &chart, band()));
            eprintln!("scale {scale} flip {flip}: {:?}", res.map(|c| (c.pcurve().clone(), c.certificate().envelope, c.certificate().max_residual)));
        }
    }
}
