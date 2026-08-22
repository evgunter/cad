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
use super::role::SideVerdict;

/// The classification band (kernel-ambient tolerance) for the
/// `name_frag_*` family.
pub(crate) fn band(tol: Tol) -> Result<Band, NamingError> {
    Band::linear(tol).map_err(|_| NamingError::Emission {
        what: "name_frag band construction failed",
    })
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
/// oriented plane from `emit_topo::face_plane`, which applies
/// `topo::Face::sense_sign` once, at the read; this function
/// deliberately does not re-apply it (it is handed a plane, not a
/// face) and must not, or the two would cancel.
pub(crate) fn side_of_face<T: Decide>(
    body: &Body<T>,
    f: FaceKey,
    origin: Point3<T>,
    normal: Vec3<T>,
    b: Band,
) -> Result<SideVerdict, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let mut pos = false;
    let mut neg = false;
    let mut any = false;
    for he in face_half_edges(body, f)? {
        let v = body
            .get_half_edge(he)
            .ok_or_else(|| bug("side_of: dangling half-edge"))?
            .start;
        let p = *body
            .get_vertex(v)
            .and_then(|vd| body.get_point(vd.point))
            .ok_or_else(|| bug("side_of: vertex without point"))?;
        any = true;
        match decide("name_frag_side_of", Margin::of((p - origin).dot(normal)), b) {
            Ok(Sign::Positive) => pos = true,
            Ok(Sign::Negative) => neg = true,
            Ok(Sign::Zero) => {}
            Err(source) => {
                return Err(NamingError::Escalated {
                    predicate: "name_frag_side_of",
                    source,
                });
            }
        }
    }
    if !any {
        return Err(bug("side_of: face has no boundary vertices"));
    }
    Ok(match (pos, neg) {
        (true, true) => SideVerdict::Mixed,
        (true, false) => SideVerdict::Positive,
        (false, true) => SideVerdict::Negative,
        (false, false) => SideVerdict::On,
    })
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
    let mut before = vec![0u32; n]; // before[i] = #{j : j certified-before i}
    for i in 0..n {
        for j in (i + 1)..n {
            // i before j iff max(i) ≤ min(j) (certified positive gap);
            // Zero (touching extents) counts as ordered too.
            let ij = decide(
                "name_frag_order_along",
                Margin::of(extents[j].min - extents[i].max),
                b,
            );
            let ji = decide(
                "name_frag_order_along",
                Margin::of(extents[i].min - extents[j].max),
                b,
            );
            match (ij, ji) {
                (Ok(Sign::Positive | Sign::Zero), Ok(Sign::Negative)) => before[j] += 1,
                (Ok(Sign::Negative), Ok(Sign::Positive | Sign::Zero)) => before[i] += 1,
                (Err(source), _) | (_, Err(source)) => {
                    return Err(NamingError::Escalated {
                        predicate: "name_frag_order_along",
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
