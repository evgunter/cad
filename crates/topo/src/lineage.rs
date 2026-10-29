//! **What an op made of each cell it touched** (D1). An op's
//! [`Lineage`] states, for each input cell it touched, one [`Fate`]:
//! kept, substituted by a cell that holds its interior, divided into
//! pieces and the cells minted between them, or removed. The record
//! carry ([`crate::ContactRecords::carried_through`]) reads it and nothing else,
//! so a removal is stated, never read off a missing image.
//!
//! An op writes its rows as it goes into the checked lists
//! [`Substitutions`] (`(dead, by)`, with [`Fusions`] over vertices) and
//! [`FaceDivisions`] (`(new face, divided-from face)`), each of which
//! refuses, where a row is added, a row that could close a cycle; the
//! op's `lineage()` states its fates from them. A Boolean also records
//! three lists of minted cells it reads back, checked the same way:
//! each operand's null-edge copies (`NullEdgeCopies`) and edge splits
//! (`EdgeDivisions`), and the zip's cone copies ([`ConeCopies`]); a
//! split records its edge splits too ([`SplitEdgeDivisions`]). All five
//! are one list of minted cells ([`Minted`]). Every list carries the
//! [`LineageRows`] arm that names it, given where it is made: that enum
//! is the one roster of lists.
//! [`Descent`] is the walk division and copy rows need: a cell and
//! every piece the rows divide off it, or every copy they make of it.

use std::collections::{BTreeMap, BTreeSet};

use crate::boolean::{Cell, EdgeJoin};
use crate::entity::FaceKey;

/// **What an op made of one input cell.**
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fate {
    /// The cell holds itself in the result, as `at` (its own key where
    /// the op keeps keys, its key across a graft) and as each copy of it
    /// the op cut (a vertex a split copied onto both of its sides, every
    /// copy on the vertex's point). A kept cell has a result cell, so a
    /// kept cell with none is not a fate.
    Kept {
        /// The cell as the result holds it.
        at: Cell,
        /// The copies, `at` excluded.
        copies: Vec<Cell>,
    },
    /// A cell that holds its interior now: a merge's kept face, a
    /// join's edge, a seam.
    Substituted(Cell),
    /// Divided into pieces and the cells the op minted between them.
    Divided(Division),
    /// Removed, with whatever lay under a touch on it.
    Removed,
}

/// **A cell's division**: its pieces, the cells the op minted between
/// them, and whether the pieces exhaust it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Division {
    /// The pieces, the one that kept the cell's key among them.
    pub pieces: Vec<Cell>,
    /// The cells the op minted in the cell's interior between its
    /// pieces: the vertex that splits an edge, the chord that divides a
    /// face.
    pub minted: Vec<Cell>,
    /// Whether the pieces and minted cells exhaust the cell.
    pub remainder: Remainder,
}

/// Whether a division exhausts its cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Remainder {
    /// The pieces and minted cells hold every point of the cell: a
    /// split's.
    Exhaustive,
    /// The op removed the rest: a rim's opening, a blend's carved span,
    /// a boolean's discarded fragments.
    Removed,
}

/// How a carry backs two cells an op minted, one on each of a record's
/// two cells, that meet at a point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MintedBacking {
    /// One cut made both, so they meet where the record puts them: the
    /// landing cites the record alone. The split's one plane.
    OneCut,
    /// The op cannot back the meeting, and the carry refuses it.
    Refused,
}

/// Why a [`Lineage`] refuses a statement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineageError {
    /// A different fate of `cell` was stated already.
    Restated {
        /// The cell.
        cell: Cell,
    },
    /// A removal stated for `cell` keeps all of it.
    NotARemoval {
        /// The cell.
        cell: Cell,
    },
}

/// **One op's lineage** (D1): the [`Fate`] of each input cell it
/// touched, read through the joins it ended with. A cell no row names
/// is kept under its key. The carry lands a record on kept and
/// substituted cells unread, relying on each image being on its input
/// cell's carrier and holding its old interior; an op that writes new
/// geometry under kept keys hands its input's geometry over
/// ([`Self::from_source`]), and the carry then reads every image that
/// does not hold its cell's geometry. Where the result does not hold a
/// cell no row names, the lineage does not cover it and the carry
/// refuses ([`crate::CarryRefusal::Unmapped`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lineage {
    rows: BTreeMap<Cell, Fate>,
    fusions: Fusions,
    merged: BTreeMap<Cell, Cell>,
    joins: Vec<EdgeJoin>,
    meeting: MintedBacking,
    glues: bool,
    source: Option<SourceGeometry>,
}

impl Default for Lineage {
    fn default() -> Self {
        Self {
            rows: BTreeMap::new(),
            fusions: Fusions::new(LineageRows::Fusions),
            merged: BTreeMap::new(),
            joins: Vec::new(),
            meeting: MintedBacking::Refused,
            glues: false,
            source: None,
        }
    }
}

impl Lineage {
    /// The lineage of an op that kept every cell under its key.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The lineage with `cell`'s fate stated.
    ///
    /// # Errors
    ///
    /// [`LineageError::Restated`] where a different fate of `cell` was
    /// stated before: one cell has one fate, and an op that reaches a
    /// cell by two routes says which fate it is
    /// ([`Self::with_removal`] merges two removals).
    pub fn with(mut self, cell: Cell, fate: Fate) -> Result<Self, LineageError> {
        match self.rows.entry(cell) {
            std::collections::btree_map::Entry::Occupied(before) => {
                if *before.get() == fate {
                    Ok(self)
                } else {
                    Err(LineageError::Restated { cell })
                }
            }
            std::collections::btree_map::Entry::Vacant(slot) => {
                slot.insert(fate);
                Ok(self)
            }
        }
    }

    /// The lineage with `cell` removed, whole ([`Fate::Removed`]) or in
    /// part ([`Fate::Divided`] with its remainder removed), merged with
    /// a removal of it stated before: an op that removes part of a cell
    /// by two routes removes the union, its pieces and minted cells
    /// those of both, and a cell removed whole by one route and in part
    /// by another keeps the part's pieces.
    ///
    /// # Errors
    ///
    /// [`LineageError::NotARemoval`] where `fate` keeps all of `cell`,
    /// and [`LineageError::Restated`] where `cell`'s fate was stated
    /// before as one that keeps all of it.
    pub fn with_removal(mut self, cell: Cell, fate: Fate) -> Result<Self, LineageError> {
        if !fate.removes() {
            return Err(LineageError::NotARemoval { cell });
        }
        let merged = match self.rows.remove(&cell) {
            None | Some(Fate::Removed) => fate,
            Some(Fate::Divided(mut d)) if d.remainder == Remainder::Removed => {
                if let Fate::Divided(more) = fate {
                    for p in more.pieces {
                        if !d.pieces.contains(&p) {
                            d.pieces.push(p);
                        }
                    }
                    for m in more.minted {
                        if !d.minted.contains(&m) {
                            d.minted.push(m);
                        }
                    }
                }
                Fate::Divided(d)
            }
            Some(_) => return Err(LineageError::Restated { cell }),
        };
        self.rows.insert(cell, merged);
        Ok(self)
    }

