//! Review 4553 probe: plane × NURBS lane on the reviewer's own walls,
//! printed for a main-vs-PR comparison (claim 3).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve3, NurbsSurface, Surface};
use geom_core::Bounds;
use geom_core::interval::Interval;
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Real, Tol, Vec3};

/// A cross-section in z = 0: knots, (x, y) control, weights.
type Section = (Vec<f64>, usize, Vec<(f64, f64)>, Vec<f64>);

fn sections() -> Vec<(&'static str, Section)> {
    let h = core::f64::consts::FRAC_1_SQRT_2;
    vec![
        (
            "quarter",
            (
                vec![0., 0., 0., 1., 1., 1.],
                2,
                vec![(1., 0.), (1., 1.), (0., 1.)],
                vec![1., h, 1.],
            ),
        ),
        (
            "semicircle",
            (
                vec![0., 0., 0., 0.5, 0.5, 1., 1., 1.],
                2,
                vec![(1., 0.), (1., 1.), (0., 1.), (-1., 1.), (-1., 0.)],
                vec![1., h, 1., h, 1.],
            ),
        ),
        (
            "steep-rational",
            (
                vec![0., 0., 0., 0.3, 0.55, 1., 1., 1.],
                2,
                vec![(0., 0.), (0.4, 0.9), (1.1, 1.2), (1.9, 0.4), (2.5, 1.0)],
                vec![1., 5., 0.2, 3., 1.],
            ),
        ),
        (
            "cubic-wavy",
            (
                vec![0., 0., 0., 0., 0.1, 0.37, 0.6, 0.81, 1., 1., 1., 1.],
                3,
                vec![
                    (0., 0.),
                    (0.3, 0.5),
                    (0.7, -0.2),
                    (1.1, 0.6),
                    (1.6, -0.1),
                    (2.0, 0.4),
                    (2.3, 0.0),
                    (2.9, 0.3),
                ],
                vec![1.; 8],
            ),
        ),
        (
            "rational-cubic-decades",
            (
                vec![0., 0., 0., 0., 0.2, 0.7, 1., 1., 1., 1.],
                3,
                vec![
                    (0., 0.),
                    (0.5, 0.8),
                    (1.0, 1.0),
                    (1.6, 0.3),
                    (2.0, 0.9),
                    (2.4, 0.2),
                ],
                vec![1., 0.03, 30., 0.5, 8., 1.],
            ),
        ),
    ]
}

