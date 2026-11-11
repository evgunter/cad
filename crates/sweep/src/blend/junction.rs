//! **A junction between two links on different supports** — where a
//! chain's band passes from one link's surface to the next one's along
//! the rolling ball's section the two share (predicate 4's band
//! reading). It is one shape in either carve that meets it, the closed
//! mixed rim ([`super::surgery`]'s `rim_phase_mixed`) and the open band
//! ([`super::open::planar`]'s `blank_phase`), and this module is the one
//! home of what it is and of its moves:
//!
//! - what a junction is, read once for both admissions ([`shape`]);
//! - where its feet are, and the section between them ([`read`]);
//! - the split of the edge between its two mates at the mate foot
//!   ([`split_mate`], over [`split_at`]);
//! - the host strut ([`strut_foot`]);
//! - the `kev` that retires the strut and the junction vertex, re-basing
//!   the split's piece into the section ([`retire`]);
//! - what both carves record of it ([`record`], over [`record_feet`]);
//! - each link's trims ([`mixed_trims`], over the rim arc's own frame,
//!   [`rim_carrier`]) and band chart ([`band_chart`]);
//! - a straight link's feet in order beside one ([`feet_clear`]).
//!
//! The closed rims' other walks make the same moves at their crossings
//! and call these: the annulus splits a mate seam with [`split_at`],
//! retires each crossing and its closure with [`retire`] and records its
//! feet with [`record_feet`]; the ladder splits each meridian with
//! [`split_at`] and its closure is a [`retire`]. The ladder records its
//! feet as it mints them, a strut's foot at the strut and a meridian's
//! split at the split, since its two are minted in two passes.
//!
//! The HOST of a junction is the one support its two links share and
//! each link's MATE is its other support; the edge between the two
//! mates runs off the chain at the junction vertex.
//!
//! What it reads of [`super::surgery`] is the vocabulary every carve
//! shares — the refusal constructors, the body lookups, the split
//! provenance ([`split_fragment`]) and the description carriers — and
//! nothing junction-shaped lives there.

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Bounds, Decide, Margin, Point3, Real, Tol, Vec3};
use topo::construction::{Construct, MevSite};
use topo::{Body, EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};

use super::BlendError;
use super::admit::{RequestedBoundary, StationKind};
use super::arms::{BallSection, BlendArm, EdgeBlend, torus_radial};
use super::battery::{Link, same_supports};
use super::build::{fan_at, outward_of};
use super::naming::BlendNaming;
use super::open::end_face::feet_in_order;
use super::surgery::{
    ContactCarrier, Described, ExpectedSource, SplitFragments, certified_curve, disagrees,
    halves_of, host_side_half, op, point_of, seam_split_param, split_fragment, stored_piece,
    unbuilt_chain, unbuilt_geometry, unresolved, unwalked,
};

/// **What a junction is, read once for every admission**: the face its
/// two links share (the HOST), each link's other support (its MATE,
/// arriving then leaving), and the one edge off the chain at the vertex,
/// between the two mates.
#[derive(Clone, Copy, Debug)]
pub(super) struct Shape {
    pub(super) host: FaceKey,
    pub(super) mates: [FaceKey; 2],
    pub(super) mate_edge: EdgeKey,
}

/// A junction's link is neither of the two bands with a section.
pub(super) const JUNCTION_UNSECTIONED: &str = "a chain's links meet on different supports where \
     one is neither a rolling-ball cylinder over two planes nor a torus; that junction is not \
     implemented";
/// Two consecutive links on one support pair among links on others.
pub(super) const JUNCTION_ON_ONE_PAIR: &str = "a chain of links on different supports runs two \
     consecutive links on one support pair; that run inside it is not implemented";
/// The two links share no one face.
pub(super) const JUNCTION_NO_HOST: &str =
    "a chain's links meet at a junction on supports that share no one face";
/// The shared face is curved.
pub(super) const JUNCTION_HOST_CURVED: &str =
    "a chain's links meet at a junction where the face they share is not a plane";
/// Other than one edge off the chain.
pub(super) const JUNCTION_VALENCE: &str =
    "a chain's junction carries other than one edge off the chain";
/// The edge off the chain is not between the mates.
pub(super) const JUNCTION_EDGE_ASTRAY: &str =
    "a chain's junction edge does not separate the junction's two other supports";

