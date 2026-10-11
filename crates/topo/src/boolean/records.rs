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
    CarriedContacts, CarriedRecord, CarriedVf, CarriedVv, Cell, EeContact, VeContact, VfContact,
    VvContact,
};
use crate::body::Body;
use crate::coincidence::{Backing, Cited, Cites};
use crate::contact::ContactClass;
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
/// ([`ContactRecords::carried_through`], [`ContactRecords::rekeyed`]): a
/// record the map cannot take is refused, never dropped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarryRefusal {
    /// `cell`, which input record `record` names, has no image: the op
    /// removed it and its lineage names no live cell holding it now, or
    /// a graft's map has no image of its kind.
    Unmapped {
        /// The record's index in the input's list.
        record: u32,
        /// The input cell.
        cell: Cell,
    },
    /// Input record `record`'s two images are incident in the result.
    Pair {
        /// The record's index in the input's list.
        record: u32,
        /// What the two images are instead.
        error: ContactPairError,
    },
    /// The op parted input record `record`'s two cells into different
    /// result bodies, which no one body's records can state.
    Parted {
        /// The record's index in the input's list.
        record: u32,
    },
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

    /// These records as an op's that made `result` from its input
    /// `input`: each cell mapped through `image`, the op's own lineage,
    /// and each record citing this list's record ([`Backing::Carried`]).
    ///
    /// `image` answers what the op made of an input cell: itself where it
    /// survives under its key, the cell that holds it now where the op
    /// replaced it (a join's edge, a merge's face), and `None` where the
    /// op names no replacement. A conventional vertex has no identity of
    /// its own, so an image landing on one is its closed edge. Two records
    /// mapped onto one pair are one record citing both, and a record
    /// whose two cells map to one cell is consumed and drops, as the
    /// boolean's does (D1's strict record-drop rule).
    ///
    /// Whether a carried touch still holds is not read here: the
    /// census confirms each record against the result's geometry.
    ///
    /// # Errors
    ///
    /// [`CarryRefusal::Unmapped`] for the first cell whose image is
    /// `None` or not live in `result`, and [`CarryRefusal::Pair`] for the
    /// first record whose images are incident there.
    pub fn carried_through<T: Real>(
        &self,
        input: u32,
        result: &Body<T>,
        image: impl Fn(Cell) -> Option<Cell>,
    ) -> Result<Self, CarryRefusal> {
        let mut out = Self::default();
        for k in 0..self.rows.len() {
            out.extend(self.carried_row(input, k, result, &image)?);
        }
        Ok(out)
    }

    /// Record `k` carried into `result` as [`Self::carried_through`]
    /// carries each: `None` where it is consumed.
    pub(crate) fn carried_row<T: Real>(
        &self,
        input: u32,
        k: usize,
        result: &Body<T>,
        image: &impl Fn(Cell) -> Option<Cell>,
    ) -> Result<Option<Cited<Contact>>, CarryRefusal> {
        let record = super::reduce::index(k);
        let [x, y] = self.rows[k].cells().map(|c| {
            image(c)
                .filter(|&i| holds(result, i))
                .map(|i| conventional_edge(result, i))
                .ok_or(CarryRefusal::Unmapped { record, cell: c })
        });
        match Contact::new(result, x?, y?) {
            Ok(pair) => Ok(Some(Cited::new(
                pair,
                Cites::one(Backing::Carried { input, record }),
            ))),
            Err(ContactPairError::OneCell(_)) => Ok(None),
            Err(error) => Err(CarryRefusal::Pair { record, error }),
        }
    }

    /// These records carried back into an op as one operand's own, each
    /// with its index here, the vertex records asserting `class`. An
    /// edge–face or face–face record has no carried kind and stays here.
    #[must_use]
    pub fn carried(&self, class: ContactClass) -> CarriedContacts {
        let mut out = CarriedContacts::default();
        for (k, c) in self.rows.iter().enumerate() {
            let record = super::reduce::index(k);
            match c.kind() {
                ContactKind::VertexVertex(a, b) => out.vv.push(CarriedVv {
                    pair: VvContact { a, b },
                    class,
                    record,
                }),
                ContactKind::VertexFace(vertex, face) => out.vf.push(CarriedVf {
                    rest: VfContact { vertex, face },
                    class,
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
                ContactKind::EdgeFace(..) | ContactKind::FaceFace(..) => {}
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
}
