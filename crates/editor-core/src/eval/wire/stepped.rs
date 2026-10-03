//! **The stepped placement rule**: the operands a linear or circular
//! rule steps by, the one constructor of each that decides them, and
//! the map of placement `i`. The operands' representation is private
//! to this module, so every value a step reads went through
//! [`SteppedOperands::linear`] or [`SteppedOperands::circular`]; the
//! evaluation (`stepped_map`) and the mate solve's derived offset both
//! call them, so one value refuses on both roads.

use geom_core::{Affine3, Band, Decide, Point3, Sign, UnitVec3, Vec3};

use super::{PATTERN_DIRECTION_ROLE, escalated, turns_off, unit};
use crate::eval::{NodeErrorKind, StepTurns};
use crate::expr::Expr;

/// The funnel name of a linear pattern's spacing sign.
pub(crate) const PATTERN_SPACING: &str = "pattern_spacing";

/// The funnel name of whether a circular pattern's step is zero.
pub(crate) const PATTERN_STEP: &str = "pattern_step";

/// The funnel name of how many whole turns a circular pattern's step
/// holds.
pub(crate) const PATTERN_STEP_TURN: &str = "pattern_step_full_turn";

/// The most whole turns [`turns_held`] decides a step against: past
/// 2^52 turns a turn is below the step's own rounding.
const MAX_TURNS: u64 = 1 << 52;

/// The resolved operands of a stepped placement rule: what the rule's
/// math consumes, every direction unit by type: a LINEAR rule's
/// direction minted through [`unit()`], a CIRCULAR rule's axis a
/// datum's `UnitVec3`, not re-decided.
pub(crate) struct SteppedOperands<T: geom_core::Real>(Rule<T>);

enum Rule<T: geom_core::Real> {
    /// A linear rule: unit direction, spacing per step.
    Linear {
        /// The stepping direction.
        direction: UnitVec3<T>,
        /// The per-step translation distance along it, positive.
        spacing: T,
    },
    /// A circular rule: the datum axis and the angle per step.
    Circular {
        /// A point on the axis, the datum's own.
        origin: Point3<T>,
        /// The axis direction, the datum's own witness.
        dir: UnitVec3<T>,
        /// The rotation angle per step, signed by the right-hand rule
        /// about `dir`, nonzero and within a turn.
        step: T,
    },
}

impl<T: Decide> SteppedOperands<T> {
    /// A linear rule's operands. The direction is normalized under
    /// [`PATTERN_DIRECTION_ROLE`]; the spacing is a size, so its sign
    /// is decided at the band as an extrude's depth is: below zero
    /// refuses naming the direction that steps the other way, spelled
    /// from `authored` (the direction slots' expressions), and zero
    /// refuses because every copy would land on the master.
    pub(crate) fn linear(
        direction: Vec3<T>,
        spacing: T,
        authored: &[Expr; 3],
        band: Band,
    ) -> Result<Self, NodeErrorKind> {
        let unit_dir = unit(direction, PATTERN_DIRECTION_ROLE, band)?;
        let decided = geom_core::k_stats::decide_reported(
            PATTERN_SPACING,
            geom_core::Margin::of(spacing),
            band,
        )
        .map_err(escalated(PATTERN_SPACING))?;
        match decided.sign {
            Sign::Positive => Ok(Self(Rule::Linear {
                direction: unit_dir,
                spacing,
            })),
            Sign::Zero => Err(NodeErrorKind::DegenerateSpacing),
            Sign::Negative => Err(NodeErrorKind::NegativeSpacing {
                spacing: decided.margin,
                reversed: authored.each_ref().map(negated),
            }),
        }
    }

    /// A circular rule's operands. The step keeps its sign; a zero (or
    /// sliver) step refuses, and so does one of a turn or more, whose
    /// refusal says how the copies land: on the master at a whole
    /// number of turns, otherwise where one angle within a turn puts
    /// them, spelled from `authored` (the step slot's expression).
    /// Both are radians against the linear band, as the revolve's
    /// full-turn check is (ledger row F14).
    pub(crate) fn circular(
        origin: Point3<T>,
        dir: UnitVec3<T>,
        step: T,
        authored: &Expr,
        band: Band,
    ) -> Result<Self, NodeErrorKind> {
        let seen = geom_core::k_stats::decide_flagged_reported(PATTERN_STEP, step, band, "F14")
            .map_err(escalated(PATTERN_STEP))?;
        let positive = match seen.sign {
            Sign::Zero => return Err(NodeErrorKind::DegenerateStep),
            Sign::Positive => true,
            Sign::Negative => false,
        };
        let turns = match turns_off(PATTERN_STEP_TURN, step, T::tau(), band)? {
            Sign::Negative => return Ok(Self(Rule::Circular { origin, dir, step })),
            Sign::Zero => StepTurns::Whole,
            Sign::Positive => match turns_held(step, band)? {
                Held::Whole => StepTurns::Whole,
                Held::Turns(held) => StepTurns::Within(within_turn(authored, positive, held)),
                Held::Beyond => StepTurns::Unresolved,
            },
        };
        Err(NodeErrorKind::FullRangeStep {
            step: crate::expr::unparse(authored),
            evaluated: authored.literal_value().is_none().then_some(seen.margin),
            turns,
        })
    }
}

