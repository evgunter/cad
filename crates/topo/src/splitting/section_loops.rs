//! **Reading a section's loops**: what the split's finish and the
//! plane-section query both need to know about the polygons the join
//! completes — the plane each side's section face wears, the in-plane
//! `u` axis, each loop's role (outline or hole), and which outline
//! encloses each hole. Neutral results: each consumer states a fault in
//! its own vocabulary.
//!
//! # The section normal of each side (derived from `enters_material`, F3)
//!
//! A face's outward normal `m` points OUT of material: direction `d`
//! enters material iff `d·m < 0`. The above body's section face has
//! the above material on its `+n_SP` side, so `+n_SP` must ENTER ⇒
//! `n_SP·m < 0` ⇒ **m = −n_SP**; symmetrically the below body's
//! section face carries **m = +n_SP** ([`section_normal`]).

use geom_brep::curve_pair::Span;
use geom_core::{Decide, Indeterminate, Point3, Real, RigidMotion, UnitVec3, Vec3};

use super::PlaneSide;
use super::containment::{LoopContainment, point_in_loop};
use crate::body::Body;
use crate::chord_join::ring_representative;
use crate::entity::{EntityId, LoopBoundary, LoopKey};
use crate::live::{CycleLoop, EdgeMember, NoCycle};
use crate::unresolved::Unresolved;
use crate::validate::{RingOuterVerdict, ring_outer_contact_about_as};

/// A traversal of a section loop met a dangling key, which it names:
/// the scratch body is torn (a kernel bug).
#[derive(Clone, Copy, Debug)]
pub(super) struct Torn(pub(super) Unresolved);

/// Whether `side`'s section faces face against the split plane's
/// normal: above (module docs). The one statement of the sign rule
/// [`section_normal`] and [`section_frame`] read.
fn faces_against_normal(side: PlaneSide) -> bool {
    matches!(side, PlaneSide::Above)
}

/// The outward normal of `side`'s section face, for the split plane's
/// normal `n`: `−n` above, `+n` below ([`faces_against_normal`]).
pub(super) fn section_normal<T: Real>(n: Vec3<T>, side: PlaneSide) -> Vec3<T> {
    if faces_against_normal(side) { -n } else { n }
}

/// The split plane `plane` (its frame, [`super::SplitPlane::frame`]) as
/// `side`'s section faces' plane, its axis their outward normal
/// ([`faces_against_normal`]).
///
/// [`point_in_loop`] is blind to which of a plane's two frames it is
/// handed, so a containment read alone takes the split plane's frame as
/// it is. [`nest`] takes this one: it also runs the contact arms
/// ([`outlines_disjoint`]), whose `lines_meet` margin is signed along
/// the axis, and handing them the side's outward normal keeps the sign
/// they have always read.
pub(super) fn section_frame<T: Real>(plane: geom::Frame<T>, side: PlaneSide) -> geom::Frame<T> {
    if faces_against_normal(side) {
        plane.reversed()
    } else {
        plane
    }
}

/// The in-plane `u` axis a section loop `l` on a plane of normal `n` is
/// charted with: its first chord's component in the plane, normalized,
/// or for a loop of one corner the in-plane component of its section
/// conic's principal direction (the conic's `u_ref`). Either way it is a
/// direction the section already holds, so it moves with the inputs
/// under a rigid motion (D10 `[placement.frame]`). Which `n` it reads is
/// the caller's to state.
///
/// A corner is a vertex the split read ON, which lies within ε of the
/// plane but not necessarily in it, so the chord itself can lean out
/// of the plane; the axis is ⊥ `n` all the same, as a plane's frame
/// must be.
///
/// # Panics
///
/// On a loop of one corner whose edge is not a whole section conic,
/// which is a kernel bug. That edge is a self-loop chord, and
/// [`crate::chord_join`] mints one only as the whole conic or as a lone
/// site's placeholder; the join refuses a loop of placeholders alone
/// (`DegenerateSection`) before any frame is read.
pub(super) fn chord_u_ref<T: Decide>(
    body: &Body<T>,
    l: LoopKey,
    points: &[Point3<T>],
    n: UnitVec3<T>,
) -> Vec3<T> {
    chord_frame(body, l, points, n).u().get()
}

