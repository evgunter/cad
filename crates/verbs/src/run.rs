//! The dispatch sites over the kernel's op doors, one per declared
//! door ([`Arity`]), and what comes back out of them.

use core::fmt;

use geom_core::{Bounds, Decide, Real, Tol};
use profile::ValidatedProfile;
use sweep::blend::BlendRefusal;
use sweep::blend::naming::BlendNaming;
use sweep::{ExtrudeError, Extruded, RevolveError, Revolved};
use topo::splitting::SplitNaming;
use topo::{
    Body, BooleanError, BooleanNaming, BooleanResult, BooleanResultKind, ContactRecords,
    SplitError, SplitPart, SplitResult, SweepStrategy, boolean_op_with, split,
};

use crate::verb::{Arity, Verb, VerbKind};

/// **What a verb produced**: the body, and the operation's own record
/// of what it minted, in the record channel for the verb's family.
///
/// No record is RESTATED here — each family's channel carries the
/// operation's own value across ([`VerbRecord`]), so a change INSIDE
/// any moved type reaches this door with no edit. A record GROWN at
/// the top level (a new `BooleanBody` field, a new blend record
/// beside `naming`) is a compile-time visit here instead — the pair
/// door destructures its result exhaustively so a grown field cannot
/// vanish in the move.
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
    pub record: VerbRecord<T>,
}

/// **The record channel, one variant per record family** — the
/// operation's own types, moved across by value, never restated.
///
/// Closed with no wildcard arm (D3): a verb family with a new record
/// shape adds a variant, and the lowerings that consume the channel
/// match it exhaustively — a new family breaks them at compile time
/// and is routed deliberately, never silently refused. Which variant a
/// given verb's run produces is fixed by the verb's family; a consumer
/// handed another family's variant is holding a kernel bug and refuses
/// typed rather than emitting names from the wrong record (the same
/// class as a blend body arriving with `None` records).
///
/// **The sweep families carry their door's whole bundle, body
/// included**, and that is why the profile door hands back a record
/// rather than a [`VerbOut`]. A blend's and a boolean's records are
/// beside their body; `Extruded` and `Revolved` are records WITH the
/// body in them, and the naming door each family owns reads the whole
/// bundle — so splitting the body out would either restate the bundle
/// field by field (the thing this channel exists not to do) or hand
/// the consumer a record its own emitter cannot be called with.
#[derive(Debug)]
pub enum VerbRecord<T: Real> {
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
    /// The extrude door's whole bundle — body, solid, shell, caps,
    /// side walls per loop, struts per loop.
    Extrude(Extruded<T>),
    /// The revolve door's whole bundle — body, solid, shell,
    /// cavities, walls, rims, poles and the case split's own keys.
    Revolve(Revolved<T>),
    /// The split's mint-time naming facts — the section faces with
    /// their side, the chord-mef fragment rows and the null-edge
    /// vertex pairs — the kernel's own type moved across by value.
    /// It sits BESIDE the two sides rather than around them
    /// ([`SplitOut`]): the split's emitter takes the sides and the
    /// record as separate arguments, so nothing is restated by
    /// taking the door's result apart — unlike a sweep's bundle,
    /// whose emitter reads the whole.
    Split(SplitNaming),
}

/// **What the split produced**: its two sides, each a body or the
/// typed empty, under ONE record — the out-type of the split door
/// ([`Verb::run_split`]).
///
/// It is its own out-type, per door, for the reason [`PairOut`] is:
/// a one-body consumer reads [`VerbOut::body`] without matching,
/// and widening that into "one body or two sides" would make every
/// blend consumer handle a two-sidedness its verbs cannot mean. The
/// two sides are the kernel's own [`SplitPart`]s moved across by
/// value — never a list of bodies, because which side is ABOVE is
/// the plane's own fact and a role the consumer selects by, and
/// never a single body with a side marker, because an EMPTY side is
/// a value of the split's contract (a plane that misses the material
/// on one side) and not a failure. The record is the third field of
/// the door's own result moved into the closed channel; the door
/// destructures that result exhaustively, so a field grown onto it
/// breaks the door at compile time instead of vanishing in the move.
#[derive(Debug)]
pub struct SplitOut<T: Real> {
    /// The material on the plane normal's side.
    pub above: SplitPart<T>,
    /// The material on the opposite side.
    pub below: SplitPart<T>,
    /// The split's own record of what it minted, in its channel.
    pub record: VerbRecord<T>,
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
    /// The extrude door refused.
    Extrude(ExtrudeError),
    /// The revolve door refused.
    Revolve(RevolveError),
    /// The split op refused.
    Split(SplitError),
    /// A verb was run through a door that is not the one it declares
    /// ([`VerbKind::arity`]) — a caller wiring bug surfaced typed,
    /// never a panic. Unreachable through a lowering that consults
    /// the declaration; a direct caller that picks the wrong door is
    /// told so by name.
    Arity {
        /// The verb whose door refused.
        verb: VerbKind,
        /// The door the verb was handed to (its own row, so the
        /// declared one is `verb.arity()`).
        given: Arity,
    },
}

