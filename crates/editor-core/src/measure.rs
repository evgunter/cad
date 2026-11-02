//! The measurement vocabulary (ERROR-DESIGN E3, CONTACT-DESIGN C5).
//!
//! A [`Measure`](crate::Node::Measure) node is an operation holding ONE
//! [`MeasurePrimitive`]: a closed-form measurement of two sited
//! references. It defines one scalar output of the primitive's
//! dimension, and that output is **observed** (D10): a function of the
//! built geometry rather than of what was written, read only by an
//! [`Assertion`](crate::Node::Assertion), directly or through a
//! definition.
//!
//! Arithmetic over measured values is an ordinary `Defined` variable
//! over the measures' outputs: there is one arithmetic language, the
//! formula's.

use geom_brep::recourse::{Reading, RefusedArm, RefusedSide, SizedDecision};
use geom_core::{Decide, SizedPass};

use crate::expr::Dimension;
use crate::node::SitedRef;

/// Which closed-form measurement a measure computes, over which two
/// references `R`.
///
/// A [`crate::Node::Measure`] holds one over its reads (a stored
/// node's [`crate::VarId`]s, authored as [`SitedRef`]s); another
/// reference type is the same primitive read elsewhere, through
/// [`Self::try_map`].
///
/// The v1 carrier scope is stated on each door in
/// [`mod@crate::eval::measure`], where the closed forms live: this
/// enum names WHAT is measured, and a pair of carriers the closed form
/// has no arm for is a typed evaluation refusal
/// ([`MeasureUnsupported`](crate::eval::measure::MeasureUnsupported)), never a
/// guess.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum MeasurePrimitive<R = SitedRef> {
    /// The distance between two referenced entities → [`Dimension::Length`]:
    /// a face, an edge or a vertex, or a pose (a point, an axis, a plane,
    /// a frame as its xy plane) read off a cell or a split's tool. Two
    /// axes read as one scalar, zero where they are one line or meet.
    Distance {
        /// The first entity.
        a: R,
        /// The second.
        b: R,
    },
    /// The angle between two referenced entities → [`Dimension::Angle`].
    Angle {
        /// The first entity.
        a: R,
        /// The second.
        b: R,
    },
    /// C5's SIGNED gap between a mating pair → [`Dimension::Length`].
    ///
    /// **Argument order is the mating ROLE**, not a symmetry: `outer`
    /// is the containing carrier (the plane the offset is measured
    /// FROM, the outer of two nested faces) and `inner` the contained
    /// one, authored rather than inferred from which radius is larger.
    /// Two faces are read by both material sides: an opposed pair (a
    /// rod on a plate, two rods side by side, a pin in a bore) reads the
    /// same in either order, and an aligned one (a flush pair, two
    /// coaxial rods end to end) negates.
    ///
    /// Each reference is a face, an edge or a vertex, so a gap is taken
    /// between two cells at whatever dimension they are
    /// (`[assertions.quiet]`), or a plane pose (a split's tool, a frame
    /// as its xy plane) against a point. Between a face and a
    /// lower-dimensional cell, or an edge and a vertex, the
    /// higher-dimensional cell's material reads the other in either
    /// seat; between two edges or two vertices the outer's reads the
    /// inner ([`mod@crate::eval::measure`]).
    Gap {
        /// The containing carrier.
        outer: R,
        /// The contained carrier.
        inner: R,
    },
}

impl<R> MeasurePrimitive<R> {
    /// The F1 dimension this primitive yields, fixed per primitive.
    pub fn dim(&self) -> Dimension {
        match self {
            Self::Distance { .. } | Self::Gap { .. } => Dimension::Length,
            Self::Angle { .. } => Dimension::Angle,
        }
    }

    /// The two references, in argument order.
    pub fn refs(&self) -> [&R; 2] {
        match self {
            Self::Distance { a, b } | Self::Angle { a, b } => [a, b],
            Self::Gap { outer, inner } => [outer, inner],
        }
    }