/// **The section plane's frame**, standing at the world origin: `w` the
/// normal `n` as the split holds it, and `u` [`chord_u_ref`]'s
/// direction, the chord's part square to `n` normalized
/// ([`RigidMotion::squared_about`], `n` kept bit for bit). The split
/// plane is a point and a decided normal, with no frame of its own to
/// turn, so the section's frame is derived from its normal and its
/// chord, and nothing is decided.
pub(super) fn chord_frame<T: Decide>(
    body: &Body<T>,
    l: LoopKey,
    points: &[Point3<T>],
    n: UnitVec3<T>,
) -> RigidMotion<T> {
    RigidMotion::squared_about(Point3::origin(), n, chord_direction(body, l, points))
}

/// The direction [`chord_u_ref`] reads before it is taken into the
/// plane: the loop's first chord, or a one-corner loop's conic
/// principal direction.
fn chord_direction<T: Real>(body: &Body<T>, l: LoopKey, points: &[Point3<T>]) -> Vec3<T> {
    if let [a, b, ..] = points {
        return *b - *a;
    }
    let first = match body.get_loop(l).map(|lp| lp.boundary) {
        Some(LoopBoundary::Cycle { first }) => first,
        other => unreachable!("section loop {l:?} has one corner and is no cycle: {other:?}"),
    };
    let Some(principal) = whole_conic_principal(body, first) else {
        unreachable!(
            "section loop {l:?} has one corner and no whole section conic: the join refuses a \
             loop of lone-site placeholders before it is charted"
        )
    };
    principal
}

/// The principal direction (`u_ref`) of the section conic the cycle at
/// `first` is, when that cycle is one edge on a section conic that is no
/// placeholder ([`crate::chord_join::lone_site_placeholder`]).
#[expect(
    clippy::disallowed_methods,
    reason = "a conic's centre frame, derived where an ellipse is held on its arc: owed a reading through the conic (work/oneround/one-stored-form-waits-on-the-readers-of-a-conics-centre.md)"
)]
fn whole_conic_principal<T: Real>(
    body: &Body<T>,
    first: crate::entity::HalfEdgeKey,
) -> Option<Vec3<T>> {
    let he = body.get_half_edge(first)?;
    let curve = body
        .linked_edge(he.edge, crate::entity::EntityId::HalfEdge(first), "edge")
        .certified()?;
    let end = body.half_edge_end(first)?;
    if he.next != first || crate::chord_join::lone_site_placeholder(he.start, end, curve) {
        return None;
    }
    match *curve.carrier() {
        ref conic @ (geom::Curve3::Circle(_) | geom::Curve3::Ellipse(_)) => {
            conic.frame().map(|frame| frame.u_ref())
        }
        _ => None,
    }
}

/// Why a loop's role could not be read.
#[derive(Debug)]
pub(super) enum SenseFault {
    /// The winding has no sign: in the band (`Some`), or (`None`) zero,
    /// or unread because the loop carries an edge the winding does not
    /// read (`crate::loop_winding`).
    Undecided(Option<Indeterminate>),
}

/// A section loop's role about the outward `normal` of the face it
/// bounds: `true` for an outline, which winds counter-clockwise about
/// it (interior-left), `false` for a hole, which winds clockwise. Read
/// by the function tier 3's check 6 falsifies a face's sense bit with,
/// on the loop's own carriers.
///
/// # Errors
///
/// [`SenseFault`].
pub(super) fn loop_sense<T: crate::props::AtRestPolicy>(
    body: &Body<T>,
    l: LoopKey,
    normal: Vec3<T>,
    band: geom_core::Band,
) -> Result<bool, SenseFault> {
    match body.planar_loop_winding(l, normal, band) {
        Some(Ok(geom_core::Sign::Positive)) => Ok(true),
        Some(Ok(geom_core::Sign::Negative)) => Ok(false),
        Some(Ok(geom_core::Sign::Zero)) | None => Err(SenseFault::Undecided(None)),
        Some(Err(diag)) => Err(SenseFault::Undecided(Some(diag))),
    }
}

/// The holes of a section, each placed in the outline that immediately
/// encloses it ([`nest`]).
pub(super) struct Nesting<O, H> {
    /// Every outline, in input order, with the holes placed in it, in
    /// input order.
    pub regions: Vec<(O, Vec<H>)>,
    /// The holes nothing decides an enclosing outline for, in input
    /// order. Each consumer says what it makes of one.
    pub unplaced: Vec<H>,
}

