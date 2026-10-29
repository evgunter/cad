//! **What a body value carries from its source** (D10
//! `[placement.placement]`): a placement's copy "carries every pose of
//! the source as the source carries it, and the source's contact
//! records and census decisions, which stay the source's".
//!
//! [`Carry`] is that one bundle, on every body value, every piece of a
//! `⊔` and every part an instance copies: the records keyed in the
//! body's arena and what each input they cite is, the findings of the
//! documents below, the pieces and their door's census. Every copy
//! takes its carry through one door, [`Carry::copied`]: a placement's
//! copy, an instance's, each member of a placed family, a pick of one
//! piece and a mirror's image. A copy keeps every arena key of its
//! source (a placement shares its body, [`super::Framed::moved`]; a
//! reflection re-mints none, `topo::mirror_body`), so what it carries
//! is its source's, its records citing their source as its one input,
//! and its pieces standing where it stands. A family read whole carries
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
pub struct Carry<T: Decide> {
    /// **The contact records its body carries**, keyed in its arena:
    /// a boolean's own, declared across an instance seam, kept by its
    /// source's door, or its source's own.
    pub contacts: Arc<topo::ContactRecords>,
    /// **What each input its records cite is**: a record's
    /// [`topo::Backing::Carried`] names an input by position, and this
    /// is that position's input ([`CitedInput`]); a
    /// [`topo::Backing::Decided`] names a row of the value's own
    /// coincidences ([`super::NodeValue::coincidences`]). It rides with
    /// the records, so a carry taken whole never cites through a list
    /// it did not come with.
    pub cited_inputs: Arc<[CitedInput]>,
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

impl<T: Decide> Carry<T> {
    /// **What a copy carries** (D10 `[placement.placement]`), the one
    /// door every copy's carry goes through: this carry's records, each
    /// citing the record it is in `source`, the copy's one input; its
    /// findings, parts, census and census decision, which stay the
    /// source's; its pieces where the copy `stands`; and each member's
    /// own carry, copied alike.
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
        let members = match &self.members {
            Some(members) => Some(
                members
                    .iter()
                    .map(|member| member.copied(stands, source))
                    .collect::<Result<Arc<[Self]>, NodeErrorKind>>()?,
            ),
            None => None,
        };
        let pieces = match stands {
            Stands::Moved(map) => moved_pieces(self.pieces.as_ref(), map)?,
            Stands::Reflected => None,
        };
        Ok(Self {
            contacts: Arc::new(self.contacts.carried_from(0)),
            cited_inputs: Arc::new([source.clone()]),
            pieces,
            members,
            ..self.clone()
        })
    }

    /// **This, standing where `map` moves it**: its pieces and each
    /// member's moved with it, and nothing else changed, since a rigid
    /// motion re-mints no key.
    fn moved(&self, map: &RigidMotion<T>) -> Result<Self, NodeErrorKind> {
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
            pieces: moved_pieces(self.pieces.as_ref(), Some(map))?,
            members,
            ..self.clone()
        })
    }

    /// This, its records `contacts`, each input they cite `cited`.
    pub(crate) fn recording(
        self,
        contacts: Arc<topo::ContactRecords>,
        cited: Vec<CitedInput>,
    ) -> Self {
        Self {
            contacts,
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
            contacts: Arc::default(),
            cited_inputs: Arc::new([]),
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

/// **Where a copy stands against its source**: moved by a rigid map
/// (`None` the identity bundle's placement by no map), or reflected
/// across a plane, where no rigid motion stands a piece.
#[derive(Clone, Copy)]
pub(crate) enum Stands<'m, T: Decide> {
    /// Placed by a rigid map; its pieces stand where it stands.
    Moved(Option<&'m RigidMotion<T>>),
    /// A mirror's image, which holds no pieces.
    Reflected,
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
                carry: piece.carry.moved(map)?,
                ..piece.clone()
            })
        })
        .collect::<Result<Vec<_>, NodeErrorKind>>()?;
    Ok(Some(Arc::new(super::Pieces {
        by: pieces.by,
        pieces: moved,
    })))
}
