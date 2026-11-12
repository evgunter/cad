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
use super::{
    Channel, EdgeSpan, FocalImage, PcurveCertifyError, PcurveCheck, PcurveKind, SpiricImage,
};

/// Grid points per unit of the residual's highest frequency per radian
/// of span, for the search that locates a miss: at this density the
/// grid's nearest point to the peak of a frequency-`k` term is within
/// `1/(2·GRID_DENSITY·k)` radians of it in `k·t`, and reads it at
/// `cos(1/(2·GRID_DENSITY)) > 0.99` of its height. Past [`GRID_CAP`]
/// points (a span longer than `GRID_CAP/(GRID_DENSITY·k)`) the grid is
/// coarser and that fraction no longer holds; a miss the grid does not
/// locate ends unsettled, the lever, and is never certified.
const GRID_DENSITY: f64 = 16.0;

/// The most points the search reads.
const GRID_CAP: usize = 4096;

/// **The residual of a harmonic image `(u, v)` on `surface` against
/// `carrier`**, in the centred parameter of `span`, with the half-width
/// its bound reads at.
///
/// # Errors
///
/// The image's structure read wrong, its own defect ending: a channel
/// kind the chart's coordinate is not
/// ([`PcurveCertifyError::ChannelKindMismatch`]); a winding past the
/// family's reach or a carrier with no `{1, cos, sin, t}` form
/// ([`PcurveCertifyError::ImageMismatch`]).
pub(super) fn harmonic_residual<T: Real>(
    (u, v): (Channel<T>, Channel<T>),
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    span: EdgeSpan<T>,
) -> Result<Residual<T>, PcurveCertifyError> {
    let chart = surface.kind();
    let Some((period_u, period_v)) = surface.angle_periods() else {
        unreachable!("harmonic_residual: a closed form takes this door on an analytic chart only")
    };
    if (u.winding().is_some(), v.winding().is_some()) != (period_u.is_some(), period_v.is_some()) {
        return Err(PcurveCertifyError::ChannelKindMismatch { chart });
    }
    let Some(form) = super::carrier_harmonic(carrier) else {
        return Err(unheld(
            PcurveKind::Harmonic,
            "the carrier has no {1, cos, sin, t} form, so no harmonic image is its own",
        ));
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
        slack: T::zero(),
        h,
    })
}

/// **A residual brought into the family**: `m(t)·R(t) = plain(s) +
/// Σ gᵢ(t)·Fᵢ(s) + E(t)·m(t)` with `m(t) ≥ floor > 0`, `|gᵢ(t)| ≤ sup gᵢ`
/// over the span and `|E(t)| ≤ slack`, so
/// `sup |R| ≤ (sup|plain| + Σ sup gᵢ·sup|Fᵢ|)/floor + slack`. Every
/// builder decides its floor positive before it writes one, so the
/// division is by a positive number.
pub(super) struct Residual<T> {
    plain: Trig3<T>,
    /// `(sup gᵢ, Fᵢ)`.
    factors: Vec<(T, Trig3<T>)>,
    floor: T,
    /// A displacement the stored image makes beyond the family's image
    /// of it (a unit sign's or a whole rate's residue), bounded.
    slack: T,
    /// The span's half-width about its midpoint.
    h: T,
}