/// **Read the junction at `vertex` between `arriving` and `leaving`**:
/// each a rolling-ball band with a section (a cylinder over two planes
/// or a torus); the two on different supports sharing exactly one face,
/// a plane, on which the carve spins its strut; the vertex carrying one
/// edge besides the two links, separating their other supports. `named`
/// is the edge a refusal names.
///
/// # Errors
///
/// [`BlendError::UnsupportedChain`] with one of the `JUNCTION_*` details;
/// [`BlendError::BodyNotIntact`] when the orbit or the edge does not
/// resolve.
pub(super) fn shape<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    arriving: &Link<T>,
    leaving: &Link<T>,
    named: EdgeKey,
) -> Result<Shape, BlendError> {
    let sectioned = |l: &Link<T>| l.arm == BlendArm::PlanePlaneCylinder || l.arm.is_coaxial_torus();
    if !(sectioned(arriving) && sectioned(leaving)) {
        return Err(unbuilt_chain(named, JUNCTION_UNSECTIONED));
    }
    if same_supports(body, arriving, leaving)? {
        return Err(unbuilt_chain(named, JUNCTION_ON_ONE_PAIR));
    }
    let shared: Vec<FaceKey> = [arriving.face_a, arriving.face_b]
        .into_iter()
        .filter(|&f| f == leaving.face_a || f == leaving.face_b)
        .collect();
    let [host] = shared[..] else {
        return Err(unbuilt_chain(named, JUNCTION_NO_HOST));
    };
    if outward_of(body, host).is_none() {
        return Err(unbuilt_chain(named, JUNCTION_HOST_CURVED));
    }
    let mate = |l: &Link<T>| if l.face_a == host { l.face_b } else { l.face_a };
    let mates = [mate(arriving), mate(leaving)];
    let incident = fan_at(body.edges_of_vertex(vertex))
        .ok_or_else(|| unwalked(EntityId::Vertex(vertex), "a junction's edge orbit"))?;
    let others: Vec<EdgeKey> = incident
        .into_iter()
        .filter(|&e| e != arriving.edge && e != leaving.edge)
        .collect();
    let [mate_edge] = others[..] else {
        return Err(unbuilt_chain(named, JUNCTION_VALENCE));
    };
    let sides = topo::readback::edge_sides(body, mate_edge)
        .map_err(|_| unresolved(EntityId::Edge(mate_edge), "a junction edge's two faces"))?;
    let mut faces = [sides.plus.face, sides.minus.face];
    faces.sort_unstable();
    let mut pair = mates;
    pair.sort_unstable();
    if faces != pair {
        return Err(unbuilt_chain(named, JUNCTION_EDGE_ASTRAY));
    }
    Ok(Shape {
        host,
        mates,
        mate_edge,
    })
}

/// One support's trimline of one link beside a junction, run the way
/// that support's loop runs along the link.
pub(super) enum MixedTrim<T: Real> {
    /// A cylinder band's straight trim, between the feet the carve
    /// mints: the carrier is the line through them.
    Line(geom::Line<T>),
    /// A torus band's trim circle in the rim arc's own frame, scaled
    /// ([`scaled`]), over the arc's window.
    Arc(Curve3<T>, (T, T)),
}

impl<T: Decide> MixedTrim<T> {
    /// The foot on this trim at a junction `p`, read from the end of
    /// the run `at_start` names: a straight trim's foot of `p`, an arc's
    /// window end.
    pub(super) fn foot(&self, p: Point3<T>, at_start: bool) -> Point3<T> {
        match self {
            Self::Line(line) => line.foot(p),
            Self::Arc(c, (t0, t1)) => c.eval(if at_start { *t0 } else { *t1 }),
        }
    }

    /// The scaffolding the trim's `mef` stores between its two feet.
    pub(super) fn spec(&self, p0: Point3<T>, p1: Point3<T>) -> Option<EdgeCurveSpec<T>> {
        match self {
            Self::Line(_) => Some(EdgeCurveSpec::line_between(p0, p1)),
            Self::Arc(c, (t0, t1)) => EdgeCurveSpec::arc_of_circle(c.clone(), *t0, *t1),
        }
    }

    /// The description the pass states it as.
    pub(super) fn contact(&self) -> ContactCarrier<T> {
        match self {
            Self::Line(line) => ContactCarrier::TrimLine(*line),
            Self::Arc(c, (t0, t1)) => ContactCarrier::Exact(c.clone(), *t0, *t1),
        }
    }
}

