//! **A point against one carrier**: its foot ([`Curve3::foot`]) and
//! whether it lies on the carrier ([`Curve3::on_carrier`]). These are
//! the one-carrier readings of a point on a curve (D3), homed in `geom`
//! beside the carrier's evaluators and its other banded decisions.
//!
//! **The foot** is a parameter of the carrier near the point's closest
//! point, and its gap `|q − C(t)|`, which bounds the point's distance
//! from the carrier from above. Only [`Curve3::foot`] makes one, so a
//! gap is always the distance to the carrier's point at the foot's own
//! parameter. Where it is found:
//!
//! - a line: in closed form;
//! - a circle: the angle of the point about the centre, exact, the
//!   radius's sign read into it;
//! - an ellipse: the angle of the point's unit-coordinate image, then
//!   Gauss–Newton. The unit-coordinate angle alone is not a foot: its
//!   point lies up to `(1 + a/b)·d` from a point `d` off the ellipse;
//! - a spiric oval: [`Curve3::param_near`]'s angle, then Gauss–Newton;
//! - a spline: the best of the caller's seed and a few samples on each
//!   knot span, then Gauss–Newton, held to the domain.
//!
//! A closed kind's angle is read on the branch within half a turn of the
//! seed, relative to the seed's direction or its opposite, so an
//! enclosure of the point's direction never crosses the angle's own cut.
//! Each polish step is kept only where it decides the gap smaller, so
//! the foot's gap is never worse than its seed's. A foot can be a poor
//! one (a far branch of a spline); it is never a wrong one, because no
//! verdict rests on its being the nearest point.
//!
//! **On the carrier** is decided on the foot's gap, zero in the band
//! (`curve_carrier_gap`). **Off the carrier** is decided only on a lower
//! bound on the point's distance from the whole carrier, past the band
//! (`curve_carrier_clear`), so a poor foot can cost an escalation but
//! never a wrong `Off`. The lower bound, per kind:
//!
//! - a line and a circle: the exact distance;
//! - an ellipse: `2|F|/(g + √(g² + 4|F|/s²))` in its plane, with the
//!   height above the plane folded in. Here `F = X²/a² + Y²/b² − 1` at the
//!   point's in-plane image, `g = |∇F|`, and `s` is the smaller
//!   semi-axis. Along the segment from the image to its foot, `F` falls
//!   to zero while `|∇F|` grows at most at the Hessian's norm `2/s²`, so
//!   `|F| ≤ g·d + d²/s²`, whose positive root is the bound;
//! - a spiric oval: the greatest of the point's distance from the
//!   cutting plane, its distance from the torus, and how far it lies
//!   short of the oval's side of the plane's trace;
//! - a spline: the certified piece walk ([`geom_core::bisect`]) over the
//!   whole domain. A piece is cleared when its middle point's distance
//!   less its reach (its half-span times the carrier's greatest speed,
//!   [`crate::carrier_speed`]) decides positive (`curve_carrier_piece_clear`),
//!   and stops once its reach is within the band
//!   (`curve_carrier_piece`). The point is off only where every piece
//!   clears. A walk past [`geom_core::bisect::PIECE_BUDGET`] refuses
//!   ([`CarrierRefusal::Exhausted`]), and a speed bound that is zero or
//!   poison refuses on kind ([`CarrierRefusal::Unbounded`]). A rational
//!   spline's speed bound is certified at `Interval`; at `f64` and
//!   extreme weights it is the same expression in round-to-nearest, not
//!   a bound.
//!
//! The gap on the band's zero side and the lower bound past it straddle
//! the whole band (`curve_carrier_straddle`): only a carrier bending
//! tighter than the band resolves, or a poor foot, allows it, and it
//! escalates.

use geom_core::bisect::{Step, bisect};
use geom_core::k_stats::{decide, decide_reported, straddle};
use geom_core::{Band, Decide, Decided, Indeterminate, Margin, MarginDiag, Point3, Sign, Vec3};

use super::Curve3;

/// The Gauss–Newton steps a foot is polished by.
const POLISH_STEPS: usize = 6;