/// How many whole turns a step past one turn holds.
enum Held {
    /// A whole number of turns, at tolerance.
    Whole,
    /// The most `k` with `|step| − k·τ` positive.
    Turns(u64),
    /// More than [`MAX_TURNS`].
    Beyond,
}

/// The whole turns a step past one turn holds, every comparison a
/// decision. A doubling search, then a bisection, so the decisions
/// grow with the logarithm of the step.
fn turns_held<T: Decide>(step: T, band: Band) -> Result<Held, NodeErrorKind> {
    let off = |k: u64| {
        let whole = T::tau() * T::from_f64(k as f64);
        turns_off(PATTERN_STEP_TURN, step, whole, band)
    };
    let (mut below, mut above) = (1, 2);
    loop {
        match off(above)? {
            Sign::Zero => return Ok(Held::Whole),
            Sign::Negative => break,
            Sign::Positive if above >= MAX_TURNS => return Ok(Held::Beyond),
            Sign::Positive => (below, above) = (above, above * 2),
        }
    }
    while above - below > 1 {
        let mid = below + (above - below) / 2;
        match off(mid)? {
            Sign::Zero => return Ok(Held::Whole),
            Sign::Positive => below = mid,
            Sign::Negative => above = mid,
        }
    }
    Ok(Held::Turns(below))
}

/// `authored`, `held` turns nearer zero, in the grammar a user types:
/// a literal as one literal in its own unit, anything else less (or,
/// for a negative step, plus) the turns in degrees. Either lands
/// every copy where `authored` does, up to rounding.
fn within_turn(authored: &Expr, positive: bool, held: u64) -> String {
    let sign = if positive { 1.0 } else { -1.0 };
    let literal = authored.literal_value().zip(
        authored
            .display_unit()
            .and_then(quantity::UnitDef::as_angle),
    );
    let within = match literal {
        Some((radians, unit)) => {
            let turn = std::f64::consts::TAU / unit.factor();
            Expr::angle_in(radians / unit.factor() - sign * turn * held as f64, unit)
        }
        None => Expr::angle_in(360.0 * held as f64, quantity::DEG).and_then(|turns| {
            if positive {
                Expr::sub(authored.clone(), turns)
            } else {
                Expr::add(authored.clone(), turns)
            }
        }),
    };
    within.map_or_else(
        |_| {
            let op = if positive { '-' } else { '+' };
            format!("{} {op} {} deg", crate::expr::unparse(authored), 360 * held)
        },
        |e| crate::expr::unparse(&e),
    )
}

/// `authored`, negated, in the grammar a user types: a literal's own
/// value negated (a zero stays `0.0`), anything else under a unary
/// minus. Either evaluates to the exact negation, so the direction it
/// spells steps the copies where the negative spacing did.
fn negated(authored: &Expr) -> String {
    let flipped = match authored.literal_value() {
        Some(v) => Expr::literal(-v + 0.0, authored.dim()),
        None => Expr::neg(authored.clone()),
    };
    flipped.map_or_else(
        |_| format!("-({})", crate::expr::unparse(authored)),
        |e| crate::expr::unparse(&e),
    )
}

/// The rigid map of placement `i ≥ 1` under a STEPPED rule (linear or
/// circular) — **the one home of the stepped placement rule's math**,
/// read by both placement-rule nodes (through `stepped_map`) and by
/// the mate solve's derived offset, so all three derive the same map
/// bit for bit. Placement 0 is the identity and reads no operand, so
/// no caller asks this for it. `i as f64` is exact up to 2^53.
pub(crate) fn stepped_rule_map<T: Decide>(ops: &SteppedOperands<T>, i: i64) -> Affine3<T> {
    let step = T::from_f64(i as f64);
    match &ops.0 {
        Rule::Linear { direction, spacing } => {
            Affine3::translation(direction.get() * (*spacing * step))
        }
        Rule::Circular {
            origin,
            dir,
            step: angle,
        } => Affine3::rotation_about_axis(*origin, dir.get(), *angle * step),
    }
}
