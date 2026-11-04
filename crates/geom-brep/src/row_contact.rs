//! **A shared row's contact class on analytic charts** (D2's structural
//! selector, [`crate::SharedRow`]): whether the two surfaces cross along
//! the whole row or touch along the whole row. A row that is neither
//! refuses.
//!
//! A structural selector owes no transversality margin: a curve known in
//! advance is picked exactly whatever the contact order. What the
//! certificate still owes is the class. A row read as crossing where the
//! surfaces touch somewhere along it is the defect this reading exists to
//! exclude, so neither class is ever read off sampled feet. Each class is a
//! bound over the whole row.
//!
//! **Why the bounds are closed forms.** An analytic row image
//! ([`analytic_row`]) holds one chart coordinate fixed and moves the other
//! as `phase ± t` (an angle) or linearly (a free coordinate on a plane,
//! cylinder or cone). Along such a row every admitted chart's unit normal
//! is a first harmonic `A + B·cos s + C·sin s` of the moving coordinate `s`:
//!   - a plane's normal is constant;
//!   - a cylinder's or cone's depends on the azimuth alone, as a first
//!     harmonic, and is constant along a generator;
//!   - a sphere's or torus's is `cos v·r̂(u) + sin v·â`, a first harmonic
//!     in `u` at fixed `v` and in `v` at fixed `u`.
//!
//! So each wall's normal along its row is a first harmonic of the carrier's
//! own parameter `t`, and three readings (`t = 0, π/2, π`) give its
//! coefficients exactly.

use geom::{Curve3, Surface};
use geom_core::{Band, Decide, Lever, Margin, Real, Sign, Vec3};

use crate::certify::CertifyError;
use crate::pcurve_cache::{Channel, Pcurve};
use crate::recourse::{Classified, Refused};

/// What a shared row's certificate reads of its two surfaces' contact
/// along the row ([`analytic_row_contact`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowContact {
    /// The surfaces cross along the whole row: their normals' sine is
    /// bounded away from zero over all of it.
    Crossing,
    /// The surfaces touch along the whole row: their normals agree, up
    /// to orientation, within the band over all of it.
    Touching,
}

/// The crossing cover's pieces, as halvings of the edge's span: 256
/// pieces, so a quarter turn's piece has half-width about 3·10⁻³ and a
/// crossing whose sine clears the normals' swing over it reads.
const COVER_HALVINGS: u32 = 8;

/// **Whether `image` is an analytic row image**: a closed-form image
/// ([`Pcurve::Harmonic`]) that holds one coordinate fixed and moves the
/// other once around per carrier period (an angle winding ±1) or
/// linearly (a free coordinate).
pub(crate) fn harmonic_row<T: Real>(image: &Pcurve<T>) -> bool {
    let zero = |x: T| geom_core::is_exact_zero(x);
    let fixed = |c: &Channel<T>| match *c {
        Channel::Angle { winding, .. } => winding == 0,
        Channel::Free { a, b, l, .. } => zero(a) && zero(b) && zero(l),
    };
    let moving = |c: &Channel<T>| match *c {
        Channel::Angle { winding, .. } => winding.abs() == 1,
        Channel::Free { a, b, l, .. } => zero(a) && zero(b) && !zero(l),
    };
    matches!(image, Pcurve::Harmonic { u, v }
        if (fixed(u) && moving(v)) || (fixed(v) && moving(u)))
}

/// **Whether `image` is an analytic row of `wall`'s chart**: `wall` is a
/// plane, cylinder, cone, sphere or torus and `image` a row image
/// ([`harmonic_row`]), the rows along which the module's first-harmonic
/// normal holds.
pub(crate) fn analytic_row<T: Real>(wall: &Surface<T>, image: &Pcurve<T>) -> bool {
    matches!(
        wall,
        Surface::Plane { .. }
            | Surface::Cylinder { .. }
            | Surface::Cone { .. }
            | Surface::Sphere { .. }
            | Surface::Torus { .. }
    ) && harmonic_row(image)
}

