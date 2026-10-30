//! **The edges a face lies on both sides of, and the two views of its
//! boundary that read them.**
//!
//! An edge whose two half-edges both lie in loops of one face
//! ([`Body::both_sides_face`]) separates nothing of that face from
//! anything else: a seam a face closed round its chart walks one way and
//! back, a wrap edge, a strut, a spur. The predicate reads half-edge
//! identity alone, with no tolerance and no geometry, and two views of a
//! face's boundary are read off it:
//!
//! - **The surface boundary** ([`surface_cycles`]) drops every such edge:
//!   what remains is the cycles of edges that have another face on their
//!   far side, the face's boundary on its surface. Region and point
//!   placement read it (`boolean::region::boundary`). A face every edge of
//!   which it lies on both sides of (the one-face ball) has an empty
//!   surface boundary, and covers its surface.
//! - **The chart image** ([`chart_image`]) drops only the spurs: a
//!   half-edge whose next is its own mate turns back along its edge
//!   ([`Body::turns_back`]), so the pair walks out and straight back and
//!   bounds no area. It is taken out, again and again until no such pair
//!   is left, since a spur whose edge was split shows its inner reversal
//!   first. Seams stay: a Green form written in a lifted chart
//!   coordinate (the torus's tube angle) reads a seam's two traversals a
//!   period apart, and that term is the face's area. Every integrator and
//!   chart reader takes this view (`props::chart_image_edges`). A chart
//!   image that is empty on a closed surface is the whole surface, signed
//!   by the face's sense (`geom_brep::props::whole_surface`).

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary};
use geom_core::Real;

impl<T: Real> Body<T> {
    /// **The face `edge` lies on both sides of**: `Some(Some(face))`
    /// where both its half-edges are in loops of `face`, `Some(None)`
    /// where they bound two faces, and `None` where a key on the walk
    /// from the edge to either face does not resolve.
    #[must_use]
    pub fn both_sides_face(&self, edge: EdgeKey) -> Option<Option<FaceKey>> {
        let e = self.get_edge(edge)?;
        let face = self.face_of_half_edge(e.he_plus)?;
        Some((self.face_of_half_edge(e.he_minus)? == face).then_some(face))
    }

    /// **Whether `he` turns back along its edge**: its loop's next
    /// half-edge is its own mate, so the two walk the edge out and
    /// straight back, a spur of the face [`Body::both_sides_face`] names.
    /// `None` where `he` or its mate does not resolve.
    #[must_use]
    pub fn turns_back(&self, he: HalfEdgeKey) -> Option<bool> {
        Some(self.get_half_edge(he)?.next == self.mate(he)?)
    }
}

/// **A loop's chart image** (module docs): `cycle`, one loop's
/// half-edges in loop order, with each half-edge that its successor in
/// what is left walks straight back ([`Body::turns_back`] on the image)
/// taken out with that successor, until none is left; the pairs that
/// close across the cycle's start included. Empty where the whole loop is
/// spurs. `None` where a member's mate does not resolve.
#[must_use]
pub fn chart_image<T: Real>(body: &Body<T>, cycle: &[HalfEdgeKey]) -> Option<Vec<HalfEdgeKey>> {
    let mut kept: Vec<HalfEdgeKey> = Vec::with_capacity(cycle.len());
    for &he in cycle {
        if let Some(&last) = kept.last()
            && body.mate(last)? == he
        {
            kept.pop();
        } else {
            kept.push(he);
        }
    }
    while let (Some(&last), Some(&first)) = (kept.last(), kept.first()) {
        if kept.len() < 2 || body.mate(last)? != first {
            break;
        }
        kept.pop();
        kept.remove(0);
    }
    if kept.len() != cycle.len() && std::env::var_os("PROBE_SPUR").is_some() {
        panic!("PROBE_SPUR topo chart_image dropped {} of {}", cycle.len() - kept.len(), cycle.len());
    }
    Some(kept)
}

/// **A face's surface boundary** (module docs): from each half-edge of
/// its loops whose edge it does not lie on both sides of, the walk to the
/// next such half-edge out of its end vertex, crossing every edge it does
/// lie on both sides of to that edge's mate ([`boundary_cycles`]). Each
/// cycle is in walk order, the cycles in the order their first
/// half-edges are stored, the outer loop first. Empty where the face lies
/// on both sides of every edge it has.
///
/// # Errors
///
/// Naming what failed, where a key on the walk does not resolve or a
/// vertex's rotation does not close: tier 1 refuses such a body.
pub fn surface_cycles<T: Real>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<Vec<HalfEdgeKey>>, &'static str> {
    let unresolved = "a surface boundary walked a key that does not resolve";
    let f = body.get_face(face).ok_or(unresolved)?;
    let mut members = Vec::new();
    for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        if let LoopBoundary::Cycle { first } = body.get_loop(lk).ok_or(unresolved)?.boundary {
            members.extend(body.loop_cycle(first).ok_or(unresolved)?);
        }
    }
    let both_sides = |h: HalfEdgeKey| {
        let edge = body.get_half_edge(h).ok_or(unresolved)?.edge;
        Ok(body.both_sides_face(edge).ok_or(unresolved)?.is_some())
    };
    let mut open = Vec::with_capacity(members.len());
    for &h in &members {
        if !both_sides(h)? {
            open.push(h);
        }
    }
    boundary_cycles(body, open, members.len(), both_sides)
}