/// Why [`nest`] refused.
pub(super) enum NestFault<H> {
    /// [`Torn`], with the key that did not resolve.
    Torn(Unresolved),
    /// Two outlines each read as enclosing the other around this hole:
    /// their outlines were decided disjoint, and disjoint outlines
    /// cannot (a kernel bug).
    Contradiction(H),
}

impl<H> From<Torn> for NestFault<H> {
    fn from(Torn(unresolved): Torn) -> Self {
        Self::Torn(unresolved)
    }
}

/// **The section nesting rule**: places each hole loop in the outline
/// loop that immediately encloses it. Every loop lies in one section
/// plane and is read about `normal`; the `O` and `H` values ride along
/// with their loops.
///
/// An outline encloses the hole when the two are decided disjoint
/// ([`outlines_disjoint`]) and the hole's anchor vertex is certified
/// inside the outline on the loops' own carriers
/// ([`point_in_loop`]). Disjoint outlines nest or are apart
/// (Jordan), so among several enclosing outlines — an island in a hole
/// in a face — exactly one is enclosed by all the others, and the hole
/// goes to it; two such would be two outlines each enclosing the other,
/// which disjoint outlines cannot be ([`NestFault::Contradiction`]).
///
/// **What decides nothing**, leaving a hole [`Nesting::unplaced`]: an
/// outline edge on a spiric or NURBS carrier, whose contacts nothing
/// here decides, and a containment or contact reading in the band. A
/// clockwise polygon touching the outline around it would land here
/// too: that is what a chord run outside the face it divides makes, and
/// the join pairs a face's crossings along the face's own section line
/// or conic so that none does. A face the join leaves to the sweep's
/// order — a curved face whose section is straight, a planar face whose
/// line the band cannot certify — is not covered by that pairing.
///
/// # Errors
///
/// [`NestFault`].
pub(super) fn nest<T: Decide, O, H>(
    body: &Body<T>,
    outlines: Vec<(O, LoopKey)>,
    holes: Vec<(H, LoopKey)>,
    plane: geom::Frame<T>,
    band: geom_core::Band,
) -> Result<Nesting<O, H>, NestFault<H>> {
    // Disjointness is proved first, so the two loops are nested or apart
    // and one point of `inner` reads which: not a containment reading
    // ([`crate::loop_side`]'s module docs).
    let encloses = |outer: LoopKey, inner: LoopKey| -> Result<bool, Torn> {
        if !outlines_disjoint(body, outer, inner, plane.axis(), band)? {
            return Ok(false);
        }
        let q = ring_representative(body, inner).map_err(Torn)?;
        Ok(matches!(
            point_in_loop(body, outer, plane, q, band),
            Ok(LoopContainment::In)
        ))
    };
    let loops: Vec<LoopKey> = outlines.iter().map(|&(_, l)| l).collect();
    let mut regions: Vec<(O, Vec<H>)> =
        outlines.into_iter().map(|(o, _)| (o, Vec::new())).collect();
    let mut unplaced = Vec::new();
    for (hole, hole_loop) in holes {
        let mut enclosing = Vec::new();
        for (i, &outline) in loops.iter().enumerate() {
            if encloses(outline, hole_loop)? {
                enclosing.push(i);
            }
        }
        let mut innermost = Vec::new();
        for &i in &enclosing {
            let mut inside_all = true;
            for &j in &enclosing {
                if j != i && !encloses(loops[j], loops[i])? {
                    inside_all = false;
                }
            }
            if inside_all {
                innermost.push(i);
            }
        }
        match innermost[..] {
            [] => unplaced.push(hole),
            [parent] => regions[parent].1.push(hole),
            _ => return Err(NestFault::Contradiction(hole)),
        }
    }
    Ok(Nesting { regions, unplaced })
}

/// **A hole of a section with no place in it**: the section nesting rule
/// ([`nest`]) places it in no outline (`beside` `None`), or it is not
/// decided apart from a hole the outline around it already holds
/// (`beside`, that hole) — a contact between the loops too close to call,
/// or two holes that meet. A region list cannot state such a hole, and a
/// hole left on a face of its own would lie over the face around it, two
/// faces of opposite sense on one area of the plane. `K` names a hole as
/// the refusing door does: the split by its section face, the section by
/// a corner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnplacedHole<K> {
    /// The hole.
    pub hole: K,
    /// The hole it is not decided apart from.
    pub beside: Option<K>,
}