fn wall<T: Real>(s: &Section) -> NurbsSurface<T> {
    let (k, p, ctl, w) = s;
    let ku = KnotVector::clamped(k.clone(), *p).unwrap();
    let kv = KnotVector::clamped(vec![0., 0., 1., 1.], 1).unwrap();
    let f = T::from_f64;
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for (i, (x, y)) in ctl.iter().enumerate() {
        for z in [0.0, 1.0] {
            control.push(Point3::new(f(*x), f(*y), f(z)));
            weights.push(w[i]);
        }
    }
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

fn carrier<T: Real>(s: &Section, z: f64) -> NurbsCurve3<T> {
    let (k, p, ctl, w) = s;
    let ku = KnotVector::clamped(k.clone(), *p).unwrap();
    let f = T::from_f64;
    let control = ctl
        .iter()
        .map(|(x, y)| Point3::new(f(*x), f(*y), f(z)))
        .collect();
    NurbsCurve3::new(ku, control, w.clone()).unwrap()
}

fn plane<T: Real>(z: f64) -> Surface<T> {
    let f = T::from_f64;
    Surface::Plane {
        origin: Point3::new(f(0.), f(0.), f(z)),
        normal: Vec3::new(f(0.), f(0.), f(1.)),
        u_ref: Vec3::new(f(1.), f(0.), f(0.)),
    }
}

#[test]
fn review_4553_pxn_table() {
    let band = Band::linear(Tol::witness()).unwrap();
    let eps = Tol::witness().get().eps;
    for z in [0.5, 0.3] {
        for (name, s) in sections() {
            let r = geom_brep::plane_nurbs_limbs::<f64>(
                &carrier::<f64>(&s, z),
                &plane::<f64>(z),
                &wall::<f64>(&s),
                1.0,
                band,
            );
            match r {
                Ok(l) => println!(
                    "PXN eps={eps:e} z={z} {name} f64 OK hull={:e} tr={:e} boxes={} tube={:?} onloc={:e}",
                    l.hull_sup, l.tube_transversality, l.tube_boxes, l.tube, l.on_locus_max
                ),
                Err(e) => println!("PXN eps={eps:e} z={z} {name} f64 ERR {e:?}"),
            }
            let r = geom_brep::plane_nurbs_limbs::<Interval>(
                &carrier::<Interval>(&s, z),
                &plane::<Interval>(z),
                &wall::<Interval>(&s),
                Interval::one(),
                band,
            );
            match r {
                Ok(l) => println!(
                    "PXN eps={eps:e} z={z} {name} itv OK hull={:e} tr={:e} boxes={} tube={:?}",
                    l.hull_sup.hi(),
                    l.tube_transversality.lo(),
                    l.tube_boxes,
                    l.tube
                ),
                Err(e) => println!("PXN eps={eps:e} z={z} {name} itv ERR {e:?}"),
            }
        }
    }
}

fn ruling<T: Real>(x: f64, y: f64) -> NurbsCurve3<T> {
    let f = T::from_f64;
    let kv = KnotVector::clamped(vec![0., 0., 1., 1.], 1).unwrap();
    NurbsCurve3::new(
        kv,
        vec![
            Point3::new(f(x), f(y), f(0.0)),
            Point3::new(f(x), f(y), f(1.0)),
        ],
        vec![1.0, 1.0],
    )
    .unwrap()
}

fn vplane<T: Real>(x: f64, y: f64, tx: f64, ty: f64) -> Surface<T> {
    let f = T::from_f64;
    Surface::Plane {
        origin: Point3::new(f(x), f(y), f(0.0)),
        normal: Vec3::new(f(tx), f(ty), f(0.0)),
        u_ref: Vec3::new(f(0.0), f(0.0), f(1.0)),
    }
}

#[test]
fn review_4553_pxn_rulings() {
    let band = Band::linear(Tol::witness()).unwrap();
    let eps = Tol::witness().get().eps;
    for (name, s) in sections() {
        let sec = carrier::<f64>(&s, 0.0);
        for u0 in [0.13, 0.41, 0.77] {
            let q = sec.eval(u0);
            let t = sec.deriv(u0);
            match geom_brep::plane_nurbs_limbs::<f64>(
                &ruling(q.x, q.y),
                &vplane(q.x, q.y, t.x, t.y),
                &wall::<f64>(&s),
                1.0,
                band,
            ) {
                Ok(l) => println!(
                    "RUL eps={eps:e} u0={u0} {name} f64 OK hull={:e} tr={:e} boxes={} tube={:?}",
                    l.hull_sup, l.tube_transversality, l.tube_boxes, l.tube
                ),
                Err(e) => println!("RUL eps={eps:e} u0={u0} {name} f64 ERR {e:?}"),
            }
            match geom_brep::plane_nurbs_limbs::<Interval>(
                &ruling(q.x, q.y),
                &vplane(q.x, q.y, t.x, t.y),
                &wall::<Interval>(&s),
                Interval::one(),
                band,
            ) {
                Ok(l) => {
                    let pads = match l.tube {
                        geom_brep::SsiTube::Chart { rung, pad_u, pad_v } => {
                            format!("rung={} pu={:e} pv={:e}", rung.hi(), pad_u.lo(), pad_v.lo())
                        }
                        other => format!("{other:?}"),
                    };
                    println!(
                        "RUL eps={eps:e} u0={u0} {name} itv OK hull={:e} tr={:e} boxes={} {pads}",
                        l.hull_sup.hi(),
                        l.tube_transversality.lo(),
                        l.tube_boxes
                    )
                }
                Err(e) => println!("RUL eps={eps:e} u0={u0} {name} itv ERR {e:?}"),
            }
        }
    }
}

#[test]
fn review_4553_pxn_decades_sweep() {
    let band = Band::linear(Tol::witness()).unwrap();
    let eps = Tol::witness().get().eps;
    let base = sections()
        .into_iter()
        .find(|(n, _)| *n == "rational-cubic-decades")
        .unwrap()
        .1;
    for (tag, w) in [
        ("d3", vec![1., 0.03, 30., 0.5, 8., 1.]),
        ("d2", vec![1., 0.1, 10., 0.5, 3., 1.]),
        ("d1", vec![1., 0.5, 2., 0.7, 1.5, 1.]),
        ("unit", vec![1.; 6]),
    ] {
        let s = (base.0.clone(), base.1, base.2.clone(), w);
        let sec = carrier::<f64>(&s, 0.0);
        let (mut ok, mut bad, mut minrung) = (0, Vec::new(), f64::INFINITY);
        for i in 1..20 {
            let u0 = f64::from(i) / 20.0 + 0.003;
            let q = sec.eval(u0);
            let t = sec.deriv(u0);
            match geom_brep::plane_nurbs_limbs::<Interval>(
                &ruling(q.x, q.y),
                &vplane(q.x, q.y, t.x, t.y),
                &wall::<Interval>(&s),
                Interval::one(),
                band,
            ) {
                Ok(l) => {
                    ok += 1;
                    if let geom_brep::SsiTube::Chart { rung, .. } = l.tube {
                        minrung = minrung.min(rung.hi());
                    }
                }
                Err(e) => bad.push(format!(
                    "{u0}:{}",
                    format!("{e:?}").chars().take(40).collect::<String>()
                )),
            }
        }
        println!("SWEEP eps={eps:e} {tag} itv ok={ok}/19 minrung={minrung:e} bad={bad:?}");
    }
}
