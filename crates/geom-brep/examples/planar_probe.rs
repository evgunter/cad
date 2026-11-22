#![allow(clippy::unwrap_used, clippy::panic, clippy::print_stdout, missing_docs)]
use core::f64::consts::PI;
use geom::Curve3;
use geom_core::{Band, Decide, Point3, Vec3};
use geom_brep::curve_pair::{Hit, Meeting, Span, meet};

fn ln<T: Decide>(a: (f64, f64), b: (f64, f64)) -> (Curve3<T>, (T, T)) {
    let len = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
    let d = ((b.0 - a.0) / len, (b.1 - a.1) / len);
    (
        Curve3::line(
            Point3::new(T::from_f64(a.0), T::from_f64(a.1), T::zero()),
            Vec3::new(T::from_f64(d.0), T::from_f64(d.1), T::zero()),
        ),
        (T::zero(), T::from_f64(len)),
    )
}
fn arc<T: Decide>(c: (f64, f64), r: f64, t0: f64, t1: f64) -> (Curve3<T>, (T, T)) {
    (
        Curve3::circle(
            Point3::new(T::from_f64(c.0), T::from_f64(c.1), T::zero()),
            Vec3::new(T::zero(), T::zero(), T::one()),
            T::from_f64(r),
            Vec3::new(T::one(), T::zero(), T::zero()),
        ),
        (T::from_f64(t0), T::from_f64(t1)),
    )
}
fn show<T: Decide + core::fmt::Debug>(name: &str, a: (Curve3<T>, (T, T)), b: (Curve3<T>, (T, T)), band: Band) {
    for (tag, x, y) in [("ab", &a, &b), ("ba", &b, &a)] {
        let r = meet(&Span::new(&x.0, x.1), &Span::new(&y.0, y.1), band);
        let kinds = |pts: &[Meeting<T>]| {
            pts.iter()
                .map(|m| {
                    let k = |h: &Hit<T>| match h { Hit::On { .. } => "On".to_string(), Hit::End(e) => format!("End({e:?})") };
                    format!("[{} / {}]", k(&m.on_a), k(&m.on_b))
                })
                .collect::<Vec<_>>()
                .join(" ")
        };
        match r {
            Ok(m) => println!("PROBE {name} {tag} {}: OK shared={:?} points={} {}", core::any::type_name::<T>(), m.shared, m.points.len(), kinds(&m.points)),
            Err(e) => println!("PROBE {name} {tag} {}: REFUSED {:?} found={} {}", core::any::type_name::<T>(), e.fault, e.found.len(), kinds(&e.found)),
        }
    }
}
fn all<T: Decide + core::fmt::Debug>() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let g = 5.5e-9;
    show::<T>("1 tangent joint line->arc", ln((-1.0, 0.0), (0.0, 0.0)), arc((0.0, 1.0), 1.0, -PI / 2.0, 0.0), band);
    show::<T>("1b corner joint line->line", ln((-1.0, 0.0), (0.0, 0.0)), ln((0.0, 0.0), (0.5, 1.0)), band);
    show::<T>("1c tangent joint arc->arc (S)", arc((0.0, 1.0), 1.0, -PI, -PI / 2.0), arc((0.0, -1.0), 1.0, 0.0, PI / 2.0), band);
    show::<T>("2 line under arc, clearance in gap", ln((-1.0, 0.0), (1.0, 0.0)), arc((0.0, 1.0 + g), 1.0, PI, 2.0 * PI), band);
    show::<T>("2b line under arc, clearance zero-band", ln((-1.0, 0.0), (1.0, 0.0)), arc((0.0, 1.0 + 3e-10), 1.0, PI, 2.0 * PI), band);
    show::<T>("3 line ends 1 short of in-gap graze", ln((-1.0, 0.0), (1.0, 0.0)), arc((2.0, 1.0 + g), 1.0, PI, 2.0 * PI), band);
    show::<T>("3b line under carrier, arc is far half", ln((-1.0, 0.0), (1.0, 0.0)), arc((0.0, 1.0 + g), 1.0, 0.25, PI - 0.25), band);
    show::<T>("4 arcs away from in-gap external tangency", arc((0.0, 0.0), 1.0, PI / 2.0, 1.5 * PI), arc((2.0 + g, 0.0), 1.0, -PI / 2.0, PI / 2.0), band);
    show::<T>("4b arcs away from in-gap internal tangency", arc((0.0, 0.0), 1.0, PI / 2.0, 1.5 * PI), arc((0.5 + g, 0.0), 0.5, PI / 2.0, 1.5 * PI), band);
    show::<T>("5 near one circle (gap), spans far apart", arc((0.0, 0.0), 1.0, 0.0, 1.0), arc((g, 0.0), 1.0, 3.0, 4.0), band);
    show::<T>("5b same circle, spans far apart", arc((0.0, 0.0), 1.0, 0.0, 1.0), arc((0.0, 0.0), 1.0, 3.0, 4.0), band);
    let fine = Band::new(1e-12, 1e-11).unwrap();
    show::<T>("6 clean crossing at 1e3, eps 1e-12", ln((999.0, 1000.0), (1001.0, 1000.0)), arc((1000.0, 1000.0), 0.5, -1.0, 1.0), fine);
    show::<T>("6b clean line crossing at 1e3, eps 1e-12", ln((999.0, 1000.0), (1001.0, 1000.0)), ln((1000.0, 999.0), (1000.3, 1001.0)), fine);
    show::<T>("6c clean arc crossing at 1e3, eps 1e-12", arc((1000.0, 1000.0), 0.5, -1.0, 1.0), arc((1000.6, 1000.0), 0.5, 2.0, 4.0), fine);
    show::<T>("6d clean crossing at origin, eps 1e-12", ln((-1.0, 0.0), (1.0, 0.0)), arc((0.0, 0.0), 0.5, -1.0, 1.0), fine);
}
fn main() { all::<f64>(); all::<geom_core::Interval>(); }
