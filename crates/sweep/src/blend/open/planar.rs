//! **The planar open band** — the blank phase: a plane–plane link
//! carved LOCALLY on its two support faces, each of its two ends a
//! trivalent vertex of one convexity whose edges the request names
//! either all — the corner patch, the fillet's sphere octant or the
//! chamfer's flat patch through the three trimline feet
//! ([`corner_plan`]) — or one: the cut-off in the plane end face
//! ([`cut_off_plan`], [`super::end_face`]).
//!
//! The carve: per cut-off, its first step — both rims split at their
//! feet and the end curve `mef`'d across the end face. Per support face,
//! one strut `mev` at each corner or joint station of its requested
//! edges, and one trimline `mef` per requested edge between the feet at
//! its two stations, carving the face into the SHRUNK face — every edge
//! the request does not name kept where it was — plus one strip per
//! requested edge. Per edge one `kef` merges the two strips across the
//! dying sharp edge; per corner three arc `mef`s, two `kef`s and one
//! `kev` fuse the corner triangles into the patch and retire the struts
//! and the sharp vertex; per cut-off its last step folds the sliver into
//! the band and retires the old vertex; per joint (two links of one
//! chain on the same two supports, [`Joint`]) one `kef` and one `kev`
//! fuse the two links' strips into one band face and retire the joint
//! vertex, leaving its two feet on the band's trimlines. A face whose
//! every boundary edge is requested is the case where every vertex of
//! its cycle is a station, and the carve takes nothing else from it.
//!
//! The other open band is [`super::ruled`]; what the two share, and the
//! seam both rest on, is stated at [`super`].

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use geom::Curve3;
use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Bounds, Decide, Point3, Real, Tol, Vec3};
use topo::{
    Body, EdgeKey, EntityId, FaceKey, FaceSurface, HalfEdgeKey, LoopKey, MefSite, MevSite,
    VertexKey,
};

use super::end_face::{CutRims, EndCurve, EndCut, cut_off, fold_sliver};
use crate::blend::admit::{
    AdmittedOpen, CornerFaces, CornerLinks, Joint, OpenBand, RequestedBoundary,
};
use crate::blend::arms::{chamfer_corner_patch, corner_ball, line_meet};
use crate::blend::battery::Convexity;
use crate::blend::build::{octant_chart, outward_of};
use crate::blend::naming::BlendNaming;
use crate::blend::surgery::{
    CORNER_SUPPORT_NOT_PLANAR, ContactCarrier, Described, SourceFaces, chord_site, face_of_half,
    halves_of, not_intact, op, open_trimline, point_of, unbuilt_geometry, unbuilt_run_out,
};
use crate::blend::{BlendError, BlendKind};

/// One corner: a trivalent vertex all of whose edges are requested.
pub(in crate::blend) struct Corner<'a, T: Real> {
    /// The admitted open links terminating here — at least one, and
    /// all of one convexity ([`CornerLinks`]).
    pub(in crate::blend) links: CornerLinks<'a, T>,
    /// The three incident support faces, in orbit order.
    pub(in crate::blend) faces: CornerFaces,
    /// The corner patch's foot on each of [`Corner::faces`], in that
    /// same orbit order — where this corner's two trimlines cross on
    /// each support. The strut every support face is carved with runs
    /// out to its own foot, so this array is the whole geometric
    /// difference between the two verbs' carves.
    pub(in crate::blend) feet: [Point3<T>; 3],
    /// What the corner patch's bounding edges turn about: the rolling
    /// ball's rest centre and its radius. `None` for a chamfer, whose
    /// patch is bounded by straight chords — there is nothing to turn
    /// about, which is the whole difference at a corner.
    arc: Option<(Point3<T>, T)>,
    /// The corner patch's surface: the sphere octant's chart (the
    /// order-free pick, [`octant_chart`]), or the
    /// chamfer's plane through the three feet.
    pub(in crate::blend) surface: Surface<T>,
    /// The corner patch's orientation bit, read exactly as a blend reads its
    /// own — off the stored convexity verdict
    /// ([`Convexity::blend_sense`]), never a sampled normal. A corner
    /// patch is a sphere about the rolling ball's rest centre whose
    /// chart normal is the outward radial, and the centre lies on the
    /// material side precisely when the corner is convex.
    ///
    /// **Any one incident link answers for all of them**, and what
    /// makes that sound is the BATTERY, not this module's door: a
    /// termination reaches the carve only through predicate 6, which
    /// runs at every open chain's two ends and admits a trihedron
    /// only where its three edges carry ONE convexity. So the three
    /// links here cannot disagree — on either side of the material.
    pub(in crate::blend) convexity: Convexity,
}

