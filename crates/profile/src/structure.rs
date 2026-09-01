//! **The structure record**: the discrete decisions one elaboration
//! made, named so another scalar can consume them instead of remaking
//! them.
//!
//! # What a structure record is for
//!
//! Elaborating a profile program splits cleanly in two. Almost
//! everything in it is CONTINUOUS — tangent-fit solves, arc-carrier
//! boundaries, bulges, the arrival family — and is already scalar-
//! generic, so an interval lane can evaluate it and enclose the answer.
//! A small, enumerable remainder is DISCRETE: which derived corner the
//! gates admit, which surviving fillet candidate the selection ladder
//! ranks first, which way a fit classifies, which vertex is the
//! canonical start, which loop is the outer one. Those choices are
//! structure, and structure is selected ONCE, at `f64`, identically for
//! every lane — the alternative is two lanes describing two different
//! solids and calling the disagreement a tolerance.
//!
//! The record is what makes "once" enforceable rather than hoped for.
//! An `f64` pass emits it; a lane pass consumes it and re-verifies
//! every entry at its own scalar. Agreement proceeds. An entry the lane
//! cannot classify aborts typed — the cue to bisect the parameter box,
//! never a silent keep-the-nominal-structure. An entry the lane
//! classifies DEFINITELY otherwise refuses typed and names the flipped
//! decision: the binding provably left the nominal's structure, which
//! is a real answer about the geometry and not an error to smooth over.
//!
//! The selection ladder is the sharpest case and the reason the record
//! carries an index rather than a rule. `fillet_select`'s own docs
//! state that in a hairline-asymmetric lens two lanes may legally pick
//! DIFFERENT pockets, because both candidates are valid fillets of the
//! authored legs — so re-running the ladder at a second scalar is not a
//! check, it is a second, independent choice. A guided pass therefore
//! never ranks: it takes the recorded index, having first verified that
//! the joint space it indexes into still has the same shape.
//!
//! # What it is NOT
//!
//! A derived value: rebuilt on demand beside the geometry it describes,
//! content-keyed with everything else, and never persisted. Nothing
//! here is a cache of numbers — every field is a decision, and the
//! coordinates the decision was made about live in the loop.

use geom_core::{Indeterminate, Real, Sign};

use crate::validate::LoopRole;

/// What the corner gates made of one derived corner.
///
/// The two gates run in a fixed order and short-circuit, so a refusal
/// names the gate that stopped the corner rather than a pair of signs:
/// the arrival gate is not evaluated at all once the incoming gate has
/// rejected, and a record that claimed a value for it would be
/// inventing one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CornerGate {
    /// Both gates admitted the corner; its candidates entered the joint
    /// space.
    Admitted,
    /// The corner is not strictly ahead of the incoming side's anchor.
    RefusedAdvance,
    /// The corner is not strictly behind the arrival side's anchor.
    RefusedReach,
}

impl core::fmt::Display for CornerGate {
    /// The outcome as prose — the one spelling a user-facing message
    /// uses, so a rendered gate never leans on `Debug`.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Admitted => "admitted",
            Self::RefusedAdvance => "refused (corner not ahead of the incoming anchor)",
            Self::RefusedReach => "refused (corner not behind the arrival anchor)",
        })
    }
}

/// One fillet resolution's discrete decisions.
///
/// A line×line fillet derives its single corner from the carrier pair
/// directly and never enumerates alternatives, so `corners` is empty
/// and `survivors` is one; the fit signs are the whole record. An
/// arc-carrier fillet enumerates 0, 1 or 2 corners, gates each, and
/// ranks the surviving (corner, candidate) pairs — the shape the two
/// remaining fields describe.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilletDecision {
    /// Per derived corner, in the carrier pair's fixed enumeration
    /// order, what the gates made of it.
    pub corners: Vec<CornerGate>,
    /// How many survivors the flattened (corner, candidate) list held.
    ///
    /// Recorded because it is what makes `candidate` mean anything: an
    /// index into a differently-shaped list addresses a different
    /// construction, so a guided pass verifies the shape before it
    /// honours the index.
    pub survivors: usize,
    /// The winner's index into that flattened list.
    pub candidate: usize,
    /// The incoming side's fit classification.
    pub fit_in: Sign,
    /// The arrival side's fit classification.
    pub fit_out: Sign,
}