    /// The lineage of an op that wrote new geometry under kept keys,
    /// `source` its input's geometry taken before it wrote: a record
    /// lands unread only where each of its cells' images holds that
    /// cell's geometry ([`SourceGeometry::holds_as`]), and is read by the
    /// census's confirm otherwise.
    #[must_use]
    pub fn from_source(mut self, source: SourceGeometry) -> Self {
        self.source = Some(source);
        self
    }

    pub(crate) fn source(&self) -> Option<&SourceGeometry> {
        self.source.as_ref()
    }

    /// The lineage read through `joins`, the joins the op ended with:
    /// each image a join killed is the edge it made.
    #[must_use]
    pub fn through_joins(mut self, joins: &[EdgeJoin]) -> Self {
        self.joins.extend_from_slice(joins);
        self
    }

    /// The lineage read through `merged`, the coplanar merges the op
    /// ended with before its joins, each `(dead, by)` a cell a merge
    /// killed with the face holding its interior
    /// ([`crate::MergedGroup::substitutions`]): each image a merge
    /// killed is its kept face.
    #[must_use]
    pub fn through_merges(mut self, merged: impl IntoIterator<Item = (Cell, Cell)>) -> Self {
        self.merged.extend(merged);
        self
    }

    /// The lineage read through `fusions`, the vertex fusions the op
    /// made before its merges and joins: each image a fusion killed is
    /// the vertex that survives every fusion after it, read on through
    /// the merges and joins as that vertex ([`settle_through`]).
    #[must_use]
    pub(crate) fn through_fusions(mut self, fusions: &Fusions) -> Self {
        self.fusions = fusions.clone();
        self
    }

    /// The lineage of an op whose one cut made every pair of cells it
    /// minted on a record's two cells ([`MintedBacking::OneCut`]).
    #[must_use]
    pub(crate) fn one_cut(mut self) -> Self {
        self.meeting = MintedBacking::OneCut;
        self
    }

    pub(crate) fn meeting(&self) -> MintedBacking {
        self.meeting
    }

    /// The lineage of an op that glues a record's cells into one cell,
    /// or into a cell and its boundary (the boolean's zip fusions and
    /// joins): a record whose every pair of images is one cell or
    /// incident is consumed, a third way beside a removal and two
    /// vertices made one. D1: "Structural sharing (same surface or point
    /// key) needs no record"; a glued pair shares its cell, or bounds
    /// it, and so is structure.
    #[must_use]
    pub(crate) fn glued(mut self) -> Self {
        self.glues = true;
        self
    }

    pub(crate) fn glues(&self) -> bool {
        self.glues
    }

    /// `cell`'s fate: its row, else kept under its key.
    #[must_use]
    pub fn fate(&self, cell: Cell) -> Fate {
        self.rows.get(&cell).cloned().unwrap_or_else(|| Fate::Kept {
            at: cell,
            copies: Vec::new(),
        })
    }

    /// Every cell of the result `cell`'s fate names, read through the
    /// joins: the cell as kept, the substitute, or the pieces and minted
    /// cells; none for a removed cell.
    #[must_use]
    pub fn images(&self, cell: Cell) -> Vec<Cell> {
        let mut out: Vec<Cell> = Vec::new();
        for c in self.fate(cell).images() {
            let c = self.joined(c);
            if !out.contains(&c) {
                out.push(c);
            }
        }
        out
    }

    /// An image read through the fusions, then the merges, then the
    /// joins ([`settle_through`]): the vertex a fused one survives in,
    /// the kept face a merge substituted for that, and the edge a join
    /// made of what is left.
    pub(crate) fn joined(&self, cell: Cell) -> Cell {
        let cell = settle_through(
            &self.fusions,
            cell,
            |c| self.merged.get(&c).copied(),
            |_| false,
        )
        .unwrap_or_else(|end| end);
        crate::boolean::joined_cell(&self.joins, cell)
    }
}

impl Fate {
    /// Every cell of the result this fate names, before the joins: the
    /// cell as kept, the substitute, or the pieces and minted cells.
    pub(crate) fn images(&self) -> Vec<Cell> {
        match self {
            Self::Kept { at, copies } => core::iter::once(*at).chain(copies.clone()).collect(),
            Self::Substituted(by) => vec![*by],
            Self::Divided(d) => d.pieces.iter().chain(&d.minted).copied().collect(),
            Self::Removed => Vec::new(),
        }
    }

    /// Every image but the minted cells, before the joins.
    pub(crate) fn pieces(&self) -> Vec<Cell> {
        match self {
            Self::Divided(d) => d.pieces.clone(),
            _ => self.images(),
        }
    }

    /// The cells the op minted in the cell, before the joins.
    pub(crate) fn minted(&self) -> &[Cell] {
        match self {
            Self::Divided(d) => &d.minted,
            _ => &[],
        }
    }

    /// Whether the op removed the cell, or part of it.
    pub(crate) fn removes(&self) -> bool {
        matches!(
            self,
            Self::Removed
                | Self::Divided(Division {
                    remainder: Remainder::Removed,
                    ..
                })
        )
    }
}

/// One cell's geometry, as keys: what the carry compares an image
/// against its input cell to land a record unread.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Shape {
    Vertex(crate::geometry::PointKey),
    Edge(
        crate::geometry::CurveKey,
        [Option<crate::geometry::PointKey>; 2],
    ),
    Face {
        surface: crate::geometry::SurfaceKey,
        sense: bool,
        points: BTreeSet<crate::geometry::PointKey>,
        curves: BTreeSet<crate::geometry::CurveKey>,
    },
}

