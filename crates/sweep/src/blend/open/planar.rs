//! **The planar open band** — the blank phase: a plane–plane link
//! carved in place between two trivalent corners whose three edges are
//! all requested, and the corner patches that terminate it — the
//! fillet's sphere octant or the chamfer's flat patch through the three
//! trimline feet ([`corner_plan`]). Per support face one strut `mev` per
//! boundary vertex and one trimline `mef` per blended edge carve the
//! face into the SHRUNK face plus one strip per edge; per edge one `kef`
//! merges the two strips across the dying sharp edge; per corner three
//! arc `mef`s, two `kef`s and one `kev` fuse the corner triangles into
//! the patch and retire the struts and the sharp vertex.
//!
//! The other open band is [`super::ruled`]; what the two share, and the
//! seam both rest on, is stated at [`super`].

use geom::Curve3;
use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Bounds, Decide, Point3, Real, Tol, Vec3};
use topo::{
    Body, EdgeKey, EntityId, FaceKey, FaceSurface, HalfEdgeKey, LoopKey, MefSite, MevSite,
    VertexKey,
};

use crate::blend::admit::{AdmittedOpen, CornerFaces, CornerLinks, RequestedBoundary};
use crate::blend::arms::{chamfer_corner_patch, corner_ball, line_meet};
use crate::blend::battery::Convexity;
use crate::blend::build::{octant_chart, outward_of};
use crate::blend::naming::BlendNaming;
use crate::blend::surgery::{
    CORNER_SUPPORT_NOT_PLANAR, ContactCarrier, Described, SourceFaces, chord_site, face_of_half,
    halves_of, not_intact, op, point_of, unbuilt_chain, unbuilt_geometry, unbuilt_run_out,
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
    // `vertex_faces` adds is a stored reference nothing here proves.
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
            .ok_or_else(|| unbuilt_geometry(EntityId::Face(f), CORNER_SUPPORT_NOT_PLANAR))?;
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
                let trim = if l.face_a == face {
                    &l.blend.trim_a.0
                } else if l.face_b == face {
                    &l.blend.trim_b.0
                } else {
                    return None;
                };
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
                "a corner's support does not carry two requested edges; run-outs at such \
                 corners are not implemented",
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

/// **What the plan read for the blank carve**, in one value because the
/// three are one reading of one source body and travel together: the
/// PLANAR open links, the corners their ends terminate at, and the
/// support faces they are carved along.
pub(in crate::blend) struct BlankPlan<'a, T: Real> {
    pub(in crate::blend) opens: &'a [AdmittedOpen<'a, T>],
    pub(in crate::blend) corners: &'a [Corner<'a, T>],
    pub(in crate::blend) supports: &'a [RequestedBoundary<T>],
}