/// A link's two trims, host then mate, each run with its own loop.
pub(super) fn mixed_trims<T: Decide>(
    body: &Body<T>,
    l: &Link<T>,
    host: FaceKey,
) -> Result<(MixedTrim<T>, MixedTrim<T>), BlendError> {
    if l.arm.is_coaxial_torus() {
        let ((ca, sa), (cb, sb)) = rim_trim_circles(l.edge, &l.blend, l.face_a == host)?;
        let rc = rim_carrier(body, l, host)?;
        let (hc, h0, h1) = scaled(&rc, ca, sa, rc.plus_on_host);
        let (mc, m0, m1) = scaled(&rc, cb, sb, !rc.plus_on_host);
        return Ok((MixedTrim::Arc(hc, (h0, h1)), MixedTrim::Arc(mc, (m0, m1))));
    }
    let ((ht, _), (mt, _)) = l.blend.trims(l.face_a == host);
    let line = |c: &Curve3<T>| match c {
        Curve3::Line(line) => Ok(MixedTrim::Line(*line)),
        _ => Err(unbuilt_geometry(
            EntityId::Edge(l.edge),
            "a cylinder band's trimline on a plane is not a line",
        )),
    };
    Ok((line(ht)?, line(mt)?))
}

/// **A junction as the plan reads it**: the band's foot on the host,
/// its foot on the edge between the mates, and the section between them.
#[derive(Clone, Copy)]
pub(super) struct JunctionReading<T: Real> {
    pub(super) host_foot: Point3<T>,
    pub(super) mate_foot: Point3<T>,
    pub(super) section: BallSection<T>,
}

/// **Read the junction at `p`** off `reader`, the link whose host-side
/// traversal STARTS there, with its `trims` ([`mixed_trims`]), and the
/// other link. The section's plane is stated from a cylinder band where
/// the junction has one, whose axis is the stored direction itself.
///
/// # Errors
///
/// [`BlendError::UnsupportedGeometry`] for a band with no rolling-ball
/// section.
pub(super) fn read<T: Decide>(
    p: Point3<T>,
    reader: &Link<T>,
    trims: &(MixedTrim<T>, MixedTrim<T>),
    other: &Link<T>,
) -> Result<JunctionReading<T>, BlendError> {
    let of = |l: &Link<T>| {
        BallSection::of(&l.blend, p).ok_or_else(|| {
            unbuilt_geometry(
                EntityId::Edge(l.edge),
                "a junction's band has no rolling-ball section",
            )
        })
    };
    let mut section = of(reader)?;
    if matches!(other.blend.surface, Surface::Cylinder { .. }) {
        section.normal = of(other)?.normal;
    }
    Ok(JunctionReading {
        host_foot: trims.0.foot(p, true),
        mate_foot: trims.1.foot(p, false),
        section,
    })
}

/// **The surface a link's band face carries beside a junction.** A
/// torus face's chart is seamed opposite the middle of its arc, and a
/// cylinder face's opposite the middle of its section (`at`), so no face
/// boundary runs along a chart seam; a cylinder read at no junction
/// keeps its arm's chart.
///
/// # Errors
///
/// [`stored_piece`]'s, for a torus link's arc.
pub(super) fn band_chart<T: Decide>(
    body: &Body<T>,
    l: &Link<T>,
    at: Option<&JunctionReading<T>>,
) -> Result<Surface<T>, BlendError> {
    Ok(match (&l.blend.surface, at) {
        (
            &Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                ..
            },
            _,
        ) => {
            let (c, (t0, t1)) = stored_piece(body, l.edge)?;
            let radial = torus_radial(center, axis, c.eval(geom::mid_param(t0, t1)));
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref: -radial,
            }
        }
        (
            &Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            },
            Some(r),
        ) => {
            let mid = (r.host_foot - r.section.centre) + (r.mate_foot - r.section.centre);
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref: -(mid - axis * mid.dot(axis)).normalize(),
            }
        }
        (s, _) => s.clone(),
    })
}

