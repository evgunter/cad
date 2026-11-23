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

use geom::Conic;
use geom_brep::curve_pair::Span;
use geom_core::{Decide, Indeterminate, Point3, Real, RigidMotion, UnitVec3, Vec3};

use super::PlaneSide;
use super::containment::{LoopContainment, point_in_loop};
use crate::body::Body;
use crate::chord_join::ring_representative;
use crate::entity::{EntityId, LoopBoundary, LoopKey};
use crate::unresolved::Unresolved;
use crate::validate::definitely_positive as positive;
use crate::validate::{RingOuterVerdict, ring_outer_contact_about_as};

/// A traversal of a section loop met a dangling key, which it names:
/// the scratch body is torn (a kernel bug).
#[derive(Clone, Copy, Debug)]
pub(super) struct Torn(pub(super) Unresolved);

/// The outward normal of `side`'s section face, for the split plane's
/// normal `n` (module docs): `−n` above, `+n` below.
pub(super) fn section_normal<T: Real>(n: Vec3<T>, side: PlaneSide) -> Vec3<T> {
    match side {
        PlaneSide::Above => -n,
        _ => n,
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
fn whole_conic_principal<T: Real>(
    body: &Body<T>,
    first: crate::entity::HalfEdgeKey,
) -> Option<Vec3<T>> {
    let he = body.get_half_edge(first)?;
    let curve = body
        .get_edge(he.edge)
        .and_then(|e| body.get_curve_geom(e.curve))
        .and_then(crate::null::CurveGeom::certified)?;
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
    normal: Vec3<T>,
    band: geom_core::Band,
) -> Result<Nesting<O, H>, NestFault<H>> {
    let encloses = |outer: LoopKey, inner: LoopKey| -> Result<bool, Torn> {
        if !outlines_disjoint(body, outer, inner, normal, band)? {
            return Ok(false);
        }
        let q = ring_representative(body, inner).map_err(Torn)?;
        Ok(matches!(
            point_in_loop(body, outer, normal, q, band),
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
/// - **A pair with an ellipse edge**, which check 9's edge arms skip
///   ([`edges_clear`]): first on the CARRIERS, closed forms that decide
///   the whole carriers apart, a superset of the arcs. A line against a
///   conic: the conic's whole carrier on one side of the line
///   (**`split_nest_line_conic`**, the offset of the conic's centre from
///   the line less its amplitude across it, `√((m·a)² + (m·b)²)`). Two
///   conics: in the unit coordinates of one, the other's carrier wholly
///   inside or wholly outside the unit circle, either way round
///   (**`split_nest_conic_conic`**; the bound is at [`conics_clear`]).
///   Then, where the carriers meet, on the EDGES: the curve-pair meeting
///   reader ([`geom_brep::curve_pair::meet`]) over the two spans finds
///   no meeting, as a segment inside an ellipse or an arc whose carrier
///   crosses the other's only off it.
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
    let (ea, eb) = (loop_edges(body, a)?, loop_edges(body, b)?);
    for x in &ea {
        for y in &eb {
            let separated = match (x, y) {
                (OutlineEdge::Undecided, _) | (_, OutlineEdge::Undecided) => return Ok(false),
                (OutlineEdge::Read(c, s), OutlineEdge::Read(d, t))
                    if matches!(c, geom::Curve3::Ellipse(_))
                        || matches!(d, geom::Curve3::Ellipse(_)) =>
                {
                    edges_clear((c, *s), (d, *t), band)
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
    Read(geom::Curve3<T>, (T, T)),
    Undecided,
}

/// A conic carrier read as a conic.
fn conic<T: Real>(carrier: &geom::Curve3<T>) -> &dyn Conic<T> {
    carrier
        .conic()
        .unwrap_or_else(|| unreachable!("a conic edge's carrier is a circle or an ellipse"))
}

/// Loop `l`'s edges in walk order, as [`outlines_disjoint`] reads them.
/// [`Torn`] where a record on the walk does not resolve, except an
/// edge's curve: the section's body is mid-operation, where that is a
/// link ([`crate::live::OPERATORS_KEEP_LINKS`]), so a torn curve panics
/// where null scaffolding reads [`OutlineEdge::Undecided`].
fn loop_edges<T: Decide>(body: &Body<T>, l: LoopKey) -> Result<Vec<OutlineEdge<T>>, Torn> {
    let record = |entity| Torn(Unresolved::Record(entity));
    // A lone-vertex loop bounds nothing a contact reading could clear.
    let first = match body.get_loop(l).ok_or(record(EntityId::Loop(l)))?.boundary {
        LoopBoundary::Cycle { first } => first,
        LoopBoundary::Empty { .. } => return Ok(vec![OutlineEdge::Undecided]),
    };
    let mut out = Vec::new();
    for he in body
        .loop_cycle(first)
        .ok_or(Torn(Unresolved::Walk(EntityId::Loop(l))))?
    {
        let h = body
            .get_half_edge(he)
            .ok_or(record(EntityId::HalfEdge(he)))?;
        let edge = body
            .get_edge(h.edge)
            .ok_or(record(EntityId::Edge(h.edge)))?;
        let Some(curve) = body.edge_curve_linked(h.edge, edge).certified() else {
            out.push(OutlineEdge::Undecided);
            continue;
        };
        out.push(match *curve.carrier() {
            geom::Curve3::Line(_) | geom::Curve3::Circle(_) | geom::Curve3::Ellipse(_) => {
                OutlineEdge::Read(curve.carrier().clone(), curve.params())
            }
            geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) | geom::Curve3::Periodic(_) => {
                OutlineEdge::Undecided
            }
        });
    }
    Ok(out)
}

/// **Two edges of loops in one plane, one of them an ellipse, are decided
/// apart**: on the carriers first, then on the spans. The carrier
/// readings are closed forms, and what they decide holds for every arc of
/// the carriers; they are read first so a pair they decide is never left
/// to the span reading's escalations. The span reading,
/// [`geom_brep::curve_pair::meet`], decides the pairs whose carriers meet
/// off the edges: apart where it reads no meeting and no shared run.
fn edges_clear<T: Decide>(
    (c, s): (&geom::Curve3<T>, (T, T)),
    (d, t): (&geom::Curve3<T>, (T, T)),
    band: geom_core::Band,
) -> bool {
    let carriers = match (c, d) {
        (geom::Curve3::Line(l), e) | (e, geom::Curve3::Line(l)) => {
            line_clears_conic(l.origin, l.dir, conic(e), band)
        }
        (e, h) => conics_clear(conic(e), conic(h), band),
    };
    carriers
        || matches!(
            geom_brep::curve_pair::meet(&Span::new(c, s), &Span::new(d, t), band),
            Ok(met) if met.points.is_empty() && met.shared.is_none()
        )
}

/// The conic's whole carrier lies definitely on one side of the line
/// through `p` along `along` (both in the conic's plane).
fn line_clears_conic<T: Decide>(
    p: Point3<T>,
    along: Vec3<T>,
    c: &dyn Conic<T>,
    band: geom_core::Band,
) -> bool {
    if !positive("split_nest_line_conic", along.norm(), band)
        || !positive("split_nest_line_conic", c.speed_lo(), band)
    {
        return false;
    }
    let m = c.frame().axis().cross(along.normalize());
    let (p_m, q_m) = c.along(m);
    let reach = (p_m.powi(2) + q_m.powi(2)).sqrt();
    positive(
        "split_nest_line_conic",
        m.dot(c.frame().origin() - p).abs() - reach,
        band,
    )
}

/// `h`'s carrier lies definitely inside or definitely outside `e`'s
/// (both in one plane).
///
/// **The bound.** In `e`'s unit coordinates (`e` the unit circle), `h`
/// is `c′ + M·(cos θ, sin θ)` with `M = [a′ b′]`, so its distance from
/// the origin lies in `[|c′| − σ, |c′| + σ]`, `σ` the largest singular
/// value of `M`: `σ² = (S + √((|a′|² − |b′|²)² + 4(a′·b′)²)) / 2`,
/// `S = |a′|² + |b′|²` — exact for the 2×2, with the radicand a sum of
/// squares so rounding cannot take it negative. `h` is inside when
/// `|c′| + σ < 1`, outside when `|c′| − σ > 1`. Both margins are levered
/// by `e`'s smaller semi-axis, the least a unit step in its coordinates
/// spans in metres. `σ` is padded up by a few ulps so the bound stays an
/// upper bound under rounding; the triangle inequality in `|c′| ± σ`
/// is the one remaining slack (exact for concentric conics).
fn conics_clear<T: Decide>(e: &dyn Conic<T>, h: &dyn Conic<T>, band: geom_core::Band) -> bool {
    let lever = e.speed_lo();
    if !positive("split_nest_conic_conic", lever, band) {
        return false;
    }
    let (ha, hb) = h.semi_axis_vectors();
    let (cx, cy) = e.unit_in_plane(h.frame().origin());
    let (ax, ay) = e.unit_of(ha);
    let (bx, by) = e.unit_of(hb);
    let centre = (cx.powi(2) + cy.powi(2)).sqrt();
    let (aa, bb, ab) = (
        ax.powi(2) + ay.powi(2),
        bx.powi(2) + by.powi(2),
        ax * bx + ay * by,
    );
    let spread = ((aa - bb).powi(2) + T::from_f64(4.0) * ab.powi(2)).sqrt();
    let reach =
        ((aa + bb + spread) * T::from_f64(0.5)).sqrt() * T::from_f64(1.0 + 8.0 * f64::EPSILON);
    let inside = (T::one() - centre - reach) * lever;
    let outside = (centre - reach - T::one()) * lever;
    positive("split_nest_conic_conic", inside, band)
        || positive("split_nest_conic_conic", outside, band)
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
    use super::edges_clear;
    use core::f64::consts::PI;
    use geom::{Curve3, Ellipse, Frame, Line};
    use geom_core::{Band, Point3, Vec3};

    fn band() -> Band {
        Band::linear(geom_core::Tol::witness()).unwrap()
    }

    fn ellipse(a: f64, b: f64) -> Curve3<f64> {
        Curve3::Ellipse(Ellipse::new(
            Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
            a,
            b,
        ))
    }

    /// The segment from `p` to `q`, on its line's unit window.
    fn segment(p: Point3<f64>, q: Point3<f64>) -> (Curve3<f64>, (f64, f64)) {
        (
            Curve3::Line(Line {
                origin: p,
                dir: q - p,
            }),
            (0.0, 1.0),
        )
    }

    fn clear(e: &(Curve3<f64>, (f64, f64)), s: &(Curve3<f64>, (f64, f64))) -> bool {
        edges_clear((&e.0, e.1), (&s.0, s.1), band())
    }

    /// **An ellipse arc against a segment**: decided apart where they are
    /// apart, and not where they touch, share an end, cross, or come within
    /// the band.
    #[test]
    fn a_segment_reads_apart_from_an_arc_only_where_it_is() {
        // A segment inside the ellipse whose line crosses the carrier: the
        // carrier reading cannot decide it, the span reading does.
        let e = (ellipse(3.0, 2.0), (0.0, 2.0 * PI));
        let inside = segment(Point3::new(-1.0, 0.0, 0.0), Point3::new(1.0, 0.5, 0.0));
        assert!(clear(&e, &inside), "a segment inside the ellipse");
        // A flat arc, and segments parallel to its major axis above its
        // minor vertex: tangent, within the band, and past it.
        let flat = (ellipse(5.0, 0.05), (0.3, 2.8));
        let above = |gap: f64| {
            segment(
                Point3::new(-1.0, 0.05 + gap, 0.0),
                Point3::new(1.0, 0.05 + gap, 0.0),
            )
        };
        assert!(!clear(&flat, &above(0.0)), "a tangent segment");
        assert!(
            !clear(&flat, &above(band().zero() * 0.5)),
            "a segment within the band"
        );
        assert!(
            clear(&flat, &above(band().escalate() * 4.0)),
            "a segment past the band"
        );
        // A segment from the arc's start point outward.
        let p0 = Point3::new(5.0 * 0.3f64.cos(), 0.05 * 0.3f64.sin(), 0.0);
        let out = segment(p0, p0 + Vec3::new(1.0, 1.0, 0.0));
        assert!(!clear(&flat, &out), "a segment sharing the arc's end");
        // A chord across an arc of more than half a turn, and the same
        // chord where that arc leaves out both of its crossings (at
        // θ = 1.25 and 1.89).
        let chord = segment(Point3::new(-0.5, 0.95, 0.0), Point3::new(0.5, 0.95, 0.0));
        let over = (ellipse(1.2, 1.0), (1.0, 5.0));
        assert!(!clear(&over, &chord), "a chord across the arc");
        let missed = (ellipse(1.2, 1.0), (2.0, 6.0));
        assert!(clear(&missed, &chord), "a chord the arc leaves out");
    }

    /// **Two conic arcs**: concentric carriers decided apart on the
    /// carriers; crossing carriers apart where the arcs miss the crossings,
    /// and not where they hold one (at `(±0.894, ±0.894)`).
    #[test]
    fn two_arcs_read_apart_only_where_they_are() {
        let inner = (ellipse(1.5, 1.0), (0.0, 2.0 * PI));
        let outer = (ellipse(3.0, 2.0), (0.0, 2.0 * PI));
        assert!(clear(&outer, &inner), "concentric ellipses");
        let wide = ellipse(2.0, 1.0);
        let tall = ellipse(1.0, 2.0);
        assert!(
            clear(&(wide.clone(), (-0.3, 0.3)), &(tall.clone(), (-0.3, 0.3))),
            "arcs clear of the crossings"
        );
        assert!(
            !clear(&(wide, (0.9, 1.3)), &(tall, (0.3, 0.6))),
            "arcs through one crossing"
        );
    }
}

#[cfg(test)]
#[allow(clippy::panic, clippy::unwrap_used, clippy::float_cmp)]
mod review_nesting_probes {
    //! Review probes for PR 7436: `arc_clears` against dense sampling.
    use super::edges_clear;
    use geom::Curve3;
    use geom::{Conic, Ellipse, Frame};
    use geom_core::{Band, Point3, Vec3};

    fn seg_c(p: Point3<f64>, q: Point3<f64>) -> Curve3<f64> {
        Curve3::Line(geom::Line { origin: p, dir: q - p })
    }
    pub(super) fn arc_seg(e: &Ellipse<f64>, span: (f64, f64), p: Point3<f64>, q: Point3<f64>, band: Band) -> bool {
        edges_clear((&Curve3::Ellipse(e.clone()), span), (&seg_c(p, q), (0.0, 1.0)), band)
    }
    fn arc_arc(c: &Ellipse<f64>, s: (f64, f64), d: &Ellipse<f64>, t: (f64, f64), band: Band) -> bool {
        edges_clear((&Curve3::Ellipse(c.clone()), s), (&Curve3::Ellipse(d.clone()), t), band)
    }
    fn band() -> Band {
        Band::linear(geom_core::Tol::witness()).unwrap()
    }

    /// A tiny deterministic generator.
    struct Lcg(u64);
    impl Lcg {
        fn f(&mut self) -> f64 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
        }
        fn r(&mut self, lo: f64, hi: f64) -> f64 {
            lo + (hi - lo) * self.f()
        }
    }

    fn seg_dist(x: Point3<f64>, p: Point3<f64>, q: Point3<f64>) -> f64 {
        let d = q - p;
        let t = ((x - p).dot(d) / d.dot(d)).clamp(0.0, 1.0);
        (x - (p + d * t)).norm()
    }

    fn ellipse(c: Point3<f64>, a: f64, b: f64, rot: f64) -> Ellipse<f64> {
        Ellipse::new(
            Frame::new(c, Vec3::unit_z(), Vec3::new(rot.cos(), rot.sin(), 0.0)),
            a,
            b,
        )
    }

    fn min_seg(e: &Ellipse<f64>, (t0, t1): (f64, f64), p: Point3<f64>, q: Point3<f64>) -> f64 {
        let n = 20000;
        (0..=n)
            .map(|i| seg_dist(e.point_at(t0 + (t1 - t0) * i as f64 / n as f64), p, q))
            .fold(f64::INFINITY, f64::min)
    }

    /// Random segments against random (often very eccentric) arcs,
    /// spans up to a full turn: a "clear" is never a sampled meeting.
    #[test]
    fn a_clear_segment_never_meets_the_arc() {
        let mut g = Lcg(7436);
        let (mut clear, mut false_clear, mut near) = (0, 0, f64::INFINITY);
        for _ in 0..20000 {
            let a = g.r(0.01, 10.0);
            let b = a * [1.0, g.r(0.001, 1.0), 1e-3][(g.f() * 3.0) as usize % 3];
            let e = ellipse(Point3::new(g.r(-1.0, 1.0), g.r(-1.0, 1.0), 0.0), a, b, g.r(0.0, 6.3));
            let t0 = g.r(-7.0, 7.0);
            let span = (t0, t0 + g.r(0.01, std::f64::consts::TAU));
            let s = 1.5 * a;
            let p = Point3::new(g.r(-s, s), g.r(-s, s), 0.0);
            let q = Point3::new(g.r(-s, s), g.r(-s, s), 0.0);
            if arc_seg(&e, span, p, q, band()) {
                clear += 1;
                let m = min_seg(&e, span, p, q);
                near = near.min(m);
                if m < 1e-12 {
                    false_clear += 1;
                }
            }
        }
        eprintln!("segment: {clear} clear, {false_clear} false, nearest clear {near:e}");
        assert_eq!(false_clear, 0);
        assert!(clear > 1000, "too few clears to mean anything: {clear}");
    }

    /// Tangent and endpoint-sharing segments, and a near touch either
    /// side of the band.
    #[test]
    fn tangent_shared_and_near_touch() {
        let e = ellipse(Point3::origin(), 5.0, 0.05, 0.0);
        let span = (0.3, 2.8);
        // Tangent at the top of the minor axis, y = 0.05.
        let tan = (Point3::new(-1.0, 0.05, 0.0), Point3::new(1.0, 0.05, 0.0));
        assert!(!arc_seg(&e, span, tan.0, tan.1, band()), "tangent");
        // A segment from the arc's start point outward.
        let p0 = e.point_at(0.3);
        let out = (p0, p0 + Vec3::new(1.0, 1.0, 0.0));
        assert!(!arc_seg(&e, span, out.0, out.1, band()), "shared end");
        for gap in [1e-12, 1e-10, 1e-8, 1e-6, 1e-4] {
            let s = (
                Point3::new(-1.0, 0.05 + gap, 0.0),
                Point3::new(1.0, 0.05 + gap, 0.0),
            );
            eprintln!("gap {gap:e}: clear {}", arc_seg(&e, span, s.0, s.1, band()));
        }
        // A circle.
        let c = ellipse(Point3::origin(), 1.0, 1.0, 0.0);
        let s = (Point3::new(-2.0, 1.0 + 1e-6, 0.0), Point3::new(2.0, 1.0 + 1e-6, 0.0));
        eprintln!("circle 1e-6 above: clear {}", arc_seg(&c, (0.0, 6.0), s.0, s.1, band()));
        let s = (Point3::new(-2.0, 0.5, 0.0), Point3::new(2.0, 0.5, 0.0));
        assert!(!arc_seg(&c, (0.0, 6.0), s.0, s.1, band()), "a chord");
        // A chord the span > π skips.
        assert!(arc_seg(&c, (std::f64::consts::PI + 0.2, std::f64::consts::TAU - 0.2), s.0, s.1, band()), "below-half arc clears a chord above");
    }

    /// Random arc pairs: a "clear" is never a sampled meeting.
    #[test]
    fn a_clear_arc_pair_never_meets() {
        let mut g = Lcg(9);
        let (mut clear, mut false_clear) = (0, 0);
        for _ in 0..4000 {
            let mk = |g: &mut Lcg| {
                let a = g.r(0.05, 3.0);
                let b = a * [1.0, g.r(0.01, 1.0), 2e-3][(g.f() * 3.0) as usize % 3];
                let t0 = g.r(-7.0, 7.0);
                (
                    ellipse(Point3::new(g.r(-1.0, 1.0), g.r(-1.0, 1.0), 0.0), a, b, g.r(0.0, 6.3)),
                    (t0, t0 + g.r(0.01, std::f64::consts::TAU)),
                )
            };
            let (c, s) = mk(&mut g);
            let (d, t) = mk(&mut g);
            if arc_arc(&c, s, &d, t, band()) {
                clear += 1;
                let n = 1500;
                let pd: Vec<_> = (0..=n).map(|j| d.point_at(t.0 + (t.1 - t.0) * j as f64 / n as f64)).collect();
                let (mut m, mut bi, mut bj) = (f64::INFINITY, 0usize, 0usize);
                for i in 0..=n {
                    let x = c.point_at(s.0 + (s.1 - s.0) * i as f64 / n as f64);
                    for (j, y) in pd.iter().enumerate() {
                        let g = (x - *y).norm();
                        if g < m { m = g; bi = i; bj = j; }
                    }
                }
                // Zoom on the nearest pair.
                let (mut lo_s, mut hi_s) = (s.0 + (s.1 - s.0) * (bi.saturating_sub(2)) as f64 / n as f64, s.0 + (s.1 - s.0) * ((bi + 2).min(n)) as f64 / n as f64);
                let (mut lo_t, mut hi_t) = (t.0 + (t.1 - t.0) * (bj.saturating_sub(2)) as f64 / n as f64, t.0 + (t.1 - t.0) * ((bj + 2).min(n)) as f64 / n as f64);
                for _ in 0..12 {
                    let k = 200;
                    let (mut bm, mut bu, mut bv) = (f64::INFINITY, 0.0, 0.0);
                    for i in 0..=k {
                        let u = lo_s + (hi_s - lo_s) * i as f64 / k as f64;
                        let x = c.point_at(u);
                        for j in 0..=k {
                            let v = lo_t + (hi_t - lo_t) * j as f64 / k as f64;
                            let g = (x - d.point_at(v)).norm();
                            if g < bm { bm = g; bu = u; bv = v; }
                        }
                    }
                    m = m.min(bm);
                    let (ws, wt) = ((hi_s - lo_s) / 20.0, (hi_t - lo_t) / 20.0);
                    lo_s = (bu - ws).max(s.0.min(s.1)); hi_s = (bu + ws).min(s.0.max(s.1));
                    lo_t = (bv - wt).max(t.0.min(t.1)); hi_t = (bv + wt).min(t.0.max(t.1));
                }
                // Sampling 1500 points each: a crossing reads below the sample spacing.
                let spacing = 3.0 * std::f64::consts::TAU / n as f64;
                if m < 1e-9 {
                    false_clear += 1;
                    eprintln!("FALSE CLEAR: {m:e} c {:?} {:?} {s:?} d {:?} {:?} {t:?}", c.frame(), c.semi_axes(), d.frame(), d.semi_axes());
                }
            }
        }
        eprintln!("arc pairs: {clear} clear, {false_clear} suspect");
        assert_eq!(false_clear, 0);
    }
}

#[cfg(test)]
#[allow(clippy::panic, clippy::unwrap_used)]
mod review_nesting_threshold {
    use super::review_nesting_probes::arc_seg;
    #[allow(unused_imports)] use super::edges_clear;
    use geom::Curve3;
    use geom::{Conic, Ellipse, Frame};
    use geom_core::{Band, Point3, Vec3};

    /// The least gap a segment parallel to the major axis, above the
    /// minor vertex, clears at; and the old carrier reading's verdict.
    #[test]
    fn threshold_above_a_vertex() {
        let band = Band::linear(geom_core::Tol::witness()).unwrap();
        for (a, b) in [(1.0, 1.0), (1.0, 0.5), (5.0, 0.05), (10.0, 1.0)] {
            let e = Ellipse::new(Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()), a, b);
            let mut first = None;
            for k in 0..60 {
                let gap = 10f64.powf(-12.0 + k as f64 * 0.2);
                let s = (Point3::new(-0.5 * a, b + gap, 0.0), Point3::new(0.5 * a, b + gap, 0.0));
                if arc_seg(&e, (0.3, 2.8), s.0, s.1, band) {
                    first = Some(gap);
                    break;
                }
            }
            eprintln!("a {a} b {b}: first clear gap {first:?}; band {band:?}");
        }
        let _ = Conic::point_at(&Ellipse::new(Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()), 1.0, 1.0), 0.0);
    }
}