    /// The two references, in argument order, exclusive.
    pub fn refs_mut(&mut self) -> [&mut R; 2] {
        match self {
            Self::Distance { a, b } | Self::Angle { a, b } => [a, b],
            Self::Gap { outer, inner } => [outer, inner],
        }
    }

    /// The same primitive over other references, `f` applied to each
    /// in argument order; the first refusal is the answer.
    ///
    /// # Errors
    ///
    /// `f`'s first.
    pub fn try_map<R2, E>(
        &self,
        mut f: impl FnMut(&R) -> Result<R2, E>,
    ) -> Result<MeasurePrimitive<R2>, E> {
        Ok(match self {
            Self::Distance { a, b } => MeasurePrimitive::Distance { a: f(a)?, b: f(b)? },
            Self::Angle { a, b } => MeasurePrimitive::Angle { a: f(a)?, b: f(b)? },
            Self::Gap { outer, inner } => MeasurePrimitive::Gap {
                outer: f(outer)?,
                inner: f(inner)?,
            },
        })
    }

    /// The same primitive over other references, `f` applied to each in
    /// argument order.
    pub fn map<R2>(&self, mut f: impl FnMut(&R) -> R2) -> MeasurePrimitive<R2> {
        match self.try_map(|r| Ok::<_, core::convert::Infallible>(f(r))) {
            Ok(mapped) => mapped,
            Err(never) => match never {},
        }
    }

    /// The primitive's name, for diagnostics and the wire.
    pub fn verb(&self) -> &'static str {
        self.kind().verb()
    }

    /// Which primitive this is, without its references.
    pub fn kind(&self) -> MeasureVerb {
        match self {
            Self::Distance { .. } => MeasureVerb::Distance,
            Self::Angle { .. } => MeasureVerb::Angle,
            Self::Gap { .. } => MeasureVerb::Gap,
        }
    }
}

/// **Which primitive a measure is, and what its references admit**
/// (FORK-VTX): the kinds a reference of each primitive may read, fixed
/// by the primitive and checked at the edit and load doors as any
/// operand seat's kind is, since a selection's kind is fixed when it is
/// minted.
///
/// | primitive | a reference reads |
/// | --- | --- |
/// | `distance` | a `Face`, an `Edge` or a `Vertex` |
/// | `angle` | a `Face` or an `Edge` |
/// | `gap` | a `Face`, an `Edge` or a `Vertex` |
///
/// What stays at evaluation is the carrier CLASS (a cone face in a
/// `distance`, a pair with no closed form): a surface class is not a
/// kind ([`MeasureUnsupported`](crate::eval::measure::MeasureUnsupported)).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum MeasureVerb {
    /// [`MeasurePrimitive::Distance`].
    Distance,
    /// [`MeasurePrimitive::Angle`].
    Angle,
    /// [`MeasurePrimitive::Gap`].
    Gap,
}

impl MeasureVerb {
    /// The primitive's name, for diagnostics and the wire.
    #[must_use]
    pub const fn verb(self) -> &'static str {
        match self {
            Self::Distance => "distance",
            Self::Angle => "angle",
            Self::Gap => "gap",
        }
    }

    /// Whether a reference of this primitive may read a variable of
    /// `kind`.
    #[must_use]
    pub const fn admits(self, kind: crate::VarKind) -> bool {
        use crate::VarKind as K;
        match self {
            Self::Distance => matches!(
                kind,
                K::Face | K::Edge | K::Vertex | K::Point | K::Axis | K::Plane | K::Frame
            ),
            Self::Angle => matches!(kind, K::Face | K::Edge),
            Self::Gap => matches!(kind, K::Face | K::Edge | K::Vertex | K::Plane | K::Frame),
        }
    }

    /// The admitted kinds as a reader says them, read off
    /// [`Self::admits`]: "a face, an edge or a vertex".
    #[must_use]
    pub fn admitted(self) -> String {
        use crate::VarKind as K;
        let said: Vec<String> = [
            K::Body,
            K::Face,
            K::Edge,
            K::Vertex,
            K::Point,
            K::Axis,
            K::Plane,
            K::Frame,
        ]
        .into_iter()
        .filter(|&kind| self.admits(kind))
        .map(|kind| {
            let word = kind.to_string();
            format!("{} {word}", crate::sentence::article(&word))
        })
        .collect();
        match said.as_slice() {
            [] => String::new(),
            [one] => one.clone(),
            [init @ .., last] => format!("{} or {last}", init.join(", ")),
        }
    }
}

