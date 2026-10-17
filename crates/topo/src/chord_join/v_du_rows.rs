//! [`chart_v_du`]'s read of a harmonic pair at its exact sup: the
//! bound on `∫ |dv|` that levers the ring's winding. Built with
//! `PCERT_RADIAL_PAIR_MUTANT` naming `topo-v-var`, that read is the
//! coefficient sum again and its row fails. And the azimuth it reads
//! is an angle by type: a free one refuses.

use core::f64::consts::TAU;

use geom_brep::{Channel, Pcurve};
use geom_core::{Bounds, Interval};

use super::*;

/// A cylinder chart image: the azimuth `0.3 + t` winding once, and the
/// axial channel `0.2 + a·cos t + b·sin t`.
fn wall_image<S: geom_core::Real>(f: impl Fn(f64) -> S, a: f64, b: f64) -> Pcurve<S> {
    Pcurve::Harmonic {
        u: Channel::Angle {
            phase: f(0.3),
            winding: 1,
        },
        v: Channel::Free {
            c: f(0.2),
            a: f(a),
            b: f(b),
            l: f(0.0),
        },
    }
}

/// **A free azimuth is no cylinder image.** The chart's azimuth is an
/// angle, so an image whose first channel is free (the shape a plane
/// mints) refuses rather than reading its slope as `du`.
#[test]
fn a_free_azimuth_is_refused() {
    let image = Pcurve::Harmonic {
        u: Channel::Free {
            c: 0.3,
            a: 0.0,
            b: 0.0,
            l: 1.0,
        },
        v: Channel::constant(0.2),
    };
    assert!(
        matches!(
            chart_v_du(FaceKey::default(), &image, (0.0, 1.0), 0.0),
            Err(SplitJoinError::SectionInvariant { .. })
        ),
        "a free azimuth refuses"
    );
}

/// `topo-v-var`: over a whole turn of `v = 0.1·cos t + 0.1·sin t`, the
/// bound on `∫ |dv|` is `sup |v′|·2π = 0.1·√2·2π`; it covers the
/// sampled `∫ |dv|` and is not over `sup |v′|·2π`, at `f64` and at
/// `Interval`. The coefficient sum reads `0.2·2π`.
#[test]
fn the_axial_travel_bound_reads_the_pair_at_its_sup() {
    let image = wall_image(|x| x, 0.1, 0.1);
    let n = 1 << 16;
    let dv = |t: f64| (-0.1 * t.sin() + 0.1 * t.cos()).abs();
    let travel = (0..n)
        .map(|i| dv((f64::from(i) + 0.5) * TAU / f64::from(n)) * TAU / f64::from(n))
        .sum::<f64>();
    let ceiling = 0.1 * 2f64.sqrt() * TAU;
    let f = Interval::from_f64;
    let face = FaceKey::default();
    let (_, at_f64) = chart_v_du(face, &image, (0.0, TAU), 0.0).unwrap().unwrap();
    let (_, at_interval) = chart_v_du(
        face,
        &wall_image(Interval::from_f64, 0.1, 0.1),
        (f(0.0), f(TAU)),
        f(0.0),
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