impl Shape {
    /// `cell`'s shape in `body`: a vertex's point; an edge's carrier and
    /// its ends' points; a face's carrier, sense, and the points and
    /// carriers of its boundary. None where `body` does not hold it.
    fn of<T: geom_core::Real>(body: &crate::Body<T>, cell: Cell) -> Option<Self> {
        let point = |v| body.get_vertex(v).map(|d| d.point);
        let start = |h| body.get_half_edge(h).and_then(|h| point(h.start));
        match cell {
            Cell::Vertex(v) => point(v).map(Self::Vertex),
            Cell::Edge(e) => {
                let edge = body.get_edge(e)?;
                let mut ends = [start(edge.he_plus), start(edge.he_minus)];
                ends.sort();
                Some(Self::Edge(edge.curve, ends))
            }
            Cell::Face(f) => {
                let face = body.get_face(f)?;
                let (mut points, mut curves) = (BTreeSet::new(), BTreeSet::new());
                // Read off each half-edge's loop, walking no loop: the
                // snapshot is taken before a door's own checks run.
                for (_, h) in body.half_edges() {
                    if body.get_loop(h.parent_loop).is_some_and(|l| l.face == f) {
                        points.extend(point(h.start));
                        curves.extend(body.get_edge(h.edge).map(|e| e.curve));
                    }
                }
                Some(Self::Face {
                    surface: face.surface,
                    sense: face.sense,
                    points,
                    curves,
                })
            }
        }
    }
}

/// **An op's input cells' geometry, as keys**, taken before the op
/// writes: every vertex's point, every edge's carrier and ends, every
/// face's carrier, sense and boundary. An op that keeps a cell's key
/// while writing new geometry hands it to its lineage
/// ([`Lineage::from_source`]), and the carry then lands a record unread
/// only on images whose geometry is the input cell's
/// ([`Self::holds_as`]). The kernel writes new geometry under a new key,
/// so a key held is geometry held.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceGeometry {
    cells: BTreeMap<Cell, Shape>,
}

impl SourceGeometry {
    /// Every cell of `body`, as its geometry's keys.
    #[must_use]
    pub fn of<T: geom_core::Real>(body: &crate::Body<T>) -> Self {
        type Bounds = (
            BTreeSet<crate::geometry::PointKey>,
            BTreeSet<crate::geometry::CurveKey>,
        );
        let mut bounds: BTreeMap<FaceKey, Bounds> = BTreeMap::new();
        for (_, h) in body.half_edges() {
            let Some(f) = body.get_loop(h.parent_loop).map(|l| l.face) else {
                continue;
            };
            let (points, curves) = bounds.entry(f).or_default();
            points.extend(body.get_vertex(h.start).map(|d| d.point));
            curves.extend(body.get_edge(h.edge).map(|e| e.curve));
        }
        let mut cells: BTreeMap<Cell, Shape> = body
            .vertices()
            .map(|(v, _)| Cell::Vertex(v))
            .chain(body.edges().map(|(e, _)| Cell::Edge(e)))
            .filter_map(|c| Shape::of(body, c).map(|s| (c, s)))
            .collect();
        for (f, face) in body.faces() {
            let (points, curves) = bounds.remove(&f).unwrap_or_default();
            cells.insert(
                Cell::Face(f),
                Shape::Face {
                    surface: face.surface,
                    sense: face.sense,
                    points,
                    curves,
                },
            );
        }
        Self { cells }
    }

    /// Whether `image`, a cell of `result`, holds input cell `cell`'s
    /// geometry unchanged: the same kind of cell, on the same carrier,
    /// with the same bounding points and carriers. An image of another
    /// kind (a vertex a join took into an edge), on another carrier, or
    /// trimmed or grown, does not, and neither does a cell this source
    /// never held.
    pub(crate) fn holds_as<T: geom_core::Real>(
        &self,
        cell: Cell,
        image: Cell,
        result: &crate::Body<T>,
    ) -> bool {
        self.cells
            .get(&cell)
            .is_some_and(|s| Shape::of(result, image).as_ref() == Some(s))
    }
}

/// **A cell's descent**: the cell and every cell the division rows
/// (`(new cell, divided-from cell)`, in any order) divide off it or off
/// one of its pieces; over copy rows (`(copy, vertex it copies)`), a
/// vertex and every copy made of it or of one of its copies.
pub(crate) struct Descent<K> {
    cells: BTreeSet<K>,
}

impl<K: Ord + Copy> Descent<K> {
    /// `root`'s descent through `rows`.
    pub(crate) fn of<'r>(root: K, rows: impl IntoIterator<Item = &'r (K, K)>) -> Self
    where
        K: 'r,
    {
        let rows: Vec<_> = rows.into_iter().collect();
        let mut cells = BTreeSet::from([root]);
        let mut todo = vec![root];
        while let Some(c) = todo.pop() {
            for &&(new, from) in &rows {
                if from == c && cells.insert(new) {
                    todo.push(new);
                }
            }
        }
        Self { cells }
    }

    /// The descent's cells, the root among them, in key order.
    pub(crate) fn cells(&self) -> impl Iterator<Item = K> + '_ {
        self.cells.iter().copied()
    }
}

/// The roots of division rows `(new cell, divided-from cell)`: each
/// divided cell no row divides off another, in key order.
pub(crate) fn roots<K: Ord + Copy>(rows: &[(K, K)]) -> Vec<K> {
    let new: BTreeSet<K> = rows.iter().map(|&(n, _)| n).collect();
    rows.iter()
        .map(|&(_, from)| from)
        .filter(|f| !new.contains(f))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Which lineage list refused a row.
///
/// **Not carried by the façade.** Where the kernel raises it
/// ([`crate::BooleanError::LineageRow`],
/// [`crate::chord_join::SplitJoinError::LineageRow`]) it is a kernel
/// defect and ends in the defect ending; where [`FaceDivisions::from_rows`]
/// raises it, it refuses rows its caller wrote, and goes back to the one
/// who holds them. Neither is reached by a move of a model, so nothing
/// branches on the arm. Falsifier: a row a model's geometry can make a
/// list refuse, which would owe its own decision and lever.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineageRows {
    /// A vertex fusion list ([`Fusions`]).
    Fusions,
    /// A substitution list ([`Substitutions`]).
    Substitutions,
    /// A face division list ([`FaceDivisions`]).
    FaceDivisions,
    /// An edge division list: a split's ([`SplitEdgeDivisions`], no
    /// operand), or one operand's in a Boolean (`EdgeDivisions`), the
    /// operand naming whose clone the keys are keys of, since the two
    /// clones' keys can be equal.
    EdgeDivisions(Option<crate::boolean::Operand>),
    /// One operand's null-edge copy list in a Boolean (`NullEdgeCopies`),
    /// the operand naming the clone as above.
    NullEdgeCopies(crate::boolean::Operand),
    /// A zip's cone copy list ([`ConeCopies`]): one row per copy it made
    /// of a pinch vertex, one per cone.
    ConeCopies,
}

