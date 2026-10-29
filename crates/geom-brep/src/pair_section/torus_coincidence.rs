//! **Whether two tori are one surface**, decided once for the pair
//! ([`tori_coincide`]): read by the torus × torus pose before any other
//! question ([`super::TorusTorusPose::decide`]) and by the Boolean's
//! carrier ladder for two torus faces it reads undeclared.
//!
//! A torus is the tube of radius `r` about its core circle, so the
//! distance between two tori is at most the core circle's move plus the
//! tube difference: the centre separation, the major-radius difference
//! and the axes' chord ([`line_tilt`]) levered at the larger major
//! radius move the core, and the minor-radius difference moves the tube
//! about it ([`TorusDisplacement::upper`]). That bound holds at every
//! point of both tori, so the decision reads no face's extent and no
//! ball. Each datum on its own is the counter-evidence: one past the
//! band moves the surface by that much somewhere, so the two are apart.

use geom_core::k_stats::{decide_reported, detached, splice, splice_superseded};
use geom_core::{
    Band, Decide, Decided, Indeterminate, Margin, MarginDiag, Point3, Real, Sign, Vec3,
};

use super::line_tilt;

/// A torus as its coincidence reads it: the centre, the unit axis and
/// the two radii.
#[derive(Clone, Copy, Debug)]
pub struct TorusData<T: Real> {
    /// The centre.
    pub center: Point3<T>,
    /// The unit axis of revolution.
    pub axis: Vec3<T>,
    /// The major radius.
    pub major_radius: T,
    /// The minor radius.
    pub minor_radius: T,
}

/// The row the displacement's upper bound is decided on.
pub const TT_COINCIDENT: &str = "tt_coincident";

/// **The displacement between two tori** (module docs).
#[derive(Clone, Copy, Debug)]
pub struct TorusDisplacement<T: Real> {
    /// Each datum with its row: the axes' tilt levered at the larger
    /// major radius, the centre separation, the major-radius and the
    /// minor-radius differences.
    pub data: [(&'static str, Margin<T>); 4],
    /// The bound on the distance from either torus to the other.
    pub upper: T,
    /// The minor-radius difference less the core's move: a lower bound
    /// on that distance at every point (negative where it bounds
    /// nothing).
    pub across: T,
}

impl<T: Real> TorusDisplacement<T> {
    /// The displacement between `a` and `b`.
    #[must_use]
    pub fn of(a: TorusData<T>, b: TorusData<T>) -> Self {
        let arm = a.major_radius.max(b.major_radius);
        let apart = a.center - b.center;
        let core = apart.norm()
            + (a.major_radius - b.major_radius).abs()
            + Margin::levered(line_tilt(a.axis, b.axis), arm).value();
        let tube = (a.minor_radius - b.minor_radius).abs();
        Self {
            data: [
                (
                    "carrier_torus_axis_parallel",
                    Margin::levered(a.axis.cross(b.axis).norm(), arm),
                ),
                ("carrier_torus_center", Margin::norm3(apart)),
                (
                    "carrier_torus_major_radius",
                    Margin::of(a.major_radius - b.major_radius),
                ),
                (
                    "carrier_torus_minor_radius",
                    Margin::of(a.minor_radius - b.minor_radius),
                ),
            ],
            upper: tube + core,
            across: tube - core,
        }
    }
}

/// **What [`tori_coincide`] decided.**
#[derive(Clone, Copy, Debug)]
pub enum TorusCoincidence {
    /// One surface: the upper bound decided `Zero`, on this margin.
    One(MarginDiag),
    /// Two surfaces: a datum decided past the band.
    Apart,
    /// Neither: a datum not finite (its margin invalid), else the first
    /// datum in the band, else the upper bound in the band, else the
    /// data decided `Zero` beside the bound past it (`tt_coincident_straddle`).
    Undecided(Indeterminate),
}

/// **Decides whether two tori are one surface** (module docs): one where
/// the upper bound decides `Zero`, apart where a datum decides past the
/// band, and otherwise undecided. A datum not finite reads no sign: its
/// residual `x − x`, poison, is what the funnel reads for it.
pub fn tori_coincide<T: Decide>(a: TorusData<T>, b: TorusData<T>, band: Band) -> TorusCoincidence {
    let d = TorusDisplacement::of(a, b);
    let finite = |m: Margin<T>| geom_core::is_finite_length(m.value());
    let (upper, upper_log) = detached(|| decide_reported(TT_COINCIDENT, Margin::of(d.upper), band));
    if let Ok(
        decided @ Decided {
            sign: Sign::Zero, ..
        },
    ) = upper
        && d.data.iter().all(|&(_, m)| finite(m))
    {
        splice(upper_log);
        return TorusCoincidence::One(decided.margin);
    }
    let mut logs = vec![upper_log];
    let (mut blocked, mut in_band, mut zero) = (None, None, None);
    for &(name, margin) in &d.data {
        let read_on = if finite(margin) {
            margin
        } else {
            Margin::of(margin.value() - margin.value())
        };
        let (read, log) = detached(|| decide_reported(name, read_on, band));
        match read {
            Ok(Decided {
                sign: Sign::Positive | Sign::Negative,
                ..
            }) if finite(margin) => {
                // The answer rests on this datum alone.
                logs.into_iter().for_each(splice_superseded);
                splice(log);
                return TorusCoincidence::Apart;
            }
            Ok(decided) => {
                zero.get_or_insert(decided);
            }
            Err(diag) if !finite(margin) || diag.margin.is_invalid() => {
                blocked.get_or_insert(diag);
            }
            Err(diag) => {
                in_band.get_or_insert(diag);
            }
        }
        logs.push(log);
    }
    logs.into_iter().for_each(splice);
    let undecided = match (blocked.or(in_band), upper, zero) {
        (Some(diag), ..) | (None, Err(diag), _) => diag,
        (None, Ok(upper), Some(lower)) => {
            geom_core::k_stats::straddle("tt_coincident_straddle", lower, upper, band)
        }
        (None, Ok(_), None) => unreachable!(
            "four data read, none definite, in the band or poisoned, and none decided Zero"
        ),
    };
    TorusCoincidence::Undecided(undecided)
}