/// The structure one replay selected, for one loop: its fillet
/// resolutions in the order the program reached them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReplayStructure {
    /// The fillet resolutions, in resolution order.
    pub fillets: Vec<FilletDecision>,
}

/// A segment's structural shape — the part of a classification that is
/// a decision rather than a measurement.
///
/// The centre and radius of an arc are continuous data the lane
/// computes for itself; which of the two kinds a segment IS, and which
/// way an arc turns, are not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentShape {
    /// A straight segment.
    Line,
    /// A circular arc with this turn sense.
    Arc {
        /// Counterclockwise is `Positive`.
        turn: Sign,
    },
}

impl core::fmt::Display for SegmentShape {
    /// The shape as prose — the one spelling a user-facing message
    /// uses, so a rendered shape never leans on `Debug`. The arc
    /// carries its turn, since which way it turns is half of what
    /// distinguishes two arcs a record disagrees about; it renders in
    /// `Sign`'s vocabulary ("an arc turning positive"), which names the
    /// sign and not the handedness — positive is counterclockwise, as
    /// the `turn` field states.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Line => f.write_str("a line"),
            Self::Arc { turn } => write!(f, "an arc turning {turn}"),
        }
    }
}

/// The structure one validation selected, for one loop.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoopCanonical {
    /// Outer or hole — the containment forest's verdict for this loop.
    pub role: LoopRole,
    /// The loops of the profile that contain this one, by input index
    /// and ascending — the containment forest's row for this loop, and
    /// so its depth. Recorded pairwise rather than as a count because
    /// the count alone cannot say WHICH answer moved when a lane
    /// disagrees.
    pub inside: Vec<usize>,
    /// The lexicographic-minimum vertex of the INPUT chain: the
    /// containment representative point.
    pub representative: usize,
    /// Whether canonicalization reversed the input chain to reach the
    /// role's required winding.
    pub reversed: bool,
    /// The rotation: which vertex of the oriented chain became the
    /// canonical start.
    pub start: usize,
    /// The canonical chain's per-segment shapes.
    pub segments: Vec<SegmentShape>,
    /// The canonical chain's declared tangent joints, sorted and
    /// deduplicated.
    pub tangent_joints: Vec<usize>,
}

/// The structure one validation selected, for a whole profile.
///
/// There is deliberately no `outer_loop` field. The containment
/// forest's verdict is already in each loop's [`LoopCanonical::role`],
/// so a profile-level copy of it would be a second spelling of one
/// fact — the kind that stays right until the day it does not, and
/// that a reader then has to decide between. Callers that want the
/// index read it off the roles.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalStructure {
    /// Per input loop, in input order.
    pub loops: Vec<LoopCanonical>,
}

/// A whole profile's structure record: what its replays selected, per
/// loop in program order, and what its validation canonicalized.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileStructure {
    /// Per program loop, in program order.
    pub replay: Vec<ReplayStructure>,
    /// The validation's own decisions.
    pub canonical: CanonicalStructure,
}

