//! **What an op divided a cell into.** An op that divides a cell keeps
//! its key on one piece and mints the rest, each logged as a division
//! row `(new cell, divided-from cell)`; a key read before the division
//! names the cell it started from. [`Lineage`] is that cell and every
//! piece divided off it, and [`Divisions`] is one op's rows, the form a
//! surgery verb hands its record carry
//! ([`crate::ContactRecords::carried_through_pieces`]).

use std::collections::BTreeSet;

use crate::boolean::Cell;
use crate::entity::{EdgeKey, FaceKey};

/// **A cell's lineage**: the cell and every cell the division rows
/// (`(new cell, divided-from cell)`, in any order) divide off it or off
/// one of its pieces.
pub(crate) struct Lineage<K> {
    root: K,
    cells: BTreeSet<K>,
}

impl<K: Ord + Copy> Lineage<K> {
    /// `root`'s lineage through `rows`.
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
        Self { root, cells }
    }

    /// The lineage's cells, the root among them, in key order.
    pub(crate) fn cells(&self) -> impl Iterator<Item = K> + '_ {
        self.cells.iter().copied()
    }

    /// The cells divided off the root, in key order.
    pub(crate) fn pieces(&self) -> impl Iterator<Item = K> + '_ {
        self.cells.iter().copied().filter(|&c| c != self.root)
    }
}

/// **One op's division rows**, `(new cell, divided-from cell)`, edges
/// and faces apart: what the op divided each input cell into. A row
/// whose two cells are one is no division and is read as none.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Divisions {
    /// Each piece an edge split minted, with the edge it was split from.
    pub edges: Vec<(EdgeKey, EdgeKey)>,
    /// Each piece a face division minted, with the face it was divided
    /// from.
    pub faces: Vec<(FaceKey, FaceKey)>,
}

impl Divisions {
    /// The cells the rows divide `cell` into, `cell` excluded: none for
    /// a vertex, or for a cell no row divides.
    #[must_use]
    pub fn pieces(&self, cell: Cell) -> Vec<Cell> {
        match cell {
            Cell::Vertex(_) => Vec::new(),
            Cell::Edge(e) => Lineage::of(e, &self.edges)
                .pieces()
                .map(Cell::Edge)
                .collect(),
            Cell::Face(f) => Lineage::of(f, &self.faces)
                .pieces()
                .map(Cell::Face)
                .collect(),
        }
    }
}

/// Which lineage list refused a row.
///
/// **Not carried by the façade.** Every refusal it rides
/// ([`crate::BooleanError::LineageRow`],
/// [`crate::chord_join::SplitJoinError::LineageRow`]) is a kernel defect
/// and ends in the defect ending: no move of the model reaches it, so
/// nothing branches on the arm. Falsifier: a row a caller's input can
/// make a list refuse, which would owe its own decision and lever.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineageRows {
    /// A vertex fusion list ([`crate::Fusions`]).
    Fusions,
    /// A substitution list ([`Substitutions`]).
    Substitutions,
    /// A face division list ([`FaceDivisions`]).
    FaceDivisions,
}

impl LineageRows {
    /// The list, as a sentence names it.
    #[must_use]
    pub fn word(self) -> &'static str {
        match self {
            Self::Fusions => "vertex fusions",
            Self::Substitutions => "substitutions",
            Self::FaceDivisions => "face divisions",
        }
    }
}

/// What is wrong with a lineage row a list refused at append
/// ([`Substitutions::push`], [`FaceDivisions::push`], `Fusions::push`).
///
/// **Not carried by the façade.** Every refusal it rides
/// ([`crate::BooleanError::LineageRow`],
/// [`crate::chord_join::SplitJoinError::LineageRow`]) is a kernel defect
/// and ends in the defect ending: no move of the model reaches it, so
/// nothing branches on the arm. Falsifier: a row a caller's input can
/// make a list refuse, which would owe its own decision and lever.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowDefect {
    /// The row maps a cell to itself.
    SelfRow,
    /// The row names a cell an earlier row of the list retired.
    NamesRetired,
    /// A division row's new cell is named by an earlier row: a minted
    /// cell is new to the list.
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
    pub cell: Cell,
}

impl core::fmt::Display for RowRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "a row of its {} {} ({:?})",
            self.rows.word(),
            self.defect.word(),
            self.cell
        )
    }
}

/// **A substitution list**: rows `(dead, by)`, each saying `by` holds
/// `dead`'s interior now, in the order the stages retired them.
/// [`Self::push`] refuses a row that maps a cell to itself or names a
/// cell an earlier row retired, so the list cannot cycle and a chase
/// through it ends within its length. The list sees only its own rows.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Substitutions {
    rows: Vec<(Cell, Cell)>,
    by_dead: std::collections::BTreeMap<Cell, Cell>,
}

impl Substitutions {
    /// Appends the row `(dead, by)`.
    ///
    /// # Errors
    ///
    /// [`RowDefect::SelfRow`] where `dead` is `by`, and
    /// [`RowDefect::NamesRetired`] where an earlier row retired either;
    /// the list is left as it was.
    pub fn push(&mut self, (dead, by): (Cell, Cell)) -> Result<(), RowRefusal> {
        let refuse = |defect, cell| {
            Err(RowRefusal {
                rows: LineageRows::Substitutions,
                defect,
                cell,
            })
        };
        if dead == by {
            return refuse(RowDefect::SelfRow, dead);
        }
        if let Some(cell) = [dead, by]
            .into_iter()
            .find(|c| self.by_dead.contains_key(c))
        {
            return refuse(RowDefect::NamesRetired, cell);
        }
        self.by_dead.insert(dead, by);
        self.rows.push((dead, by));
        Ok(())
    }

