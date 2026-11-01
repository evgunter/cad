//! **The contact record a body carries** (D1, CONTACT-DESIGN C3): an
//! unordered pair of the body's own cells whose interiors meet, plus the
//! decisions it cites, kept in one list a citation addresses by
//! position ([`Backing::Carried`]).
//!
//! The record is the pair and nothing else. It carries no operand label
//! (the boolean's reduction keeps its operand-tagged working set,
//! [`super::ReductionContacts`], and drops the tags at its carry), no
//! stored granularity and no witness. Its kind is the two cells'
//! dimensions ([`ContactKind`]); whether it touches at a point, along a
//! curve or over a patch is read from the two cells' geometry by the
//! census.

use geom_core::Real;

use super::{
    CarriedRecord, Cell, EeContact, EfContact, FfContact, OperandRecords, VeContact, VfContact,
    VvContact,
};
use crate::body::Body;
use crate::coincidence::{Backing, Cited, Cites};
use crate::entity::{EdgeKey, EntityId, FaceKey, LoopBoundary, VertexKey};
use crate::live::{linked, proven};

/// **A contact record's two cells**, in canonical order: the lesser
/// cell first by [`Cell`]'s order (vertex < edge < face, then key), so
/// one pair has one spelling. The two are distinct and neither bounds
/// the other ([`Contact::new`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Contact {
    cells: [Cell; 2],
}

/// Why two cells are not a contact record: what they have in common is
/// structure, not a touch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContactPairError {
    /// The two cells are one.
    OneCell(Cell),
    /// One cell bounds the other, in canonical order: a vertex ends the
    /// edge or lies on the face's boundary, or the edge bounds the face.
    Incident([Cell; 2]),
}

/// Why a carry of a record list refused one of its records
/// ([`ContactRecords::carried_through`], [`ContactRecords::rekeyed`]): a record
/// the carry cannot place is refused, never dropped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarryRefusal {
    /// `cell`, which input record `record` names, has no image the
    /// result holds and the op's lineage states no removal of it: the
    /// lineage does not cover it, or a graft's map has no image of its
    /// kind.
    Unmapped {
        /// The record's index in the input's list.
        record: u32,
        /// The input cell.
        cell: Cell,
    },
    /// Every pair of input record `record`'s images is one cell or
    /// incident in the result, and not two vertices made one.
    Pair {
        /// The record's index in the input's list.
        record: u32,
        /// What the images are instead.
        error: ContactPairError,
    },
    /// The op parted input record `record`'s two cells into different
    /// result bodies, which no one body's records can state.
    Parted {
        /// The record's index in the input's list.
        record: u32,
    },
    /// Whether input record `record`'s touch is on `cells`, a pair
    /// drawn from a divided cell's images, could not be read: the
    /// census's confirm pass escalated, or has no lane for the pair.
    Unread {
        /// The record's index in the input's list.
        record: u32,
        /// The pair of images.
        cells: [Cell; 2],
    },
    /// Input record `record`'s two cells do not meet where the op put
    /// them, and the op removed and moved neither: a touch the op's
    /// lineage says is still there, and the result says is not. The
    /// carry raises it for a record on a divided cell that lands on no
    /// pair of its images, every pair read definitely; the boolean's
    /// output stage for a carried vertex pair whose two points it reads
    /// apart.
    Stale {
        /// The record's index in the input's list.
        record: u32,
    },
    /// Two cells the op minted, one on each of input record `record`'s
    /// two cells, meet at a point, and the op cannot back the meeting
    /// (D1): only a cut that made both, or an op that decides the
    /// meeting at its door, can.
    MintedMeeting {
        /// The record's index in the input's list.
        record: u32,
        /// The two minted cells.
        cells: [Cell; 2],
    },
}

impl CarryRefusal {
    /// The refused record's index.
    pub(crate) fn record(self) -> u32 {
        match self {
            Self::Unmapped { record, .. }
            | Self::Pair { record, .. }
            | Self::Parted { record }
            | Self::Unread { record, .. }
            | Self::Stale { record }
            | Self::MintedMeeting { record, .. } => record,
        }
    }

    /// The refusal with its record index `index`.
    pub(crate) fn renumbered(mut self, index: u32) -> Self {
        match &mut self {
            Self::Unmapped { record, .. }
            | Self::Pair { record, .. }
            | Self::Parted { record }
            | Self::Unread { record, .. }
            | Self::Stale { record }
            | Self::MintedMeeting { record, .. } => *record = index,
        }
        self
    }
}

