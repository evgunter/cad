//! **What a body value carries from its source** (D10
//! `[placement.placement]`): a placement's copy "carries every pose of
//! the source as the source carries it, and the source's contact
//! records and census decisions, which stay the source's".
//!
//! [`Carry`] is that one bundle, on every body value, every piece of a
//! `⊔` and every part an instance copies: the records keyed in the
//! body's arena, the findings of the documents below, the pieces and
//! their door's census. A copy shares its source's body
//! ([`super::Framed::moved`]) and so keeps every key, so the one thing
//! a placement changes in what it carries is where the pieces stand
//! ([`Carry::moved`]). A placement's copy of one body and an instance's
//! take it through that one door (`wire::copy_of`). A family read
//! whole carries each member's own ([`Carry::members`]): every member
//! is evaluated on its own, at its own index values, so its records
//! are keyed in its own arena, and each reader of a family's bodies
//! reads member `i`'s through [`Carry::member`]. A projection of a
//! member and a pick of one piece carry their source's carry by hand,
//! and a mirror builds its own; one door for all of them is
//! `work/intent/a-mirror-of-an-assemble-reports-its-findings-twice.md`'s
//! cure.

use std::sync::Arc;

use geom_core::{Decide, RigidMotion};

use super::NodeErrorKind;

/// **A body value's at-rest result**, as its copies carry it.
///
/// A family's (an `Instances` value's) is its members' together: their
/// findings, each once; their parts, summed; and each member's own
/// carry ([`Self::members`]), whose records are keyed in that member's
/// arena. It holds no records, pieces or census of its own.
#[derive(Debug, Clone)]
pub struct Carry<T: Decide> {
    /// **The contact records its body carries**, keyed in its arena:
    /// declared across an instance seam, kept by its source's door, or
    /// its source's own. A boolean's own records ride its payload
    /// ([`super::BooleanValue::Body::contacts`]) and its copy's here.
    pub contacts: Arc<topo::ContactRecords>,
    /// The at-rest findings of the documents below (`ASSEMBLY.md` A5),
    /// each with the route it came by: what an instance reports as its
    /// part's, keyed in the same arena as `contacts`.
    pub findings: Arc<crate::checks::at_rest::CarriedFindings>,
    /// How many parts its body is (`crates/editor-core/ASSEMBLY.md`, A2
    /// and A10): a document's product is its pieces, so an instance's
    /// body is as many parts as the referenced document's piece outputs,
    /// each counted at its own `parts`; two piece outputs that are a
    /// split's two halves are two parts. The placers and `Part` carry the
    /// count through; every other op builds a body of its own and counts
    /// 1, an explicit union included. A body of several parts is an
    /// ordinary operand (D10 `[operations.operators]`: `(p ⊔ q) − t`
    /// cuts both pieces).
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
    /// them), and a copy of either. A census reading such a body as one
    /// arrival discards what it finds inside it.
    pub censused: bool,
    /// **A family's members' own carries**, in member order: each
    /// member's records keyed in its own arena, its findings, parts,
    /// pieces and census. `None` on a value that is no family.
    pub members: Option<Arc<[Carry<T>]>>,
}

impl<T: Decide> Default for Carry<T> {
    /// A body of its own: no records, findings or pieces, one part.
    fn default() -> Self {
        Self {
            contacts: Arc::default(),
            findings: Arc::default(),
            parts: 1,
            pieces: None,
            census: None,
            censused: false,
            members: None,
        }
    }
}

impl<T: Decide> Carry<T> {
    /// **What a copy under `map` carries**: this, its pieces standing
    /// where the copy stands. `None` is the identity bundle's placement
    /// by no map.
    ///
    /// # Errors
    ///
    /// [`NodeErrorKind::Motion`] where a piece's composed frame
    /// overflows.
    pub(crate) fn moved(&self, map: Option<&RigidMotion<T>>) -> Result<Self, NodeErrorKind> {
        let members = match &self.members {
            Some(members) => Some(
                members
                    .iter()
                    .map(|member| member.moved(map))
                    .collect::<Result<Arc<[Self]>, NodeErrorKind>>()?,
            ),
            None => None,
        };
        Ok(Self {
            pieces: moved_pieces(self.pieces.as_ref(), map)?,
            members,
            ..self.clone()
        })
    }

    /// This, with `contacts` as its records.
    pub(crate) fn recording(self, contacts: Arc<topo::ContactRecords>) -> Self {
        Self { contacts, ..self }
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
            contacts: Arc::default(),
            findings: Arc::new(findings),
            parts: members.iter().map(|member| member.parts).sum(),
            pieces: None,
            census: None,
            censused: !members.is_empty() && members.iter().all(|member| member.censused),
            members: Some(Arc::from(members)),
        }
    }

    /// **Member `i`'s carry**: a family's member's own, and this on a
    /// value that is no family.
    pub(crate) fn member(&self, i: usize) -> &Self {
        self.members
            .as_ref()
            .and_then(|members| members.get(i))
            .unwrap_or(self)
    }
}

/// **Pieces moved by `map`** with their holder: every piece at every
/// depth stands where its holder stands, so `map` composes onto each
/// piece's frame. `None` where there are no pieces.
fn moved_pieces<T: Decide>(
    pieces: Option<&Arc<super::Pieces<T>>>,
    map: Option<&RigidMotion<T>>,
) -> Result<Option<Arc<super::Pieces<T>>>, NodeErrorKind> {
    let Some(pieces) = pieces else {
        return Ok(None);
    };
    let Some(map) = map else {
        return Ok(Some(Arc::clone(pieces)));
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
                carry: piece.carry.moved(Some(map))?,
                ..piece.clone()
            })
        })
        .collect::<Result<Vec<_>, NodeErrorKind>>()?;
    Ok(Some(Arc::new(super::Pieces {
        by: pieces.by,
        pieces: moved,
    })))
}
