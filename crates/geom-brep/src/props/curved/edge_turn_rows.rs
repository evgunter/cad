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

/// The frame of a far generic pose: origin `(30, −110, 200)`, axis
/// `(1, 1, 1)/√3`, and two axes across it.
fn far_frame<T: Decide>() -> (Point3<T>, Vec3<T>, Vec3<T>, Vec3<T>) {
    let (s3, s2, s6) = (
        3.0_f64.sqrt().recip(),
        2.0_f64.sqrt().recip(),
        6.0_f64.sqrt().recip(),
    );
    (
        p(30.0, -110.0, 200.0),
        v(s3, s3, s3),
        v(s2, -s2, 0.0),
        v(s6, s6, -2.0 * s6),
    )
}

/// The patch `u ∈ [0, span]`, `z ∈ [0, 1]` of the far cylinder of radius
/// `radius`, walked with the material on its left or (`reversed`) the
/// other way round; its lower rim's parsed turn is `span + off/radius`.
fn far_patch<T: Decide>(
    radius: f64,
    span: f64,
    off: f64,
    reversed: bool,
    band: Band,
) -> Result<Vec<[i64; 1]>, PropsError> {
    let (o, n, x, y) = far_frame::<T>();
    let r = T::from_f64(radius);
    let rim = |z: f64, forward: bool| {
        LoopEdge::hand_built(
            Curve3::circle(o + n * T::from_f64(z), n, r, x),
            T::zero(),
            T::from_f64(span),
            forward,
            0,
            0,
        )
    };
    let ruling = |u: f64, forward: bool| {
        let (s, c) = u.sin_cos();
        LoopEdge::hand_built(
            Curve3::line(o + (x * T::from_f64(c) + y * T::from_f64(s)) * r, n),
            T::zero(),
            T::one(),
            forward,
            0,
            0,
        )
    };
    let d = T::from_f64(span + off / radius);
    let (edges, rims) = if reversed {
        (
            [
                ruling(0.0, true),
                rim(1.0, true),
                ruling(span, false),
                rim(0.0, false),
            ],
            [(T::one(), T::from_f64(span)), (T::zero(), -d)],
        )
    } else {
        (
            [
                rim(0.0, true),
                ruling(span, true),
                rim(1.0, false),
                ruling(0.0, false),
            ],
            [(T::zero(), d), (T::one(), -T::from_f64(span))],
        )
    };
    cylinder_windings(o, n, r, &[&edges], &rims, &[(0, T::from_f64(2.0))], band)
}

/// **A far partial rim's carried turn off its ends refuses**, walked
/// either way round, at `f64` and at the interval scalar: an arc's own
/// arms are not `(r², 0)`, so a carried miss read off the arms reversed,
/// or off a turn other than the one swept, measures one of these.
#[test]
fn a_far_partial_rims_carried_turn_off_its_ends_refuses() {
    fn rows<T: Decide>(scalar: &str) {
        let band = band();
        for reversed in [false, true] {
            for (radius, span) in [(213.0, 1.0), (213.0, 4.0), (50.0, 0.3)] {
                let what = format!("r {radius} span {span} reversed {reversed} at {scalar}");
                assert_eq!(
                    far_patch::<T>(radius, span, 0.0, reversed, band),
                    Ok(vec![[0]]),
                    "the patch, {what}"
                );
                for sign in [1.0, -1.0] {
                    assert_eq!(
                        far_patch::<T>(radius, span, sign * past(band), reversed, band),
                        Err(PropsError::LoopsWindChart),
                        "a rim's turn off its ends ({sign}), {what}"
                    );
                }
            }
        }
    }
    rows::<f64>("f64");
    rows::<geom_core::Interval>("the interval scalar");
}