impl core::fmt::Display for CarryRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unmapped { record, cell } => write!(
                f,
                "record {record}'s cell {cell:?} has no image the result holds, and the op \
                 states no removal of it"
            ),
            Self::Pair { record, error } => write!(
                f,
                "every pair of record {record}'s images is structure, not a touch ({error:?})"
            ),
            Self::Parted { record } => write!(
                f,
                "the op parted record {record}'s two cells into different bodies"
            ),
            Self::Unread { record, cells } => write!(
                f,
                "whether record {record}'s touch is on {cells:?} could not be read"
            ),
            Self::Stale { record } => write!(
                f,
                "record {record}'s cells do not meet where the op put them, and the op \
                 removed and moved neither"
            ),
            Self::MintedMeeting { record, cells } => write!(
                f,
                "two cells the op minted on record {record}'s cells meet at a point, \
                 {cells:?}, and the op cannot back the meeting"
            ),
        }
    }
}

/// **A record's kind, read from its cells' dimensions**: the six pairs
/// a body's cells can make, each in canonical order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContactKind {
    /// Two vertices at one point.
    VertexVertex(VertexKey, VertexKey),
    /// A vertex on an edge's interior.
    VertexEdge(VertexKey, EdgeKey),
    /// A vertex on a face's interior.
    VertexFace(VertexKey, FaceKey),
    /// Two edges whose interiors cross or overlap.
    EdgeEdge(EdgeKey, EdgeKey),
    /// An edge whose interior meets a face's.
    EdgeFace(EdgeKey, FaceKey),
    /// Two faces whose interiors meet.
    FaceFace(FaceKey, FaceKey),
}

impl Contact {
    /// The record of `x` and `y`, cells of `body`.
    ///
    /// # Errors
    ///
    /// [`ContactPairError::OneCell`] where the two are one, and
    /// [`ContactPairError::Incident`] where one bounds the other in
    /// `body`. A cell `body` does not hold bounds nothing: whether the
    /// record is live is the census's question, not this one.
    pub fn new<T: Real>(body: &Body<T>, x: Cell, y: Cell) -> Result<Self, ContactPairError> {
        let contact = Self::of(x, y)?;
        if incident(body, contact.cells) {
            return Err(ContactPairError::Incident(contact.cells));
        }
        Ok(contact)
    }

    /// The pair `x`, `y` in canonical order, refusing only one cell: for
    /// a map that keeps incidence (a graft into another arena) and for
    /// the producer that has already read it.
    pub(crate) fn of(x: Cell, y: Cell) -> Result<Self, ContactPairError> {
        match x.cmp(&y) {
            core::cmp::Ordering::Less => Ok(Self { cells: [x, y] }),
            core::cmp::Ordering::Greater => Ok(Self { cells: [y, x] }),
            core::cmp::Ordering::Equal => Err(ContactPairError::OneCell(x)),
        }
    }

    /// The two cells, in canonical order.
    #[must_use]
    pub const fn cells(self) -> [Cell; 2] {
        self.cells
    }

    /// The record's kind.
    #[must_use]
    pub fn kind(self) -> ContactKind {
        match self.cells {
            [Cell::Vertex(a), Cell::Vertex(b)] => ContactKind::VertexVertex(a, b),
            [Cell::Vertex(v), Cell::Edge(e)] => ContactKind::VertexEdge(v, e),
            [Cell::Vertex(v), Cell::Face(f)] => ContactKind::VertexFace(v, f),
            [Cell::Edge(a), Cell::Edge(b)] => ContactKind::EdgeEdge(a, b),
            [Cell::Edge(e), Cell::Face(f)] => ContactKind::EdgeFace(e, f),
            [Cell::Face(a), Cell::Face(b)] => ContactKind::FaceFace(a, b),
            [Cell::Edge(_) | Cell::Face(_), Cell::Vertex(_)] | [Cell::Face(_), Cell::Edge(_)] => {
                unreachable!("a contact's cells are in canonical order")
            }
        }
    }
}

/// Whether the lesser cell of `cells` bounds the greater in `body`: a
/// vertex ends the edge or lies on one of the face's loops, or the edge
/// bounds the face. A cell `body` does not hold bounds nothing; a live
/// cell's links are read as links ([`crate::live`]), so a torn one
/// panics rather than read as non-incident.
pub(crate) fn incident<T: Real>(body: &Body<T>, cells: [Cell; 2]) -> bool {
    let half_edges = |e: EdgeKey| {
        body.get_edge(e).map(|edge| {
            [(edge.he_plus, "he_plus"), (edge.he_minus, "he_minus")].map(|(h, field)| {
                (
                    h,
                    linked(
                        &body.half_edges,
                        h,
                        EntityId::HalfEdge,
                        EntityId::Edge(e),
                        field,
                    ),
                )
            })
        })
    };
    match cells {
        [Cell::Vertex(v), Cell::Edge(e)] => {
            half_edges(e).is_some_and(|hs| hs.iter().any(|(_, h)| h.start == v))
        }
        [Cell::Vertex(v), Cell::Face(f)] => body.get_face(f).is_some_and(|face| {
            body.face_loops_linked(f, face)
                .any(|(_, l)| match l.boundary {
                    LoopBoundary::Empty { vertex } => vertex == v,
                    LoopBoundary::Cycle { first } => body
                        .loop_walk(first)
                        .closed("loop", first)
                        .into_iter()
                        .any(|h| proven(&body.half_edges, h, EntityId::HalfEdge).start == v),
                })
        }),
        [Cell::Edge(e), Cell::Face(f)] => {
            body.get_face(f).is_some()
                && half_edges(e)
                    .is_some_and(|hs| hs.iter().any(|&(h, _)| body.face_of_linked(h) == f))
        }
        _ => false,
    }
}

