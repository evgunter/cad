//! **The ellipse × torus root door**: the certified crossings of an
//! ELLIPSE carrier with a torus. Like the conic × quadric door
//! ([`super::conic_quadric`]) it owns no root machinery: it reads the
//! residual's harmonics from their one home and hands them to the shared
//! certified subdivision ([`super::circle_roots`]), whose answer it gives.
//!
//! # Against a torus, the residual is of degree four
//!
//! The torus's implicit `F = (S + R² − r²)² − 4R²(S − h²)` has
//! `S = |C − c|²` of degree two along an ellipse (on a circle its second
//! harmonic vanishes) and `h = (C − c)·â` of degree one, so `F` is a
//! trigonometric polynomial of degree FOUR — an octic in the tangent
//! half-angle, up to eight crossings per turn
//! ([`geom_brep::ConicTorusHarmonics`]). No half-angle ladder runs: its
//! octic has no certified solver, and the quartic ladder's only part in
//! the degree-2 doors is its escalations. The shared certified
//! subdivision answers alone ([`super::circle_roots::certified_subdivision`],
//! at degree four), under the `bool_ellipse_torus_sub_*` rows, with the
//! circle × torus door's metering: `F`'s rounding charged against its
//! term bound, read in residual metres through the floor
//! `2r(R² − r²)` on `|F|` per metre of residual. Its answers are
//! `Certified`, `Miss` or `Uncertain`, never an escalation.
//!
//! **Every root's slack is charged from the computation's own running
//! bound** (`bool_ellipse_torus_root_slack`): the residual at the root,
//! evaluated with a first-order bound on its rounding
//! ([`geom_brep::conic_torus_residual`]), plus its own magnitude, over
//! the residual's least slope near the root — `F`'s least slope over the
//! smaller of the carrier's and the surface's ceilings on `|F|` per
//! metre of residual — at the carrier's top speed, must be definitely
//! inside the band ([`super::circle_roots::RootSlack`]). A shallow
//! crossing whose root the representation cannot place refuses as
//! `Uncertain` there.
//!
//! A piece is read CLEAR in residual metres through the surface's
//! ceiling `2r((2R + 2r)² + 3r²)`, capped at `r/2` — the bound
//! `|res| ≥ min(r/2, |F| / f_surface)` holds everywhere — so a carrier
//! within the band of the torus never reads clear.

use geom_core::{Band, Decide};

use super::BooleanError;
use super::circle_roots::{
    CircleRoots, RootSlack, SubdivisionFrame, SubdivisionRows, TrigPoly, certified_subdivision,
    rounding_charge,
};
/// The torus arm's subdivision rows (module docs).
const TORUS_ROWS: SubdivisionRows = SubdivisionRows {
    clear: "bool_ellipse_torus_sub_clear",
    monotone: "bool_ellipse_torus_sub_monotone",
    side: "bool_ellipse_torus_sub_side",
    width: "bool_ellipse_torus_sub_width",
};

/// The certified crossings of the `carrier` ellipse with the `surface`
/// torus, reported within `π` of the midpoint of `[t0, t1]` (module
/// docs).
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] when `carrier` is not an
/// ellipse or `surface` not a torus — the caller dispatched on those
/// kinds, so a mismatch is a desync, never an answer. The subdivision
/// answers `Certified`, `Miss` or `Uncertain` and never escalates.
pub(super) fn ellipse_torus_roots<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<CircleRoots<T>, BooleanError> {
    let (
        geom::Curve3::Ellipse { .. },
        &geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        },
    ) = (carrier, surface)
    else {
        return Err(BooleanError::ClassificationInvariant {
            what: "the ellipse × torus root door was handed a carrier that is not an ellipse \
                   or a surface that is not a torus",
        });
    };
    let conic = geom_brep::Conic::of(carrier).ok_or(BooleanError::ClassificationInvariant {
        what: "an ellipse carrier has no conic frame",
    })?;
    torus_walk(
        &conic,
        (center, axis, major_radius, minor_radius),
        (t0, t1),
        surface,
        Some(TORUS_ROOT_SLACK),
        band,
    )
}

/// The torus arm's root-slack row.
const TORUS_ROOT_SLACK: &str = "bool_ellipse_torus_root_slack";

