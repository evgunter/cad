//! **The interval brackets the periodic-chart closure decision and its
//! interval lanes read through**: a point bracket, `τ`'s bracket, and the
//! three reads that keep a refused bracket's refusal — one side, the
//! midpoint, the absolute value. One home, so the shared decision
//! ([`super::closure`]) and its consumers ([`super::quad`],
//! [`super::spiric`]) read them from neither of each other.

use geom_core::Bounds;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;

/// Shorthand: a point bracket.
pub(super) fn pt(x: f64) -> Interval {
    Interval::point(x)
}

/// `τ` as a bracket: it lies between the `f64` nearest it, which rounds
/// below, and that value's successor.
pub(super) fn tau_bracket() -> Interval {
    Interval::hull(
        pt(core::f64::consts::TAU),
        pt(core::f64::consts::TAU.next_up()),
    )
}

/// One side of an enclosure, **keeping the refusal**: `NaN` whenever
/// the enclosure may not certify.
///
/// Interval arithmetic carries its refusal in the decoration, so a refused
/// bracket's endpoints are ordinary numbers and a bare `.lo()`/`.hi()`
/// hands a consumer a plausible bound with nothing behind it. `NaN` is
/// what every consumer on these paths already reads as "no bound" —
/// the `is_finite` tests, the margin classifiers, the gauges and the
/// knot-collapse windows all refuse on it — so this is the one door
/// that turns the decoration back into the value those readers expect.
pub(super) fn lo_or_refuse(x: Interval) -> f64 {
    if !x.is_certified() { f64::NAN } else { x.lo() }
}

/// Midpoint of a bracket (structure selection for integration limits;
/// the bracket's width is repaid by the endpoint pad). A refused
/// bracket has no midpoint, and says so.
pub(super) fn mid(x: Interval) -> f64 {
    if !x.is_certified() {
        return f64::NAN;
    }
    (x.lo() + x.hi()) * 0.5
}

/// Interval absolute value: `|x|` as an enclosure.
pub(super) trait AbsEnclosure {
    fn abs_enclosure(self) -> Interval;
}

impl AbsEnclosure for Interval {
    fn abs_enclosure(self) -> Interval {
        if !self.is_certified() {
            return self;
        }
        if self.lo() >= 0.0 {
            self
        } else if self.hi() <= 0.0 {
            -self
        } else {
            Interval::from_bounds(0.0, self.mag())
        }
    }
}