impl<T: Real> Residual<T> {
    /// The bound on `sup |R|` over the span.
    fn bound(&self) -> T {
        let h = self.h;
        #[cfg(test)]
        let h = if super::mutants::closed_sup_half_span() {
            h * T::from_f64(0.5)
        } else {
            h
        };
        let sum = self
            .factors
            .iter()
            .fold(self.plain.sup(h), |acc, (g, f)| acc + *g * f.sup(h));
        sum / self.floor + self.slack
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

/// The radial and tangential directions of a frame at the angle `u0`.
fn radial_pair<T: Real>(frame: &Frame<T>, u0: T) -> (Vec3<T>, Vec3<T>) {
    (frame.radial(u0), frame.tangential(u0))
}

/// A closed form's structure read wrong: its own defect ending, never a
/// lane's gap.
fn unheld(image: PcurveKind, why: &'static str) -> PcurveCertifyError {
    PcurveCertifyError::ImageMismatch { image, why }
}

/// An identity of the image's structure the band could not decide.
fn identity_escalated(cause: geom_core::Indeterminate) -> PcurveCertifyError {
    PcurveCertifyError::Escalated {
        check: PcurveCheck::ImageIdentity,
        sample: 0,
        cause,
    }
}

/// **The whole winding `σ = ±1` of a unit sign** whose residue `|x| − 1`
/// a door has decided `Zero`: the sign of `x`, decided. `|x|` is one to
/// within the band, so only a band past a metre leaves it undecided.
fn unit_winding<T: Decide>(
    name: &'static str,
    x: T,
    image: PcurveKind,
    band: Band,
) -> Result<(i32, T), PcurveCertifyError> {
    match geom_core::k_stats::decide(name, Margin::of(x), band).map_err(identity_escalated)? {
        Sign::Positive => Ok((1, T::one())),
        Sign::Negative => Ok((-1, -T::one())),
        Sign::Zero => Err(unheld(image, "a unit sign reads zero")),
    }
}

/// `ρ = R + r·cos t ≥ R − r` on a ring, the floor of a spiric wall's
/// multiplier.
pub(super) fn spiric_floor<T: Real>(major: T, minor: T) -> T {
    #[cfg(test)]
    if super::mutants::spiric_floor_major() {
        return major;
    }
    major - minor
}

/// `q² = 1 − 2β·cos E + β² ≥ (1 − |β|)²`, the floor of a focal section's
/// multiplier.
pub(super) fn focal_floor<T: Real>(beta: T) -> T {
    let gap = T::one() - beta.abs();
    #[cfg(test)]
    if super::mutants::focal_floor_single() {
        return gap;
    }
    gap.powi(2)
}

/// **A spiric's torus is a ring cut in two ovals**, decided: `r > 0` and
/// `R − r − |d| > 0`, so `ρ ≥ R − r > 0` and `f ≥ √((R − r)² − d²) > 0`
/// over every `t`, the floors the residual divides by.
fn two_ovals<T: Decide>((big, small, d): (T, T, T), band: Band) -> Result<(), PcurveCertifyError> {
    for (name, margin) in [
        ("pcurve_closed_spiric_tube", small),
        ("pcurve_closed_spiric_ring", big - small - d.abs()),
    ] {
        match geom_core::k_stats::decide(name, Margin::of(margin), band)
            .map_err(identity_escalated)?
        {
            Sign::Positive => {}
            Sign::Zero | Sign::Negative => {
                return Err(unheld(
                    PcurveKind::Spiric,
                    "the spiric's torus is no ring cut in two ovals, so its f and ρ have no \
                     positive floor",
                ));
            }
        }
    }
    Ok(())
}

/// **The residual of a spiric image against its carrier** (module docs):
/// the cap on its plane, the wall on a torus. The image's copies of the
/// carrier's scalars are decided in band first ([`super::spiric_copies`]),
/// and the residual reads each side's `f` on its own scalars: `f_c` the
/// carrier's, `f_i` the image's, their difference
/// `|f_i − f_c| = |f_i² − f_c²|/(f_i + f_c)` bounded by the scalars'
/// differences over the two `f` floors.
///
/// # Errors
///
/// The image's structure read wrong ([`PcurveCertifyError::ImageMismatch`]):
/// a carrier that is no spiric, a chart that holds no spiric image of
/// this kind, copies past the band, a ring that is not cut in two ovals,
/// a wall's sense that is no unit sign; or one of those in band
/// ([`PcurveCertifyError::Escalated`] at [`PcurveCheck::ImageIdentity`]).
fn spiric_residual<T: Decide>(
    (major, minor, offset, image): (T, T, T, SpiricImage<T>),
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    span: EdgeSpan<T>,
    band: Band,
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
        return Err(unheld(
            PcurveKind::Spiric,
            "the carrier is no spiric, so no spiric image is its own",
        ));
    };
    super::spiric_copies((major, minor, offset), (c_major, c_minor, c_offset), band)?;
    two_ovals((major, minor, offset), band)?;
    two_ovals((c_major, c_minor, c_offset), band)?;
    let (t_m, h) = centre(span);
    let origin = geom_core::Point3::origin();
    let m = c_axis.cross(n);
    // The carrier: `c + n·d + m·f_c(t) + axis·r·sin t`.
    let sin_t = Trig::free([T::zero(), T::zero(), T::one(), T::zero()], t_m);
    let carrier_plain = Trig3::constant(center + n * c_offset - origin)
        .plus(&Trig3::along(&sin_t.scaled(c_minor), c_axis));
    let (lo_i, _) = geom::spiric_f_range(major, minor, offset);
    let (lo_c, hi_c) = geom::spiric_f_range(c_major, c_minor, c_offset);
    let delta_f = ((major - c_major).abs() + (minor - c_minor).abs())
        * (major + c_major + minor + c_minor)
        + (offset - c_offset).abs() * (offset.abs() + c_offset.abs());
    let delta_f = delta_f / (lo_i + lo_c);
    // `R = plain + f_c·(F_c − F_i) + (f_c − f_i)·F_i`, `F_c` and `F_i`
    // the two sides' `f` coefficients.
    let factors = |f_carrier: Trig3<T>, f_image: Trig3<T>| {
        vec![(hi_c, f_carrier.minus(&f_image)), (delta_f, f_image)]
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
                slack: T::zero(),
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
            let reach = t_m.abs() + h;
            let residue = super::spiric_sense(sense, surface, reach, band)?;
            let (sigma, s) = unit_winding(
                "pcurve_closed_spiric_sense",
                sense,
                PcurveKind::Spiric,
                band,
            )?;
            let kappa = s * torus.mu::<T>();
            // `ρ_i = R + r·cos t`, the image's own, `≥ R − r > 0`.
            let rho = Trig::free([major, minor, T::zero(), T::zero()], t_m);
            let (cos_v, sin_v) = Trig::angle(v0, sigma, t_m).ok_or_else(winding_past)?;
            let (a0, b0) = radial_pair(&Frame::new(c_s, a_s, u_ref), u0);
            // `ρ·radial(u) = a₀·d + κ·b₀·f`, times the tube's `R_s + r_s·cos v`.
            let tube = Trig::constant(big_s).plus(&cos_v.scaled(small_s));
            let image_plain = Trig3::constant(c_s - origin)
                .times(&rho)
                .plus(&Trig3::along(&sin_v.times(&rho).scaled(small_s), a_s))
                .plus(&Trig3::along(&tube, a0 * offset));
            // The stored sense's residue from `σ` moves the azimuth by at
            // most `|residue|·π` and the tube angle by `|residue|·|t|`.
            let arm_u = big_s.abs() + small_s.abs();
            let slack = residue.abs() * (T::pi() * arm_u + small_s.abs() * reach);
            Ok(Residual {
                plain: carrier_plain.times(&rho).minus(&image_plain),
                factors: factors(Trig3::along(&rho, m), Trig3::along(&tube, b0 * kappa)),
                floor: spiric_floor(major, minor),
                slack,
                h,
            })
        }
        _ => Err(unheld(
            PcurveKind::Spiric,
            "a spiric image is its cutting plane's cap or a torus's wall, and this chart is \
             neither of its kind",
        )),
    }
}