/// The corner patch at one fully-requested trivalent vertex: its
/// surface, and its foot on each of the three supports.
///
/// The two verbs differ here and only here at a corner.
///
/// - **Fillet**: the ball at rest touches all three supports, so its
///   foot on each is the ball centre projected onto it, and that point
///   lies on both of that support's trimlines because the centre is on
///   both incident spines ([`octant_chart`] picks the
///   octant's chart). The corner's convexity is ONE decision made in
///   three places that must agree — the ball's side, the feet's sign
///   (`centre + n·r` at a convex rest, `centre − n·r` at a concave
///   one, each the tangency point of ITS ball), and the chart's aim —
///   so all three fold the same verdict, read once below.
/// - **Chamfer**: there is no ball, so each foot is derived from the
///   trimlines directly — the two incident strips' trimlines on that
///   support, crossed in closed form ([`line_meet`]) — and the patch
///   is the plane through the three feet ([`chamfer_corner_patch`]).
///   Convexity does not appear in this arm at all: the feet come from
///   trimlines whose in-plane direction is read off the traversal, and
///   the patch's chart normal is folded outward against the supports'
///   own normal sum ([`crate::blend::arms`]).
pub(in crate::blend) fn corner_plan<'a, T: Decide + Bounds>(
    body: &Body<T>,
    links: CornerLinks<'a, T>,
    radius: T,
    kind: BlendKind,
) -> Result<Corner<'a, T>, BlendError> {
    // Both corner tokens are derived from THIS vertex, here: the
    // faces two statements below, the links in the argument. That
    // pairing is what `octant_chart`'s agreement check reads, and it
    // is why that check cannot fire from this call site.
    let vertex = links.vertex();
    // The caller walked this vertex's edge orbit successfully, which
    // proves the orbit half of this walk; the `parent_loop` deref
    // `Body::faces_of_vertex` adds is a stored reference nothing here proves.
    // The valence the corner derivation needs is the FACE orbit's; on a
    // manifold body it is the edge valence the door checked, and a
    // disagreement is itself the refusal.
    let faces = CornerFaces::admit(body, vertex)?;
    let p = *body
        .get_vertex(vertex)
        .and_then(|x| body.get_point(x.point))
        .ok_or_else(|| not_intact(EntityId::Vertex(vertex), "a corner's stored point"))?;
    let mut normals = [Vec3::new(T::zero(), T::zero(), T::zero()); 3];
    for (slot, &f) in normals.iter_mut().zip(faces.as_slice()) {
        *slot = outward_of(body, f)
            .ok_or_else(|| unbuilt_geometry(EntityId::Face(f), CORNER_SUPPORT_NOT_PLANAR))?
            .vec();
    }
    // Any one incident link answers for all of them (`Corner`'s field
    // doc): the battery's corner predicate admits a termination only
    // where all three of its edges carry one convexity.
    let convexity = links.first().convexity();
    let (arc, feet, surface) = match kind {
        BlendKind::Fillet => {
            let ball = corner_ball([p; 3], normals, radius, convexity);
            // The ball at rest is at distance `radius` from every
            // support — inside the material at a convex corner, in the
            // void at a concave one — so its foot on each is the
            // centre displaced back TOWARD that support: along the
            // outward normal from a convex rest, against it from a
            // concave one. Either way the foot is on both of the
            // support's trimlines, because the centre is on both
            // incident spines.
            //
            // ONE fold, ONE home: `Convexity::signed` — the value the
            // plane–plane band arm folds into its feet and the
            // plane–sphere arm into its spine, and (as the side bit
            // `Convexity::ball_side`) what the shared sheet reduction
            // hands each trace (`battery::curved_arm`). `corner_ball`
            // alone needs its NEGATIVE, the rest DEPTH
            // (`c·n = p·n − toward`), the displacement's opposite by
            // definition of tangency, and spells it as `-signed(..)`.
            let toward = convexity.signed(radius);
            let mut feet = [ball.center; 3];
            for (foot, &n) in feet.iter_mut().zip(normals.iter()) {
                *foot = ball.center + n * toward;
            }
            let (u_ref, axis) = octant_chart(body, &faces, &links, convexity)?;
            (
                Some((ball.center, radius)),
                feet,
                Surface::Sphere {
                    center: ball.center,
                    radius,
                    axis,
                    u_ref,
                },
            )
        }
        BlendKind::Chamfer => {
            let feet = chamfer_feet(&faces, &links, p)?;
            (None, feet, chamfer_corner_patch(feet, normals))
        }
    };
    Ok(Corner {
        links,
        faces,
        feet,
        arc,
        surface,
        convexity,
    })
}

