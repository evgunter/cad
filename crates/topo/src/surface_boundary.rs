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
//!   placement read it: `boolean::region::boundary` reads its cycles, and
//!   tier 3's region arm (`crate::one_region`) reads each cycle against
//!   the others, a torus's by their windings
//!   ([`surface_cycle_windings`]). A face every edge of which it lies on
//!   both sides of (the one-face ball) has an empty surface boundary, and
//!   covers its surface.
//! - **The chart image** ([`chart_image`]) drops only the spurs: a
//!   half-edge followed in its loop by its own mate walks its edge out
//!   and straight back, which bounds no area. The reduction and the rule
//!   that a loop left empty bounds nothing are `geom_brep::chart_image`'s.
//!   Seams stay: a Green form written in a lifted chart coordinate (the
//!   torus's tube angle) reads a seam's two traversals a period apart, and
//!   that term is the face's area. Every integrator and chart reader
//!   takes this view (`props::chart_image_edges`). A chart image that is
//!   empty on a closed surface is the whole surface, signed by the face's
//!   sense (`geom_brep::props::whole_surface`).
//!
//! **The tips** ([`tips`]) are what both views drop that the face still
//! holds: the end of a half-edge its loop follows by its own mate, a
//! vertex with no other edge, which on a body past tier 2 is a wrap
//! edge's tip and so the face's chart singularity. The chart image hands
//! its loop's tips back ([`ChartImage`]). The readers of a sphere face's
//! side keep them as the datum of that side: tier 3's check 6
//! (`geom_brep::props::boundary_material_sign_holding`), the closed-form
//! lane's cross-check of the bit (`geom_brep::props::curved_face_holding`)
//! and the mass-property doors' seam refusals (`props::face_tips`). Two
//! readers of a sphere face do not: a boolean's sphere window, which
//! reads a face holding a pole as the whole ball's box, and the seam
//! region's reader, which refuses a region holding a tip
//! (`work/nearpoint/a-seam-region-holding-a-wrap-tip-is-never-read`).

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, VertexKey};
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
}

/// **A loop's chart image** (module docs): `cycle`, one loop's
/// half-edges in loop order, reduced by
/// [`geom_brep::chart_image::without_reversals`] with a half-edge followed
/// by its own mate read as its edge walked straight back, and the tips
/// that reduction drops ([`tips`]). Its half-edges are empty where the
/// whole loop is spurs. `None` where a member's mate or successor does
/// not resolve.
#[must_use]
pub fn chart_image<T: Real>(body: &Body<T>, cycle: &[HalfEdgeKey]) -> Option<ChartImage> {
    let mates = cycle
        .iter()
        .map(|&he| Some((he, body.mate(he)?)))
        .collect::<Option<Vec<_>>>()?;
    let image =
        geom_brep::chart_image::without_reversals(&mates, |&(_, mate), &(next, _)| mate == next);
    Some(ChartImage {
        half_edges: image.into_iter().map(|(he, _)| he).collect(),
        tips: tips(body, cycle)?,
    })
}

/// **A loop's chart image and the tips it drops** ([`chart_image`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChartImage {
    /// The loop's half-edges with its spurs taken out, in loop order.
    pub half_edges: Vec<HalfEdgeKey>,
    /// The tips of its spurs ([`tips`]).
    pub tips: Vec<VertexKey>,
}

/// **The tips among `hes`**: the end vertex of each half-edge whose
/// successor is its own mate. Nothing else meets that vertex, since its
/// rotation (`next(mate(·))`) closes on the one edge, so it is the
/// valence-1 end of a spur, stored structure read with no geometry, in
/// the order of `hes`. `None` where a key does not resolve.
///
/// On a tier-1 body this is one fact read three ways: the vertex here,
/// the valence-1 vertex `validate::wrap_tips` counts, and the orbit of
/// one `Body::strut_tip` walks. A wrap tip is such an end of a wrap edge;
/// tier 2 refuses any other at rest (`ScaffoldingStrutVertex`), so past
/// tier 2 every tip here is a wrap tip.
#[must_use]
pub fn tips<T: Real>(body: &Body<T>, hes: &[HalfEdgeKey]) -> Option<Vec<VertexKey>> {
    let mut tips = Vec::new();
    for &h in hes {
        if body.get_half_edge(h)?.next == body.mate(h)? {
            tips.push(body.get_half_edge(body.mate(h)?)?.start);
        }
    }
    Some(tips)
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
    Ok(surface_cycle_windings(body, face)?
        .into_iter()
        .map(|(cycle, _)| cycle)
        .collect())
}

/// **A surface cycle and its winding** ([`surface_cycle_windings`]): its
/// half-edges in walk order, and the composed joint elements of every
/// half-edge its walk enters, `None` where one stores no element.
pub type WoundCycle = (Vec<HalfEdgeKey>, Option<crate::joint::Winding>);

