//! **The n-ary union's naming** ([`crate::Node::Union`]; DM4).
//!
//! A union's names record WHICH MEMBER an entity came from and not
//! which fold step reached it. The fold's own tables are the pair
//! emitter's, so a member's entity accumulates a `FromA`/`FromB`
//! descent chain as long as its position in the list; that chain is
//! what this module takes out, in two halves.
//!
//! **Going in** ([`member_view`]): each member's table enters its fold
//! step already wrapped in one [`RoleSeg::FromMember`] carrying that
//! member's node id. Every operand of every step is therefore in the
//! union's own name space, and every name the pair emitter embeds — a
//! seam's two sides, a merge's constituents, a fragment's partners —
//! already says which member it came from. It has to be put there: a
//! pass-through op mints no name (N1), so N placements of one
//! prototype carry N IDENTICAL tables and no inner name can tell them
//! apart.
//!
//! **Coming out** ([`name_union`], and [`collapse_name`] for the
//! refusal paths): a fold-table name is rewritten by descending its
//! `FromA`/`FromB` chain to the [`RoleSeg::FromMember`] at its foot —
//! one wrapper, whatever the depth. `Seam`, `Merged` and `Fragment`
//! keep the shapes the pair emitter minted, with the names they embed
//! collapsed by the same rule.
//!
//! # How an intermediate row is told from a member's row
//!
//! By the HEAD segment alone. Every row of every fold step is minted
//! under the union's id, so the id separates nothing; what separates
//! them is that a member-keyed row's head is `FromMember` and an
//! intermediate row's is `FromA`/`FromB`, which is descended through.

use std::sync::Arc;

use crate::names::emit::{NamingError, check_total};
use crate::names::role::{Qualifier, RoleSeg, StableName};
use crate::names::table::{Entry, NameTable};
use crate::node::RecipeNodeId;

/// One member's table as the UNION sees it: every row's name wrapped
/// in one [`RoleSeg::FromMember`] naming this member, minted under the
/// union's node.
///
/// The entities are untouched — same body index, same keys — so the
/// view is a name rewrite and the pair emitter's reverse lookups read
/// it exactly as they read the member's own table.
pub(crate) fn member_view(
    union: RecipeNodeId,
    member: RecipeNodeId,
    table: &NameTable,
) -> Result<NameTable, NamingError> {
    let mut view = NameTable::new();
    for (name, entry) in table.iter() {
        let keyed = StableName {
            kind: name.kind,
            node: union,
            path: vec![RoleSeg::FromMember {
                member,
                of: Box::new(name.clone()),
            }],
        };
        match entry {
            Entry::Unique(e) => view.insert(keyed, *e),
            Entry::Tied(es) => view.insert_tied(keyed, es.clone()),
        }?;
    }
    Ok(view)
}

/// Rewrites the fold's final table into member-keyed names.
///
/// `folded` is the table the last fold step emitted (under `node`),
/// `body` the body it names. The rewrite is one-to-one on rows —
/// distinct entities keep distinct names, and a collision would be an
/// emission bug, refused typed by [`NameTable::insert`] rather than
/// aliased. [`check_total`] re-runs afterwards, so the rewritten table
/// is held to the same totality the fold's was.
pub(crate) fn name_union<T: geom_core::Decide>(
    node: RecipeNodeId,
    body: &topo::Body<T>,
    folded: &NameTable,
) -> Result<Arc<NameTable>, NamingError> {
    let mut t = NameTable::new();
    for (name, entry) in folded.iter() {
        let keyed = collapse(node, name)?;
        match entry {
            Entry::Unique(e) => t.insert(keyed, *e),
            Entry::Tied(es) => t.insert_tied(keyed, es.clone()),
        }?;
    }
    check_total(&t, body, 0)?;
    Ok(Arc::new(t))
}

/// One fold-table name in the union's published space.
///
/// The same rewrite [`name_union`] applies to a whole table, exposed
/// for the paths that carry a name out of a step that did NOT finish:
/// a refusal raised at step `k` reads the ACCUMULATED table, whose
/// rows are still `FromA`/`FromB`-headed, and a name in that shape is
/// in the fold's internal space — no published table holds it and
/// nothing can resolve it. Every name a union's refusal carries goes
/// through here first, so what a caller is handed denotes in the space
/// this node's own names live in.
pub(crate) fn collapse_name(
    node: RecipeNodeId,
    name: &StableName,
) -> Result<StableName, NamingError> {
    collapse(node, name)
}