/// The chamfer's three feet at one corner: on each support, where the
/// two incident strips' trimlines on that support cross.
///
/// Exactly two of the corner's admitted links touch each support (the
/// corner is trivalent and fully requested), and each link's trimline
/// on that support is the one keyed to it by
/// `Link::face_a`/`Link::face_b` — read by support key, never by slot
/// order.
fn chamfer_feet<T: Decide + Bounds>(
    faces: &CornerFaces,
    links: &CornerLinks<'_, T>,
    vertex_point: Point3<T>,
) -> Result<[Point3<T>; 3], BlendError> {
    let (seed, others) = links.sorted();
    let mut feet = [vertex_point; 3];
    for (slot, &face) in faces.as_slice().iter().enumerate() {
        let mut on_face = core::iter::once(&seed)
            .chain(others.iter())
            .filter_map(|o| {
                let l = o.link();
                let (trim, _) = l.trim_on(face)?;
                match *trim {
                    Curve3::Line { origin, dir } => Some(Ok((origin, dir))),
                    _ => Some(Err(unbuilt_geometry(
                        EntityId::Edge(l.edge),
                        "a chamfer strip's trimline is not a line",
                    ))),
                }
            });
        let (Some(first), Some(second)) = (on_face.next(), on_face.next()) else {
            return Err(unbuilt_run_out(
                EntityId::Face(face),
                "a corner's support does not carry two requested edges",
            ));
        };
        let (o1, d1) = first?;
        let (o2, d2) = second?;
        // Both trimlines lie in this support, so their cross product
        // is the support's own normal up to a nonzero scale — and
        // `line_meet`'s ratio is invariant under that scale, so no
        // second reading of the face's stored plane is needed.
        feet[slot] = line_meet(o1, d1, o2, d2, d1.cross(d2));
    }
    Ok(feet)
}

/// One joint, planned: the joint and its foot on each of its two
/// supports, in [`Joint::faces`] order.
pub(in crate::blend) struct JointPlan<'a, T: Real> {
    pub(in crate::blend) joint: &'a Joint,
    pub(in crate::blend) feet: [Point3<T>; 2],
}

/// Plan a joint: its foot on each support is the joint vertex
/// projected onto the arriving link's trimline there.
///
/// Both links' arms are one function of the same two planes, so their
/// trimlines on a support are one line and the projection lands on
/// both; the arriving link's is read because the joint names it. The
/// same derivation serves both verbs — a fillet's trimline and a
/// chamfer's are each a line parallel to the edge — so, unlike a
/// corner, a joint has no per-verb arm.
///
/// # Errors
///
/// [`BlendError::BodyNotIntact`] when the joint's arriving link is not
/// among `opens` or its vertex has no point;
/// [`BlendError::UnsupportedGeometry`] when a trimline is not a line.
pub(in crate::blend) fn joint_plan<'a, T: Decide>(
    body: &Body<T>,
    joint: &'a Joint,
    opens: &[AdmittedOpen<'_, T>],
) -> Result<JointPlan<'a, T>, BlendError> {
    let vertex = joint.vertex();
    let link = opens
        .iter()
        .find(|o| o.edge() == joint.arriving())
        .map(AdmittedOpen::link)
        .ok_or_else(|| {
            not_intact(
                EntityId::Vertex(vertex),
                "a joint's arriving link is not an admitted planar link",
            )
        })?;
    let p = point_of(body, vertex)
        .ok_or_else(|| not_intact(EntityId::Vertex(vertex), "a joint's stored point"))?;
    let mut feet = [p; 2];
    for (foot, face) in feet.iter_mut().zip(joint.faces()) {
        let Some((trim, _)) = link.trim_on(face) else {
            return Err(not_intact(
                EntityId::Vertex(vertex),
                "a joint's face is not a support of its arriving link",
            ));
        };
        let Curve3::Line { origin, dir } = *trim else {
            return Err(unbuilt_geometry(
                EntityId::Edge(link.edge),
                "a planar band's trimline is not a line",
            ));
        };
        *foot = origin + dir * ((p - origin).dot(dir) / dir.dot(dir));
    }
    Ok(JointPlan { joint, feet })
}

/// One cut-off end of a planar band, planned: the link that ends there
/// and the end ([`EndCut`]).
pub(in crate::blend) struct CutOffPlan<'a, T: Real> {
    pub(in crate::blend) link: AdmittedOpen<'a, T>,
    pub(in crate::blend) end: EndCut<T>,
}

