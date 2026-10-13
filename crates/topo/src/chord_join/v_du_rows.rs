//! [`chart_v_du`]'s two reads of a harmonic pair at its exact sup:
//! the gate on the azimuth channel's harmonic part, and the bound on
//! `∫ |dv|` that levers the ring's winding. Built with
//! `PCERT_RADIAL_PAIR_MUTANT` naming `topo-gate` or `topo-v-var`, that
//! read is the coefficient sum again and its row fails.

use core::f64::consts::TAU;

use geom_brep::Pcurve;
use geom_core::predicate::Band;
use geom_core::{Bounds, Interval, Point2, Tol, Vec2};

use super::*;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn lift(p: &Pcurve<f64>) -> Pcurve<Interval> {
    let Pcurve::Harmonic { p0, pa, pb, pl } = *p else {
        unreachable!("the rows store harmonic images")
    };
    let f = Interval::from_f64;
    let v = |q: Vec2<f64>| Vec2::new(f(q.x), f(q.y));
    Pcurve::Harmonic {
        p0: Point2::new(f(p0.x), f(p0.y)),
        pa: v(pa),
        pb: v(pb),
        pl: v(pl),
    }
}

/// `topo-gate`: an image whose azimuth channel carries a harmonic pair
/// `(k, k)` with `k·r = 0.6·zero` stands `0.85·zero` off a linear
/// azimuth at the radius, inside the band, and reads `v du`; the
/// coefficient sum reads `1.2·zero` and escalates.
#[test]
fn a_harmonic_azimuth_inside_the_band_reads_its_sup() {
    let r = 0.5;
    let k = 0.6 * band().zero() / r;
    let image = Pcurve::Harmonic {
        p0: Point2::new(0.3, 0.2),
        pa: Vec2::new(k, 0.0),
        pb: Vec2::new(k, 0.0),
        pl: Vec2::new(1.0, 0.0),
    };
    let f = Interval::from_f64;
    let face = FaceKey::default();
    assert!(
        matches!(
            chart_v_du(face, &image, (0.0, 1.0), 0.0, r, band()),
            Ok(Some(_))
        ),
        "f64: the harmonic azimuth inside the band reads v du"
    );
    assert!(
        matches!(
            chart_v_du(face, &lift(&image), (f(0.0), f(1.0)), f(0.0), f(r), band()),
            Ok(Some(_))
        ),
        "Interval: the harmonic azimuth inside the band reads v du"
    );
}

/// `topo-v-var`: over a whole turn of `v = 0.1·cos t + 0.1·sin t`, the
/// bound on `∫ |dv|` is `sup |v′|·2π = 0.1·√2·2π`; it covers the
/// sampled `∫ |dv|` and is not over `sup |v′|·2π`, at `f64` and at
/// `Interval`. The coefficient sum reads `0.2·2π`.
#[test]
fn the_axial_travel_bound_reads_the_pair_at_its_sup() {
    let image = Pcurve::Harmonic {
        p0: Point2::new(0.3, 0.2),
        pa: Vec2::new(0.0, 0.1),
        pb: Vec2::new(0.0, 0.1),
        pl: Vec2::new(1.0, 0.0),
    };
    let n = 1 << 16;
    let dv = |t: f64| (-0.1 * t.sin() + 0.1 * t.cos()).abs();
    let travel = (0..n)
        .map(|i| dv((f64::from(i) + 0.5) * TAU / f64::from(n)) * TAU / f64::from(n))
        .sum::<f64>();
    let ceiling = 0.1 * 2f64.sqrt() * TAU;
    let f = Interval::from_f64;
    let face = FaceKey::default();
    let (_, at_f64) = chart_v_du(face, &image, (0.0, TAU), 0.0, 0.5, band())
        .unwrap()
        .unwrap();
    let (_, at_interval) = chart_v_du(
        face,
        &lift(&image),
        (f(0.0), f(TAU)),
        f(0.0),
        f(0.5),
        band(),
    )
    .unwrap()
    .unwrap();
    let slack = 1e-12;
    for (scalar, lo, hi) in [
        ("f64", at_f64, at_f64),
        ("Interval", at_interval.lo(), at_interval.hi()),
    ] {
        assert!(
            hi >= travel && lo <= ceiling + slack,
            "{scalar}: the bound [{lo}, {hi}] covers ∫|dv| = {travel} and is not over \
             sup|v′|·2π = {ceiling}"
        );
    }
}
