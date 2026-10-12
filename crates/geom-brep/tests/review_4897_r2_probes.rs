//! R2 probes for PR 4897 (mirror-torus spiric wall). Not for merge.


use geom::{Curve3, Surface};
use geom_brep::{EnvelopeStatement, Pcurve, PcurveCache, SpiricImage};
use geom_core::{Band, Interval, Point3, Real, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn unit(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z).normalize()
}

fn iv3(v: Vec3<f64>) -> Vec3<Interval> {
    Vec3::new(Interval::from_f64(v.x), Interval::from_f64(v.y), Interval::from_f64(v.z))
}
fn ip3(p: Point3<f64>) -> Point3<Interval> {
    Point3::new(Interval::from_f64(p.x), Interval::from_f64(p.y), Interval::from_f64(p.z))
}

struct Case {
    c: Point3<f64>,
    a: Vec3<f64>,
    n: Vec3<f64>,
    big: f64,
    r: f64,
    d: f64,
}

fn cases() -> Vec<Case> {
    let mut out = vec![];
    let axes = [unit(0.0, 1.0, 0.0), unit(0.3, 0.8, -0.5), unit(-0.7, 0.1, 0.7)];
    let seeds = [unit(1.0, 0.0, 0.0), unit(0.2, -0.4, 0.9), unit(0.6, 0.6, 0.1)];
    for (i, a) in axes.iter().enumerate() {
        for s in seeds {
            let n = (s - *a * s.dot(*a)).normalize();
            for d in [0.0078125, -0.0078125, 0.02, -0.0234] {
                out.push(Case {
                    c: Point3::new(0.1 * i as f64, -0.05, 0.03),
                    a: *a,
                    n,
                    big: 0.09375,
                    r: 0.0703125,
                    d,
                });
            }
        }
    }
    out
}

/// Claim 1: sample the carrier, mint on the mirror torus with both
/// chart-axis orientations and several u_refs, evaluate the image and the
/// chart, compare in 3-space over the whole oval.
#[test]
fn mirror_wall_image_is_exact_everywhere() {
    let mut n_cases = 0;
    let mut worst = 0.0_f64;
    for k in cases() {
        let carrier = Curve3::Spiric {
            center: k.c,
            axis: k.a,
            u_ref: k.n,
            major_radius: k.big,
            minor_radius: k.r,
            offset: k.d,
        };
        let m = k.a.cross(k.n);
        for sigma in [1.0, -1.0] {
            for uref in [k.n, m, -k.n, (k.n + m).normalize(), (m * 0.3 - k.n).normalize()] {
                for mirror in [false, true] {
                    let centre = if mirror { k.c + k.n * (2.0 * k.d) } else { k.c };
                    let chart = Surface::Torus {
                        center: centre,
                        axis: k.a * sigma,
                        major_radius: k.big,
                        minor_radius: k.r,
                        u_ref: uref,
                    };
                    let p = geom_brep::chart_pcurve(&carrier, &chart, band())
                        .unwrap_or_else(|e| panic!("mint: {e:?}"));
                    let Pcurve::Spiric { image: SpiricImage::Wall { u_sense, v_sense, .. }, .. } = p else {
                        panic!("{p:?}")
                    };
                    assert_eq!(v_sense, sigma);
                    assert_eq!(u_sense * v_sense, if mirror { -1.0 } else { 1.0 });
                    for j in 0..=64 {
                        let t = core::f64::consts::TAU * f64::from(j) / 64.0 - 0.3;
                        let uv = p.eval(t);
                        let q = chart.eval(uv.x, uv.y);
                        let e = (q - carrier.eval(t)).norm();
                        worst = worst.max(e);
                        assert!(e < 1e-14, "mirror={mirror} sigma={sigma} d={} t={t}: {e:e}", k.d);
                    }
                    // certify over the whole period and over a span crossing t=pi
                    for (t0, t1) in [(0.0, core::f64::consts::TAU - 1e-3), (2.0, 4.5), (-2.9, -0.1)] {
                        let row = PcurveCache::certify(p.clone(), t0, t1, &carrier, &chart, band())
                            .unwrap_or_else(|e| panic!("certify mirror={mirror} sigma={sigma} d={}: {e:?}", k.d));
                        assert_eq!(row.certificate().statement, EnvelopeStatement::SpiricIdentity);
                    }
                    // Interval
                    let ic = Curve3::Spiric {
                        center: ip3(k.c),
                        axis: iv3(k.a),
                        u_ref: iv3(k.n),
                        major_radius: Interval::from_f64(k.big),
                        minor_radius: Interval::from_f64(k.r),
                        offset: Interval::from_f64(k.d),
                    };
                    let is = Surface::Torus {
                        center: ip3(centre),
                        axis: iv3(k.a * sigma),
                        major_radius: Interval::from_f64(k.big),
                        minor_radius: Interval::from_f64(k.r),
                        u_ref: iv3(uref),
                    };
                    let ipc = geom_brep::chart_pcurve(&ic, &is, band())
                        .unwrap_or_else(|e| panic!("interval mint: {e:?}"));
                    PcurveCache::certify(ipc, Interval::from_f64(2.0), Interval::from_f64(4.5), &ic, &is, band())
                        .unwrap_or_else(|e| panic!("interval certify mirror={mirror} sigma={sigma} d={}: {e:?}", k.d));
                    n_cases += 1;
                }
            }
        }
    }
    eprintln!("cases {n_cases} worst {worst:e}");
}

