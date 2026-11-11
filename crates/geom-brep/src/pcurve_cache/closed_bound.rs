//! **A stated closed-form image's miss from its carrier, bounded over the
//! span by the coefficients of the residual**: the door
//! [`super::StatedDoor::Closed`] names.
//!
//! The residual `R(t) = C(t) − S(P(t))` of a harmonic image against a line
//! or conic carrier is a trigonometric polynomial ([`super::trig`]): the
//! carrier is `c + a·cos t + b·sin t + l·t`; a free channel is the same
//! scalar form, an angle channel `φ + w·t` enters through its cosine and
//! sine, and every analytic chart is `O + radial(u)·ρ(v) + axis·h(v)` (a
//! plane `O + u·u_ref + v·v_ref`) with `ρ` and `h` polynomials in `v` (a
//! cylinder or cone) or in `cos v`, `sin v` (a sphere, torus or spindle).
//! So `R` is written exactly in `sʲ·cos ks`, `sʲ·sin ks`, `s = t − t_m`
//! about the span's midpoint, and over `|s| ≤ h`
//!
//! ```text
//! sup |R(t)| ≤ Σₖ Σⱼ hʲ · sup_s |cₖⱼ·cos ks + dₖⱼ·sin ks|.
//! ```
//!
//! Nothing is sampled and nothing re-derives the image: the carrier is
//! read in its own coefficients and the image's angles are read as
//! angles, so an image a whole period away, a mirrored winding against a
//! mirrored frame and a sphere point's other name are each the same
//! function, and their residual is rounding.
//!
//! Two closed forms leave the family by one function each, and are
//! brought back into it:
//!
//! - **A spiric** carries `f(t) = √((R + r·cos t)² − d²)`. On its plane
//!   the cap image is affine in `{1, f, sin t}`, and so is its carrier.
//!   On a torus the wall image's azimuth is `u₀ + κ·θ`, `θ = atan2(f, d)`,
//!   whose cosine and sine are `d/ρ` and `f/ρ` with `ρ = R + r·cos t > 0`,
//!   so `ρ·R(t)` is in the family with `f` as one more factor:
//!   `ρ·R = A(s) + f·B(s)`, and `|R| ≤ (sup|A| + sup f·sup|B|)/(R − r)`.
//! - **A focal section**'s azimuth is `u₀ + κ·ν`, `ν = E + 2λ`,
//!   `λ = atan2(β·sin E, 1 − β·cos E)`, whose double angle has cosine and
//!   sine `((1 − β cos E)² − β² sin² E)/q²` and `2β sin E (1 − β cos E)/q²`
//!   with `q² = 1 − 2β·cos E + β² ≥ (1 − |β|)²`, so `q²·R(t)` is in the
//!   family, and `|R| ≤ sup|q²·R|/(1 − |β|)²`.
//!
//! The bound is a sum of upper bounds and locates no point. Where it does
//! not clear the band, the miss is read at a grid over the span fine
//! enough for the residual's highest frequency, and a point past the band
//! there is a located miss ([`PcurveCertifyError::MissLocated`]).

use geom::{Curve3, Frame, Surface};
use geom_core::k_stats::decide_reported;
use geom_core::predicate::{Band, Decided, Sign};
use geom_core::{Decide, Margin, Real, Vec3};

use super::trig::{Trig, Trig3};
use super::{Channel, EdgeSpan, FocalImage, PcurveCertifyError, PcurveCheck, SpiricImage};

/// The largest winding a channel's frequency is read at. A closed-form
/// table holds one turn per carrier period; past this the image is not a
/// closed form any construction states.
const MAX_WINDING: i32 = 64;

/// Grid points per unit of the residual's highest frequency per radian
/// of span, for the search that locates a miss: at this density the
/// grid's nearest point to the peak of a frequency-`k` term reads it at
/// `cos(π·k/(2·GRID_DENSITY·k))`, `> 0.99` of its height.
const GRID_DENSITY: f64 = 16.0;

/// The most points the search reads.
const GRID_CAP: usize = 4096;