/// Whether `body` holds `cell`.
pub(crate) fn holds<T: Real>(body: &Body<T>, cell: Cell) -> bool {
    match cell {
        Cell::Vertex(v) => body.get_vertex(v).is_some(),
        Cell::Edge(e) => body.get_edge(e).is_some(),
        Cell::Face(f) => body.get_face(f).is_some(),
    }
}

/// `cell`, or where it is a conventional vertex, its closed edge: a
/// record at that point is a record on the edge's interior.
pub(crate) fn conventional_edge<T: Real>(body: &Body<T>, cell: Cell) -> Cell {
    match cell {
        Cell::Vertex(v) if super::edge_join::is_conventional_vertex(body, v) => body
            .get_vertex(v)
            .and_then(|d| d.emanating)
            .and_then(|h| body.get_half_edge(h))
            .map_or(cell, |h| Cell::Edge(h.edge)),
        _ => cell,
    }
}

/// **A body's contact records** (D1): one list, deterministic in
/// discovery order and deduplicated, each record citing the decisions
/// that back it. A record's index in [`Self::rows`] is what a
/// [`Backing::Carried`] names.
///
/// `PartialEq` is load-bearing, not a convenience: D9's bit-identical
/// replay promises that a rerun reproduces the RECORDS bit-identically
/// (C4's replay clause), and a replay row that can only compare naming
/// is checking a shadow of that promise. Comparing records compares
/// arena keys, which is exactly right here — replay reruns the same
/// pipeline on the same input, so key identity is part of what
/// determinism means.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ContactRecords {
    rows: Vec<Cited<Contact>>,
}

impl ContactRecords {
    /// Every record, in list order.
    #[must_use]
    pub fn rows(&self) -> &[Cited<Contact>] {
        &self.rows
    }

    /// The number of records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Whether there is no record.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Adds `record`, or where a record of the same pair is already
    /// here, adds `record`'s citations to it.
    pub(crate) fn push(&mut self, record: Cited<Contact>) {
        match self.rows.iter_mut().find(|r| r.record == record.record) {
            Some(r) => r.cites = r.cites.union(&record.cites),
            None => self.rows.push(record),
        }
    }

