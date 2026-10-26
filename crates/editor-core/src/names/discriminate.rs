//! N2 discriminators (spec D3): the NAMED margined trilean predicates
//! (`name_frag_*` through `k_stats`) that tell fragments apart against
//! recipe-covariant references. Geometry enters as
//! VERDICTS only — no values, no bare indices, ever, in a name.
//! In-band margins escalate typed (never a silent pick); genuinely
//! undiscriminated siblings get the N2 tie mark at the table.

use geom_core::{Band, Tol};

use super::emit::NamingError;

/// The classification band (kernel-ambient tolerance) for the
/// `name_frag_*` family.
pub(crate) fn band(tol: Tol) -> Result<Band, NamingError> {
    Ok(Band::linear(tol)?)
}

/// The N2 discriminator family's predicate-name prefix. Every name
/// this module hands the funnel starts with it, and consumers that
/// ask "is this predicate the fragment's OWN qualifier vocabulary"
/// — `resolve`'s diagnosis ladder — ask it with this.
pub(crate) const FAMILY: &str = "name_frag_";

/// The on-member-edge predicate's name (`emit_union`'s `Flush`): whether
/// a point of a union's result lies on a member edge or at a member
/// vertex, and at which end.
///
/// **In the [`FAMILY`], unlike [`CHORD_ON_RIM`].** Its verdicts decide
/// which member edge a finished edge is a piece of, and which member
/// vertex or edge a finished vertex is named for, so they enter the
/// name, and a flip of one is an N2 discriminator flip. `resolve`'s
/// ladder reads a family flip on the path as the name's own.
pub(crate) const ON_MEMBER_EDGE: &str = "name_frag_on_member_edge";

/// The chord-on-rim predicate's name (`emit_topo`'s `chord_within`):
/// whether a boolean's chord between two faces of one operand lies
/// within an edge those faces share — the rim a merged pair's key side
/// reads it through to, an unmerged pair's one shared rim, or, where
/// the pair shares several edges, each of them in turn
/// (`rim_holding`). On a straight rim it places the chord's ends on the
/// rim's segment; on a curved one it orders them along the carrier
/// (`Track::cover`).
///
/// **Outside the [`FAMILY`], and that is a choice with a cost.** The
/// family is the fragment QUALIFIER vocabulary: `resolve`'s diagnosis
/// ladder reads a `name_frag_` flip as the name's own discriminator
/// changing sign. This predicate is not a qualifier — its verdict
/// picks which shared edge a chord descends to, and adds nothing to
/// the name's qualifiers — so it stays out. The cost: when its flip is
/// what makes a chord's name vanish or move (the chord leaves its rim
/// and the emitter refuses, or lands in another shared edge), the
/// ladder ranks that flip as a generic one rather than as the name's
/// own.
pub(crate) const CHORD_ON_RIM: &str = "name_chord_on_rim";

/// What each of the naming layer's decisions decides, in the words a
/// refusal or a diagnosis states in place of the predicate's name
/// (which is routing, kept to `Debug`): a clause with no colon or dash
/// of its own.
pub(crate) const DECISION_WORDS: &[(&str, &str)] = &[
    (ON_MEMBER_EDGE, "a point's place along an edge"),
    (CHORD_ON_RIM, "whether a chord lies on its rim"),
    (
        crate::pose::SEL_PLANE_DISTANCE,
        "which side of a selection's stated distance from a plane a point lies on",
    ),
];