/// Which recorded decision a guided pass could not honour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    /// The gate outcome of one derived corner of one fillet.
    CornerGate {
        /// Which fillet resolution, in resolution order.
        fillet: usize,
        /// Which derived corner, in enumeration order.
        corner: usize,
    },
    /// The SHAPE of one fillet's flattened joint space — how many
    /// survivors the recorded index addresses.
    ///
    /// Also the site named when the joint space cannot be BUILT at all
    /// here: a carrier-meet classification this scalar cannot make
    /// leaves the survivor list undefined, so the recorded index has
    /// nothing to address.
    JointSpace {
        /// Which fillet resolution.
        fillet: usize,
    },
    /// The ratified tangent-circle construction at a consumed fillet
    /// resolution.
    ///
    /// Deliberately NOT split into `FitIn` / `FitOut`: the fit signs
    /// are produced INSIDE that construction, so a scalar that cannot
    /// classify one of them has not yet reached a sign to attribute the
    /// failure to. Naming a leg here would be inventing a fact; the
    /// escalation payload names the predicate, which is the part that
    /// is actually known.
    FilletConstruction {
        /// Which fillet resolution.
        fillet: usize,
    },
    /// One fillet's incoming-side fit classification.
    FitIn {
        /// Which fillet resolution.
        fillet: usize,
    },
    /// One fillet's arrival-side fit classification.
    FitOut {
        /// Which fillet resolution.
        fillet: usize,
    },
    /// Which loop contains which — the containment forest.
    Containment {
        /// The contained loop's input index.
        loop_: usize,
        /// The loop it was tested against.
        against: usize,
    },
    /// A loop's outer/hole role.
    Role {
        /// The loop's input index.
        loop_: usize,
    },
    /// One canonical segment's shape.
    SegmentShape {
        /// The loop's input index.
        loop_: usize,
        /// The canonical segment index.
        segment: usize,
    },
    /// A loop's declared tangent-joint set after canonicalization.
    TangentJoints {
        /// The loop's input index.
        loop_: usize,
    },
    /// **The guide never reached the chain.** An entry verb mints the
    /// chain's core, and exactly the entry rows install the guide into
    /// it; a row that mints a core without installing would leave the
    /// walk running under a fresh RECORDING guide, which is free lane
    /// selection wearing a guided pass's name — the one failure this
    /// machinery must not have silently. Not a lane disagreement: an
    /// internal invariant break, refused typed.
    GuideNotInstalled,
    /// The record's own shape does not describe this program: a
    /// different loop count, fillet count, or vertex count. Not a lane
    /// disagreement at all — the record and the program are not about
    /// the same thing.
    RecordShape,
}

/// A recorded decision's value, in the one vocabulary a refusal can
/// report both sides of.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecisionValue {
    /// A corner-gate outcome.
    Gate(CornerGate),
    /// A classification sign.
    Sign(Sign),
    /// A position — an index into something.
    Index(usize),
    /// A cardinality. Distinct from [`DecisionValue::Index`] on
    /// purpose: a refusal that reports "the record says 5, this pass
    /// says 4" is unreadable when one of those is a position and the
    /// other is a length, and the two are mixed at exactly the sites
    /// where a record describes a different program.
    Count(usize),
    /// A segment shape.
    Shape(SegmentShape),
    /// A containment answer.
    Inside(bool),
    /// A set of indices, in full.
    ///
    /// Sets are carried whole rather than as a count: two sets of equal
    /// size with different members are a real disagreement, and a
    /// refusal that reported only the sizes would call them equal.
    Set(Vec<usize>),
    /// A loop role.
    Role(LoopRole),
}

/// Why a guided pass could not reproduce the recorded structure.
#[derive(Clone, Debug, PartialEq)]
pub enum StructureRefusalKind {
    /// The lane cannot classify the decision's own predicate at all.
    /// The parameter box is too wide to answer the question — the
    /// driver's cue to bisect it, and never grounds to assume the
    /// nominal answer.
    Indeterminate(Indeterminate),
    /// The lane classifies the decision DEFINITELY otherwise: this
    /// binding provably leaves the nominal elaboration's structure.
    Flipped {
        /// What the `f64` pass decided.
        recorded: DecisionValue,
        /// What this lane decides.
        found: DecisionValue,
    },
}

/// A guided pass's refusal to proceed on a consumed decision.
#[derive(Clone, Debug, PartialEq)]
pub struct StructureRefusal {
    /// Which decision.
    pub decision: Decision,
    /// Why it could not be honoured.
    pub kind: StructureRefusalKind,
}

impl StructureRefusal {
    /// The indeterminate arm.
    pub(crate) fn indeterminate(decision: Decision, source: Indeterminate) -> Self {
        Self {
            decision,
            kind: StructureRefusalKind::Indeterminate(source),
        }
    }

    /// The definite-disagreement arm.
    pub(crate) fn flipped(
        decision: Decision,
        recorded: DecisionValue,
        found: DecisionValue,
    ) -> Self {
        Self {
            decision,
            kind: StructureRefusalKind::Flipped { recorded, found },
        }
    }

