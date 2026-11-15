//! Reviewer probe (rectangle runs): every case's finish result and
//! certified volume bracket printed as bits, for a head-against-base diff.
//! Prints only; asserts nothing. Run with `--ignored --nocapture` on both
//! trees and diff the `RRDIFF` lines.

use geom_core::{Bounds, Decide, Interval, Point2, Point3, Real, Tol};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use topo::construction::Construct;
use topo::{Body, Pcurve};

fn tol() -> Tol {
    Tol::witness()
}

fn quad<T: Real>(h: f64, s: f64) -> Vec<ProfileLoop<T>> {
    let c = |x: f64, y: f64| {
        (
            Point2::new(T::from_f64(x * s), T::from_f64(y * s)),
            T::zero(),
        )
    };
    vec![bulge_loop(vec![
        c(-1.0, -1.0),
        c(1.0, -1.0),
        c(h, 1.0),
        c(-1.0, 1.0),
    ])]
}

fn circle<T: Real>(r: f64) -> Vec<ProfileLoop<T>> {
    vec![bulge_loop(vec![
        (Point2::new(T::from_f64(r), T::zero()), T::one()),
        (Point2::new(T::from_f64(-r), T::zero()), T::one()),
    ])]
}

fn places<T: Real>(n: usize) -> Vec<SketchPlane<T>> {
    (0..n)
        .map(|i| {
            SketchPlane::xy_at(Point3::new(
                T::zero(),
                T::zero(),
                T::from_f64(i as f64 * 0.7),
            ))
        })
        .collect()
}

fn report<T: Decide + Bounds + geom_core::CertifiedBounds + topo::AtRestPolicy>(
    name: &str,
    body: Body<T>,
) {
    match topo::finish(body, tol()) {
        Err(e) => eprintln!("RRDIFF {name} {} finish-refused {:?}", T::NAME, e),
        Ok(f) => {
            match topo::mass_properties(&f, tol(), topo::test_support::pinned_resolution(tol())) {
                Err(e) => eprintln!("RRDIFF {name} {} measure-refused {e:?}", T::NAME),
                Ok(m) => {
                    let m = m.enclosure;
                    eprintln!(
                        "RRDIFF {name} {} vol [{:e}, {:e}] pad {:e} area [{:e}, {:e}] apad {:e}",
                        T::NAME,
                        m.volume.lo(),
                        m.volume.hi(),
                        m.volume_pad,
                        m.surface_area.lo(),
                        m.surface_area.hi(),
                        m.area_pad
                    );
                }
            }
        }
    }
}

fn split_rims<T: Decide + Bounds + topo::AtRestPolicy>(
    body: &mut Body<T>,
    frac: f64,
    count: usize,
) {
    let rims: Vec<_> = body
        .pcurves()
        .filter_map(|(he, c)| match c.pcurve() {
            Pcurve::IsoLine { pl, .. } if (pl.y.lo(), pl.y.hi()) == (0.0, 0.0) => Some(he),
            Pcurve::IsoArc { .. } => Some(he),
            _ => None,
        })
        .collect();
    let mut done = std::collections::BTreeSet::new();
    for he in rims {
        if done.len() >= count {
            break;
        }
        let Some(h) = body.get_half_edge(he) else {
            continue;
        };
        let edge = h.edge;
        if !done.insert(format!("{edge:?}")) {
            continue;
        }
        let Some(c) = body.pcurve(he) else { continue };
        let (t0, t1) = c.params();
        let _ = body.split_edge(edge, t0 + (t1 - t0) * T::from_f64(frac), tol());
    }
}

fn cases<T: Decide + Bounds + geom_core::CertifiedBounds + topo::AtRestPolicy>() {
    for (k, (h, s, n, deg)) in [
        (1.2, 1.0, 3, 2),
        (0.7, 1.0, 3, 2),
        (2.5, 0.3, 4, 3),
        (1.0, 1.0, 2, 1),
        (1.9, 7.0, 3, 2),
    ]
    .into_iter()
    .enumerate()
    {
        let secs: Vec<_> = (0..n).map(|i| quad::<T>(h + 0.1 * i as f64, s)).collect();
        let make = || {
            sweep::test_support::loft_body_construction::<T>(&secs, &places(n), deg, tol())
                .map(|l| l.body)
        };
        match make() {
            Err(e) => eprintln!("RRDIFF quad{k} {} build-refused {e:?}", T::NAME),
            Ok(b) => {
                report(&format!("quad{k}"), b.clone());
                for (j, (frac, count)) in [(1.0 / 3.0, 1), (0.37, 3), (1.0 / 7.0, 8)]
                    .into_iter()
                    .enumerate()
                {
                    let mut bb = b.clone();
                    split_rims(&mut bb, frac, count);
                    report(&format!("quad{k}-split{j}"), bb.clone());
                    if topo::mint_pcurves(&mut bb, tol()).is_ok() {
                        report(&format!("quad{k}-split{j}-reminted"), bb);
                    }
                }
            }
        }
    }
    for (k, rs) in [
        vec![1.0, 1.3, 1.0],
        vec![0.5, 0.5],
        vec![2.0, 1.0, 1.5, 0.8],
    ]
    .into_iter()
    .enumerate()
    {
        let secs: Vec<_> = rs.iter().map(|r| circle::<T>(*r)).collect();
        let n = rs.len();
        match sweep::test_support::loft_body_construction::<T>(
            &secs,
            &places(n),
            (n - 1).min(2),
            tol(),
        ) {
            Err(e) => eprintln!("RRDIFF circ{k} {} build-refused {e:?}", T::NAME),
            Ok(l) => {
                report(&format!("circ{k}"), l.body.clone());
                let mut bb = l.body;
                split_rims(&mut bb, 0.41, 4);
                report(&format!("circ{k}-split"), bb);
            }
        }
    }
}

#[test]
#[ignore = "reviewer probe: prints a head-against-base differential, asserts nothing"]
fn reviewer_rectangle_runs_differential() {
    cases::<f64>();
    cases::<Interval>();
}
