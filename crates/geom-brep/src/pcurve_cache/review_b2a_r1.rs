//! Review probes for PR 5244 (PCERT B2a), lane r1. Not for merging.

use core::f64::consts::{FRAC_PI_6, PI, TAU};

use geom_core::{Interval, Tol};

use super::*;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn circle(center: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    }
}

/// `(name, surface, carrier, span)` over every analytic chart kind.
fn fixtures() -> Vec<(&'static str, Surface<f64>, Curve3<f64>, (f64, f64))> {
    let z = Vec3::unit_z();
    let x = Vec3::unit_x();
    let y = Vec3::unit_y();
    let o = Point3::origin();
    let plane = Surface::Plane {
        origin: o,
        normal: z,
        u_ref: x,
    };
    let cyl = Surface::Cylinder {
        origin: o,
        axis: z,
        radius: 0.5,
        u_ref: x,
    };
    let cone = Surface::Cone {
        apex: o,
        axis: z,
        half_angle: FRAC_PI_6,
        u_ref: x,
    };
    let sphere = Surface::Sphere {
        center: o,
        radius: 1.0,
        axis: z,
        u_ref: x,
    };
    let torus = Surface::Torus {
        center: o,
        axis: z,
        major_radius: 1.0,
        minor_radius: 0.25,
        u_ref: x,
    };
    let s = (0.2, 1.0);
    vec![
        ("plane circle", plane, circle(o, z, 0.5, x), s),
        ("cylinder rim", cyl.clone(), circle(Point3::new(0.0, 0.0, 0.3), z, 0.5, x), s),
        (
            "cone rim",
            cone.clone(),
            circle(Point3::new(0.0, 0.0, 1.0), z, FRAC_PI_6.tan(), x),
            s,
        ),
        (
            "sphere parallel",
            sphere.clone(),
            circle(Point3::new(0.0, 0.0, 0.5), z, 0.75_f64.sqrt(), x),
            s,
        ),
        ("sphere meridian", sphere.clone(), circle(o, y, 1.0, x), s),
        ("torus parallel", torus.clone(), circle(o, z, 1.25, x), s),
        (
            "torus meridian",
            torus.clone(),
            circle(Point3::new(1.0, 0.0, 0.0), y, 0.25, x),
            s,
        ),
    ]
}

fn lift(p: &Pcurve<f64>) -> Pcurve<Interval> {
    let Pcurve::Harmonic { u, v } = *p else {
        panic!("harmonic")
    };
    let f = Interval::from_f64;
    let l = |c: Channel<f64>| match c {
        Channel::Angle { phase, winding } => Channel::Angle {
            phase: f(phase),
            winding,
        },
        Channel::Free { c, a, b, l } => Channel::Free {
            c: f(c),
            a: f(a),
            b: f(b),
            l: f(l),
        },
    };
    Pcurve::Harmonic { u: l(u), v: l(v) }
}

fn both(
    p: &Pcurve<f64>,
    (t0, t1): (f64, f64),
    carrier: &Curve3<f64>,
    s: &Surface<f64>,
) -> (String, String) {
    let f = Interval::from_f64;
    let a = PcurveCache::certify(p.clone(), t0, t1, carrier, s, band());
    let b = PcurveCache::certify(
        lift(p),
        f(t0),
        f(t1),
        &carrier.map_scalar(f),
        &s.map_scalar(f),
        band(),
    );
    let short = |r: String| r.chars().take(160).collect::<String>();
    (
        short(match a {
            Ok(_) => "Ok".into(),
            Err(e) => format!("{e:?}"),
        }),
        short(match b {
            Ok(_) => "Ok".into(),
            Err(e) => format!("{e:?}"),
        }),
    )
}

fn swapped(c: Channel<f64>) -> Channel<f64> {
    match c {
        Channel::Angle { .. } => {
            let [c, a, b, l] = c.coefficients();
            Channel::Free { c, a, b, l }
        }
        Channel::Free { c, l, .. } => Channel::Angle {
            phase: c,
            winding: l.round() as i32,
        },
    }
}