/// **The residual of a harmonic image `(u, v)` on `surface` against
/// `carrier`**, in the centred parameter of `span`, with the half-width
/// its bound reads at.
///
/// # Errors
///
/// The image's structure read wrong: a channel kind the chart's coordinate
/// is not ([`PcurveCertifyError::ChannelKindMismatch`]), a winding past
/// [`MAX_WINDING`] ([`PcurveCertifyError::ChartWindingUnsupported`]), a
/// carrier with no `{1, cos, sin, t}` form
/// ([`PcurveCertifyError::ImageMismatch`]), a spline chart
/// ([`PcurveCertifyError::UnsupportedChart`]).
pub(super) fn harmonic_residual<T: Real>(
    (u, v): (Channel<T>, Channel<T>),
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    span: EdgeSpan<T>,
) -> Result<Residual<T>, PcurveCertifyError> {
    let chart = surface.kind();
    let Some((period_u, period_v)) = surface.angle_periods() else {
        return Err(PcurveCertifyError::UnsupportedChart { chart });
    };
    if (u.winding().is_some(), v.winding().is_some()) != (period_u.is_some(), period_v.is_some()) {
        return Err(PcurveCertifyError::ChannelKindMismatch { chart });
    }
    if [u.winding(), v.winding()]
        .into_iter()
        .flatten()
        .any(|w| w.abs() > MAX_WINDING)
    {
        return Err(PcurveCertifyError::ChartWindingUnsupported);
    }
    let Some(form) = super::carrier_harmonic(carrier) else {
        return Err(PcurveCertifyError::ImageMismatch {
            image: super::PcurveKind::Harmonic,
            why: "the carrier has no {1, cos, sin, t} form, so no harmonic image is its own",
        });
    };
    let (t_m, h) = centre(span);
    let carrier = Trig3::harmonic(
        (form.c - geom_core::Point3::origin(), form.a, form.b, form.l),
        t_m,
    );
    let image = chart_image(surface, (u, v), t_m)?;
    Ok(Residual {
        plain: carrier.minus(&image),
        factors: Vec::new(),
        floor: T::one(),
        h,
    })
}

/// **A residual brought into the family**: `m(t)·R(t) = plain(s) +
/// Σ gᵢ(t)·Fᵢ(s)` with `m(t) ≥ floor > 0` and `|gᵢ(t)| ≤ sup gᵢ` over the
/// span, so `sup |R| ≤ (sup|plain| + Σ sup gᵢ·sup|Fᵢ|)/floor`.
pub(super) struct Residual<T> {
    plain: Trig3<T>,
    /// `(sup gᵢ, Fᵢ)`.
    factors: Vec<(T, Trig3<T>)>,
    floor: T,
    /// The span's half-width about its midpoint.
    h: T,
}

impl<T: Real> Residual<T> {
    /// The bound on `sup |R|` over the span.
    fn bound(&self) -> T {
        let sum = self
            .factors
            .iter()
            .fold(self.plain.sup(self.h), |acc, (g, f)| {
                acc + *g * f.sup(self.h)
            });
        sum / self.floor
    }

    /// The highest frequency of any part, plus one where a factor `g`
    /// moves with the parameter too.
    fn frequency(&self) -> usize {
        let f = |x: &Trig3<T>| x.frequencies().saturating_sub(1);
        self.factors
            .iter()
            .map(|(_, x)| f(x) + 1)
            .fold(f(&self.plain), usize::max)
    }
}

/// `x`'s sign where `x` is exactly `±1`.
fn unit<T: Real>(x: T) -> Option<i32> {
    if geom_core::is_exact_zero(x - T::one()) {
        Some(1)
    } else if geom_core::is_exact_zero(x + T::one()) {
        Some(-1)
    } else {
        None
    }
}

/// The radial and tangential directions of a frame at the angle `u0`.
fn radial_pair<T: Real>(frame: &Frame<T>, u0: T) -> (Vec3<T>, Vec3<T>) {
    (frame.radial(u0), frame.tangential(u0))
}

