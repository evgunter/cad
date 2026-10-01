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

/// The on-member-edge predicate's name (`emit_union`'s member-edge
/// ranker): whether a vertex of a union's result lies on a member edge,
/// at which end, and whether two such vertices are one place.
///
/// **In the [`FAMILY`], unlike [`CHORD_ON_RIM`].** Its verdicts decide a
/// member-edge piece's `OrderAlong { rank, of }` — which places cut the
/// edge, so how many cells there are and which one a piece starts in —
/// so they enter the name, and a flip of one is an N2 discriminator
/// flip. `resolve`'s ladder reads a family flip on the path as the
/// name's own; a flip about a vertex elsewhere on the same member edge
/// does move that edge's count, so the reading holds for it too.
pub(crate) const ON_MEMBER_EDGE: &str = "name_frag_on_member_edge";

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
        ORDER_ALONG => Some("the order of two pieces along an edge"),
        ON_MEMBER_EDGE => Some("a point's place along an edge"),
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

/// Ranks `extents` along their carrier via the margined pairwise
/// order `name_frag_order_along` (margin: gap between extents).
/// Returns `rank[i]` (0-based); `None` when some pair is genuinely
/// unordered (overlapping extents — the N2 tie); in-band escalates
/// typed.
pub(crate) fn order_along<T: Decide>(
    extents: &[Extent<T>],
    b: Band,
) -> Result<Option<Vec<u32>>, NamingError> {
    let n = extents.len();
    // A rank is a `u32`, and every count below is at most `n − 1`.
    super::emit::to_u32(n, "a ranked group has more members than a rank holds")?;
    let mut before = vec![0u32; n]; // before[i] = #{j : j certified-before i}
    for i in 0..n {
        for j in (i + 1)..n {
            // i before j iff max(i) ≤ min(j) (certified positive gap);
            // Zero (touching extents) counts as ordered too.
            let ij = decide(ORDER_ALONG, Margin::of(extents[j].min - extents[i].max), b);
            let ji = decide(ORDER_ALONG, Margin::of(extents[i].min - extents[j].max), b);
            match (ij, ji) {
                (Ok(Sign::Positive | Sign::Zero), Ok(Sign::Negative)) => before[j] += 1,
                (Ok(Sign::Negative), Ok(Sign::Positive | Sign::Zero)) => before[i] += 1,
                (Err(source), _) | (_, Err(source)) => {
                    return Err(NamingError::Escalated {
                        predicate: ORDER_ALONG,
                        source,
                    });
                }
                // Overlapping or doubly-zero extents: unordered.
                _ => return Ok(None),
            }
        }
    }
    // A consistent strict order yields distinct ranks 0..n.
    let mut seen = vec![false; n];
    for &r in &before {
        let ix = r as usize;
        if ix >= n || seen[ix] {
            return Ok(None);
        }
        seen[ix] = true;
    }
    Ok(Some(before))
}