impl fmt::Display for VerbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blend(refusal) => write!(f, "{refusal}"),
            Self::Boolean(refusal) => write!(f, "{refusal}"),
            Self::Extrude(refusal) => write!(f, "{refusal}"),
            Self::Revolve(refusal) => write!(f, "{refusal}"),
            Self::Split(refusal) => write!(f, "{refusal}"),
            Self::Arity { verb, given } => write!(
                f,
                "the {verb:?} verb answers the {:?} door and was run through the {given:?} door",
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
    /// [`VerbError::Arity`] if this verb answers another door — its
    /// operand is two bodies or a profile, or it hands back two sides.
    pub fn run(&self, operand: &Body<T>, tol: Tol) -> Result<VerbOut<T>, VerbError> {
        let blended = match self {
            Self::Fillet { edges, radius } => {
                sweep::blend::build::fillet_edges(operand, edges, *radius, tol)
            }
            Self::Chamfer { edges, distance } => {
                sweep::blend::build::chamfer_edges(operand, edges, *distance, tol)
            }
            Self::Boolean { .. }
            | Self::Extrude { .. }
            | Self::Revolve { .. }
            | Self::Split { .. } => {
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
    /// [`VerbError::Arity`] if this verb answers another door — its
    /// operand is one body or a profile.
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
                    BooleanResult::Body(bb) => {
                        // Exhaustive destructure, deliberately: a field
                        // grown onto `BooleanBody` breaks this door at
                        // compile time instead of silently vanishing
                        // in a field-by-field move.
                        let topo::BooleanBody {
                            body,
                            kind,
                            contacts,
                            naming,
                        } = bb;
                        Ok(PairOut::Out(VerbOut {
                            body,
                            record: VerbRecord::Boolean {
                                kind,
                                contacts,
                                naming,
                            },
                        }))
                    }
                }
            }
            Self::Fillet { .. }
            | Self::Chamfer { .. }
            | Self::Extrude { .. }
            | Self::Revolve { .. }
            | Self::Split { .. } => Err(VerbError::Arity {
                verb: self.kind(),
                given: Arity::Two,
            }),
        }
    }

    /// **Run this profile-operand verb against its operand profile.**
    ///
    /// The validated profile comes in BORROWED, never in the payload:
    /// it is the thing operated on, and a declaration owning a clone
    /// of it is what the operand rule exists to prevent.
    ///
    /// What comes back is the record alone, not a [`VerbOut`], for the
    /// reason [`VerbRecord`]'s docs give: a sweep door's bundle IS its
    /// record and the body is a field of it, so a consumer takes the
    /// body out of the record after its emitter has read it.
    ///
    /// # Errors
    ///
    /// [`VerbError::Extrude`] / [`VerbError::Revolve`] carrying the
    /// door's own typed refusal verbatim (`sweep::extrude` and
    /// `sweep::revolve` enumerate the cases); [`VerbError::Arity`] if
    /// this verb answers another door — its operand is one or two
    /// bodies.
    pub fn run_profile(
        &self,
        operand: &ValidatedProfile<T>,
        tol: Tol,
    ) -> Result<VerbRecord<T>, VerbError> {
        match self {
            Self::Extrude { distance } => {
                sweep::extrude(operand, sweep::Extrusion::Distance(*distance), tol)
                    .map(VerbRecord::Extrude)
                    .map_err(VerbError::Extrude)
            }
            Self::Revolve { axis, revolution } => sweep::revolve(operand, *axis, *revolution, tol)
                .map(VerbRecord::Revolve)
                .map_err(VerbError::Revolve),
            Self::Fillet { .. } | Self::Chamfer { .. } | Self::Boolean { .. } | Self::Split { .. } => {
                Err(VerbError::Arity {
                    verb: self.kind(),
                    given: Arity::Profile,
                })
            }
        }
    }

    /// **Run this parting verb against its operand body.**
    ///
    /// The operand comes in borrowed, never in the payload, exactly as
    /// at [`Verb::run`]; what differs is what comes back. A split hands
    /// back TWO sides, each a body or the typed empty, and the one-body
    /// out-type cannot carry them — so this is the split's own door
    /// with its own out-type ([`SplitOut`]), and the D7 pinch lane
    /// inside the kernel door (`topo::split` reruns a one-sided pinch
    /// mirrored and swaps the sides back) is the door's, reached here
    /// unchanged: this dispatches and re-wraps, and adds no decision
    /// of its own.
    ///
    /// # Errors
    ///
    /// [`VerbError::Split`] carrying the door's [`SplitError`]
    /// verbatim (`topo::split` enumerates the cases — every stage's
    /// typed refusal passed through whole); [`VerbError::Arity`] if
    /// this verb answers another door.
    pub fn run_split(&self, operand: &Body<T>, tol: Tol) -> Result<SplitOut<T>, VerbError> {
        match self {
            Self::Split { plane } => {
                // Exhaustive destructure, deliberately: a field grown
                // onto `SplitResult` breaks this door at compile time
                // instead of silently vanishing in the move.
                let SplitResult {
                    above,
                    below,
                    naming,
                } = split(operand, plane, tol).map_err(VerbError::Split)?;
                Ok(SplitOut {
                    above,
                    below,
                    record: VerbRecord::Split(naming),
                })
            }
            Self::Fillet { .. }
            | Self::Chamfer { .. }
            | Self::Extrude { .. }
            | Self::Revolve { .. }
            | Self::Boolean { .. } => Err(VerbError::Arity {
                verb: self.kind(),
                given: Arity::Split,
            }),
        }
    }
}
