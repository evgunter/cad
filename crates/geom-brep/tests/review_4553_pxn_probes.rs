//! Review 4553's plane × NURBS probes, as rows: the reviewer's walls,
//! certified at both scalars at the run's ε.
//!
//! The plane × NURBS lane reads its tube off the wall's net refined to
//! `PXN_WALL_SPANS` spans per direction in certification arithmetic.
//! Read off the wall's own few spans instead, a rational wall whose
//! weights run over decades mints a whole-domain chart speed up to a
//! thousand times its sampled one, its pads collapse by as much, and
//! rulings `main` certifies refuse `TubeNotOneArc`. These rows hold
//! every ruling certified at `f64` and at `Interval`, and the minted
//! speed within a stated factor of the wall's sampled one.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve3, NurbsSurface, Surface};
use geom_brep::{PlaneNurbsLimbs, SsiTube};
use geom_core::interval::Interval;
use geom_core::spline::KnotVector;
use geom_core::spline::{Knots, SpanLocate};
use geom_core::{Band, Bounds, Point3, Real, Tol, Vec3};

/// A cross-section in z = 0: knots, degree, (x, y) control, weights.
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
    ]
}

/// The rational cubic with weights over `decades`: `d3` runs from 0.03
/// to 30.
fn decades(w: [f64; 6]) -> Section {
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
        w.to_vec(),
    )
}