/// Where a spline's foot search samples each knot span, as fractions of
/// it.
const SPLINE_SAMPLES: [f64; 4] = [0.125, 0.375, 0.625, 0.875];

/// A point's foot on a carrier, made only by [`Curve3::foot`]
/// (module docs).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Foot<T> {
    t: T,
    gap: T,
}

impl<T: Copy> Foot<T> {
    /// The carrier's parameter, as found: a closed carrier's is not
    /// reduced to any branch.
    pub fn t(&self) -> T {
        self.t
    }

    /// `|q − C(t)|`, an upper bound on the point's distance from the
    /// carrier.
    pub fn gap(&self) -> T {
        self.gap
    }
}

/// Where [`Curve3::on_carrier`] places a point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OnCarrier {
    /// Within the band of the carrier at the foot; `margin` is the gap
    /// reading's.
    On {
        /// The gap reading's margin.
        margin: MarginDiag,
    },
    /// Definitely past the band from the whole carrier.
    Off,
}

/// Why [`Curve3::on_carrier`] gives no answer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CarrierRefusal {
    /// A decision the answer rests on fell in the band's gap.
    Escalated(Indeterminate),
    /// The gap on the band's zero side and the lower bound past it
    /// straddle the whole band (`curve_carrier_straddle`): two decided
    /// readings of one distance that no margin of either says.
    Straddle(Indeterminate),
    /// A spline's piece walk ran past its budget.
    Exhausted,
    /// A spline whose greatest speed is zero or poison: its pieces have
    /// no reach to clear by.
    Unbounded,
}

impl From<Indeterminate> for CarrierRefusal {
    fn from(diag: Indeterminate) -> Self {
        Self::Escalated(diag)
    }
}

/// A run `(a, b)` of a spline's domain.
#[derive(Clone, Copy)]
struct Piece<T> {
    a: T,
    b: T,
}

impl<T: Decide> Curve3<T> {
    /// **`q`'s foot on the carrier** (module docs). `near` is a seed: the
    /// parameter a consumer already holds for `q`, or any parameter of
    /// the stretch it reads.
    pub fn foot(&self, q: Point3<T>, near: T) -> Foot<T> {
        let seed = match self {
            Self::Line { origin, dir } => (q - *origin).dot(*dir) / dir.dot(*dir),
            Self::Circle {
                center,
                axis,
                u_ref,
                radius,
            } => {
                // Scaled by the radius, whose sign turns the angles.
                let w = (q - *center) * *radius;
                return self.foot_at(
                    q,
                    angle_near(w.dot(*u_ref), w.dot(axis.cross(*u_ref)), near),
                );
            }
            Self::Ellipse {
                center,
                axis,
                semi_u,
                semi_v,
                u_ref,
            } => {
                let w = q - *center;
                angle_near(
                    w.dot(*u_ref) / *semi_u,
                    w.dot(axis.cross(*u_ref)) / *semi_v,
                    near,
                )
            }
            Self::Spiric {
                center,
                axis,
                major_radius,
                ..
            } => {
                // The torus's minor angle about the tube's centre circle,
                // as [`Curve3::param_near`] reads it.
                let w = q - *center;
                let h = w.dot(*axis);
                let rho = (w - *axis * h).norm();
                angle_near(rho - *major_radius, h, near)
            }
            Self::Nurbs(n) => {
                let knots = n.knot_values();
                let (lo, hi) = knots.domain();
                let breaks: Vec<T> = knots.runs().map(|(k, _)| k).collect();
                let mut best = self.foot_at(q, near.max(lo).min(hi));
                for k in breaks.windows(2) {
                    for s in SPLINE_SAMPLES {
                        let f = self.foot_at(q, k[0] + (k[1] - k[0]) * T::from_f64(s));
                        if smaller(f.gap, best.gap) {
                            best = f;
                        }
                    }
                }
                return self.polish(q, best, Some((lo, hi)));
            }
        };
        let at = self.foot_at(q, seed);
        match self {
            Self::Line { .. } => at,
            _ => self.polish(q, at, None),
        }
    }

