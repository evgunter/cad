//! N2 discriminators (spec D3): the NAMED margined trilean predicates
//! (`name_frag_*` through `k_stats`) that rank fragments against
//! recipe-covariant references. Geometry enters as
//! VERDICTS only — no values, no bare indices, ever, in a name.
//! In-band margins escalate typed (never a silent pick); genuinely
//! undiscriminated siblings get the N2 tie mark at the table.

use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Sign, Tol};

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

/// The order-along predicate's name, written once: the ranker records
/// it and [`decision_words`] states what it decides.
pub(crate) const ORDER_ALONG: &str = "name_frag_order_along";

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

/// The on-seam-line predicate's name (`emit_union::rank_along_seam`):
/// whether a union seam's pieces, and the crossings of it a vertex group
/// holds, lie on one straight line.
///
/// **In the [`FAMILY`]**: its verdicts decide whether the group is
/// ranked along that line or tied, so they enter the names.
pub(crate) const ON_SEAM_LINE: &str = "name_frag_on_seam_line";

/// The chord-on-rim predicate's name (`emit_topo`'s `chord_on_rim`):
/// whether a boolean's chord between two merged faces lies within the
/// rim its key's side reads it through to.
///
/// **Outside the [`FAMILY`], and that is a choice with a cost.** The
/// family is the fragment QUALIFIER vocabulary: `resolve`'s diagnosis
/// ladder reads a `name_frag_` flip as the name's own discriminator
/// changing sign. This predicate is not a qualifier — its verdict
/// enters no name — so it stays out. The cost: when its flip is what
/// makes a chord's name vanish (the chord leaves its rim and the
/// emitter refuses), the ladder ranks that flip as a generic one rather
/// than as the name's own.
pub(crate) const CHORD_ON_RIM: &str = "name_chord_on_rim";

/// What one of the naming layer's decisions decides, in the words a
/// refusal or a diagnosis states in place of the predicate's name
/// (which is routing, kept to `Debug`): a clause with no colon or dash
/// of its own. `None` for a predicate this layer does not own.
pub(crate) fn decision_words(predicate: &str) -> Option<&'static str> {
    match predicate {
        ORDER_ALONG => Some("the order of two crossings along an edge"),
        ON_MEMBER_EDGE => Some("a point's place along an edge"),
        ON_SEAM_LINE => Some("whether a point lies on a seam's line"),
        CHORD_ON_RIM => Some("whether a chord lies on its rim"),
        _ => None,
    }
}

/// One candidate's extent along an oriented carrier: the certified
/// min/max of its probe parameters (values stay HERE — only the
/// resulting order enters names).
pub(crate) struct Extent<T> {
    /// Least probe parameter.
    pub min: T,
    /// Greatest probe parameter.
    pub max: T,
}

/// Whether extent `i` lies certainly before `j` along their carrier
/// (`Some(true)`), certainly after it (`Some(false)`), or neither:
/// `i` before `j` iff `max(i) ≤ min(j)`, decided through
/// `name_frag_order_along`. Touching extents count as ordered; two
/// that overlap, or two equal points, do not. In-band escalates typed.
pub(crate) fn extent_before<T: Decide>(
    i: &Extent<T>,
    j: &Extent<T>,
    b: Band,
) -> Result<Option<bool>, NamingError> {
    let ij = decide(ORDER_ALONG, Margin::of(j.min - i.max), b);
    let ji = decide(ORDER_ALONG, Margin::of(i.min - j.max), b);
    match (ij, ji) {
        (Ok(Sign::Positive | Sign::Zero), Ok(Sign::Negative)) => Ok(Some(true)),
        (Ok(Sign::Negative), Ok(Sign::Positive | Sign::Zero)) => Ok(Some(false)),
        (Err(source), _) | (_, Err(source)) => Err(NamingError::Escalated {
            predicate: ORDER_ALONG,
            source,
        }),
        _ => Ok(None),
    }
}

/// Ranks `n` candidates along their carrier by the pairwise order
/// `before(i, j)` (`i < j`) answers, [`extent_before`]'s or one built
/// on it: `rank[i]` (0-based), or `None` when some pair is unordered
/// (the N2 tie) or the pairs do not make one strict order.
pub(crate) fn rank_by(
    n: usize,
    mut before: impl FnMut(usize, usize) -> Result<Option<bool>, NamingError>,
) -> Result<Option<Vec<u32>>, NamingError> {
    // A rank is a `u32`, and every count below is at most `n − 1`.
    super::emit::to_u32(n, "a ranked group has more members than a rank holds")?;
    let mut ahead = vec![0u32; n]; // ahead[i] = #{j : j certified-before i}
    for i in 0..n {
        for j in (i + 1)..n {
            match before(i, j)? {
                Some(true) => ahead[j] += 1,
                Some(false) => ahead[i] += 1,
                None => return Ok(None),
            }
        }
    }
    // A consistent strict order yields distinct ranks 0..n.
    let mut seen = vec![false; n];
    for &r in &ahead {
        let ix = r as usize;
        if ix >= n || seen[ix] {
            return Ok(None);
        }
        seen[ix] = true;
    }
    Ok(Some(ahead))
}
