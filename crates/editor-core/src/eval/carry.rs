//! **What a body value carries from its source** (D10
//! `[placement.placement]`): a placement's copy "carries every pose of
//! the source as the source carries it, and the source's contact
//! records and census decisions, which stay the source's".
//!
//! [`Carry`] is that one bundle, on every body value, every piece of a
//! `⊔` and every part an instance copies: how the body's own records
//! cite at this value and what each input they cite is, the findings of
//! the documents below, the pieces and their door's census. The records
//! themselves are the body's ([`topo::FinishedBody::records`]). Every value
//! that carries its source's takes it through one door,
//! [`Carry::copied`]: a placement's copy, an instance's, each member of
//! a placed family, a pick of one piece, a union or an intersect of one
//! body, and a mirror's image, which is a construction of its own (D10
//! `[repetition]`) but carries as a copy carries. Each keeps every arena
//! key of its source (a placement shares its body,
//! [`super::Framed::moved`]; a reflection re-mints none,
//! `topo::mirror_body`), so what it carries is its source's, its records
//! citing their source as its one input, and its pieces standing where
//! it stands. A family read whole carries
//! each member's own ([`Carry::members`]): every member is evaluated on
//! its own, at its own index values, so its records are keyed in its
//! own arena, and each reader of a family's bodies reads member `i`'s
//! through [`Carry::member`].

use std::sync::Arc;

use geom_core::{Decide, RigidMotion};

use super::NodeErrorKind;
use crate::coincide::CitedInput;

/// **A body value's at-rest result**, as its copies carry it.
///
/// A family's (an `Instances` value's) is its members' together: their
/// findings, each once; their parts, summed; and each member's own
/// carry ([`Self::members`]), whose records are keyed in that member's
/// arena. It holds no records, pieces or census of its own.
#[derive(Debug, Clone)]
pub struct Carry<T: topo::AtRestPolicy + Decide> {
    /// **How its body's contact records cite, at this value**
    /// ([`Citing`]). The records are the body's
    /// ([`topo::FinishedBody::records`]); this holds none of their
    /// cells, only what each one's backing is in the value's terms
    /// ([`Self::contacts`]).
    pub citing: Citing,
    /// **What each input its records cite is**: a record's
    /// [`topo::Backing::Carried`] names an input by position, and this
    /// is that position's input ([`CitedInput`]); a
    /// [`topo::Backing::Decided`] names a row of the value's own
    /// coincidences ([`super::NodeValue::coincidences`]). It rides with
    /// the citing, so a carry taken whole keeps the list its records
    /// cite. A door that grafts a piece's boolean result reads it to
    /// renumber that result's carried citations into its own list
    /// (`wire::distribute`); every other reader re-cites a carry it
    /// copies ([`Self::copied`]).
    pub cited_inputs: Arc<[CitedInput]>,
    /// The at-rest findings of the documents below (`ASSEMBLY.md` A5),
    /// each with the route it came by: what an instance reports as its
    /// part's, keyed in the same arena as the body's records.
    pub findings: Arc<crate::checks::at_rest::CarriedFindings>,
    /// How many parts its body is (`crates/editor-core/ASSEMBLY.md`, A2
    /// and A10): a document's product is its pieces, so an instance's
    /// body is as many parts as the referenced document's piece outputs,
    /// each counted at its own `parts`; two piece outputs that are a
    /// split's two halves are two parts. Every value that carries its
    /// source's ([`Self::copied`]) carries the count through, a union or
    /// an intersect of one body among them; every other op builds a body
    /// of its own and counts 1, a union of several members included. A
    /// body of several parts is an ordinary operand (D10
    /// `[operations.operators]`: `(p ⊔ q) − t` cuts both pieces, and so
    /// does `∪(p ⊔ q) − t`).
    pub parts: usize,
    /// **The pieces its body holds** ([`super::Pieces`]): a `⊔`'s
    /// members, or those a copy carries from its source where it
    /// stands; `None` for a body of no pieces.
    pub pieces: Option<Arc<super::Pieces<T>>>,
    /// **The census at the door that assembled its pieces**
    /// ([`crate::product::Census`]): a `⊔`'s, or a cut's or a meet's
    /// that kept its operand's pieces; the one a copy carries from its
    /// source. Each door's census runs once, where a reader first asks
    /// for its verdict, and every reader reads that one.
    pub census: Option<Arc<crate::product::Census<T>>>,
    /// **Whether a door already decided the meetings inside its body**:
    /// a `⊔`'s value, an instance's copy (its part's census decided
    /// them), and any value that carries either's ([`Self::copied`]). A
    /// census reading such a body as one arrival discards what it finds
    /// inside it.
    pub censused: bool,
    /// **Each body's own carry, on a value of several bodies**: a
    /// family's members, or a split's two sides.
    ///
    /// - A family's: in member order, each member's records keyed in its
    ///   own arena, its findings, parts, pieces and census.
    /// - A split's: at its halves' output bodies, each side's records
    ///   keyed in its arena.
    ///
    /// `None` on a value of one body.
    pub members: Option<Arc<[Carry<T>]>>,
}