    /// The foot at `t` as read, unpolished.
    fn foot_at(&self, q: Point3<T>, t: T) -> Foot<T> {
        Foot {
            t,
            gap: (q - self.eval(t)).norm(),
        }
    }

    /// Gauss–Newton from `from`, each step kept only where it decides the
    /// gap smaller, held to `domain` where there is one.
    fn polish(&self, q: Point3<T>, from: Foot<T>, domain: Option<(T, T)>) -> Foot<T> {
        let mut best = from;
        for _ in 0..POLISH_STEPS {
            let (p, d) = self.ders1(best.t);
            let mut t = best.t - (p - q).dot(d) / d.dot(d);
            if let Some((lo, hi)) = domain {
                t = t.max(lo).min(hi);
            }
            let next = self.foot_at(q, t);
            if !smaller(next.gap, best.gap) {
                break;
            }
            best = next;
        }
        best
    }

    /// **Whether `q` lies on the carrier**, read at its foot (module
    /// docs): on by the gap at the foot's parameter, off by a lower bound
    /// on its distance from the whole carrier. The gap is read afresh for
    /// `q`, so a foot found for another point is only a poor one.
    ///
    /// # Errors
    ///
    /// [`CarrierRefusal`]: an escalation where neither decides, the gap
    /// in the band's zero and the lower bound past it straddling the band,
    /// or a spline's walk that cannot run.
    pub fn on_carrier(
        &self,
        q: Point3<T>,
        foot: &Foot<T>,
        band: Band,
    ) -> Result<OnCarrier, CarrierRefusal> {
        let gap = (q - self.eval(foot.t)).norm();
        let gap = decide_reported("curve_carrier_gap", Margin::of(gap), band);
        if let Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) = gap
        {
            return Ok(OnCarrier::On { margin });
        }
        let clear = match self {
            Self::Nurbs(_) => return self.spline_clear(q, gap, band),
            _ => decide_reported("curve_carrier_clear", Margin::of(self.clearance(q)), band),
        };
        match (clear, gap) {
            (
                Ok(Decided {
                    sign: Sign::Positive,
                    ..
                }),
                _,
            ) => Ok(OnCarrier::Off),
            (_, Err(diag)) | (Err(diag), _) => Err(diag.into()),
            (Ok(low), Ok(high)) => Err(CarrierRefusal::Straddle(straddle(
                "curve_carrier_straddle",
                low,
                high,
                band,
            ))),
        }
    }

    /// A lower bound on `q`'s distance from the whole carrier, on every
    /// kind but a spline (module docs).
    fn clearance(&self, q: Point3<T>) -> T {
        match *self {
            Self::Line { origin, dir } => {
                let w = q - origin;
                let u = dir * (T::one() / dir.norm());
                (w - u * w.dot(u)).norm()
            }
            Self::Circle {
                center,
                axis,
                radius,
                ..
            } => {
                let (h, rho) = height_and_radius(q - center, axis);
                (h.powi(2) + (rho - radius).powi(2)).sqrt()
            }
            Self::Ellipse {
                center,
                axis,
                semi_u,
                semi_v,
                u_ref,
            } => {
                let w = q - center;
                let (x, y) = (w.dot(u_ref) / semi_u, w.dot(axis.cross(u_ref)) / semi_v);
                let two = T::from_f64(2.0);
                let f = x.powi(2) + y.powi(2) - T::one();
                let g2 = (two * x / semi_u).powi(2) + (two * y / semi_v).powi(2);
                let g = g2.sqrt();
                let small = semi_u.abs().min(semi_v.abs());
                let lower =
                    two * f.abs() / (g + (g2 + T::from_f64(4.0) * f.abs() / small.powi(2)).sqrt());
                (lower.powi(2) + w.dot(axis).powi(2)).sqrt()
            }
            Self::Spiric {
                center,
                axis,
                u_ref,
                major_radius,
                minor_radius,
                offset,
            } => {
                let w = q - center;
                let plane = (w.dot(u_ref) - offset).abs();
                let (h, rho) = height_and_radius(w, axis);
                let torus = ((rho - major_radius).powi(2) + h.powi(2)).sqrt() - minor_radius;
                let side = ((major_radius - minor_radius).powi(2) - offset.powi(2)).sqrt()
                    - w.dot(axis.cross(u_ref));
                plane.max(torus.abs()).max(side)
            }
            Self::Nurbs(_) => unreachable!("a spline's clearance is its piece walk"),
        }
    }

    /// A spline's `Off` by its piece walk (module docs), `gap` the foot's
    /// reading.
    fn spline_clear(
        &self,
        q: Point3<T>,
        gap: Result<Decided, Indeterminate>,
        band: Band,
    ) -> Result<OnCarrier, CarrierRefusal> {
        let Self::Nurbs(n) = self else {
            unreachable!("only a spline is walked")
        };
        let hi = crate::carrier_speed::speed_range(self).1;
        let exact = Band::new(f64::from_bits(1), f64::from_bits(2))
            .map_err(|_| CarrierRefusal::Unbounded)?;
        if !geom_core::is_finite_length(hi.get())
            || decide("curve_carrier_speed", Margin::of(hi.get()), exact) != Ok(Sign::Positive)
        {
            return Err(CarrierRefusal::Unbounded);
        }
        let (lo, top) = n.knot_values().domain();
        let mut kept: Option<Result<Decided, Indeterminate>> = None;
        bisect(
            [Piece { a: lo, b: top }],
            |p: Piece<T>| -> Step<Piece<T>, core::convert::Infallible> {
                let m = crate::mid_param(p.a, p.b);
                let reach = geom_core::bisect::reach(hi, (p.a, p.b));
                let clear = decide_reported(
                    "curve_carrier_piece_clear",
                    Margin::of((self.eval(m) - q).norm() - reach),
                    band,
                );
                if matches!(
                    clear,
                    Ok(Decided {
                        sign: Sign::Positive,
                        ..
                    })
                ) {
                    return Step::Settled;
                }
                if matches!(
                    decide("curve_carrier_piece", Margin::of(reach), band),
                    Ok(Sign::Negative | Sign::Zero)
                ) {
                    kept = kept.or(Some(clear));
                    return Step::Settled;
                }
                Step::Halve([Piece { a: p.a, b: m }, Piece { a: m, b: p.b }])
            },
        )
        .map_err(|_| CarrierRefusal::Exhausted)?;
        match (kept, gap) {
            (None, _) => Ok(OnCarrier::Off),
            (_, Err(diag)) | (Some(Err(diag)), _) => Err(diag.into()),
            (Some(Ok(low)), Ok(high)) => Err(CarrierRefusal::Straddle(straddle(
                "curve_carrier_straddle",
                low,
                high,
                band,
            ))),
        }
    }
}