    /// Every record as the cell pair it names, in list order.
    pub fn cell_pairs(&self) -> impl Iterator<Item = (Cell, Cell)> + '_ {
        self.rows.iter().map(|c| {
            let [x, y] = c.cells();
            (x, y)
        })
    }

    /// Every record's kind, in list order.
    pub fn kinds(&self) -> impl Iterator<Item = ContactKind> + '_ {
        self.rows.iter().map(|c| c.kind())
    }

    /// Every vertex–vertex record's two vertices, in list order.
    pub fn vertex_vertex(&self) -> impl Iterator<Item = (VertexKey, VertexKey)> + '_ {
        self.kinds().filter_map(|k| match k {
            ContactKind::VertexVertex(a, b) => Some((a, b)),
            _ => None,
        })
    }

    /// Every vertex–edge record's vertex and edge, in list order.
    pub fn vertex_edge(&self) -> impl Iterator<Item = (VertexKey, EdgeKey)> + '_ {
        self.kinds().filter_map(|k| match k {
            ContactKind::VertexEdge(v, e) => Some((v, e)),
            _ => None,
        })
    }

    /// Every vertex–face record's vertex and face, in list order.
    pub fn vertex_face(&self) -> impl Iterator<Item = (VertexKey, FaceKey)> + '_ {
        self.kinds().filter_map(|k| match k {
            ContactKind::VertexFace(v, f) => Some((v, f)),
            _ => None,
        })
    }

    /// Every edge–edge record's two edges, in list order.
    pub fn edge_edge(&self) -> impl Iterator<Item = (EdgeKey, EdgeKey)> + '_ {
        self.kinds().filter_map(|k| match k {
            ContactKind::EdgeEdge(a, b) => Some((a, b)),
            _ => None,
        })
    }

    /// Every edge–face record's edge and face, in list order.
    pub fn edge_face(&self) -> impl Iterator<Item = (EdgeKey, FaceKey)> + '_ {
        self.kinds().filter_map(|k| match k {
            ContactKind::EdgeFace(e, f) => Some((e, f)),
            _ => None,
        })
    }

    /// Every face–face record's two faces, in list order.
    pub fn face_face(&self) -> impl Iterator<Item = (FaceKey, FaceKey)> + '_ {
        self.kinds().filter_map(|k| match k {
            ContactKind::FaceFace(a, b) => Some((a, b)),
            _ => None,
        })
    }

    /// Every record's citations, in list order.
    pub fn cites(&self) -> impl Iterator<Item = &Cites> + '_ {
        self.rows.iter().map(|c| &c.cites)
    }

    /// Every row of the producing op's own list some record cites
    /// ([`Backing::Decided`]), ascending and once each.
    #[must_use]
    pub fn decided(&self) -> Vec<u32> {
        let mut used: Vec<u32> = self
            .cites()
            .flat_map(Cites::iter)
            .filter_map(|b| match b {
                Backing::Decided(k) => Some(k),
                Backing::Carried { .. } => None,
            })
            .collect();
        used.sort_unstable();
        used.dedup();
        used
    }

    /// These records with every [`Backing::Decided`] citation renumbered
    /// through `row`, the citation dropped where `row` answers `None`;
    /// carried citations stay.
    ///
    /// # Errors
    ///
    /// The index in [`Self::rows`] of the first record whose citations
    /// all dropped.
    pub fn renumbered(mut self, row: impl Fn(u32) -> Option<u32>) -> Result<Self, usize> {
        for (k, c) in self.rows.iter_mut().enumerate() {
            c.cites = c
                .cites
                .try_map(|b| {
                    Ok::<_, core::convert::Infallible>(match b {
                        Backing::Decided(i) => row(i).map(Backing::Decided),
                        carried @ Backing::Carried { .. } => Some(carried),
                    })
                })
                .unwrap_or_else(|never| match never {})
                .ok_or(k)?;
        }
        Ok(self)
    }

    /// The index in [`Self::rows`] of the record of `x` and `y`, in
    /// either order: what a record carried from this list cites
    /// ([`Backing::Carried`]).
    #[must_use]
    pub fn position(&self, (x, y): (Cell, Cell)) -> Option<u32> {
        let pair = Contact::of(x, y).ok()?;
        self.rows
            .iter()
            .position(|c| c.record == pair)
            .map(super::reduce::index)
    }

    /// These records as an op's that carries them from its input
    /// `input` unchanged: each cell kept, each record citing this list's
    /// record ([`Backing::Carried`]) in place of what it cited here.
    #[must_use]
    pub fn carried_from(&self, input: u32) -> Self {
        let rows = self
            .rows
            .iter()
            .enumerate()
            .map(|(k, c)| {
                let record = super::reduce::index(k);
                Cited::new(c.record, Cites::one(Backing::Carried { input, record }))
            })
            .collect();
        Self { rows }
    }

    /// **The one carry** (D1): these records as an op's that made
    /// `result` from its input `input`, through `lineage`, what the op
    /// made of each input cell. Each record citing this list's record
    /// ([`Backing::Carried`]) lands as [`landings`] places it: on its
    /// cells' images by lineage, unread, where the op kept or
    /// substituted both cells, and otherwise on every pair drawn from
    /// its two cells' images whose interiors meet, read by the census's
    /// own record confirm at `band` through `region` (the chart-region
    /// door the census's doors take, [`crate::RegionLane`]; without it a
    /// face–face pair is unread). Nothing else of the result is
    /// searched.
    ///
    /// A conventional vertex has no identity of its own, so an image
    /// landing on one is its closed edge. Two records landing on one
    /// pair are one record citing both.
    ///
    /// A record that lands on no pair, every pair read definitely, is
    /// **consumed** where the op removed one of its cells or divided one
    /// with its remainder removed; so too a vertex–vertex record whose
    /// two vertices the op made one (D1's strict record-drop rule). It is
    /// never dropped because no witness turned up: otherwise it refuses.
    ///
    /// # Errors
    ///
    /// [`CarryRefusal::Unmapped`] for a cell the lineage does not cover;
    /// [`CarryRefusal::Unread`] for a pair drawn from a divided cell
    /// that the confirm cannot read; [`CarryRefusal::MintedMeeting`] for
    /// two cells the op minted on the record's two cells meeting at a
    /// point it cannot back;
    /// [`CarryRefusal::Stale`] for a record that lands nowhere with no
    /// removal to consume it, and [`CarryRefusal::Pair`] for one whose
    /// every pair of images is one cell or incident.
    pub fn carried_through<T: geom_core::Decide>(
        &self,
        input: u32,
        lineage: &crate::Lineage,
        result: &Body<T>,
        band: geom_core::Band,
        region: Option<crate::RegionLane<T>>,
    ) -> Result<Self, CarryRefusal> {
        let [out] = <[Self; 1]>::try_from(self.carried_onto(
            input,
            lineage,
            &[result],
            band,
            region,
            None,
        )?)
        .unwrap_or_else(|_| unreachable!("one side carries one list"));
        Ok(out)
    }

    /// These records carried onto `sides`, the bodies an op made from
    /// its input `input`, one list per side, as [`Self::carried_through`]
    /// carries them onto one ([`landings`]); each record the op parts
    /// onto different sides listed in `parted` where it is given.
    ///
    /// # Errors
    ///
    /// As [`landings`]; [`CarryRefusal::MintedMeeting`] for a minted
    /// meeting where the lineage is not one cut's.
    pub(crate) fn carried_onto<T: geom_core::Decide>(
        &self,
        input: u32,
        lineage: &crate::Lineage,
        sides: &[&Body<T>],
        band: geom_core::Band,
        region: Option<crate::RegionLane<T>>,
        parted: Option<&mut Vec<Parting>>,
    ) -> Result<Vec<Self>, CarryRefusal> {
        let rows: Vec<[Cell; 2]> = self.rows.iter().map(|r| r.cells()).collect();
        let mut confirms = Confirms::new(sides.len());
        let landed = landings(&rows, lineage, sides, &mut confirms, band, region, parted)?;
        let mut out: Vec<Self> = sides.iter().map(|_| Self::default()).collect();
        for landing in landed {
            let record = super::reduce::index(landing.row);
            if landing.minted && lineage.meeting() == crate::lineage::MintedBacking::Refused {
                return Err(CarryRefusal::MintedMeeting {
                    record,
                    cells: landing.pair.cells(),
                });
            }
            out[landing.side].push(Cited::new(
                landing.pair,
                Cites::one(Backing::Carried { input, record }),
            ));
        }
        Ok(out)
    }

    /// These records with every citation mapped through `f`, each to
    /// the citations it stands for.
    ///
    /// # Errors
    ///
    /// The index in [`Self::rows`] of the first record whose citations
    /// all mapped to none.
    pub fn recited(self, mut f: impl FnMut(Backing) -> Vec<Backing>) -> Result<Self, usize> {
        self.try_recited(|b| Ok(f(b)), |k| k)
    }

    /// [`Self::recited`] through a map that can refuse.
    ///
    /// # Errors
    ///
    /// The first refusal of `f`, or `uncited` of the index in
    /// [`Self::rows`] of the first record whose citations all mapped to
    /// none.
    pub fn try_recited<E>(
        mut self,
        f: impl FnMut(Backing) -> Result<Vec<Backing>, E>,
        uncited: impl FnOnce(usize) -> E,
    ) -> Result<Self, E> {
        let mut f = f;
        for (k, c) in self.rows.iter_mut().enumerate() {
            match c.cites.try_map(&mut f)? {
                Some(cites) => c.cites = cites,
                None => return Err(uncited(k)),
            }
        }
        Ok(self)
    }

    /// These records carried back into an op as one operand's own, each
    /// with its index here.
    #[must_use]
    pub fn carried(&self) -> OperandRecords {
        let mut out = OperandRecords::default();
        for (k, c) in self.rows.iter().enumerate() {
            let record = super::reduce::index(k);
            match c.kind() {
                ContactKind::VertexVertex(a, b) => out.vv.push(CarriedRecord {
                    contact: VvContact { a, b },
                    record,
                }),
                ContactKind::VertexFace(vertex, face) => out.vf.push(CarriedRecord {
                    contact: VfContact { vertex, face },
                    record,
                }),
                ContactKind::VertexEdge(vertex, edge) => out.ve.push(CarriedRecord {
                    contact: VeContact { vertex, edge },
                    record,
                }),
                ContactKind::EdgeEdge(a, b) => out.ee.push(CarriedRecord {
                    contact: EeContact { a, b },
                    record,
                }),
                ContactKind::EdgeFace(edge, face) => out.ef.push(CarriedRecord {
                    contact: EfContact { edge, face },
                    record,
                }),
                ContactKind::FaceFace(a, b) => out.ff.push(CarriedRecord {
                    contact: FfContact { a, b },
                    record,
                }),
            }
        }
        out
    }

    /// The same records with every cell re-keyed through `key` into
    /// `body`'s arena (a graft), each pair built as [`Contact::new`]
    /// builds it there and pushed with [`Self::push`]'s merge, so two
    /// records re-keyed onto one pair are one record citing both. Each
    /// keeps its own citations: a graft moves a record, it does not carry
    /// it into an op.
    ///
    /// # Errors
    ///
    /// [`CarryRefusal::Unmapped`] for the first cell `key` has no image
    /// of the same kind for, and [`CarryRefusal::Pair`] for the first
    /// pair whose images are one cell or incident in `body`.
    pub fn rekeyed<T: Real>(
        &self,
        body: &Body<T>,
        key: impl Fn(Cell) -> Option<Cell>,
    ) -> Result<Self, CarryRefusal> {
        let mut out = Self::default();
        for (k, c) in self.rows.iter().enumerate() {
            let record = super::reduce::index(k);
            let image = |c: Cell| match key(c) {
                Some(i) if core::mem::discriminant(&i) == core::mem::discriminant(&c) => Ok(i),
                _ => Err(CarryRefusal::Unmapped { record, cell: c }),
            };
            let [x, y] = c.cells();
            let pair = Contact::new(body, image(x)?, image(y)?)
                .map_err(|error| CarryRefusal::Pair { record, error })?;
            out.push(Cited::new(pair, c.cites.clone()));
        }
        Ok(out)
    }
}