impl<T: topo::AtRestPolicy + Decide> Default for Carry<T> {
    /// A body of its own: no records, findings or pieces, one part.
    fn default() -> Self {
        Self {
            citing: Citing::Own,
            cited_inputs: Arc::new([]),
            findings: Arc::default(),
            parts: 1,
            pieces: None,
            census: None,
            censused: false,
            members: None,
        }
    }
}

impl<T: topo::AtRestPolicy + Decide> Carry<T> {
    /// **What a value that carries its source's carries** (D10
    /// `[placement.placement]`), the one door every copy's carry goes
    /// through, and a mirror's image's: this carry's records, each
    /// citing the record it is in `source`, its one input; its findings,
    /// parts, census and census decision, which stay the source's; its
    /// pieces where it `stands`; and each member's own carry, carried
    /// alike.
    ///
    /// # Errors
    ///
    /// [`NodeErrorKind::Motion`] where a piece's composed frame
    /// overflows.
    pub(crate) fn copied(
        &self,
        stands: Stands<'_, T>,
        source: &CitedInput,
    ) -> Result<Self, NodeErrorKind> {
        self.standing(stands, Some(source))
    }

    /// **This, standing as `stands` says**: its pieces and each
    /// member's with it; its records re-cited as carried from `source`
    /// where one is named, and kept as they are where none is (a
    /// piece's own, moved with its holder, which re-mints no key).
    fn standing(
        &self,
        stands: Stands<'_, T>,
        source: Option<&CitedInput>,
    ) -> Result<Self, NodeErrorKind> {
        let members = match &self.members {
            Some(members) => Some(
                members
                    .iter()
                    .map(|member| member.standing(stands, source))
                    .collect::<Result<Arc<[Self]>, NodeErrorKind>>()?,
            ),
            None => None,
        };
        let pieces = match stands {
            Stands::Still => self.pieces.clone(),
            Stands::Moved(map) => moved_pieces(self.pieces.as_ref(), map)?,
            Stands::Reflected => None,
        };
        let (citing, cited_inputs) = match source {
            Some(source) => (Citing::Copied, Arc::from([source.clone()])),
            None => (self.citing.clone(), Arc::clone(&self.cited_inputs)),
        };
        Ok(Self {
            citing,
            cited_inputs,
            pieces,
            members,
            ..self.clone()
        })
    }