/// **Split `edge` beside `vertex` where the band's mate trim crosses it**,
/// at `target`, recording its pieces against the source the caller knows
/// (`expected`, where it knows one): the parameter is
/// [`seam_split_param`]'s, the split and its provenance
/// [`split_fragment`]'s. A junction's mate edge, an annulus crossing's
/// mate seam and a ladder vertex's meridian are split so. `named` is
/// the link a refusal names.
///
/// # Errors
///
/// [`seam_split_param`]'s and [`split_fragment`]'s.
#[allow(clippy::too_many_arguments)] // the split's own inputs, at every carve.
pub(super) fn split_at<T: Decide + Bounds + topo::AtRestPolicy>(
    body: &mut Body<T>,
    edge: EdgeKey,
    vertex: VertexKey,
    named: EdgeKey,
    target: Point3<T>,
    expected: Option<ExpectedSource>,
    rec: &mut BlendNaming,
    site: &'static str,
    tol: Tol,
) -> Result<SplitFragments, BlendError> {
    let t = seam_split_param(body, edge, named, target)?;
    split_fragment(body, edge, vertex, t, expected, rec, site, tol)
}

/// **Split the edge between a junction's two mates at the mate foot**
/// ([`split_at`]), read LIVE off the vertex: another band of the same
/// call carving the same mates (the other rim of an extruded wall) has
/// split it already, and its fragment is what meets this junction.
/// `chain` is every edge of the chain at `vertex`, so the one other edge
/// there is the mate edge; `named` is the link a refusal names.
///
/// # Errors
///
/// [`BlendError::BodyNotIntact`] when the vertex carries other than one
/// edge off the chain; [`split_at`]'s.
#[allow(clippy::too_many_arguments)] // the split's own inputs, at both carves.
pub(super) fn split_mate<T: Decide + Bounds + topo::AtRestPolicy>(
    body: &mut Body<T>,
    vertex: VertexKey,
    chain: &[EdgeKey],
    named: EdgeKey,
    mate_foot: Point3<T>,
    expected: ExpectedSource,
    rec: &mut BlendNaming,
    tol: Tol,
) -> Result<SplitFragments, BlendError> {
    let incident = fan_at(body.edges_of_vertex(vertex))
        .ok_or_else(|| unwalked(EntityId::Vertex(vertex), "a junction's edge orbit"))?;
    let off: Vec<EdgeKey> = incident
        .into_iter()
        .filter(|e| !chain.contains(e))
        .collect();
    let [live] = off[..] else {
        return Err(disagrees(
            EntityId::Vertex(vertex),
            "a junction carries other than one edge off its chain at the carve",
        ));
    };
    split_at(
        body,
        live,
        vertex,
        named,
        mate_foot,
        Some(expected),
        rec,
        "junction split",
        tol,
    )
}

/// **Retire a crossing**: the vertex carries the edge `dying` from the
/// host foot and the split's `near` piece; `dying` is killed from the
/// foot, the vertex with it, and `near` re-bases onto the host foot,
/// where it spans the two feet ([`merged_chord_spec`]). At a junction,
/// once both links' strips are merged into their band faces, `dying` is
/// the strut and `near` becomes the section; at an annulus crossing or a
/// ladder's closure `dying` is the host's foot edge and `near` becomes
/// the mate's piece across the band, or its slit. `site` names the move
/// in an operator refusal.
///
/// # Errors
///
/// [`BlendError::Op`] from the `kev`.
pub(super) fn retire<T: Decide + topo::AtRestPolicy>(
    body: &mut Body<T>,
    dying: EdgeKey,
    vertex: VertexKey,
    near: EdgeKey,
    site: &'static str,
    tol: Tol,
) -> Result<(), BlendError> {
    let (sp, sm) = halves_of(body, dying)
        .ok_or_else(|| unresolved(EntityId::Edge(dying), "a crossing's foot edge"))?;
    let half = if body.half_edge_end(sm) == Some(vertex) {
        sm
    } else {
        sp
    };
    let chord = merged_chord_spec(body, near, half, site)?;
    body.kev_describing(half, &[(near, chord)], tol)
        .map_err(|e| op(site, e))?;
    Ok(())
}

/// **A crossing's two feet, recorded**: the host foot (`RimFoot`) and
/// the mate split (a split of the source edge, under the band's identity
/// `band`).
pub(super) fn record_feet(
    rec: &mut BlendNaming,
    split: &SplitFragments,
    host_foot: VertexKey,
    vertex: VertexKey,
    band: &[EdgeKey],
) {
    rec.rim_feet.push((host_foot, vertex));
    rec.meridian_splits
        .push((split.vertex, split.source, band.to_vec()));
}