fn decade_families() -> [(&'static str, Section); 4] {
    [
        ("d3", decades([1., 0.03, 30., 0.5, 8., 1.])),
        ("d2", decades([1., 0.1, 10., 0.5, 3., 1.])),
        ("d1", decades([1., 0.5, 2., 0.7, 1.5, 1.])),
        ("unit", decades([1.; 6])),
    ]
}

/// The section extruded by a degree-1 `v` over `z ∈ [0, 1]`.
fn wall<T: SpanLocate>(s: &Section) -> NurbsSurface<T> {
    let (k, p, ctl, w) = s;
    let ku = KnotVector::clamped(k.clone(), *p).unwrap();
    let kv = KnotVector::clamped(vec![0., 0., 1., 1.], 1).unwrap();
    let f = T::from_f64;
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for (i, (x, y)) in ctl.iter().enumerate() {
        for z in [0.0, 1.0] {
            control.push(Point3::new(f(*x), f(*y), f(z)));
            weights.push(f(w[i]));
        }
    }
    NurbsSurface::new(
        Knots::from(ku).lift(geom_core::Real::from_f64),
        Knots::from(kv).lift(geom_core::Real::from_f64),
        control,
        weights,
    )
    .unwrap()
}

fn section_curve(s: &Section) -> NurbsCurve3<f64> {
    let (k, p, ctl, w) = s;
    let ku = KnotVector::clamped(k.clone(), *p).unwrap();
    let control = ctl.iter().map(|(x, y)| Point3::new(*x, *y, 0.0)).collect();
    NurbsCurve3::new(
        Knots::from(ku).lift(geom_core::Real::from_f64),
        control,
        w.clone(),
    )
    .unwrap()
}

/// The vertical ruling of the wall through `(x, y)`.
fn ruling<T: SpanLocate>(x: f64, y: f64) -> NurbsCurve3<T> {
    let f = T::from_f64;
    let kv = KnotVector::clamped(vec![0., 0., 1., 1.], 1).unwrap();
    NurbsCurve3::new(
        Knots::from(kv).lift(geom_core::Real::from_f64),
        vec![
            Point3::new(f(x), f(y), f(0.0)),
            Point3::new(f(x), f(y), f(1.0)),
        ],
        vec![Real::one(); 2],
    )
    .unwrap()
}

/// The vertical plane through `(x, y)` whose normal is `(tx, ty, 0)`.
fn vplane<T: SpanLocate>(x: f64, y: f64, tx: f64, ty: f64) -> Surface<T> {
    let f = T::from_f64;
    Surface::Plane {
        origin: Point3::new(f(x), f(y), f(0.0)),
        normal: Vec3::new(f(tx), f(ty), f(0.0)),
        u_ref: Vec3::new(f(0.0), f(0.0), f(1.0)),
    }
}

/// The wall's `{u, v}` speeds, `sup ‖S_u‖` and `sup ‖S_v‖`, sampled on
/// a 2001 × 3 grid: a lower bound on what any certified speed can read.
fn sampled_speeds(w: &NurbsSurface<f64>) -> (f64, f64) {
    let (mut su, mut sv) = (0.0_f64, 0.0_f64);
    for i in 0..=2000 {
        for j in 0..=2 {
            let jet = w.ders(f64::from(i) / 2000.0, f64::from(j) / 2.0);
            su = su.max(jet.du.norm());
            sv = sv.max(jet.dv.norm());
        }
    }
    (su, sv)
}

/// How far above the sampled speed the minted one may read. Measured on
/// these walls the grid reads at most 2.8× along `u` and 15.8× along
/// `v` (`d3`: the extrusion's quotient-rule hull at weights 0.03…30);
/// the wall's own spans read 115× and 1000×.
const SPEED_SLACK: f64 = 32.0;

/// One certified ruling's pads, as minted speeds over sampled ones.
fn speed_ratios(tube: SsiTube<f64>, sampled: (f64, f64)) -> (f64, f64) {
    let SsiTube::Chart { rung, pad_u, pad_v } = tube else {
        panic!("a plane × NURBS tube is a chart tube")
    };
    (rung / pad_u / sampled.0, rung / pad_v / sampled.1)
}

/// Every ruling of `s` at `u0s`, at both scalars: the failures, and the
/// worst minted-over-sampled speed per axis.
fn run(label: &str, s: &Section, u0s: &[f64]) -> (Vec<String>, (f64, f64)) {
    let band = Band::linear(Tol::witness()).unwrap();
    let sampled = sampled_speeds(&wall::<f64>(s));
    let sec = section_curve(s);
    let mut bad = Vec::new();
    let mut worst = (0.0_f64, 0.0_f64);
    let mut read = |scalar: &str, u0: f64, r: (f64, f64)| -> Option<String> {
        worst = (worst.0.max(r.0), worst.1.max(r.1));
        (!(r.0 <= SPEED_SLACK && r.1 <= SPEED_SLACK)).then(|| {
            format!(
                "{label} u0={u0} {scalar}: minted speed {:.3e}× / {:.3e}× the sampled",
                r.0, r.1
            )
        })
    };
    for &u0 in u0s {
        let (q, t) = (sec.eval(u0), sec.deriv(u0));
        let at_f64: Result<PlaneNurbsLimbs<f64>, _> = geom_brep::plane_nurbs_limbs(
            &geom::Curve3::Nurbs(std::sync::Arc::new(ruling(q.x, q.y))),
            (ruling(q.x, q.y)).knot_values().domain(),
            &vplane(q.x, q.y, t.x, t.y),
            &wall::<f64>(s),
            1.0,
            band,
        );
        match at_f64 {
            Ok(l) => bad.extend(read("f64", u0, speed_ratios(l.tube, sampled))),
            Err(e) => bad.push(format!("{label} u0={u0} f64: {e:?}")),
        }
        let at_itv: Result<PlaneNurbsLimbs<Interval>, _> = geom_brep::plane_nurbs_limbs(
            &geom::Curve3::Nurbs(std::sync::Arc::new(ruling(q.x, q.y))),
            (ruling(q.x, q.y)).knot_values().domain(),
            &vplane(q.x, q.y, t.x, t.y),
            &wall::<Interval>(s),
            Interval::one(),
            band,
        );
        match at_itv {
            Ok(l) => {
                let SsiTube::Chart { rung, pad_u, pad_v } = l.tube else {
                    panic!("a plane × NURBS tube is a chart tube")
                };
                let tube = SsiTube::Chart {
                    rung: rung.hi(),
                    pad_u: pad_u.lo(),
                    pad_v: pad_v.lo(),
                };
                bad.extend(read("Interval", u0, speed_ratios(tube, sampled)));
            }
            Err(e) => bad.push(format!("{label} u0={u0} Interval: {e:?}")),
        }
    }
    (bad, worst)
}

/// **The reviewer's rulings certify at both scalars**, on every
/// section, with pads off the sampled speed: three vertical rulings per
/// wall, each against the plane through it normal to the section.
#[test]
fn every_section_ruling_certifies_at_both_scalars() {
    let mut bad = Vec::new();
    for (name, s) in sections() {
        bad.extend(run(name, &s, &[0.13, 0.41, 0.77]).0);
    }
    assert!(bad.is_empty(), "rulings refused:\n{}", bad.join("\n"));
}

/// **The decades sweep certifies its rulings at both scalars**, for
/// every weight family: rulings at `u0 = i/20 + 0.003`, all nineteen
/// on `d3` and `d2`. Read off the wall's own cells, `d3` certified 17
/// of them at 1e-9 and 6 at 1e-12 at `Interval`, every refusal
/// `TubeNotOneArc`, and both families minted 30× to 1000× the sampled
/// speed. `d1` and `unit` mint at most 4× it even on the wall's own
/// cells, so they ride as controls on every fourth ruling.
#[test]
fn the_decades_sweep_certifies_every_ruling_at_both_scalars() {
    let all: Vec<f64> = (1..20).map(|i| f64::from(i) / 20.0 + 0.003).collect();
    let controls: Vec<f64> = all.iter().copied().step_by(4).collect();
    let mut bad = Vec::new();
    let mut table = String::new();
    for (name, s) in decade_families() {
        let u0s = if matches!(name, "d3" | "d2") {
            &all
        } else {
            &controls
        };
        let (b, worst) = run(name, &s, u0s);
        table.push_str(&format!(
            "\n  {name}: minted / sampled speed u {:.3}×, v {:.3}×",
            worst.0, worst.1
        ));
        bad.extend(b);
    }
    assert!(
        bad.is_empty(),
        "rulings refused:\n{}{table}",
        bad.join("\n")
    );
    println!("{table}");
}