    /// **Its body's contact records as this value cites them**: `held`,
    /// the records the value's body holds, each backed as [`Citing`]
    /// says.
    ///
    /// # Panics
    ///
    /// Where the value cites record by record ([`Citing::Each`]) and
    /// `held` is not the body the citations were written over: an
    /// evaluator bug.
    #[must_use]
    pub fn contacts<'h>(
        &self,
        held: &'h topo::ContactRecords,
    ) -> std::borrow::Cow<'h, topo::ContactRecords> {
        use std::borrow::Cow;
        match &self.citing {
            Citing::Own => Cow::Borrowed(held),
            Citing::Copied => Cow::Owned(held.carried_from(0)),
            Citing::Each(cites) => Cow::Owned(held.citing(cites).unwrap_or_else(|| {
                unreachable!(
                    "a value cites {} records and its body holds {}",
                    cites.len(),
                    held.len()
                )
            })),
        }
    }

    /// **This carry as a reader of its body reads it**: where its door's
    /// census names the inputs its body's records cite (each arrival, an
    /// instance by its part), those inputs; else this. The records are
    /// the body's either way.
    pub(crate) fn kept(
        &self,
        doc: &crate::doc::Doc<crate::program::ProfileProgram>,
        tol: geom_core::Tol,
    ) -> Self
    where
        T: topo::AtRestPolicy,
    {
        match self
            .census
            .as_ref()
            .and_then(|census| census.verdict(doc, tol).ok())
        {
            Some(verdict) => Self {
                citing: Citing::Own,
                cited_inputs: verdict.cited_inputs.clone().into(),
                ..self.clone()
            },
            None => self.clone(),
        }
    }

    /// This, its body's records citing as `citing` says, each input
    /// they cite `cited`.
    pub(crate) fn recording(self, citing: Citing, cited: Vec<CitedInput>) -> Self {
        Self {
            citing,
            cited_inputs: cited.into(),
            ..self
        }
    }

    /// **What a family whose members carry `members` carries**: their
    /// findings, each once (the members of a family of copies carry
    /// their source's, which it reports once, as the source's); their
    /// parts, summed; each member's carry as its own; and no records,
    /// pieces or census of its own. Its body is a door's census only
    /// where every member's is.
    pub(crate) fn of_family(members: Vec<Self>) -> Self {
        let mut findings = crate::checks::at_rest::CarriedFindings::default();
        for member in &members {
            let of = member.findings.as_ref();
            for row in &of.interference {
                if !findings.interference.contains(row) {
                    findings.interference.push(row.clone());
                }
            }
            for row in &of.at_rest {
                if !findings.at_rest.contains(row) {
                    findings.at_rest.push(row.clone());
                }
            }
            for row in &of.coincidences {
                if !findings.coincidences.contains(row) {
                    findings.coincidences.push(row.clone());
                }
            }
        }
        Self {
            citing: Citing::Own,
            cited_inputs: Arc::new([]),
            findings: Arc::new(findings),
            parts: members.iter().map(|member| member.parts).sum(),
            pieces: None,
            census: None,
            censused: !members.is_empty() && members.iter().all(|member| member.censused),
            members: Some(Arc::from(members)),
        }
    }

    /// **Body `i`'s carry** ([`Self::members`]): a family's member's
    /// own, a split's side's own, and this on a value of one body.
    pub fn member(&self, i: usize) -> &Self {
        self.members
            .as_ref()
            .and_then(|members| members.get(i))
            .unwrap_or(self)
    }
}

/// **How a value's body's contact records cite**, in the value's terms:
/// which row or which input's record backs each. It holds no cell of
/// any record; [`Carry::contacts`] reads the records off the body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Citing {
    /// As the body holds them: the value's door finished the body
    /// with these citations.
    Own,
    /// A copy's: record `k` cites record `k` of its source, its one
    /// input.
    Copied,
    /// Record `k` cites `.0[k]`: an op whose rows and inputs the
    /// evaluator names in the node's own space (a union's fold).
    Each(Arc<[topo::Cites]>),
}

impl Citing {
    /// Each record of `contacts` citing what it cites there.
    pub(crate) fn each(contacts: &topo::ContactRecords) -> Self {
        Self::Each(contacts.cites().cloned().collect())
    }
}

/// **Where a value stands against the source whose carry it carries.**
#[derive(Clone, Copy)]
pub(crate) enum Stands<'m, T: Decide> {
    /// Where its source stands: a pick of a piece, a union or an
    /// intersect of one body, a placement by the identity bundle.
    Still,
    /// Moved by a rigid map; its pieces stand where it stands.
    Moved(&'m RigidMotion<T>),
    /// Reflected across a plane, where no rigid motion stands a piece:
    /// a mirror's image, which holds none.
    Reflected,
}

impl<'m, T: Decide> Stands<'m, T> {
    /// Where a placement by `map` stands: [`Self::Still`] by no map.
    pub(crate) fn placed(map: Option<&'m RigidMotion<T>>) -> Self {
        map.map_or(Self::Still, Self::Moved)
    }
}

/// **Pieces moved by `map`** with their holder: every piece at every
/// depth stands where its holder stands, so `map` composes onto each
/// piece's frame. `None` where there are no pieces.
fn moved_pieces<T: topo::AtRestPolicy + Decide>(
    pieces: Option<&Arc<super::Pieces<T>>>,
    map: &RigidMotion<T>,
) -> Result<Option<Arc<super::Pieces<T>>>, NodeErrorKind> {
    let Some(pieces) = pieces else {
        return Ok(None);
    };
    let moved = pieces
        .pieces
        .iter()
        .map(|piece| {
            Ok(super::Piece {
                bodies: piece
                    .bodies
                    .iter()
                    .map(|body| body.moved(map).map_err(NodeErrorKind::Motion))
                    .collect::<Result<_, _>>()?,
                carry: piece.carry.standing(Stands::Moved(map), None)?,
                ..piece.clone()
            })
        })
        .collect::<Result<Vec<_>, NodeErrorKind>>()?;
    Ok(Some(Arc::new(super::Pieces {
        by: pieces.by,
        pieces: moved,
    })))
}
