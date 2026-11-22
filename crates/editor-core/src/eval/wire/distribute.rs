//! **`−` and `∩` distribute over `⊔`** (D10 `[operations.operators]`):
//! a `⊔` value is its pieces, so a boolean reading one acts on each
//! piece and assembles what it made, `(p ⊔ q) − t = (p − t) ⊔ (q − t)`
//! and `(p ⊔ q) ∩ t = (p ∩ t) ⊔ (q ∩ t)`. A meeting inside the `⊔` is
//! its own decision and no boolean above it decides it again. The
//! kernel reads such a `⊔` whole too, where its pieces' seat is an area
//! record (`topo::boolean`'s declared wall), so this is a second
//! implementation of the rule, which
//! `work/tally/the-editors-distribution-over-a-seat-retires-for-the-kernels-whole-body-boolean.md`
//! retires.
//!
//! Each piece's result keeps that piece's records and is keyed by the
//! piece's read, as the `⊔` keyed it ([`names::name_assemble`]); a
//! piece the boolean empties drops out, and a result with no piece left
//! is the typed empty result. The rows a piece's boolean decided name
//! its cells as the node's operand spells them, through the read the
//! node took it in by, so the coincidence door reads them where the
//! operand stands.
//!
//! A `⊔` read as the TOOL of a cut is cut by whole: the kernel reads it
//! as its material, its pieces fused on their seats' records
//! (`crates/topo/README.md`, `[topo.cut-material]`).

use std::sync::Arc;

use geom_core::{Decide, Tol};

use super::{BooleanValue, NodeErrorKind, OpOut, OpResult, ValuePayload};
use crate::eval::{Piece, Pieces};
use crate::names::{self, NameTable};
use crate::node::RecipeNodeId;

/// **A boolean's operand, as a distribution reads it**: a body read
/// through a seat, or one piece of one, with its names, records and
/// pieces.
pub(super) struct Operand<T: topo::AtRestPolicy + Decide> {
    /// The node it is read from (error context).
    pub node: RecipeNodeId,
    /// The read its entities are keyed by in what the boolean makes:
    /// the seat's, or the piece's.
    pub read: crate::VarId,
    /// Its body, where the node reads it.
    pub body: crate::eval::Framed<T>,
    /// Its own names, total over `body`.
    pub table: Arc<NameTable>,
    /// What its body carries, as a reader reads it
    /// ([`crate::eval::Carry::kept`]): its records keyed in `body`'s
    /// arena, and the pieces it holds. A unit takes its piece's carry as
    /// the piece holds it.
    pub carry: crate::eval::Carry<T>,
    /// The read the node takes it in through: what its rows name it by.
    pub seat: crate::VarId,
    /// How a piece's names nest in the seat's table: each holder's
    /// keying node and the piece's read there, outermost first.
    pub wraps: Vec<(RecipeNodeId, crate::VarId)>,
}

impl<T: Decide + topo::AtRestPolicy> Operand<T> {
    /// The body `node` defines, read through `read`.
    pub(super) fn read(
        results: &super::Results<T>,
        node: RecipeNodeId,
        read: crate::VarId,
        doc: &crate::doc::Doc<crate::program::ProfileProgram>,
        tol: Tol,
    ) -> Result<Self, NodeErrorKind> {
        let value = super::value_of(results, node)?;
        Ok(Self {
            node,
            read,
            body: super::read_body(results, node)?,
            table: super::read_table(value),
            carry: value.carry.kept(doc, tol),
            seat: read,
            wraps: Vec::new(),
        })
    }

    /// **Its body, taken** as a door that takes finished bodies takes it
    /// ([`crate::eval::Held::finished`]), in its own frame.
    pub(super) fn taken(&self, tol: Tol) -> Result<Arc<T::Finished>, NodeErrorKind> {
        self.body
            .holding()
            .finished(tol)
            .map_err(NodeErrorKind::unfinished(self.node))
    }