/// **A shared row's one admission test on an analytic wall**: `image` is
/// an analytic row of `wall` ([`analytic_row`]) and certifies there as
/// the image of `carrier` over `span` ([`crate::PcurveCache::certify`]).
pub fn analytic_row_certifies<T: Decide>(
    wall: &Surface<T>,
    image: &Pcurve<T>,
    (carrier, (t0, t1)): (&Curve3<T>, (T, T)),
    band: Band,
) -> bool {
    analytic_row(wall, image)
        && crate::pcurve_cache::PcurveCache::certify(image.clone(), t0, t1, carrier, wall, band)
            .is_ok()
}

/// **The shared row of two analytic walls along `carrier`**, where it is
/// one: each wall's closed-form image of the carrier over `(t0, t1)`
/// ([`crate::pcurve_cache::chart_pcurve_over`]) is an analytic row of
/// that wall's chart and certifies there ([`analytic_row_certifies`]).
/// `None` otherwise. A construction that knows its edge is a row of both
/// walls' charts (a blend's section, where two bands meet) states it
/// here; the certificate then reads its contact class
/// ([`RowContact`]).
pub fn analytic_shared_row<T: Decide>(
    pair: crate::SurfacePair,
    walls: [(crate::SurfaceKey, &Surface<T>); 2],
    carrier: &Curve3<T>,
    (t0, t1): (T, T),
    band: Band,
) -> Option<crate::CurveSelector<T>> {
    let image = |wall: &Surface<T>| {
        crate::pcurve_cache::chart_pcurve_over(carrier, t0, t1, wall, band)
            .ok()
            .filter(|image| analytic_row_certifies(wall, image, (carrier, (t0, t1)), band))
    };
    let [(ka, wa), (kb, wb)] = walls;
    let row = crate::SharedRow::new(pair, (ka, image(wa)?), (kb, image(wb)?))?;
    Some(crate::CurveSelector::Structural(
        crate::Structural::SharedRow(row),
    ))
}

/// The first harmonic `a + b·cos t + c·sin t`.
#[derive(Clone, Copy)]
struct Harmonic<T: Real> {
    a: Vec3<T>,
    b: Vec3<T>,
    c: Vec3<T>,
}

impl<T: Real> Harmonic<T> {
    /// The coefficients of a first harmonic `f`, read at `0`, `π/2`, `π`.
    fn of(f: &impl Fn(T) -> Vec3<T>) -> Self {
        let half = T::from_f64(0.5);
        let (f0, fq, fp) = (f(T::zero()), f(T::pi() * half), f(T::pi()));
        let a = (f0 + fp) * half;
        Self {
            a,
            b: (f0 - fp) * half,
            c: fq - a,
        }
    }

    /// `self − other·s`.
    fn minus(self, other: Self, s: T) -> Self {
        Self {
            a: self.a - other.a * s,
            b: self.b - other.b * s,
            c: self.c - other.c * s,
        }
    }

    /// `√(|b|² + |c|²)`, which bounds `|b·cos t + c·sin t|` and the
    /// derivative's magnitude at every `t`.
    fn swing(self) -> T {
        (self.b.norm_squared() + self.c.norm_squared()).sqrt()
    }

    /// A bound on `|f(t)|` at every `t`: `|a|` plus the swing.
    fn sup(self) -> T {
        self.a.norm() + self.swing()
    }
}

/// **The contact class of a shared row on two analytic charts**, over
/// the edge's interval `(t0, t1)`, each wall with its certified row
/// image, read at `extent`, the lever the edge certificate reads it at.
/// Returns the class and, for a touching row, the levered bound it
/// decided as zero, a residual the certificate folds. The edge
/// certificate of an analytic shared row runs this reading.
///
/// # Errors
///
/// As [`contact_of`].
pub fn analytic_row_contact<T: Decide>(
    span: (T, T),
    walls: [(&Surface<T>, &Pcurve<T>); 2],
    extent: Lever<T>,
    band: Band,
) -> Result<(RowContact, T), CertifyError> {
    let normal = |(wall, image): (&Surface<T>, &Pcurve<T>), t: T| {
        let uv = image.eval(t);
        crate::implicit::implicit_gradient(wall, wall.eval(uv.x, uv.y)).normalize()
    };
    let [a, b] = walls;
    contact_of(|t| normal(a, t), |t| normal(b, t), span, extent, band)
}

