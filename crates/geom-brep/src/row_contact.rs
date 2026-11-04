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
//! (`description::harmonic_row`) holds one chart coordinate fixed and moves
//! the other: it turns, as `phase ± t`, or slides linearly. Away from the
//! chart's singular points, each admitted chart's unit normal along such a
//! row is a first harmonic `A + B·cos t + C·sin t` of the carrier's own
//! parameter:
//!   - a plane's normal is constant, and so is a cylinder's along a
//!     generator, and a cone's along a generator on one nappe;
//!   - a cylinder's or cone's along a parallel is a first harmonic of the
//!     azimuth;
//!   - a sphere's or ring torus's is `cos v·r̂(u) + sin v·â`, a first
//!     harmonic in `u` at fixed `v` and in `v` at fixed `u`.
//!
//! The exceptions are the singular points: a cone's normal flips across
//! its apex, and a torus with `r ≥ R` has no smooth normal on its axis.
//! A row that reaches one refuses (`row_clear_of_singularities`). A sphere's
//! poles are no exception: its normal is `(p − c)/R` there too.
//!
//! So each wall's normal along its row is a first harmonic of `t`. Three
//! readings inside the edge's span give its coefficients exactly, and the
//! harmonic then holds at every `t`.

use geom::{Curve3, Surface};
use geom_core::k_stats::Magnitude;
use geom_core::{Band, Decide, Lever, Margin, Real, Sign, Vec3};

use crate::certify::{CertifyError, NOT_A_SAMPLE, RowRefusal};
use crate::description::{HarmonicRow, WallRow, harmonic_row, wall_row};
use crate::keys::SurfaceKey;
use crate::pcurve_cache::{Pcurve, PcurveCache, PcurveCheck};
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

/// **The shared row of two analytic walls along `carrier`**, where it is
/// one: each wall's closed-form image of the carrier over `(t0, t1)`
/// ([`crate::pcurve_cache::chart_pcurve_over`]) is a row of that wall's
/// analytic chart and certifies there (`certify::pair_row_certifies`).
/// `None` otherwise. A construction that knows its edge is a row of both
/// walls' charts (a blend's section, where two bands meet) states it
/// here; the certificate then reads its contact class
/// ([`RowContact`]).
pub fn analytic_shared_row<T: Decide>(
    pair: crate::SurfacePair,
    walls: [(SurfaceKey, &Surface<T>); 2],
    carrier: &Curve3<T>,
    (t0, t1): (T, T),
    band: Band,
) -> Option<crate::CurveSelector<T>> {
    let image = |wall: &Surface<T>| {
        crate::pcurve_cache::chart_pcurve_over(carrier, t0, t1, wall, band).ok()
    };
    let [(ka, wa), (kb, wb)] = walls;
    let (ia, ib) = (image(wa)?, image(wb)?);
    let rows =
        crate::certify::pair_row_certifies([(wa, &ia), (wb, &ib)], (carrier, (t0, t1)), band);
    if rows != Some([WallRow::Analytic, WallRow::Analytic]) {
        return None;
    }
    let row = crate::SharedRow::new(pair, (ka, ia), (kb, ib))?;
    Some(crate::CurveSelector::Structural(
        crate::Structural::SharedRow(row),
    ))
}

/// One wall's unit normal at its row image's point at `t`.
pub(crate) fn row_normal<T: Decide>(wall: &Surface<T>, image: &Pcurve<T>, t: T) -> Vec3<T> {
    let uv = image.eval(t);
    crate::implicit::implicit_gradient(wall, wall.eval(uv.x, uv.y)).normalize()
}