/// The emission bug this module can raise: a fold table carrying a
/// segment the pair emitter does not mint.
const FOREIGN: &str = "a union fold's table carries a segment the boolean emitter does not mint";

/// One fold-table name, keyed by member.
///
/// Returns a name in the UNION's own space (`node` is the union's, as
/// every fold row's already is): the head segment is
/// [`RoleSeg::FromMember`], [`RoleSeg::Seam`], [`RoleSeg::Merged`] or
/// [`RoleSeg::OutputBody`], followed by the `Fragment` discriminators
/// the fold accumulated, outermost step last.
fn collapse(node: RecipeNodeId, name: &StableName) -> Result<StableName, NamingError> {
    let bug = |what| NamingError::Emission { what };
    if name.node != node {
        return Err(bug(
            "a union fold's table carries a row minted by another node",
        ));
    }
    let Some((head, tail)) = name.path.split_first() else {
        return Err(bug("a union fold's table carries a name with no role"));
    };
    let mut path = match head {
        // Already member-keyed: the foot of a descent chain, put there
        // by `member_view` before the step ran.
        RoleSeg::FromMember { .. } => vec![head.clone()],
        // The accumulated body's own name at every step, and the
        // union's at the last one: one body out, one output-body row.
        RoleSeg::OutputBody => vec![RoleSeg::OutputBody],
        // The descent. Every operand of every step is in this node's
        // space, so a `FromA`/`FromB` argument is always an earlier
        // step's row: descended THROUGH, carrying its own
        // discriminators out with it.
        RoleSeg::FromA(inner) | RoleSeg::FromB(inner) => collapse(node, inner)?.path,
        // A seam between two members. The pair emitter's `a`/`b` are
        // the crossing entities in the two OPERANDS' tables, which are
        // this node's space on both sides, so each is collapsed the
        // same way every other row is. The pair is then CANONICALIZED
        // by name order: a union is commutative, so "which side" would
        // record only which of the two members the fold reached first,
        // which is the position this node exists not to record.
        RoleSeg::Seam { a, b } => {
            let (x, y) = (collapse(node, a)?, collapse(node, b)?);
            let (a, b) = if x <= y { (x, y) } else { (y, x) };
            vec![RoleSeg::Seam {
                a: Box::new(a),
                b: Box::new(b),
            }]
        }
        // An F7 merged face: its constituents are result-face names in
        // the minting node's space (N3), so they stay in this union's
        // space, each collapsed by this same rule.
        //
        // UNREACHABLE as the fold is built today, and stated so rather
        // than left to look exercised: the pair emitter mints `Merged`
        // only for a DECLARED contact's merge groups, and a union
        // carries no declaration channel, so every step runs with
        // `BooleanDeclarations::none()`. No row in this suite reaches
        // this arm. The channel is a live design question
        // (`work/docm/n-ary-union-has-no-declaration-channel`); the arm
        // is written because the rule it states is the one every other
        // embedded-name arm here states, so leaving it out would make
        // the descent partial for a reason that is not a design one.
        //
        // The sort-and-dedup makes the constituent SET the name, the
        // same choice the pair emitter's twin makes (`emit_topo.rs`,
        // review R8): two merge groups collapsing to ONE constituent
        // set collide LOUDLY at insert (`DuplicateName` → typed
        // `NamingError`), never silently aliasing two faces onto one
        // name.
        RoleSeg::Merged(constituents) => {
            let mut set = constituents
                .iter()
                .map(|c| collapse(node, c))
                .collect::<Result<Vec<_>, _>>()?;
            set.sort();
            set.dedup();
            vec![RoleSeg::Merged(set)]
        }
        // Everything else is a segment the boolean emitter does not
        // mint, so a fold table carrying one is an emission bug. Named
        // one by one rather than caught by a wildcard, so a new
        // `RoleSeg` stops the compiler here and is decided, instead of
        // silently joining this list.
        RoleSeg::Fragment(_)
        | RoleSeg::Cap(_)
        | RoleSeg::Lateral(_)
        | RoleSeg::RimEdge(_, _)
        | RoleSeg::LateralEdge(_)
        | RoleSeg::CapVertex(_, _)
        | RoleSeg::Band(_)
        | RoleSeg::BandRim(_)
        | RoleSeg::BandRimPi(_)
        | RoleSeg::BandPi(_)
        | RoleSeg::Meridian(_, _)
        | RoleSeg::MeridianVertex(_, _)
        | RoleSeg::RevolveCap(_)
        | RoleSeg::Pole(_)
        | RoleSeg::AxisEdge(_)
        | RoleSeg::SplitBody(_)
        | RoleSeg::SectionFace { .. }
        | RoleSeg::SectionEdge { .. }
        | RoleSeg::SplitFragment { .. }
        | RoleSeg::CrossingVertex { .. }
        | RoleSeg::OnToolVertex { .. }
        | RoleSeg::FromTarget(_)
        | RoleSeg::BlendFace(_)
        | RoleSeg::CornerFace(_)
        | RoleSeg::TrimEdge { .. }
        | RoleSeg::FootVertex { .. }
        | RoleSeg::CornerArc { .. }
        | RoleSeg::BandFace(_)
        | RoleSeg::BandTrim { .. }
        | RoleSeg::BandFoot(_)
        | RoleSeg::BandCross(_)
        | RoleSeg::BandCut(_)
        | RoleSeg::BandSlit(_)
        | RoleSeg::InPart { .. }
        | RoleSeg::Instance { .. } => return Err(bug(FOREIGN)),
    };
    for seg in tail {
        path.push(match seg {
            // The discriminator's partner names are OPERAND-space
            // names (N2) — this node's space, like every other embedded
            // name here — so they collapse the same way. The verdicts
            // are untouched: they are the recorded predicate evidence,
            // not a reference.
            RoleSeg::Fragment(Qualifier::SideOf(vec)) => {
                let partners = vec
                    .iter()
                    .map(|(n, v)| Ok((collapse(node, n)?, *v)))
                    .collect::<Result<Vec<_>, NamingError>>()?;
                RoleSeg::Fragment(Qualifier::SideOf(partners))
            }
            RoleSeg::Fragment(q @ Qualifier::OrderAlong { .. }) => RoleSeg::Fragment(q.clone()),
            // Only a `Fragment` follows a head segment in a boolean
            // table; anything else in the tail is an emission bug.
            // Spelled out for the same reason the head match is.
            RoleSeg::OutputBody
            | RoleSeg::FromA(_)
            | RoleSeg::FromB(_)
            | RoleSeg::FromMember { .. }
            | RoleSeg::Seam { .. }
            | RoleSeg::Merged(_)
            | RoleSeg::Cap(_)
            | RoleSeg::Lateral(_)
            | RoleSeg::RimEdge(_, _)
            | RoleSeg::LateralEdge(_)
            | RoleSeg::CapVertex(_, _)
            | RoleSeg::Band(_)
            | RoleSeg::BandRim(_)
            | RoleSeg::BandRimPi(_)
            | RoleSeg::BandPi(_)
            | RoleSeg::Meridian(_, _)
            | RoleSeg::MeridianVertex(_, _)
            | RoleSeg::RevolveCap(_)
            | RoleSeg::Pole(_)
            | RoleSeg::AxisEdge(_)
            | RoleSeg::SplitBody(_)
            | RoleSeg::SectionFace { .. }
            | RoleSeg::SectionEdge { .. }
            | RoleSeg::SplitFragment { .. }
            | RoleSeg::CrossingVertex { .. }
            | RoleSeg::OnToolVertex { .. }
            | RoleSeg::FromTarget(_)
            | RoleSeg::BlendFace(_)
            | RoleSeg::CornerFace(_)
            | RoleSeg::TrimEdge { .. }
            | RoleSeg::FootVertex { .. }
            | RoleSeg::CornerArc { .. }
            | RoleSeg::BandFace(_)
            | RoleSeg::BandTrim { .. }
            | RoleSeg::BandFoot(_)
            | RoleSeg::BandCross(_)
            | RoleSeg::BandCut(_)
            | RoleSeg::BandSlit(_)
            | RoleSeg::InPart { .. }
            | RoleSeg::Instance { .. } => return Err(bug(FOREIGN)),
        });
    }
    Ok(StableName {
        kind: name.kind,
        node,
        path,
    })
}
