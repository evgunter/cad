//! **Fusing two coincident vertices of one face**: the zero-length
//! joint the boolean's zips and partings and the split's pinch cones
//! ([`crate::splitting`]'s `section_cones`) collapse two vertices on one
//! point key with.

use geom_core::{Decide, Tol};

use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, VertexKey};
use crate::euler::{EulerOpError, FaceSurface, MefSite};
use crate::euler_ring::MekrSite;
use geom_brep::EdgeCurveSpec;

/// Where a zero-length joint runs between two vertices of one face.
pub(crate) enum Joint {
    /// Across two of its loops (`mekr`, which joins them): `target`'s
    /// loop absorbs `ring`'s.
    Loops {
        target: HalfEdgeKey,
        ring: HalfEdgeKey,
    },
    /// Across one loop (`mef`, which divides the face).
    Chord { he1: HalfEdgeKey, he2: HalfEdgeKey },
}

impl Joint {
    /// **The joint from `he1`'s start to `he2`'s**, two half-edges of one
    /// face: across their one loop, or across their two, the face's
    /// outline absorbing the other where `he2` lies on it (`mekr`
    /// absorbs a ring into the other loop; the joint reads the same
    /// either way round). `None` where a half-edge resolves no loop or
    /// face.
    pub(crate) fn between<T: geom_core::Real>(
        body: &Body<T>,
        he1: HalfEdgeKey,
        he2: HalfEdgeKey,
    ) -> Option<Self> {
        let loop_of = |h: HalfEdgeKey| body.get_half_edge(h).map(|d| d.parent_loop);
        let (l1, l2) = (loop_of(he1)?, loop_of(he2)?);
        if l1 == l2 {
            return Some(Self::Chord { he1, he2 });
        }
        let outer = body.get_face(body.face_of_half_edge(he2)?)?.outer;
        Some(if outer == l2 {
            Self::Loops {
                target: he2,
                ring: he1,
            }
        } else {
            Self::Loops {
                target: he1,
                ring: he2,
            }
        })
    }
}

/// One fusion: `dead` collapsed onto `kept`, and the face a chord
/// divided off (a joint across two loops divides none).
pub(crate) struct Fused {
    pub(crate) dead: VertexKey,
    pub(crate) kept: VertexKey,
    pub(crate) divided: Option<FaceKey>,
}

/// **Fuses two coincident vertices**: a zero-length edge between them at
/// `p` along `joint` (the canonical self-loop carrier, whose
/// certification requires the pair bitwise coincident), collapsed onto
/// `keep`, one of its ends, by a `kev` that keeps the merged fan's
/// carriers, each re-certified at the kept vertex under the run's band.
///
/// A chord's `mef` divides the face, and each of the two corners the
/// vertices had in it then runs from one's leaving side to the other's
/// arriving side; a joint across two loops joins them into one.
///
/// # Errors
///
/// The `mef`, `mekr` or `kev`'s refusal; `desync` of what failed where
/// a half-edge the joint made no longer resolves.
pub(crate) fn fuse<T: Decide + crate::props::AtRestPolicy, E: From<EulerOpError>>(
    body: &mut Body<T>,
    joint: Joint,
    (p, keep): (geom_core::Point3<T>, VertexKey),
    desync: fn(&'static str) -> E,
    tol: Tol,
) -> Result<Fused, E> {
    let carrier = EdgeCurveSpec::self_loop_circle_at(p);
    let (plus, minus, divided) = match joint {
        Joint::Loops { target, ring } => {
            let made = body.mekr(MekrSite::Cycles { target, ring }, carrier, tol)?;
            (made.he_plus, made.he_minus, None)
        }
        Joint::Chord { he1, he2 } => {
            let made = body.mef(
                MefSite::Chords { he1, he2 },
                carrier,
                FaceSurface::Inherit,
                tol,
            )?;
            (made.he_plus, made.he_minus, Some(made.face))
        }
    };
    let start = |he: HalfEdgeKey| {
        body.get_half_edge(he)
            .map(|h| h.start)
            .ok_or_else(|| desync("a joint half-edge no longer resolves"))
    };
    let he = if start(plus)? == keep { plus } else { minus };
    let dead = body
        .half_edge_end(he)
        .ok_or_else(|| desync("a joint half-edge has no end"))?;
    body.kev_describing(he, &[], tol)?;
    Ok(Fused {
        dead,
        kept: keep,
        divided,
    })
}