/// **The residual of a focal-section image against its carrier**
/// (module docs): on a cone, its slant a free channel; on a torus, its
/// tube angle `v₀ + w·t` with `w` the decided tube rate, the rest of the
/// stored channel a slack at the tube's radius. The image's structure is
/// decided in band first ([`super::focal_section_identity`]), and the
/// stored sense's residue from `σ` is a slack at the azimuth's arm.
///
/// # Errors
///
/// The image's structure read wrong ([`PcurveCertifyError::ImageMismatch`]):
/// a chart and carrier no focal section is the image of, a sense or tube
/// rate past the band from `±1`, `|β|` not below one; or one of those in
/// band ([`PcurveCertifyError::Escalated`] at
/// [`PcurveCheck::ImageIdentity`]).
fn focal_residual<T: Decide>(
    pcurve: &super::Pcurve<T>,
    image: FocalImage<T>,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    span: EdgeSpan<T>,
    band: Band,
) -> Result<Residual<T>, PcurveCertifyError> {
    let kind = PcurveKind::FocalSection;
    let tube = match (surface, carrier) {
        (Surface::Cone { .. }, Curve3::Ellipse(..)) => None,
        (&Surface::Torus { minor_radius, .. }, Curve3::Circle(..)) => Some(minor_radius),
        _ => {
            return Err(unheld(
                kind,
                "a focal-section image is a cone's ellipse or a torus's circle, and this chart \
                 and carrier are neither",
            ));
        }
    };
    let arm = super::focal_section_identity(pcurve, span.params, surface, tube, band)?;
    let Some(form) = super::carrier_harmonic(carrier) else {
        unreachable!("focal_residual: an ellipse or a circle has its harmonic form")
    };
    let (_, s) = unit_winding("pcurve_closed_focal_sense", image.sense, kind, band)?;
    let (t_m, h) = centre(span);
    let origin = geom_core::Point3::origin();
    let beta = image.beta;
    let one = Trig::constant(T::one());
    // `E = t − t₀`.
    let (cos_e, sin_e) = Trig::angle(-image.t0, 1, t_m).ok_or_else(winding_past)?;
    let a = one.minus(&cos_e.scaled(beta));
    let b = sin_e.scaled(beta);
    let (c2, s2) = (
        a.times(&a).minus(&b.times(&b)),
        a.times(&b).scaled(T::from_f64(2.0)),
    );
    // `q²·cos ν`, `q²·sin ν`, `ν = E + 2λ`.
    let n_c = cos_e.times(&c2).minus(&sin_e.times(&s2));
    let n_s = sin_e.times(&c2).plus(&cos_e.times(&s2));
    let q2 = Trig::constant(T::one() + beta.powi(2)).minus(&cos_e.scaled(beta + beta));
    let carrier = Trig3::harmonic((form.c - origin, form.a, form.b, form.l), t_m).times(&q2);
    let along = |frame: Frame<T>| {
        let (a0, b0) = radial_pair(&frame, image.u0);
        Trig3::along(&n_c, a0).plus(&Trig3::along(&n_s, b0 * s))
    };
    // The stored sense's residue from `σ` moves the azimuth by at most
    // `|residue|·sup|ν|`, `|ν| ≤ |E| + π`, at the azimuth's arm.
    let e_reach = (t_m - image.t0).abs() + h;
    let mut slack = (image.sense - s).abs() * (e_reach + T::pi()) * arm;
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
            // The tube angle as `v₀ + w·t`; the stored `va·cos t + vb·sin t
            // + (vl − w)·t` beside it is a slack at the tube's radius.
            let (w, w_t) = unit_winding("pcurve_closed_focal_tube_rate", image.vl, kind, band)?;
            let reach = t_m.abs() + h;
            slack = slack
                + minor_radius.abs()
                    * (image.va.abs() + image.vb.abs() + (image.vl - w_t).abs() * reach);
            let (cos_v, sin_v) = Trig::angle(image.v0, w, t_m).ok_or_else(winding_past)?;
            let tube = Trig::constant(major_radius).plus(&cos_v.scaled(minor_radius));
            Trig3::constant(center - origin)
                .times(&q2)
                .plus(&Trig3::along(&sin_v.times(&q2).scaled(minor_radius), axis))
                .plus(&along(Frame::new(center, axis, u_ref)).times(&tube))
        }
        _ => unreachable!("focal_residual: the chart was read a cone or a torus above"),
    };
    Ok(Residual {
        plain: carrier.minus(&mapped),
        factors: Vec::new(),
        floor: focal_floor(beta),
        slack,
        h,
    })
}