impl LineageRows {
    /// What one row of the list records, as a sentence names it.
    #[must_use]
    pub fn word(self) -> &'static str {
        use crate::boolean::Operand::{A, B};
        match self {
            Self::Fusions => "vertex fusion",
            Self::Substitutions => "substitution",
            Self::FaceDivisions => "face division",
            Self::EdgeDivisions(None) => "edge division",
            Self::EdgeDivisions(Some(A)) => "first operand's edge division",
            Self::EdgeDivisions(Some(B)) => "second operand's edge division",
            Self::NullEdgeCopies(A) => "first operand's null-edge copy",
            Self::NullEdgeCopies(B) => "second operand's null-edge copy",
            Self::ConeCopies => "cone copy",
        }
    }
}

/// What is wrong with a lineage row a list refused at append
/// ([`Substitutions::push`], [`Minted::push`]).
///
/// **Not carried by the façade.** Where the kernel raises it
/// ([`crate::BooleanError::LineageRow`],
/// [`crate::chord_join::SplitJoinError::LineageRow`]) it is a kernel
/// defect and ends in the defect ending; where [`FaceDivisions::from_rows`]
/// raises it, it refuses rows its caller wrote, and goes back to the one
/// who holds them. Neither is reached by a move of a model, so nothing
/// branches on the arm. Falsifier: a row a model's geometry can make a
/// list refuse, which would owe its own decision and lever.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowDefect {
    /// The row maps a cell to itself.
    SelfRow,
    /// The row names a cell an earlier row of the list retired.
    NamesRetired,
    /// A row's minted cell (a division's new face or edge, a null edge's
    /// or a zip's copy) is named by an earlier row: a minted cell is new
    /// to the list.
    NotFresh,
}

impl RowDefect {
    /// What the row does wrong, as a sentence says it.
    #[must_use]
    pub fn word(self) -> &'static str {
        match self {
            Self::SelfRow => "maps a cell to itself",
            Self::NamesRetired => "names a cell an earlier row retired",
            Self::NotFresh => "mints a cell an earlier row already names",
        }
    }
}

/// A lineage row refused at append: the list, the row's defect and the
/// cell it is about. The list is left as it was. Every lane maps this
/// one value into its own refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RowRefusal {
    /// The list that refused the row.
    pub rows: LineageRows,
    /// What is wrong with the row.
    pub defect: RowDefect,
    /// The cell the defect is about.
    pub key: Cell,
}

impl RowRefusal {
    /// The refusal of `list` for a row whose defect is about `key`: the
    /// one place a list builds one.
    pub(crate) fn of<K: LineageKey>(list: LineageRows, defect: RowDefect, key: K) -> Self {
        Self {
            rows: list,
            defect,
            key: key.cell(),
        }
    }

    /// The clause every op's refusal opens with, `op` naming the op
    /// whose records refused the row (`"Boolean"`, `"split"`, `"join"`):
    /// each op's message adds what follows from it and the kernel-defect
    /// ending.
    #[must_use]
    pub fn refused_by(self, op: &str) -> String {
        format!("the {op}'s lineage records refused {self}")
    }
}

impl core::fmt::Display for RowRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "a {} row that {} ({:?})",
            self.rows.word(),
            self.defect.word(),
            self.key
        )
    }
}

/// A key a lineage list can hold: the cell a refusal names it as.
pub trait LineageKey: Ord + Copy + core::fmt::Debug {
    /// The key as a cell.
    fn cell(self) -> Cell;
}

impl LineageKey for Cell {
    fn cell(self) -> Cell {
        self
    }
}

impl LineageKey for FaceKey {
    fn cell(self) -> Cell {
        Cell::Face(self)
    }
}

impl LineageKey for crate::entity::EdgeKey {
    fn cell(self) -> Cell {
        Cell::Edge(self)
    }
}

impl LineageKey for crate::entity::VertexKey {
    fn cell(self) -> Cell {
        Cell::Vertex(self)
    }
}

/// **Where `cell`'s interior ends up** through an op's vertex fusions,
/// then its substitutions: the vertex a fused vertex finally fused into
/// ([`Substitutions::survivor`]), then each cell a later substitution
/// (`next`) moved that to, until `live` holds one (`Ok`) or no row moves
/// it on (`Err`, the last cell reached). A substitution can kill a
/// fusion's survivor, so the fusions are read first. The boolean's
/// `Settler::settle` and [`Lineage`]'s images read through this one
/// chase. It ends: each list's rows run one way.
pub(crate) fn settle_through(
    fusions: &Fusions,
    cell: Cell,
    next: impl Fn(Cell) -> Option<Cell>,
    live: impl Fn(Cell) -> bool,
) -> Result<Cell, Cell> {
    let mut at = match cell {
        Cell::Vertex(v) => Cell::Vertex(fusions.survivor(v)),
        other => other,
    };
    loop {
        if live(at) {
            return Ok(at);
        }
        match next(at) {
            Some(by) if by != at => at = by,
            _ => return Err(at),
        }
    }
}

/// **A substitution list**: rows `(dead, by)`, each saying `by` holds
/// `dead`'s interior now, in the order the stages retired them.
/// [`Self::push`] refuses a row that maps a key to itself or names a
/// key an earlier row retired. So a row's `by` is live in the list when
/// it is added and only a later row can retire it: the rows run one
/// way, the list cannot cycle, and every chase through it ends. The
/// list sees only its own rows: a key retired outside it is not checked
/// here.
///
/// One rule, two instances: over cells and faces (the boolean's
/// descendant map and its naming's substitutions) and over vertices
/// ([`Fusions`]). A list carries the [`LineageRows`] a refusal names it
/// by, given where it is made ([`Self::new`]): the key type does not
/// say what a list records.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Substitutions<K: LineageKey = Cell> {
    tag: LineageRows,
    rows: Vec<(K, K)>,
    by_dead: std::collections::BTreeMap<K, K>,
}

/// **A vertex fusion list**: the fusions `(dead, kept)` in the order
/// their `kev`s ran, the vertex instance of [`Substitutions`], made as
/// `Substitutions::new(LineageRows::Fusions)`: each key folds onto one
/// survivor ([`Substitutions::survivor`]).
pub type Fusions = Substitutions<crate::entity::VertexKey>;

impl<K: LineageKey> Substitutions<K> {
    /// An empty list, named `tag` in a refusal.
    #[must_use]
    pub fn new(tag: LineageRows) -> Self {
        Self {
            tag,
            rows: Vec::new(),
            by_dead: std::collections::BTreeMap::new(),
        }
    }

    /// The list a refusal names this one as.
    #[must_use]
    pub fn tag(&self) -> LineageRows {
        self.tag
    }