impl<K> UnplacedHole<K> {
    /// What every door refusing it says, before its recourse.
    pub(super) fn reason(&self) -> &'static str {
        match self.beside {
            None => {
                "no outline of the section can be shown to enclose one of its holes: a \
                 contact between them too close to call"
            }
            Some(_) => {
                "two holes of one outline of the section cannot be shown apart: they meet, \
                 or come too close to call"
            }
        }
    }
}

/// Whether two loops in the section plane, with `normal`, are DECIDED
/// disjoint: `true` only where every pair of their edges is.
///
/// - **Line and circle edges**: tier 3's check 9 contact reading
///   ([`ring_outer_contact_about_as`]), which on loops in one plane that
///   carry only these kinds decides every shared point — at a vertex of
///   either loop, along an arc, or where two edges cross.
/// - **A pair with an ellipse edge**, which check 9's edge arms skip:
///   the curve-pair meeting reader over the two edges' spans
///   ([`edges_clear`]).
/// - **A spiric or NURBS edge, or an edge with no certified curve**:
///   nothing decides it, so `false`.
///
/// A margin in the band is `false` too: an undecided pair is not a
/// disjoint one.
pub(super) fn outlines_disjoint<T: Decide>(
    body: &Body<T>,
    a: LoopKey,
    b: LoopKey,
    normal: Vec3<T>,
    band: geom_core::Band,
) -> Result<bool, Torn> {
    if !matches!(
        ring_outer_contact_about_as(body, a, b, Some(normal), band),
        RingOuterVerdict::Disjoint
    ) {
        return Ok(false);
    }
    // A lone-vertex loop bounds nothing a contact reading could clear.
    let cycle = |l: LoopKey| match body.cycle_loop(l) {
        Ok(lp) => Ok(Some(lp)),
        Err(NoCycle::Lone(_)) => Ok(None),
        Err(NoCycle::Stale) => Err(Torn(Unresolved::Record(EntityId::Loop(l)))),
        Err(NoCycle::Torn) => Err(Torn(Unresolved::Walk(EntityId::Loop(l)))),
    };
    let (Some(a), Some(b)) = (cycle(a)?, cycle(b)?) else {
        return Ok(false);
    };
    let (ea, eb) = (loop_edges(a), loop_edges(b));
    for x in &ea {
        for y in &eb {
            let separated = match (x, y) {
                (OutlineEdge::Undecided, _) | (_, OutlineEdge::Undecided) => return Ok(false),
                (OutlineEdge::Read(x), OutlineEdge::Read(y)) if x.elliptic || y.elliptic => {
                    edges_clear(x, y, band)
                }
                // Lines and circles: check 9 decided them above.
                _ => true,
            };
            if !separated {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

/// One outline edge, as [`outlines_disjoint`] reads it: a line, circle
/// or ellipse edge by its carrier and parameter window, or a kind
/// nothing decides.
enum OutlineEdge<T: Real> {
    Read(Edge<T>),
    Undecided,
}

/// A line, circle or ellipse edge: its carrier, its parameter window,
/// and whether it is an ellipse, read once where the edge is.
struct Edge<T: Real> {
    carrier: geom::Curve3<T>,
    params: (T, T),
    elliptic: bool,
}

/// A cycle's edges in walk order, as [`outlines_disjoint`] reads them:
/// null scaffolding, or a carrier nothing here decides, reads
/// [`OutlineEdge::Undecided`]. The walk is the [`CycleLoop`]'s proven
/// one; an edge's curve is a link ([`crate::live::OPERATORS_KEEP_LINKS`]).
fn loop_edges<T: Decide>(lp: CycleLoop<'_, T>) -> Vec<OutlineEdge<T>> {
    lp.edges()
        .iter()
        .map(|&EdgeMember { edge, .. }| {
            let Some(curve) = edge.certified() else {
                return OutlineEdge::Undecided;
            };
            let elliptic = match *curve.carrier() {
                geom::Curve3::Line(_) | geom::Curve3::Circle(_) => false,
                geom::Curve3::Ellipse(_) => true,
                geom::Curve3::Spiric { .. }
                | geom::Curve3::Nurbs(_)
                | geom::Curve3::Periodic(_) => {
                    return OutlineEdge::Undecided;
                }
            };
            OutlineEdge::Read(Edge {
                carrier: curve.carrier().clone(),
                params: curve.params(),
                elliptic,
            })
        })
        .collect()
}

/// **Two edges of loops in one plane, one of them an ellipse, are decided
/// apart**: the curve-pair meeting reader
/// ([`geom_brep::curve_pair::meet`]) over the two spans reads no meeting
/// and no shared run.
fn edges_clear<T: Decide>(x: &Edge<T>, y: &Edge<T>, band: geom_core::Band) -> bool {
    matches!(
        geom_brep::curve_pair::meet(
            &Span::new(&x.carrier, x.params),
            &Span::new(&y.carrier, y.params),
            band
        ),
        Ok(met) if met.points.is_empty() && met.shared.is_none()
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod spline_loop_row {
    use geom_core::{Band, Tol, Vec3};

    /// **A loop with a spline edge has a role, by its bulge**: the
    /// crescent whose chord triangle is clockwise about `+z`, and which
    /// its rational arc's bulge makes counter-clockwise, is an outline
    /// about `+z` and a hole about `−z`.
    #[test]
    fn a_loop_wound_by_its_spline_edge_reads_its_role() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let (body, l, _) = crate::merge_faces::winding_arm_tests::spline_crescent(tol);
        for (normal, outline) in [(Vec3::unit_z(), true), (-Vec3::unit_z(), false)] {
            let read = super::loop_sense(&body, l, normal, band);
            assert!(
                matches!(read, Ok(role) if role == outline),
                "about {normal:?}: {read:?}"
            );
        }
    }
}

/// **A torn ring curve panics before the outlines are answered**: on a
/// holed block's top face, whose ring lies clear inside its outer loop,
/// a torn curve on the ring panics naming the link. Check 9
/// (`ring_outer_contact_about_as`), which [`outlines_disjoint`] asks first,
/// reads every curve of both loops as a link and panics there, so the
/// outline reading's own read of the same link is never reached.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_hop_rows {
    use geom_core::{Band, Tol, Vec3};

    use crate::entity::{EntityId, GeomRef};
    use crate::live::OPERATORS_KEEP_LINKS;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};

    #[test]
    fn the_ring_contact_check_panics_on_a_torn_curve() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body = crate::test_support_fixtures::holed_block::<f64>(2.0, &[1.0], tol);
        let (outer, ring) = body
            .faces()
            .find(|(_, f)| f.rings.len() == 1)
            .map(|(_, f)| (f.outer, f.rings[0]))
            .unwrap();
        let disjoint = |b: &crate::body::Body<f64>| {
            super::outlines_disjoint(b, outer, ring, Vec3::unit_z(), band).map_err(|_| "torn")
        };
        assert_eq!(
            disjoint(&body),
            Ok(true),
            "the sound ring lies clear inside the outer loop"
        );
        let first = match body.get_loop(ring).unwrap().boundary {
            crate::entity::LoopBoundary::Cycle { first } => first,
            crate::entity::LoopBoundary::Empty { .. } => panic!("the ring is a cycle"),
        };
        let edge = body.get_half_edge(first).unwrap().edge;
        let curve = body.get_edge(edge).unwrap().curve;
        body.curves.remove(curve);
        let named = format!(
            "{}'s curve names {}",
            EntityId::Edge(edge),
            GeomRef::Curve(curve)
        );
        assert_torn_op_panics(
            "outlines_disjoint",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| disjoint(b),
        );
    }
}

#[cfg(test)]
#[allow(clippy::panic, clippy::unwrap_used)]
mod edge_pair_rows {
    //! [`super::edges_clear`]: two edges, one of them elliptic, decided
    //! apart or not.
    use super::{Edge, edges_clear};
    use core::f64::consts::PI;
    use geom::{Curve3, Ellipse, Frame, Line};
    use geom_core::{Band, Point3, Vec3};

    fn band() -> Band {
        Band::linear(geom_core::Tol::witness()).unwrap()
    }

    /// The arc of the ellipse of semi-axes `a` along x and `b` along y
    /// over `params`.
    fn arc(a: f64, b: f64, params: (f64, f64)) -> Edge<f64> {
        Edge {
            carrier: Curve3::Ellipse(Ellipse::new(
                Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
                a,
                b,
            )),
            params,
            elliptic: true,
        }
    }

    /// The segment from `p` to `q` ([`Line::chord`]).
    fn segment(p: Point3<f64>, q: Point3<f64>) -> Edge<f64> {
        let (line, length) = Line::chord(p, q);
        Edge {
            carrier: Curve3::Line(line),
            params: (0.0, length),
            elliptic: false,
        }
    }

    fn clear(x: &Edge<f64>, y: &Edge<f64>) -> bool {
        edges_clear(x, y, band())
    }

    /// **An ellipse arc against a segment**: decided apart where they are
    /// apart, and not where they touch, share an end, cross, or come within
    /// the band.
    #[test]
    fn a_segment_reads_apart_from_an_arc_only_where_it_is() {
        // A segment inside the ellipse whose line crosses the carrier.
        let e = arc(3.0, 2.0, (0.0, 2.0 * PI));
        let inside = segment(Point3::new(-1.0, 0.0, 0.0), Point3::new(1.0, 0.5, 0.0));
        assert!(clear(&e, &inside), "a segment inside the ellipse");
        // Segments parallel to the major axis above the minor vertex, from
        // a circle to a flat ellipse: tangent, within the band, and past it.
        for (a, b) in [(1.0, 1.0), (1.0, 0.5), (5.0, 0.05), (10.0, 1.0)] {
            let e = arc(a, b, (0.3, 2.8));
            let above = |gap: f64| {
                segment(
                    Point3::new(-0.5 * a, b + gap, 0.0),
                    Point3::new(0.5 * a, b + gap, 0.0),
                )
            };
            assert!(!clear(&e, &above(0.0)), "{a} × {b}: a tangent segment");
            assert!(
                !clear(&e, &above(band().zero() * 0.5)),
                "{a} × {b}: a segment within the band"
            );
            assert!(
                clear(&e, &above(band().escalate() * 4.0)),
                "{a} × {b}: a segment past the band"
            );
        }
        let flat = arc(5.0, 0.05, (0.3, 2.8));
        // A segment from the arc's start point outward.
        let p0 = Point3::new(5.0 * 0.3f64.cos(), 0.05 * 0.3f64.sin(), 0.0);
        let out = segment(p0, p0 + Vec3::new(1.0, 1.0, 0.0));
        assert!(!clear(&flat, &out), "a segment sharing the arc's end");
        // A chord across an arc of more than half a turn, and the same
        // chord where that arc leaves out both of its crossings (at
        // θ = 1.25 and 1.89).
        let chord = segment(Point3::new(-0.5, 0.95, 0.0), Point3::new(0.5, 0.95, 0.0));
        let over = arc(1.2, 1.0, (1.0, 5.0));
        assert!(!clear(&over, &chord), "a chord across the arc");
        let missed = arc(1.2, 1.0, (2.0, 6.0));
        assert!(clear(&missed, &chord), "a chord the arc leaves out");
    }

    /// **Two conic arcs**: concentric carriers apart; crossing carriers
    /// apart where the arcs miss the crossings,
    /// and not where they hold one (at `(±0.894, ±0.894)`).
    #[test]
    fn two_arcs_read_apart_only_where_they_are() {
        let inner = arc(1.5, 1.0, (0.0, 2.0 * PI));
        let outer = arc(3.0, 2.0, (0.0, 2.0 * PI));
        assert!(clear(&outer, &inner), "concentric ellipses");
        assert!(
            clear(&arc(2.0, 1.0, (-0.3, 0.3)), &arc(1.0, 2.0, (-0.3, 0.3))),
            "arcs clear of the crossings"
        );
        assert!(
            !clear(&arc(2.0, 1.0, (0.9, 1.3)), &arc(1.0, 2.0, (0.3, 0.6))),
            "arcs through one crossing"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod far_clear_rows {
    use geom::{Curve3, Line};
    use geom_core::test_support::stored_belly;
    use geom_core::{Band, Point3, StoredHarmonic, Vec3};

    use super::{Edge, edges_clear};

    /// `(wrong, placed, clears)` over 2000 segments in the plane of an
    /// ellipse held at its centre, semi-axes `far` and `far/3`, both in the
    /// plane `z = 0` exactly. Each runs
    /// ten metres along the tangent at a point at any angle round the turn,
    /// that point in the box and inside the segment's run, set off it by a
    /// gap within twice the band's escalation.
    /// - **The judge.** A clear ([`edges_clear`]: the meeting reader finds
    ///   no meeting and no run) is judged exactly on the stored data,
    ///   across the plane through the segment's start along the reader's
    ///   in-plane normal `m`. It is wrong unless the belly is definitely
    ///   negative.
    /// - **`placed`.** Among the wrong clears, `placed` counts those whose
    ///   exact belly lies within the band's zero: there the touch is
    ///   dropped by the carrier walk
    ///   (`work/conic/a-centre-held-far-conics-touch-is-dropped-by-a-carrier-walk-on-points-evaluated-about-its-centre.md`),
    ///   not by the belly.
    fn far_segment_clears(far: f64) -> (usize, usize, usize) {
        let band = Band::linear_at(geom_core::Tol::witness(), 1e-12).unwrap();
        let mut rng = test_utils::fuzz::start("section_loops::far_segment_clears");
        let (mut wrong, mut placed, mut clears) = (0, 0, 0);
        for _ in 0..test_utils::fuzz::scaled(2000) {
            // In the plane `z = 0` exactly: the axis is `z`, and every
            // point and direction below has `z = 0`, so the segment lies
            // in the ellipse's stored plane and the in-plane belly is the
            // whole question.
            let axis = Vec3::unit_z();
            let turn = rng.range(0.0, core::f64::consts::TAU);
            let u = Vec3::new(turn.cos(), turn.sin(), 0.0);
            let v = axis.cross(u);
            let (a, b) = (far, far / 3.0);
            let theta = rng.range(0.0, core::f64::consts::TAU);
            let crest = Point3::new(rng.range(-500.0, 500.0), rng.range(-500.0, 500.0), 0.0);
            let centre = crest - (u * (a * theta.cos()) + v * (b * theta.sin()));
            let outward = (u * (b * theta.cos()) + v * (a * theta.sin())).normalize();
            let along = axis.cross(outward);
            let frame = geom::Frame::new(centre, axis, u);
            let ellipse = Edge {
                carrier: Curve3::Ellipse(geom::Ellipse::new(frame, a, b)),
                params: (0.0, core::f64::consts::TAU),
                elliptic: true,
            };
            let gap = rng.range(-2.0, 2.0) * band.escalate();
            let by = crest + outward * gap;
            let p = by - along * rng.range(2.0, 8.0);
            let q = p + along * 10.0;
            let (line, length) = Line::chord(p, q);
            let segment = Edge {
                carrier: Curve3::Line(line),
                params: (0.0, length),
                elliptic: false,
            };
            if !edges_clear(&segment, &ellipse, band) {
                continue;
            }
            clears += 1;
            let arr = |w: Vec3<f64>| [w.x, w.y, w.z];
            let at = frame.origin();
            let stored = StoredHarmonic {
                at: [at.x, at.y, at.z],
                axis: arr(frame.axis()),
                u_ref: arr(frame.u_ref()),
                semi: (a, b),
                phase: (0.0, 0.0),
            };
            let (o, n) = ([p.x, p.y, p.z], arr(frame.axis().cross(line.dir.get())));
            let threshold = band.escalate() * (1.0 - 64.0 * f64::EPSILON);
            if !stored_belly::below(stored, o, n, -threshold) {
                wrong += 1;
                let zero = band.zero() * (1.0 + 64.0 * f64::EPSILON);
                let touch = !stored_belly::exceeds(stored, o, n, zero)
                    && !stored_belly::below(stored, o, n, -zero);
                placed += usize::from(touch);
            }
        }
        (wrong, placed, clears)
    }

    /// **A segment clears a far-centred conic only where it is clear, past
    /// the touch's placement** ([`edges_clear`]): no clear where the exact
    /// belly is not definitely negative, but for a touch the carrier walk
    /// drops ([`far_segment_clears`]'s `placed`).
    #[test]
    fn a_segment_clears_a_far_centred_conic_only_where_it_is_clear() {
        let got: Vec<_> = [1e4, 1e5, 1e6]
            .map(|far| (far, far_segment_clears(far)))
            .to_vec();
        println!("(far, (wrong, placed, clears)): {got:?}");
        assert!(
            got.iter().all(|g| g.1.0 == g.1.1),
            "cleared wrongly past the touch's placement: {got:?}; {}",
            test_utils::fuzz::replay()
        );
    }
}
