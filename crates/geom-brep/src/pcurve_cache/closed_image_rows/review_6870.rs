//! Review probes for PR 6870 (not for merge). Run with `--nocapture`.
#![allow(clippy::all)]
use super::*;
use geom_core::Bounds;
use crate::pcurve_cache::PcurveCertifyError;

fn lo_hi_miss(
    surface: &Surface<f64>,
    carrier: &Curve3<f64>,
    image: &Pcurve<f64>,
    (t0, t1): (f64, f64),
    n: u32,
) -> (f64, f64, f64) {
    let s = surface.map_scalar(Interval::from_certified);
    let c = carrier.map_scalar(Interval::from_certified);
    let i = lift::<Interval>(image);
    let mut best = (0.0, 0.0, t0);
    for k in 0..=n {
        let tf = t0 + (t1 - t0) * f64::from(k) / f64::from(n);
        let t = Interval::from_certified(tf);
        let uv = i.eval(t);
        let d = (s.eval(uv.x, uv.y) - c.eval(t)).norm();
        if d.hi() > best.1 {
            best = (d.lo(), d.hi(), tf);
        }
    }
    best
}

fn rot(v: Vec3<f64>, k: Vec3<f64>, th: f64) -> Vec3<f64> {
    // Rodrigues, k unit
    v * th.cos() + k.cross(v) * th.sin() + k * (k.dot(v) * (1.0 - th.cos()))
}