/// **A record an op parts** ([`CarryRefusal::Parted`]): different
/// sides hold its two cells' images and no side holds both.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Parting {
    /// The record's index among the rows carried.
    pub(crate) record: u32,
    /// Each held image of each of the record's two cells, in the
    /// record's order, with the index of the side that holds it.
    pub(crate) held: [Vec<(usize, Cell)>; 2],
}

/// **One record's landing** ([`landings`]): the pair of its two cells'
/// images on side `side` whose interiors meet.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Landing {
    /// The record's index among the rows carried.
    pub(crate) row: usize,
    /// The side it landed on.
    pub(crate) side: usize,
    /// The image of each of the record's two cells, in the record's
    /// order.
    pub(crate) at: [Cell; 2],
    /// The pair, in canonical order.
    pub(crate) pair: Contact,
    /// Whether the two are cells the op minted, one on each of the
    /// record's cells, meeting at a point (D1): only a cut that made
    /// both, or an op that decides the meeting at its door, backs one.
    pub(crate) minted: bool,
}

/// Each side's census confirm, read at most once per side.
pub(crate) struct Confirms<'b, T: geom_core::Decide>(
    Vec<Option<crate::census::RecordConfirm<'b, T>>>,
);

impl<T: geom_core::Decide> Confirms<'_, T> {
    pub(crate) fn new(sides: usize) -> Self {
        Self((0..sides).map(|_| None).collect())
    }
}