/// One seat of a clearance assertion ([`Claim::Clearance`]), resolved:
/// the body the reference landed in, where it was read, and the faces
/// its entity kind selects.
///
/// The resolution is the evaluator's — it already walked the N5 ladder
/// to read the carrier of every other primitive's reference — so this
/// carries the ANSWER of that walk and no naming machinery.
pub struct ClearanceSeat<'b, T: geom_core::Real> {
    /// The node the body was read at.
    pub at: crate::node::RecipeNodeId,
    /// Which of that node's output bodies.
    pub index: u32,
    /// The body itself, at this lane's scalar.
    pub body: &'b topo::Body<T>,
    /// The faces in scope: every face of the body for a body-kind
    /// reference, the one face for a face-kind reference.
    pub faces: Vec<topo::entity::FaceKey>,
}

impl<'b, T: geom_core::Real> ClearanceSeat<'b, T> {
    /// The seat as the engine's side: the body and its faces, without
    /// the address.
    pub fn side(&self) -> topo::clearance::Side<'b, T> {
        topo::clearance::Side {
            body: self.body,
            faces: self.faces.clone(),
        }
    }

    /// Whether two seats read one body: the same output of the same
    /// node. Two such seats that meet are at separation 0, read off the
    /// topology they share.
    pub fn same_body(&self, other: &Self) -> bool {
        self.at == other.at && self.index == other.index
    }
}

/// The relation an [`Assertion`](crate::Node::Assertion) states
/// between its value and its bound (D10: `≤`, `≥`, `=`).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum AssertionRelation {
    /// The measured quantity must be at least the bound.
    AtLeast,
    /// The measured quantity must be at most the bound.
    AtMost,
    /// The measured quantity must equal the bound, at the document's
    /// tolerance (a Zero margin, D4).
    Equal,
}

impl AssertionRelation {
    /// The relation as it reads in a report.
    pub fn symbol(self) -> &'static str {
        match self {
            Self::AtLeast => ">=",
            Self::AtMost => "<=",
            Self::Equal => "=",
        }
    }
}

/// **What an [`Assertion`](crate::Node::Assertion) claims** (E10, E3's
/// clearance amendment): a relation between a scalar variable and a
/// bound, or a clearance between two seats bounded below, above or both.
///
/// The two are one node kind because they are one thing to every
/// reader but the evaluator: a report-only verdict that checks and
/// never places. A band is a clearance's alone, so the type admits it
/// nowhere else.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, untagged)]
#[serde(bound(serialize = "", deserialize = ""))]
pub enum Claim<S: crate::expr::Slot = crate::VarId> {
    /// `value relation bound`.
    Value {
        /// The value checked: any scalar variable, typically an observed
        /// one (a measure's output, or a definition over outputs). Not a
        /// slot: a slot's address fixes its dimension, and this one's is
        /// whatever the variable's is.
        value: S,
        /// The bound, in the value's dimension; a mismatch is a typed
        /// document error at every door, never a silent comparison of
        /// radians with metres.
        bound: S,
        /// How the value must relate to the bound: `>=`, `<=` or `=`.
        relation: AssertionRelation,
    },
    /// **`clearance(a, b)` within `bounds`.** Each seat reads a `Body`
    /// (all of its faces) or a `Face` (that one), the two seats of
    /// [`crate::OperandSlot::Cleared`]. No clearance value exists: the
    /// verdict comes from E7's sweep at the bounds, and the minimum is
    /// never computed.
    Clearance {
        /// The first seat.
        a: S::Read,
        /// The second.
        b: S::Read,
        /// The bounds, each a length.
        bounds: ClearanceBounds<S>,
    },
}