    /// **Its body placed in `into`'s frame** ([`topo::Placed`]): `own`,
    /// its body already taken in its own frame ([`Self::taken`]), under
    /// the motion into `into`, or alone where the two are one frame. The
    /// door it is handed to moves it.
    pub(super) fn placed_in<'a>(
        &self,
        into: &crate::eval::Frame<T>,
        own: &'a Arc<T::Finished>,
    ) -> Result<topo::Placed<'a, T::Finished>, NodeErrorKind> {
        self.body
            .placed_in(into, own)
            .map_err(NodeErrorKind::Motion)
    }

    /// Its names as the seat's table spells them: each holder's
    /// keying, innermost first.
    pub(super) fn spelled(&self) -> Result<Arc<NameTable>, NodeErrorKind> {
        let mut view = Arc::clone(&self.table);
        for &(by, read) in self.wraps.iter().rev() {
            view = Arc::new(names::member_view(by, read, &view).map_err(NodeErrorKind::Naming)?);
        }
        Ok(view)
    }

    /// Its pieces, each an operand where it stands: a family piece is
    /// one per member, each its member's names and none of the
    /// family's records (as a union reads a family's members).
    pub(super) fn units(&self) -> Result<Vec<Self>, NodeErrorKind> {
        let Some(pieces) = &self.carry.pieces else {
            return Ok(Vec::new());
        };
        let mut units = Vec::with_capacity(pieces.pieces.len());
        for piece in &pieces.pieces {
            let bodies = piece.bodies.clone();
            let mut wraps = self.wraps.clone();
            wraps.push((pieces.by, piece.read));
            let unit = |body: crate::eval::Framed<T>, table, carry: &crate::eval::Carry<T>| Self {
                node: self.node,
                read: piece.read,
                body,
                table,
                carry: carry.clone(),
                seat: self.seat,
                wraps: wraps.clone(),
            };
            if piece.family {
                for (j, body) in bodies.into_iter().enumerate() {
                    let index = names::narrow(j, names::Numbered::OutputBody)
                        .map_err(NodeErrorKind::Naming)?;
                    let table = piece
                        .table
                        .project(index)
                        .map_err(|dup| NodeErrorKind::Naming(names::NamingError::from(dup)))?;
                    units.push(unit(body, Arc::new(table), piece.carry.member(j)));
                }
            } else {
                let Some(body) = bodies.into_iter().next() else {
                    unreachable!("a piece that is a body holds one")
                };
                units.push(unit(body, Arc::clone(&piece.table), &piece.carry));
            }
        }
        Ok(units)
    }

    /// `rows` with every cell this piece's boolean named by its own
    /// read and table spelled as the seat's table spells it. A cell
    /// named by a tie is left as it is.
    pub(super) fn respelled(
        &self,
        rows: &[crate::coincide::NamedCoincidence],
    ) -> Result<Vec<crate::coincide::NamedCoincidence>, NodeErrorKind> {
        use crate::coincide::NamedCell;
        let spelled = self.spelled()?;
        Ok(rows
            .iter()
            .map(|row| crate::coincide::NamedCoincidence {
                cells: row
                    .cells
                    .iter()
                    .map(|cell| match cell {
                        NamedCell::Entity { input, name } if *input == self.read => {
                            match self.table.lookup(name) {
                                Some(names::Entry::Unique(e)) => spelled.name_of(e).map_or_else(
                                    || cell.clone(),
                                    |name| NamedCell::Entity {
                                        input: self.seat,
                                        name: name.clone(),
                                    },
                                ),
                                _ => cell.clone(),
                            }
                        }
                        _ => cell.clone(),
                    })
                    .collect(),
                ..row.clone()
            })
            .collect())
    }
}