/// **The residual of a spiric image against its carrier** (module docs):
/// the cap on its plane, the wall on a torus.
///
/// # Errors
///
/// A carrier that is no spiric or an image on a chart that holds no
/// spiric image ([`PcurveCertifyError::ImageMismatch`],
/// [`PcurveCertifyError::UnsupportedChart`]); a wall's sense that is not
/// exactly `±1` ([`PcurveCertifyError::ImageMismatch`]).
fn spiric_residual<T: Real>(
    (major, minor, offset, image): (T, T, T, SpiricImage<T>),
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    span: EdgeSpan<T>,
) -> Result<Residual<T>, PcurveCertifyError> {
    let Curve3::Spiric {
        center,
        axis: c_axis,
        u_ref: n,
        major_radius: c_major,
        minor_radius: c_minor,
        offset: c_offset,
    } = *carrier
    else {
        return Err(PcurveCertifyError::ImageMismatch {
            image: super::PcurveKind::Spiric,
            why: "the carrier is no spiric, so no spiric image is its own",
        });
    };
    let (t_m, h) = centre(span);
    let origin = geom_core::Point3::origin();
    let m = c_axis.cross(n);
    // The carrier: `c + n·d + m·f_c(t) + axis·r·sin t`.
    let sin_t = Trig::free([T::zero(), T::zero(), T::one(), T::zero()], t_m);
    let carrier_plain = Trig3::constant(center + n * c_offset - origin)
        .plus(&Trig3::along(&sin_t.scaled(c_minor), c_axis));
    let f_sup = |big: T, small: T, d: T| geom::spiric_f_range(big, small, d).1;
    // One `f` where the image's copies are the carrier's scalars, two
    // factors where they are not.
    let same = [major - c_major, minor - c_minor, offset - c_offset]
        .into_iter()
        .all(geom_core::is_exact_zero);
    let factors = |carrier_f: Trig3<T>, image_f: Trig3<T>| {
        if same {
            vec![(f_sup(major, minor, offset), carrier_f.minus(&image_f))]
        } else {
            vec![
                (f_sup(c_major, c_minor, c_offset), carrier_f),
                (f_sup(major, minor, offset), image_f.scaled_by(-T::one())),
            ]
        }
    };
    match (image, surface) {
        (
            SpiricImage::Cap { p0, pm, pa },
            &Surface::Plane {
                origin: o,
                normal,
                u_ref,
            },
        ) => {
            let v_ref = normal.cross(u_ref);
            let chart = |p: geom_core::Vec2<T>| u_ref * p.x + v_ref * p.y;
            let image_plain = Trig3::constant(o - origin + chart(p0 - geom_core::Point2::origin()))
                .plus(&Trig3::along(&sin_t, chart(pa)));
            Ok(Residual {
                plain: carrier_plain.minus(&image_plain),
                factors: factors(Trig3::constant(m), Trig3::constant(chart(pm))),
                floor: T::one(),
                h,
            })
        }
        (
            SpiricImage::Wall {
                u0,
                v0,
                sense,
                torus,
            },
            &Surface::Torus {
                center: c_s,
                axis: a_s,
                major_radius: big_s,
                minor_radius: small_s,
                u_ref,
            },
        ) => {
            let Some(sigma) = unit(sense) else {
                return Err(PcurveCertifyError::ImageMismatch {
                    image: super::PcurveKind::Spiric,
                    why: "the wall image's sense is not a unit sign",
                });
            };
            let kappa = sense * torus.mu::<T>();
            // `ρ_i = R + r·cos t`, the image's own, positive on the ring.
            let rho = Trig::free([major, minor, T::zero(), T::zero()], t_m);
            let (cos_v, sin_v) = Trig::angle(v0, sigma, t_m);
            let (a0, b0) = radial_pair(&Frame::new(c_s, a_s, u_ref), u0);
            // `ρ·radial(u) = a₀·d + κ·b₀·f`, times the tube's `R_s + r_s·cos v`.
            let tube = Trig::constant(big_s).plus(&cos_v.scaled(small_s));
            let image_plain = Trig3::constant(c_s - origin)
                .times(&rho)
                .plus(&Trig3::along(&sin_v.times(&rho).scaled(small_s), a_s))
                .plus(&Trig3::along(&tube, a0 * offset));
            Ok(Residual {
                plain: carrier_plain.times(&rho).minus(&image_plain),
                factors: factors(Trig3::along(&rho, m), Trig3::along(&tube, b0 * kappa)),
                floor: major - minor,
                h,
            })
        }
        _ => Err(PcurveCertifyError::UnsupportedChart {
            chart: surface.kind(),
        }),
    }
}