impl<S: crate::expr::Slot> Claim<S> {
    /// The payload expressions in their one order: a value claim's value
    /// then bound, a clearance's bounds low to high.
    pub fn exprs(&self) -> Vec<&S> {
        match self {
            Self::Value { value, bound, .. } => vec![value, bound],
            Self::Clearance { bounds, .. } => bounds.exprs(),
        }
    }

    /// [`Self::exprs`], exclusive.
    pub fn exprs_mut(&mut self) -> Vec<&mut S> {
        match self {
            Self::Value { value, bound, .. } => vec![value, bound],
            Self::Clearance { bounds, .. } => bounds.exprs_mut(),
        }
    }

    /// The same claim with every payload expression rewritten by `f` and
    /// every seat by `read`; the first refusal is the answer.
    ///
    /// # Errors
    ///
    /// `f`'s or `read`'s first.
    pub fn try_map<S2: crate::expr::Slot, E>(
        &self,
        mut f: impl FnMut(&S) -> Result<S2, E>,
        mut read: impl FnMut(crate::OperandSlot, &S::Read) -> Result<S2::Read, E>,
    ) -> Result<Claim<S2>, E> {
        Ok(match self {
            Self::Value {
                value,
                bound,
                relation,
            } => Claim::Value {
                value: f(value)?,
                bound: f(bound)?,
                relation: *relation,
            },
            Self::Clearance { a, b, bounds } => Claim::Clearance {
                a: read(crate::OperandSlot::Cleared(0), a)?,
                b: read(crate::OperandSlot::Cleared(1), b)?,
                bounds: bounds.try_map(&mut f)?,
            },
        })
    }
}

/// **A clearance's bounds**: below, above, or both. `= c` keeps its own
/// spelling, so a report reads the equation the author wrote, and is
/// decided as the band with `lo = hi = c`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ClearanceBounds<S> {
    /// `clearance(a, b) ≥ c`.
    AtLeast(S),
    /// `clearance(a, b) ≤ c`.
    AtMost(S),
    /// `clearance(a, b) = c`.
    Equal(S),
    /// `lo ≤ clearance(a, b) ≤ hi`.
    Within {
        /// The lower bound.
        lo: S,
        /// The upper bound.
        hi: S,
    },
}

impl<S> ClearanceBounds<S> {
    /// The bound expressions, low to high (`= c` reads one).
    pub fn exprs(&self) -> Vec<&S> {
        match self {
            Self::AtLeast(c) | Self::AtMost(c) | Self::Equal(c) => vec![c],
            Self::Within { lo, hi } => vec![lo, hi],
        }
    }

    /// [`Self::exprs`], exclusive.
    pub fn exprs_mut(&mut self) -> Vec<&mut S> {
        match self {
            Self::AtLeast(c) | Self::AtMost(c) | Self::Equal(c) => vec![c],
            Self::Within { lo, hi } => vec![lo, hi],
        }
    }

    /// The same bounds over other expressions.
    ///
    /// # Errors
    ///
    /// `f`'s first.
    pub fn try_map<S2, E>(
        &self,
        mut f: impl FnMut(&S) -> Result<S2, E>,
    ) -> Result<ClearanceBounds<S2>, E> {
        Ok(match self {
            Self::AtLeast(c) => ClearanceBounds::AtLeast(f(c)?),
            Self::AtMost(c) => ClearanceBounds::AtMost(f(c)?),
            Self::Equal(c) => ClearanceBounds::Equal(f(c)?),
            Self::Within { lo, hi } => ClearanceBounds::Within {
                lo: f(lo)?,
                hi: f(hi)?,
            },
        })
    }