    /// Appends the row `(dead, by)`.
    ///
    /// # Errors
    ///
    /// [`RowDefect::SelfRow`] where `dead` is `by`, and
    /// [`RowDefect::NamesRetired`] where an earlier row retired either;
    /// the list is left as it was.
    pub(crate) fn push(&mut self, (dead, by): (K, K)) -> Result<(), RowRefusal> {
        let refuse = |defect, key: K| Err(RowRefusal::of(self.tag, defect, key));
        if dead == by {
            return refuse(RowDefect::SelfRow, dead);
        }
        if let Some(key) = [dead, by]
            .into_iter()
            .find(|k| self.by_dead.contains_key(k))
        {
            return refuse(RowDefect::NamesRetired, key);
        }
        self.by_dead.insert(dead, by);
        self.rows.push((dead, by));
        Ok(())
    }

    /// Appends `later`'s rows, which ran after these, in order.
    ///
    /// # Errors
    ///
    /// As [`Self::push`], at the first row that refuses; the list is
    /// left as it was.
    pub(crate) fn extend(&mut self, later: &Self) -> Result<(), RowRefusal> {
        let mut out = self.clone();
        later.rows.iter().try_for_each(|&row| out.push(row))?;
        *self = out;
        Ok(())
    }

    /// The key holding `dead`'s interior, where a row retired it.
    #[must_use]
    pub fn get(&self, dead: K) -> Option<K> {
        self.by_dead.get(&dead).copied()
    }

    /// The rows, in the order they were added.
    #[must_use]
    pub fn rows(&self) -> &[(K, K)] {
        &self.rows
    }

    /// The key `k` ends at through every row: itself where no row
    /// retired it. Total, since the list cannot cycle.
    #[must_use]
    pub fn survivor(&self, k: K) -> K {
        self.chase(k, |_| false).unwrap_or_else(|end| end)
    }

    /// The first key `live` holds along the rows from `at`: `Ok(live)`,
    /// or `Err(dead)` where the rows stop at a key `live` does not hold.
    /// Total, since the list cannot cycle.
    pub fn chase(&self, mut at: K, live: impl Fn(K) -> bool) -> Result<K, K> {
        loop {
            if live(at) {
                return Ok(at);
            }
            match self.get(at) {
                Some(by) => at = by,
                None => return Err(at),
            }
        }
    }
}

impl<'a, K: LineageKey> IntoIterator for &'a Substitutions<K> {
    type Item = &'a (K, K);
    type IntoIter = core::slice::Iter<'a, (K, K)>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.iter()
    }
}

/// **A list of minted cells**: rows `(minted cell, cell it was minted
/// from)` in the order they were minted. [`Self::push`] refuses a row
/// whose two cells are one ([`RowDefect::SelfRow`]), or whose minted
/// cell an earlier row names ([`RowDefect::NotFresh`]); the list is left
/// as it was. A list carries the [`LineageRows`] a refusal names it by,
/// given where it is made ([`Self::new`]); each instance is an alias
/// naming what it records ([`FaceDivisions`], [`SplitEdgeDivisions`],
/// [`ConeCopies`], and a Boolean's per-operand `EdgeDivisions` and
/// `NullEdgeCopies`), with the arm of the same name.
///
/// **The invariant: a minted cell is new to the list.** A mint hands
/// out a fresh key, and an arena never hands out a key it handed out
/// before: a slot it reuses carries a new version, so the new key
/// equals no key an earlier row holds. That is what makes the check
/// hold where the rows are written, and it is all the list needs to be
/// acyclic: in a cycle each row's minted-from cell is the next row's
/// minted cell, which no row before that one may name, so each row
/// would have to come before the one ahead of it all the way round. A
/// reader's chase to the cell a cell descends from ([`Self::root`])
/// therefore ends, and each cell is the minted cell of one row at most,
/// so the cells minted off a cell form a tree ([`Descent`]). An arena
/// that reused a bare key would break the invariant, and [`Self::push`]
/// would refuse the row rather than let the list cycle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Minted<K> {
    tag: LineageRows,
    rows: Vec<(K, K)>,
    /// Each minted cell → the cell it was minted from.
    from: BTreeMap<K, K>,
    /// Every cell a row names, minted or minted-from.
    named: BTreeSet<K>,
}

/// **A face division list**: a division mints its new face (`mef`,
/// `mfkrh`), so each row is `(new face, divided-from face)`.
pub type FaceDivisions = Minted<FaceKey>;

/// **A split's edge division list**: each piece a crossing split minted
/// (`Body::split_edge`), `(new edge, divided-from edge)`.
pub type SplitEdgeDivisions = Minted<crate::entity::EdgeKey>;

/// **A cone copy list**: one row per split a Boolean's zip made of
/// a pinch so each cone of the result's boundary there holds a vertex of
/// its own (`BooleanNaming::cone_splits`). A split mints its copy
/// (`mev_null`), so each row is `(copy, vertex it copies)`.
pub type ConeCopies = Minted<crate::entity::VertexKey>;

/// **One operand's edge division list in a Boolean**: one row per edge
/// its reduction split (`Body::split_edge`), `(new edge, divided-from
/// edge)`.
pub(crate) type EdgeDivisions = Minted<crate::entity::EdgeKey>;

/// **One operand's null-edge copy list in a Boolean**: one row per end
/// of a null edge other than the vertex it was minted at. A null edge
/// mints its copy (`mev`), so each row is `(copy, vertex it copies)`.
pub(crate) type NullEdgeCopies = Minted<crate::entity::VertexKey>;

impl<K> Minted<K> {
    /// An empty list, named `tag` in a refusal.
    #[must_use]
    pub fn new(tag: LineageRows) -> Self {
        Self {
            tag,
            rows: Vec::new(),
            from: BTreeMap::new(),
            named: BTreeSet::new(),
        }
    }

    /// The rows, in the order they were added.
    #[must_use]
    pub fn rows(&self) -> &[(K, K)] {
        &self.rows
    }

    /// Whether no row was added.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

impl<K: LineageKey> Minted<K> {
    /// The list a refusal names this one as.
    #[must_use]
    pub fn tag(&self) -> LineageRows {
        self.tag
    }

    /// Appends `(new, from)`.
    ///
    /// # Errors
    ///
    /// [`RowDefect::SelfRow`] where `new` is `from`, and
    /// [`RowDefect::NotFresh`] where an earlier row names `new`; the
    /// list is left as it was.
    pub(crate) fn push(&mut self, (new, from): (K, K)) -> Result<(), RowRefusal> {
        let defect = if new == from {
            RowDefect::SelfRow
        } else if self.named.contains(&new) {
            RowDefect::NotFresh
        } else {
            self.named.extend([new, from]);
            self.from.insert(new, from);
            self.rows.push((new, from));
            return Ok(());
        };
        Err(RowRefusal::of(self.tag, defect, new))
    }

    /// The cell `cell` was minted from, where a row minted it.
    #[must_use]
    pub fn from(&self, cell: K) -> Option<K> {
        self.from.get(&cell).copied()
    }