/// Plan the cut-off of `link` at `vertex`: each support's trimline
/// carried along itself to the end face — a chord for a chamfer, and for
/// a fillet the arc about the spine's crossing (the battery admitted the
/// end only where the end face is perpendicular to the spine).
///
/// # Errors
///
/// [`EndCut::plan`]'s, and [`BlendError::UnsupportedGeometry`] when a
/// trimline is not a line or a fillet's band is not a cylinder.
pub(in crate::blend) fn cut_off_plan<'a, T: Decide + Bounds>(
    body: &Body<T>,
    link: AdmittedOpen<'a, T>,
    vertex: VertexKey,
    kind: BlendKind,
) -> Result<CutOffPlan<'a, T>, BlendError> {
    let l = link.link();
    let (q_a, along) = open_trimline(l, l.face_a)?;
    let (q_b, _) = open_trimline(l, l.face_b)?;
    let curve = match (kind, &l.blend.surface) {
        (BlendKind::Chamfer, _) => EndCurve::Chord,
        (BlendKind::Fillet, Surface::Cylinder { origin, radius, .. }) => EndCurve::Arc {
            center: *origin,
            radius: *radius,
        },
        (BlendKind::Fillet, _) => {
            return Err(unbuilt_geometry(
                EntityId::Edge(l.edge),
                "a plane–plane fillet's band is not a cylinder about its edge",
            ));
        }
    };
    let end = EndCut::plan(
        body,
        vertex,
        l.edge,
        (l.face_a, l.face_b),
        (q_a, q_b),
        along,
        curve,
    )?;
    Ok(CutOffPlan { link, end })
}

/// **What the plan read for the blank carve**, in one value because the
/// five are one reading of one source body and travel together: the
/// PLANAR open bands and their links, the corners and cut-offs their
/// ends terminate at, the joints inside them, and the support faces they
/// are carved along.
pub(in crate::blend) struct BlankPlan<'a, T: Real> {
    pub(in crate::blend) bands: &'a [&'a OpenBand<'a, T>],
    pub(in crate::blend) opens: &'a [AdmittedOpen<'a, T>],
    pub(in crate::blend) corners: &'a [Corner<'a, T>],
    pub(in crate::blend) cut_offs: &'a [CutOffPlan<'a, T>],
    pub(in crate::blend) joints: &'a [JointPlan<'a, T>],
    pub(in crate::blend) supports: &'a [RequestedBoundary<T>],
}

/// The one half-edge of `face`'s cycles that starts at `foot` — where a
/// trimline chord hangs. Read LIVE: each chord splits the face under it,
/// and a foot of valence one or two meets the shrinking face once.
fn half_from<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    foot: VertexKey,
) -> Result<HalfEdgeKey, BlendError> {
    let (he, _, _, _) = chord_site(body, face, |row| row.1 == foot, 0, 0)?;
    Ok(he)
}

