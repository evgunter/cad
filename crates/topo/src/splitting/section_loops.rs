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

use geom_core::{Decide, Indeterminate, Point3, Real, Vec3};

use super::PlaneSide;
use super::containment::{LoopContainment, point_in_carrier_loop};
use crate::body::Body;
use crate::chord_join::ring_representative;
use crate::entity::{LoopBoundary, LoopKey};
use crate::validate::{RingOuterVerdict, ring_outer_contact_about};

/// A traversal of a section loop met a dangling key: the scratch body
/// is torn (a kernel bug).
#[derive(Clone, Copy, Debug)]
pub(super) struct Torn;

/// The outward normal of `side`'s section face, for the split plane's
/// normal `n` (module docs): `−n` above, `+n` below.
pub(super) fn section_normal<T: Real>(n: Vec3<T>, side: PlaneSide) -> Vec3<T> {
    match side {
        PlaneSide::Above => -n,
        _ => n,
    }
}

/// The in-plane `u` axis a section loop is charted with: its first
/// chord, normalized — deterministic data, no comparisons. `None` for
/// a loop of fewer than two corners.
pub(super) fn chord_u_ref<T: Real>(points: &[Point3<T>]) -> Option<Vec3<T>> {
    match points {
        [a, b, ..] => Some((*b - *a).normalize()),
        _ => None,
    }
}

