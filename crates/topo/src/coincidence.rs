//! **The coincidence record** (D10): every coincidence an operation
//! decides from values, one row per decision, in decision order.
//!
//! A row says which two cells were decided one, what relation was
//! decided between them, at which decision, and the margin that
//! decision read. It records; it proves nothing. Whether a row holds
//! across the family is decided at the document's one door
//! (`editor-core`'s `coincide::prove`), which the `unproven-coincidence`
//! lint reads.
//!
//! A row's cells are keyed in the deciding operation's INPUTS, the
//! arenas the decision read, so the document layer names each by its
//! input's table and the row survives whatever the operation later
//! does to those cells (a merge, a split into fragments).
//!
//! What is recorded is what a ladder decides from a margin. A pair on
//! one surface key is structure and is not recorded, and an ON verdict
//! that only places topology is not a coincidence (D1).

use geom_core::MarginDiag;

use crate::boolean::{Cell, Operand};

/// One cell of a row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RowCell {
    /// A cell of one of the deciding operation's input bodies, keyed in
    /// that body's arena: a boolean's operand, or [`Operand::A`] for the
    /// input of an operation that reads one body.
    Input {
        /// Which input body.
        input: Operand,
        /// The cell, in that body's keys.
        cell: Cell,
    },
    /// The plane the operation cuts with (a split's tool): a carrier
    /// the operation reads, not a cell of any body.
    Tool,
    /// A cell of the operation's own result, for an operation that
    /// reads no body and decides on the one it makes (an import's
    /// anchor).
    Result {
        /// The cell, in the result's keys.
        cell: Cell,
    },
}

impl RowCell {
    /// A face of input `input`.
    #[must_use]
    pub const fn face(input: Operand, face: crate::FaceKey) -> Self {
        Self::Input {
            input,
            cell: Cell::Face(face),
        }
    }
}

/// What was decided between a row's two cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Relation {
    /// One carrier, the two outward sides aligned: a continuation.
    SameOriented,
    /// One carrier, the two outward sides opposed: a contact.
    SameOpposite,
    /// The first cell lies on the second's carrier.
    OnCarrier,
    /// The two cells make equal angles with a third (an isosceles
    /// turn, whose mitre lands on that third edge).
    EqualAngles,
    /// Two surfaces touching along a locus, tangent there: the outward
    /// sides opposed (a tangent contact) or, `aligned`, one surface
    /// carried on tangentially (a seam).
    Tangent {
        /// The two outward sides agree (a seam).
        aligned: bool,
    },
}

impl Relation {
    /// The class a declaration of a face pair glued under this relation
    /// asserts: one carrier opposed a `Rest` contact, aligned a
    /// continuation, a tangency a `Tangent` contact or a seam. `None`
    /// for a relation that glues no face pair.
    #[must_use]
    pub const fn glued_class(self) -> Option<crate::contact::BooleanCoincidence> {
        use crate::contact::BooleanCoincidence as C;
        match self {
            Self::SameOpposite => Some(C::REST),
            Self::SameOriented => Some(C::Continuation),
            Self::Tangent { aligned: false } => Some(C::TANGENT),
            Self::Tangent { aligned: true } => Some(C::Seam),
            Self::OnCarrier | Self::EqualAngles => None,
        }
    }
}

/// The decision a row was recorded at: a closed set, one per site
/// that decides a coincidence from values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DecisionSite {
    /// The plane ladder: a pair of planes the operation glued, read as
    /// one displacement over its consumed extent.
    PlaneLadder,
    /// The carrier ladder, for a curved pair.
    CarrierLadder,
    /// The tangent witness lane: two faces touching tangentially along
    /// a closed-form locus or a shared rim, verified along it.
    TangentWitness,
    /// The cylinder×sphere pair's coaxial classification: the sphere's
    /// centre on the cylinder's axis.
    CoaxialSphere,
    /// A split's ON verdict at a vertex whose neighbourhood leaves it
    /// with two or more runs on one side, so pieces of one side touch
    /// there.
    SplitOn,
    /// The blend battery's isosceles turn (`fillet3_turn_isosceles`).
    BatteryTurn,
    /// A boolean's vertex identity: a vertex of one operand decided on
    /// a vertex, an edge or a face of the other, or a carried record's
    /// vertex on a vertex the reduction minted, whose touch survives
    /// into the result.
    VertexFusion,
    /// The at-rest census: two faces of placed copies decided one
    /// carrier, opposed.
    CensusAtRest,
    /// An imported file's vertex anchor: two of the body's vertices
    /// decided at one point.
    ImportAnchor,
}