/// Claim 3: a kind mismatch on either axis refuses typed, at both lanes,
/// on every analytic chart.
#[test]
fn r1_channel_kind_mismatch_every_chart() {
    let mut bad = Vec::new();
    for (name, s, carrier, span) in fixtures() {
        let derived = chart_pcurve(&carrier, &s, band()).unwrap();
        let (a, b) = both(&derived, span, &carrier, &s);
        eprintln!("{name}: derived {derived:?} -> f64 {a} | iv {b}");
        assert_eq!((a.as_str(), b.as_str()), ("Ok", "Ok"), "{name} derived certifies");
        let Pcurve::Harmonic { u, v } = derived else {
            panic!()
        };
        for (axis, p) in [
            ("u", Pcurve::Harmonic { u: swapped(u), v }),
            ("v", Pcurve::Harmonic { u, v: swapped(v) }),
            (
                "uv",
                Pcurve::Harmonic {
                    u: swapped(u),
                    v: swapped(v),
                },
            ),
        ] {
            let (a, b) = both(&p, span, &carrier, &s);
            eprintln!("  {name} swap {axis}: f64 {a} | iv {b}");
            if !a.starts_with("ChannelKindMismatch") || !b.starts_with("ChannelKindMismatch") {
                bad.push(format!("{name} {axis}: {a} | {b}"));
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// Claim 2: every stored winding other than the derived one refuses; which
/// variant each lane names.
#[test]
fn r1_winding_offsets_every_chart() {
    let mut ok_wrong = Vec::new();
    for (name, s, carrier, span) in fixtures() {
        let derived = chart_pcurve(&carrier, &s, band()).unwrap();
        let Pcurve::Harmonic { u, v } = derived else {
            panic!()
        };
        for (axis, ch) in [("u", u), ("v", v)] {
            let Channel::Angle { phase, winding } = ch else {
                continue;
            };
            for w in [
                winding - 1,
                winding + 1,
                -winding,
                2,
                -2,
                1000,
                i32::MAX,
                i32::MIN,
            ] {
                if w == winding {
                    continue;
                }
                let c = Channel::Angle { phase, winding: w };
                let p = if axis == "u" {
                    Pcurve::Harmonic { u: c, v }
                } else {
                    Pcurve::Harmonic { u, v: c }
                };
                let (a, b) = both(&p, span, &carrier, &s);
                eprintln!("  {name} {axis} {winding}->{w}: f64 {a} | iv {b}");
                if a == "Ok" || b == "Ok" {
                    ok_wrong.push(format!("{name} {axis} {winding}->{w}: {a} | {b}"));
                }
            }
        }
    }
    // The sphere meridian's twin (v −w) is expected to be Ok only if the
    // phase is the twin's too; here the phase is P_d's, so every row should
    // refuse.
    assert!(ok_wrong.is_empty(), "{ok_wrong:#?}");
}

/// The sphere meridian's twin, built through the chart's involution,
/// certifies (windings name the twin), and the twin with its windings
/// restored to the derivation's refuses.
#[test]
fn r1_sphere_meridian_twin_named_by_windings() {
    let (_, s, carrier, span) = fixtures().remove(4);
    let derived = chart_pcurve(&carrier, &s, band()).unwrap();
    let twin = derived.map_affine(ChartMap {
        u: AxisMap::shift(PI),
        v: AxisMap::reflect_about(PI),
    });
    let (a, b) = both(&twin, span, &carrier, &s);
    eprintln!("twin: {twin:?} f64 {a} | iv {b}");
    assert_eq!((a.as_str(), b.as_str()), ("Ok", "Ok"));
    let Pcurve::Harmonic { u, v } = twin else {
        panic!()
    };
    let Channel::Angle { phase, winding } = v else {
        panic!()
    };
    let unnamed = Pcurve::Harmonic {
        u,
        v: Channel::Angle {
            phase,
            winding: -winding,
        },
    };
    let (a, b) = both(&unnamed, span, &carrier, &s);
    eprintln!("twin with derived windings: f64 {a} | iv {b}");
    assert!(a != "Ok" && b != "Ok");
}

/// The edge winding past i32: a reflection of `i32::MIN`.
#[test]
fn r1_reflect_i32_min() {
    let c = Channel::Angle {
        phase: 0.0,
        winding: i32::MIN,
    };
    let r = std::panic::catch_unwind(|| c.mapped(AxisMap::reflect()));
    eprintln!("reflect i32::MIN: {r:?}");
    let r = std::panic::catch_unwind(|| Period::<f64>::rate(i32::MIN));
    eprintln!("rate i32::MIN: {r:?}");
}

/// The retired harmonic period gate: spans past one period now certify on
/// every periodic chart.
#[test]
fn r1_span_past_a_period() {
    for (name, s, carrier, _) in fixtures() {
        let derived = chart_pcurve(&carrier, &s, band()).unwrap();
        let (a, b) = both(&derived, (0.0, TAU + 0.5), &carrier, &s);
        eprintln!("{name} over τ+0.5: f64 {a} | iv {b}");
        let (a, b) = both(&derived, (0.0, 3.0 * TAU), &carrier, &s);
        eprintln!("{name} over 3τ: f64 {a} | iv {b}");
    }
}

/// Claim 2's band-sized lever: a cone rim whose radius is a few band
/// widths, derived and certified.
#[test]
fn r1_cone_rim_at_a_band_sized_lever() {
    let tol = Tol::witness();
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: 0.1,
        u_ref: Vec3::unit_x(),
    };
    for m in [0.5, 1.0, 1.5, 3.0, 10.0] {
        let rho = m * tol.k() * tol.eps();
        let h = rho / 0.1_f64.tan();
        let carrier = circle(Point3::new(0.0, 0.0, h), Vec3::unit_z(), rho, Vec3::unit_x());
        let f = Interval::from_f64;
        let d = chart_pcurve(&carrier, &cone, band());
        let di = chart_pcurve(&carrier.map_scalar(f), &cone.map_scalar(f), band());
        eprintln!(
            "rho {rho:e}: derive f64 {:?} | iv {:?}",
            d.as_ref().map(Pcurve::harmonic_form),
            di.as_ref().map(|p| p.winding_probe())
        );
        if let Ok(p) = d {
            let (a, b) = both(&p, (0.2, 1.0), &carrier, &cone);
            eprintln!("   certify f64 {a} | iv {b}");
        }
    }
}

impl Pcurve<Interval> {
    fn winding_probe(&self) -> Option<[Option<i32>; 2]> {
        match *self {
            Pcurve::Harmonic { u, v } => Some([u.winding(), v.winding()]),
            _ => None,
        }
    }
}