/// Why a loop's role could not be read.
#[derive(Debug)]
pub(super) enum SenseFault {
    /// [`Torn`].
    Torn,
    /// The winding has no sign: in the band (`Some`), zero, or (`None`)
    /// a loop with an edge that states no certified curve.
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
pub(super) fn loop_sense<T: Decide>(
    body: &Body<T>,
    l: LoopKey,
    normal: Vec3<T>,
    band: geom_core::Band,
) -> Result<bool, SenseFault> {
    match body
        .planar_loop_winding(l, normal, band)
        .map_err(|_| SenseFault::Torn)?
    {
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
    /// [`Torn`].
    Torn,
    /// Two outlines each read as enclosing the other around this hole:
    /// their outlines were decided disjoint, and disjoint outlines
    /// cannot (a kernel bug).
    Contradiction(H),
}

impl<H> From<Torn> for NestFault<H> {
    fn from(Torn: Torn) -> Self {
        Self::Torn
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
/// ([`point_in_carrier_loop`]). Disjoint outlines nest or are apart
/// (Jordan), so among several enclosing outlines — an island in a hole
/// in a face — exactly one is enclosed by all the others, and the hole
/// goes to it; two such would be two outlines each enclosing the other,
/// which disjoint outlines cannot be ([`NestFault::Contradiction`]).
///
/// **What decides nothing**, leaving a hole [`Nesting::unplaced`]: an
/// outline edge on a spiric or NURBS carrier, whose contacts nothing
/// here decides; a containment or contact reading in the band; and the
/// clockwise polygons the join mints when it chords a curved face
/// across the wrong arc, which touch the outline around them
/// (`work/cleave/split-pairs-curved-face-crossings-across-the-wrong-arc.md`).
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
        let q = ring_representative(body, inner).map_err(|_| Torn)?;
        Ok(matches!(
            point_in_carrier_loop(body, outer, normal, q, band),
            Ok(Some(LoopContainment::In))
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

/// Whether two loops in the section plane, with `normal`, are DECIDED
/// disjoint: `true` only where every pair of their edges is.
///
/// - **Line and circle edges**: tier 3's check 9 contact reading
///   ([`ring_outer_contact_about`]), which on loops in one plane that
///   carry only these kinds decides every shared point — at a vertex of
///   either loop, along an arc, or where two edges cross.
/// - **A pair with an ellipse edge**, which check 9's edge arms skip:
///   separated on the CARRIERS, a superset of the arcs. A line against
///   a conic: the conic's whole carrier lies on one side of the line
///   through the segment (**`split_nest_line_conic`**, the offset of
///   the conic's centre from the line less its amplitude across it,
///   `√((m·a)² + (m·b)²)`). Two conics: in the unit coordinates of one,
///   the other's carrier lies wholly inside or wholly outside the unit
///   circle, either way round (**`split_nest_conic_conic`**; the bound
///   is at [`conics_clear`]).
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
        ring_outer_contact_about(body, a, b, Some(normal), band),
        RingOuterVerdict::Disjoint
    ) {
        return Ok(false);
    }
    let (ea, eb) = (loop_edges(body, a)?, loop_edges(body, b)?);
    for x in &ea {
        for y in &eb {
            let separated = match (x, y) {
                (OutlineEdge::Undecided, _) | (_, OutlineEdge::Undecided) => return Ok(false),
                (OutlineEdge::Line(p, q), OutlineEdge::Conic(c, true))
                | (OutlineEdge::Conic(c, true), OutlineEdge::Line(p, q)) => {
                    line_clears_conic(*p, *q, c, band)
                }
                (OutlineEdge::Conic(c, true), OutlineEdge::Conic(d, _))
                | (OutlineEdge::Conic(c, _), OutlineEdge::Conic(d, true)) => {
                    conics_clear(c, d, band)
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

/// One outline edge, as [`outlines_disjoint`] reads it: a segment by
/// its ends; a conic by its carrier, flagged `true` for an ellipse
/// (the kind check 9's edge arms skip); or a kind nothing decides.
enum OutlineEdge<T: Real> {
    Line(Point3<T>, Point3<T>),
    Conic(Conic<T>, bool),
    Undecided,
}

/// A conic carrier: centre and the two semi-axis vectors.
struct Conic<T: Real> {
    centre: Point3<T>,
    a: Vec3<T>,
    b: Vec3<T>,
}

fn loop_edges<T: Decide>(body: &Body<T>, l: LoopKey) -> Result<Vec<OutlineEdge<T>>, Torn> {
    let corrupt = || Torn;
    // A lone-vertex loop bounds nothing a contact reading could clear.
    let first = match body.get_loop(l).ok_or_else(corrupt)?.boundary {
        LoopBoundary::Cycle { first } => first,
        LoopBoundary::Empty { .. } => return Ok(vec![OutlineEdge::Undecided]),
    };
    let mut out = Vec::new();
    for he in body.loop_cycle(first).ok_or_else(corrupt)? {
        let h = body.get_half_edge(he).ok_or_else(corrupt)?;
        let edge = body.get_edge(h.edge).ok_or_else(corrupt)?;
        let Some(curve) = body
            .get_curve_geom(edge.curve)
            .and_then(crate::null::CurveGeom::certified)
        else {
            out.push(OutlineEdge::Undecided);
            continue;
        };
        out.push(match *curve.carrier() {
            geom::Curve3::Line { .. } => {
                let end = body.half_edge_end(he).ok_or_else(corrupt)?;
                let point = |v| -> Result<Point3<T>, Torn> {
                    let p = body.get_vertex(v).ok_or_else(corrupt)?.point;
                    body.get_point(p).copied().ok_or_else(corrupt)
                };
                OutlineEdge::Line(point(h.start)?, point(end)?)
            }
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => OutlineEdge::Conic(
                Conic {
                    centre: center,
                    a: u_ref * radius,
                    b: axis.cross(u_ref) * radius,
                },
                false,
            ),
            geom::Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => OutlineEdge::Conic(
                Conic {
                    centre: center,
                    a: u_ref * major,
                    b: axis.cross(u_ref) * minor,
                },
                true,
            ),
            geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => OutlineEdge::Undecided,
        });
    }
    Ok(out)
}

/// The conic's whole carrier lies definitely on one side of the line
/// through `p`, `q` (both in the conic's plane).
fn line_clears_conic<T: Decide>(
    p: Point3<T>,
    q: Point3<T>,
    c: &Conic<T>,
    band: geom_core::Band,
) -> bool {
    let along = q - p;
    if !positive("split_nest_line_conic", along.norm(), band)
        || !positive("split_nest_line_conic", c.a.norm().min(c.b.norm()), band)
    {
        return false;
    }
    let m = c.a.cross(c.b).normalize().cross(along.normalize());
    let reach = (m.dot(c.a).powi(2) + m.dot(c.b).powi(2)).sqrt();
    positive(
        "split_nest_line_conic",
        m.dot(c.centre - p).abs() - reach,
        band,
    )
}

/// `margin` (metres) definitely positive under `band`.
fn positive<T: Decide>(name: &'static str, margin: T, band: geom_core::Band) -> bool {
    matches!(
        crate::validate::decide(name, geom_core::Margin::of(margin), band),
        Ok(geom_core::Sign::Positive)
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
fn conics_clear<T: Decide>(e: &Conic<T>, h: &Conic<T>, band: geom_core::Band) -> bool {
    let (ae, be) = (e.a.norm(), e.b.norm());
    let lever = ae.min(be);
    if !positive("split_nest_conic_conic", lever, band) {
        return false;
    }
    let unit = |v: Vec3<T>| (v.dot(e.a) / ae.powi(2), v.dot(e.b) / be.powi(2));
    let (cx, cy) = unit(h.centre - e.centre);
    let (ax, ay) = unit(h.a);
    let (bx, by) = unit(h.b);
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