/// **How a row's Zero was discharged.** Every row a lane records today
/// was decided numerically, against the band; a Zero the symbolic tier
/// proves identically is a theorem, an arm the door's replay at `Sym`
/// adds when it reads one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Discharge {
    /// The margin was classified against the band.
    Numeric,
}

/// **One coincidence an operation decided from values.**
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coincidence {
    /// The two cells decided one, in the order the decision read them.
    pub cells: [RowCell; 2],
    /// What was decided between them.
    pub relation: Relation,
    /// Where it was decided.
    pub site: DecisionSite,
    /// The margin that decision read: Zero, or in band where a
    /// declaration bridged it.
    pub margin: MarginDiag,
    /// How that margin's Zero was discharged.
    pub discharge: Discharge,
}

/// **One decision a contact record cites** (D1 (ii)): the coincidence
/// that backs the touch the record states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Backing {
    /// Row `.0` of the coincidences of the operation whose result
    /// carries the record.
    Decided(u32),
    /// Record `record` of input `input`'s own records, in that input's
    /// list order ([`crate::ContactRecords::rows`]), carried in. That
    /// record cites its own backing, so a chain of these ends at a
    /// [`Backing::Decided`].
    Carried {
        /// The input, by position among the operation's inputs (a
        /// boolean's operand A is 0, B is 1).
        input: u32,
        /// The record's index in that input's list.
        record: u32,
    },
}

/// **Every decision a contact record cites**: a set that is never
/// empty, held sorted. A touch one decision made cites that decision;
/// a touch a chain of decisions implies (two vertices each decided
/// one with a third) cites every decision on every shortest chain
/// joining its two cells, so no choice between chains is made.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cites(Vec<Backing>);

impl Cites {
    /// The one decision `backing`.
    #[must_use]
    pub fn one(backing: Backing) -> Self {
        Self(vec![backing])
    }

    /// Row `row` of the operation's own coincidences.
    #[must_use]
    pub fn decided(row: u32) -> Self {
        Self::one(Backing::Decided(row))
    }

    /// The set of `backings`, or `None` when there is none: a record
    /// with nothing to cite cannot be built.
    #[must_use]
    pub fn of(backings: impl IntoIterator<Item = Backing>) -> Option<Self> {
        let mut all: Vec<Backing> = backings.into_iter().collect();
        all.sort_unstable();
        all.dedup();
        (!all.is_empty()).then_some(Self(all))
    }

    /// Both sets.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self::of(self.0.iter().chain(&other.0).copied())
            .unwrap_or_else(|| unreachable!("a union of non-empty sets is non-empty"))
    }

    /// Each decision, in order.
    pub fn iter(&self) -> impl Iterator<Item = Backing> + '_ {
        self.0.iter().copied()
    }

    /// Each decision mapped through `f`, one or more per decision.
    ///
    /// # Errors
    ///
    /// The first `f` refuses.
    pub fn try_map<E, I: IntoIterator<Item = Backing>>(
        &self,
        f: impl FnMut(Backing) -> Result<I, E>,
    ) -> Result<Option<Self>, E> {
        let mapped = self
            .0
            .iter()
            .copied()
            .map(f)
            .collect::<Result<Vec<_>, E>>()?;
        Ok(Self::of(mapped.into_iter().flatten()))
    }
}

/// **A contact record and the decisions that back it.** The record's
/// cells read through it, so a reader of cells needs no unwrapping; a
/// record cannot be built without its [`Cites`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cited<R> {
    /// The record.
    pub record: R,
    /// What backs it.
    pub cites: Cites,
}

impl<R> Cited<R> {
    /// `record`, backed by `cites`.
    #[must_use]
    pub const fn new(record: R, cites: Cites) -> Self {
        Self { record, cites }
    }
}

impl<R> core::ops::Deref for Cited<R> {
    type Target = R;

    fn deref(&self) -> &R {
        &self.record
    }
}

/// A record against its cells alone: whether it records that pair,
/// whatever backs it.
impl<R: PartialEq> PartialEq<R> for Cited<R> {
    fn eq(&self, other: &R) -> bool {
        self.record == *other
    }
}