    /// The entry row that minted this chain's core did not install the
    /// guide into it.
    pub(crate) fn guide_not_installed() -> Self {
        Self {
            decision: Decision::GuideNotInstalled,
            kind: StructureRefusalKind::Flipped {
                recorded: DecisionValue::Inside(true),
                found: DecisionValue::Inside(false),
            },
        }
    }

    /// The record-does-not-describe-this-program arm: two CARDINALITIES
    /// that should have matched.
    pub(crate) fn shape(recorded: usize, found: usize) -> Self {
        Self {
            decision: Decision::RecordShape,
            kind: StructureRefusalKind::Flipped {
                recorded: DecisionValue::Count(recorded),
                found: DecisionValue::Count(found),
            },
        }
    }

    /// The same arm for a recorded POSITION that the thing it indexes
    /// is too short to contain — reported as the position against the
    /// cardinality, because calling them both "5 vs 4" is what made
    /// these two sites unreadable.
    pub(crate) fn out_of_range(index: usize, len: usize) -> Self {
        Self {
            decision: Decision::RecordShape,
            kind: StructureRefusalKind::Flipped {
                recorded: DecisionValue::Index(index),
                found: DecisionValue::Count(len),
            },
        }
    }
}

impl core::fmt::Display for Decision {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CornerGate { fillet, corner } => {
                write!(f, "fillet {fillet}'s corner-{corner} gate")
            }
            Self::JointSpace { fillet } => write!(f, "fillet {fillet}'s joint space"),
            Self::FilletConstruction { fillet } => {
                write!(f, "fillet {fillet}'s tangent-circle construction")
            }
            Self::FitIn { fillet } => write!(f, "fillet {fillet}'s incoming fit"),
            Self::FitOut { fillet } => write!(f, "fillet {fillet}'s arrival fit"),
            Self::Containment { loop_, against } => {
                write!(f, "whether loop {loop_} lies inside loop {against}")
            }
            Self::Role { loop_ } => write!(f, "loop {loop_}'s role"),
            Self::SegmentShape { loop_, segment } => {
                write!(f, "loop {loop_}'s canonical segment {segment}")
            }
            Self::TangentJoints { loop_ } => write!(f, "loop {loop_}'s declared tangent joints"),
            Self::GuideNotInstalled => {
                write!(f, "the guide's installation into the chain's core")
            }
            Self::RecordShape => write!(f, "the structure record's shape"),
        }
    }
}

// A payload that has a vocabulary reaches prose through that
// vocabulary's own `Display`, never through `Debug`: a refusal sentence
// is read by a person, and a fieldless variant's `Debug` spelling is the
// type's identifier, not a word. `Set` is the exception and keeps a
// `Debug` list — its members are loop indices, identifiers-as-location,
// which have no prose form to reach for — but a noun introduces it, so
// the list reads as the value it is rather than as a dump that leaked
// into a sentence.
impl core::fmt::Display for DecisionValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Gate(g) => write!(f, "{g}"),
            Self::Sign(s) => write!(f, "{s}"),
            Self::Index(i) => write!(f, "index {i}"),
            Self::Count(n) => write!(f, "{n} of them"),
            Self::Shape(s) => write!(f, "{s}"),
            Self::Inside(b) => write!(f, "inside = {b}"),
            Self::Set(v) => write!(f, "indices {v:?}"),
            Self::Role(r) => write!(f, "{r}"),
        }
    }
}

impl core::fmt::Display for StructureRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            StructureRefusalKind::Indeterminate(source) => write!(
                f,
                "{} cannot be re-verified at this scalar: {source} — the elaboration's \
                 structure stands unconfirmed, so nothing is assumed about it; \
                 narrow the parameter box and try again",
                self.decision
            ),
            StructureRefusalKind::Flipped { recorded, found } => write!(
                f,
                "{} is decided differently at this scalar: the elaboration selected \
                 {recorded}, this binding gives {found} — the binding leaves the \
                 elaborated structure, which no lane may silently re-select",
                self.decision
            ),
        }
    }
}

impl std::error::Error for StructureRefusal {}

