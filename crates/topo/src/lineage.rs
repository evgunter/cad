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
//! op's `lineage()` states its fates from them. [`Descent`] is the walk
//! those division rows need: a cell and every piece the rows divide
//! off it.

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
/// is kept under its key; where the result does not hold it there, the
/// lineage does not cover it and the carry refuses
/// ([`crate::CarryRefusal::Unmapped`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lineage {
    rows: BTreeMap<Cell, Fate>,
    joins: Vec<EdgeJoin>,
    meeting: MintedBacking,
    glues: bool,
}

impl Default for Lineage {
    fn default() -> Self {
        Self {
            rows: BTreeMap::new(),
            joins: Vec::new(),
            meeting: MintedBacking::Refused,
            glues: false,
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

    /// The lineage read through `joins`, the joins the op ended with:
    /// each image a join killed is the edge it made.
    #[must_use]
    pub fn through_joins(mut self, joins: &[EdgeJoin]) -> Self {
        self.joins.extend_from_slice(joins);
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

    /// An image read through the joins.
    pub(crate) fn joined(&self, cell: Cell) -> Cell {
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

/// **A cell's descent**: the cell and every cell the division rows
/// (`(new cell, divided-from cell)`, in any order) divide off it or off
/// one of its pieces.
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
    /// A vertex fusion list ([`crate::Fusions`]).
    Fusions,
    /// A substitution list ([`Substitutions`]).
    Substitutions,
    /// A face division list ([`FaceDivisions`]).
    FaceDivisions,
    /// A Boolean's null-edge copy list (`VertexCopies`).
    NullEdgeCopies,
}

impl LineageRows {
    /// The list, as a sentence names it.
    #[must_use]
    pub fn word(self) -> &'static str {
        match self {
            Self::Fusions => "vertex fusion",
            Self::Substitutions => "substitution",
            Self::FaceDivisions => "face division",
            Self::NullEdgeCopies => "null-edge copy",
        }
    }
}

/// What is wrong with a lineage row a list refused at append
/// ([`Substitutions::push`], [`FaceDivisions::push`], `Fusions::push`,
/// `VertexCopies::push`).
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
    /// A row's minted cell (a division's new face, a null edge's copy)
    /// is named by an earlier row: a minted cell is new to the list.
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

/// A key a substitution list can hold: the list it names in a refusal,
/// and the cell it is.
pub trait LineageKey: Ord + Copy + core::fmt::Debug {
    /// The list a refusal over this key names.
    const ROWS: LineageRows;
    /// The key as a cell.
    fn cell(self) -> Cell;
}

impl LineageKey for Cell {
    const ROWS: LineageRows = LineageRows::Substitutions;
    fn cell(self) -> Cell {
        self
    }
}

impl LineageKey for FaceKey {
    const ROWS: LineageRows = LineageRows::Substitutions;
    fn cell(self) -> Cell {
        Cell::Face(self)
    }
}

impl LineageKey for crate::entity::VertexKey {
    const ROWS: LineageRows = LineageRows::Fusions;
    fn cell(self) -> Cell {
        Cell::Vertex(self)
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
/// One rule, two instances: over cells (the boolean's descendant map
/// and its naming's substitutions) and over vertices ([`Fusions`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Substitutions<K: LineageKey = Cell> {
    rows: Vec<(K, K)>,
    by_dead: std::collections::BTreeMap<K, K>,
}

/// **A vertex fusion list**: the fusions `(dead, kept)` in the order
/// their `kev`s ran, the vertex instance of [`Substitutions`]: each key
/// folds onto one survivor ([`Substitutions::survivor`]).
pub type Fusions = Substitutions<crate::entity::VertexKey>;

impl<K: LineageKey> Default for Substitutions<K> {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            by_dead: std::collections::BTreeMap::new(),
        }
    }
}

impl<K: LineageKey> Substitutions<K> {
    /// Appends the row `(dead, by)`.
    ///
    /// # Errors
    ///
    /// [`RowDefect::SelfRow`] where `dead` is `by`, and
    /// [`RowDefect::NamesRetired`] where an earlier row retired either;
    /// the list is left as it was.
    pub(crate) fn push(&mut self, (dead, by): (K, K)) -> Result<(), RowRefusal> {
        let refuse = |defect, key: K| {
            Err(RowRefusal {
                rows: K::ROWS,
                defect,
                key: key.cell(),
            })
        };
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

/// **Rows `(minted cell, cell it was minted from)`, each minted cell
/// new to the list**: the one check [`FaceDivisions`] and
/// [`VertexCopies`] make where a row is added. A row whose two cells
/// are one, or whose minted cell an earlier row names, is refused.
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
/// therefore ends. An arena that reused a bare key would break the
/// invariant, and [`Self::push`] would refuse the row
/// ([`RowDefect::NotFresh`]) rather than let the list cycle.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Minted<K> {
    rows: Vec<(K, K)>,
    /// Each minted cell → the cell it was minted from.
    from: BTreeMap<K, K>,
    /// Every cell a row names, minted or minted-from.
    named: BTreeSet<K>,
}

impl<K> Default for Minted<K> {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            from: BTreeMap::new(),
            named: BTreeSet::new(),
        }
    }
}

impl<K: Ord + Copy> Minted<K> {
    /// Appends `(new, from)`, a refusal naming `list` and `new`'s cell
    /// where it is refused; the list is left as it was.
    fn push(
        &mut self,
        (new, from): (K, K),
        list: LineageRows,
        cell: impl FnOnce(K) -> Cell,
    ) -> Result<(), RowRefusal> {
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
        Err(RowRefusal {
            rows: list,
            defect,
            key: cell(new),
        })
    }