/// **The contact class of two unit-normal fields `na`, `nb`**, each a
/// first harmonic of `t`, over `(t0, t1)`:
///
/// - **Touching**: `sin θ ≤ |n_a − σ·n_b|` for unit normals and either
///   `σ = ±1`. `n_a − σ·n_b` is a first harmonic, so its sup over every
///   `t` is at most `|A| + √(|B|² + |C|²)`. The smaller of the two
///   orientations' bounds, levered by the edge's extent, decided zero,
///   reads the row as touching everywhere.
/// - **Crossing**: `t ↦ n_a × n_b` moves no faster than
///   `L = swing_a + swing_b`, since a first harmonic's derivative is
///   bounded by its swing. So over a piece of half-width `h` about `m`,
///   `sin θ ≥ sin θ(m) − L·h`. The span is cut into [`COVER_HALVINGS`]
///   halvings' worth of closed pieces, ends included, and the least of
///   those bounds, levered by the extent, decided positive, reads the row
///   as crossing everywhere.
/// - **Neither**: the least sine at the pieces' middles, decided zero, is
///   a point where the surfaces touch on a row that does not touch
///   everywhere. It refuses as a zero crossing,
///   [`CertifyError::NotTransverse`], as a spline row that turns does.
///
/// # Errors
///
/// [`CertifyError::NotTransverse`] for a row that touches at a point and
/// not everywhere, and [`CertifyError::Escalated`]
/// ([`crate::CertCheck::Transversality`]) where a reading lands in the
/// band or the cover's bound does not clear zero.
pub(crate) fn contact_of<T: Decide>(
    na: impl Fn(T) -> Vec3<T>,
    nb: impl Fn(T) -> Vec3<T>,
    (t0, t1): (T, T),
    extent: Lever<T>,
    band: Band,
) -> Result<(RowContact, T), CertifyError> {
    let (ha, hb) = (Harmonic::of(&na), Harmonic::of(&nb));
    let escalated = |cause| CertifyError::Escalated {
        check: crate::CertCheck::Transversality,
        sample: crate::certify::NOT_A_SAMPLE,
        cause,
    };
    let touch = ha
        .minus(hb, T::one())
        .sup()
        .min(ha.minus(hb, -T::one()).sup());
    let touch = Margin::levered(touch, extent);
    let touch_value = touch.value();
    let touches =
        crate::dihedral::decide_reported("shared_row_touch", touch, band).map_err(escalated)?;
    if touches.sign == Sign::Zero {
        return Ok((RowContact::Touching, touch_value));
    }
    let lipschitz = ha.swing() + hb.swing();
    let half = T::from_f64(0.5);
    let mut pieces = vec![(t0, t1)];
    for _ in 0..COVER_HALVINGS {
        pieces = pieces
            .into_iter()
            .flat_map(|(lo, hi)| {
                let mid = (lo + hi) * half;
                [(lo, mid), (mid, hi)]
            })
            .collect();
    }
    let (mut least, mut closest) = (T::from_f64(f64::MAX), T::from_f64(f64::MAX));
    for (lo, hi) in pieces {
        let sine = crate::tangent::normal_sine(na((lo + hi) * half), nb((lo + hi) * half));
        least = least.min(sine - lipschitz * ((hi - lo) * half));
        closest = closest.min(sine);
    }
    let point = crate::dihedral::decide_reported(
        "shared_row_touch_point",
        Margin::levered(closest, extent),
        band,
    )
    .map_err(escalated)?;
    if point.sign == Sign::Zero {
        return Err(CertifyError::NotTransverse {
            sample: crate::certify::NOT_A_SAMPLE,
            lever: None,
            verdict: Refused::Zero(Classified {
                margin: point.margin,
                band,
            }),
        });
    }
    crate::dihedral::decide_positive("shared_row_crossing", Margin::levered(least, extent), band)
        .map_err(escalated)?;
    Ok((RowContact::Crossing, T::zero()))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use geom_core::Interval;

    fn band() -> Band {
        Band::linear(geom_core::Tol::witness()).unwrap()
    }

    /// Two unit-normal fields, both first harmonics, that agree at
    /// `t = 0` and nowhere else on `(−π, π)`: `(cos t, sin t, 0)` and that
    /// field with its `sin t` part tilted by `tilt` about the x axis. The
    /// surfaces they belong to touch at one point of the row and cross
    /// everywhere else on it.
    fn touching_at_zero<T: Real>(tilt: f64) -> (impl Fn(T) -> Vec3<T>, impl Fn(T) -> Vec3<T>) {
        let (s, c) = (T::from_f64(tilt.sin()), T::from_f64(tilt.cos()));
        (
            |t: T| Vec3::new(t.cos(), t.sin(), T::zero()),
            move |t: T| Vec3::new(t.cos(), t.sin() * c, t.sin() * s),
        )
    }

    /// The span `[−π/512, π − π/512]`: 0 is the middle of its first
    /// cover piece, so the cover reads the touch point itself.
    fn span_with_touch_at_a_middle<T: Real>() -> (T, T) {
        let end = |k: f64| T::from_f64(core::f64::consts::PI * k);
        (end(-1.0 / 512.0), end(1.0 - 1.0 / 512.0))
    }

    #[test]
    fn a_row_that_touches_at_one_point_refuses_as_a_zero_crossing() {
        let lever = geom_core::test_support::fixture_lever(1.0);
        let (na, nb) = touching_at_zero::<f64>(0.6);
        let got = contact_of(na, nb, span_with_touch_at_a_middle(), lever, band());
        assert!(
            matches!(
                got,
                Err(CertifyError::NotTransverse {
                    verdict: Refused::Zero(_),
                    ..
                })
            ),
            "f64: a touch point on a crossing row reads as a zero crossing: {got:?}"
        );
        let lever = geom_core::test_support::fixture_lever(Interval::from_f64(1.0));
        let (na, nb) = touching_at_zero::<Interval>(0.6);
        let got = contact_of(na, nb, span_with_touch_at_a_middle(), lever, band());
        assert!(
            matches!(
                got,
                Err(CertifyError::NotTransverse {
                    verdict: Refused::Zero(_),
                    ..
                })
            ),
            "Interval: a touch point on a crossing row reads as a zero crossing: {got:?}"
        );
    }

    /// The defect the whole-row reading exists to keep dead: a row whose
    /// surfaces touch at an end, or between any sampled feet, never reads
    /// as crossing. The edge certificate's interior feet on these spans
    /// all read a definite crossing angle.
    #[test]
    fn a_row_touching_off_its_feet_never_reads_as_crossing() {
        let lever = geom_core::test_support::fixture_lever(1.0);
        let pi = core::f64::consts::PI;
        for (what, span) in [
            ("at its start", (0.0, pi / 2.0)),
            ("at its end", (-pi / 2.0, 0.0)),
            ("between feet", (-0.3, 1.1)),
        ] {
            let (na, nb) = touching_at_zero::<f64>(0.6);
            let feet: Vec<f64> = (1..crate::CERT_SAMPLES - 1)
                .map(|i| crate::sample_param(span.0, span.1, i))
                .map(|t| crate::tangent::normal_sine(na(t), nb(t)))
                .collect();
            assert!(
                feet.iter().all(|s| *s > 1e-3),
                "{what}: every interior foot reads a crossing ({feet:?})"
            );
            let got = contact_of(na, nb, span, lever, band());
            assert!(
                got.is_err(),
                "{what}: a row touching off its feet refuses, not {got:?}"
            );
        }
    }

    #[test]
    fn coincident_normals_read_as_touching_and_a_constant_angle_as_crossing() {
        let lever = geom_core::test_support::fixture_lever(1.0);
        let span = (0.0, core::f64::consts::FRAC_PI_2);
        let same = |t: f64| Vec3::new(t.cos(), t.sin(), 0.0);
        let flipped = |t: f64| -same(t);
        for nb in [&same as &dyn Fn(f64) -> Vec3<f64>, &flipped] {
            let got = contact_of(same, nb, span, lever, band()).unwrap();
            assert_eq!(got.0, RowContact::Touching, "either orientation touches");
        }
        let (na, nb) = touching_at_zero::<f64>(0.6);
        let shifted = |t: f64| nb(t + 1.0);
        let got = contact_of(|t| na(t + 1.0), shifted, (0.0, 1.0), lever, band());
        assert_eq!(
            got.unwrap().0,
            RowContact::Crossing,
            "a row clear of its touch point crosses"
        );
    }

    /// The premise every bound here stands on: along each admitted row of
    /// each admitted chart, the unit normal is the first harmonic its
    /// three readings give. Checked at points of the row other than the
    /// three, on both row directions of every chart.
    #[test]
    fn an_admitted_rows_normal_is_its_first_harmonic() {
        let x = Vec3::new(0.3, -0.2, 0.9).normalize();
        let u_ref = Vec3::new(1.0, 0.0, 0.0).cross(x).normalize();
        let o = geom_core::Point3::new(0.4, -0.7, 1.3);
        let charts = [
            Surface::Plane {
                origin: o,
                normal: x,
                u_ref,
            },
            Surface::Cylinder {
                origin: o,
                axis: x,
                radius: 0.6,
                u_ref,
            },
            Surface::Cone {
                apex: o,
                axis: x,
                half_angle: 0.5,
                u_ref,
            },
            Surface::Sphere {
                center: o,
                radius: 0.8,
                axis: x,
                u_ref,
            },
            Surface::Torus {
                center: o,
                axis: x,
                major_radius: 1.1,
                minor_radius: 0.3,
                u_ref,
            },
        ];
        let angle = |phase: f64, winding: i32| Channel::Angle { phase, winding };
        let free = |c: f64, l: f64| Channel::Free {
            c,
            a: 0.0,
            b: 0.0,
            l,
        };
        for chart in &charts {
            let (u_fixed, v_moving, v_fixed) = match chart {
                Surface::Sphere { .. } | Surface::Torus { .. } => {
                    (angle(0.7, 0), angle(0.2, -1), angle(0.4, 0))
                }
                _ => (free(0.7, 0.0), free(0.2, 0.5), free(0.4, 0.0)),
            };
            let u_moving = match chart {
                Surface::Plane { .. } => free(0.1, 0.8),
                _ => angle(0.1, 1),
            };
            for image in [
                Pcurve::Harmonic {
                    u: u_moving,
                    v: v_fixed,
                },
                Pcurve::Harmonic {
                    u: u_fixed,
                    v: v_moving,
                },
            ] {
                assert!(
                    analytic_row(chart, &image),
                    "{:?} admits {image:?}",
                    chart.kind()
                );
                let normal = |t: f64| {
                    let uv = image.eval(t);
                    crate::implicit::implicit_gradient(chart, chart.eval(uv.x, uv.y)).normalize()
                };
                let h = Harmonic::of(&normal);
                for k in 0..16 {
                    let t = 0.37 + 0.41 * f64::from(k);
                    let fit = h.a + h.b * t.cos() + h.c * t.sin();
                    assert!(
                        (fit - normal(t)).norm() < 1e-12,
                        "{:?} along {image:?} at t = {t}: {fit:?} against {:?}",
                        chart.kind(),
                        normal(t)
                    );
                }
            }
        }
    }
}