/// **The angle of `(x, y)`, on the branch within half a turn of
/// `near`**: read relative to `near`'s direction, or to its opposite where
/// the point lies definitely behind it, so the angle's own branch cut is
/// never crossed and an enclosure of the point's direction stays an
/// enclosure of one angle.
fn angle_near<T: Decide>(x: T, y: T, near: T) -> T {
    let (sin, cos) = near.sin_cos();
    let (along, across) = (x * cos + y * sin, y * cos - x * sin);
    let behind = Band::new(f64::from_bits(1), f64::from_bits(2)).is_ok_and(|exact| {
        decide("curve_foot_branch", Margin::of(along), exact) == Ok(Sign::Negative)
    });
    if behind {
        near + T::pi() + (T::zero() - across).atan2(T::zero() - along)
    } else {
        near + across.atan2(along)
    }
}

/// `w`'s height along `axis` and its distance from the axis.
fn height_and_radius<T: Decide>(w: Vec3<T>, axis: Vec3<T>) -> (T, T) {
    let h = w.dot(axis);
    (h, (w - axis * h).norm())
}

/// Whether `a` is decided smaller than `b`: a heuristic's choice, read
/// on the least band so it takes the sign alone, and no verdict.
fn smaller<T: Decide>(a: T, b: T) -> bool {
    Band::new(f64::from_bits(1), f64::from_bits(2)).is_ok_and(|exact| {
        decide("curve_foot_polish", Margin::of(b - a), exact) == Ok(Sign::Positive)
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod rows;