#[allow(clippy::type_complexity)]
pub(in crate::blend) fn blank_phase<T: Decide + Bounds + topo::AtRestPolicy>(
    body: &mut Body<T>,
    plan: &BlankPlan<'_, T>,
    sources: &SourceFaces,
    rec: &mut BlendNaming,
    tol: Tol,
    kind: BlendKind,
) -> Result<(Vec<FaceKey>, Vec<FaceKey>, Described<T>), BlendError> {
    let (bands, opens, corners, cut_offs, joints, supports) = (
        plan.bands,
        plan.opens,
        plan.corners,
        plan.cut_offs,
        plan.joints,
        plan.supports,
    );
    // The carve is one shape for both verbs — feet, trimline chords
    // between them, a kef per link, the corner fusion and the cut-offs'
    // folds. What differs is what each new edge IS: the fillet's band
    // touches its supports tangentially, turns its corner on an arc and
    // ends in an arc; the chamfer's meets them at an angle and closes
    // its corner and its ends on chords.
    let trim_carrier = || match kind {
        BlendKind::Fillet => ContactCarrier::TrimLine,
        BlendKind::Chamfer => ContactCarrier::Chord,
    };
    let mut described: Described<T> = Vec::new();
    if opens.is_empty() {
        return Ok((Vec::new(), Vec::new(), described));
    }
    // Per (station vertex, support face): the foot vertex the carve
    // landed there — a strut's far end or a split rim's new vertex.
    let mut foot_of: BTreeMap<(VertexKey, FaceKey), VertexKey> = BTreeMap::new();
    let foot_at =
        |foot_of: &BTreeMap<(VertexKey, FaceKey), VertexKey>, v: VertexKey, f: FaceKey| {
            foot_of
                .get(&(v, f))
                .copied()
                .ok_or_else(|| not_intact(EntityId::Vertex(v), "a station's foot on its support"))
        };

    // ---- Per cut-off: both rims split at their feet and the end curve
    // `mef`'d across the end face ([`cut_off`]). ----
    let mut cuts: Vec<CutRims> = Vec::with_capacity(cut_offs.len());
    for c in cut_offs {
        let l = c.link.link();
        let (rims, row) = cut_off(body, &c.end, l.edge, (l.face_a, l.face_b), rec, tol)?;
        described.push(row);
        once(
            &mut foot_of,
            (c.end.vertex, l.face_a),
            rims.a.vertex,
            EntityId::Vertex(c.end.vertex),
        )?;
        once(
            &mut foot_of,
            (c.end.vertex, l.face_b),
            rims.b.vertex,
            EntityId::Vertex(c.end.vertex),
        )?;
        cuts.push(rims);
    }

    // ---- Per support face: a strut at every corner and joint station,
    // then a trimline chord per requested edge between its two feet. The
    // stations and the edges were walked, admitted and footed in the
    // plan phase ([`RequestedBoundary`]) — so this loop reads no source
    // geometry and has no coverage refusal of its own to make. ----
    // Per (edge, face): the half-edge of the edge now inside that
    // face's strip (for the kef), keyed structurally.
    let mut strip_half: BTreeMap<(EdgeKey, FaceKey), HalfEdgeKey> = BTreeMap::new();
    // Per vertex: its strut edges, one per support (for the corner and
    // joint fusions).
    let mut strut_of: BTreeMap<VertexKey, Vec<EdgeKey>> = BTreeMap::new();
    for support in supports {
        let f = support.face();
        for station in support.stations().iter().filter(|s| s.kind.spins_a_strut()) {
            let v = station.vertex;
            let p = *body
                .get_vertex(v)
                .and_then(|x| body.get_point(x.point))
                .ok_or_else(|| not_intact(EntityId::Vertex(v), "a support boundary vertex"))?;
            let fp = station.foot;
            // **NOT re-described at rest, and that is a REPORTED
            // finding rather than an oversight** (P-1b, #1116).
            //
            // This strut is a straight CHORD between two points of the
            // support surface, so on a CURVED support it is a secant —
            // it does not lie on the surface its two faces share, and
            // no chart image of that surface describes it. It therefore
            // reaches rest through the scaffolding door and tier 3's
            // transience fence names it. The fix is fillet-verb work
            // (put the strut ON the support), not a description change.
            //
            // An escalation stating the image once carried `margin:
            // Invalid` — the kernel's POISON outcome, the question never
            // validly posed — not a distance too large: on a PLANAR
            // support the f64 residual is exactly `0e0`. The class,
            // poison reaching the predicate at all, is #1143 and M10's;
            // this site is the witness for where poison enters
            // `pcurve_map_residual` on a secant input.
            let created = body
                .mev(
                    MevSite::Fan {
                        he1: station.half_edge,
                        he2: station.half_edge,
                    },
                    fp,
                    EdgeCurveSpec::line_between(p, fp),
                    tol,
                )
                .map_err(|e| op("strut mev", e))?;
            strut_of.entry(v).or_default().push(created.edge);
            rec.feet.push((created.vertex, v, f));
            once(&mut foot_of, (v, f), created.vertex, EntityId::Vertex(v))?;
        }
        for chord in support.chords() {
            // The strip is the run from the foot at `from` through the
            // edge to the foot at `to` (`MefSite::Chords` moves `he1`'s
            // side onto the new face), so the support keeps its key.
            let (x, y) = (
                foot_at(&foot_of, chord.from, f)?,
                foot_at(&foot_of, chord.to, f)?,
            );
            let (he1, he2) = (half_from(body, f, x)?, half_from(body, f, y)?);
            let (px, py) = (
                point_of(body, x).ok_or_else(|| not_intact(EntityId::Vertex(x), "a foot"))?,
                point_of(body, y).ok_or_else(|| not_intact(EntityId::Vertex(y), "a foot"))?,
            );
            let created = body
                .mef(
                    MefSite::Chords { he1, he2 },
                    EdgeCurveSpec::line_between(px, py),
                    FaceSurface::Inherit,
                    tol,
                )
                .map_err(|e| op("trimline mef", e))?;
            described.push((created.edge, trim_carrier(), chord.edge));
            // The chord runs foot(from) → foot(to): it parallels its
            // own source edge, in this support face. Birth data,
            // straight off the plan.
            rec.trims.push((created.edge, chord.edge, f));
            once(
                &mut strip_half,
                (chord.edge, f),
                chord.half_edge,
                EntityId::Edge(chord.edge),
            )?;
        }
    }

    // ---- Per link: merge the two strips across the dying edge. ----
    let mut hexagon: BTreeMap<EdgeKey, LoopKey> = BTreeMap::new();
    for o in opens {
        let e = o.link().edge;
        // `strip_half` holds a row per (boundary edge, support face)
        // carved above. A miss means the verdict's two support faces
        // for this link are not the faces whose boundary carries it.
        let half_on = |f: FaceKey| {
            strip_half
                .get(&(e, f))
                .copied()
                .ok_or_else(|| not_intact(EntityId::Face(f), "a link's support"))
        };
        let (half_a, half_b) = (half_on(o.link().face_a)?, half_on(o.link().face_b)?);
        let survivor_loop = body
            .get_half_edge(half_b)
            .map(|h| h.parent_loop)
            .ok_or_else(|| not_intact(EntityId::HalfEdge(half_b), "a carved strip's half"))?;
        sources.kef_minted(body, half_a, "edge-strip kef", tol)?;
        once(&mut hexagon, e, survivor_loop, EntityId::Edge(e))?;
    }
    let hex_face = |body: &Body<T>, e: EdgeKey| -> Option<FaceKey> {
        Some(body.get_loop(*hexagon.get(&e)?)?.face)
    };

    // ---- Per corner: three arcs, then the corner fusion. ----
    let mut corner_faces = Vec::with_capacity(corners.len());
    for c in corners {
        let vertex = c.links.vertex();
        // One arc off one incident link's merged strip: the mint the
        // loop below runs once per link, hoisted so the SEEDED link and
        // the rest reach the same body of code.
        let arc_of = |body: &mut Body<T>,
                      o: &AdmittedOpen<'_, T>,
                      described: &mut Described<T>,
                      rec: &mut BlendNaming|
         -> Result<EdgeKey, BlendError> {
            let l = o.link();
            let f = hex_face(body, l.edge)
                .ok_or_else(|| not_intact(EntityId::Edge(l.edge), "a merged strip's face"))?;
            // The arc spans the two feet flanking the corner in the
            // merged strip's cycle: from the foot whose half-edge ENDS
            // at the corner to the foot two positions on.
            let (he1, he2, v1, v2) = chord_site(
                body,
                f,
                |row| body.half_edge_end(row.0) == Some(vertex),
                0,
                2,
            )?;
            let (p1, p2) = (
                point_of(body, v1)
                    .ok_or_else(|| not_intact(EntityId::Vertex(v1), "an arc foot's vertex"))?,
                point_of(body, v2)
                    .ok_or_else(|| not_intact(EntityId::Vertex(v2), "an arc foot's vertex"))?,
            );
            let created = body
                .mef(
                    MefSite::Chords { he1, he2 },
                    EdgeCurveSpec::line_between(p1, p2),
                    FaceSurface::Inherit,
                    tol,
                )
                .map_err(|e| op("corner-arc mef", e))?;
            described.push((
                created.edge,
                match c.arc {
                    Some((center, radius)) => ContactCarrier::CornerArc { center, radius },
                    None => ContactCarrier::Chord,
                },
                l.edge,
            ));
            rec.arcs.push((created.edge, vertex, l.edge));
            Ok(created.edge)
        };
        // The seed's arc is a VALUE: `sorted` keeps one link in a slot
        // of its own, so there is no "no arc was minted" state to
        // reach. The seed is the LOWEST-KEYED incident link, which is
        // the only thing read of it here and below.
        let (seed, others) = c.links.sorted();
        let first_arc = arc_of(body, &seed, &mut described, rec)?;
        for o in &others {
            arc_of(body, o, &mut described, rec)?;
        }
        // Fuse the three triangles: kef the struts that still separate
        // two faces (sorted), kev the last one together with the sharp
        // vertex.
        let mut struts_here: Vec<EdgeKey> = strut_of.get(&vertex).cloned().unwrap_or_default();
        struts_here.sort_unstable();
        // Also checked here rather than inherited: `strut_of` holds one
        // strut per (corner vertex, support face) — `foot_of`'s key
        // refuses a second — so three at this vertex is three struts on
        // three DISTINCT supports —
        // which is the whole premise of the one-spur fusion below.
        if struts_here.len() != 3 {
            return Err(unbuilt_run_out(
                EntityId::Vertex(vertex),
                "a corner did not receive a strut on each of three distinct supports",
            ));
        }
        let mut spur: Option<EdgeKey> = None;
        for s in struts_here {
            // Every strut was minted by this phase's `mev` above and is
            // killed at most once, in this loop, by the `kef` below.
            let Some((hp, _)) = halves_of(body, s) else {
                unreachable!(
                    "corner fusion: a strut edge was minted by this phase's strut `mev` \
                     and has not been killed"
                )
            };
            if topo::readback::edge_sides(body, s).is_ok_and(|x| x.plus.face == x.minus.face) {
                if spur.replace(s).is_some() {
                    unreachable!(
                        "corner fusion: a SECOND strut survived the fusion — exactly \
                         three struts on three distinct supports (checked immediately \
                         above) fuse to leave exactly one spur"
                    )
                }
                continue;
            }
            sources.kef_minted(body, hp, "corner-strut kef", tol)?;
        }
        // Row 0 (`D96`): NO, for both spur arms — the premise is a
        // COUNT this call checked immediately above, but WHICH strut
        // survives is the outcome of three Euler operators, not a
        // shape a type carries (`docs/SMELL-T-LOG.md`, `T-c`).
        let Some(s) = spur else {
            unreachable!(
                "corner fusion: NO strut survived the fusion — exactly three struts on \
                 three distinct supports (checked immediately above) fuse to leave \
                 exactly one spur"
            )
        };
        let Some((hp, hm)) = halves_of(body, s) else {
            unreachable!(
                "corner fusion: the spur strut was minted by this phase and skipped \
                          by the `kef` above"
            )
        };
        // The spur's far vertex must be the sharp corner: kev from the
        // foot-side half.
        let dying = if body.half_edge_end(hm) == Some(vertex) {
            hm
        } else {
            hp
        };
        // A spur's far vertex has valence one, so the keys-only kill
        // merges no fan.
        debug_assert!(
            body.kev_merged_members(dying).is_ok_and(|m| m.is_empty()),
            "corner kev: the spur's far vertex has valence one"
        );
        body.kev(dying).map_err(|e| op("corner kev", e))?;
        // The corner patch is whatever face the first arc's non-blend
        // half now bounds.
        let Some((ahp, ahm)) = halves_of(body, first_arc) else {
            unreachable!(
                "corner fusion: the seed link's arc was minted above and nothing \
                          between here and there kills it"
            )
        };
        // `first_arc` was minted on the SEED's merged strip, so the
        // face it is not the patch of is that same strip. Read from
        // `seed`, never from `CornerLinks::first` — those are two
        // different links unless the caller happens to feed the
        // incidence lists in ascending edge order, which `sorted`
        // exists precisely not to depend on.
        let quad = hex_face(body, seed.edge());
        let patch = match (face_of_half(body, ahp), face_of_half(body, ahm)) {
            (Some(f1), Some(f2)) => {
                if Some(f1) == quad {
                    f2
                } else {
                    f1
                }
            }
            _ => unreachable!(
                "corner fusion: both halves of an arc this phase minted bound a face; \
                 `mef` mints the arc into two loops and the `kev` above kills neither"
            ),
        };
        rec.corners.push((patch, vertex));
        rec.dead.vertices.push(vertex);
        corner_faces.push(patch);
    }

    // ---- Per cut-off: fold the sliver into the band ([`fold_sliver`]),
    // before the joints' kefs retire any link's merged strip loop. ----
    for (c, rims) in cut_offs.iter().zip(&cuts) {
        let e = c.link.edge();
        let band = hex_face(body, e)
            .ok_or_else(|| not_intact(EntityId::Edge(e), "a merged strip's face"))?;
        fold_sliver(body, sources, band, c.end.vertex, rims, rec, tol)?;
    }

    // ---- Per joint: fuse the two links' strips into one band face.
    // After the link kefs the joint vertex carries only its two struts,
    // each separating the arriving link's strip from the leaving one's;
    // a kef across the lower-keyed one merges the strips, and the other
    // is left a spur whose far vertex is the joint. ----
    for jp in joints {
        let vertex = jp.joint.vertex();
        let mut struts_here: Vec<EdgeKey> = strut_of.get(&vertex).cloned().unwrap_or_default();
        struts_here.sort_unstable();
        let [merge, spur] = struts_here[..] else {
            return Err(unbuilt_run_out(
                EntityId::Vertex(vertex),
                "a joint did not receive a strut on each of its two supports",
            ));
        };
        let Some((hp, _)) = halves_of(body, merge) else {
            unreachable!("joint fusion: a strut edge was minted by this phase's strut `mev`")
        };
        sources.kef_minted(body, hp, "joint-strut kef", tol)?;
        let Some((hp, hm)) = halves_of(body, spur) else {
            unreachable!(
                "joint fusion: the spur strut was minted by this phase and the kef above \
                 killed the other strut"
            )
        };
        let dying = if body.half_edge_end(hm) == Some(vertex) {
            hm
        } else {
            hp
        };
        debug_assert!(
            body.kev_merged_members(dying).is_ok_and(|m| m.is_empty()),
            "joint kev: the spur's far vertex has valence one"
        );
        body.kev(dying).map_err(|e| op("joint kev", e))?;
        rec.dead.vertices.push(vertex);
    }

    // One band face per band: the face every link's surviving strip
    // loop now belongs to — a joint's kef killed one of each pair.
    let mut blend_faces = Vec::with_capacity(bands.len());
    for band in bands {
        let mut live = band.links().filter_map(|o| hex_face(body, o.edge()));
        let first = band.first().edge();
        let f = live
            .next()
            .ok_or_else(|| not_intact(EntityId::Edge(first), "a merged strip's face"))?;
        if live.any(|g| g != f) {
            return Err(not_intact(
                EntityId::Edge(first),
                "a band's links did not fuse into one face across their joints",
            ));
        }
        let edges: Vec<EdgeKey> = band.links().map(|o| o.edge()).collect();
        // The source edges were excised across their strips (the kefs
        // above): they are gone from the result.
        rec.dead.edges.extend(edges.iter().copied());
        if let [edge] = edges[..] {
            rec.blends.push((f, edge));
        } else {
            rec.joined_blends.push((f, edges));
        }
        blend_faces.push(f);
    }
    Ok((blend_faces, corner_faces, described))
}