#[allow(clippy::type_complexity)]
pub(in crate::blend) fn blank_phase<T: Decide + Bounds>(
    body: &mut Body<T>,
    plan: &BlankPlan<'_, T>,
    sources: &SourceFaces,
    rec: &mut BlendNaming,
    tol: Tol,
    kind: BlendKind,
) -> Result<(Vec<FaceKey>, Vec<FaceKey>, Described<T>), BlendError> {
    let (opens, corners, supports) = (plan.opens, plan.corners, plan.supports);
    // The carve is one shape for both verbs — struts to the feet,
    // trimline chords between them, a kef per link, three corner
    // chords and the fusion. What differs is what each new edge IS:
    // the fillet's band touches its supports tangentially and turns
    // its corner on an arc; the chamfer's meets them at an angle and
    // closes its corner on a chord.
    let trim_carrier = || match kind {
        BlendKind::Fillet => ContactCarrier::TrimLine,
        BlendKind::Chamfer => ContactCarrier::Chord,
    };
    let mut described: Described<T> = Vec::new();
    if opens.is_empty() {
        return Ok((Vec::new(), Vec::new(), described));
    }

    // ---- Per support face: struts at every boundary vertex, then a
    // trimline chord per boundary edge. The boundary each face is
    // carved along was walked, admitted and footed in the plan phase
    // ([`RequestedBoundary`]) — so this loop reads no source geometry
    // and has no coverage refusal of its own to make. ----
    // Per (edge, face): the half-edge of the edge now inside that
    // face's strip (for the kef), keyed structurally.
    let mut strip_half: Vec<(EdgeKey, FaceKey, HalfEdgeKey)> = Vec::new();
    // Per (vertex, face): the strut edge (for the corner merges).
    let mut strut_of: Vec<(VertexKey, FaceKey, EdgeKey)> = Vec::new();
    for support in supports {
        let f = support.face();
        let walk = support.stations();
        let n = walk.len();
        let mut struts = Vec::with_capacity(n);
        for station in walk {
            let v = station.vertex;
            let p = *body
                .get_vertex(v)
                .and_then(|x| body.get_point(x.point))
                .ok_or_else(|| not_intact(EntityId::Vertex(v), "a support boundary vertex"))?;
            let fp = station.foot;
            // **NOT re-described at rest, and that is a REPORTED
            // finding rather than an oversight** (P-1b, #1116; the
            // CAUSE below is corrected — the first version of this
            // comment misread it).
            //
            // The standing reason: this strut is a straight CHORD
            // between two points of the support surface, so on a
            // CURVED support it is a secant — it does not lie on the
            // surface its two faces share, and no chart image of that
            // surface describes it. It therefore reaches rest through
            // the scaffolding door and tier 3's transience fence names
            // it, on a body whose edge genuinely is not where its
            // faces are. The fix is fillet-verb work (put the strut ON
            // the support), not a description change, so it is not
            // taken here. An independent interval A/B reproduced the
            // escalation exactly, so declining stands.
            //
            // **What was recorded wrongly.** This said stating the
            // image "makes `ChartResidual` escalate at ε = 1e-6 on the
            // die fixture, which is the geometry saying so". It is
            // not the geometry saying anything. The escalation carries
            // `margin: Invalid`, which is the kernel's POISON outcome
            // — *the question was never validly posed* — and not a
            // distance that came out too large. On the die's PLANAR
            // support the f64 residual is exactly `0e0`: the chord
            // between two points of a plane lies in that plane, so
            // there is no disagreement there to report. Reading a
            // poison verdict as a measurement is the specific mistake,
            // and it is worth naming because poison and
            // "definitely too big" are reported through the same
            // escalation door and read identically at a glance.
            //
            // The CLASS — poison reaching this predicate at all — is
            // #1143 and M10 owns it. What stays here is the mechanism
            // question this site is the witness for: where poison
            // enters `pcurve_map_residual` on a secant input.
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
            strut_of.push((v, f, created.edge));
            rec.feet.push((created.vertex, v, f));
            struts.push((created.he_minus, fp));
        }
        let mut first_trim: Option<HalfEdgeKey> = None;
        for i in 0..n {
            let (he1, fp_i) = struts[i];
            let he2 = if i + 1 < n {
                struts[i + 1].0
            } else {
                first_trim.ok_or_else(|| {
                    unbuilt_chain(
                        walk[i].edge,
                        "a support face with a single boundary edge is not implemented",
                    )
                })?
            };
            let fp_j = struts[(i + 1) % n].1;
            let created = body
                .mef(
                    MefSite::Chords { he1, he2 },
                    EdgeCurveSpec::line_between(fp_i, fp_j),
                    FaceSurface::Inherit,
                    tol,
                )
                .map_err(|e| op("trimline mef", e))?;
            first_trim.get_or_insert(created.he_plus);
            described.push((created.edge, trim_carrier()));
            // The chord runs foot(start of walk[i]) → foot(start of
            // walk[i+1]): it parallels walk[i]'s own source edge, in
            // this support face. Birth data, straight off the plan.
            rec.trims.push((created.edge, walk[i].edge, f));
            strip_half.push((walk[i].edge, f, walk[i].half_edge));
        }
    }

    // ---- Per link: merge the two strips across the dying edge. ----
    let mut hexagon: Vec<(EdgeKey, LoopKey)> = Vec::new();
    for o in opens {
        let e = o.link().edge;
        // `strip_half` holds a row per (boundary edge, support face)
        // carved above. A miss means the verdict's two support faces
        // for this link are not the faces whose boundary carries it.
        let half_a = strip_half
            .iter()
            .find(|(ee, ff, _)| *ee == e && *ff == o.link().face_a)
            .map(|(_, _, h)| *h)
            .ok_or_else(|| not_intact(EntityId::Face(o.link().face_a), "a link's support"))?;
        let half_b = strip_half
            .iter()
            .find(|(ee, ff, _)| *ee == e && *ff == o.link().face_b)
            .map(|(_, _, h)| *h)
            .ok_or_else(|| not_intact(EntityId::Face(o.link().face_b), "a link's support"))?;
        let survivor_loop = body
            .get_half_edge(half_b)
            .map(|h| h.parent_loop)
            .ok_or_else(|| not_intact(EntityId::HalfEdge(half_b), "a carved strip's half"))?;
        sources.kef_minted(body, half_a, "edge-strip kef")?;
        hexagon.push((e, survivor_loop));
    }
    let hex_face = |body: &Body<T>, e: EdgeKey| -> Option<FaceKey> {
        let lp = hexagon.iter().find(|(ee, _)| *ee == e)?.1;
        Some(body.get_loop(lp)?.face)
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
        let mut struts_here: Vec<EdgeKey> = strut_of
            .iter()
            .filter(|(v, _, _)| *v == vertex)
            .map(|(_, _, e)| *e)
            .collect();
        struts_here.sort_unstable();
        // Also checked here rather than inherited: `strut_of` carries
        // one row per (corner vertex, support face), so three rows at
        // this vertex is three struts on three DISTINCT supports —
        // which is the whole premise of the one-spur fusion below.
        if struts_here.len() != 3 {
            return Err(unbuilt_run_out(
                EntityId::Vertex(vertex),
                "a corner did not receive a strut on each of three distinct supports; \
                 run-outs at such corners are not implemented",
            ));
        }
        let mut spur: Option<EdgeKey> = None;
        for s in struts_here {
            // Every strut was minted by this phase's `mev` above and is
            // killed at most once, in this loop, by the `kef` below.
            let Some((hp, hm)) = halves_of(body, s) else {
                unreachable!(
                    "corner fusion: a strut edge was minted by this phase's strut `mev` \
                     and has not been killed"
                )
            };
            let (fa, fb) = (face_of_half(body, hp), face_of_half(body, hm));
            if fa.is_some() && fa == fb {
                if spur.replace(s).is_some() {
                    unreachable!(
                        "corner fusion: a SECOND strut survived the fusion — exactly \
                         three struts on three distinct supports (checked immediately \
                         above) fuse to leave exactly one spur"
                    )
                }
                continue;
            }
            sources.kef_minted(body, hp, "corner-strut kef")?;
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

    let mut blend_faces = Vec::with_capacity(opens.len());
    for o in opens {
        let f = hex_face(body, o.link().edge)
            .ok_or_else(|| not_intact(EntityId::Edge(o.link().edge), "a merged strip's face"))?;
        rec.blends.push((f, o.link().edge));
        // The source edge was excised across its two strips (the kef
        // above): it is gone from the result.
        rec.dead.edges.push(o.link().edge);
        blend_faces.push(f);
    }
    Ok((blend_faces, corner_faces, described))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;

    use geom::Surface;

    use super::{AdmittedOpen, BlendKind, CornerLinks, corner_plan};
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
                .map(|c| AdmittedOpen::admit(c).expect("a cube's links are plane–plane"))
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