/// **The residual of a focal-section image against its carrier**
/// (module docs): on a cone, its slant a free channel; on a torus, its
/// tube angle `v₀ + w·t`.
///
/// # Errors
///
/// A carrier with no `{1, cos, sin, t}` form, a sense that is not
/// exactly `±1`, or a tube angle that is no whole winding
/// ([`PcurveCertifyError::ImageMismatch`]); a chart that holds no focal
/// section ([`PcurveCertifyError::UnsupportedChart`]).
fn focal_residual<T: Real>(
    image: FocalImage<T>,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    span: EdgeSpan<T>,
) -> Result<Residual<T>, PcurveCertifyError> {
    let mismatch = |why| PcurveCertifyError::ImageMismatch {
        image: super::PcurveKind::FocalSection,
        why,
    };
    let Some(form) = super::carrier_harmonic(carrier) else {
        return Err(mismatch("the carrier has no {1, cos, sin, t} form"));
    };
    let Some(_) = unit(image.sense) else {
        return Err(mismatch("the image's sense is not a unit sign"));
    };
    let (t_m, h) = centre(span);
    let origin = geom_core::Point3::origin();
    let beta = image.beta;
    let one = Trig::constant(T::one());
    // `E = t − t₀`.
    let (cos_e, sin_e) = Trig::angle(-image.t0, 1, t_m);
    let a = one.minus(&cos_e.scaled(beta));
    let b = sin_e.scaled(beta);
    let (c2, s2) = (
        a.times(&a).minus(&b.times(&b)),
        a.times(&b).scaled(T::from_f64(2.0)),
    );
    // `q²·cos ν`, `q²·sin ν`, `ν = E + 2λ`.
    let n_c = cos_e.times(&c2).minus(&sin_e.times(&s2));
    let n_s = sin_e.times(&c2).plus(&cos_e.times(&s2));
    let q2 = Trig::constant(T::one() + beta * beta).minus(&cos_e.scaled(beta + beta));
    let carrier = Trig3::harmonic((form.c - origin, form.a, form.b, form.l), t_m).times(&q2);
    let along = |frame: Frame<T>| {
        let (a0, b0) = radial_pair(&frame, image.u0);
        Trig3::along(&n_c, a0).plus(&Trig3::along(&n_s, b0 * image.sense))
    };
    let mapped = match *surface {
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let (sin_a, cos_a) = half_angle.sin_cos();
            let slant = Trig::free([image.v0, image.va, image.vb, image.vl], t_m);
            Trig3::constant(apex - origin)
                .times(&q2)
                .plus(&Trig3::along(&slant.times(&q2).scaled(cos_a), axis))
                .plus(&along(Frame::new(apex, axis, u_ref)).times(&slant.scaled(sin_a)))
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            let whole = [image.va, image.vb]
                .into_iter()
                .all(geom_core::is_exact_zero);
            let Some(w) = unit(image.vl).filter(|_| whole) else {
                return Err(mismatch("the image's tube angle is not one whole winding"));
            };
            let (cos_v, sin_v) = Trig::angle(image.v0, w, t_m);
            let tube = Trig::constant(major_radius).plus(&cos_v.scaled(minor_radius));
            Trig3::constant(center - origin)
                .times(&q2)
                .plus(&Trig3::along(&sin_v.times(&q2).scaled(minor_radius), axis))
                .plus(&along(Frame::new(center, axis, u_ref)).times(&tube))
        }
        _ => {
            return Err(PcurveCertifyError::UnsupportedChart {
                chart: surface.kind(),
            });
        }
    };
    let gap = T::one() - beta.abs();
    Ok(Residual {
        plain: carrier.minus(&mapped),
        factors: Vec::new(),
        floor: gap * gap,
        h,
    })
}

/// The span's midpoint `t_m` and a half-width `h` with every `t` the
/// interval admits inside `[t_m − h, t_m + h]`: both read off the span's
/// outer ends, `f64` structure (C6), `h` rounded outward.
fn centre<T: Real>(span: EdgeSpan<T>) -> (T, T) {
    let (lo, hi) = span.outer;
    let mid = lo + (hi - lo) * 0.5;
    let h = (hi - mid).max(mid - lo).next_up();
    (T::from_f64(mid), T::from_f64(h))
}

