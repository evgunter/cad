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
    /// **The minimum clearance between two selections** →
    /// [`Dimension::Length`] (E3's last v1 primitive, E7's engine).
    ///
    /// Each reference's ENTITY KIND is the selection's face scope: a
    /// reference to a BODY selects all of that body's faces, a
    /// reference to a FACE selects that one. Those are the two scopes
    /// M10-5's `clearance::FaceScope` carries. A reference to anything
    /// else (a vertex, an edge, a pose) refuses typed.
    ///
    /// **Its value is an enclosure, so it exists only where enclosures
    /// do.** At `f64`, `Probe` and `Dual<f64>` the measure has no
    /// value at all and says so
    /// ([`crate::eval::ValuePayload::MeasureUnavailable`]): a station
    /// pair found by a point-scalar search is an upper bound on the
    /// minimum and not the minimum. At `Interval`, and at the symbolic
    /// tier over it, the value over a leaf IS
    /// `topo::clearance::min_separation`'s bracket (at the tier, each end
    /// re-entered as a computed value).
    MinClearance {
        /// The first selection.
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
            Self::Distance { .. } | Self::Gap { .. } | Self::MinClearance { .. } => {
                Dimension::Length
            }
            Self::Angle { .. } => Dimension::Angle,
        }
    }

    /// The two references, in argument order.
    pub fn refs(&self) -> [&R; 2] {
        match self {
            Self::Distance { a, b } | Self::Angle { a, b } | Self::MinClearance { a, b } => [a, b],
            Self::Gap { outer, inner } => [outer, inner],
        }
    }

    /// The two references, in argument order, exclusive.
    pub fn refs_mut(&mut self) -> [&mut R; 2] {
        match self {
            Self::Distance { a, b } | Self::Angle { a, b } | Self::MinClearance { a, b } => [a, b],
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
            Self::MinClearance { a, b } => MeasurePrimitive::MinClearance { a: f(a)?, b: f(b)? },
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
            Self::MinClearance { .. } => MeasureVerb::MinClearance,
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
/// | `min_clearance` | a `Body` or a `Face` |
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
    /// [`MeasurePrimitive::MinClearance`].
    MinClearance,
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
            Self::MinClearance => "min_clearance",
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
            Self::MinClearance => matches!(kind, K::Body | K::Face),
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

/// **Why a measure has no value at the scalar the build ran at**
/// ([`crate::eval::ValuePayload::MeasureUnavailable`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasureUnavailableAt {
    /// The primitive's answer is an ENCLOSURE, and this build's scalar
    /// is a point: it has nowhere to put one.
    ///
    /// Not a degradation and not a fallback — the two are the same
    /// thing said twice, which is why this carries the DOOR that can
    /// answer instead of a number: a reader is told where the answer
    /// lives, not handed a worse one.
    NeedsEnclosure {
        /// Which primitive.
        verb: &'static str,
        /// The scalar this build ran at, in its own name
        /// ([`geom_core::Real::NAME`]).
        scalar: &'static str,
        /// The door that answers it, named so the recourse is in the
        /// refusal rather than in a reader's memory.
        door: &'static str,
    },
    /// The engine's bracket has a lower end and no upper one: no pair it
    /// examined within its cell budget was a pair of points of the two
    /// scopes (`topo::clearance::MinSeparation::hi`). The lower end alone
    /// is not an enclosure of the measure, so there is no value.
    NoUpperBound {
        /// Which primitive.
        verb: &'static str,
        /// The scalar this build ran at, in its own name
        /// ([`geom_core::Real::NAME`]).
        scalar: &'static str,
        /// The door that answers it at a larger cell budget.
        door: &'static str,
    },
}

impl MeasureUnavailableAt {
    /// The primitive whose answer is unavailable — the one word a
    /// goldening form needs, without spelling the whole prose.
    pub fn verb(&self) -> &'static str {
        match self {
            Self::NeedsEnclosure { verb, .. } | Self::NoUpperBound { verb, .. } => verb,
        }
    }
}

impl core::fmt::Display for MeasureUnavailableAt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NeedsEnclosure { verb, scalar, door } => write!(
                f,
                "`{verb}` answers with a certified enclosure, which only the {interval} \
                 scalar computes; a {scalar} point search finds an upper bound on the minimum, \
                 not the minimum. Recourse: evaluate the document at the {interval} scalar \
                 over a parameter box, where `{door}` computes the bracket",
                interval = <geom_core::Interval as geom_core::Real>::NAME,
            ),
            Self::NoUpperBound { verb, scalar, door } => write!(
                f,
                "`{verb}` found no pair of points of the two scopes within its cell budget at \
                 the {scalar} scalar, so its bracket has a lower end and no upper end. \
                 Recourse: `{door}` at a larger cell budget"
            ),
        }
    }
}

/// One side of a [`MeasurePrimitive::MinClearance`], resolved: the body
/// the reference landed in, where it was read, and the faces its entity
/// kind selects.
///
/// The resolution is the evaluator's — it already walked the N5 ladder
/// to read the carrier of every other primitive's reference — so this
/// carries the ANSWER of that walk and no naming machinery.
pub struct MinClearanceOperand<'b, T: geom_core::Real> {
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

impl<'b, T: geom_core::Real> MinClearanceOperand<'b, T> {
    /// The operand as the engine's side: the body and its faces, without
    /// the address.
    pub fn side(&self) -> topo::clearance::Side<'b, T> {
        topo::clearance::Side {
            body: self.body,
            faces: self.faces.clone(),
        }
    }

    /// Whether two operands read one body: the same output of the same
    /// node. Two such operands that meet are at separation 0, read off
    /// the topology they share.
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
        }
    }
}

/// Why an assertion produced no verdict.
///
/// The upstream-FAILURE lanes are NOT here: a failed or poisoned
/// measure poisons its assertion through the ordinary DAG edge (F2),
/// so the assertion has no value at all rather than an
/// `Unevaluated` one. What is left is the two cases where the measure
/// node itself came out fine and the COMPARISON still has no answer:
/// the margin was undecidable, or there was no measured value to
/// compare — a typed absence
/// ([`crate::eval::ValuePayload::MeasureUnavailable`]), which is a
/// value and not a failure, and therefore reaches here rather than
/// poisoning.
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
    /// The measure has no value at this build's scalar, and says why.
    /// E10's third state used for exactly what it is for: the
    /// requirement is recorded, the run cannot answer it, and neither
    /// half of that is hidden.
    MeasureUnavailable(MeasureUnavailableAt),
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
            Self::MeasureUnavailable(why) => write!(f, "there is no measured value: {why}"),
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
        }
    }

    /// The verdict's state as a word, for reports.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Holds { .. } => "Holds",
            Self::Violated { .. } => "Violated",
            Self::Unevaluated { .. } => "Unevaluated",
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
