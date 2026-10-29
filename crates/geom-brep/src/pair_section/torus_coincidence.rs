//! **Whether two tori are one surface**, decided once for the pair
//! ([`tori_coincide`]): read by the torus × torus pose before any other
//! question ([`super::TorusTorusPose::decide`]) and by the Boolean's
//! carrier ladder for two torus faces it reads undeclared.
//!
//! A ring torus (`R > r > 0`) is the set of points at distance `r` from
//! its core circle, so the distance between two of them is at most the
//! core circle's move plus the tube difference: the centre separation,
//! the major-radius difference and the axes' chord ([`line_tilt`], which
//! bounds how far a unit offset turns between them) levered at the
//! larger major radius move the core, and the minor-radius difference
//! moves the tube about it ([`TorusDisplacement::upper`]). That bound
//! holds at every point of both tori, so the decision reads no face's
//! extent and no ball. Off the ring convention a torus is not that set,
//! and the decision refuses it first.
//!
//! Each datum on its own is the counter-evidence: one past the band moves
//! the surface by that much somewhere, so the two are apart. The tilt
//! datum is the sine `|a₁ × a₂|`, not the chord: the chord bounds the
//! move from above, the sine bounds from below how far the far side of
//! the ring moves, and only a lower bound may read the two apart.

use geom_core::k_stats::{decide_reported, detached, splice, splice_superseded, straddle};
use geom_core::{
    Band, Decide, Decided, Indeterminate, Margin, MarginDiag, Point3, Real, Sign, Vec3,
};

use super::line_tilt;
use crate::identity::{IdentityWalk, identity_walk};
use crate::torus_convention::TorusRefusal;

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

/// **A datum of two tori's coincidence.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TorusDatum {
    /// The axes' tilt, levered at the larger major radius.
    AxisTilt,
    /// The centre separation.
    Centre,
    /// The major-radius difference.
    MajorRadius,
    /// The minor-radius difference.
    MinorRadius,
}

impl TorusDatum {
    /// The row the datum is decided on.
    #[must_use]
    pub const fn row(self) -> &'static str {
        match self {
            Self::AxisTilt => "carrier_torus_axis_parallel",
            Self::Centre => "carrier_torus_center",
            Self::MajorRadius => "carrier_torus_major_radius",
            Self::MinorRadius => "carrier_torus_minor_radius",
        }
    }
}

/// The row the displacement's upper bound is decided on.
pub const TT_COINCIDENT: &str = "tt_coincident";
/// The row the displacement's lower bound is decided on.
pub const TT_COINCIDENT_FLOOR: &str = "tt_coincident_floor";

/// **The displacement between two tori** (module docs).
#[derive(Clone, Copy, Debug)]
pub struct TorusDisplacement<T: Real> {
    /// Each datum, named.
    pub data: [(TorusDatum, Margin<T>); 4],
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
                    TorusDatum::AxisTilt,
                    Margin::levered(a.axis.cross(b.axis).norm(), arm),
                ),
                (TorusDatum::Centre, Margin::norm3(apart)),
                (
                    TorusDatum::MajorRadius,
                    Margin::of(a.major_radius - b.major_radius),
                ),
                (
                    TorusDatum::MinorRadius,
                    Margin::of(a.minor_radius - b.minor_radius),
                ),
            ],
            upper: tube + core,
            across: tube - core,
        }
    }
}

/// **What [`tori_coincide`] decided.**
#[derive(Clone, Debug)]
pub enum TorusCoincidence {
    /// One surface: the upper bound decided `Zero`, on this margin.
    One(MarginDiag),
    /// Two surfaces: a datum decided past the band.
    Apart,
    /// Neither: the data's own refusal ([`identity_walk`]), else the
    /// upper bound in the band, else every datum decided `Zero` beside
    /// the bound decided past the band (`tt_coincident_straddle`, which
    /// only an escalation factor under four can reach).
    Undecided(Indeterminate),
    /// A torus not shown inside its ring convention, which the bound
    /// does not hold for.
    OffRing(TorusRefusal),
}

/// **Decides whether two tori are one surface** (module docs): one where
/// the upper bound decides `Zero`, apart where a datum decides past the
/// band, and otherwise undecided; a torus off its ring convention
/// refuses first.
pub fn tori_coincide<T: Decide>(a: TorusData<T>, b: TorusData<T>, band: Band) -> TorusCoincidence {
    for t in [a, b] {
        if let Err(refusal) = TorusRefusal::require(t.major_radius, t.minor_radius, band) {
            return TorusCoincidence::OffRing(refusal);
        }
    }
    let d = TorusDisplacement::of(a, b);
    let (upper, upper_log) = detached(|| decide_reported(TT_COINCIDENT, Margin::of(d.upper), band));
    let upper = match upper {
        Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) => {
            splice(upper_log);
            return TorusCoincidence::One(margin);
        }
        upper => upper,
    };
    let (walk, walk_log) =
        detached(|| identity_walk(d.data.map(|(datum, m)| (datum.row(), m)), band));
    if matches!(walk, IdentityWalk::Distinct) {
        // The answer rests on the datum alone.
        splice_superseded(upper_log);
    } else {
        splice(upper_log);
    }
    splice(walk_log);
    match (walk, upper) {
        (IdentityWalk::Distinct, _) => TorusCoincidence::Apart,
        (IdentityWalk::Undecided(diag), _) | (IdentityWalk::One(_), Err(diag)) => {
            TorusCoincidence::Undecided(diag)
        }
        // The bound is a sum of magnitudes, never decided below zero.
        (IdentityWalk::One(lower), Ok(upper)) => TorusCoincidence::Undecided(straddle(
            "tt_coincident_straddle",
            Decided {
                sign: Sign::Zero,
                margin: lower.unwrap_or(upper.margin),
            },
            upper,
            band,
        )),
    }
}
