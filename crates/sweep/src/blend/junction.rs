//! **A junction between two links on different supports** — where a
//! chain's band passes from one link's surface to the next one's along
//! the rolling ball's section the two share (predicate 4's band
//! reading). It is one shape in either carve that meets it, the closed
//! mixed rim ([`super::surgery`]'s `rim_phase_mixed`) and the open band
//! ([`super::open::planar`]'s `blank_phase`), and this module is the one
//! home of its moves:
//!
//! - where its feet are, and the section between them ([`read`]);
//! - the split of the edge between its two mates at the mate foot
//!   ([`split_mate`]);
//! - the host strut, [`super::surgery::strut_foot`], every carve's;
//! - the `kev` that retires the strut and the junction vertex, re-basing
//!   the split's piece into the section ([`retire`]);
//! - what both carves record of it ([`record`]);
//! - each link's trims ([`mixed_trims`]) and band chart ([`band_chart`]).
//!
//! The HOST of a junction is the one support its two links share and
//! each link's MATE is its other support; the edge between the two
//! mates runs off the chain at the junction vertex.

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Bounds, Decide, Margin, Point3, Real, Sign, Tol};
use topo::construction::Construct;
use topo::{Body, EdgeKey, EntityId, FaceKey, VertexKey};

use super::admit::{RequestedBoundary, StationKind};
use super::arms::{BallSection, torus_radial};
use super::battery::Link;
use super::build::fan_at;
use super::naming::BlendNaming;
use super::surgery::{
    ContactCarrier, Described, ExpectedSource, SplitFragments, disagrees, halves_of,
    merged_chord_spec, op, point_of, rim_carrier, rim_trim_circles, scaled, seam_split_param,
    split_fragment, stored_piece, unbuilt_geometry, unbuilt_run_out, unresolved, unwalked,
};
use super::{BlendDecision, BlendError, BlendSite, classify};

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

/// **Split the edge between a junction's two mates at the mate foot**,
/// read LIVE off the vertex: another band of the same call carving the
/// same mates (the other rim of an extruded wall) has split it already,
/// and its fragment is what meets this junction. `chain` is every edge
/// of the chain at `vertex`, so the one other edge there is the mate
/// edge; `named` is the link a refusal names.
///
/// # Errors
///
/// [`BlendError::BodyNotIntact`] when the vertex carries other than one
/// edge off the chain; [`seam_split_param`]'s and [`split_fragment`]'s.
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
    let t = seam_split_param(body, live, named, mate_foot)?;
    split_fragment(
        body,
        live,
        vertex,
        t,
        Some(expected),
        rec,
        "junction split",
        tol,
    )
}

/// **Retire a junction**: once both links' strips are merged into their
/// band faces, the junction vertex carries only the host `strut` and
/// the split's `near` piece, each between the two bands. The strut dies
/// from its foot, the vertex with it, and `near` re-bases onto the host
/// foot, where it runs between the two feet as the section.
///
/// # Errors
///
/// [`BlendError::Op`] from the `kev`.
pub(super) fn retire<T: Decide + topo::AtRestPolicy>(
    body: &mut Body<T>,
    strut: EdgeKey,
    vertex: VertexKey,
    near: EdgeKey,
    tol: Tol,
) -> Result<(), BlendError> {
    let (sp, sm) = halves_of(body, strut)
        .ok_or_else(|| unresolved(EntityId::Edge(strut), "a junction's host strut"))?;
    let dying = if body.half_edge_end(sm) == Some(vertex) {
        sm
    } else {
        sp
    };
    let chord = merged_chord_spec(body, near, dying, "junction kev")?;
    body.kev_describing(dying, &[(near, chord)], tol)
        .map_err(|e| op("junction kev", e))?;
    Ok(())
}

/// **What a retired junction leaves, recorded**: the section described
/// as the shared row of its two bands' charts, under `link`; the host
/// foot (`RimFoot`), the mate split (a split of the mate edge, under the
/// band's identity `band`), the section (`BandSection`), and the dead
/// junction vertex.
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
    rec.rim_feet.push((host_foot, vertex));
    rec.meridian_splits
        .push((split.vertex, split.source, band.to_vec()));
    rec.sections.push((split.near, split.source, vertex));
    rec.dead.vertices.push(vertex);
}

/// The refusal when a straight link is too short for its two ends.
pub(super) const JUNCTION_FEET_CROSS: &str = "a straight link between a junction and its \
     other end is shorter than its two ends' feet need: they cross or coincide on its trimline";

/// **A straight link's feet in order, on each support, where one of its
/// ends is a junction** (`fillet3_cut_off_feet`). The trimline chord the
/// carve `mef`s runs from the foot at one station to the foot at the
/// other; the junction's foot sits square to the junction, and the other
/// end's (a cut-off's on an oblique end face, a turn's, a corner's) may
/// sit further along the edge, so on a short link the two cross. The
/// span between them along the edge is decided definitely positive;
/// feet that cross or coincide refuse typed, and feet apart only within
/// the band escalate. A torus link's chords run between two junctions'
/// feet, its own trim's window ends, so it is not read here.
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
            match classify(
                BlendSite::Link { edge: chord.edge },
                BlendDecision::CutOffFeet,
                Margin::of(span),
                band,
            )? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => {
                    return Err(unbuilt_run_out(
                        EntityId::Edge(chord.edge),
                        JUNCTION_FEET_CROSS,
                    ));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use core::f64::consts::FRAC_PI_2;

    use geom_core::{Band, Point2, Tol, Vec3};
    use profile::{SketchPlane, test_support::bulge_loop};

    use super::super::BlendError;
    use super::super::admit::{AdmittedOpen, OpenBand, RequestedBoundary};
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
                (*v, [l.face_a, l.face_b], [c.end.foot_a, c.end.foot_b])
            })
            .collect();
        let host = open.junctions()[0].host();
        // The junction on the line along `y = 0`, at `x = 1.7`.
        let on_y0 = |p: &&super::super::open::planar::JunctionPlan<'_, f64>| {
            point(p.junction.vertex()).y == 0.0
        };
        let top = |shift: f64| {
            let rows: Vec<_> = plans
                .iter()
                .map(|p| {
                    let mut foot = p.reading.host_foot;
                    if on_y0(&p) {
                        foot = foot - Vec3::new(shift, 0.0, 0.0);
                    }
                    (p.junction, [foot, p.reading.mate_foot])
                })
                .collect();
            RequestedBoundary::admit(&body, host, &opens, &[], &[], &cuts, &[], &rows)
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