    /// The cell `cell` descends from through every row: itself where no
    /// row minted it. Total, since the list cannot cycle.
    #[must_use]
    pub fn root(&self, cell: K) -> K {
        let mut at = cell;
        while let Some(from) = self.from(at) {
            at = from;
        }
        at
    }
}

impl Minted<FaceKey> {
    /// The face division list of `rows`, each added in turn as
    /// [`Self::push`] adds it:
    /// the one way to build a list outside the kernel, where a mint
    /// records its own rows. Only a face division list can be built
    /// there: the others are the kernel's records of its own mints.
    ///
    /// # Errors
    ///
    /// As [`Self::push`], at the first row it refuses.
    pub fn from_rows(
        rows: impl IntoIterator<Item = (FaceKey, FaceKey)>,
    ) -> Result<Self, RowRefusal> {
        let mut list = Self::new(LineageRows::FaceDivisions);
        rows.into_iter().try_for_each(|row| list.push(row))?;
        Ok(list)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use slotmap::SlotMap;

    fn faces(n: usize) -> Vec<FaceKey> {
        let mut arena: SlotMap<FaceKey, ()> = SlotMap::with_key();
        (0..n).map(|_| arena.insert(())).collect()
    }

    fn removed_part(pieces: &[FaceKey]) -> Fate {
        Fate::Divided(Division {
            pieces: pieces.iter().map(|&f| Cell::Face(f)).collect(),
            minted: Vec::new(),
            remainder: Remainder::Removed,
        })
    }

    /// One cell has one fate: a different second statement refuses, the
    /// same one is the same statement. Red where the second replaces the
    /// first.
    #[test]
    fn a_second_fate_for_a_cell_refuses() {
        let f = faces(2);
        let cell = Cell::Face(f[0]);
        let stated = Lineage::new().with(cell, Fate::Removed).unwrap();
        assert_eq!(
            stated.clone().with(cell, removed_part(&[f[1]])),
            Err(LineageError::Restated { cell }),
            "a different fate refuses"
        );
        assert_eq!(
            stated.clone().with(cell, Fate::Removed),
            Ok(stated),
            "the same fate restated is one statement"
        );
    }

    /// Two removals of one cell merge to the same fate in either order:
    /// a part removal keeps its pieces over a whole removal, and two part
    /// removals keep both's pieces. A removal over a fate that keeps the
    /// cell refuses, as does a "removal" that keeps it. Red where the
    /// later statement wins.
    #[test]
    fn removals_of_one_cell_merge_in_either_order() {
        let f = faces(3);
        let cell = Cell::Face(f[0]);
        let merged = |first: Fate, second: Fate| {
            Lineage::new()
                .with_removal(cell, first)
                .and_then(|l| l.with_removal(cell, second))
                .map(|l| l.fate(cell))
        };
        for (first, second) in [
            (Fate::Removed, removed_part(&[f[1]])),
            (removed_part(&[f[1]]), Fate::Removed),
        ] {
            assert_eq!(
                merged(first, second),
                Ok(removed_part(&[f[1]])),
                "whole and part"
            );
        }
        assert_eq!(
            merged(removed_part(&[f[1]]), removed_part(&[f[2]])),
            Ok(removed_part(&[f[1], f[2]])),
            "two parts"
        );
        let kept = Fate::Substituted(Cell::Face(f[1]));
        assert_eq!(
            Lineage::new()
                .with(cell, kept.clone())
                .and_then(|l| l.with_removal(cell, Fate::Removed)),
            Err(LineageError::Restated { cell }),
            "a removal over a kept cell"
        );
        assert_eq!(
            Lineage::new().with_removal(cell, kept),
            Err(LineageError::NotARemoval { cell }),
            "a removal that keeps the cell"
        );
    }

    #[test]
    fn a_descent_follows_rows_listed_after_the_rows_that_read_them() {
        let f = faces(5);
        // f[0] divided into f[1], f[1] into f[2], listed child first;
        // f[3] divided off f[4], a face outside the descent.
        let rows = [(f[2], f[1]), (f[3], f[4]), (f[1], f[0])];
        let descent = Descent::of(f[0], &rows);
        for (i, want) in [true, true, true, false, false].into_iter().enumerate() {
            assert_eq!(
                descent.cells().any(|g| g == f[i]),
                want,
                "face {i} of f0's descent"
            );
        }
        let mut want = vec![f[0], f[4]];
        want.sort();
        assert_eq!(roots(&rows), want, "f1 is divided off f0, so no root");
    }

    /// **A substitution list refuses each row that could close a cycle,
    /// where it is added, and is left as it was.** One row per check:
    /// a row onto itself, a row whose dead cell an earlier row retired,
    /// a row onto a cell an earlier row retired. Red if any of the three
    /// checks is dropped: that row would push, and the chase from `a`
    /// would walk it.
    #[test]
    fn a_substitution_list_refuses_a_self_row_and_a_row_naming_a_retired_cell() {
        let f = faces(3);
        let [a, b, c] = [f[0], f[1], f[2]].map(Cell::Face);
        let mut list: Substitutions = Substitutions::new(LineageRows::Substitutions);
        list.push((a, b)).unwrap();
        let refused = |defect, key| {
            Err(RowRefusal {
                rows: LineageRows::Substitutions,
                defect,
                key,
            })
        };
        for (row, want, case) in [
            ((c, c), refused(RowDefect::SelfRow, c), "a row onto itself"),
            (
                (a, c),
                refused(RowDefect::NamesRetired, a),
                "a retired cell retired again",
            ),
            (
                (c, a),
                refused(RowDefect::NamesRetired, a),
                "a row onto a retired cell",
            ),
            (
                (b, a),
                refused(RowDefect::NamesRetired, a),
                "the row back that closes a cycle",
            ),
        ] {
            assert_eq!(list.push(row), want, "{case}");
            assert_eq!(list.rows(), [(a, b)], "{case}: the list is as it was");
        }
        list.push((b, c)).unwrap();
        assert_eq!(
            list.chase(a, |x| x == c),
            Ok(c),
            "a chain is walked to its end"
        );
        assert_eq!(
            list.chase(a, |_| false),
            Err(c),
            "and ends where no row goes on"
        );
    }

    /// **A face division list refuses a row onto itself, and a row whose
    /// new face an earlier row names, as divided-from or as new.** The
    /// last is the row that would close a cycle: `f1` was divided off
    /// `f0`, and `(f0, f1)` would make `f0` a piece of its own piece.
    /// Red if either check is dropped.
    #[test]
    fn a_face_division_list_refuses_a_self_row_and_a_new_face_it_already_names() {
        let f = faces(4);
        let mut list = FaceDivisions::new(LineageRows::FaceDivisions);
        list.push((f[1], f[0])).unwrap();
        let refused = |defect, face| {
            Err(RowRefusal {
                rows: LineageRows::FaceDivisions,
                defect,
                key: Cell::Face(face),
            })
        };
        for (row, want, case) in [
            (
                (f[2], f[2]),
                refused(RowDefect::SelfRow, f[2]),
                "a row onto itself",
            ),
            (
                (f[1], f[3]),
                refused(RowDefect::NotFresh, f[1]),
                "a new face minted twice",
            ),
            (
                (f[0], f[1]),
                refused(RowDefect::NotFresh, f[0]),
                "the row that closes a cycle",
            ),
        ] {
            assert_eq!(list.push(row), want, "{case}");
            assert_eq!(list.rows(), [(f[1], f[0])], "{case}: the list is as it was");
        }
        list.push((f[2], f[1])).unwrap();
        list.push((f[3], f[1])).unwrap();
        assert_eq!(
            [f[0], f[1], f[2], f[3]].map(|g| list.root(g)),
            [f[0]; 4],
            "every piece descends from f0, through every division"
        );
        assert_eq!(list.from(f[2]), Some(f[1]));
        assert_eq!(list.from(f[0]), None);
    }

    /// **The checked constructor outside the kernel checks.** A caller
    /// building a list from rows it wrote gets the same refusal a push
    /// would give, at the first row that has it. Red if `from_rows`
    /// collects without pushing.
    #[test]
    fn a_list_built_from_rows_refuses_the_row_that_closes_a_cycle() {
        let f = faces(2);
        assert_eq!(
            FaceDivisions::from_rows([(f[1], f[0]), (f[0], f[1])]),
            Err(RowRefusal {
                rows: LineageRows::FaceDivisions,
                defect: RowDefect::NotFresh,
                key: Cell::Face(f[0]),
            })
        );
        let list = FaceDivisions::from_rows([(f[1], f[0])]).unwrap();
        assert_eq!(list.rows(), [(f[1], f[0])]);
    }

    /// **The invariant the face division list stands on: an arena never
    /// hands a key out twice.** A face removed and a face minted into
    /// its slot get keys that differ, so the division that mints the
    /// second is fresh to a list that names the first, and the list
    /// takes it. Red if slot reuse stops versioning keys: the reused
    /// key would equal the removed one, which the list names, and the
    /// push would refuse it ([`RowDefect::NotFresh`], pinned by the row
    /// above).
    #[test]
    fn a_face_minted_into_a_reused_slot_is_fresh_to_the_list() {
        let mut arena: SlotMap<FaceKey, ()> = SlotMap::with_key();
        let parent = arena.insert(());
        let piece = arena.insert(());
        let mut list = FaceDivisions::new(LineageRows::FaceDivisions);
        list.push((piece, parent)).unwrap();
        arena.remove(parent);
        let reused = arena.insert(());
        assert_ne!(reused, parent, "the reused slot carries a new version");
        list.push((reused, piece))
            .expect("a key minted into a reused slot is new to the list");
    }

    /// **Every list the kernel makes names itself by what it records,
    /// not by its key type**: the lists in a Boolean's and a split's
    /// naming, made by their `Default`s, carry their own arm, though
    /// vertex fusions and cone copies share a key type, and so do the
    /// split's and an operand's edge divisions. Red if a `Default`
    /// hands a list another list's arm.
    #[test]
    fn every_list_the_kernel_makes_names_itself() {
        let boolean = crate::BooleanNaming::default();
        assert_eq!(boolean.vertex_merges.tag(), LineageRows::Fusions);
        assert_eq!(boolean.substitutions.tag(), LineageRows::Substitutions);
        assert_eq!(boolean.face_fragments_a.tag(), LineageRows::FaceDivisions);
        assert_eq!(boolean.face_fragments_b.tag(), LineageRows::FaceDivisions);
        assert_eq!(boolean.cone_splits.tag(), LineageRows::ConeCopies);
        let split = crate::splitting::SplitNaming::default();
        assert_eq!(split.face_fragments.tag(), LineageRows::FaceDivisions);
        assert_eq!(split.edge_pieces.tag(), LineageRows::EdgeDivisions(None));
        let mut vertices: SlotMap<crate::entity::VertexKey, ()> = SlotMap::with_key();
        let v = vertices.insert(());
        assert_eq!(
            boolean.vertex_merges.clone().push((v, v)).unwrap_err().rows,
            LineageRows::Fusions,
            "a refusal names the list's own arm"
        );
    }

    /// **An edge division list refuses an edge it already names, where
    /// the split is recorded**, so [`EdgeDivisions::root`] (the walk a
    /// reduction's piece takes back to its operand edge) is total. Red
    /// if the push takes a non-fresh or self row.
    #[test]
    fn an_edge_division_list_refuses_an_edge_it_already_names() {
        use crate::entity::EdgeKey;
        let mut arena: SlotMap<EdgeKey, ()> = SlotMap::with_key();
        let e: Vec<EdgeKey> = (0..4).map(|_| arena.insert(())).collect();
        let mut list =
            EdgeDivisions::new(LineageRows::EdgeDivisions(Some(crate::boolean::Operand::B)));
        list.push((e[1], e[0])).unwrap();
        let refused = |key, defect| {
            Err(RowRefusal {
                rows: LineageRows::EdgeDivisions(Some(crate::boolean::Operand::B)),
                defect,
                key: Cell::Edge(key),
            })
        };
        assert_eq!(list.push((e[0], e[1])), refused(e[0], RowDefect::NotFresh));
        assert_eq!(list.push((e[1], e[2])), refused(e[1], RowDefect::NotFresh));
        assert_eq!(list.push((e[2], e[2])), refused(e[2], RowDefect::SelfRow));
        assert!(
            list.push((e[0], e[1]))
                .unwrap_err()
                .to_string()
                .starts_with("a second operand's edge division row"),
            "the refusal names whose clone the key is a key of"
        );
        list.push((e[2], e[1])).unwrap();
        list.push((e[3], e[0])).unwrap();
        assert_eq!(
            e.iter().map(|&k| list.root(k)).collect::<Vec<_>>(),
            [e[0]; 4]
        );
    }

    /// **A cone copy list refuses a copy it already names**, so a
    /// vertex's copies form a tree: its [`Descent`] lists each once, and
    /// [`ConeCopies::root`] reaches the split vertex. Red if the push takes a non-fresh row (the copy of
    /// a copy's original would close a cycle the forward walk could not
    /// leave).
    #[test]
    fn a_copy_per_cone_list_refuses_a_copy_it_already_names() {
        use crate::entity::VertexKey;
        let mut arena: SlotMap<VertexKey, ()> = SlotMap::with_key();
        let v: Vec<VertexKey> = (0..4).map(|_| arena.insert(())).collect();
        let mut list = ConeCopies::new(LineageRows::ConeCopies);
        for row in [(v[1], v[0]), (v[2], v[1]), (v[3], v[0])] {
            list.push(row).unwrap();
        }
        assert_eq!(
            list.push((v[0], v[2])),
            Err(RowRefusal {
                rows: LineageRows::ConeCopies,
                defect: RowDefect::NotFresh,
                key: Cell::Vertex(v[0]),
            })
        );
        assert_eq!(list.rows().len(), 3, "the list is as it was");
        let copies = |at| Descent::of(at, list.rows()).cells().collect::<Vec<_>>();
        assert_eq!(copies(v[0]), v);
        assert_eq!(copies(v[2]), [v[2]]);
        assert_eq!(
            v.iter().map(|&k| list.root(k)).collect::<Vec<_>>(),
            [v[0]; 4]
        );
    }

    /// **The same invariant over vertices**: a vertex removed and a
    /// vertex minted into its slot get keys that differ, so the copy a
    /// null edge mints there is fresh to a copy list that names the
    /// first. Red if slot reuse stops versioning vertex keys.
    #[test]
    fn a_vertex_minted_into_a_reused_slot_is_fresh_to_the_list() {
        use crate::entity::VertexKey;
        let mut arena: SlotMap<VertexKey, ()> = SlotMap::with_key();
        let original = arena.insert(());
        let copy = arena.insert(());
        let mut list = NullEdgeCopies::new(LineageRows::NullEdgeCopies(crate::boolean::Operand::A));
        list.push((copy, original)).unwrap();
        arena.remove(original);
        let reused = arena.insert(());
        assert_ne!(reused, original, "the reused slot carries a new version");
        list.push((reused, copy))
            .expect("a key minted into a reused slot is new to the list");
    }

    /// **A null-edge copy list refuses a copy it already names, where the
    /// copy is recorded.** A copy recorded again, or a copy of the vertex
    /// a copy of it was minted at, would close a cycle the chase back to
    /// the original could not leave; the list refuses it at the push and
    /// stays as it was, so [`Minted::root`] is total. Red if the
    /// push takes the row.
    #[test]
    fn a_null_edge_copy_list_refuses_a_copy_it_already_names() {
        use crate::entity::VertexKey;
        let mut arena: SlotMap<VertexKey, ()> = SlotMap::with_key();
        let v: Vec<VertexKey> = (0..4).map(|_| arena.insert(())).collect();
        let mut list = NullEdgeCopies::new(LineageRows::NullEdgeCopies(crate::boolean::Operand::A));
        list.push((v[1], v[0])).unwrap();
        let refused = |key, defect| {
            Err(RowRefusal {
                rows: LineageRows::NullEdgeCopies(crate::boolean::Operand::A),
                defect,
                key: Cell::Vertex(key),
            })
        };
        assert_eq!(list.push((v[0], v[1])), refused(v[0], RowDefect::NotFresh));
        assert_eq!(list.push((v[1], v[2])), refused(v[1], RowDefect::NotFresh));
        assert_eq!(list.push((v[2], v[2])), refused(v[2], RowDefect::SelfRow));
        list.push((v[2], v[1])).unwrap();
        list.push((v[3], v[0])).unwrap();
        assert_eq!(
            v.iter().map(|&k| list.root(k)).collect::<Vec<_>>(),
            [v[0]; 4]
        );
    }

    /// **A refused row reads the same in every lane that carries it.**
    /// One sentence for the row ([`RowRefusal`]'s `Display`), inside
    /// the Boolean's refusal, the chord join's, and the join's lifted
    /// into the Boolean's (`BooleanError`'s `From<SplitJoinError>`, which hands
    /// it over as the Boolean's own [`crate::BooleanError::LineageRow`]).
    /// Red if a lane spells the row again, or the lift nests it.
    #[test]
    fn a_refused_row_reads_alike_in_every_lane_that_carries_it() {
        use crate::boolean::BooleanError;
        use crate::chord_join::SplitJoinError;
        let f = faces(1);
        let refusal = RowRefusal {
            rows: LineageRows::FaceDivisions,
            defect: RowDefect::NotFresh,
            key: Cell::Face(f[0]),
        };
        let row = refusal.to_string();
        assert_eq!(
            row,
            format!(
                "a face division row that mints a cell an earlier row already names ({:?})",
                Cell::Face(f[0])
            )
        );
        let lifted = BooleanError::from(SplitJoinError::LineageRow(refusal));
        assert!(
            matches!(
                lifted,
                BooleanError::LineageRow { rows: LineageRows::FaceDivisions, defect: RowDefect::NotFresh, key }
                    if key == Cell::Face(f[0])
            ),
            "the lift is the Boolean's own: {lifted:?}"
        );
        for shown in [
            lifted.to_string(),
            SplitJoinError::LineageRow(refusal).to_string(),
        ] {
            assert!(shown.contains(&row), "{shown}");
            assert!(shown.ends_with(geom_core::KERNEL_DEFECT_ENDING), "{shown}");
        }
    }

    /// **A fused vertex's image follows the fusions, then the
    /// substitutions.** Fusions A → B and B → C, then a merge that kills
    /// C into face F: A, B and C all image to F. Red where the fusions
    /// sit in the merges' one-hop map, which stops at C, a dead cell.
    /// No body reaches a chain of fusions today, so the chain is
    /// synthetic.
    #[test]
    fn a_fused_vertex_reads_through_a_later_substitution_of_its_survivor() {
        let v = |n: u64| -> crate::entity::VertexKey {
            slotmap::KeyData::from_ffi((1u64 << 32) | n).into()
        };
        let face: FaceKey = slotmap::KeyData::from_ffi((1u64 << 32) | 9).into();
        let (a, b, c) = (v(1), v(2), v(3));
        let mut fusions = Fusions::new(LineageRows::Fusions);
        fusions.push((a, b)).unwrap();
        fusions.push((b, c)).unwrap();
        let lineage = Lineage::new()
            .through_fusions(&fusions)
            .through_merges([(Cell::Vertex(c), Cell::Face(face))]);
        for x in [a, b, c] {
            assert_eq!(
                lineage.images(Cell::Vertex(x)),
                vec![Cell::Face(face)],
                "{x:?}'s image"
            );
        }
        let live = |k: Cell| k == Cell::Face(face);
        assert_eq!(
            settle_through(
                &fusions,
                Cell::Vertex(a),
                |k| (k == Cell::Vertex(c)).then_some(Cell::Face(face)),
                live
            ),
            Ok(Cell::Face(face)),
            "the settler's chase"
        );
    }
}