/// **What a boolean made of each piece, assembled** (`id`'s output):
/// each non-empty result grafted beside the others as a piece keyed by
/// the read its piece came through, with that result's names, rows and
/// records. A record a piece's result holds cites, at this value, what
/// it cited there: its rows renumbered onto the assembled rows and its
/// inputs onto the assembled inputs ([`crate::eval::Citing::Each`]). A
/// touch the door's census records between two pieces cites the
/// census's row, numbered after every piece's row; those rows are the
/// census's own ([`crate::product::Census`]) and are not among the
/// value's coincidences. Nothing left is the typed empty result.
pub(super) fn assembled<T: Decide + topo::AtRestPolicy>(
    id: RecipeNodeId,
    outs: Vec<(crate::VarId, OpOut<T>)>,
    tol: Tol,
) -> OpResult<T> {
    let mut fused = topo::Assembled::new();
    // Computed in its first result's frame
    // ([`crate::eval::Framed::computing_among`]), each grafted into it
    // through its own placement ([`crate::eval::Framed::graft_into`]).
    let frame = crate::eval::Framed::computing_among(
        outs.iter()
            .filter_map(|(_, out)| out.payload.one_body().flatten()),
    )
    .map_or(crate::eval::Frame::Own, |first| first.frame().clone());
    let mut named: Vec<(crate::VarId, Arc<NameTable>, Vec<topo::GraftKeys>)> = Vec::new();
    let mut pieces: Vec<Piece<T>> = Vec::new();
    let mut coincidences: Vec<crate::coincide::NamedCoincidence> = Vec::new();
    let mut citing: Vec<crate::coincide::CitedInput> = Vec::new();
    // What each record the pieces carry in cites, in the graft's order.
    let mut cites: Vec<topo::Cites> = Vec::new();
    let mut kinds = Vec::new();
    let mut parts = 0;
    for (read, out) in outs {
        let (body, kind) = match &out.payload {
            ValuePayload::Boolean(BooleanValue::Empty) => continue,
            ValuePayload::Boolean(BooleanValue::Body { body, kind }) => (body.clone(), *kind),
            ValuePayload::Body(body) => (body.clone(), topo::BooleanResultKind::OperandA),
            _ => unreachable!("a boolean of one piece makes a body or nothing"),
        };
        let keys = crate::eval::Framed::graft_into(&frame, &mut fused, &body, tol)
            .map_err(NodeErrorKind::from)?;
        // What each of the piece's records cites, in this value's terms:
        // its own inputs at their places in the assembled list, its own
        // rows after the rows of the pieces before it.
        let base = coincidences.len();
        let inputs: Vec<u32> = out
            .carry
            .cited_inputs
            .iter()
            .map(|input| {
                let at = citing.iter().position(|c| c == input).unwrap_or_else(|| {
                    citing.push(input.clone());
                    citing.len() - 1
                });
                names::narrow(at, names::Numbered::InputPosition).map_err(NodeErrorKind::Naming)
            })
            .collect::<Result<_, _>>()?;
        let rows = (base..base + out.coincidences.len())
            .map(|row| names::narrow(row, names::Numbered::Row))
            .collect::<Result<Vec<u32>, _>>()
            .map_err(NodeErrorKind::Naming)?;
        let recited = out
            .carry
            .contacts(body.holding().records())
            .into_owned()
            .recited(|b| match b {
                topo::Backing::Decided(k) => vec![topo::Backing::Decided(
                    rows.get(k as usize).copied().unwrap_or_else(|| {
                        unreachable!(
                            "a piece's record cites its row {k}, though its boolean decided {}",
                            rows.len()
                        )
                    }),
                )],
                topo::Backing::Carried { input, record } => vec![topo::Backing::Carried {
                    input: inputs.get(input as usize).copied().unwrap_or_else(|| {
                        unreachable!(
                            "a piece's record cites its input {input}, which its boolean does \
                             not publish"
                        )
                    }),
                    record,
                }],
            })
            .unwrap_or_else(|k| unreachable!("record {k} cites nothing once renumbered"));
        cites.extend(recited.cites().cloned());
        coincidences.extend(out.coincidences.iter().cloned());
        let table = out
            .outputs
            .first()
            .map_or_else(names::empty, super::super::OutputNames::shared_table);
        parts += out.carry.parts;
        kinds.push(kind);
        pieces.push(Piece {
            read,
            bodies: vec![body],
            family: false,
            table: Arc::clone(&table),
            carry: out.carry.clone(),
        });
        named.push((read, table, vec![keys]));
    }
    let kind = match kinds.as_slice() {
        [] => {
            return Ok(OpOut::plain(
                ValuePayload::Boolean(BooleanValue::Empty),
                names::empty(),
            ));
        }
        [one] => *one,
        _ => topo::BooleanResultKind::Assembly,
    };
    let table = names::name_assemble(id, &named, fused.body()).map_err(NodeErrorKind::Naming)?;
    // The census's rows are numbered after the pieces' own, so a record
    // it mints names no piece's row.
    let first_row =
        names::narrow(coincidences.len(), names::Numbered::Row).map_err(NodeErrorKind::Naming)?;
    let fused = T::finish_assembled(fused.numbering_from(first_row), tol)
        .map_err(|findings| NodeErrorKind::NotFinished { findings })?;
    let topo::Censused {
        body: fused,
        rows,
        findings,
    } = fused;
    // The records the census minted follow the carried ones on the
    // body, citing as the gate wrote them.
    cites.extend(
        topo::FinishedBody::records(&fused)
            .cites()
            .skip(cites.len())
            .cloned(),
    );
    let fused = Arc::new(fused);
    let pieces = Arc::new(Pieces { by: id, pieces });
    // What it made is the `⊔` of what each piece's boolean made, so the
    // census of that `⊔` runs at this door, over the pieces made here
    // ([`crate::product::Census`]).
    let census = Some(Arc::new(crate::product::Census::new(
        id,
        Arc::clone(&pieces),
        Some(crate::product::Decided {
            body: crate::eval::Held::Finished(Arc::clone(&fused)),
            rows,
            findings,
        }),
    )));
    Ok(OpOut::plain(
        ValuePayload::Boolean(BooleanValue::Body {
            body: crate::eval::Framed::in_frame(fused, frame),
            kind,
        }),
        table,
    )
    .carrying(crate::eval::Carry {
        parts,
        pieces: Some(pieces),
        census,
        censused: true,
        citing: crate::eval::Citing::Each(cites.into()),
        cited_inputs: citing.into(),
        findings: Arc::default(),
        members: None,
    })
    .with_coincidences(coincidences))
}
