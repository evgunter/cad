//! **The coincidence record** (D10): every coincidence an operation
//! decides from values, one row per decision, in decision order.
//!
//! A row says which cells were decided one (two, or a meeting's
//! three), what relation was decided between them, at which decision,
//! and the margin that decision read. It records; it proves nothing. Whether a row holds
//! across the family is decided at the document's one door
//! (`editor-core`'s `coincide::prove`), which the `unproven-coincidence`
//! lint reads.
//!
//! A row's cells are keyed in the deciding operation's INPUTS, the
//! arenas the decision read, so the document layer names each by its
//! input's table and the row survives whatever the operation later
//! does to those cells (a merge, a split into fragments).
//!
//! What is recorded is what a ladder decides from a margin and the
//! result holds. A pair on one surface key is structure and is not
//! recorded, and an ON verdict that only places topology is not a
//! coincidence (D1): a boolean records every vertex identity it decides
//! with a margin, as a row of the relation it decided where its inputs'
//! cells state it and as a [`Relation::Meet`] of those cells where they
//! would state another, and a face-pair decision only where the two
//! faces meet, at a point, an edge or an area.

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
    /// A cell of the body the row was decided on, for a decision about
    /// one body at rest rather than about an operation's inputs: the
    /// at-rest census's rows ([`crate::census_rest_decision`] and its
    /// contact rows) are the only ones that carry it.
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

    /// An edge of input `input`.
    #[must_use]
    pub const fn edge(input: Operand, edge: crate::EdgeKey) -> Self {
        Self::Input {
            input,
            cell: Cell::Edge(edge),
        }
    }
}

/// What was decided between a row's cells: two, or a
/// [`Relation::Meet`]'s three.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Relation {
    /// One carrier, the two outward sides aligned: a continuation.
    SameOriented,
    /// One carrier, the two outward sides opposed: a contact.
    SameOpposite,
    /// The first cell lies on the second's carrier.
    OnCarrier,
    /// The two cells' bands end at one point of a third edge (a turn
    /// whose mitre lands on that edge).
    CommonFoot,
    /// The two cells meet in first-order contact: their tangent lines
    /// at the common point are one line, the second carrying on the
    /// first's heading through it (`aligned`, a smooth joint) or
    /// turning back along it (a cusp).
    Tangent {
        /// Whether the second carries on the first rather than
        /// reversing it.
        aligned: bool,
    },
    /// Two surfaces on distinct carriers tangent along a locus. It says
    /// nothing of the materials: whether the outward sides agree is a
    /// fact of the two faces' senses, read off them where it matters.
    SurfaceTangent,
    /// The two cells' carriers' axes of revolution are one line.
    Coaxial,
    /// The two cells' carriers have one radius.
    SameRadius,
    /// The two cells' carriers' axes of revolution meet in one point.
    AxesMeet,
    /// The two cells' carriers are ruled along one direction, so one
    /// cross-section plane sections both into curves that do not vary
    /// along it (a cylinder and a plane parallel to its axis).
    CoRuled,
    /// The cells' carriers share one point, decided by the margin:
    /// unordered, between two cells or more. It states a point decided
    /// on a cell where no one cell of the row is that point (a vertex
    /// minted on an edge, read as that edge), or where three cells or
    /// more meet there.
    Meet,
}