/// **[`surface_cycles`] with each cycle's winding in the face's chart**
/// ([`crate::joint::Winding`]), read off the same walk: the joint
/// elements of every half-edge the walk enters, the cycle's members and
/// the edges the face lies on both sides of that it turns through. A
/// half-edge's element carries its image onto the end of its loop
/// predecessor's, and an edge's two halves share one image, so the
/// elements entered between two members compose to the element that
/// would join them were they consecutive: a kill's sum (`crate::joint`'s
/// module docs). Every stored element is read and none is decided. A
/// cycle's winding is `None` where a half-edge it enters stores no
/// element.
///
/// # Errors
///
/// As [`surface_cycles`].
pub fn surface_cycle_windings<T: Real>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<WoundCycle>, &'static str> {
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
    let mut elements: Vec<Option<Vec<crate::joint::JointElement>>> = Vec::new();
    let cycles = boundary_cycles(body, open, members.len(), both_sides, |k, h| {
        if elements.len() <= k {
            elements.push(Some(Vec::new()));
        }
        let entered = &mut elements[k];
        match (entered.as_mut(), body.joint(h)) {
            (Some(list), Some(element)) => list.push(element),
            _ => *entered = None,
        }
    })?;
    Ok(cycles
        .into_iter()
        .zip(elements)
        .map(|(cycle, elements)| (cycle, elements.map(crate::joint::Winding::of)))
        .collect())
}

/// **The boundary cycles of a set of half-edges**: from each of `open`,
/// the boundary half-edges in their stored order, the next boundary
/// half-edge out of its end vertex ([`successor`]), at most `bound`
/// turns. `entered` sees, with its cycle's index, every half-edge each
/// turn enters, in walk order. The one walk of the surface boundary
/// ([`surface_cycle_windings`]) and of a seam region's boundary
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
    mut entered: impl FnMut(usize, HalfEdgeKey),
) -> Result<Vec<Vec<HalfEdgeKey>>, &'static str> {
    let mut cycles = Vec::new();
    while let Some(&first) = open.first() {
        let k = cycles.len();
        let mut cycle = Vec::new();
        let mut h = first;
        loop {
            cycle.push(h);
            open.retain(|&x| x != h);
            let n = successor(body, h, bound, &inside, |e| entered(k, e))?;
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

/// **The boundary half-edge after `h`**: out of its end vertex, turning
/// there through every half-edge `inside` reads as interior to the set
/// (`next(mate(·))`, the vertex's rotation), at most `bound` turns.
/// `entered` sees each half-edge the turn enters, the interior ones and
/// then the successor, in order.
///
/// # Errors
///
/// As [`boundary_cycles`]'s.
fn successor<T: Real>(
    body: &Body<T>,
    h: HalfEdgeKey,
    bound: usize,
    inside: &impl Fn(HalfEdgeKey) -> Result<bool, &'static str>,
    mut entered: impl FnMut(HalfEdgeKey),
) -> Result<HalfEdgeKey, &'static str> {
    let unresolved = "a boundary walk reached a key that does not resolve";
    let next = |h: HalfEdgeKey| body.get_half_edge(h).map(|h| h.next).ok_or(unresolved);
    let mut n = next(h)?;
    let mut turns = 0;
    while inside(n)? {
        entered(n);
        n = next(body.mate(n).ok_or(unresolved)?)?;
        turns += 1;
        if turns > bound {
            return Err("a boundary walk's vertex has no finite rotation");
        }
    }
    entered(n);
    Ok(n)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::construction::MevSite;
    use geom_core::{Point3, Tol};

    /// A lone strut on a seed face: its loop walks it out and back, so
    /// the face lies on both sides of it, each half turns back, both
    /// views are empty, and its two ends are the chart image's tips.
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
            assert_eq!(
                body.get_half_edge(he).unwrap().next,
                body.mate(he).unwrap(),
                "{he:?} turns back"
            );
        }
        let cycle = body.loop_cycle(e.he_plus).unwrap();
        let image = chart_image(&body, &cycle).unwrap();
        assert_eq!(image.half_edges, Vec::new(), "no chart image");
        let mut ends = image.tips;
        ends.sort();
        let mut want = vec![seed.vertex, strut.vertex];
        want.sort();
        assert_eq!(ends, want, "the chart image drops both ends");
        assert_eq!(
            surface_cycles(&body, seed.face),
            Ok(Vec::new()),
            "no surface boundary"
        );
    }

    /// A strut off a strut: the loop walks `a` out, `b` out, `b` back,
    /// `a` back, so the inner reversal is taken out first and the outer
    /// one then meets; a chain of spurs leaves nothing, and its tips are
    /// its two free ends, not the vertex between the struts.
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
        let b = body
            .mev_line(
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
            .filter(|&&h| body.get_half_edge(h).unwrap().next == body.mate(h).unwrap())
            .count();
        assert_eq!(turning, 2, "only the far strut and the seed turn back");
        let mut want = vec![seed.vertex, b.vertex];
        want.sort();
        // Every rotation of the cycle reads the same image and tips.
        for k in 0..cycle.len() {
            let mut rotated = cycle.clone();
            rotated.rotate_left(k);
            let image = chart_image(&body, &rotated).unwrap();
            assert_eq!(image.half_edges, Vec::new(), "from {k}: no image");
            let mut ends = image.tips;
            ends.sort();
            assert_eq!(ends, want, "from {k}: the free ends");
        }
    }
}