/// **What a retired junction leaves, recorded**: the section described
/// as the shared row of its two bands' charts, under `link`; its feet
/// ([`record_feet`]); the section (`BandSection`); and the dead junction
/// vertex.
#[allow(clippy::too_many_arguments)] // the junction's own rows, at both carves.
pub(super) fn record<T: Real>(
    described: &mut Described<T>,
    rec: &mut BlendNaming,
    split: &SplitFragments,
    host_foot: VertexKey,
    vertex: VertexKey,
    section: BallSection<T>,
    link: EdgeKey,
    band: &[EdgeKey],
) {
    described.push((split.near, ContactCarrier::Section(section), link));
    record_feet(rec, split, host_foot, vertex, band);
    rec.sections.push((split.near, split.source, vertex));
    rec.dead.vertices.push(vertex);
}

/// The refusal when a straight link is too short for its two ends.
pub(super) const JUNCTION_FEET_CROSS: &str = "a straight link between a junction and its \
     other end is shorter than its two ends' feet need: they cross or coincide on its trimline";

/// **A straight link's feet in order, on each support, where one of its
/// ends is a junction** (`fillet3_cut_off_feet`, decided by
/// [`feet_in_order`], as two cut-offs' feet on a shared rim are). The
/// trimline chord the carve `mef`s runs from the foot at one station to
/// the foot at the other; the junction's foot sits square to the
/// junction, and the other end's (a cut-off's on an oblique end face, a
/// turn's, a corner's) may sit further along the edge, so on a short
/// link the two would cross. A torus link's chords run between two
/// junctions' feet, its own trim's window ends, so it is not read here.
///
/// **It is what refuses a link too short for its ends.** Feet that
/// cross put the far end's foot behind the junction's, so the end face's
/// rim, which passes through that foot, runs within a setback of the
/// link on the junction's other side. Predicate 2's support screen does
/// not read that pair: the edge sits at the chain's end and is exempt
/// there, being metered in closed form by the plan (`battery::chain_apart`).
/// This meter is the plan's reading of it
/// (`band_open_mixed_chain_shapes::a_short_link_leaning_back_refuses_as_its_feet_cross`).
///
/// The two feet-order readings stay two: here the span is a distance
/// along a trimline, a line the plan holds; on a shared rim it is a
/// parameter span along a stored carrier, metered at its speed floor.
///
/// # Errors
///
/// [`BlendError::UnsupportedRunOut`] ([`JUNCTION_FEET_CROSS`]);
/// [`BlendError::Escalated`] in band.
pub(super) fn feet_clear<T: Decide + Bounds>(
    body: &Body<T>,
    supports: &[RequestedBoundary<T>],
    straight: impl Fn(EdgeKey) -> bool,
    band: Band,
) -> Result<(), BlendError> {
    let junction = |s: &RequestedBoundary<T>, v: VertexKey| {
        s.stations().iter().any(|st| {
            st.vertex == v
                && matches!(
                    st.kind,
                    StationKind::JunctionHost | StationKind::JunctionMate
                )
        })
    };
    for support in supports {
        for chord in support.chords() {
            if !straight(chord.edge)
                || !(junction(support, chord.from) || junction(support, chord.to))
            {
                continue;
            }
            let at = |v: VertexKey| {
                point_of(body, v).ok_or_else(|| unresolved(EntityId::Vertex(v), "a station"))
            };
            let d = (at(chord.to)? - at(chord.from)?).normalize();
            let span = (chord.feet[1] - chord.feet[0]).dot(d);
            feet_in_order(chord.edge, Margin::of(span), JUNCTION_FEET_CROSS, band)?;
        }
    }
    Ok(())
}