fn unit(v: Vec3<f64>) -> Vec3<f64> {
    v * (1.0 / v.norm())
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn r(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn dir(&mut self) -> Vec3<f64> {
        unit(Vec3::new(self.r(-1., 1.), self.r(-1., 1.), self.r(-1., 1.)))
    }
}

fn frame(g: &mut Rng) -> (Vec3<f64>, Vec3<f64>) {
    let axis = g.dir();
    let b = g.dir();
    let u = b - axis * b.dot(axis);
    let u = unit(u);
    let u = unit(u - axis * u.dot(axis));
    (axis, u)
}

fn verdict(
    surface: &Surface<f64>,
    carrier: &Curve3<f64>,
    span: (f64, f64),
) -> (String, f64, f64) {
    match chart_pcurve_over(carrier, span.0, span.1, surface, band()) {
        Ok(image) if matches!(image, Pcurve::Fitted(_) | Pcurve::Projected(_)) => {
            let doors: Vec<_> = DOORS
                .iter()
                .take(0)
                .map(|_| String::new())
                .collect::<Vec<_>>();
            (format!("general-lane {doors:?}"), f64::NAN, f64::NAN)
        }
        Ok(image) => {
            let (lo, hi, _) = lo_hi_miss(surface, carrier, &image, span, 4096);
            let doors: Vec<_> = DOORS
                .iter()
                .map(|d| match (d.certify)(surface, &image, carrier, span) {
                    Ok(()) => "ok".to_string(),
                    Err(e) => format!("{e:?}").chars().take(60).collect(),
                })
                .collect();
            (format!("{doors:?}"), lo, hi)
        }
        Err(e) => (format!("derive-refused {e:?}").chars().take(90).collect(), f64::NAN, f64::NAN),
    }
}

/// Sphere meridians: pole starts (exact, near), antipodal, equator,
/// in-band plane tilt and in-band centre offset.
#[test]
fn probe_sphere_meridians() {
    let mut g = Rng(0x9e3779b97f4a7c15);
    let mut worst = 0.0f64;
    let mut n_ref = 0;
    let mut rows = 0;
    let esc = band().escalate();
    for trial in 0..60 {
        let (axis, u_ref) = frame(&mut g);
        let centre = Point3::new(g.r(-10., 10.), g.r(-10., 10.), g.r(-10., 10.));
        let r = [0.01, 0.5, 2.13, 9.0][trial % 4];
        let u0 = g.r(-PI, PI);
        let radial = rot(u_ref, axis, u0);
        let normal0 = axis.cross(radial);
        for &s in &[
            0.0, 1e-15, 1e-13, 1e-11, 1e-9, 1e-7, 4.4e-5, 1e-3, 0.3, PI / 2.0, PI - 1e-6, PI,
            -1e-7, -4.4e-5,
        ] {
            for &(tilt, off) in &[(0.0, 0.0), (0.5, 0.0), (-0.5, 0.0), (0.0, 0.5)] {
                // tilt/off as a fraction of the escalation band at r
                let tau = tilt * band().zero() / r;
                let normal = unit(rot(normal0, radial, tau));
                let start = unit(axis * s.cos() + radial * s.sin());
                let start = unit(start - normal * start.dot(normal));
                let ctr = centre + normal * (off * band().zero());
                let rr = (r * r - (off * band().zero()).powi(2)).sqrt();
                let sphere = Surface::Sphere { center: centre, radius: r, axis, u_ref };
                let carrier = Curve3::circle(ctr, normal, rr, start);
                for span in [(-0.4, TAU - 0.4), (0.1, 3.0), (PI / 2.0 - 0.2, PI / 2.0 + 0.2)] {
                    rows += 1;
                    let (v, lo, hi) = verdict(&sphere, &carrier, span);
                    let bad = !(hi < 1e-13) || !v.contains("\"ok\", \"ok\"");
                    if hi.is_finite() {
                        worst = worst.max(hi / r.max(1.0));
                    }
                    if bad {
                        n_ref += 1;
                        println!(
                            "SPH trial {trial} r {r} s {s:e} tilt {tilt} off {off} span {span:?}: {v} miss [{lo:e},{hi:e}]"
                        );
                    }
                }
            }
        }
    }
    println!("SPH rows {rows} flagged {n_ref} worst rel miss {worst:e}");
}

/// Torus / spindle meridians across R/r, at the outer and inner
/// equator and the pole start; Villarceau.
#[test]
fn probe_torus_meridians() {
    let mut g = Rng(0x1234_5678_9abc_def1);
    let mut rows = 0;
    let mut flagged = 0;
    for trial in 0..12 {
        let (axis, u_ref) = frame(&mut g);
        let centre = Point3::new(g.r(-10., 10.), g.r(-10., 10.), g.r(-10., 10.));
        let r = [0.3, 2.0, 7.0][trial % 3];
        let u0 = g.r(-PI, PI);
        let radial = rot(u_ref, axis, u0);
        let normal = axis.cross(radial);
        for &q in &[1.0, 0.5, 1e-1, 1e-3, 1e-5, 1e-7, 1e-9, 1.5, 10.0, 1e3] {
            let big = q * r;
            let surface = if q < 1.0 {
                Surface::Spindle { center: centre, axis, major_radius: big, minor_radius: r, u_ref }
            } else if q > 1.0 {
                Surface::Torus { center: centre, axis, major_radius: big, minor_radius: r, u_ref }
            } else {
                Surface::Spindle { center: centre, axis, major_radius: big * (1.0 - 1e-9), minor_radius: r, u_ref }
            };
            let big = if q == 1.0 { big * (1.0 - 1e-9) } else { big };
            for (name, start) in [
                ("outer", radial),
                ("inner", radial * -1.0),
                ("pole", axis),
                ("south", axis * -1.0),
                ("tilted", unit(axis * 0.6 + radial * 0.8)),
            ] {
                for sgn in [1.0, -1.0] {
                    let carrier = Curve3::circle(centre + radial * big, normal * sgn, r, start);
                    let x = 0.999 * q.min(1.0).asin();
                    let spans: Vec<(f64, f64)> = if q < 1.0 {
                        vec![(-1.0, 1.0), (0.2, 1.2), (-0.05, 0.05), (-1.2, x), (-x, 1.2), (x - 1e-3, x), (-x, 1e-3 - x)]
                    } else {
                        vec![(-0.4, TAU - 0.4), (1.0, 2.0)]
                    };
                    for span in spans {
                        rows += 1;
                        let (v, lo, hi) = verdict(&surface, &carrier, span);
                        if !(hi < 1e-13) || !v.contains("\"ok\", \"ok\"") {
                            flagged += 1;
                            println!(
                                "TOR trial {trial} r {r} R/r {q:e} {name} sgn {sgn} span {span:?}: {v} miss [{lo:e},{hi:e}]"
                            );
                        }
                    }
                }
            }
        }
        // Villarceau at several minor/major ratios
        for &s in &[0.9, 0.5, 1e-1, 1e-3, 1e-5, 1e-7] {
            let torus = Surface::Torus { center: centre, axis, major_radius: r, minor_radius: s * r, u_ref };
            for sense in [1.0, -1.0] {
                let lean = radial.cross(axis) * f64::cos(s.asin()) + axis * (sense * s);
                let villarceau = Curve3::circle(
                    centre + radial * (s * r),
                    radial.cross(lean),
                    r,
                    radial * f64::cos(u0) + lean * f64::sin(u0),
                );
                rows += 1;
                let (v, lo, hi) = verdict(&torus, &villarceau, (-0.4, TAU - 0.4));
                if !(hi < 1e-13) || !v.contains("\"ok\", \"ok\"") {
                    flagged += 1;
                    println!("VIL trial {trial} r {r} s {s:e} sense {sense}: {v} miss [{lo:e},{hi:e}]");
                }
            }
        }
    }
    println!("TOR rows {rows} flagged {flagged}");
}

/// A cone ruling whose origin sits near the apex: its azimuth is read
/// off `radial(origin − apex)`, short, and shown at the span's far
/// radius.
#[test]
fn probe_cone_ruling_near_apex() {
    let mut g = Rng(0xdead_beef_cafe_f00d);
    for trial in 0..6 {
        let (axis, u_ref) = frame(&mut g);
        let apex = Point3::new(g.r(-10., 10.), g.r(-10., 10.), g.r(-10., 10.));
        let ha = [0.5, 0.1, 1.2][trial % 3];
        let u0 = g.r(-PI, PI);
        let radial = rot(u_ref, axis, u0);
        let gdir = unit(axis * ha.cos() + radial * ha.sin());
        let cone = Surface::Cone { apex, axis, half_angle: ha, u_ref };
        for &d in &[1.0, 1e-3, 1e-5, 1e-7, 1e-9, 1e-11, 0.0] {
            let line = Curve3::line(apex + gdir * d, gdir);
            for span in [(0.0, 2.0), (0.5, 5.0)] {
                let (v, lo, hi) = verdict(&cone, &line, span);
                println!("CONE trial {trial} ha {ha} d {d:e} span {span:?}: {v} miss [{lo:e},{hi:e}]");
            }
        }
    }
}

/// Dump derived meridian image bits for product-style fixtures (axis
/// aligned, integer-ish placements) and some tilted ones; diff across
/// the head and the `meridian-direction-unprojected` build.
#[test]
fn probe_dump_meridian_bits() {
    let mut out = Vec::new();
    let axes = [
        (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)),
        (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        (unit(Vec3::new(0.36, -0.48, 0.8)), Vec3::new(0.8, 0.6, 0.0)),
        (unit(Vec3::new(1.0, 1.0, 1.0)), unit(Vec3::new(1.0, -1.0, 0.0))),
    ];
    for (ai, &(axis, u_ref)) in axes.iter().enumerate() {
        let cv = axis.cross(u_ref);
        for centre in [Point3::new(0.0, 0.0, 0.0), Point3::new(10.0, -5.0, 2.5), Point3::new(0.3, 0.7, -1.1)] {
            for &r in &[1.0, 2.5, 0.125] {
                for k in 0..8 {
                    let u0 = f64::from(k) * PI / 4.0;
                    let radial = u_ref * u0.cos() + cv * u0.sin();
                    let normal = axis.cross(radial);
                    for (name, start) in [("eq", radial), ("pole", axis), ("mid", unit(axis + radial))] {
                        let sphere = Surface::Sphere { center: centre, radius: r, axis, u_ref };
                        let c = Curve3::circle(centre, normal, r, start);
                        let img = chart_pcurve_over(&c, 0.1, 2.0, &sphere, band());
                        out.push(format!("S a{ai} {centre:?} r{r} k{k} {name}: {}", bits(&img)));
                        let torus = Surface::Torus { center: centre, axis, major_radius: 3.0 * r, minor_radius: r, u_ref };
                        let c = Curve3::circle(centre + radial * (3.0 * r), normal, r, start);
                        let img = chart_pcurve_over(&c, 0.1, 2.0, &torus, band());
                        out.push(format!("T a{ai} {centre:?} r{r} k{k} {name}: {}", bits(&img)));
                    }
                }
            }
        }
    }
    let path = format!("/home/user/bits-{}.txt", super::super::mutants::name_for_review());
    std::fs::write(&path, out.join("\n")).unwrap();
    println!("wrote {} rows to {path}", out.len());
}

fn bits(img: &Result<Pcurve<f64>, PcurveCertifyError>) -> String {
    let ch = |c: &Channel<f64>| match *c {
        Channel::Angle { phase, winding } => format!("A{:016x}/{winding:?}", phase.to_bits()),
        Channel::Free { c, a, b, l } => format!("F{:x},{:x},{:x},{:x}", c.to_bits(), a.to_bits(), b.to_bits(), l.to_bits()),
    };
    match img {
        Ok(Pcurve::Harmonic { u, v }) => format!("{} {}", ch(u), ch(v)),
        Ok(other) => format!("other {:?}", std::mem::discriminant(other)),
        Err(e) => format!("err {e:?}").chars().take(60).collect(),
    }
}

/// Replays `every_spelling` at the seed `CAD_FUZZ_SEED` and, for every
/// refused stated image, measures its enclosed miss densely.
#[test]
fn probe_seed_dense() {
    let g = &mut fuzz::start(EVERY_SPELLING);
    for _ in 0..fuzz::scaled(24) {
        for kind in KINDS {
            let (surface, carrier, span) = drawn(g, kind);
            let Ok(image) = chart_pcurve_over(&carrier, span.0, span.1, &surface, band()) else {
                continue;
            };
            if !matches!(image, Pcurve::Harmonic { .. }) {
                continue;
            }
            for (how, surface, carrier, image) in spellings(g, &surface, &carrier, span, &image) {
                for door in DOORS {
                    let got = (door.certify)(&surface, &image, &carrier, span);
                    if got.is_err() {
                        let (lo, hi, t) = lo_hi_miss(&surface, &carrier, &image, span, 20000);
                        println!(
                            "REFUSED {kind:?} {how} at {}: dense miss [{lo:e},{hi:e}] at t {t}\n  surface {surface:?}\n  carrier {carrier:?}\n  image {image:?}\n  span {span:?}\n  got {got:?}",
                            door.name
                        );
                    }
                }
            }
        }
    }
}

/// The mint door (check 4's incidence envelope) on the pinned seeds and
/// on the table's pole-start meridians: does it still refuse an image
/// the derivation now puts on its circle?
#[test]
fn probe_mint_pole_start() {
    use crate::pcurve_cache::PcurveCache;
    let mut cases: Vec<(String, Surface<f64>, Curve3<f64>, (f64, f64))> = super::pole_start_meridians()
        .into_iter()
        .map(|(n, s, c, sp)| (n.to_string(), s, c, sp))
        .collect();
    let centre = Point3::new(7.25, -8.5, 9.75);
    let axis = Vec3::new(0.36, -0.48, 0.8);
    let u_ref = Vec3::new(0.8, 0.6, 0.0);
    let v_ref = axis.cross(u_ref);
    for s in [1e-3, 1e-5, 1e-7, 0.0, 1e-1] {
        let u0 = 0.3f64;
        let radial = u_ref * u0.cos() + v_ref * u0.sin();
        let normal = v_ref * u0.cos() - u_ref * u0.sin();
        let sphere = Surface::Sphere { center: centre, radius: 2.0, axis, u_ref };
        let great = Curve3::circle(centre, normal, 2.0, axis * s.cos() + radial * s.sin());
        cases.push((format!("table s={s:e}"), sphere, great, (-0.4, TAU - 0.4)));
        let spindle = Surface::Spindle { center: centre, axis, major_radius: s * 2.0, minor_radius: 2.0, u_ref };
        let meridian = Curve3::circle(centre + radial * (s * 2.0), normal, 2.0, radial);
        cases.push((format!("spindle s={s:e}"), spindle, meridian, (-1.5, 1.5)));
    }
    for (name, sphere, carrier, span) in cases {
        let Ok(image) = chart_pcurve_over(&carrier, span.0, span.1, &sphere, band()) else { println!("MINT {name}: derive refused"); continue; };
        let miss = lo_hi_miss(&sphere, &carrier, &image, span, 4096);
        let f = PcurveCache::<f64>::certify(image.clone(), span.0, span.1, &carrier, &sphere, band());
        let si = sphere.map_scalar(Interval::from_certified);
        let ci = carrier.map_scalar(Interval::from_certified);
        let i = PcurveCache::<Interval>::certify(
            lift::<Interval>(&image),
            Interval::from_certified(span.0),
            Interval::from_certified(span.1),
            &ci,
            &si,
            band(),
        );
        let short = |r: String| r.chars().take(200).collect::<String>();
        println!(
            "MINT {name}: miss {miss:?}\n  f64 {}\n  interval {}",
            short(format!("{:?}", f.as_ref().map(|_| ()))),
            short(format!("{:?}", i.as_ref().map(|_| ())))
        );
    }
}