    fn from(&self, cell: K) -> Option<K> {
        self.from.get(&cell).copied()
    }

    /// Total, since the list cannot cycle.
    fn root(&self, cell: K) -> K {
        let mut at = cell;
        while let Some(from) = self.from(at) {
            at = from;
        }
        at
    }
}

/// **A face division list**: rows `(new face, divided-from face)` in
/// the order the faces were minted. A division mints its new face
/// (`mef`, `mfkrh`), so [`Self::push`] refuses a row whose two faces
/// are one, or whose new face an earlier row names: the list cannot
/// cycle (`Minted`'s invariant).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FaceDivisions(Minted<FaceKey>);

impl FaceDivisions {
    /// Appends the division `(new, from)`.
    ///
    /// # Errors
    ///
    /// [`RowDefect::SelfRow`] where `new` is `from`, and
    /// [`RowDefect::NotFresh`] where an earlier row names `new`; the
    /// list is left as it was.
    pub(crate) fn push(&mut self, row: (FaceKey, FaceKey)) -> Result<(), RowRefusal> {
        self.0.push(row, LineageRows::FaceDivisions, Cell::Face)
    }

    /// The list of `rows`, each added in turn as [`Self::push`] adds it:
    /// the one way to build a list outside the kernel, where a mint
    /// records its own rows.
    ///
    /// # Errors
    ///
    /// As [`Self::push`], at the first row it refuses.
    pub fn from_rows(
        rows: impl IntoIterator<Item = (FaceKey, FaceKey)>,
    ) -> Result<Self, RowRefusal> {
        let mut list = Self::default();
        rows.into_iter().try_for_each(|row| list.push(row))?;
        Ok(list)
    }

    /// The rows, in the order they were added.
    #[must_use]
    pub fn rows(&self) -> &[(FaceKey, FaceKey)] {
        &self.0.rows
    }

    /// The face `face` was divided from, where a row divided it off one.
    #[must_use]
    pub fn from(&self, face: FaceKey) -> Option<FaceKey> {
        self.0.from(face)
    }

    /// The face `face` descends from through every division: itself
    /// where no row divided it off one. Total, since the list cannot
    /// cycle.
    #[must_use]
    pub fn root(&self, face: FaceKey) -> FaceKey {
        self.0.root(face)
    }
}

/// **A null-edge copy list**: rows `(copy, vertex it copies)`, one per
/// end of a null edge other than the vertex it was minted at, in the
/// order the null edges were minted. A null edge mints its copy
/// (`mev`), so [`Self::push`] refuses a row whose two vertices are one,
/// or whose copy an earlier row names: the list cannot cycle
/// (`Minted`'s invariant).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct VertexCopies(Minted<crate::entity::VertexKey>);

impl VertexCopies {
    /// Appends the copy `(copy, of)`.
    ///
    /// # Errors
    ///
    /// [`RowDefect::SelfRow`] where `copy` is `of`, and
    /// [`RowDefect::NotFresh`] where an earlier row names `copy`; the
    /// list is left as it was.
    pub(crate) fn push(
        &mut self,
        row: (crate::entity::VertexKey, crate::entity::VertexKey),
    ) -> Result<(), RowRefusal> {
        self.0.push(row, LineageRows::NullEdgeCopies, Cell::Vertex)
    }

    /// The vertex `v` is a copy of through every copy: `v` itself where
    /// no row copied it. Total, since the list cannot cycle.
    #[must_use]
    pub(crate) fn root(&self, v: crate::entity::VertexKey) -> crate::entity::VertexKey {
        self.0.root(v)
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
        let mut list: Substitutions = Substitutions::default();
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
        let mut list = FaceDivisions::default();
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
        let mut list = FaceDivisions::default();
        list.push((piece, parent)).unwrap();
        arena.remove(parent);
        let reused = arena.insert(());
        assert_ne!(reused, parent, "the reused slot carries a new version");
        list.push((reused, piece))
            .expect("a key minted into a reused slot is new to the list");
    }

    /// **A null-edge copy list refuses a copy it already names, where the
    /// copy is recorded.** A copy recorded again, or a copy of the vertex
    /// a copy of it was minted at, would close a cycle the chase back to
    /// the original could not leave; the list refuses it at the push and
    /// stays as it was, so [`VertexCopies::root`] is total. Red if the
    /// push takes the row.
    #[test]
    fn a_null_edge_copy_list_refuses_a_copy_it_already_names() {
        use crate::entity::VertexKey;
        let mut arena: SlotMap<VertexKey, ()> = SlotMap::with_key();
        let v: Vec<VertexKey> = (0..4).map(|_| arena.insert(())).collect();
        let mut list = VertexCopies::default();
        list.push((v[1], v[0])).unwrap();
        let refused = |key, defect| {
            Err(RowRefusal {
                rows: LineageRows::NullEdgeCopies,
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
}