/// [`ellipse_torus_roots`]' walk, its root-slack meter under `slack` (`None`
/// only in this module's rows, which show the meter is what decides).
/// The meter's ceiling on `|F| / |res|` is the smaller of the carrier's
/// and the surface's ([`geom_brep::ConicTorusHarmonics`]): a root it
/// meters reads ON the surface.
fn torus_walk<T: Decide>(
    conic: &geom_brep::Conic<T>,
    torus: (geom_core::Point3<T>, geom_core::Vec3<T>, T, T),
    (t0, t1): (T, T),
    surface: &geom::Surface<T>,
    slack: Option<&'static str>,
    band: Band,
) -> Result<CircleRoots<T>, BooleanError> {
    let (center, axis, major_radius, minor_radius) = torus;
    let h = geom_brep::conic_torus_harmonics(conic, center, axis, major_radius, minor_radius);
    let placed = |theta: T| {
        geom_brep::conic_torus_residual(conic, center, axis, major_radius, minor_radius, theta)
    };
    let meter = slack.map(|row| RootSlack {
        row,
        residual: &placed,
        f_per_metre_hi: h.f_per_metre_hi.min(h.f_per_metre_surface),
    });
    certified_subdivision(
        &TrigPoly {
            cos: h.cos,
            sin: h.sin,
            degree: 4,
        },
        &|theta| geom_brep::implicit_residual(surface, conic.point(theta)),
        &SubdivisionFrame {
            t0,
            t1,
            speed_hi: conic.speed_hi(),
            noise: rounding_charge(h.terms),
            f_per_metre: h.f_per_metre_lo,
            f_per_metre_hi: h.f_per_metre_surface,
            residual_reach: Some(minor_radius / T::from_f64(2.0)),
        },
        &TORUS_ROWS,
        meter.as_ref(),
        band,
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! Each pose is checked against the geometry, not the door's own
    //! algebra: a certified root must put the carrier ON the surface, and
    //! its count must be the count of sign changes of the true distance
    //! along the carrier, each bisected to the bit.

    use core::f64::consts::{PI, TAU};

    use super::*;
    use crate::boolean::conic_oracle::distance;
    use geom_core::{Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The ellipse `c + a·û cos θ + b·v̂ sin θ` on the unit normal `n`.
    fn ellipse(c: [f64; 3], n: [f64; 3], u: [f64; 3], a: f64, b: f64) -> geom::Curve3<f64> {
        let n = Vec3::from_array(n).normalize();
        let u = Vec3::from_array(u);
        geom::Curve3::Ellipse {
            center: Point3::from_array(c),
            axis: n,
            major: a,
            minor: b,
            u_ref: (u - n * u.dot(n)).normalize(),
        }
    }

    fn torus(c: [f64; 3], axis: [f64; 3], big: f64, small: f64) -> geom::Surface<f64> {
        crate::boolean::conic_oracle::torus(
            Point3::from_array(c),
            Vec3::from_array(axis),
            big,
            small,
        )
    }

    /// The sign changes of the true distance on `[t0, t1]`.
    fn oracle(e: &geom::Curve3<f64>, s: &geom::Surface<f64>, t0: f64, t1: f64) -> Vec<f64> {
        let f = |t: f64| distance(s, e.eval(t));
        let steps = 20_000;
        let mut out = Vec::new();
        for k in 0..steps {
            let at = |k: u32| t0 + (t1 - t0) * f64::from(k) / f64::from(steps);
            let (mut a, mut b) = (at(k), at(k + 1));
            if f(a).signum() == f(b).signum() {
                continue;
            }
            for _ in 0..80 {
                let m = (a + b) / 2.0;
                if f(m).signum() == f(a).signum() {
                    a = m;
                } else {
                    b = m;
                }
            }
            out.push((a + b) / 2.0);
        }
        out
    }

    fn door(e: &geom::Curve3<f64>, s: &geom::Surface<f64>, t0: f64, t1: f64) -> CircleRoots<f64> {
        ellipse_torus_roots(e, t0, t1, s, band()).unwrap()
    }

    /// Certified roots matching the oracle in number and place, every
    /// one on the surface and within `π` of the arc's midpoint.
    fn assert_matches_oracle(
        label: &str,
        e: &geom::Curve3<f64>,
        s: &geom::Surface<f64>,
        t0: f64,
        want: usize,
    ) {
        let t1 = t0 + 1.0;
        let CircleRoots::Certified { count, thetas } = door(e, s, t0, t1) else {
            panic!("{label}: certified roots, got {:?}", door(e, s, t0, t1));
        };
        assert_eq!(count, want, "{label}: the certified count");
        let mid = (t0 + t1) / 2.0;
        let mut got = thetas[..count].to_vec();
        for &t in &got {
            assert!((t - mid).abs() <= PI, "{label}: {t} within π of {mid}");
            let off = distance(s, e.eval(t)).abs();
            assert!(off < 1e-12, "{label}: root {t} lies {off} off the surface");
        }
        got.sort_by(f64::total_cmp);
        let truth = oracle(e, s, mid - PI, mid + PI);
        assert_eq!(got.len(), truth.len(), "{label}: {got:?} vs {truth:?}");
        for (a, b) in got.iter().zip(&truth) {
            assert!(
                (a - b).abs() < 1e-9,
                "{label}: root {a} vs the oracle's {b}"
            );
        }
    }

    /// **Against a torus, up to eight crossings, certified wherever the
    /// band can place them.** The residual is of degree four: an ellipse
    /// in the torus's equatorial plane, centred on its axis, with
    /// semi-axes `0.53` and `0.5` against a tube of radius `0.01` about
    /// the core circle of radius `0.515`, enters and leaves the tube in
    /// each quadrant — eight crossings, which no degree-2 residual has.
    /// Moved off the axis it crosses six or four times; tilted and offset,
    /// two or four. At several arcs, against the true distance:
    ///
    /// - the answer is never a `Miss`, and a certified one has the true
    ///   count, every root on the torus and at its true crossing;
    /// - it MUST be certified where the band resolves the pose: where the
    ///   door's own noise in metres of residual (its harmonics' rounding
    ///   charge over the floor on `|F|/|res|`), carried along the arc at
    ///   the least true crossing slope, is two orders inside the zero
    ///   band. Past that (the off-axis pose at ε 1e-12, whose noise places
    ///   a root only to ~5e-12 m) an `Uncertain` is the honest answer.
    #[test]
    fn torus_crossings_match_the_true_distance() {
        let flat = ellipse([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 0.53, 0.5);
        let tilted = ellipse([0.1, 0.2, 0.0], [0.3, -0.2, 1.0], [1.0, 1.0, 0.0], 0.8, 0.5);
        let poses = [
            (
                "flat × ring",
                &flat,
                torus([0.0; 3], [0.0, 0.0, 1.0], 0.515, 0.01),
                8,
            ),
            (
                "flat × ring, off the axis",
                &flat,
                torus([0.012, 0.0, 0.0], [0.0, 0.0, 1.0], 0.515, 0.01),
                6,
            ),
            (
                "flat × ring, raised",
                &flat,
                torus([0.0, 0.0, 0.007], [0.0, 0.0, 1.0], 0.515, 0.01),
                8,
            ),
            (
                "tilted × a torus through it",
                &tilted,
                torus([0.9, 0.2, 0.0], [0.0, 1.0, 0.2], 0.3, 0.1),
                2,
            ),
            (
                "tilted × a fat torus about it",
                &tilted,
                torus([0.1, 0.2, 0.0], [0.0, 0.0, 1.0], 0.6, 0.12),
                4,
            ),
        ];
        let band = band();
        for t0 in [0.0, 1.7, -2.9] {
            for (label, e, s, want) in &poses {
                let mid = t0 + 0.5;
                let truth = oracle(e, s, mid - PI, mid + PI);
                assert_eq!(truth.len(), *want, "{label}: the pose's own crossings");
                // The door's noise in residual metres, and the least slope
                // of the true distance per metre of arc at a crossing.
                let geom::Surface::Torus {
                    center,
                    axis,
                    major_radius,
                    minor_radius,
                    ..
                } = *s
                else {
                    unreachable!("a torus")
                };
                let conic = geom_brep::Conic::of(e).unwrap();
                let h = geom_brep::conic_torus_harmonics(
                    &conic,
                    center,
                    axis,
                    major_radius,
                    minor_radius,
                );
                let noise = rounding_charge(h.terms) / h.f_per_metre_lo;
                let slope = truth
                    .iter()
                    .map(|&r| {
                        let d = 1e-7;
                        let rise = distance(s, e.eval(r + d)) - distance(s, e.eval(r - d));
                        (rise / (2.0 * d * e.deriv(r).norm())).abs()
                    })
                    .fold(f64::INFINITY, f64::min);
                let resolvable = 100.0 * noise / slope <= band.zero();
                match ellipse_torus_roots(e, t0, t0 + 1.0, s, band).unwrap() {
                    CircleRoots::Certified { .. } => {
                        assert_matches_oracle(label, e, s, t0, *want);
                    }
                    CircleRoots::Uncertain if !resolvable => {}
                    other => panic!(
                        "{label} at {t0}: {other:?} where the band places a root to {:e} m",
                        noise / slope
                    ),
                }
            }
        }
    }

    /// An ellipse clear of a torus — inside its tube, through its hole,
    /// beside it — is a certified `Miss`.
    #[test]
    fn a_carrier_clear_of_a_torus_is_a_miss() {
        let flat = ellipse([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 0.53, 0.5);
        for (label, s) in [
            (
                "inside the tube",
                torus([0.0; 3], [0.0, 0.0, 1.0], 0.515, 0.03),
            ),
            (
                "through the hole",
                torus([0.0, 0.0, 0.2], [0.0, 0.0, 1.0], 0.25, 0.1),
            ),
            (
                "beside it",
                torus([2.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.4, 0.2),
            ),
            (
                "round it",
                torus([0.0, 0.0, 0.1], [0.1, 0.0, 1.0], 1.0, 0.3),
            ),
        ] {
            assert!(
                matches!(door(&flat, &s, 0.0, TAU), CircleRoots::Miss),
                "{label}: got {:?}",
                door(&flat, &s, 0.0, TAU)
            );
            assert!(
                oracle(&flat, &s, 0.0, TAU).is_empty(),
                "{label}: the oracle agrees"
            );
        }
    }

    /// A carrier that is not an ellipse, or a surface that is not a
    /// torus, is a dispatch desync.
    #[test]
    fn a_wrong_kind_is_a_desync() {
        let circle = geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let ring = torus([0.0; 3], [0.0, 0.0, 1.0], 0.515, 0.01);
        let e = ellipse([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 1.0, 0.5);
        let ball = geom::Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 0.5,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        for (carrier, surface) in [(&circle, &ring), (&e, &ball)] {
            let got = ellipse_torus_roots(carrier, 0.0, 1.0, surface, band());
            assert!(
                matches!(got, Err(BooleanError::ClassificationInvariant { .. })),
                "a wrong kind refuses as a kernel invariant, got {got:?}"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torus_rows {
    //! The torus arm against the geometry: random ellipses against random
    //! tori in four families — posed through the carrier (crossings),
    //! grazing it by a few bands either side of zero (near-tangent), the
    //! same carried a kilometre out (far-out), and at millimetre scale.

    use core::f64::consts::{PI, TAU};
    use std::fmt::Write as _;

    use super::*;
    use crate::boolean::conic_oracle::{distance, unit};
    use geom_core::{Point3, Vec3};
    use test_utils::fuzz;

    /// The families a pose is drawn from.
    const FAMILIES: [&str; 4] = ["crossing", "graze", "far-out", "millimetre"];

    /// One pose of family `family` at band zero `eps`: the carrier, the
    /// torus and the arc.
    fn pose(
        rng: &mut fuzz::Rng,
        family: usize,
        eps: f64,
    ) -> (geom::Curve3<f64>, geom::Surface<f64>, f64, f64) {
        let scale = if family == 3 { 1e-3 } else { 1.0 };
        let out = if family == 2 {
            Vec3::new(
                rng.range(-1e3, 1e3),
                rng.range(-1e3, 1e3),
                rng.range(-1e3, 1e3),
            )
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        };
        let n = unit(rng);
        let u = unit(rng);
        let big = scale * rng.range(0.1, 2.0);
        let small = big / rng.range(1.0, 20.0);
        let (major, minor) = if rng.below(2) == 0 {
            (big, small)
        } else {
            (small, big)
        };
        let center = Point3::new(0.0, 0.0, 0.0) + out;
        let e = geom::Curve3::Ellipse {
            center,
            axis: n,
            major,
            minor,
            u_ref: (u - n * u.dot(n)).normalize(),
        };
        let theta = rng.range(0.0, TAU);
        let p = e.eval(theta);
        let tangent = e.deriv(theta).normalize();
        let tube = scale * rng.range(0.02, 0.5);
        let ring = tube * rng.range(1.2, 6.0);
        // A direction off the carrier at `p`, square to its tangent: the
        // tube's near point lies along it, `gap` from `p` — a few bands
        // either side of zero on a graze, a crossing's depth otherwise.
        let off = {
            let w = unit(rng);
            (w - tangent * w.dot(tangent)).normalize()
        };
        let gap = if family == 1 {
            eps * rng.range(-40.0, 40.0)
        } else {
            -tube * rng.range(0.0, 1.0)
        };
        let core = p + off * (tube + gap);
        // The core circle passes through `core` square to `off` there.
        let along = {
            let w = unit(rng);
            (w - off * w.dot(off)).normalize()
        };
        let axis = {
            let w = unit(rng);
            let w = w - along * w.dot(along);
            w.normalize()
        };
        let to_hub = axis.cross(along).normalize();
        let s = crate::boolean::conic_oracle::torus(core + to_hub * ring, axis, ring, tube);
        let t0 = rng.range(0.0, TAU);
        let t1 = t0 + rng.range(0.1, TAU);
        (e, s, t0, t1)
    }

    /// The true crossings of `e` with the torus `s` round the turn about
    /// `mid`, in `f64` on the true distance: every sign change of a dense
    /// sampling, and every sampled extremum of `|d|` whose refined value
    /// crosses zero (a graze pair inside one step), each bisected. Also the
    /// least `|d|` seen, extrema included.
    fn true_crossings(e: &geom::Curve3<f64>, s: &geom::Surface<f64>, mid: f64) -> (Vec<f64>, f64) {
        let f = |t: f64| distance(s, e.eval(t));
        let n = 8_000_u32;
        let ts: Vec<f64> = (0..=n)
            .map(|k| mid - PI + TAU * f64::from(k) / f64::from(n))
            .collect();
        let fs: Vec<f64> = ts.iter().map(|&t| f(t)).collect();
        let bisect = |mut lo: f64, mut hi: f64| {
            let below = f(lo) < 0.0;
            for _ in 0..80 {
                let m = 0.5 * (lo + hi);
                if (f(m) < 0.0) == below {
                    lo = m;
                } else {
                    hi = m;
                }
            }
            0.5 * (lo + hi)
        };
        let mut least = fs.iter().fold(f64::INFINITY, |m, d| m.min(d.abs()));
        let mut roots = Vec::new();
        for k in 0..fs.len() - 1 {
            if (fs[k] < 0.0) != (fs[k + 1] < 0.0) {
                roots.push(bisect(ts[k], ts[k + 1]));
            }
        }
        for k in 1..fs.len() - 1 {
            let same = (fs[k - 1] < 0.0) == (fs[k] < 0.0) && (fs[k] < 0.0) == (fs[k + 1] < 0.0);
            if !same || fs[k].abs() > fs[k - 1].abs() || fs[k].abs() > fs[k + 1].abs() {
                continue;
            }
            // Golden-section search for the extremum of the sign `f` has here.
            let sign = fs[k].signum();
            let (mut a, mut b) = (ts[k - 1], ts[k + 1]);
            let g = 0.5 * (5.0_f64.sqrt() - 1.0);
            for _ in 0..90 {
                let (c, d) = (b - g * (b - a), a + g * (b - a));
                if sign * f(c) < sign * f(d) {
                    b = d;
                } else {
                    a = c;
                }
            }
            let t = 0.5 * (a + b);
            least = least.min(f(t).abs());
            if (f(t) < 0.0) != (fs[k] < 0.0) {
                roots.push(bisect(ts[k - 1], t));
                roots.push(bisect(t, ts[k + 1]));
            }
        }
        roots.sort_by(f64::total_cmp);
        (roots, least)
    }

    /// The arc length from `t` to the nearest of `truth` along `e`, and
    /// the `f64` oracle's own uncertainty there: the distance's rounding
    /// (`64u` of the pose's scale) over its slope along the arc.
    fn arc_off(e: &geom::Curve3<f64>, s: &geom::Surface<f64>, t: f64, truth: &[f64]) -> (f64, f64) {
        let r = truth
            .iter()
            .copied()
            .min_by(|a, b| (a - t).abs().total_cmp(&(b - t).abs()))
            .expect("a true crossing");
        let speed = e.deriv(r).norm();
        let geom::Surface::Torus {
            center,
            major_radius,
            minor_radius,
            ..
        } = *s
        else {
            unreachable!("a torus")
        };
        let scale = (e.eval(r) - Point3::new(0.0, 0.0, 0.0)).norm()
            + (center - Point3::new(0.0, 0.0, 0.0)).norm()
            + major_radius
            + minor_radius;
        let h = 1e-6;
        let slope =
            ((distance(s, e.eval(r + h)) - distance(s, e.eval(r - h))) / (2.0 * h * speed)).abs();
        (
            (t - r).abs() * speed,
            64.0 * f64::EPSILON * 0.5 * scale / slope,
        )
    }

    /// **Every certified answer is exactly true of the geometry**, at
    /// three bands, over every family:
    ///
    /// - a certified count is the TRUE count ([`true_crossings`], graze
    ///   pairs inside a sampling step included);
    /// - every certified root lies on the torus (its true distance inside
    ///   the zero band) and, as arc length, within the zero band of a true
    ///   crossing, wherever the `f64` oracle resolves that crossing to a
    ///   quarter band (where it cannot, the place is not checked, and the
    ///   count of such roots is printed);
    /// - a `Miss` has no true crossing and comes no nearer than the band.
    ///
    /// The counts per answer are printed. The high-precision check of the
    /// same families, root places everywhere, is
    /// [`dump_for_the_mpmath_oracle`]; the slack meter's calibration is
    /// pinned by [`the_slack_meter_charges_every_term`].
    #[test]
    fn certified_torus_answers_hold_against_the_true_distance() {
        let mut rng = fuzz::start("ellipse_torus::certified_torus_answers_hold");
        for eps in [1e-6, 1e-9, 1e-12] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            for (family, name) in FAMILIES.iter().enumerate() {
                let (mut certified, mut misses, mut uncertain, mut unplaced) = (0, 0, 0, 0);
                for i in 0..fuzz::scaled(120) {
                    let (e, s, t0, t1) = pose(&mut rng, family, eps);
                    let mid = (t0 + t1) / 2.0;
                    let found = ellipse_torus_roots(&e, t0, t1, &s, band).unwrap_or_else(|err| {
                        panic!("ε {eps}, {name} {i}: the torus arm escalated {err:?}")
                    });
                    let (truth, least) = true_crossings(&e, &s, mid);
                    let label = format!(
                        "ε {eps}, {name} {i}: {e:?} against {s:?} on [{t0}, {t1}] — {}",
                        fuzz::replay()
                    );
                    match found {
                        CircleRoots::Certified { count, thetas } => {
                            certified += 1;
                            assert_eq!(
                                count,
                                truth.len(),
                                "{label}: certified {count} of {truth:?}"
                            );
                            for &t in &thetas[..count] {
                                let off = distance(&s, e.eval(t)).abs();
                                assert!(off <= eps, "{label}: root {t} lies {off} off the torus");
                                let (arc, resolution) = arc_off(&e, &s, t, &truth);
                                if resolution > eps / 4.0 {
                                    unplaced += 1;
                                    continue;
                                }
                                assert!(
                                    arc <= eps + resolution,
                                    "{label}: root {t} is {arc:e} m of arc from a true crossing"
                                );
                            }
                        }
                        CircleRoots::Miss => {
                            misses += 1;
                            assert!(
                                truth.is_empty() && least > eps,
                                "{label}: a Miss {least} from the torus, crossings {truth:?}"
                            );
                        }
                        CircleRoots::Uncertain => uncertain += 1,
                        other => panic!("{label}: {other:?}"),
                    }
                }
                println!(
                    "ε {eps}, {name}: {certified} certified ({unplaced} roots past the oracle's \
                     resolution), {misses} misses, {uncertain} uncertain"
                );
            }
        }
    }

    /// An ellipse centred at the origin (normal `n`, `u_ref` `u`,
    /// semi-axes `semi`), a torus (`hub`, `t_axis`, radii `R` and `r`),
    /// and an arc.
    struct Pose {
        n: [f64; 3],
        u: [f64; 3],
        semi: [f64; 2],
        hub: [f64; 3],
        t_axis: [f64; 3],
        radii: [f64; 2],
        arc: [f64; 2],
    }

    impl Pose {
        fn build(&self) -> (geom::Curve3<f64>, geom::Surface<f64>) {
            let e = geom::Curve3::Ellipse {
                center: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::from_array(self.n),
                major: self.semi[0],
                minor: self.semi[1],
                u_ref: Vec3::from_array(self.u),
            };
            let s = crate::boolean::conic_oracle::torus(
                Point3::from_array(self.hub),
                Vec3::from_array(self.t_axis),
                self.radii[0],
                self.radii[1],
            );
            (e, s)
        }

        /// The door's answer, and the walk's with no slack meter.
        fn answers(&self, band: Band) -> (CircleRoots<f64>, CircleRoots<f64>) {
            let (e, s) = self.build();
            let conic = geom_brep::Conic::of(&e).unwrap();
            let torus = (
                Point3::from_array(self.hub),
                Vec3::from_array(self.t_axis).normalize(),
                self.radii[0],
                self.radii[1],
            );
            let [t0, t1] = self.arc;
            (
                ellipse_torus_roots(&e, t0, t1, &s, band).unwrap(),
                torus_walk(&conic, torus, (t0, t1), &s, None, band).unwrap(),
            )
        }
    }

    /// **A root the representation cannot place is refused, not
    /// certified.** Two grazes the fuzz drew at ε = 1e-12, each crossing
    /// the torus twice within 1e-4 rad: there the residual's slope along
    /// the carrier is about 1e-5, so its `f64` rounding moves the bisected
    /// root by tens of zero bands. The true roots are the 40-digit ones
    /// `scripts/oracles/ellipse_torus_mpmath.py` finds from the stored
    /// values. The door answers `Uncertain` for both, and never a `Miss`
    /// (they cross); any root it certified would have to lie within the
    /// zero band, as arc length, of a true one. On the first the meter is
    /// what decides: with no slack meter the walk certifies all four of its
    /// roots, the graze pair tens of bands off, which the row checks too so
    /// it cannot pass on a pose the meter does not decide. On the second
    /// the walk itself declines: its clear margin reads the CEILING on
    /// `|F|/|res|`, and the graze's pieces stop clearing before a root
    /// reaches the meter.
    #[test]
    fn a_root_the_band_cannot_place_is_not_certified() {
        let band = Band::new(1e-12, 1e-11).unwrap();
        let cases = [
            (
                Pose {
                    n: [
                        0.203_297_924_652_609_5,
                        -0.849_258_597_911_928_6,
                        -0.487_267_675_620_502_5,
                    ],
                    u: [
                        -0.947_599_164_518_603_8,
                        -0.295_910_810_043_571_84,
                        0.120_385_281_089_513_89,
                    ],
                    semi: [0.468_167_765_509_919_23, 0.029_302_888_415_728_03],
                    hub: [
                        0.555_302_572_861_934_7,
                        -0.432_052_166_574_670_16,
                        -1.582_439_375_116_944_3,
                    ],
                    t_axis: [
                        0.631_422_079_152_858_2,
                        -0.720_417_358_371_581_6,
                        0.286_888_458_665_025_5,
                    ],
                    radii: [1.962_544_801_084_216_4, 0.349_075_956_625_705_34],
                    arc: [1.226_257_671_700_713_4, 6.066_069_630_592_406_4],
                },
                [
                    1.655_031_910_583_527_7,
                    1.655_125_629_734_536_3,
                    2.047_503_220_344_307_7,
                    5.772_456_607_894_865,
                ],
            ),
            (
                Pose {
                    n: [
                        0.585_210_592_572_929_7,
                        0.025_848_412_147_504_423,
                        -0.810_469_260_323_852_8,
                    ],
                    u: [
                        0.351_512_957_155_989_74,
                        0.892_611_310_206_308_1,
                        0.282_282_996_022_129_2,
                    ],
                    semi: [1.622_886_978_643_085_3, 0.116_744_224_570_161_3],
                    hub: [
                        -0.396_557_758_150_929_8,
                        -0.550_975_344_440_982_3,
                        -1.626_369_091_986_125_1,
                    ],
                    t_axis: [
                        0.970_926_661_587_580_1,
                        0.209_414_540_512_199_7,
                        0.115_961_062_604_914_39,
                    ],
                    radii: [1.632_177_296_965_134, 0.289_732_944_160_409_07],
                    arc: [2.866_752_917_421_971, 4.963_815_025_747_772],
                },
                [
                    2.204_785_365_190_525_8,
                    3.334_730_708_216_815,
                    3.689_491_881_643_564_1,
                    3.689_517_963_451_27,
                ],
            ),
        ];
        let worst = |pose: &Pose, roots: &[f64], truth: &[f64; 4]| {
            let [major, minor] = pose.semi;
            roots
                .iter()
                .map(|&t| {
                    truth
                        .iter()
                        .map(|&r| {
                            let (sr, cr) = r.sin_cos();
                            (t - r).abs() * (major * sr).hypot(minor * cr)
                        })
                        .fold(f64::INFINITY, f64::min)
                })
                .fold(0.0, f64::max)
        };
        for (k, (pose, truth)) in cases.iter().enumerate() {
            let (door, bare) = pose.answers(band);
            let meter_decides = k == 0;
            match door {
                CircleRoots::Uncertain => {}
                CircleRoots::Certified { count, thetas } => {
                    let off = worst(pose, &thetas[..count], truth);
                    assert!(off <= band.zero(), "case {k}: a root {off:e} m of arc off");
                }
                other => panic!("case {k}: {other:?} for a carrier crossing four times"),
            }
            if !meter_decides {
                assert!(
                    matches!(bare, CircleRoots::Uncertain),
                    "case {k}: the walk declines on its own, got {bare:?}"
                );
                continue;
            }
            let CircleRoots::Certified { count, thetas } = bare else {
                panic!("case {k}: the unmetered walk certifies, got {bare:?}");
            };
            let off = worst(pose, &thetas[..count], truth);
            assert!(
                off > 10.0 * band.zero(),
                "case {k}: the unmetered walk's worst root is {off:e} m of arc off — not a pose \
                 the meter decides"
            );
        }
    }

    /// **The slack meter charges every term of its slack.** Each pose is a
    /// graze at ε = 1e-12 that the unmetered walk certifies and the door
    /// refuses on the slack alone, chosen (from 5,594 such draws) where
    /// one term of `speed_hi · f_hi · (|res| + δres) / L` is a share of the
    /// slack the decision turns on — so each of these leaves its pose
    /// certified, and this row red:
    ///
    /// - **the running error `δres`**: the residual at the root reads
    ///   exactly zero, so the slack is the error alone (56 bands);
    /// - **the top speed**: 1.90 m/rad, the slack 1.13 bands — 0.59
    ///   without it;
    /// - **the ceiling `f_hi`**: the near-surface one, at least four times
    ///   the floor `2r(R² − r²)`, the slack 3.30 bands;
    /// - **the `sin`/`cos` ulp** in the running error: the slack 1.21
    ///   bands, under one without it;
    /// - **the scale**: 344 bands, refused by a meter even a thousand times
    ///   too lenient would still refuse at the band, and certified by one
    ///   lenient past that.
    #[test]
    fn the_slack_meter_charges_every_term() {
        let band = Band::new(1e-12, 1e-11).unwrap();
        let poses = [
            (
                "the running error",
                Pose {
                    n: [
                        0.608_777_588_456_305_7,
                        0.792_806_880_226_369_2,
                        -0.029_104_955_919_164_444,
                    ],
                    u: [
                        0.485_919_165_157_832_5,
                        -0.401_623_333_475_385_6,
                        -0.776_261_079_109_621_6,
                    ],
                    semi: [0.166_107_129_471_099, 0.072_460_761_202_133_53],
                    hub: [
                        -0.606_935_995_556_665,
                        0.445_634_111_326_365_6,
                        0.772_244_692_466_632_6,
                    ],
                    t_axis: [
                        0.047_166_878_093_737_43,
                        0.983_794_528_737_532_9,
                        -0.172_984_423_683_134_33,
                    ],
                    radii: [0.787_695_745_846_768, 0.249_148_700_815_951_58],
                    arc: [1.826_098_935_850_065_4, 3.826_098_935_850_065_4],
                },
            ),
            (
                "the top speed",
                Pose {
                    n: [
                        0.796_180_890_523_230_3,
                        -0.312_126_739_000_469_4,
                        -0.518_336_655_434_061_3,
                    ],
                    u: [
                        0.108_199_663_945_675_26,
                        0.916_309_872_538_711_2,
                        -0.385_576_257_321_601_4,
                    ],
                    semi: [1.897_792_441_002_41, 0.176_052_613_311_962_4],
                    hub: [
                        0.522_791_742_813_101_9,
                        2.950_837_667_185_968_6,
                        0.911_037_986_622_844_4,
                    ],
                    t_axis: [
                        0.008_176_122_043_039_566,
                        -0.781_740_634_607_516_3,
                        0.623_550_103_224_893_3,
                    ],
                    radii: [2.492_090_926_110_979, 0.447_393_809_566_632_05],
                    arc: [-0.777_699_639_108_853, 1.222_300_360_891_147_1],
                },
            ),
            (
                "the ceiling",
                Pose {
                    n: [
                        -0.794_255_938_990_818_1,
                        -0.522_864_951_735_142_1,
                        -0.309_466_873_226_879_1,
                    ],
                    u: [
                        -0.365_255_625_124_564_2,
                        0.817_927_303_931_229,
                        -0.444_503_378_838_288_2,
                    ],
                    semi: [1.096_570_959_622_172, 0.131_313_893_207_345_3],
                    hub: [
                        1.990_766_843_786_235_1,
                        -0.138_859_359_694_686_77,
                        0.048_045_398_969_543_296,
                    ],
                    t_axis: [
                        0.056_391_572_931_986_225,
                        0.569_510_769_104_171_2,
                        0.820_047_239_112_864_7,
                    ],
                    radii: [1.460_867_728_698_105_8, 0.386_776_223_321_915_6],
                    arc: [2.468_826_743_661_549, 4.468_826_743_661_549],
                },
            ),
            (
                "the sin and cos ulp",
                Pose {
                    n: [
                        -0.510_488_158_461_255_3,
                        0.779_906_099_752_252_2,
                        0.362_144_053_777_058_05,
                    ],
                    u: [
                        -0.859_838_578_391_589_4,
                        -0.458_616_410_292_053_24,
                        -0.224_384_953_417_918_9,
                    ],
                    semi: [1.014_843_470_380_15, 0.073_807_684_612_035_13],
                    hub: [
                        -0.152_651_666_856_109_65,
                        1.090_960_847_224_618_6,
                        0.005_360_826_971_560_123,
                    ],
                    t_axis: [
                        0.265_715_657_987_361_74,
                        0.859_541_037_651_336_7,
                        0.436_559_725_230_817_5,
                    ],
                    radii: [0.933_095_683_392_288_8, 0.325_506_321_029_914_4],
                    arc: [1.898_796_207_743_798_5, 3.898_796_207_743_798_5],
                },
            ),
            (
                "the scale",
                Pose {
                    n: [
                        -0.021_747_478_926_737_597,
                        -0.481_401_999_515_005_95,
                        0.876_230_085_093_684_4,
                    ],
                    u: [
                        -0.998_103_641_670_022_3,
                        0.060_936_986_391_912_46,
                        0.008_706_559_280_888_903,
                    ],
                    semi: [0.529_248_978_353_020_8, 0.034_569_611_120_486_406],
                    hub: [
                        -1.132_901_132_950_861_1,
                        1.129_724_772_975_331,
                        -1.359_293_926_047_994_8,
                    ],
                    t_axis: [
                        -0.741_673_070_558_528_5,
                        -0.669_752_596_305_118_5,
                        0.036_776_570_678_042_624,
                    ],
                    radii: [1.626_295_145_063_405_8, 0.390_029_589_115_468_9],
                    arc: [-0.900_470_601_956_637, 1.099_529_398_043_363_1],
                },
            ),
        ];
        for (term, pose) in &poses {
            let (door, bare) = pose.answers(band);
            assert!(
                matches!(bare, CircleRoots::Certified { .. }),
                "{term}: the unmetered walk certifies the pose, got {bare:?}"
            );
            assert!(
                matches!(door, CircleRoots::Uncertain),
                "{term}: the meter refuses the root it cannot place, got {door:?}"
            );
        }
    }

    /// **The draws, for the high-precision oracle.** Prints one JSON
    /// line per pose, tagged `ETDUMP` — the carrier, the torus, the arc,
    /// the band, the door's answer, and the residual with its running
    /// rounding bound at every certified root
    /// ([`geom_brep::conic_torus_residual`]).
    /// `scripts/oracles/ellipse_torus_mpmath.py` then re-solves each pose
    /// at 40 digits and checks the count, every root's place along the
    /// arc, every `Miss` and every bound. Run:
    /// `cargo nextest run -p topo --lib --run-ignored only
    /// dump_for_the_mpmath_oracle --no-capture | sed -n 's/^ETDUMP //p' >
    /// /tmp/et.jsonl`, then
    /// `python3 scripts/oracles/ellipse_torus_mpmath.py /tmp/et.jsonl`.
    #[test]
    #[ignore = "an oracle dump; run command in the docs"]
    fn dump_for_the_mpmath_oracle() {
        let mut rng = fuzz::start("ellipse_torus::dump_for_the_mpmath_oracle");
        let mut out = String::new();
        for eps in [1e-6, 1e-9, 1e-12] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            for (family, name) in FAMILIES.iter().enumerate() {
                for _ in 0..fuzz::scaled(250) {
                    let (e, s, t0, t1) = pose(&mut rng, family, eps);
                    let (
                        geom::Curve3::Ellipse {
                            center,
                            axis,
                            major,
                            minor,
                            u_ref,
                        },
                        geom::Surface::Torus {
                            center: hub,
                            axis: t_axis,
                            major_radius,
                            minor_radius,
                            ..
                        },
                    ) = (&e, &s)
                    else {
                        unreachable!("the pose is an ellipse and a torus")
                    };
                    let (answer, roots) = match ellipse_torus_roots(&e, t0, t1, &s, band) {
                        Ok(CircleRoots::Certified { count, thetas }) => {
                            ("certified", thetas[..count].to_vec())
                        }
                        Ok(CircleRoots::Miss) => ("miss", vec![]),
                        Ok(CircleRoots::Uncertain) => ("uncertain", vec![]),
                        other => panic!("{other:?}"),
                    };
                    let conic = geom_brep::Conic::of(&e).expect("an ellipse");
                    let bounds: Vec<[f64; 2]> = roots
                        .iter()
                        .map(|&t| {
                            let r = geom_brep::conic_torus_residual(
                                &conic,
                                *hub,
                                *t_axis,
                                *major_radius,
                                *minor_radius,
                                t,
                            );
                            [r.value, r.error]
                        })
                        .collect();
                    let v = |p: Vec3<f64>| format!("[{:?}, {:?}, {:?}]", p.x, p.y, p.z);
                    let pt = |p: Point3<f64>| format!("[{:?}, {:?}, {:?}]", p.x, p.y, p.z);
                    writeln!(
                        out,
                        "ETDUMP {{\"family\": \"{name}\", \"eps\": {eps:?}, \"center\": {}, \"axis\": {}, \
                         \"u_ref\": {}, \"major\": {major:?}, \"minor\": {minor:?}, \"hub\": {}, \
                         \"t_axis\": {}, \"R\": {major_radius:?}, \"r\": {minor_radius:?}, \
                         \"t0\": {t0:?}, \"t1\": {t1:?}, \"answer\": \"{answer}\", \"roots\": {roots:?}, \
                         \"residuals\": {bounds:?}}}",
                        pt(*center),
                        v(*axis),
                        v(*u_ref),
                        pt(*hub),
                        v(*t_axis),
                    )
                    .unwrap();
                }
            }
        }
        print!("{out}");
    }
}