/// **A row of `wall` keeps clear of its chart's singular points over
/// `(t0, t1)`**, the premise of the module's first harmonic. A cone's row
/// must not reach its apex (`v = 0`): a parallel's `v` is decided nonzero,
/// and a generator's `v` at both ends is decided nonzero and of one sign
/// (it is linear in `t`). A torus must be a ring torus, `R − r` decided
/// positive. Planes, cylinders and spheres have no singular point on their
/// normals.
///
/// # Errors
///
/// [`CertifyError::Row`] ([`RowRefusal::Singular`]) on `key` where the
/// row reaches one, [`CertifyError::Unimplemented`] for an image that is
/// no analytic row of `wall`, [`RowRefusal::ImageEscalated`]
/// ([`PcurveCheck::ChartClass`]) on `key` where the row's reading lands in
/// the band (`description::harmonic_row`), and [`CertifyError::Escalated`]
/// ([`crate::CertCheck::ImageSpan`]) where a singular-point decision does.
pub(crate) fn row_clear_of_singularities<T: Decide>(
    key: SurfaceKey,
    wall: &Surface<T>,
    image: &Pcurve<T>,
    (t0, t1): (T, T),
    band: Band,
) -> Result<HarmonicRow, CertifyError> {
    if wall_row(wall, image) != Ok(WallRow::Analytic) {
        return Err(CertifyError::Unimplemented);
    }
    let row = match harmonic_row(image, (t0, t1), band) {
        Ok(Some(row)) => row,
        Ok(None) => return Err(CertifyError::Unimplemented),
        Err(cause) => {
            return Err(CertifyError::Row {
                surface: key,
                refusal: RowRefusal::ImageEscalated {
                    check: PcurveCheck::ChartClass,
                    cause,
                },
            });
        }
    };
    let singular = Err::<HarmonicRow, _>(CertifyError::Row {
        surface: key,
        refusal: RowRefusal::Singular,
    });
    let escalated = |cause| CertifyError::Escalated {
        check: crate::CertCheck::ImageSpan,
        sample: NOT_A_SAMPLE,
        cause,
    };
    let side = |name, x: T| crate::dihedral::decide_reported(name, Margin::of(x), band);
    match *wall {
        Surface::Cone { .. } => {
            let v = |t: T| -> Result<Sign, CertifyError> {
                Ok(side("shared_row_apex", image.eval(t).y)
                    .map_err(escalated)?
                    .sign)
            };
            let start = v(t0)?;
            // A parallel holds `v`; a generator's `v` is linear in `t`, so
            // its two ends' signs are its whole span's.
            let end = if row.along_u { start } else { v(t1)? };
            if start == Sign::Zero || start != end {
                singular
            } else {
                Ok(row)
            }
        }
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => match side("shared_row_ring_torus", major_radius - minor_radius)
            .map_err(escalated)?
            .sign
        {
            Sign::Positive => Ok(row),
            _ => singular,
        },
        _ => Ok(row),
    }
}

/// **The contact class of a shared row on two analytic charts**, each
/// wall with its certified row image ([`PcurveCache`], over the edge's
/// interval), read at `extent`, the lever the edge certificate reads it
/// at (`edge_extent_inf` over the edge). The edge certificate of an
/// analytic shared row runs this reading.
///
/// # Errors
///
/// As `row_clear_of_singularities` for either wall, then as `contact_of`.
pub fn analytic_row_contact<T: Decide>(
    walls: [(SurfaceKey, &Surface<T>, &PcurveCache<T>); 2],
    extent: Lever<T>,
    band: Band,
) -> Result<RowContact, CertifyError> {
    let span = walls[0].2.params();
    let [(ka, a, ca), (kb, b, cb)] = walls;
    let ra = row_clear_of_singularities(ka, a, ca.pcurve(), span, band)?;
    let rb = row_clear_of_singularities(kb, b, cb.pcurve(), span, band)?;
    read_contact(
        [(a, ca.pcurve(), ra), (b, cb.pcurve(), rb)],
        span,
        extent,
        band,
    )
}

/// [`analytic_row_contact`] once both rows are read and known clear of
/// their charts' singular points (`row_clear_of_singularities`).
pub(crate) fn read_contact<T: Decide>(
    walls: [(&Surface<T>, &Pcurve<T>, HarmonicRow); 2],
    span: (T, T),
    extent: Lever<T>,
    band: Band,
) -> Result<RowContact, CertifyError> {
    let [(a, ia, ra), (b, ib, rb)] = walls;
    contact_of(
        |t| row_normal(a, ia, t),
        |t| row_normal(b, ib, t),
        [ra.turning, rb.turning],
        span,
        extent,
        band,
    )
}

/// The first harmonic `a + b·cos τ + c·sin τ`, `τ = t − t_c`.
#[derive(Clone, Copy)]
struct Harmonic<T: Real> {
    a: Vec3<T>,
    b: Vec3<T>,
    c: Vec3<T>,
}