/// A winding past the trigonometric family's reach
/// ([`super::trig::MAX_WINDING`]): no closed form of a line or a conic
/// turns that often per period, so the image's structure is read wrong.
fn winding_past() -> PcurveCertifyError {
    unheld(
        PcurveKind::Harmonic,
        "the image winds more often per period of its carrier than any closed form of a line \
         or a conic does",
    )
}

/// The span's midpoint `t_m` and a half-width `h` with every `t` the
/// interval admits inside `[t_m − h, t_m + h]`: both read off the span's
/// outer ends, `f64` structure (C6), `h` rounded outward.
fn centre<T: Real>(span: EdgeSpan<T>) -> (T, T) {
    let (lo, hi) = span.outer;
    let mid = geom_core::bracket_mid((lo, hi));
    let h = (hi - mid).max(mid - lo).next_up();
    (T::from_f64(mid), T::from_f64(h))
}

/// The scalar a free channel is, or the cosine and sine an angle channel
/// enters through.
enum Read<T> {
    Free(Trig<T>),
    Angle(Trig<T>, Trig<T>),
}

fn read<T: Real>(channel: Channel<T>, t_m: T) -> Result<Read<T>, PcurveCertifyError> {
    Ok(match channel {
        Channel::Free { .. } => Read::Free(Trig::free(channel.coefficients(), t_m)),
        Channel::Angle { phase, winding } => {
            let (cos, sin) = Trig::angle(phase, winding, t_m).ok_or_else(winding_past)?;
            Read::Angle(cos, sin)
        }
    })
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
    let (u, v) = (read(u, t_m)?, read(v, t_m)?);
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
            unreachable!("chart_image: a closed form takes this door on an analytic chart only")
        }
    })
}