    /// The same shape over the bounds' evaluated values, in
    /// [`Self::exprs`] order.
    ///
    /// # Panics
    ///
    /// When `values` is not one value per expression: the evaluator
    /// hands back what [`Self::exprs`] asked for.
    pub fn with<T: Copy>(&self, values: &[T]) -> ClearanceBounds<T> {
        match (self, values) {
            (Self::AtLeast(_), [c]) => ClearanceBounds::AtLeast(*c),
            (Self::AtMost(_), [c]) => ClearanceBounds::AtMost(*c),
            (Self::Equal(_), [c]) => ClearanceBounds::Equal(*c),
            (Self::Within { .. }, [lo, hi]) => ClearanceBounds::Within { lo: *lo, hi: *hi },
            _ => unreachable!("one evaluated value per bound expression"),
        }
    }

    /// The relation as the row reads it: `clearance(a, b) ≥ c`, with each
    /// bound said by `say`.
    pub fn equation(&self, a: &str, b: &str, mut say: impl FnMut(&S) -> String) -> String {
        match self {
            Self::AtLeast(c) => format!("clearance({a}, {b}) ≥ {}", say(c)),
            Self::AtMost(c) => format!("clearance({a}, {b}) ≤ {}", say(c)),
            Self::Equal(c) => format!("clearance({a}, {b}) = {}", say(c)),
            Self::Within { lo, hi } => {
                format!("{} ≤ clearance({a}, {b}) ≤ {}", say(lo), say(hi))
            }
        }
    }
}

impl ClearanceBounds<geom_core::Interval> {
    /// The engine's question at these bounds.
    #[must_use]
    pub fn sweep_bound(&self) -> topo::clearance::ClearanceBound {
        use topo::clearance::ClearanceBound as B;
        match *self {
            Self::AtLeast(c) => B::AtLeast(c),
            Self::AtMost(c) => B::AtMost(c),
            Self::Equal(c) => B::Within { lo: c, hi: c },
            Self::Within { lo, hi } => B::Within { lo, hi },
        }
    }
}

/// **An assertion's evaluated verdict — REPORT ONLY (D10, E10).**
///
/// This is the whole of an assertion's product. Nothing downstream
/// reads it: no gate consults it, no op takes it as an operand, and no
/// product or export changes shape because of it. That is structural
/// rather than promised — an assertion node denotes no body, so the
/// product gather skips it exactly as it skips a declaration, and its
/// value payload is not an admissible operand for any op in the
/// vocabulary. A `Violated` verdict is therefore a fact a REPORT reads:
/// an assertion checks and never places (D10).
#[derive(Debug, Clone, PartialEq)]
pub enum AssertionVerdict<T> {
    /// The measured value satisfies the bound.
    Holds {
        /// What the measure evaluated to.
        measured: T,
        /// What the bound evaluated to.
        bound: T,
    },
    /// The measured value violates the bound — BOTH numbers, because a
    /// verdict without them cannot be acted on.
    Violated {
        /// What the measure evaluated to.
        measured: T,
        /// What the bound evaluated to.
        bound: T,
    },
    /// No verdict is available: the comparison itself could not be
    /// decided at the run's tolerance, so the assertion says so rather
    /// than picking a side.
    Unevaluated {
        /// Why, in the reporting layer's own words.
        reason: UnevaluatedReason,
    },
    /// **A clearance requirement's answer** ([`Claim::Clearance`]): the
    /// sweep's report, whose verdict holds (with an upper bound's
    /// proving pair), is violated (with the pair below the lower bound,
    /// or with every pair clear of the upper one), or refuses typed —
    /// the third state, never a poisoned node. At a scalar that cannot
    /// certify, that third state names the lane that can
    /// ([`crate::clearance::ClearanceRefusal::NeedsCertifyingLane`]).
    Clearance(Box<crate::clearance::ClearanceReport>),
}