impl<T: Real> Harmonic<T> {
    /// The coefficients of a first harmonic from its values at
    /// `τ = −d, 0, d`, `0 < d ≤ 2π/3`: `f(d) + f(−d) = 2a + 2b·cos d` and
    /// `f(d) − f(−d) = 2c·sin d`, with `1 − cos d` spelled `2·sin²(d/2)`.
    fn of_turning([fm, f0, fp]: [Vec3<T>; 3], d: T) -> Self {
        let half = T::from_f64(0.5);
        let one_minus_cos = (d * half).sin().powi(2) * T::from_f64(2.0);
        let b = (f0 - (fm + fp) * half) / one_minus_cos;
        Self {
            a: f0 - b,
            b,
            c: (fp - fm) / (d.sin() * T::from_f64(2.0)),
        }
    }

    /// A constant.
    fn constant(a: Vec3<T>) -> Self {
        let zero = Vec3::new(T::zero(), T::zero(), T::zero());
        Self {
            a,
            b: zero,
            c: zero,
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

    /// A bound on `|f(τ)|` at every `τ`: `|a| + √(|b|² + |c|²)`, since
    /// `|b·cos τ + c·sin τ| ≤ √(|b|² + |c|²)`.
    fn sup(self) -> T {
        self.a.norm() + (self.b.norm_squared() + self.c.norm_squared()).sqrt()
    }

    /// A bound on `|f(τ)·g(τ)|` at every `τ`. The product is
    /// `p₀ + p₁c·cos τ + p₁s·sin τ + p₂c·cos 2τ + p₂s·sin 2τ` with
    /// `p₀ = a·a' + (b·b' + c·c')/2`, `p₁c = a·b' + b·a'`,
    /// `p₁s = a·c' + c·a'`, `p₂c = (b·b' − c·c')/2` and
    /// `p₂s = (b·c' + c·b')/2`, so `|p₀|` plus the two harmonics'
    /// amplitudes bounds it.
    fn dot_sup(self, o: Self) -> T {
        let half = T::from_f64(0.5);
        let amp = |x: T, y: T| (x.powi(2) + y.powi(2)).sqrt();
        let p0 = self.a.dot(o.a) + (self.b.dot(o.b) + self.c.dot(o.c)) * half;
        let p1 = amp(
            self.a.dot(o.b) + self.b.dot(o.a),
            self.a.dot(o.c) + self.c.dot(o.a),
        );
        let p2 = amp(
            (self.b.dot(o.b) - self.c.dot(o.c)) * half,
            (self.b.dot(o.c) + self.c.dot(o.b)) * half,
        );
        p0.abs() + p1 + p2
    }
}

/// **The contact class of two unit-normal fields `na`, `nb`** over
/// `(t0, t1)`. Each is a first harmonic of `t` that turns, or a constant
/// (`turning`). Both are read at `t₀`, `t₀ + d`, `t₀ + 2d` with
/// `d = (t₁ − t₀)/3`, which gives each harmonic exactly: those three
/// points lie in the span, where the premise holds.
///
/// - **Touching**: `sin θ ≤ |n_a − σ·n_b|` for unit normals and either
///   `σ = ±1`. `n_a − σ·n_b` is a first harmonic, so [`Harmonic::sup`]
///   bounds it at every `t`. The smaller orientation's bound, levered by
///   `extent`, decided zero, reads the row as touching everywhere.
/// - **A touch point**: the least sine at the three readings, levered and
///   decided zero, on a row that does not touch everywhere, refuses as a
///   zero crossing, [`CertifyError::NotTransverse`], as a spline row that
///   turns does.
/// - **Crossing**: `cos θ = n_a·n_b`, a degree-2 trigonometric
///   polynomial, is at most `P` in magnitude at every `t`
///   ([`Harmonic::dot_sup`]), so `sin θ ≥ √(1 − P²)`, clamped at zero.
///   That bound, levered and decided positive, reads the row as crossing
///   everywhere. Two walls whose rows are one circle turn their normals
///   by one rotation, so `n_a·n_b` is constant and the bound is the angle
///   itself: no constant angle is charged a variation it does not have.
///
/// # Errors
///
/// [`CertifyError::NotTransverse`] for a touch point, and
/// [`CertifyError::Escalated`] ([`crate::CertCheck::Transversality`])
/// where a reading lands in the band, or where the crossing bound does not
/// clear it (a bound of zero where the walls' angle varies along the row).
pub(crate) fn contact_of<T: Decide>(
    na: impl Fn(T) -> Vec3<T>,
    nb: impl Fn(T) -> Vec3<T>,
    turning: [bool; 2],
    (t0, t1): (T, T),
    extent: Lever<T>,
    band: Band,
) -> Result<RowContact, CertifyError> {
    let escalated = |cause| CertifyError::Escalated {
        check: crate::CertCheck::Transversality,
        sample: NOT_A_SAMPLE,
        cause,
    };
    let d = (t1 - t0) * T::from_f64(1.0 / 3.0);
    let at = [t0, t0 + d, t0 + d + d];
    let (va, vb) = (at.map(&na), at.map(&nb));
    let harmonic = |v: [Vec3<T>; 3], turns: bool| {
        if turns {
            Harmonic::of_turning(v, d)
        } else {
            Harmonic::constant(v[1])
        }
    };
    let (ha, hb) = (harmonic(va, turning[0]), harmonic(vb, turning[1]));
    let touch = ha
        .minus(hb, T::one())
        .sup()
        .min(ha.minus(hb, -T::one()).sup());
    let touches =
        crate::dihedral::decide_magnitude("shared_row_touch", Margin::levered(touch, extent), band)
            .map_err(escalated)?;
    if touches == Magnitude::Zero {
        return Ok(RowContact::Touching);
    }
    let sines: [T; 3] = core::array::from_fn(|k| crate::tangent::normal_sine(va[k], vb[k]));
    let closest = sines[0].min(sines[1]).min(sines[2]);
    let (point, margin) = crate::dihedral::decide_magnitude_reported(
        "shared_row_touch_point",
        Margin::levered(closest, extent),
        band,
    )
    .map_err(escalated)?;
    if point == Magnitude::Zero {
        return Err(CertifyError::NotTransverse {
            sample: NOT_A_SAMPLE,
            lever: None,
            verdict: Refused::Zero(Classified { margin, band }),
        });
    }
    let cos_bound = ha.dot_sup(hb);
    let sine = (T::one() - cos_bound.powi(2)).max(T::zero()).sqrt();
    crate::dihedral::decide_positive("shared_row_crossing", Margin::levered(sine, extent), band)
        .map_err(escalated)?;
    Ok(RowContact::Crossing)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;
    use crate::pcurve_cache::Channel;
    use geom_core::Interval;

    fn band() -> Band {
        Band::linear(geom_core::Tol::witness()).unwrap()
    }

    /// Two turning unit-normal fields that agree at `t = 0` and nowhere
    /// else on `(−π, π)`: `(cos t, sin t, 0)` and that field with its
    /// `sin t` part tilted by `tilt` about the x axis. Their surfaces would
    /// touch at one point of the row and cross everywhere else on it.
    fn touching_at_zero<T: Real>(tilt: f64) -> (impl Fn(T) -> Vec3<T>, impl Fn(T) -> Vec3<T>) {
        let (s, c) = (T::from_f64(tilt.sin()), T::from_f64(tilt.cos()));
        (
            |t: T| Vec3::new(t.cos(), t.sin(), T::zero()),
            move |t: T| Vec3::new(t.cos(), t.sin() * c, t.sin() * s),
        )
    }

    /// Two turning fields at a constant angle `angle`: both rotate about z,
    /// as two walls' normals do along one shared circle.
    fn at_constant_angle(angle: f64) -> (impl Fn(f64) -> Vec3<f64>, impl Fn(f64) -> Vec3<f64>) {
        let (s, c) = angle.sin_cos();
        (
            |t: f64| Vec3::new(t.cos(), t.sin(), 0.0),
            move |t: f64| Vec3::new(t.cos() * c, t.sin() * c, s),
        )
    }

    fn escalated(got: &Result<RowContact, CertifyError>) -> bool {
        matches!(
            got,
            Err(CertifyError::Escalated {
                check: crate::CertCheck::Transversality,
                ..
            })
        )
    }

    fn zero_crossing(got: &Result<RowContact, CertifyError>) -> bool {
        matches!(
            got,
            Err(CertifyError::NotTransverse {
                verdict: Refused::Zero(_),
                ..
            })
        )
    }

    /// A touch point at one of the reading's three points refuses as a
    /// zero crossing: `(−0.5, 1.0)` reads at `−0.5`, `0` and `0.5`.
    #[test]
    fn a_row_that_touches_at_a_reading_refuses_as_a_zero_crossing() {
        let lever = geom_core::test_support::fixture_lever(1.0);
        let (na, nb) = touching_at_zero::<f64>(0.6);
        let got = contact_of(na, nb, [true; 2], (-0.5, 1.0), lever, band());
        assert!(zero_crossing(&got), "f64: {got:?}");
        let lift = Interval::from_f64;
        let lever = geom_core::test_support::fixture_lever(lift(1.0));
        let (na, nb) = touching_at_zero::<Interval>(0.6);
        let got = contact_of(na, nb, [true; 2], (lift(-0.5), lift(1.0)), lever, band());
        assert!(zero_crossing(&got), "Interval: {got:?}");
    }

    /// The defect the whole-row reading exists to keep dead: a row whose
    /// surfaces touch at an end, or between any sampled feet, never reads
    /// as crossing. The edge certificate's interior feet on these spans all
    /// read a definite crossing angle. The reading refuses each: as a zero
    /// crossing where one of its readings (`t₀`, `t₀ + d`, `t₀ + 2d`) lands
    /// on the touch, the start here, and as an escalation of the crossing
    /// bound otherwise.
    #[test]
    fn a_row_touching_off_its_feet_never_reads_as_crossing() {
        let lever = geom_core::test_support::fixture_lever(1.0);
        let pi = core::f64::consts::PI;
        for (what, span, touch_read) in [
            ("at its start", (0.0, pi / 2.0), true),
            ("at its end", (-pi / 2.0, 0.0), false),
            ("between feet", (-0.3, 1.1), false),
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
            let got = contact_of(na, nb, [true; 2], span, lever, band());
            if touch_read {
                assert!(zero_crossing(&got), "{what}: a zero crossing, not {got:?}");
            } else {
                assert!(
                    escalated(&got),
                    "{what}: the crossing bound escalates, not {got:?}"
                );
            }
        }
    }

    #[test]
    fn coincident_normals_read_as_touching_either_way_round() {
        let lever = geom_core::test_support::fixture_lever(1.0);
        let span = (0.0, core::f64::consts::FRAC_PI_2);
        let same = |t: f64| Vec3::new(t.cos(), t.sin(), 0.0);
        let flipped = |t: f64| -same(t);
        for nb in [&same as &dyn Fn(f64) -> Vec3<f64>, &flipped] {
            let got = contact_of(same, nb, [true; 2], span, lever, band());
            assert_eq!(
                got.unwrap(),
                RowContact::Touching,
                "either orientation touches"
            );
        }
    }

    /// A constant angle is charged no variation: a degree over a full turn
    /// and a milliradian over a quarter turn both read as crossing.
    #[test]
    fn a_shallow_constant_angle_reads_as_crossing() {
        let lever = geom_core::test_support::fixture_lever(1.0);
        let tau = core::f64::consts::TAU;
        for (angle, span) in [(1f64.to_radians(), (0.0, tau)), (1e-3, (0.0, tau / 4.0))] {
            let (na, nb) = at_constant_angle(angle);
            let got = contact_of(na, nb, [true; 2], span, lever, band());
            assert_eq!(
                got.unwrap(),
                RowContact::Crossing,
                "{angle} rad over {span:?}"
            );
        }
    }

    /// The premise every bound here stands on: along each admitted row of
    /// each admitted chart, clear of its singular points, the unit normal
    /// is the first harmonic three readings in the span give. Checked at
    /// points of the row outside the span, on both row directions of
    /// every chart.
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
                let read = harmonic_row(&image, (0.2, 1.1), band());
                let (Ok(WallRow::Analytic), Ok(Some(row))) = (wall_row(chart, &image), read) else {
                    panic!("{:?} admits {image:?}", chart.kind());
                };
                let normal = |t: f64| row_normal(chart, &image, t);
                let (t0, d) = (0.2, 0.3);
                let at = [t0, t0 + d, t0 + 2.0 * d].map(normal);
                let h = if row.turning {
                    Harmonic::of_turning(at, d)
                } else {
                    Harmonic::constant(at[1])
                };
                for k in 0..16 {
                    let t = 0.37 + 0.41 * f64::from(k);
                    let tau = t - (t0 + d);
                    let fit = h.a + h.b * tau.cos() + h.c * tau.sin();
                    assert!(
                        (fit - normal(t)).norm() < 1e-11,
                        "{:?} along {image:?} at t = {t}: {fit:?} against {:?}",
                        chart.kind(),
                        normal(t)
                    );
                }
            }
        }
    }
}
