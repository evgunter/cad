//! **`−` and `∩` distribute over `⊔`** (D10 `[operations.operators]`):
//! a `⊔` value is its pieces, so a boolean reading one acts on each
//! piece and assembles what it made, `(p ⊔ q) − t = (p − t) ⊔ (q − t)`
//! and `(p ⊔ q) ∩ t = (p ∩ t) ⊔ (q ∩ t)`. A meeting inside the `⊔` is
//! its own decision and no boolean above it decides it again; this is
//! also the kernel's own recourse for a cut through two pieces that
//! touch (`topo::boolean`'s `VertexReadTwice`, "apply to each piece and
//! combine").
//!
//! Each piece's result keeps that piece's records and is keyed by the
//! piece's read, as the `⊔` keyed it ([`names::name_assemble`]); a
//! piece the boolean empties drops out, and a result with no piece left
//! is the typed empty result. The rows a piece's boolean decided name
//! its cells as the node's operand spells them, through the read the
//! node took it in by, so the coincidence door reads them where the
//! operand stands.
//!
//! A `⊔` read as the TOOL of a cut is cut by whole
//! (`work/cleave/a-vertex-piercing-two-declared-coincident-faces-reads-as-one.md`).

use std::sync::Arc;

use geom_core::{Decide, Tol};
use topo::Body;

use super::{BooleanValue, NodeErrorKind, OpOut, OpResult, ValuePayload};
use crate::eval::{Piece, Pieces};
use crate::names::{self, NameTable};
use crate::node::RecipeNodeId;

/// **A boolean's operand, as a distribution reads it**: a body read
/// through a seat, or one piece of one, with its names, records and
/// pieces.
pub(super) struct Operand<T: Decide> {
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

    /// Its body, finished in its own frame: the gate every boolean
    /// operand passes.
    pub(super) fn finished(&self, tol: Tol) -> Result<topo::AtRestBody<T>, NodeErrorKind> {
        self.gated(self.body.body().clone(), tol)
    }

    /// **Its body read into `into`'s frame, finished**: `own`, its
    /// finished body in its own frame, where the two are one frame, so
    /// an operand read twice is gated once.
    pub(super) fn finished_in(
        &self,
        into: &crate::eval::Framed<T>,
        own: &topo::AtRestBody<T>,
        tol: Tol,
    ) -> Result<topo::AtRestBody<T>, NodeErrorKind> {
        if into
            .relative(&self.body)
            .map_err(NodeErrorKind::Motion)?
            .is_none()
        {
            return Ok(own.clone());
        }
        let read = into.read_in(&self.body, tol)?;
        self.gated((*read).clone(), tol)
    }

    fn gated(&self, body: Body<T>, tol: Tol) -> Result<topo::AtRestBody<T>, NodeErrorKind> {
        T::gate_at_rest_kept(body, tol).map_err(|errors| NodeErrorKind::UnfinishedOperand {
            input: self.node,
            errors,
        })
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
                    let index = u32::try_from(j).unwrap_or(u32::MAX);
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
/// records; the records' citations renumbered onto the assembled rows
/// and inputs. Nothing left is the typed empty result.
pub(super) fn assembled<T: Decide + topo::AtRestPolicy>(
    id: RecipeNodeId,
    outs: Vec<(crate::VarId, OpOut<T>)>,
    tol: Tol,
) -> OpResult<T> {
    let mut fused = Body::new();
    // Computed in the first result's frame that is its space's, else the
    // first's ([`crate::eval::Framed::computing_among`]), each other read
    // into it.
    let mut computing: Option<crate::eval::Framed<T>> = crate::eval::Framed::computing_among(
        outs.iter().filter_map(|(_, out)| match &out.payload {
            ValuePayload::Boolean(BooleanValue::Body { body, .. }) | ValuePayload::Body(body) => {
                Some(body)
            }
            _ => None,
        }),
    )
    .cloned();
    let mut named: Vec<(crate::VarId, Arc<NameTable>, Vec<topo::GraftKeys>)> = Vec::new();
    let mut pieces: Vec<Piece<T>> = Vec::new();
    let mut contacts = topo::ContactRecords::default();
    let mut coincidences: Vec<crate::coincide::NamedCoincidence> = Vec::new();
    let mut citing: Vec<crate::coincide::CitedInput> = Vec::new();
    let mut kinds = Vec::new();
    let mut parts = 0;
    for (read, out) in outs {
        let records = &out.carry.contacts;
        let (body, kind) = match &out.payload {
            ValuePayload::Boolean(BooleanValue::Empty) => continue,
            ValuePayload::Boolean(BooleanValue::Body { body, kind }) => (body.clone(), *kind),
            ValuePayload::Body(body) => (body.clone(), topo::BooleanResultKind::OperandA),
            _ => unreachable!("a boolean of one piece makes a body or nothing"),
        };
        let first = computing.get_or_insert_with(|| body.clone());
        let read_in = first.read_in(&body, tol)?;
        let keys = topo::graft_disjoint_all_keyed(&mut fused, read_in.as_ref())
            .map_err(NodeErrorKind::Boolean)?;
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
                u32::try_from(at).unwrap_or(u32::MAX)
            })
            .collect();
        let recited = (**records)
            .clone()
            .recited(|b| match b {
                topo::Backing::Decided(k) => vec![topo::Backing::Decided(
                    u32::try_from(base + k as usize).unwrap_or(u32::MAX),
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
        crate::product::carry_contacts(&fused, &mut contacts, &recited, &keys)
            .map_err(|what| NodeErrorKind::Naming(names::NamingError::Emission { what }))?;
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
    let table = names::name_assemble(id, &named, &fused).map_err(NodeErrorKind::Naming)?;
    let frame = computing.map_or(crate::eval::Frame::Own, |first| first.frame().clone());
    let pieces = Arc::new(Pieces { by: id, pieces });
    // What it made is the `⊔` of what each piece's boolean made, so the
    // census of that `⊔` runs at this door, over the pieces made here
    // ([`crate::product::Census`]).
    let census = Some(Arc::new(crate::product::Census::new(
        id,
        Arc::clone(&pieces),
    )));
    Ok(OpOut::plain(
        ValuePayload::Boolean(BooleanValue::Body {
            body: crate::eval::Framed::in_frame(Arc::new(fused), frame),
            kind,
        }),
        table,
    )
    .carrying(crate::eval::Carry {
        parts,
        pieces: Some(pieces),
        census,
        censused: true,
        contacts: Arc::new(contacts),
        cited_inputs: citing.into(),
        findings: Arc::default(),
        members: None,
    })
    .with_coincidences(coincidences))
}