/// **The one carry's placement** (D1): each of `rows`, a record's two
/// input cells, put on the pairs drawn from its cells' images under
/// `lineage` that a side holds. Where the lineage kept or substituted
/// both cells, and every image holds its cell's geometry where the op
/// handed its input's over ([`crate::Lineage::from_source`]), every such
/// pair that is not structure is a landing, unread: each image is on
/// its cell's carrier and holds its old interior, and the record was
/// read where it was minted. Otherwise the pairs are the ones whose
/// interiors meet, read by the census's own record confirm at `band`
/// through `region`. The sides hold disjoint keys. A conventional
/// vertex reads as its closed edge.
///
/// A row that lands nowhere, every pair read definitely, is consumed
/// where the lineage removed one of its cells or part of one, where an
/// image does not hold its cell's geometry (the touch went with what
/// the op moved), where its
/// two vertices became one vertex (D1's strict record-drop rule), or,
/// for an op that glues cells ([`crate::Lineage::glued`]), where every
/// pair of its images is structure.
///
/// # Errors
///
/// [`CarryRefusal::Unread`] for a pair the confirm cannot read; for a
/// row that lands nowhere and is not consumed, [`CarryRefusal::Stale`]
/// where some pair was read, [`CarryRefusal::Pair`] where every pair is
/// one cell or incident, [`CarryRefusal::Parted`] where its two cells'
/// images are held by different sides only (listed in `parted` instead
/// where it is given), and
/// [`CarryRefusal::Unmapped`] for a cell with no image held anywhere.
pub(crate) fn landings<'b, T: geom_core::Decide>(
    rows: &[[Cell; 2]],
    lineage: &crate::Lineage,
    sides: &[&'b Body<T>],
    confirms: &mut Confirms<'b, T>,
    band: geom_core::Band,
    region: Option<crate::RegionLane<T>>,
    mut parted: Option<&mut Vec<Parting>>,
) -> Result<Vec<Landing>, CarryRefusal> {
    let mut out = Vec::new();
    for (k, &cells) in rows.iter().enumerate() {
        let record = super::reduce::index(k);
        let fates = cells.map(|c| lineage.fate(c));
        // A minted cell an image of the cell's pieces also names (a
        // vertex a join took into the piece's edge) is that piece.
        let minted: [Vec<Cell>; 2] = [0, 1].map(|i| {
            let pieces: Vec<Cell> = fates[i]
                .pieces()
                .iter()
                .map(|&p| lineage.joined(p))
                .collect();
            fates[i]
                .minted()
                .iter()
                .map(|&m| lineage.joined(m))
                .filter(|m| !pieces.contains(m))
                .collect()
        });
        let images = cells.map(|c| lineage.images(c));
        // An image that does not hold its input cell's geometry, where the
        // op handed its input's geometry over: the op moved what the
        // record's touch lay on.
        let moved = lineage.source().is_some_and(|source| {
            (0..2).any(|i| {
                images[i].iter().any(|&c| {
                    sides
                        .iter()
                        .find(|b| holds(b, c))
                        .is_some_and(|b| !source.holds_as(cells[i], c, b))
                })
            })
        });
        // Cells the op kept or substituted, unmoved, hold the touch, and
        // the record was read where it was minted: it lands by lineage.
        let unread = !moved
            && fates
                .iter()
                .all(|f| matches!(f, crate::Fate::Kept { .. } | crate::Fate::Substituted(_)));
        let mut landed = false;
        let mut read = false;
        let mut structure: Option<ContactPairError> = None;
        let mut held_by: [bool; 2] = [false, false];
        let mut held_at: [Vec<(usize, Cell)>; 2] = [Vec::new(), Vec::new()];
        for (s, (&body, confirm)) in sides.iter().zip(&mut confirms.0).enumerate() {
            // Each held image as the cell a record lands on, whether the
            // op minted it, and whether it is a point: read before a
            // conventional vertex becomes its closed edge.
            let held = |i: usize| {
                let mut at: Vec<(Cell, bool, bool)> = Vec::new();
                for &c in images[i].iter().filter(|&&c| holds(body, c)) {
                    let is_minted = minted[i].contains(&c);
                    let point = matches!(c, Cell::Vertex(_));
                    let c = conventional_edge(body, c);
                    match at.iter_mut().find(|(x, ..)| *x == c) {
                        Some((_, m, p)) => {
                            *m |= is_minted;
                            *p |= point;
                        }
                        None => at.push((c, is_minted, point)),
                    }
                }
                at
            };
            let held = [held(0), held(1)];
            for i in 0..2 {
                held_by[i] |= !held[i].is_empty();
                held_at[i].extend(held[i].iter().map(|&(c, ..)| (s, c)));
            }
            for &(a, a_minted, a_point) in &held[0] {
                for &(b, b_minted, b_point) in &held[1] {
                    let pair = match Contact::new(body, a, b) {
                        Ok(pair) => pair,
                        Err(error) => {
                            structure.get_or_insert(error);
                            continue;
                        }
                    };
                    if !unread {
                        read = true;
                        let confirm = confirm.get_or_insert_with(|| {
                            crate::census::RecordConfirm::of(body, band, region)
                        });
                        if !confirm.meets(pair).map_err(|_| CarryRefusal::Unread {
                            record,
                            cells: pair.cells(),
                        })? {
                            continue;
                        }
                    }
                    landed = true;
                    out.push(Landing {
                        row: k,
                        side: s,
                        at: [a, b],
                        pair,
                        minted: (a_point || b_point) && a_minted && b_minted,
                    });
                }
            }
        }
        if landed {
            continue;
        }
        let removed = fates.iter().any(crate::Fate::removes);
        if read {
            // A touch on geometry the op moved went with it.
            if removed || moved {
                continue;
            }
            return Err(CarryRefusal::Stale { record });
        }
        if let Some(error) = structure {
            let fused = matches!(error, ContactPairError::OneCell(Cell::Vertex(_)))
                && matches!(cells, [Cell::Vertex(_), Cell::Vertex(_)]);
            // Structure needs no record (D1): two vertices made one, or
            // cells an op glued ([`crate::Lineage::glued`]).
            if fused || lineage.glues() {
                continue;
            }
            return Err(CarryRefusal::Pair { record, error });
        }
        if held_by == [true, true] {
            let Some(parted) = parted.as_deref_mut() else {
                return Err(CarryRefusal::Parted { record });
            };
            parted.push(Parting {
                record,
                held: held_at,
            });
            continue;
        }
        // A cell no side holds is covered only by its own removal.
        for i in 0..2 {
            if !held_by[i] && !fates[i].removes() {
                return Err(CarryRefusal::Unmapped {
                    record,
                    cell: cells[i],
                });
            }
        }
    }
    Ok(out)
}

