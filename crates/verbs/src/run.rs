//! The dispatch sites over the kernel's op doors, one per declared
//! arity, and what comes back out of them.

use core::fmt;

use geom_core::{Bounds, Decide, Real, Tol};
use sweep::blend::BlendRefusal;
use sweep::blend::naming::BlendNaming;
use topo::{
    Body, BooleanError, BooleanNaming, BooleanResult, BooleanResultKind, ContactRecords,
    SweepStrategy, boolean_op_with,
};

use crate::verb::{Arity, Verb, VerbKind};

/// **What a verb produced**: the body, and the operation's own record
/// of what it minted, in the record channel for the verb's family.
///
/// No record is RESTATED here — each family's channel carries the
/// operation's own value across ([`VerbRecord`]), so a change to what
/// the blend surgery or the boolean pipeline records reaches this door
/// with no edit.
///
/// The blend doors also hand back their solid, shell and per-role face
/// lists. Those are geometry a caller who wants them reads off the door
/// directly; the lowering reads the body and the record, so those are
/// what a verb result carries.
#[derive(Debug)]
pub struct VerbOut<T: Real> {
    /// The operation's output body.
    pub body: Body<T>,
    /// The operation's own record of the result, per family.
    pub record: VerbRecord,
}

/// **The record channel, one variant per record family** — the
/// operation's own types, moved across by value, never restated.
///
/// Closed with no wildcard arm (D3): a verb family with a new record
/// shape adds a variant, and every consumer of the channel is forced to
/// visit it. Which variant a given verb's run produces is fixed by the
/// verb's family; a consumer holding the other family's variant is
/// holding a kernel bug and refuses typed rather than emitting names
/// from the wrong record (the same class as a blend body arriving with
/// `None` records).
#[derive(Debug)]
pub enum VerbRecord {
    /// A blend door's per-entity birth records. The `Option` is the op
    /// door's own and its whole point is that an EMPTY record must not
    /// be constructible: `None` says "this body has no birth records",
    /// which the document layer refuses as a kernel bug, and an empty
    /// struct would be refused by nothing.
    Blend(Option<BlendNaming>),
    /// The boolean's non-body result, whole — the three fields of the
    /// kernel's `BooleanBody` beside the body itself, each the
    /// kernel's own type moved across by value.
    Boolean {
        /// How the result body came to be.
        kind: BooleanResultKind,
        /// Declared contacts surviving into the result, result keys —
        /// the tier-3′ currency the consuming layer carries onward.
        contacts: ContactRecords,
        /// Mint-time naming facts the naming layer consumes.
        naming: BooleanNaming,
    },
}

/// **What a two-operand verb produced**: the typed empty success, or a
/// body with its record.
///
/// `Empty` exists because it is part of the boolean's contract (F8: a
/// regularized result that vanishes is a VALUE, not an error), and the
/// one-operand door does not carry it because no one-operand verb can
/// produce it — the split is per DOOR, so a blend consumer never
/// handles an emptiness its verbs cannot mean.
// Size skew vs `Empty` is inherent (same posture as `BooleanResult`).
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum PairOut<T: Real> {
    /// The regularized result is empty.
    Empty,
    /// A real result body and its record.
    Out(VerbOut<T>),
}

/// **Why a verb refused**, carrying the op door's own typed refusal
/// unaltered.
///
/// Closed with no wildcard arm (D3): the arms grow one per migrated op
/// family, and a consumer that renders refusals is forced to visit the
/// new one. The blend pair shares a single arm because the two doors
/// share one refusal vocabulary and the door itself records which verb
/// refused (`BlendRefusal::verb`).
#[derive(Debug)]
pub enum VerbError {
    /// A blend door refused.
    Blend(BlendRefusal),
    /// The boolean pipeline refused.
    Boolean(BooleanError),
    /// A run door was handed a different operand count than the verb
    /// declares ([`VerbKind::arity`]) — a caller wiring bug surfaced
    /// typed, never a panic. Unreachable through a lowering that
    /// consults the declaration; a direct caller that picks the wrong
    /// door is told so by name.
    Arity {
        /// The verb whose door refused.
        verb: VerbKind,
        /// The operand count the door was handed (the door's own
        /// arity, so the declared one is `verb.arity()`).
        given: Arity,
    },
}

impl fmt::Display for VerbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blend(refusal) => write!(f, "{refusal}"),
            Self::Boolean(refusal) => write!(f, "{refusal}"),
            Self::Arity { verb, given } => write!(
                f,
                "the {verb:?} verb declares {:?} operand(s) and was run through the {given:?}-operand door",
                verb.arity()
            ),
        }
    }
}

impl core::error::Error for VerbError {}

impl<T: Decide + Bounds + geom_brep::PcurveFittedLane> Verb<T> {
    /// **Run this one-operand verb against its operand.**
    ///
    /// The operand comes in borrowed, never in the payload. Every
    /// check, every refusal and every minted entity is the op door's —
    /// this dispatches and re-wraps, and adds no decision of its own.
    ///
    /// # Errors
    ///
    /// [`VerbError::Blend`] carrying the door's [`BlendRefusal`]
    /// verbatim; the door's own docs
    /// (`sweep::blend::build::fillet_edges`,
    /// `sweep::blend::build::chamfer_edges`) enumerate the cases.
    /// [`VerbError::Arity`] if this verb declares two operands.
    pub fn run(&self, operand: &Body<T>, tol: Tol) -> Result<VerbOut<T>, VerbError> {
        let blended = match self {
            Self::Fillet { edges, radius } => {
                sweep::blend::build::fillet_edges(operand, edges, *radius, tol)
            }
            Self::Chamfer { edges, distance } => {
                sweep::blend::build::chamfer_edges(operand, edges, *distance, tol)
            }
            Self::Boolean { .. } => {
                return Err(VerbError::Arity {
                    verb: self.kind(),
                    given: Arity::One,
                });
            }
        }
        .map_err(VerbError::Blend)?;
        Ok(VerbOut {
            body: blended.body,
            record: VerbRecord::Blend(blended.naming),
        })
    }

    /// **Run this two-operand verb against its operands.**
    ///
    /// Both bodies come in borrowed, in operand order. `sweep` is the
    /// candidate-generation strategy — a property of the RUN, not of
    /// the operation (both strategies produce bit-identical results;
    /// the kernel's differential suite pins it), which is why it rides
    /// beside the tolerance witness here instead of in the payload.
    ///
    /// # Errors
    ///
    /// [`VerbError::Boolean`] carrying the pipeline's [`BooleanError`]
    /// verbatim (`topo::boolean_op_with` enumerates the cases);
    /// [`VerbError::Arity`] if this verb declares one operand.
    pub fn run_pair(
        &self,
        a: &Body<T>,
        b: &Body<T>,
        sweep: SweepStrategy,
        tol: Tol,
    ) -> Result<PairOut<T>, VerbError> {
        match self {
            Self::Boolean { op, declare } => {
                match boolean_op_with(*op, a, b, declare, sweep, tol).map_err(VerbError::Boolean)? {
                    BooleanResult::Empty => Ok(PairOut::Empty),
                    BooleanResult::Body(bb) => Ok(PairOut::Out(VerbOut {
                        body: bb.body,
                        record: VerbRecord::Boolean {
                            kind: bb.kind,
                            contacts: bb.contacts,
                            naming: bb.naming,
                        },
                    })),
                }
            }
            Self::Fillet { .. } | Self::Chamfer { .. } => Err(VerbError::Arity {
                verb: self.kind(),
                given: Arity::Two,
            }),
        }
    }
}
