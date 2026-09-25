//! N2 discriminators (spec D3): fragment qualifiers as sign vectors
//! of NAMED margined trilean predicates (`name_frag_*` through
//! `k_stats`) against recipe-covariant references. Geometry enters as
//! VERDICTS only — no values, no bare indices, ever, in a name.
//! In-band margins escalate typed (never a silent pick); genuinely
//! undiscriminated siblings get the N2 tie mark at the table.

use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Sign, Tol, Vec3};
use topo::{Body, FaceKey};

use super::emit::{NamingError, face_half_edges};
use super::emit_topo::face_plane;
use super::role::SideVerdict;

/// The classification band (kernel-ambient tolerance) for the
/// `name_frag_*` family.
pub(crate) fn band(tol: Tol) -> Result<Band, NamingError> {
    Ok(Band::linear(tol)?)
}

/// Aggregated side-of verdict of face `f` against the oriented plane
/// `(origin, normal)` — the partner's own carrier (recipe-covariant).
/// Per vertex: `name_frag_side_of` (margin in meters); `Zero`
/// verdicts (on-carrier seam vertices) are neutral; in-band
/// escalates typed.
///
/// **`normal` must be the partner face's OUTWARD normal** (S10
/// category A). The verdict IS the sign of `(p − origin) · normal`, so
/// the reference's orientation is not a convenience here — it is the
/// entire discriminator, and it goes straight into a stable name as
/// `Qualifier::SideOf`. Negating it swaps every Positive for a
/// Negative and renames fragments that never moved. Callers obtain the
/// oriented plane from `emit_topo::face_plane`, which folds
/// `topo::Face::sense` in once, at the read; this function
/// deliberately does not fold it again (it is handed a plane, not a
/// face) and must not, or the two would cancel.
pub(crate) fn side_of_face<T: Decide>(
    body: &Body<T>,
    f: FaceKey,
    origin: Point3<T>,
    normal: Vec3<T>,
    b: Band,
) -> Result<SideVerdict, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let mut signs = Vec::new();
    for he in face_half_edges(body, f)? {
        let v = body
            .get_half_edge(he)
            .ok_or_else(|| bug("side_of: dangling half-edge"))?
            .start;
        let p = super::emit::vertex_point(body, v)?;
        match decide(SIDE_OF, Margin::of((p - origin).dot(normal)), b) {
            Ok(sign) => signs.push(sign),
            Err(source) => {
                return Err(NamingError::Escalated {
                    predicate: SIDE_OF,
                    source,
                });
            }
        }
    }
    aggregate_side(&signs).ok_or_else(|| bug("side_of: face has no boundary vertices"))
}

/// **THE rule that turns a face's per-vertex `name_frag_side_of`
/// stream into the [`SideVerdict`] a `Qualifier::SideOf` entry
/// records.** `None` when the stream is empty — a face with no
/// boundary vertices, which is a kernel bug at the emission and an
/// absent answer here.
///
/// Written once and called twice, deliberately: [`side_of_face`] calls
/// it when the emission mints the qualifier, and
/// `resolve`'s shadow-execution rung calls it when it
/// re-derives that same qualifier at diagnosis time. A second spelling
/// would let the two disagree about what a fragment's side IS, and the
/// rung's whole claim is that it recovers the verdict the emission
/// would have recorded.
///
/// `Zero` verdicts are neutral (on-carrier seam vertices): they push
/// neither side, so a face every probe decides `Zero` lies IN the
/// carrier and is [`SideVerdict::On`].
pub(crate) fn aggregate_side(signs: &[Sign]) -> Option<SideVerdict> {
    if signs.is_empty() {
        return None;
    }
    let pos = signs.contains(&Sign::Positive);
    let neg = signs.contains(&Sign::Negative);
    Some(match (pos, neg) {
        (true, true) => SideVerdict::Mixed,
        (true, false) => SideVerdict::Positive,
        (false, true) => SideVerdict::Negative,
        (false, false) => SideVerdict::On,
    })
}

/// Re-runs ONE (fragment, partner) pair's `name_frag_side_of` probes
/// **outside every log**, and hands back the signs they decided, in
/// decision order.
///
/// This is [`side_of_face`] with two differences and no third. The
/// probes run under [`geom_core::k_stats::detached`], whose recording
/// is READ here and never spliced, so nothing this function decides
/// can reach a node's verdict log, an escalation log or a margin sink
/// — the caller is a DIAGNOSIS, and a diagnosis that wrote to the
/// substrate it diagnoses would be diffing its own footprints. And the
/// answer is the per-vertex SIGN STREAM rather than the aggregated
/// [`SideVerdict`], because the caller needs both: the stream for the
/// population diff and, through [`aggregate_side`], the verdict.
///
/// `partner` is a face of `partner_body`; its outward-oriented carrier
/// is read through the same [`face_plane`] door the emission reads it
/// through, so a caller cannot orient the reference differently from
/// the run it is reconstructing (the orientation IS the discriminator
/// — [`side_of_face`]'s docs).
///
/// **An escalation refuses rather than truncating.** The funnel
/// records the definite verdicts made before an in-band margin and
/// then hands back the escalation, so a caller COULD aggregate the
/// prefix — and it would be aggregating a face's side from some of its
/// vertices, which is a fabricated verdict, not a partial one. In the
/// emission the same escalation fails the node outright; here it
/// refuses typed and the rung says so.
pub(crate) fn shadow_side_of<T: Decide>(
    body: &Body<T>,
    fragment: FaceKey,
    partner_body: &Body<T>,
    partner: FaceKey,
    tol: Tol,
) -> Result<Vec<Sign>, NamingError> {
    let b = band(tol)?;
    let (out, recording) = geom_core::k_stats::detached(|| {
        let (origin, normal) = face_plane(partner_body, partner)?;
        side_of_face(body, fragment, origin, normal, b)
    });
    out?;
    Ok(recording
        .recorded()
        .verdicts
        .iter()
        .filter(|v| v.predicate == SIDE_OF)
        .map(|v| v.sign)
        .collect())
}

/// The N2 discriminator family's predicate-name prefix. Every name
/// this module hands the funnel starts with it, and consumers that
/// ask "is this predicate the fragment's OWN qualifier vocabulary"
/// — `resolve`'s diagnosis ladder — ask it with this.
pub(crate) const FAMILY: &str = "name_frag_";

/// The side-of predicate's name, written once: the emission records it
/// and the shadow probe filters its recording by it, so the two cannot
/// name different predicates.
pub(crate) const SIDE_OF: &str = "name_frag_side_of";

/// The order-along predicate's name, written once for the same reason
/// [`SIDE_OF`] is. Its consumers are all in this module today; the
/// constant exists so the family has one home rather than one and a
/// half.
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