/// A rim link's (host, mate) trim circles as `(center, radius)` pairs,
/// selected by SUPPORT FACE — `surgery::resolve_rim`'s discipline — never by
/// slot order. `classify_arm` keys `trim_a` to `face_a`, and `face_a` is
/// whichever support carries `he_plus`, which the request does not
/// choose; the `(Sphere, Plane)` arm swaps its own trims for exactly
/// this reason. Callers pass `first_is_a = (link.face_a == face)` for
/// the support `face` whose trim they want FIRST — the link's host for
/// the (host, mate) pair, or either support to read its own trim;
/// reading `trim_a` blind would take the mate's trim for the host's on
/// exactly the links where the two disagree. Pinned by
/// `surgery::tests::trim_selection_is_by_support_kind`.
#[allow(clippy::type_complexity)]
pub(super) fn rim_trim_circles<T: Real>(
    edge: EdgeKey,
    blend: &EdgeBlend<T>,
    first_is_a: bool,
) -> Result<((Point3<T>, T), (Point3<T>, T)), BlendError> {
    let ((first_trim, _), (other_trim, _)) = blend.trims(first_is_a);
    let Curve3::Circle(geom::Circle {
        frame: geom::Frame { origin: pc, .. },
        radius: pr,
    }) = *first_trim
    else {
        return Err(unbuilt_geometry(
            EntityId::Edge(edge),
            "a rim blend's trimline on one of its supports is not a circle",
        ));
    };
    let Curve3::Circle(geom::Circle {
        frame: geom::Frame { origin: sc, .. },
        radius: sr,
    }) = *other_trim
    else {
        return Err(unbuilt_geometry(
            EntityId::Edge(edge),
            "a rim blend's trimline on one of its supports is not a circle",
        ));
    };
    Ok(((pc, pr), (sc, sr)))
}

/// A rim edge's stored circle carrier, read once.
pub(super) struct RimCarrier<T: Real> {
    pub(super) axis: Vec3<T>,
    pub(super) u_ref: Vec3<T>,
    pub(super) t0: T,
    pub(super) t1: T,
    /// Whether the host-side half is the edge's `he_plus`.
    pub(super) plus_on_host: bool,
}

/// One rim edge's stored circle carrier, plus which side its `he_plus`
/// lies on. The band inherits this frame rather than reconstructing it,
/// which is what keeps every arc's seam structure and parameter window
/// the rim's own.
pub(super) fn rim_carrier<T: Decide>(
    body: &Body<T>,
    link: &Link<T>,
    host: FaceKey,
) -> Result<RimCarrier<T>, BlendError> {
    let e = link.edge;
    let ed = body
        .get_edge(e)
        .ok_or_else(|| unresolved(EntityId::Edge(e), "a rim edge"))?;
    let c = certified_curve(body, e)?;
    let host_half = host_side_half(body, link, host)
        .ok_or_else(|| unresolved(EntityId::Edge(e), "a rim edge's host-side half"))?;
    let Curve3::Circle(geom::Circle {
        frame: geom::Frame { axis, u_ref, .. },
        ..
    }) = *c.carrier()
    else {
        return Err(unbuilt_geometry(
            EntityId::Edge(e),
            "a rim edge's carrier is not a circle, the only rim carrier the band is built \
             for",
        ));
    };
    let (t0, t1) = c.params();
    Ok(RimCarrier {
        axis,
        u_ref,
        t0,
        t1,
        plus_on_host: ed.he_plus == host_half,
    })
}

/// The scaled trim carrier for the arc REPLACING a rim edge on one
/// side: same frame, same parameter window, oriented so `he_plus` runs
/// with that side's loop — reversed by [`Curve3::reversed`] over the
/// negated window, never by an endpoint `atan2` (π-arc safe).
pub(super) fn scaled<T: Real>(
    rc: &RimCarrier<T>,
    center: Point3<T>,
    radius: T,
    forward: bool,
) -> (Curve3<T>, T, T) {
    let circle = Curve3::circle(center, rc.axis, radius, rc.u_ref);
    if forward {
        (circle, rc.t0, rc.t1)
    } else {
        let back = circle
            .reversed()
            .unwrap_or_else(|| unreachable!("a circle reverses"));
        (back, -rc.t1, -rc.t0)
    }
}