/// The scalar a free channel is, or the cosine and sine an angle channel
/// enters through.
enum Read<T> {
    Free(Trig<T>),
    Angle(Trig<T>, Trig<T>),
}

fn read<T: Real>(channel: Channel<T>, t_m: T) -> Read<T> {
    match channel {
        Channel::Free { .. } => Read::Free(Trig::free(channel.coefficients(), t_m)),
        Channel::Angle { phase, winding } => {
            let (cos, sin) = Trig::angle(phase, winding, t_m);
            Read::Angle(cos, sin)
        }
    }
}

/// **`S(P(t))` for a harmonic image**, exactly, in the family. The channel
/// kinds are the chart's (checked by the caller), so each arm reads the
/// kinds its chart has.
fn chart_image<T: Real>(
    surface: &Surface<T>,
    (u, v): (Channel<T>, Channel<T>),
    t_m: T,
) -> Result<Trig3<T>, PcurveCertifyError> {
    let mismatch = || PcurveCertifyError::ChannelKindMismatch {
        chart: surface.kind(),
    };
    let (u, v) = (read(u, t_m), read(v, t_m));
    // `radial(u) = u_ref·cos u + v_ref·sin u`, the charts' own frame
    // (`Frame::radial`).
    let radial = |frame: Frame<T>| match &u {
        Read::Angle(cos, sin) => {
            Ok(Trig3::along(cos, frame.u_ref).plus(&Trig3::along(sin, frame.v_ref())))
        }
        Read::Free(_) => Err(mismatch()),
    };
    let free = |x: &Read<T>| match x {
        Read::Free(x) => Ok(x.clone()),
        Read::Angle(..) => Err(mismatch()),
    };
    let angle = |x: &Read<T>| match x {
        Read::Angle(cos, sin) => Ok((cos.clone(), sin.clone())),
        Read::Free(_) => Err(mismatch()),
    };
    Ok(match *surface {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => {
            let v_ref = normal.cross(u_ref);
            Trig3::constant(origin - geom_core::Point3::origin())
                .plus(&Trig3::along(&free(&u)?, u_ref))
                .plus(&Trig3::along(&free(&v)?, v_ref))
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => Trig3::constant(origin - geom_core::Point3::origin())
            .plus(&radial(Frame::new(origin, axis, u_ref))?.times(&Trig::constant(radius)))
            .plus(&Trig3::along(&free(&v)?, axis)),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let (sin_a, cos_a) = half_angle.sin_cos();
            let slant = free(&v)?;
            Trig3::constant(apex - geom_core::Point3::origin())
                .plus(&Trig3::along(&slant.scaled(cos_a), axis))
                .plus(&radial(Frame::new(apex, axis, u_ref))?.times(&slant.scaled(sin_a)))
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => {
            let (cos_v, sin_v) = angle(&v)?;
            Trig3::constant(center - geom_core::Point3::origin())
                .plus(&radial(Frame::new(center, axis, u_ref))?.times(&cos_v.scaled(radius)))
                .plus(&Trig3::along(&sin_v.scaled(radius), axis))
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        }
        | Surface::Spindle {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            let (cos_v, sin_v) = angle(&v)?;
            let rho = Trig::constant(major_radius).plus(&cos_v.scaled(minor_radius));
            Trig3::constant(center - geom_core::Point3::origin())
                .plus(&radial(Frame::new(center, axis, u_ref))?.times(&rho))
                .plus(&Trig3::along(&sin_v.scaled(minor_radius), axis))
        }
        Surface::Nurbs(_) | Surface::Approx(_) => {
            return Err(PcurveCertifyError::UnsupportedChart {
                chart: surface.kind(),
            });
        }
    })
}

/// **The bound's verdict**: `Zero` certifies, and the bound is the
/// certificate's envelope. Otherwise the miss is read at a grid over the
/// span: a point past the band is a located miss; else a bound past the
/// band is [`PcurveCertifyError::ResidualExceeded`] at
/// [`PcurveCheck::Envelope`] (nothing located, a sum of upper bounds), and
/// one in band [`PcurveCertifyError::Escalated`] there.
///
/// `frequency` is the residual's highest frequency, which sets the grid;
/// `miss` is the meter `|C(t) − S(P(t))|` at a point.
pub(super) fn verdict<T: Decide>(
    bound: T,
    frequency: usize,
    span: EdgeSpan<T>,
    band: Band,
    miss: impl Fn(T) -> T,
) -> Result<T, PcurveCertifyError> {
    let refused = match geom_core::k_stats::decide("pcurve_closed_bound", Margin::of(bound), band) {
        Ok(Sign::Zero) => return Ok(bound),
        refused => refused,
    };
    let (lo, hi) = span.outer;
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    let points = {
        let per_radian = GRID_DENSITY * (frequency.max(1) as f64);
        (((hi - lo) * per_radian).ceil() as usize).clamp(2, GRID_CAP)
    };
    for i in 0..=points {
        #[allow(clippy::cast_precision_loss)]
        let t = lo + (hi - lo) * (i as f64 / points as f64);
        if let Ok(Decided {
            sign: Sign::Positive | Sign::Negative,
            margin,
        }) = decide_reported("pcurve_closed_miss", Margin::of(miss(T::from_f64(t))), band)
        {
            return Err(PcurveCertifyError::MissLocated { margin });
        }
    }
    Err(match refused {
        Err(cause) => PcurveCertifyError::Escalated {
            check: PcurveCheck::Envelope,
            sample: 0,
            cause,
        },
        Ok(_) => PcurveCertifyError::ResidualExceeded {
            check: PcurveCheck::Envelope,
            sample: 0,
        },
    })
}

/// **A closed-form image in the form this door reads**, decided once by
/// the route ([`super::StatedDoor::of`]).
#[derive(Clone, Copy, Debug)]
pub(crate) enum ClosedForm<T: Real> {
    /// A [`super::Pcurve::Harmonic`] image's two channels.
    Harmonic(Channel<T>, Channel<T>),
    /// A [`super::Pcurve::Spiric`] image: its copies of the carrier's
    /// `R`, `r`, `d` and its chart image.
    Spiric(T, T, T, SpiricImage<T>),
    /// A [`super::Pcurve::FocalSection`] image.
    Focal(FocalImage<T>),
}

impl<T: Real> ClosedForm<T> {
    /// The form of `image`, where this door writes its residual.
    pub(super) fn of(image: &super::Pcurve<T>) -> Option<Self> {
        match *image {
            super::Pcurve::Harmonic { u, v } => Some(Self::Harmonic(u, v)),
            super::Pcurve::Spiric {
                major,
                minor,
                offset,
                image,
            } => Some(Self::Spiric(major, minor, offset, image)),
            super::Pcurve::FocalSection(image) => Some(Self::Focal(image)),
            super::Pcurve::IsoLine { .. }
            | super::Pcurve::IsoArc { .. }
            | super::Pcurve::Fitted(_)
            | super::Pcurve::General(_)
            | super::Pcurve::Projected(_) => None,
        }
    }
}

/// **The certificate of a closed-form `image` stated over `span`**: the
/// residual's coefficient bound over the span ([`Trig3::sup`]), decided
/// by [`verdict`]. The carrier's incidence on its chart is the caller's
/// to read first.
///
/// # Errors
///
/// The residual's structural refusals ([`harmonic_residual`]) and the
/// verdict's.
pub(super) fn certify<T: Decide>(
    (image, form): (&super::Pcurve<T>, ClosedForm<T>),
    span: EdgeSpan<T>,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<super::PcurveCertificate<T>, PcurveCertifyError> {
    let residual = match form {
        ClosedForm::Harmonic(u, v) => harmonic_residual((u, v), carrier, surface, span)?,
        ClosedForm::Spiric(major, minor, offset, image) => {
            spiric_residual((major, minor, offset, image), carrier, surface, span)?
        }
        ClosedForm::Focal(image) => focal_residual(image, carrier, surface, span)?,
    };
    let miss = |t: T| {
        let p = image.eval(t);
        carrier.eval(t).distance(surface.eval(p.x, p.y))
    };
    let bound = verdict(residual.bound(), residual.frequency(), span, band, miss)?;
    Ok(super::PcurveCertificate::closed(
        0,
        T::zero(),
        bound,
        super::EnvelopeStatement::MapResidualClosedForm,
    ))
}