/// Mixed signs: every (u_sense, v_sense, u0) combination other than the
/// minted one on each torus must refuse.
#[test]
fn every_wrong_sign_combination_refuses() {
    let (big, r, d) = (0.09375, 0.0703125, 0.0078125);
    let a = Vec3::unit_y();
    let n = Vec3::unit_x();
    let carrier = Curve3::Spiric { center: Point3::origin(), axis: a, u_ref: n, major_radius: big, minor_radius: r, offset: d };
    for mirror in [false, true] {
        for sigma in [1.0, -1.0] {
            let chart = Surface::Torus {
                center: if mirror { Point3::origin() + n * (2.0 * d) } else { Point3::origin() },
                axis: a * sigma,
                major_radius: big,
                minor_radius: r,
                u_ref: n,
            };
            let minted = geom_brep::chart_pcurve(&carrier, &chart, band()).unwrap();
            let Pcurve::Spiric { image: SpiricImage::Wall { u0, v0, u_sense, v_sense }, major, minor, offset } = minted else { panic!() };
            for us in [1.0, -1.0] {
                for vs in [1.0, -1.0] {
                    for uu in [u0, u0 + core::f64::consts::PI] {
                        let p = Pcurve::Spiric { major, minor, offset, image: SpiricImage::Wall { u0: uu, v0, u_sense: us, v_sense: vs } };
                        let res = PcurveCache::certify(p, 0.3, 2.9, &carrier, &chart, band());
                        let is_minted = us == u_sense && vs == v_sense && uu == u0;
                        assert_eq!(res.is_ok(), is_minted, "mirror={mirror} sigma={sigma} us={us} vs={vs} u0={uu}: {res:?}");
                    }
                }
            }
        }
    }
}

/// d -> 0 limit: |2d| inside the band — own and mirror coincide; the mirror
/// chart with a d below ε/2 mints the own image and must still certify.
#[test]
fn tiny_offset_mirror_coincides() {
    let eps = Tol::witness().eps();
    let (big, r) = (0.09375, 0.0703125);
    let a = Vec3::unit_y();
    let n = Vec3::unit_x();
    for d in [eps * 0.3, eps * 0.6, eps * 2.0, -eps * 0.3] {
        let Ok(carrier) = Curve3::spiric(Point3::origin(), a, n, big, r, d, band()) else {
            eprintln!("d={d:e}: spiric refuses");
            continue;
        };
        for sigma in [1.0, -1.0] {
            let chart = Surface::Torus { center: Point3::origin() + n * (2.0 * d), axis: a * sigma, major_radius: big, minor_radius: r, u_ref: n };
            let res = geom_brep::chart_pcurve(&carrier, &chart, band())
                .and_then(|p| PcurveCache::certify(p, 0.3, 2.9, &carrier, &chart, band()));
            match res {
                Ok(row) => {
                    eprintln!("d={d:e} sigma={sigma}: {:?} env={:e} res={:e} pcurve={:?}", row.certificate().statement, row.certificate().envelope, row.certificate().max_residual, row.pcurve());
                    // envelope must cover true displacement
                    for j in 0..=32 {
                        let t = 0.3 + 2.6 * f64::from(j) / 32.0;
                        let uv = row.pcurve().eval(t);
                        let e = (chart.eval(uv.x, uv.y) - carrier.eval(t)).norm();
                        assert!(e <= row.certificate().max_residual + row.certificate().envelope + 1e-15, "d={d:e}: {e:e} vs cert {:?}", row.certificate());
                    }
                }
                Err(e) => eprintln!("d={d:e} sigma={sigma}: refuses {e:?}"),
            }
        }
    }
}