/// **The band's STRUT foot on a host support** — the `mev` from a rim
/// vertex out to that support's trimline, and the ONE home of the move:
/// the ladder's struts at every vertex of its ring, the annulus's at a
/// crossing whose host side has no seam to split
/// (`surgery::HostFoot::Strut`), the mixed rim's at every junction, and
/// the planar open band's at every corner, joint, mitre and junction.
///
/// The foot `fp` is the SCALED rim carrier evaluated at the vertex's own
/// parameter, so the azimuth is inherited from the rim exactly rather
/// than reconstructed by an `atan2` — which is what makes the foot a
/// representation of the rim's own frame and not a measurement. It is
/// passed IN rather than evaluated here: the annulus phase already holds
/// it as the crossing's foot target, and evaluating it twice would let
/// the two copies drift apart under any later change to either site.
/// What is shared here is the MOVE, its site convention and its
/// scaffolding carrier.
///
/// **This `mev` leaves the body tier-2 INVALID until the caller's trim
/// `mef` lands** — the new foot is valence-1, which tier 2 bans as
/// scaffolding. The window is the ladder's own and both closed-rim
/// phases inherit it; validity is a claim about the body at REST, which
/// `surgery::blend_surgery`'s debug postcondition takes.
///
/// `MevSite::Fan { he1: he, he2: he }` splices both halves immediately
/// before `he`, so the host loop reads `strut(v→foot)`,
/// `strut(foot→v)`, `he` — a spur at the vertex. `he_minus` is
/// therefore the half STARTING at the foot, which is what both phases'
/// trim `mef` runs between.
///
/// The chord is scaffolding: a radial line between the two points,
/// upgraded to nothing later because a strut never survives the carve
/// (it dies at its crossing by `kef` or by the closure `kev`).
pub(super) fn strut_foot<T: Decide + Bounds + topo::AtRestPolicy>(
    body: &mut Body<T>,
    he: HalfEdgeKey,
    v: VertexKey,
    fp: Point3<T>,
    site: &'static str,
    tol: Tol,
) -> Result<topo::construction::MevCreated, BlendError> {
    let p = point_of(body, v).ok_or_else(|| unresolved(EntityId::Vertex(v), "a rim vertex"))?;
    let created = body
        .mev(
            MevSite::Fan { he1: he, he2: he },
            fp,
            EdgeCurveSpec::line_between(p, fp),
            tol,
        )
        .map_err(|e| op(site, e))?;
    Ok(created)
}