impl Extend<Cited<Contact>> for ContactRecords {
    fn extend<I: IntoIterator<Item = Cited<Contact>>>(&mut self, iter: I) {
        for record in iter {
            self.push(record);
        }
    }
}

impl FromIterator<Cited<Contact>> for ContactRecords {
    fn from_iter<I: IntoIterator<Item = Cited<Contact>>>(iter: I) -> Self {
        let mut out = Self::default();
        out.extend(iter);
        out
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{Cell, Contact, ContactKind, ContactPairError, ContactRecords};
    use crate::boolean::{CarriedRecord, EfContact, FfContact};
    use crate::coincidence::{Backing, Cited, Cites};
    use crate::test_support_fixtures::prism_z;
    use geom_core::Tol;

    fn prism() -> crate::test_support_fixtures::Prism<f64> {
        prism_z::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)],
            0.0,
            1.0,
            Tol::witness(),
        )
    }

    /// One pair has one spelling whichever order it is given in, and
    /// its kind reads off the canonical order. Red if the constructor
    /// keeps the given order.
    #[test]
    fn a_pair_is_canonical_in_either_order() {
        let p = prism();
        let (v, f) = (Cell::Vertex(p.top[0]), Cell::Face(p.bottom_face));
        let one = Contact::new(&p.body, v, f).unwrap();
        assert_eq!(one, Contact::new(&p.body, f, v).unwrap());
        assert_eq!(one.cells(), [v, f]);
        assert_eq!(one.kind(), ContactKind::VertexFace(p.top[0], p.bottom_face));
    }

    /// A cell with itself, or a vertex of the face it is paired with,
    /// is structure. Red if either is built.
    #[test]
    fn one_cell_and_an_incident_pair_refuse() {
        let p = prism();
        let v = Cell::Vertex(p.bottom[0]);
        assert_eq!(
            Contact::new(&p.body, v, v),
            Err(ContactPairError::OneCell(v))
        );
        let f = Cell::Face(p.bottom_face);
        assert_eq!(
            Contact::new(&p.body, f, v),
            Err(ContactPairError::Incident([v, f]))
        );
    }

    /// A pair pushed twice is one record citing both; a carried list
    /// cites each record by its index. Red if the second push appends,
    /// or a carried record cites another's index.
    #[test]
    fn a_pair_pushed_twice_is_one_record_and_positions_are_indices() {
        let p = prism();
        let (a, b) = (Cell::Vertex(p.top[0]), Cell::Face(p.bottom_face));
        let (c, d) = (Cell::Vertex(p.top[1]), Cell::Face(p.bottom_face));
        let mut records = ContactRecords::default();
        records.push(Cited::new(
            Contact::new(&p.body, a, b).unwrap(),
            Cites::decided(0),
        ));
        records.push(Cited::new(
            Contact::new(&p.body, c, d).unwrap(),
            Cites::decided(1),
        ));
        records.push(Cited::new(
            Contact::new(&p.body, b, a).unwrap(),
            Cites::decided(2),
        ));
        assert_eq!(records.len(), 2);
        assert_eq!(records.position((d, c)), Some(1));
        let cited: Vec<Vec<Backing>> = records
            .carried_from(1)
            .cites()
            .map(|c| c.iter().collect())
            .collect();
        assert_eq!(
            cited,
            [0, 1]
                .map(|record| vec![Backing::Carried { input: 1, record }])
                .to_vec()
        );
        assert_eq!(
            records.rows()[0].cites.iter().collect::<Vec<_>>(),
            [Backing::Decided(0), Backing::Decided(2)]
        );
    }

    /// An operand's edge–face and face–face records re-enter an op as
    /// its own, each with its index. Red if either kind is dropped, or
    /// filed under the other's index.
    #[test]
    fn curve_and_patch_records_carry_back_in_with_their_indices() {
        let p = prism();
        let edge = p
            .body
            .edges()
            .map(|(e, _)| e)
            .find(|&e| {
                p.body
                    .edge_vertices(e)
                    .is_some_and(|(v, w)| p.top.contains(&v) && p.top.contains(&w))
            })
            .unwrap();
        let mut records = ContactRecords::default();
        for (k, (x, y)) in [
            (Cell::Vertex(p.top[0]), Cell::Face(p.bottom_face)),
            (Cell::Face(p.top_face), Cell::Face(p.bottom_face)),
            (Cell::Edge(edge), Cell::Face(p.bottom_face)),
        ]
        .into_iter()
        .enumerate()
        {
            records.push(Cited::new(
                Contact::new(&p.body, x, y).unwrap(),
                Cites::decided(u32::try_from(k).unwrap()),
            ));
        }
        let carried = records.carried();
        let (low, high) = if p.top_face < p.bottom_face {
            (p.top_face, p.bottom_face)
        } else {
            (p.bottom_face, p.top_face)
        };
        assert_eq!(
            carried.ff,
            [CarriedRecord {
                contact: FfContact { a: low, b: high },
                record: 1
            }]
        );
        assert_eq!(
            carried.ef,
            [CarriedRecord {
                contact: EfContact {
                    edge,
                    face: p.bottom_face
                },
                record: 2
            }]
        );
        assert_eq!(
            carried.vf.len(),
            1,
            "the vertex record stays filed as before"
        );
    }
}