    /// The cell holding `dead`'s interior, where a row retired it.
    #[must_use]
    pub fn get(&self, dead: Cell) -> Option<Cell> {
        self.by_dead.get(&dead).copied()
    }

    /// The rows, in the order they were added.
    #[must_use]
    pub fn rows(&self) -> &[(Cell, Cell)] {
        &self.rows
    }

    /// The first cell `live` holds along the rows from `at`: `Ok(live)`,
    /// or `Err(dead)` where the rows stop at a cell `live` does not hold.
    /// Total, since the list cannot cycle.
    pub fn chase(&self, mut at: Cell, live: impl Fn(Cell) -> bool) -> Result<Cell, Cell> {
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

/// **A face division list**: rows `(new face, divided-from face)` in
/// the order the faces were minted. [`Self::push`] refuses a row whose
/// two faces are one, or whose new face an earlier row names.
///
/// **The invariant: a minted face is new to the list.** A division
/// mints its new face (`mef`, `mfkrh`), and an arena never hands out a
/// key it handed out before: a slot it reuses carries a new version,
/// so the new key equals no key an earlier row holds. That is what
/// makes the check hold where the rows are written, and it is all the
/// list needs to be acyclic: in a cycle each row's divided-from face is
/// the next row's new face, which no row before that one may name, so
/// each row would have to come before the one ahead of it all the way
/// round. A reader's chase to the face a face descends from
/// ([`Self::root`]) therefore ends. An arena that reused a bare key
/// would break the invariant, and [`Self::push`] would refuse the row
/// ([`RowDefect::NotFresh`]) rather than let the list cycle.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FaceDivisions {
    rows: Vec<(FaceKey, FaceKey)>,
    /// Each new face → the face it was divided from.
    from: std::collections::BTreeMap<FaceKey, FaceKey>,
    /// Every face a row names, new or divided-from.
    named: BTreeSet<FaceKey>,
}

impl FaceDivisions {
    /// Appends the division `(new, from)`.
    ///
    /// # Errors
    ///
    /// [`RowDefect::SelfRow`] where `new` is `from`, and
    /// [`RowDefect::NotFresh`] where an earlier row names `new`; the
    /// list is left as it was.
    pub fn push(&mut self, (new, from): (FaceKey, FaceKey)) -> Result<(), RowRefusal> {
        let refuse = |defect| {
            Err(RowRefusal {
                rows: LineageRows::FaceDivisions,
                defect,
                cell: Cell::Face(new),
            })
        };
        if new == from {
            return refuse(RowDefect::SelfRow);
        }
        if self.named.contains(&new) {
            return refuse(RowDefect::NotFresh);
        }
        self.named.extend([new, from]);
        self.from.insert(new, from);
        self.rows.push((new, from));
        Ok(())
    }

    /// The rows, in the order they were added.
    #[must_use]
    pub fn rows(&self) -> &[(FaceKey, FaceKey)] {
        &self.rows
    }

    /// The face `face` was divided from, where a row divided it off one.
    #[must_use]
    pub fn from(&self, face: FaceKey) -> Option<FaceKey> {
        self.from.get(&face).copied()
    }

    /// The face `face` descends from through every division: itself
    /// where no row divided it off one. Total, since the list cannot
    /// cycle.
    #[must_use]
    pub fn root(&self, face: FaceKey) -> FaceKey {
        let mut at = face;
        while let Some(from) = self.from(at) {
            at = from;
        }
        at
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

    #[test]
    fn a_lineage_follows_rows_listed_after_the_rows_that_read_them() {
        let f = faces(5);
        // f[0] divided into f[1], f[1] into f[2], listed child first;
        // f[3] divided off f[4], a face outside the lineage.
        let rows = [(f[2], f[1]), (f[3], f[4]), (f[1], f[0])];
        let lineage = Lineage::of(f[0], &rows);
        for (i, want) in [true, true, true, false, false].into_iter().enumerate() {
            assert_eq!(
                lineage.cells().any(|g| g == f[i]),
                want,
                "face {i} of f0's lineage"
            );
        }
        assert_eq!(
            lineage.pieces().collect::<Vec<_>>().len(),
            2,
            "f0 itself is no piece"
        );
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
        let mut list = Substitutions::default();
        list.push((a, b)).unwrap();
        let refused = |defect, cell| {
            Err(RowRefusal {
                rows: LineageRows::Substitutions,
                defect,
                cell,
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
                cell: Cell::Face(face),
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

    /// **The invariant the face division list stands on: an arena never
    /// hands a key out twice.** A face removed and a face minted into
    /// its slot get keys that differ, so the division that mints the
    /// second is fresh to a list that names the first, and the list
    /// takes it. If the arena reused the bare key, the new face would
    /// equal one the list names; the second half spells that key out
    /// and shows the list refuses it instead of cycling. Red if slot
    /// reuse stops versioning keys (the first push refuses), or if the
    /// fresh check is dropped (the second pushes).
    #[test]
    fn a_face_minted_into_a_reused_slot_is_fresh_and_a_bare_reused_key_is_not() {
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

        let bare: FaceKey = slotmap::KeyData::from_ffi(slotmap::Key::data(&parent).as_ffi()).into();
        assert_eq!(
            list.push((bare, piece)),
            Err(RowRefusal {
                rows: LineageRows::FaceDivisions,
                defect: RowDefect::NotFresh,
                cell: Cell::Face(parent),
            }),
            "a key the list names, handed out again, is refused"
        );
    }
}
