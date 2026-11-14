//! **A carried turn that misses its ends still refuses**
//! (`props_chart_loop_whole`), at `f64` and at the interval scalar. A rim's
//! or a parallel's turn about the axis is its stored turn carried there,
//! and its miss is read in its own frame ([`EdgeTurn::carried`]): the
//! carry cancels, so that miss is the stored turn's against the edge's
//! ends about its centre. A door's rim has its stored turn as its own
//! parameter span and its ends evaluated at that span, so its miss is
//! the tilt the parse admits squared, and no door builds a loop this
//! reading refuses. The stored turn reaches the loop apart from the edge,
//! though (the cylinder's parsed rims, the torus's pieces), and that seam
//! is where these rows put a turn `2Kε` past the slack off its edge's
//! ends: read as [`EdgeTurn::exact`], or with the miss read off anything
//! but the turn that went into the sweep, the loop measures.

use core::f64::consts::TAU;

use geom::Curve3;
use geom_core::{Band, Decide, Point3, Tol, Vec3};

use super::closure::{VERTEX_SLACK, closed_form_windings};
use super::{TorusPiece, TorusStep, cylinder_windings, torus_chart_loop};
use crate::props::{LoopEdge, PropsError};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// How far a turn is off its ends, metered at a radius of `1`: past the
/// loop's slack by `2Kε`.
fn past(band: Band) -> f64 {
    VERTEX_SLACK * band.zero() + 2.0 * band.escalate()
}

fn p<T: Decide>(x: f64, y: f64, z: f64) -> Point3<T> {
    Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
}

fn v<T: Decide>(x: f64, y: f64, z: f64) -> Vec3<T> {
    Vec3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
}

/// A whole circle of `radius` about the `z` axis at height `z`, walked
/// up (`+u`) or down.
fn whole<T: Decide>(radius: f64, z: f64, up: bool) -> LoopEdge<T> {
    LoopEdge::hand_built(
        Curve3::circle(
            p(0.0, 0.0, z),
            v(0.0, 0.0, 1.0),
            T::from_f64(radius),
            v(1.0, 0.0, 0.0),
        ),
        T::zero(),
        T::tau(),
        up,
        0,
        0,
    )
}

/// The unit cylinder's band between whole rims at heights 0 and 1, its
/// lower rim's parsed turn `τ + off`.
fn cylinder_band<T: Decide>(off: f64, band: Band) -> Result<Vec<[i64; 1]>, PropsError> {
    let (lower, upper) = ([whole::<T>(1.0, 0.0, true)], [whole::<T>(1.0, 1.0, false)]);
    let rims = [
        (T::zero(), T::tau() + T::from_f64(off)),
        (T::one(), -T::tau()),
    ];
    cylinder_windings(
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        T::one(),
        &[&lower, &upper],
        &rims,
        &[(0, T::zero()), (1, T::zero())],
        band,
    )
}

/// The torus of radii `1` and `¼`, its band between whole parallels at
/// tube angles `∓0.3`, its lower parallel's turn `τ + off/ρ`.
fn torus_band<T: Decide>(off: f64, band: Band) -> Result<Vec<[i64; 2]>, PropsError> {
    let (major, minor) = (1.0, 0.25);
    let parallel = |v: f64, up: bool| {
        let (s, c) = v.sin_cos();
        let rho = major + minor * c;
        (whole::<T>(rho, minor * s, up), s, c, rho)
    };
    let mut loops = Vec::new();
    for (tube, up, shift) in [(-0.3, true, off), (0.3, false, 0.0)] {
        let (edge, s, c, rho) = parallel(tube, up);
        let turn = if up { TAU } else { -TAU };
        let edges = [edge];
        let pieces = [TorusPiece {
            edge: &edges[0],
            step: TorusStep::Rim {
                sin_v: T::from_f64(s),
                cos_v: T::from_f64(c),
                du: T::from_f64(turn) + T::from_f64(shift / rho),
                rho: T::from_f64(rho),
            },
            sign: T::one(),
            rising: None,
            apex_after: None,
            apex_before: false,
        }];
        loops.push(torus_chart_loop(
            p(0.0, 0.0, 0.0),
            v(0.0, 0.0, 1.0),
            T::from_f64(major),
            T::from_f64(minor),
            &edges,
            &pieces,
            band,
        )?);
    }
    closed_form_windings(&loops, band)
}

#[test]
fn a_rims_carried_turn_off_its_ends_refuses() {
    fn rows<T: Decide>(scalar: &str) {
        let band = band();
        assert_eq!(
            cylinder_band::<T>(0.0, band),
            Ok(vec![[1], [-1]]),
            "the cylinder's band at {scalar}"
        );
        assert_eq!(
            cylinder_band::<T>(past(band), band),
            Err(PropsError::LoopsWindChart),
            "the cylinder's band, a rim's turn off its ends, at {scalar}"
        );
        assert_eq!(
            torus_band::<T>(0.0, band),
            Ok(vec![[1, 0], [-1, 0]]),
            "the torus's band at {scalar}"
        );
        assert_eq!(
            torus_band::<T>(past(band), band),
            Err(PropsError::LoopsWindChart),
            "the torus's band, a parallel's turn off its ends, at {scalar}"
        );
    }
    rows::<f64>("f64");
    rows::<geom_core::Interval>("the interval scalar");
}