impl<T> AssertionVerdict<T> {
    /// The same verdict with both numbers taken through `f` — the door
    /// a lane that evaluated at a WRAPPED scalar reports through, so a
    /// consumer sees the enclosure and not the wrapper.
    pub fn map<U>(self, f: impl Fn(T) -> U) -> AssertionVerdict<U> {
        match self {
            Self::Holds { measured, bound } => AssertionVerdict::Holds {
                measured: f(measured),
                bound: f(bound),
            },
            Self::Violated { measured, bound } => AssertionVerdict::Violated {
                measured: f(measured),
                bound: f(bound),
            },
            Self::Unevaluated { reason } => AssertionVerdict::Unevaluated { reason },
            Self::Clearance(report) => AssertionVerdict::Clearance(report),
        }
    }
}

/// Why an assertion produced no verdict.
///
/// The upstream-FAILURE lanes are NOT here: a failed or poisoned
/// measure poisons its assertion through the ordinary DAG edge (F2),
/// so the assertion has no value at all rather than an
/// `Unevaluated` one. What is left is the case where the measure node
/// itself came out fine and the COMPARISON still has no answer: the
/// margin was undecidable.
// `Eq` is not derived: the undecided arm keeps the escalation, whose
// reporting margin is floating point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnevaluatedReason {
    /// The margin between measured and bound landed in the sliver
    /// band: the run's tolerance cannot separate them, and guessing a
    /// side would manufacture the certainty the band exists to deny.
    ///
    /// **It keeps the escalation**, so the refusal ends through
    /// [`ASSERT_BOUND_DECISION`] with the value its own margin gives
    /// (D4 ¶1 (i)) rather than offering an unvalued tighten that a
    /// poisoned margin and an enclosure straddling zero would both
    /// receive.
    Indeterminate {
        /// The escalation of the measured-versus-bound decision, with
        /// the reporting margin it was classified on.
        cause: geom_core::Indeterminate,
    },
}

impl core::fmt::Display for UnevaluatedReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Indeterminate { cause } => write!(
                f,
                "the measured value and the bound are not separated at this tolerance, so the \
                 assertion has no verdict. {}",
                ASSERT_BOUND_DECISION.recourse(RefusedArm::Undecided(cause), Reading::AtRest)
            ),
        }
    }
}

impl<T> AssertionVerdict<T> {
    /// Does the assertion hold? `None` when there is no verdict — the
    /// three states stay three at every reader, so nothing collapses
    /// `Unevaluated` into a silent pass.
    pub fn holds(&self) -> Option<bool> {
        match self {
            Self::Holds { .. } => Some(true),
            Self::Violated { .. } => Some(false),
            Self::Unevaluated { .. } => None,
            Self::Clearance(report) => report.verdict().holds(),
        }
    }

    /// The verdict's state as a word, for reports.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Holds { .. } => "Holds",
            Self::Violated { .. } => "Violated",
            Self::Unevaluated { .. } => "Unevaluated",
            Self::Clearance(report) => report.verdict().label(),
        }
    }
}