/// Record `value` under `key`, which the carve mints once: a second
/// row for one key is a plan the body disagrees with, refused rather
/// than overwritten.
fn once<K: Ord, V>(
    map: &mut BTreeMap<K, V>,
    key: K,
    value: V,
    at: EntityId,
) -> Result<(), BlendError> {
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            Ok(())
        }
        Entry::Occupied(_) => Err(not_intact(
            at,
            "the carve minted a second row for one station, strip or link",
        )),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;

    use geom::Surface;

    use super::{AdmittedOpen, BlendKind, CornerLinks, OpenBand, corner_plan};
    use crate::blend::battery::{Chain, ChainClosure, Convexity, Link};
    use crate::test_support::{L, R, all_links, cube};

    /// One open chain per link of a cube, so the door has something to
    /// admit. The fixture FALSIFIES nothing about the geometry — the
    /// convexity carried is the one `all_links` resolved.
    fn open_chain(link: Link<f64>) -> Chain<f64> {
        let (head, tail) = (link.start, link.end);
        Chain::new(
            link,
            Vec::new(),
            Vec::new(),
            ChainClosure::Open { head, tail },
        )
    }

    /// **The corner plan FOLDS its links' convexity verdict — as one
    /// decision, at every site that reads its sign.** The same cube
    /// corner is planned twice: once under the verdict its links
    /// really carry (convex), once under the same links with the
    /// verdict FALSIFIED to concave. The two plans must disagree in
    /// exactly the mirrored ways — ball centre reflected to the other
    /// side of the vertex, feet reflected with it, sense bit flipped,
    /// chart pole flipped — and any single fold left convex-hardcoded
    /// breaks one of the four assertions while the others stay green,
    /// which is what makes each an independent pin.
    ///
    /// **The concave half of this probe is not a body.** The fixture
    /// falsifies the battery's stored verdict on a cube whose geometry
    /// is untouched — a lie about a convex body, not a concave one.
    /// What it pins is the PLAN's derivation as a function of the
    /// verdict. A real concave body carved end to end is the filleted
    /// vented cavity of the concave-fillet suite.
    #[test]
    fn a_corner_plan_takes_its_links_convexity() {
        let body = cube(L, Tol::witness());
        let links = all_links(&body, Tol::witness());
        let v = links[0].start;
        let plan_with = |flip: bool| {
            let flipped: Vec<Link<f64>> = links
                .iter()
                .cloned()
                .map(|mut l| {
                    if flip {
                        l.convexity = Convexity::Concave;
                    }
                    l
                })
                .collect();
            let chains: Vec<Chain<f64>> = flipped.into_iter().map(open_chain).collect();
            let admitted: Vec<AdmittedOpen<'_, f64>> = chains
                .iter()
                .map(|c| {
                    OpenBand::admit(&body, c)
                        .expect("a cube's links are plane–plane")
                        .first()
                })
                .collect();
            let mut here = admitted.iter().filter(|o| {
                let l = o.link();
                l.start == v || l.end == v
            });
            let first = *here.next().expect("the seed link of this corner");
            let mut corner_links = CornerLinks::seed(v, first).expect("the seed link touches v");
            for o in here {
                corner_links
                    .also(*o)
                    .expect("every filtered link touches v");
            }
            let corner = corner_plan(&body, corner_links, R, BlendKind::Fillet)
                .expect("either verdict plans");
            let (centre, _) = corner.arc.expect("a fillet corner plans an arc centre");
            let Surface::Sphere { axis, .. } = corner.surface else {
                panic!("a fillet corner patch is a sphere");
            };
            let p = super::point_of(&body, v).expect("the corner's point");
            (corner.convexity.blend_sense(), centre, corner.feet, axis, p)
        };
        let (conv_sense, cc, conv_feet, ax, p) = plan_with(false);
        let (conc_sense, kc, conc_feet, kx, _) = plan_with(true);
        assert!(conv_sense, "a convex octant is outward");
        assert!(!conc_sense, "a concave octant faces its ball centre");
        assert!(
            (kc - (p + (p - cc))).norm() < 1e-14,
            "the concave rest is the convex one reflected through the vertex"
        );
        for i in 0..3 {
            let mirrored = p + (p - conv_feet[i]);
            assert!(
                (conc_feet[i] - mirrored).norm() < 1e-14,
                "foot {i} reflects with the ball it touches"
            );
        }
        assert!(
            (ax + kx).norm() < 1e-14,
            "the two verdicts' chart poles are antipodal (the fold's sign)"
        );
    }
}