/// How an elaboration treats the discrete decisions inside it.
///
/// Recording is the default and costs one push per fillet: no
/// arithmetic changes, so a recording pass emits the same bits as one
/// that recorded nothing.
#[derive(Clone, Debug)]
pub(crate) enum Guide<T: Real> {
    /// Selecting freely and recording what was selected.
    Recording(ReplayStructure),
    /// Consuming a prior pass's selections and re-verifying each.
    Guided {
        /// The decisions to consume, in resolution order.
        record: ReplayStructure,
        /// How many have been consumed.
        next: usize,
        /// Ties the guide to the loop's scalar without storing one.
        _scalar: core::marker::PhantomData<fn() -> T>,
    },
}

impl<T: Real> Guide<T> {
    /// A fresh recording guide.
    pub(crate) fn recording() -> Self {
        Self::Recording(ReplayStructure::default())
    }

    /// Whether this guide is still consuming a record.
    ///
    /// Read by the driver AFTER the entry step, where it answers a
    /// different question than it looks like: the entry rows install by
    /// TAKING the guide (leaving a fresh recording one behind), so a
    /// guide that is still `Guided` in the driver's hand is one no row
    /// took, and the chain is therefore elaborating unguided.
    pub(crate) fn is_guided(&self) -> bool {
        matches!(self, Self::Guided { .. })
    }

    /// A guide that consumes `record`.
    pub(crate) fn guided(record: ReplayStructure) -> Self {
        Self::Guided {
            record,
            next: 0,
            _scalar: core::marker::PhantomData,
        }
    }

    /// The decision this resolution must honour, and its index — or
    /// `None` when this pass is selecting freely.
    ///
    /// A guided pass that reaches MORE resolutions than the record
    /// describes refuses here rather than falling through to free
    /// selection: running off the end of the record is exactly the
    /// state in which a lane would start choosing structure for itself.
    pub(crate) fn consume(&mut self) -> Result<Option<(usize, FilletDecision)>, StructureRefusal> {
        match self {
            Self::Recording(_) => Ok(None),
            Self::Guided { record, next, .. } => {
                let i = *next;
                *next += 1;
                match record.fillets.get(i) {
                    Some(d) => Ok(Some((i, d.clone()))),
                    None => Err(StructureRefusal::shape(record.fillets.len(), i + 1)),
                }
            }
        }
    }

    /// The fit signs of a line×line fillet resolution: the whole of its
    /// discrete content, since a straight carrier pair derives ONE
    /// corner and its construction admits one candidate — there is no
    /// enumeration to gate and no ladder to rank.
    ///
    /// Returns the signs the emission must use. Under guidance those
    /// are the RECORDED ones: a fit sign decides whether a straight
    /// piece and its declared joint exist at all, so the lane's own
    /// answer is compared and reported, never adopted.
    pub(crate) fn line_fits(
        &mut self,
        fit_in: Sign,
        fit_out: Sign,
    ) -> Result<(Sign, Sign), StructureRefusal> {
        match self.consume()? {
            None => {
                self.record(FilletDecision {
                    corners: Vec::new(),
                    survivors: 1,
                    candidate: 0,
                    fit_in,
                    fit_out,
                });
                Ok((fit_in, fit_out))
            }
            Some((fillet, decision)) => {
                for (site, recorded, found) in [
                    (Decision::FitIn { fillet }, decision.fit_in, fit_in),
                    (Decision::FitOut { fillet }, decision.fit_out, fit_out),
                ] {
                    if recorded != found {
                        return Err(StructureRefusal::flipped(
                            site,
                            DecisionValue::Sign(recorded),
                            DecisionValue::Sign(found),
                        ));
                    }
                }
                Ok((decision.fit_in, decision.fit_out))
            }
        }
    }

    /// Records one resolution's decisions (a no-op under guidance,
    /// where the decisions came from the record).
    pub(crate) fn record(&mut self, decision: FilletDecision) {
        if let Self::Recording(s) = self {
            s.fillets.push(decision);
        }
    }

    /// The record of what this pass actually did — for a guided pass,
    /// the prefix of its input the elaboration genuinely reached, so a
    /// program with fewer resolutions than the record describes is
    /// visible to the caller as the shorter record it produced.
    pub(crate) fn into_record(self) -> ReplayStructure {
        match self {
            Self::Recording(s) => s,
            Self::Guided {
                mut record, next, ..
            } => {
                record.fillets.truncate(next);
                record
            }
        }
    }
}