/// **The signed comparison an assertion makes**, decided through the
/// one `k_stats` funnel under the existing `assert_bound` predicate
/// name.
///
/// The comparand is `measured − bound` for `AtLeast` and `Equal` and
/// its negation for `AtMost`, in the MEASURE's own dimension. `Equal`
/// holds on a Zero margin and is violated by either definite sign; the
/// sliver band is `Unevaluated`, as for the other two.
///
/// # Dimension (audit F16, `docs/predicate-dimension-audit.md`)
///
/// Length measures make that comparand honest metres against the
/// linear band; Angle measures make it RADIANS against the same band,
/// which is the audit's dimensionless-comparand shape. E3 forecloses
/// the obvious repair — a lever arm would need a chosen length scale,
/// which it rejects by name — so the site is FLAGGED, not cast, and
/// the row argues it.
///
/// An `Indeterminate` escalation becomes
/// [`AssertionVerdict::Unevaluated`] rather than a node failure: a
/// bound the run cannot separate from the measurement is a fact about
/// the report, not a broken document.
pub(crate) fn decide_assertion<T: Decide>(
    measured: T,
    bound: T,
    relation: AssertionRelation,
    band: geom_core::Band,
) -> AssertionVerdict<T> {
    let comparand = match relation {
        AssertionRelation::AtLeast | AssertionRelation::Equal => measured - bound,
        AssertionRelation::AtMost => bound - measured,
    };
    let sign = match geom_core::k_stats::decide_flagged(ASSERT_BOUND, comparand, band, "F16") {
        Ok(sign) => sign,
        Err(cause) => {
            return AssertionVerdict::Unevaluated {
                reason: UnevaluatedReason::Indeterminate { cause },
            };
        }
    };
    // At the bound exactly, a non-strict relation holds.
    use AssertionRelation::{AtLeast, AtMost, Equal};
    use geom_core::Sign::{Negative, Positive, Zero};
    let holds = match (relation, sign) {
        (AtLeast | AtMost, Positive | Zero) | (Equal, Zero) => true,
        (AtLeast | AtMost, Negative) | (Equal, Positive | Negative) => false,
    };
    if holds {
        AssertionVerdict::Holds { measured, bound }
    } else {
        AssertionVerdict::Violated { measured, bound }
    }
}

/// The funnel site name of the assertion comparison. A roster carrier
/// (`docs/K-REPORT.md`) rather than a literal at the decide site.
pub const ASSERT_BOUND: &str = "assert_bound";

/// **The assertion's own decision**, as the one ending table reads it
/// (D4 ¶1 (i)): the comparand is `measured − bound` and BOTH definite
/// signs are verdicts — at the bound exactly a non-strict relation holds
/// — so the only refused arm is the undecided one, and the lever is the
/// bound the document names. A smaller tolerance decides an in-band
/// margin, so the offer is valued from that margin; a straddling
/// enclosure and an unreadable margin get the lever alone, which is the
/// defect this spelling closes.
pub const ASSERT_BOUND_DECISION: SizedDecision = SizedDecision {
    lever: "move the bound",
    size: "difference",
    passes: SizedPass::AnySign,
    // The stored description the lever edits IS the asserted bound.
    side: RefusedSide::Lever,
    no_size: None,
};

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{AssertionRelation, AssertionVerdict, UnevaluatedReason};
    use geom_core::{Band, Tol};

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the run's band")
    }

    fn decide(measured: f64, bound: f64, relation: AssertionRelation) -> &'static str {
        super::decide_assertion(measured, bound, relation, band()).label()
    }

    /// `=` holds on a Zero margin, is violated by either definite sign,
    /// and leaves the sliver band undecided. Breaks if `Equal` reuses an
    /// end of `AtLeast` (`10 = 9` holds) or loses the band arm (a margin
    /// of a few ε is called `Violated`).
    #[test]
    fn equal_decides_zero_both_signs_and_the_band() {
        use AssertionRelation::{AtLeast, AtMost, Equal};
        let eps = Tol::witness().eps();
        let ten = 0.010;
        assert_eq!(decide(ten, ten, Equal), "Holds");
        assert_eq!(decide(ten, ten + eps / 2.0, Equal), "Holds");
        // Past the band at every ε row (K·ε is 10ε).
        let off = 100.0 * eps;
        assert_eq!(decide(ten, ten + off, Equal), "Violated");
        assert_eq!(decide(ten, ten - off, Equal), "Violated");
        assert_eq!(decide(ten, 0.009, Equal), "Violated");
        assert_eq!(decide(ten, 0.009, AtLeast), "Holds");
        assert_eq!(decide(ten, 0.009, AtMost), "Violated");
        let sliver = ten + 3.0 * eps;
        match super::decide_assertion(ten, sliver, Equal, band()) {
            AssertionVerdict::Unevaluated {
                reason: UnevaluatedReason::Indeterminate { .. },
            } => {}
            other => panic!("a margin of 3ε is undecided under `=`, not {other:?}"),
        }
    }
}