/// What a crossing's kill ([`retire`]) hands `Body::kev_describing` for the one
/// member its merge re-bases: the member as the chord between the
/// endpoints the merge gives it (`Body::kev_merged_members`) — the
/// scaffolding this surgery's struts and trims carry too, until the
/// description pass states the true carrier (`surgery::attach_contact`, over
/// the `described` list the member is on). The member spans two
/// distinct feet, so the chord is a line and never a closed circle.
/// A chord and not the meridian arc it will rest as: the arc's sweep
/// is an `atan2` over directions read off a centre that is itself
/// computed, and its scaffolding residual sets the carrier against a
/// rotation of its start point by that sweep, so the certified scalar
/// encloses the residual wider than the tightest band and the kill
/// escalates; the chord's carrier and its scaffold are one affine
/// expression each, and enclose it at a few units in the last place.
fn merged_chord_spec<T: Decide>(
    body: &Body<T>,
    member: EdgeKey,
    dying: HalfEdgeKey,
    site: &'static str,
) -> Result<EdgeCurveSpec<T>, BlendError> {
    let members = body.kev_merged_members(dying).map_err(|e| op(site, e))?;
    let merged = members.iter().find(|m| m.edge == member).ok_or_else(|| {
        disagrees(
            EntityId::Edge(member),
            "the meridian a closure kill's merge re-bases",
        )
    })?;
    Ok(EdgeCurveSpec::line_between(merged.start, merged.end))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use core::f64::consts::FRAC_PI_2;

    use geom_core::{Band, Point2, Tol, Vec3};
    use profile::{SketchPlane, test_support::bulge_loop};

    use super::super::BlendError;
    use super::super::admit::{
        AdmittedOpen, OpenBand, PlannedStation, RequestedBoundary, StationKind,
    };
    use super::super::battery::{BlendRequest, run_battery};
    use super::super::open::planar::{cut_off_plan, junction_plan};
    use super::{JUNCTION_FEET_CROSS, feet_clear};

    /// **[`feet_clear`] reads a straight link's two feet in order.** The
    /// L-plate's run (a line along `y = 0` cut off square at `x = 0`, a
    /// quarter arc of radius `0.3`, a line along `x = 2`) admits its top
    /// as planned; moved past the cut-off's foot, the junction's foot on
    /// the top crosses it and the meter refuses. The crossing is built
    /// into the plan here because predicate 2's screen answers first on
    /// every prism tried: an end face whose foot lands behind a
    /// junction's runs its rim through the next link's strip.
    #[test]
    fn a_junctions_foot_past_its_links_other_end_refuses() {
        let tol = Tol::witness();
        let q = (FRAC_PI_2 / 4.0).tan();
        let body = crate::test_support::extruded(
            SketchPlane::xy(),
            vec![bulge_loop(vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(1.7, 0.0), q),
                (Point2::new(2.0, 0.3), 0.0),
                (Point2::new(2.0, 1.0), 0.0),
                (Point2::new(1.0, 1.0), 0.0),
                (Point2::new(1.0, 2.0), 0.0),
                (Point2::new(0.0, 2.0), 0.0),
            ])],
            0.5,
            tol,
        );
        let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let mut edges: Vec<_> = topo::query::all_edges(&body)
            .into_iter()
            .filter(|&e| {
                let (a, b) = body.edge_vertices(e).unwrap();
                let (a, b) = (point(a), point(b));
                let (x, y) = (0.5 * (a.x + b.x), 0.5 * (a.y + b.y));
                a.z == 0.5 && b.z == 0.5 && (y == 0.0 || (x > 1.5 && y < 0.9))
            })
            .collect();
        edges.sort_unstable();
        assert_eq!(edges.len(), 3, "the line, the arc and the line");
        let band = Band::linear(tol).unwrap();
        let request = BlendRequest {
            body: &body,
            edges,
            size: 0.1,
        };
        let verdict = run_battery(&request, band).expect("the run's verdict");
        let [chain] = &verdict.chains[..] else {
            panic!("one chain, got {}", verdict.chains.len());
        };
        let open = OpenBand::admit(&body, chain).expect("the open door admits the run");
        let opens: Vec<AdmittedOpen<'_, f64>> = open.links().collect();
        let plans: Vec<_> = open
            .junctions()
            .iter()
            .map(|j| junction_plan(&body, j, &opens).unwrap())
            .collect();
        let cuts: Vec<_> = verdict
            .end_faces
            .iter()
            .map(|(v, section)| {
                let o = *opens
                    .iter()
                    .find(|o| o.link().start == *v || o.link().end == *v)
                    .unwrap();
                let c = cut_off_plan(&body, o, *v, section.clone()).unwrap();
                let l = o.link();
                PlannedStation {
                    vertex: *v,
                    on: vec![
                        (l.face_a, c.end.foot_a, StationKind::CutOff),
                        (l.face_b, c.end.foot_b, StationKind::CutOff),
                    ],
                }
            })
            .collect::<Vec<_>>();
        let host = open.junctions()[0].host();
        // The junction on the line along `y = 0`, at `x = 1.7`.
        let on_y0 = |p: &&super::super::open::planar::JunctionPlan<'_, f64>| {
            point(p.junction.vertex()).y == 0.0
        };
        let top = |shift: f64| {
            let mut planned: Vec<PlannedStation<f64>> = plans
                .iter()
                .map(|p| {
                    let mut foot = p.reading.host_foot;
                    if on_y0(&p) {
                        foot = foot - Vec3::new(shift, 0.0, 0.0);
                    }
                    let j = p.junction;
                    let mut on = vec![(j.host(), foot, StationKind::JunctionHost)];
                    on.extend(
                        j.mates()
                            .map(|f| (f, p.reading.mate_foot, StationKind::JunctionMate)),
                    );
                    PlannedStation {
                        vertex: j.vertex(),
                        on,
                    }
                })
                .collect();
            planned.extend(cuts.iter().map(|c| PlannedStation {
                vertex: c.vertex,
                on: c.on.clone(),
            }));
            RequestedBoundary::admit(&body, host, &opens, &planned)
                .expect("the top admits its stations")
        };
        let straight = |e| {
            opens
                .iter()
                .any(|o| o.edge() == e && !o.link().arm.is_coaxial_torus())
        };
        assert!(
            feet_clear(&body, &[top(0.0)], straight, band).is_ok(),
            "the planned feet are in order"
        );
        let got = feet_clear(&body, &[top(1.75)], straight, band);
        assert!(
            matches!(
                &got,
                Err(BlendError::UnsupportedRunOut { detail, .. }) if *detail == JUNCTION_FEET_CROSS
            ),
            "a junction's foot past the cut-off's refuses, got {got:?}"
        );
    }
}