impl Relation {
    /// The relation in words, for a refusal's prose: a noun phrase,
    /// never the derived `Debug` (`Tangent` carries a field).
    #[must_use]
    pub const fn noun(self) -> &'static str {
        match self {
            Self::SameOriented => "one carrier, sides aligned",
            Self::SameOpposite => "one carrier, sides opposed",
            Self::OnCarrier => "a cell on another's carrier",
            Self::CommonFoot => "two bands ending at one point of an edge",
            Self::Tangent { aligned: true } => "a smooth tangent joint",
            Self::Tangent { aligned: false } => "a tangent cusp",
            Self::SurfaceTangent => "two surfaces tangent along a locus",
            Self::Coaxial => "one axis of revolution",
            Self::SameRadius => "one radius",
            Self::AxesMeet => "axes meeting in a point",
            Self::CoRuled => "carriers ruled along one direction",
            Self::Meet => "a point the cells share",
        }
    }

    /// The class a face pair glued under this relation glues as: one
    /// carrier opposed a `Rest` contact, aligned a continuation, a
    /// tangency `Tangent`. `None` for a relation that glues no face
    /// pair.
    #[must_use]
    pub const fn glued_class(self) -> Option<crate::contact::GlueClass> {
        use crate::contact::GlueClass as C;
        match self {
            Self::SameOpposite => Some(C::Rest),
            Self::SameOriented => Some(C::Continuation),
            Self::SurfaceTangent => Some(C::Tangent),
            Self::Tangent { .. }
            | Self::OnCarrier
            | Self::CommonFoot
            | Self::Coaxial
            | Self::SameRadius
            | Self::AxesMeet
            | Self::CoRuled
            | Self::Meet => None,
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
    /// a closed-form locus or a shared rim, verified along it. The
    /// margin is the touch: the pair's separation along the shared
    /// normal, its worst sample plus the span's sag
    /// (`contact_tangent_touch`), standing jointly for the locus it was
    /// read along (`MarginDiag::jointly`).
    TangentWitness,
    /// The cylinder×sphere pair's coaxial classification: the sphere's
    /// centre on the cylinder's axis.
    CoaxialSphere,
    /// The sphere×torus pair's coaxial classification: the sphere's
    /// centre on the torus's axis.
    SphereOnTorusAxis,
    /// The cylinder×torus pair's coaxial classification: the cylinder's
    /// axis on the torus's.
    CoaxialCylinderTorus,
    /// The torus pair's coaxial classification: the two tori on one axis.
    CoaxialTori,
    /// The cylinder pair's crossing classification: two walls of one
    /// radius whose axes meet, cut by each other in the two
    /// bisector-plane ellipses.
    CrossingCylinders,
    /// A split's ON verdict at a vertex whose neighbourhood leaves it
    /// with two or more runs on one side, so pieces of one side touch
    /// there.
    SplitOn,
    /// The blend battery's turn whose two bands' feet on its third edge
    /// are one point (`fillet3_turn_feet_gap` and
    /// `fillet3_turn_overrun_chord`, jointly).
    BatteryTurn,
    /// The blend battery's chain junction, its two links decided
    /// tangent so one band runs through it (`fillet3_chain_g1`).
    BatteryJoint,
    /// The blend battery's curved support pair, decided to share the
    /// axis or ruling its band is minted on
    /// (`fillet3_support_coaxiality`).
    BatterySupportAxis,
    /// A profile's junction no constructor made, its carriers' margin
    /// decided Zero (`profile`'s joint pass).
    ProfileJunction,
    /// A boolean's vertex identity: a vertex of one operand decided on
    /// a vertex, an edge or a face of the other, or a carried record's
    /// vertex on a vertex the reduction minted, every one the boolean
    /// decides with a margin, spelled in its input cells
    /// ([`Relation::Meet`] where a cell was read as one it lies in).
    VertexFusion,
    /// The at-rest census: two cells of different groups of a body at
    /// rest (two copies, two imported instances) decided one, or two
    /// placed faces decided one carrier, opposed.
    CensusAtRest,
}

/// **How a row's Zero was discharged**: against the band, or, in a
/// replay at `Sym`, proven by the symbolic tier (ERROR-DESIGN E12) —
/// the deciding margin zero wherever the construction takes the
/// decisions it took. It says that the margin is zero, and nothing
/// about how the tier proved it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Discharge {
    /// The margin was classified against the band.
    Numeric,
    /// The symbolic tier proved the margin zero.
    Proven,
}

impl Discharge {
    /// How `margin`'s Zero was discharged, as its decision stamped it
    /// ([`MarginDiag::proven`]).
    #[must_use]
    pub const fn of(margin: MarginDiag) -> Self {
        if margin.proven() {
            Self::Proven
        } else {
            Self::Numeric
        }
    }
}

/// **One coincidence an operation decided from values.**
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coincidence {
    /// The two cells decided one, in the order the decision read them;
    /// a [`Relation::Meet`]'s sorted.
    pub cells: [RowCell; 2],
    /// A [`Relation::Meet`]'s cells after its first two, sorted after
    /// `cells`, each once, those it has first: the cell whose crossing
    /// minted a vertex read as the edge it was minted on, and the face a
    /// ring tie's two pierces pierced, which their decision was taken
    /// against. `None` on every other row.
    pub more: [Option<RowCell>; 3],
    /// What was decided between them.
    pub relation: Relation,
    /// Where it was decided.
    pub site: DecisionSite,
    /// The margin that decision read, decided Zero.
    pub margin: MarginDiag,
    /// How that margin's Zero was discharged.
    pub discharge: Discharge,
}

impl Coincidence {
    /// Every cell of the row: `cells`, then `more`'s.
    #[must_use = "the cells are only read through the iterator"]
    pub fn every_cell(&self) -> impl Iterator<Item = RowCell> {
        self.cells
            .into_iter()
            .chain(self.more.into_iter().flatten())
    }

    /// The row's A-operand face and B-operand face, where it decided
    /// one face of each operand a carrier pair, in that order; `None`
    /// for any other pair of cells, and for a [`Relation::Meet`], which
    /// decides a point.
    #[must_use]
    pub fn face_pair(&self) -> Option<(crate::FaceKey, crate::FaceKey)> {
        if self.relation == Relation::Meet {
            return None;
        }
        match self.cells {
            [
                RowCell::Input {
                    input: Operand::A,
                    cell: Cell::Face(a),
                },
                RowCell::Input {
                    input: Operand::B,
                    cell: Cell::Face(b),
                },
            ] => Some((a, b)),
            _ => None,
        }
    }
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
/// record cannot be built without its [`Cites`], which is never empty.
/// That each citation resolves is the producing operation's to keep,
/// not the type's: the boolean's carry refuses a citation of a decision
/// it did not make.
///
/// Two `Cited` are equal when their records and their citations are; a
/// `Cited` against a bare record compares the record alone.
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