/// **The boundary cycles of a set of half-edges**: from each of `open`,
/// the boundary half-edges in their stored order, the next boundary
/// half-edge out of its end vertex, turning there through every
/// half-edge `inside` reads as interior to the set (`next(mate(·))`, the
/// vertex's rotation), at most `bound` turns. The one walk of the
/// surface boundary ([`surface_cycles`]) and of a seam region's boundary
/// (`props::seam_region`).
///
/// # Errors
///
/// Naming what failed: a key that does not resolve, an `inside` that
/// refuses, a rotation past `bound`, or a boundary half-edge reached
/// twice or never left (no one successor).
pub(crate) fn boundary_cycles<T: Real>(
    body: &Body<T>,
    mut open: Vec<HalfEdgeKey>,
    bound: usize,
    inside: impl Fn(HalfEdgeKey) -> Result<bool, &'static str>,
) -> Result<Vec<Vec<HalfEdgeKey>>, &'static str> {
    let unresolved = "a boundary walk reached a key that does not resolve";
    let next = |h: HalfEdgeKey| body.get_half_edge(h).map(|h| h.next).ok_or(unresolved);
    let mut cycles = Vec::new();
    while let Some(&first) = open.first() {
        let mut cycle = Vec::new();
        let mut h = first;
        loop {
            cycle.push(h);
            open.retain(|&x| x != h);
            let mut n = next(h)?;
            let mut turns = 0;
            while inside(n)? {
                n = next(body.mate(n).ok_or(unresolved)?)?;
                turns += 1;
                if turns > bound {
                    return Err("a boundary walk's vertex has no finite rotation");
                }
            }
            if n == first {
                break;
            }
            if !open.contains(&n) {
                return Err("a boundary walk's half-edge has no one successor");
            }
            h = n;
        }
        cycles.push(cycle);
    }
    Ok(cycles)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::construction::MevSite;
    use geom_core::{Point3, Tol};

    /// A lone strut on a seed face: its loop walks it out and back, so
    /// the face lies on both sides of it, each half turns back, and both
    /// views are empty.
    #[test]
    fn a_lone_strut_is_both_sides_and_a_spur() {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
        let strut = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                Point3::new(1.0, 0.0, 0.0),
                Tol::witness(),
            )
            .unwrap();
        assert_eq!(body.both_sides_face(strut.edge), Some(Some(seed.face)));
        let e = body.get_edge(strut.edge).unwrap();
        for he in [e.he_plus, e.he_minus] {
            assert_eq!(body.turns_back(he), Some(true), "{he:?} turns back");
        }
        let cycle = body.loop_cycle(e.he_plus).unwrap();
        assert_eq!(
            chart_image(&body, &cycle),
            Some(Vec::new()),
            "no chart image"
        );
        assert_eq!(
            surface_cycles(&body, seed.face),
            Ok(Vec::new()),
            "no surface boundary"
        );
    }

    /// A strut off a strut: the loop walks `a` out, `b` out, `b` back,
    /// `a` back, so the inner reversal is taken out first and the outer
    /// one then meets; a chain of spurs leaves nothing.
    #[test]
    fn a_split_spur_comes_out_inner_reversal_first() {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
        let a = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                Point3::new(1.0, 0.0, 0.0),
                Tol::witness(),
            )
            .unwrap();
        body.mev_line(
            MevSite::Fan {
                he1: a.he_minus,
                he2: a.he_minus,
            },
            Point3::new(2.0, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
        let cycle = body.loop_cycle(a.he_plus).unwrap();
        assert_eq!(cycle.len(), 4, "two struts, each walked both ways");
        let turning = cycle
            .iter()
            .filter(|&&h| body.turns_back(h) == Some(true))
            .count();
        assert_eq!(turning, 2, "only the far strut and the seed turn back");
        assert_eq!(chart_image(&body, &cycle), Some(Vec::new()));
        // Every rotation of the cycle reads the same image.
        for k in 0..cycle.len() {
            let mut rotated = cycle.clone();
            rotated.rotate_left(k);
            assert_eq!(chart_image(&body, &rotated), Some(Vec::new()), "from {k}");
        }
    }
}