/// **The bound's verdict**: `Zero` certifies, and the bound is the
/// certificate's envelope. Otherwise the miss is read at a grid over the
/// span and its maximum decided: past the band, a located miss, the
/// largest the grid saw; else a bound past the band is
/// [`PcurveCertifyError::ResidualExceeded`] at [`PcurveCheck::Envelope`]
/// (nothing located, a sum of upper bounds), and one in band
/// [`PcurveCertifyError::Escalated`] there. A bound that reads negative
/// is no sum of magnitudes over a positive floor: the image's structure,
/// read wrong.
///
/// `frequency` is the residual's highest frequency, which sets the grid;
/// `miss` is the meter `|C(t) − S(P(t))|` at a point.
pub(super) fn verdict<T: Decide>(
    bound: T,
    frequency: usize,
    (span, kind): (EdgeSpan<T>, PcurveKind),
    band: Band,
    miss: impl Fn(T) -> T,
) -> Result<T, PcurveCertifyError> {
    let refused = match geom_core::k_stats::decide("pcurve_closed_bound", Margin::of(bound), band) {
        Ok(Sign::Zero) => return Ok(bound),
        Ok(Sign::Negative) => {
            return Err(unheld(
                kind,
                "the residual's bound reads negative, which no sum of magnitudes over a \
                 positive floor does",
            ));
        }
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
    let largest = (0..=points)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let t = lo + (hi - lo) * (i as f64 / points as f64);
            miss(T::from_f64(t))
        })
        .fold(T::zero(), T::max);
    if let Ok(Decided {
        sign: Sign::Positive,
        margin,
    }) = decide_reported("pcurve_closed_miss", Margin::of(largest), band)
    {
        return Err(PcurveCertifyError::MissLocated { margin });
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
    /// An image kind no analytic chart holds: an iso arc, a spline
    /// chart's rim. Its structure, read wrong.
    Unheld(PcurveKind),
}

impl<T: Real> ClosedForm<T> {
    /// The form of `image` on an analytic chart, where this door reads
    /// it.
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
            super::Pcurve::IsoArc { .. } => Some(Self::Unheld(PcurveKind::IsoArc)),
            super::Pcurve::IsoLine { .. }
            | super::Pcurve::Fitted(_)
            | super::Pcurve::General(_)
            | super::Pcurve::Projected(_) => None,
        }
    }
}

/// **The certificate of a closed-form `image` stated over `span`**: the
/// residual's coefficient bound over the span ([`Residual::bound`]),
/// decided by [`verdict`]. The carrier's incidence on its chart is the
/// caller's to read first.
///
/// # Errors
///
/// The residual's structural refusals, each its own defect ending, and
/// the verdict's.
pub(super) fn certify<T: Decide>(
    (image, form): (&super::Pcurve<T>, ClosedForm<T>),
    span: EdgeSpan<T>,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<super::PcurveCertificate<T>, PcurveCertifyError> {
    let residual = residual_of((image, form), span, carrier, surface, band)?;
    let miss = |t: T| {
        let p = image.eval(t);
        carrier.eval(t).distance(surface.eval(p.x, p.y))
    };
    let kind = PcurveKind::of(image);
    let bound = verdict(
        residual.bound(),
        residual.frequency(),
        (span, kind),
        band,
        miss,
    )?;
    Ok(super::PcurveCertificate::closed(
        0,
        T::zero(),
        bound,
        super::EnvelopeStatement::MapResidualClosedForm,
    ))
}

/// The residual of a closed form against its carrier, by its form.
fn residual_of<T: Decide>(
    (image, form): (&super::Pcurve<T>, ClosedForm<T>),
    span: EdgeSpan<T>,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<Residual<T>, PcurveCertifyError> {
    match form {
        ClosedForm::Harmonic(u, v) => harmonic_residual((u, v), carrier, surface, span),
        ClosedForm::Spiric(major, minor, offset, spiric) => {
            spiric_residual((major, minor, offset, spiric), carrier, surface, span, band)
        }
        ClosedForm::Focal(focal) => focal_residual(image, focal, carrier, surface, span, band),
        ClosedForm::Unheld(kind) => Err(unheld(
            kind,
            "an iso arc is a spline chart's rim, and no analytic chart holds one",
        )),
    }
}

/// The coefficient bound a closed form's certificate would read, for the
/// rows that hold it against a dense miss.
#[cfg(test)]
pub(super) fn bound_of(
    image: &super::Pcurve<f64>,
    carrier: &Curve3<f64>,
    surface: &Surface<f64>,
    (t0, t1): (f64, f64),
    band: Band,
) -> Option<f64> {
    let form = ClosedForm::of(image)?;
    let span = EdgeSpan {
        params: (t0, t1),
        outer: (t0, t1),
    };
    residual_of((image, form), span, carrier, surface, band)
        .ok()
        .map(|r| r.bound())
}
