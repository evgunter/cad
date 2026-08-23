//! **The in-place edge-blend composition surgery** (M6 unit 1): fillet
//! a SUBSET of a body's edges by operating on the body itself — split
//! the support faces along the stored trimlines, excise the edge
//! strips, and graft the blend walls in — instead of rebuilding a
//! whole polyhedron from scratch.
//!
//! This is the unit M5 banked at PR 12 (deviation 1's second door and
//! deviation 2), sized by that review at one reviewed unit, and
//! sequenced at the head of M6 by Evan's #169 ruling. It is what makes
//! the COMPOSED DIE possible: the filleted blank, the 21 pips and the
//! filleted pip rims in ONE body.
//!
//! # What the surgery does, per chain kind
//!
//! **Open chains** (plane–plane links, the box edges): every open
//! chain must be a single link terminating at trivalent corners
//! whose THREE incident edges are all requested — the sphere-octant
//! configuration, reached in place. Per support face: one strut
//! `mev` per boundary vertex (to the corner ball's foot on that
//! face) and one trimline `mef` per blended edge carve the face
//! into the SHRUNK face plus one strip per edge — the shrunk face
//! keeps its `FaceKey`, its surface, its sense bit (S12
//! parent-sense inheritance) **and its rings**, which is what
//! carries a face's rings through the fillet. Then per edge one
//! `kef` merges the two strips across the dying sharp edge; per
//! corner three arc `mef`s split the corner triangles off, two
//! `kef`s and one `kev` fuse them into the octant and retire the
//! struts and the sharp vertex.
//!
//! **Closed chains** (any link whose blend is a TORUS) come in TWO
//! shapes, which
//! are different surgeries and not two settings of one — see
//! [`RimShape`].
//!
//! The **LADDER** rim (the pip rims): the chain must be a ring of its
//! PLANE support and the entire boundary of its curved support (a
//! cap). The rim edges are replaced by a torus BAND:
//! struts and trim `mef`s on both supports carve the two annular
//! strips (the plane's hole widens from the rim circle to the trim
//! circle — the fillet eats into the FLAT face, which is what makes it
//! a fillet and not a gouge; the curved side splits its MERIDIAN seam
//! edges at the trim circle instead of strutting into the cap),
//! rim-edge `kef`s merge them and strut `kef`s fuse the pieces around
//! the ring. The band is an annulus and a curved face must be
//! RING-FREE (`props`' closed-form inventory; the donut's own
//! representation), so at the closure vertex the last strut dies by a
//! fan-merging `kev` that leaves one upper meridian remnant as the
//! band's SLIT — a double-traversed minor-circle `Seam` edge, with
//! the band's torus chart seamed at that azimuth (certification
//! demands a seam lie in its surface's `u_ref` half-plane; the chart
//! reference is conventional data, D2). The trim circles' carriers
//! are the rim carrier's own frame SCALED (same axis, same `u_ref`,
//! same parameter window), so the band's arcs inherit the rim's seam
//! structure exactly — no `atan2` reconstruction, no π-arc
//! ambiguity.
//!
//! The **ANNULUS** rim (a full solid of revolution's latitude rim):
//! ONE closed edge, so there is no consecutive pair and no ladder to
//! walk. Both supports are revolution WALLS — a full revolve mints
//! each profile segment as one face whose single cycle carries two
//! closed latitude rims and a doubly-traversed seam meridian — and the
//! band is minted as one more wall of that same shape: two seam splits
//! (the plane's takes the strut `mev`'s place), one closed-edge `mef`
//! per support carving its strip, one rim `kef` merging the strips,
//! and the ladder's own closure `kev` retiring the rim vertex and
//! fan-merging the sphere seam's remnant into the slit.
//!
//! # What decides, and what does not
//!
//! The battery already judged every margin (the C8 ordering contract —
//! [`super::build::fillet_edges`] runs it first and hands the verdict
//! in). The surgery adds exactly ONE new numeric decision, the ring
//! carry-through honesty check: **`fillet3_ring_clearance`**, a Q1
//! trilean whose margin (meters) is the closed-form clearance between
//! a support face's ring and a blend's trimline — circle-vs-line and
//! circle-vs-circle, exact, never sampled. Positive carries the ring
//! through; zero/negative refuses typed
//! ([`FilletError::RingClearance`]); in-band escalates with the same
//! recourse (two-tolerance, D4 ¶1 addendum). Everything else in this
//! module is structural: cycle walks, key equality, stored senses.
//!
//! # Out of scope, refused typed
//!
//! Multi-link open chains (junction carry-through), concave chains
//! (material-adding blends), partially-requested corners (run-outs),
//! closed rims that are neither a circle-carried ring of a plane
//! against a sphere cap nor a one-edge rim between two revolution
//! walls, and two closed rims sharing one support in ONE call (the
//! second band's plan would name a seam the first consumed —
//! [`shared_support_gate`], whose recourse is sequential calls) — each
//! refuses through the frontier vocabulary
//! ([`FilletError::UnsupportedChain`],
//! [`FilletError::UnsupportedRunOut`],
//! [`FilletError::UnsupportedGeometry`],
//! [`FilletError::UnsupportedBody`], and
//! [`FilletError::FilletCornerUnsupported`] for a corner's own
//! configuration), naming itself and carrying the offending entity. The refusals are the honest boundary of the unit,
//! not gates hiding reachable geometry.
//!
//! # The three refusal classes, and what is NOT one
//!
//! A frontier is one thing; an invalid input is another; an impossible
//! state is a third (D2 addendum, rows 2, 1 and 4). This module keeps
//! them apart at every site:
//!
//! - **Row 2**, above: valid input, unbuilt door, carries the fillet
//!   recourse that is true of it.
//! - **Row 1**, [`FilletError::BodyNotIntact`]: a stored reference
//!   that did not resolve, or a cycle that did not close. **This is not a kernel
//!   bug channel.** A body that fails referential integrity is
//!   reachable at this door without any kernel bug in the trace —
//!   `topo::instance::graft_disjoint_all`'s own docs record that a
//!   refusal raised mid-transplant leaves its destination *spent,
//!   never resumable*, and a caller that keeps that body may hand it
//!   here. So these sites refuse typed, naming the entity.
//! - **Row 4**, `unreachable!`: only where the state is impossible on
//!   facts THIS call establishes — a key this call minted, a key a
//!   walk in this call returned, or a count this call checked. Each
//!   carries that proof in its message. No site inherits its proof
//!   from whole-body validity, which the paragraph above is exactly
//!   why.

use geom::Curve3;
use geom::Surface;
use geom_brep::{EdgeCurveSpec, EdgeGeometry};
use geom_core::{Band, Bounds, Decide, Margin, Point3, Real, Sign, Vec3};
use topo::{
    Body, EdgeKey, EntityId, FaceKey, FaceSurface, HalfEdgeKey, LoopKey, MefSite, MevSite,
    VertexKey,
};

use super::admit::{ConvexOpen, CornerFaces, CornerLinks, RequestedBoundary};
use super::battery::{BatteryVerdict, Chain, ChainClosure, Convexity, Link};
use super::blend::{EdgeBlend, chamfer_corner_patch, corner_ball, line_meet};
use super::build::{Filleted, face_cycle, outward_of};
use super::naming::{FilletNaming, RimSide};
use super::{BlendKind, CornerConfig, FilletError, FilletSite, RunOutPolicy, decide};
use geom_core::Tol;

// ------------------------------------------------------------------
// The three refusal classes this module can produce (D2 addendum).
// One named constructor each, so a site's class is greppable at the
// site and no site can inherit a class from a closure's name.
// ------------------------------------------------------------------

/// **Row 1** — the body or the verdict handed to the surgery does not
/// hold together where the plan read it: a stored reference that did
/// not resolve, or a cycle that did not close. Invalid input, not a
/// frontier.
pub(super) fn not_intact(at: EntityId, detail: &'static str) -> FilletError {
    FilletError::BodyNotIntact { at, detail }
}

/// **Row 2** — the chain's own shape is outside the built door.
pub(super) fn unbuilt_chain(edge: EdgeKey, detail: &'static str) -> FilletError {
    FilletError::UnsupportedChain { edge, detail }
}

/// **Row 2** — the corner's own CONFIGURATION is not the sphere octant
/// (the OQ6 vocabulary, shared with the battery's classifier).
pub(super) fn unbuilt_corner_config(vertex: VertexKey, corner: CornerConfig) -> FilletError {
    FilletError::FilletCornerUnsupported {
        vertex,
        corner,
        policy: RunOutPolicy::RunOutStopAtVertex,
    }
}

/// **Row 2** — the REQUEST does not cover a termination the octant
/// assembly needs. A run-out, not a corner configuration.
pub(super) fn unbuilt_run_out(at: EntityId, detail: &'static str) -> FilletError {
    FilletError::UnsupportedRunOut { at, detail }
}

/// The one sentence for "a support of a corner is not a plane". Four
/// branches in two modules observe exactly this fact; sharing the
/// string is what stops them wording it four ways.
pub(super) const CORNER_SUPPORT_NOT_PLANAR: &str =
    "a corner support face is not a plane; the corner patch is built over three planes only";

/// **Row 2** — a stored carrier, trimline or surface is not a shape
/// the surgery's closed forms cover.
pub(super) fn unbuilt_geometry(at: EntityId, detail: &'static str) -> FilletError {
    FilletError::UnsupportedGeometry { at, detail }
}

/// **An Euler operator refused during assembly**, kept whole and
/// tagged with the surgery step that ran it.
///
/// A plain function, not a closure factory: the step name is an
/// argument at every call rather than a value captured once per phase,
/// so `FilletError::Op` cannot be constructed here without naming its
/// site, and the operator's own typed refusal — `StaleKey`,
/// `Certification`, the whole vocabulary — travels intact.
fn op(site: &'static str, source: topo::EulerOpError) -> FilletError {
    FilletError::Op { site, source }
}

/// The one new margin this unit decides (module docs): the exact
/// clearance between a support face's ring and a blend trimline.
/// A K row name reaching the funnel through a const, not a literal at
/// the decide site, so it is a roster carrier (`docs/K-REPORT.md`,
/// "The inventory method, restated").
const RING_CLEARANCE: &str = "fillet3_ring_clearance";

// ------------------------------------------------------------------
// The plan: everything classified and derived before any mutation.
// ------------------------------------------------------------------

/// One corner: a trivalent vertex all of whose edges are requested.
struct Corner<'a, T: Real> {
    /// The admitted open links terminating here — at least one, every
    /// one convex ([`CornerLinks`]).
    links: CornerLinks<'a, T>,
    /// The three incident support faces, in orbit order.
    faces: CornerFaces,
    /// The corner patch's foot on each of [`Corner::faces`], in that
    /// same orbit order — where this corner's two trimlines cross on
    /// each support. The strut every support face is carved with runs
    /// out to its own foot, so this array is the whole geometric
    /// difference between the two verbs' carves.
    feet: [Point3<T>; 3],
    /// What the corner patch's bounding edges turn about: the rolling
    /// ball's rest centre and its radius. `None` for a chamfer, whose
    /// patch is bounded by straight chords — there is nothing to turn
    /// about, which is the whole difference at a corner.
    arc: Option<(Point3<T>, T)>,
    /// The corner patch's surface: the sphere octant's chart (the
    /// order-free pick, [`super::build::octant_chart`]), or the
    /// chamfer's plane through the three feet.
    surface: Surface<T>,
    /// The octant's orientation bit, read exactly as a blend reads its
    /// own — off the stored convexity verdict
    /// ([`Convexity::blend_sense`]), never a sampled normal. A corner
    /// patch is a sphere about the rolling ball's rest centre whose
    /// chart normal is the outward radial, and the centre lies on the
    /// material side precisely when the corner is convex. Any one
    /// incident link answers for all of them: the surgery's door
    /// admits convex links only, so they cannot disagree.
    convexity: Convexity,
}

/// One closed chain resolved onto its supports.
///
/// The HOST side is ONE face. The MATE side is per-arc: on a LADDER
/// rim a revolve-minted cap arrives as half-cap faces split by meridian
/// seam edges through the pole, so each rim arc bounds its own mate
/// face and consecutive arcs meet at a rim vertex where exactly one
/// MERIDIAN edge descends into the cap. [`RimShape`] carries what is
/// true of each shape beyond that.
struct RimPlan<'a, T: Real> {
    chain: &'a Chain<T>,
    /// The support the carve treats as the FIRST side — the one whose
    /// strip is merged away and whose seam's rim-side piece dies at the
    /// closure. It is the PLANAR support whenever the rim has one (a
    /// ladder rim always does, and so does a plane–sphere annulus); a
    /// rim between two curved walls has no planar side and takes the
    /// link's own `face_a`, which is a slot and not a kind.
    host: FaceKey,
    /// The other support of each link, per chain-link order.
    mates: Vec<FaceKey>,
    /// Which of the two closed-rim shapes this is.
    shape: RimShape,
}

/// The two shapes a closed rim's band takes, which are different
/// surgeries and not two settings of one.
enum RimShape {
    /// **The quad ladder.** The rim is a RING of its planar support and
    /// each link's sphere face is a half-cap carrying exactly that arc:
    /// the band is carved as a ladder of struts and trim arcs around
    /// the ring, closed by one slit at the closure vertex.
    Ladder {
        /// The rim ring loop on the plane side.
        ring: LoopKey,
    },
    /// **The annulus.** One closed edge, so the band has two closed
    /// boundary circles and no ladder to walk. Both supports are
    /// revolution WALLS of ANY analytic kind — a full revolve mints each
    /// profile segment as one face whose single cycle carries two closed
    /// latitude rims and a double-traversed seam meridian, and that
    /// shape is what the carve needs, not a plane and a sphere. The band
    /// is minted as one more wall of the same shape.
    Annulus(AnnulusRim),
}

/// A one-link closed rim resolved onto the two walls it separates.
struct AnnulusRim {
    /// The rim's single vertex: both seams meet the rim here, and it is
    /// the azimuth at which the band is slit.
    vertex: VertexKey,
    /// The HOST wall's seam meridian at [`AnnulusRim::vertex`] — the
    /// edge its loop traverses twice. Split at the host trimline, it
    /// supplies the band's foot instead of a strut `mev`.
    host_seam: EdgeKey,
    /// The MATE wall's seam meridian at [`AnnulusRim::vertex`]. Split
    /// at the mate trimline, its rim-side piece becomes the slit.
    mate_seam: EdgeKey,
}

impl<T: Real> RimPlan<'_, T> {
    /// The host-side ring loop, for a ladder rim only: an annulus
    /// rim's host support carries the rim in its own boundary cycle,
    /// not as a ring.
    fn ladder_ring(&self) -> Option<LoopKey> {
        match self.shape {
            RimShape::Ladder { ring } => Some(ring),
            RimShape::Annulus(_) => None,
        }
    }
}

/// A rim edge's stored circle carrier, read once.
struct RimCarrier<T: Real> {
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    t0: T,
    t1: T,
    /// Whether the host-side half is the edge's `he_plus`.
    plus_on_host: bool,
}

/// **The assembly front door + construction** — called by
/// [`super::build::fillet_edges`] AFTER the battery, for every
/// request. The verdict's chains are the input; nothing re-derives
/// what the battery already resolved.
pub(super) fn blend_surgery<T: Decide + Bounds>(
    source: &Body<T>,
    verdict: &BatteryVerdict<T>,
    band: Band,
    tol: Tol,
) -> Result<Filleted<T>, FilletError> {
    let (solids, shells) = (source.solids().count(), source.shells().count());
    if solids != 1 || shells != 1 {
        return Err(FilletError::UnsupportedBody { solids, shells });
    }
    let radius = verdict.radius;
    let kind = verdict.kind;

    // ---- Classify the verdict's chains (structural only). The open
    // chains go through the door as [`ConvexOpen`], which IS the
    // three-clause admission below. ----
    let mut opens: Vec<ConvexOpen<'_, T>> = Vec::new();
    let mut rims: Vec<RimPlan<'_, T>> = Vec::new();
    for chain in &verdict.chains {
        match chain.closure {
            ChainClosure::Open { .. } => opens.push(ConvexOpen::admit(chain)?),
            // The band replacement is the rolling ball's torus over a
            // closed rim, whatever kinds its two supports are. A chamfer has no closed-chain band at
            // all — its one arm is plane–plane, whose closed chains
            // would need a ruled ring the surgery does not carve — so
            // it refuses here rather than entering the rim phase with
            // a strip.
            ChainClosure::Closed => match kind {
                BlendKind::Fillet => rims.push(resolve_rim(source, chain)?),
                BlendKind::Chamfer => {
                    return Err(unbuilt_chain(
                        chain.first().edge,
                        "a closed chamfer chain has no band; only open chains between \
                         trivalent corners are implemented",
                    ));
                }
            },
        }
    }
    opens.sort_by_key(ConvexOpen::edge);
    rims.sort_by_key(|r| r.chain.first().edge);
    shared_support_gate(&rims)?;

    // ---- Corners: every open-link end must be a fully-requested
    // trivalent vertex. Each end's incidence list is seeded by the link
    // that discovered it, so it is non-empty by shape rather than by a
    // check three functions deep. ----
    let mut ends: Vec<CornerLinks<'_, T>> = Vec::new();
    for o in &opens {
        for v in [o.link().start, o.link().end] {
            match ends.iter_mut().find(|c| c.vertex() == v) {
                Some(c) => c.also(*o)?,
                None => ends.push(CornerLinks::seed(v, *o)?),
            }
        }
    }
    ends.sort_by_key(CornerLinks::vertex);
    let mut corners: Vec<Corner<'_, T>> = Vec::new();
    for links in ends {
        let v = links.vertex();
        let Some(mut incident) = vertex_edges_of(source, v) else {
            return Err(not_intact(
                EntityId::Vertex(v),
                "a chain end's vertex orbit does not walk",
            ));
        };
        incident.sort_unstable();
        let mut here: Vec<EdgeKey> = links.sorted().iter().map(|l| l.edge()).collect();
        here.dedup();
        // Two different refusals, and they are not the same class: the
        // valence is the corner's own configuration (the OQ6
        // vocabulary the battery's classifier already speaks), while
        // "three edges, not all requested" is a property of the
        // REQUEST at a corner whose shape is the supported one.
        if incident.len() != 3 {
            return Err(unbuilt_corner_config(
                v,
                CornerConfig::NEdgeVertex {
                    valence: incident.len(),
                },
            ));
        }
        if here != incident {
            return Err(unbuilt_run_out(
                EntityId::Vertex(v),
                "a chain terminates at a trivalent vertex whose three edges are not all \
                 requested; run-outs at such corners are not implemented",
            ));
        }
        corners.push(corner_plan(source, links, radius, kind)?);
    }

    // ---- The support faces, admitted before anything is carved: each
    // one's ENTIRE outer cycle must be requested, which is what makes
    // the blank phase's carve well-defined. ----
    let mut support_keys: Vec<FaceKey> = Vec::new();
    for o in &opens {
        for f in [o.link().face_a, o.link().face_b] {
            if !support_keys.contains(&f) {
                support_keys.push(f);
            }
        }
    }
    support_keys.sort_unstable();
    let corner_rows: Vec<(VertexKey, &CornerFaces, [Point3<T>; 3])> = corners
        .iter()
        .map(|c| (c.links.vertex(), &c.faces, c.feet))
        .collect();
    let mut supports: Vec<RequestedBoundary<T>> = Vec::with_capacity(support_keys.len());
    for f in support_keys {
        supports.push(RequestedBoundary::admit(source, f, &opens, &corner_rows)?);
    }

    // ---- The ring carry-through honesty check (the one decision this
    // module adds — module docs). ----
    ring_clearance_pass(source, &opens, &rims, band)?;

    // ---- Mutation, on a clone. From here on every step is an Euler
    // operator or a certified setter; refusals map to Op/Certify. ----
    let mut body = source.clone();
    let Some((solid, _)) = source.solids().next() else {
        unreachable!("fillet surgery: `solids().count() == 1` was checked at entry")
    };
    let Some((shell, _)) = source.shells().next() else {
        unreachable!("fillet surgery: `shells().count() == 1` was checked at entry")
    };

    let mut rec = FilletNaming::default();
    let (blend_faces, corner_faces, mut described) =
        blank_phase(&mut body, &opens, &corners, &supports, &mut rec, tol, kind)?;
    let mut band_faces = Vec::with_capacity(rims.len());
    let mut band_surfaces = Vec::with_capacity(rims.len());
    for rim in &rims {
        let (band_face, band_surface, mut arcs) = match &rim.shape {
            RimShape::Ladder { ring } => rim_phase(&mut body, rim, *ring, &mut rec, tol)?,
            RimShape::Annulus(ann) => rim_phase_annulus(&mut body, rim, ann, &mut rec, tol)?,
        };
        band_faces.push(band_face);
        band_surfaces.push(band_surface);
        described.append(&mut arcs);
    }

    // ---- Surfaces and senses first (attach.rs: attach surfaces
    // before upgrading edge descriptions), then every new edge's
    // intrinsic description, then the pcurve re-mint (the input's
    // caches are stale the moment the first strut lands). ----
    // **The sense bit is the band's, not the verb's.** A rolling-ball
    // band's chart normal is the radial one, which is outward exactly
    // on a convex chain — so it folds the stored convexity verdict. A
    // chamfer's chart normal is minted as an explicit positive
    // combination of the supports' own OUTWARD normals
    // (`blend::chamfer_strip`, `blend::chamfer_corner_patch`), so it is
    // outward whatever the convexity is, and reading the verdict here
    // would flip a face that was already right.
    let band_sense = |convexity: Convexity| match kind {
        BlendKind::Fillet => convexity.blend_sense(),
        BlendKind::Chamfer => true,
    };
    for (i, o) in opens.iter().enumerate() {
        let fk = blend_faces[i];
        body.set_face_surface(fk, FaceSurface::New(o.link().blend.surface.clone()))
            .map_err(|e| op("blend face surface", e))?;
        body.set_face_sense(fk, band_sense(o.convexity()))
            .map_err(|e| op("blend face sense", e))?;
    }
    for (i, c) in corners.iter().enumerate() {
        let fk = corner_faces[i];
        body.set_face_surface(fk, FaceSurface::New(c.surface.clone()))
            .map_err(|e| op("corner patch surface", e))?;
        body.set_face_sense(fk, band_sense(c.convexity))
            .map_err(|e| op("corner patch sense", e))?;
    }
    for (i, rim) in rims.iter().enumerate() {
        let fk = band_faces[i];
        body.set_face_surface(fk, FaceSurface::New(band_surfaces[i].clone()))
            .map_err(|e| op("band face surface", e))?;
        body.set_face_sense(fk, rim.chain.first().convexity.blend_sense())
            .map_err(|e| op("band face sense", e))?;
    }
    for (edge, carrier) in described {
        attach_contact(&mut body, edge, carrier, tol)?;
    }
    topo::mint_pcurves(&mut body, tol).map_err(|source| FilletError::Certify {
        site: "pcurve re-mint after surgery",
        source,
    })?;

    #[cfg(debug_assertions)]
    debug_assert_eq!(
        topo::validate_closed(&body),
        Ok(()),
        "surgery postcondition: the result is not tier-2 valid (kernel bug)",
    );

    rec.dead.edges.sort_unstable();
    rec.dead.edges.dedup();
    rec.dead.vertices.sort_unstable();
    rec.dead.vertices.dedup();
    Ok(Filleted {
        body,
        solid,
        shell,
        blend_faces,
        corner_faces,
        band_faces,
        naming: Some(rec),
    })
}

// ------------------------------------------------------------------
// Plan helpers (read-only).
// ------------------------------------------------------------------

/// A vertex's incident edges, sorted (the corner front-door check).
fn vertex_edges_of<T: Decide>(body: &Body<T>, vertex: VertexKey) -> Option<Vec<EdgeKey>> {
    let he = body.get_vertex(vertex)?.emanating?;
    let mut edges: Vec<EdgeKey> = body
        .vertex_orbit(he)?
        .iter()
        .filter_map(|h| body.get_half_edge(*h).map(|x| x.edge))
        .collect();
    edges.sort_unstable();
    edges.dedup();
    Some(edges)
}

/// The corner patch at one fully-requested trivalent vertex: its
/// surface, and its foot on each of the three supports.
///
/// The two verbs differ here and only here at a corner.
///
/// - **Fillet**: the ball at rest touches all three supports, so its
///   foot on each is the ball centre projected onto it, and that point
///   lies on both of that support's trimlines because the centre is on
///   both incident spines ([`super::build::octant_chart`] picks the
///   octant's chart).
/// - **Chamfer**: there is no ball, so each foot is derived from the
///   trimlines directly — the two incident strips' trimlines on that
///   support, crossed in closed form ([`line_meet`]) — and the patch
///   is the plane through the three feet ([`chamfer_corner_patch`]).
///
/// **Convexity does not appear in the chamfer's arm.** The feet come
/// from trimlines whose in-plane direction is read off the traversal,
/// and the patch's chart normal is folded outward against the supports'
/// own normal sum; both are stated in [`super::blend`]. So there is no
/// convex-only argument here to derive one of and leave the rest
/// stale (#644's shape): the concave widening moves the ADMISSION
/// doors and nothing in this derivation.
fn corner_plan<'a, T: Decide + Bounds>(
    body: &Body<T>,
    links: CornerLinks<'a, T>,
    radius: T,
    kind: BlendKind,
) -> Result<Corner<'a, T>, FilletError> {
    let vertex = links.vertex();
    // The caller walked this vertex's edge orbit successfully, which
    // proves the orbit half of this walk; the `parent_loop` deref
    // `vertex_faces` adds is a stored reference nothing here proves.
    // The valence the octant derivation needs is the FACE orbit's; on a
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
    // doc): the door admits convex links only.
    let convexity = links.first().convexity();
    let (arc, feet, surface) = match kind {
        BlendKind::Fillet => {
            let ball = corner_ball([p; 3], normals, radius, true);
            // The ball at rest is at distance `radius` inside every
            // support, so its foot on each is the centre displaced
            // back along that support's outward normal — and that
            // point is on both of the support's trimlines, because the
            // centre is on both incident spines.
            let mut feet = [ball.center; 3];
            for (foot, &n) in feet.iter_mut().zip(normals.iter()) {
                *foot = ball.center + n * radius;
            }
            let (u_ref, axis) = super::build::octant_chart(body, &faces, &links)?;
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
) -> Result<[Point3<T>; 3], FilletError> {
    let here = links.sorted();
    let mut feet = [vertex_point; 3];
    for (slot, &face) in faces.as_slice().iter().enumerate() {
        let mut on_face = here.iter().filter_map(|o| {
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

/// Resolve one closed chain onto its two supports, with every
/// structural precondition of the band replacement checked.
///
/// **Which gates are per-KIND and which are per-SHAPE.** The band a
/// closed rim replaces is a torus, and that is the only geometric claim
/// this resolution rests on — so the arm gate asks for a torus arm
/// ([`super::blend::BlendArm::is_coaxial_torus`]) and not for one pair of kinds. Below
/// it the two shapes part company:
///
/// - the ANNULUS is two revolution WALLS and needs no planar support at
///   all: its trimlines are latitude circles on each support, minted per
///   kind by the arm, and the carve reads only the walls' seam
///   meridians. Every kind gate is dropped here, deliberately;
/// - the LADDER is a ring-and-half-caps configuration, and its gates —
///   one shared PLANE support, the rim as a RING of it, ring-free mate
///   caps each carrying exactly one arc — are that configuration's own.
///   They stay, and the refusals name the shape rather than the kinds.
///
/// The CONCAVE gate stays for both: a concave chain adds material, which
/// no closed-rim carve in this module builds.
fn resolve_rim<'a, T: Decide + Bounds>(
    body: &Body<T>,
    chain: &'a Chain<T>,
) -> Result<RimPlan<'a, T>, FilletError> {
    let link0 = chain.first();
    let is_plane = |f: FaceKey| -> Option<bool> {
        let fd = body.get_face(f)?;
        Some(matches!(
            body.get_surface(fd.surface)?,
            Surface::Plane { .. }
        ))
    };
    for link in chain.links() {
        if !link.arm.is_coaxial_torus() {
            return Err(unbuilt_chain(
                link.edge,
                "a closed chain's blend is not a torus (the torus band is the only \
                 closed blend built)",
            ));
        }
        if !matches!(link.convexity, Convexity::Convex) {
            return Err(unbuilt_chain(
                link.edge,
                "a concave chain adds material, which the surgery does not build — \
                 not implemented",
            ));
        }
    }

    // A one-link rim is a different band and a different surgery: its
    // supports are revolution WALLS, not a ring-and-cap pair, so it
    // resolves against the wall shape rather than the ring one — and it
    // asks nothing about either support's kind.
    if chain.link_count() == 1 {
        let a_planar = is_plane(link0.face_a).ok_or_else(|| {
            not_intact(EntityId::Face(link0.face_a), "a rim link's first support")
        })?;
        let b_planar = is_plane(link0.face_b).ok_or_else(|| {
            not_intact(EntityId::Face(link0.face_b), "a rim link's second support")
        })?;
        // The host is the planar support when there is one, so a
        // plane-and-curved rim carves exactly as it always has; between
        // two curved walls the roles are the link's own slots.
        let host = if b_planar && !a_planar {
            link0.face_b
        } else {
            link0.face_a
        };
        let mate = if host == link0.face_a {
            link0.face_b
        } else {
            link0.face_a
        };
        let host_half = host_side_half(body, link0, host)
            .ok_or_else(|| not_intact(EntityId::Edge(link0.edge), "a rim edge"))?;
        let host_loop = body
            .get_half_edge(host_half)
            .map(|h| h.parent_loop)
            .ok_or_else(|| {
                not_intact(EntityId::HalfEdge(host_half), "a rim edge's host-side half")
            })?;
        let shape = resolve_annulus(body, link0, mate, host_loop, host_half)?;
        return Ok(RimPlan {
            chain,
            host,
            mates: vec![mate],
            shape,
        });
    }

    // ---- The LADDER, and its own configuration's gates. ----
    let mut plane = None;
    let mut mates = Vec::with_capacity(chain.link_count());
    for link in chain.links() {
        let (p, s) = match is_plane(link.face_a) {
            Some(true) => (link.face_a, link.face_b),
            Some(false) => (link.face_b, link.face_a),
            None => {
                return Err(not_intact(
                    EntityId::Face(link.face_a),
                    "a rim link's first support",
                ));
            }
        };
        if !is_plane(p).unwrap_or(false) {
            return Err(unbuilt_chain(
                link.edge,
                "a multi-link closed chain has no planar support; the quad-ladder band \
                 is carved around a RING of one, and the ring-free annulus band is the \
                 one-edge rim's",
            ));
        }
        if *plane.get_or_insert(p) != p {
            return Err(unbuilt_chain(
                link.edge,
                "a closed chain's links do not share one plane support",
            ));
        }
        mates.push(s);
    }
    let Some(plane) = plane else {
        unreachable!(
            "resolve_rim: the loop above runs at least once (a chain always carries its \
             first link) and every pass sets `plane`"
        )
    };
    let plane_half = host_side_half(body, link0, plane)
        .ok_or_else(|| not_intact(EntityId::Edge(link0.edge), "a rim edge"))?;
    let plane_loop = body
        .get_half_edge(plane_half)
        .map(|h| h.parent_loop)
        .ok_or_else(|| {
            not_intact(
                EntityId::HalfEdge(plane_half),
                "a rim edge's plane-side half",
            )
        })?;

    // The rim must be a RING of the plane; each arc's mate face is a
    // ring-free cap piece carrying exactly that one chain arc on its
    // boundary (revolve-minted half-caps).
    let ring = plane_loop;
    let pd = body
        .get_face(plane)
        .ok_or_else(|| not_intact(EntityId::Face(plane), "a rim's plane support"))?;
    if !pd.rings.contains(&ring) {
        return Err(unbuilt_chain(
            link0.edge,
            "a closed chain is not a ring of its plane support",
        ));
    }
    let chain_edges: Vec<EdgeKey> = chain.links().map(|l| l.edge).collect();
    for (link, &s) in chain.links().zip(mates.iter()) {
        let sd = body
            .get_face(s)
            .ok_or_else(|| not_intact(EntityId::Face(s), "a rim's curved support"))?;
        if !sd.rings.is_empty() {
            return Err(unbuilt_chain(
                link.edge,
                "a rim's curved support carries rings of its own",
            ));
        }
        let on_boundary: Vec<EdgeKey> = face_cycle(body, s)
            .ok_or_else(|| {
                not_intact(
                    EntityId::Face(s),
                    "a rim's curved support has no boundary cycle that walks",
                )
            })?
            .iter()
            .filter_map(|he| body.get_half_edge(*he).map(|h| h.edge))
            .filter(|e| chain_edges.contains(e))
            .collect();
        if on_boundary != [link.edge] {
            return Err(unbuilt_chain(
                link.edge,
                "a curved support does not carry exactly its own rim arc (the \
                 half-cap discipline the band replacement needs)",
            ));
        }
    }
    // The ring cycle must be exactly the chain, and every rim vertex
    // must drop exactly ONE meridian into the cap (checked while
    // carving — structurally here: valence 3).
    let ring_len = loop_walk(body, ring)
        .ok_or_else(|| not_intact(EntityId::Loop(ring), "a rim's ring loop"))?
        .len();
    if ring_len != chain.link_count() {
        return Err(unbuilt_chain(
            link0.edge,
            "a rim ring carries edges outside the requested chain",
        ));
    }
    Ok(RimPlan {
        chain,
        host: plane,
        mates,
        shape: RimShape::Ladder { ring },
    })
}

/// **One support face, one band per call.** Every plan in this module
/// is resolved against the SOURCE body before anything is carved (the
/// decide-first discipline), but the bands are then carved one after
/// another into one clone. A LADDER rim's carve is confined to its own
/// ring and the caps that ring bounds, so two of them never meet. An
/// ANNULUS rim's carve reaches its supports' SEAM MERIDIANS, which a
/// revolution wall has exactly one of and therefore SHARES with the
/// wall's other latitude rim: the first band splits that seam and hands
/// its rim-side piece to a slit, and the second band's plan — resolved
/// against the source — still names a seam key that no longer spans its
/// own trimline crossing.
///
/// So the request is refused here, before any mutation, rather than
/// mid-carve on a stale key with a detail about geometry that is
/// perfectly fine. The recourse is real and exact: **sequential calls
/// compose** — filleting one rim, then the other on the result, builds
/// both bands and matches the closed form (`verbs_arms1_r1_probes`'s
/// sequential row). Widening this to one call needs per-rim plan
/// refresh between carves, which is a different shape from
/// decide-everything-first and is scheduled as its own change (#935).
fn shared_support_gate<T: Real>(rims: &[RimPlan<'_, T>]) -> Result<(), FilletError> {
    for (i, a) in rims.iter().enumerate() {
        if !matches!(a.shape, RimShape::Annulus(_)) {
            continue;
        }
        for b in rims.iter().skip(i + 1) {
            let shares = a.host == b.host
                || a.mates.iter().any(|s| b.mates.contains(s))
                || b.mates.contains(&a.host)
                || a.mates.contains(&b.host);
            if shares {
                return Err(unbuilt_chain(
                    b.chain.first().edge,
                    "two closed rims of one request share a support face, and a one-edge \
                     rim's band consumes that face's seam meridian — fillet them in \
                     SEQUENTIAL calls (the second on the first's result), which composes \
                     exactly; one call is not implemented",
                ));
            }
        }
    }
    Ok(())
}

/// Resolve a ONE-LINK closed rim onto the two revolution walls it
/// separates, with every structural precondition of the annulus band
/// checked.
///
/// A wall is what a full revolve mints for one profile segment: a face
/// whose single boundary cycle carries two closed latitude rims and one
/// seam meridian, traversed twice. That SHAPE is the whole hypothesis —
/// neither support's kind enters, so a sphere-and-cone rim resolves by
/// exactly the checks a plane-and-sphere one does. The band replacing
/// this rim is one more wall of that shape, and its two feet come from
/// splitting the two seams — so both must be there, and both must meet
/// the rim at its one vertex.
fn resolve_annulus<T: Decide + Bounds>(
    body: &Body<T>,
    link0: &Link<T>,
    mate: FaceKey,
    host_loop: LoopKey,
    host_half: HalfEdgeKey,
) -> Result<RimShape, FilletError> {
    if link0.start != link0.end {
        return Err(unbuilt_chain(
            link0.edge,
            "a one-link closed chain whose edge is not itself closed",
        ));
    }
    let vertex = link0.start;
    let sd = body
        .get_face(mate)
        .ok_or_else(|| not_intact(EntityId::Face(mate), "a rim's second support"))?;
    if !sd.rings.is_empty() {
        return Err(unbuilt_chain(
            link0.edge,
            "a rim's second support carries rings of its own",
        ));
    }
    let ed = body
        .get_edge(link0.edge)
        .ok_or_else(|| not_intact(EntityId::Edge(link0.edge), "a rim edge"))?;
    let mate_half = if ed.he_plus == host_half {
        ed.he_minus
    } else {
        ed.he_plus
    };
    let mate_loop = loop_of_half(body, mate_half)
        .ok_or_else(|| not_intact(EntityId::HalfEdge(mate_half), "a rim edge's mate-side half"))?;
    let host_seam = wall_seam(body, host_loop, link0.edge, vertex)?;
    let mate_seam = wall_seam(body, mate_loop, link0.edge, vertex)?;
    // The rim vertex carries the rim and the two seams and nothing else:
    // the band's slit is minted from the sphere seam's rim-side piece
    // and the plane seam's rim-side piece dies with this vertex, so a
    // third incident edge would be left behind by both.
    let mut incident = vertex_edges_of(body, vertex)
        .ok_or_else(|| not_intact(EntityId::Vertex(vertex), "a rim vertex's edge orbit"))?;
    incident.sort_unstable();
    let mut expected = vec![link0.edge, host_seam, mate_seam];
    expected.sort_unstable();
    if incident != expected {
        return Err(unbuilt_chain(
            link0.edge,
            "a one-edge rim's vertex carries more than the rim and its two supports' seam \
             meridians; the annulus band is built for revolution walls only",
        ));
    }
    Ok(RimShape::Annulus(AnnulusRim {
        vertex,
        host_seam,
        mate_seam,
    }))
}

/// The seam meridian of a revolution wall's boundary cycle: the one
/// edge the cycle traverses TWICE and which meets the rim at `vertex`.
/// The rim itself must be carried once, so the wall really is the
/// one-rim-per-side shape the band replacement assumes.
fn wall_seam<T: Decide>(
    body: &Body<T>,
    lp: LoopKey,
    rim: EdgeKey,
    vertex: VertexKey,
) -> Result<EdgeKey, FilletError> {
    let walk = loop_walk(body, lp)
        .ok_or_else(|| not_intact(EntityId::Loop(lp), "a rim support's boundary cycle"))?;
    let count = |e: EdgeKey| walk.iter().filter(|(_, _, k)| *k == e).count();
    if count(rim) != 1 {
        return Err(unbuilt_chain(
            rim,
            "a one-edge rim is carried more than once by a support's boundary cycle",
        ));
    }
    let mut seams: Vec<EdgeKey> = Vec::new();
    for &(_, _, e) in &walk {
        if e != rim && count(e) == 2 && edge_touches(body, e, vertex) && !seams.contains(&e) {
            seams.push(e);
        }
    }
    let [seam] = seams[..] else {
        return Err(unbuilt_chain(
            rim,
            "a one-edge rim's support is not a revolution wall (no single doubly-traversed \
             seam meridian at the rim vertex); the annulus band is not built for it",
        ));
    };
    Ok(seam)
}

/// The half-edge of `link.edge` lying on face `host`'s side.
fn host_side_half<T: Decide>(body: &Body<T>, link: &Link<T>, host: FaceKey) -> Option<HalfEdgeKey> {
    if link.face_a == host {
        Some(link.he_plus)
    } else {
        let e = body.get_edge(link.edge)?;
        Some(if e.he_plus == link.he_plus {
            e.he_minus
        } else {
            e.he_plus
        })
    }
}

/// A loop's cycle as `(half-edge, start vertex, edge)` rows, in cycle
/// order (D9: the stored anchor's order, never re-sorted).
fn loop_walk<T: Decide>(
    body: &Body<T>,
    lp: LoopKey,
) -> Option<Vec<(HalfEdgeKey, VertexKey, EdgeKey)>> {
    let topo::LoopBoundary::Cycle { first } = body.get_loop(lp)?.boundary else {
        return None;
    };
    let cycle = body.loop_cycle(first)?;
    let mut out = Vec::with_capacity(cycle.len());
    for he in cycle {
        let h = body.get_half_edge(he)?;
        out.push((he, h.start, h.edge));
    }
    Some(out)
}

// ------------------------------------------------------------------
// The ring carry-through check.
// ------------------------------------------------------------------

/// A ring read as one circle: every edge of the ring must carry a
/// `Circle` carrier on one shared centre/radius (the pip rims and the
/// widened trim circles — the only rings this kernel mints on planar
/// faces at rest). Anything else refuses typed rather than sampling.
fn ring_circle<T: Decide>(body: &Body<T>, ring: LoopKey) -> Result<(Point3<T>, T), FilletError> {
    let walk = loop_walk(body, ring)
        .ok_or_else(|| not_intact(EntityId::Loop(ring), "a support face's ring"))?;
    let mut found: Option<(Point3<T>, T)> = None;
    for (_, _, edge) in walk {
        let e = body
            .get_edge(edge)
            .ok_or_else(|| not_intact(EntityId::Edge(edge), "a ring edge"))?;
        let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            return Err(unbuilt_geometry(
                EntityId::Edge(edge),
                "a ring edge carries no certified carrier",
            ));
        };
        let Curve3::Circle { center, radius, .. } = *c.carrier() else {
            return Err(unbuilt_geometry(
                EntityId::Edge(edge),
                "a ring edge's carrier is not a circle — the exact ring-clearance \
                 check covers circle rings only",
            ));
        };
        // Key equality is not available across arcs of one rim (each
        // arc is its own curve row), so the shared-circle fact is
        // structural per mint and simply adopted from the first arc:
        // the clearance margin below uses one centre for the whole
        // ring, which is exact for every ring this kernel mints.
        found.get_or_insert((center, radius));
    }
    let Some(circle) = found else {
        unreachable!(
            "ring_circle: `loop_walk` above returned a cycle, and a cycle always \
             carries at least its anchor half-edge"
        )
    };
    Ok(circle)
}

/// A rim link's (host, mate) trim circles as `(center, radius)` pairs,
/// selected by SUPPORT FACE — the [`resolve_rim`] discipline — never by
/// slot order. `classify_arm` keys `trim_a` to `face_a`, and `face_a` is
/// whichever support carries `he_plus`, which the request does not
/// choose; the `(Sphere, Plane)` arm swaps its own trims for exactly
/// this reason. Callers pass `host_is_a = (link.face_a == rim.host)`;
/// reading `trim_a` blind would take the mate's trim for the host's on
/// exactly the links where the two disagree. Pinned by
/// `tests::trim_selection_is_by_support_kind`.
#[allow(clippy::type_complexity)]
fn rim_trim_circles<T: Real>(
    edge: EdgeKey,
    blend: &EdgeBlend<T>,
    host_is_a: bool,
) -> Result<((Point3<T>, T), (Point3<T>, T)), FilletError> {
    let (host_trim, mate_trim) = if host_is_a {
        (&blend.trim_a.0, &blend.trim_b.0)
    } else {
        (&blend.trim_b.0, &blend.trim_a.0)
    };
    let Curve3::Circle {
        center: pc,
        radius: pr,
        ..
    } = *host_trim
    else {
        return Err(unbuilt_geometry(
            EntityId::Edge(edge),
            "a rim blend's host trimline is not a circle",
        ));
    };
    let Curve3::Circle {
        center: sc,
        radius: sr,
        ..
    } = *mate_trim
    else {
        return Err(unbuilt_geometry(
            EntityId::Edge(edge),
            "a rim blend's mate trimline is not a circle",
        ));
    };
    Ok(((pc, pr), (sc, sr)))
}

/// **`fillet3_ring_clearance`** — decide one ring carry-through
/// margin (module docs): the exact closed-form clearance between a
/// support face's ring and a blend trimline, meters. Positive
/// carries the ring through; zero/negative refuses
/// [`FilletError::RingClearance`]; an in-band margin escalates with
/// the SAME recourse (two-tolerance, D4 ¶1 addendum — this arm is
/// trio-pinned like every `fillet3_*` predicate). Public for exactly
/// that trio: the margins themselves are derived inside the surgery
/// from stored trimlines and ring carriers, never sampled.
///
/// In practice predicate 2's sampled screen usually fires first on
/// the same configuration (for a straight edge the two margins are
/// the same length); this check is the EXACT form of it, and the one
/// the ring carry-through soundness argument actually rests on —
/// sampling can overestimate a gap, the closed form cannot. The
/// refuse arm is therefore FRONT-DOOR-SCREENED by predicate 2: it is
/// exercised directly by the trio pins, not through `fillet_edges`'
/// live assemblies.
///
/// # Errors
///
/// [`FilletError::RingClearance`] / [`FilletError::Escalated`].
pub fn ring_clearance<T: Decide + Bounds>(
    face: FaceKey,
    margin: T,
    band: Band,
) -> Result<(), FilletError> {
    match decide(RING_CLEARANCE, Margin::of(margin), band).map_err(|e| FilletError::Escalated {
        site: FilletSite::Chain,
        source: e,
    })? {
        Sign::Positive => Ok(()),
        _ => Err(FilletError::RingClearance {
            face,
            margin: margin.lo(),
        }),
    }
}

/// The pre-mutation honesty pass (module docs): every ring of every
/// touched support face must clear every blend trimline by a definite
/// margin, in closed form.
fn ring_clearance_pass<T: Decide + Bounds>(
    body: &Body<T>,
    opens: &[ConvexOpen<'_, T>],
    rims: &[RimPlan<'_, T>],
    band: Band,
) -> Result<(), FilletError> {
    // A ring's EFFECTIVE radius: its own circle, widened to the trim
    // circle when the ring is itself a requested rim (a single call
    // may blend the box edges and the rims together).
    let effective = |ring: LoopKey| -> Result<Option<(Point3<T>, T)>, FilletError> {
        for rim in rims {
            if rim.ladder_ring() == Some(ring) {
                let l0 = rim.chain.first();
                let (plane_trim, _) = rim_trim_circles(l0.edge, &l0.blend, l0.face_a == rim.host)?;
                return Ok(Some(plane_trim));
            }
        }
        Ok(None)
    };
    // (a) Open links: every ring of each support face against the
    // link's straight trimline on that face.
    for o in opens {
        let l = o.link();
        let mid = edge_midpoint(body, l.edge).ok_or_else(|| {
            not_intact(
                EntityId::Edge(l.edge),
                "a link edge's stored carrier, for its midpoint",
            )
        })?;
        for (face, trim) in [(l.face_a, &l.blend.trim_a.0), (l.face_b, &l.blend.trim_b.0)] {
            let Curve3::Line { origin, dir } = *trim else {
                return Err(unbuilt_geometry(
                    EntityId::Edge(l.edge),
                    "an open link's trimline is not a line",
                ));
            };
            // The inward unit: from the sharp edge toward the trim,
            // in the support plane (perpendicular to the trim by
            // construction of the setback).
            let m = (origin - mid).normalize();
            let fd = body
                .get_face(face)
                .ok_or_else(|| not_intact(EntityId::Face(face), "a link's support face"))?;
            for ring in fd.rings.clone() {
                let (c, a) = match effective(ring)? {
                    Some(widened) => widened,
                    None => ring_circle(body, ring)?,
                };
                // The trimline is unbounded within the face, so only
                // the transverse clearance matters, and `m ⊥ dir`
                // EXACTLY, not approximately: the battery seeds
                // `plane_plane_blend` with the carrier evaluated at
                // `(t0 + t1)/2`, the trim origin is that point
                // displaced by a combination of the two support
                // normals (both ⊥ the tangent `dir`), and
                // `edge_midpoint` below reproduces the SAME
                // `(t0 + t1)/2` construction on the same stored
                // carrier — so `origin - mid` is purely transverse by
                // shared construction, never by cancellation.
                let _ = dir;
                let margin = (c - origin).dot(m) - a;
                ring_clearance(face, margin, band)?;
            }
        }
    }
    // (b) Rims: each widened trim circle against the host face's
    // OTHER rings and its straight outer boundary edges.
    for rim in rims {
        let l0 = rim.chain.first();
        let ((ci, si), _) = rim_trim_circles(l0.edge, &l0.blend, l0.face_a == rim.host)?;
        let fd = body
            .get_face(rim.host)
            .ok_or_else(|| not_intact(EntityId::Face(rim.host), "a rim's host support"))?;
        for ring in fd.rings.clone() {
            if rim.ladder_ring() == Some(ring) {
                continue;
            }
            let (cj, aj) = match effective(ring)? {
                Some(widened) => widened,
                None => ring_circle(body, ring)?,
            };
            let margin = (cj - ci).norm() - si - aj;
            ring_clearance(rim.host, margin, band)?;
        }
        // Outer boundary — a LADDER rim's question only. There the rim
        // is a ring, so the host's outer boundary is a separate cycle
        // the widened trim circle must clear. An ANNULUS rim's trim
        // circle IS the replacement for part of that outer boundary and
        // is separated from the rest of it by predicate 2's
        // boundary-pair consumption sweep, which meters exactly those
        // pairs at the arm's own setbacks; running the external-
        // separation form here would refuse every such rim on its own
        // rim edge.
        //
        // **This is an exactness step-down for the annulus rim, said
        // plainly:** a ladder rim gets BOTH a sampled screen (predicate
        // 2) and this closed-form backstop, and an annulus rim gets the
        // sampled screen alone. What makes that hold today is that the
        // pairs at issue are the ones the screen is exact on anyway —
        // an annulus support's boundary is two coaxial latitude circles
        // and one meridian seam, and `CHAIN_SAMPLES` points on a circle
        // of a coaxial pair bound the true gap from above by a term
        // that vanishes with the sample spacing, never from below. It
        // stops holding the day an annulus support carries a boundary
        // whose closest approach is BETWEEN samples — a non-coaxial
        // ring, or a trimmed wall — which is also the day the outer
        // sweep's own external-separation form becomes applicable
        // again.
        let RimShape::Ladder { .. } = rim.shape else {
            continue;
        };
        // Scoped honestly: the line arm measures the
        // INFINITE carrier line, and the circle arm is EXTERNAL
        // separation (`‖cj − ci‖ − si − aj`) only — no containment
        // form. Both err in the conservative direction (a false
        // refusal, never a false pass); the two false-refusal
        // classes are (1) a trim circle NESTED inside a circular
        // outer boundary, where the containment margin
        // `aj − (‖cj − ci‖ + si)` is positive but the external form
        // reads negative, and (2) a distant line edge whose EXTENSION
        // passes near the trim circle. Neither occurs on the bodies
        // this kernel mints today (planar outer boundaries are convex
        // blank/trimline cycles); a body that hits one refuses
        // `RingClearance` loudly rather than passing silently.
        // Anything else is already screened by predicate 2's sampled
        // sweep and adds nothing exact here.
        let outer = face_cycle(body, rim.host).ok_or_else(|| {
            not_intact(
                EntityId::Face(rim.host),
                "a rim's host support has no outer cycle that walks",
            )
        })?;
        for he in outer {
            let Some(h) = body.get_half_edge(he) else {
                continue;
            };
            let Some(e) = body.get_edge(h.edge) else {
                continue;
            };
            let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
                continue;
            };
            match *c.carrier() {
                Curve3::Line { origin, dir } => {
                    let d = ci - origin;
                    let margin = (d - dir * d.dot(dir)).norm() - si;
                    ring_clearance(rim.host, margin, band)?;
                }
                Curve3::Circle { center, radius, .. } => {
                    let margin = (center - ci).norm() - si - radius;
                    ring_clearance(rim.host, margin, band)?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

/// A link edge's midpoint (the trimline inward direction's anchor).
fn edge_midpoint<T: Decide>(body: &Body<T>, edge: EdgeKey) -> Option<Point3<T>> {
    let e = body.get_edge(edge)?;
    let c = body.get_curve_geom(e.curve)?.certified()?;
    let (t0, t1) = c.params();
    Some(c.carrier().eval((t0 + t1) * T::from_f64(0.5)))
}

// ------------------------------------------------------------------
// The blank phase: open links and corners, in place.
// ------------------------------------------------------------------

/// A recorded new edge awaiting its intrinsic description.
enum ContactCarrier<T: Real> {
    /// A straight trimline where the band meets its support
    /// TANGENTIALLY — the rolling ball's contact line (carrier rebuilt
    /// from the edge's vertices).
    TrimLine,
    /// A straight edge where two surfaces meet TRANSVERSALLY: every
    /// edge a chamfer mints, since a strip meets its supports and its
    /// corner patches at a definite angle, and two planes crossing at
    /// an angle intersect exactly in the line between the endpoints.
    Chord,
    /// A corner arc about the corner ball's centre (sweep < π).
    CornerArc { center: Point3<T>, radius: T },
    /// An exact stored arc (the rim trim circles — π-safe).
    Exact(Curve3<T>, T, T),
    /// A torus band's SLIT: a double-traversed minor-circle arc
    /// described as a [`EdgeGeometry::Seam`] of the band's own
    /// surface (sweep < π; the donut's representation).
    SeamArc { center: Point3<T>, radius: T },
}

type Described<T> = Vec<(EdgeKey, ContactCarrier<T>)>;

#[allow(clippy::type_complexity)]
fn blank_phase<T: Decide + Bounds>(
    body: &mut Body<T>,
    opens: &[ConvexOpen<'_, T>],
    corners: &[Corner<'_, T>],
    supports: &[RequestedBoundary<T>],
    rec: &mut FilletNaming,
    tol: Tol,
    kind: BlendKind,
) -> Result<(Vec<FaceKey>, Vec<FaceKey>, Described<T>), FilletError> {
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
        body.kef(half_a).map_err(|e| op("edge-strip kef", e))?;
        hexagon.push((e, survivor_loop));
    }
    let hex_face = |body: &Body<T>, e: EdgeKey| -> Option<FaceKey> {
        let lp = hexagon.iter().find(|(ee, _)| *ee == e)?.1;
        Some(body.get_loop(lp)?.face)
    };

    // ---- Per corner: three arcs, then the octant fusion. ----
    let mut corner_faces = Vec::with_capacity(corners.len());
    for c in corners {
        let vertex = c.links.vertex();
        // Non-empty by the shape of `CornerLinks`, which is seeded by
        // the link that discovered this corner — so the arc loop below
        // always runs, which is what the `unreachable!`s after it rest
        // on.
        let links_here = c.links.sorted();
        let mut first_arc: Option<EdgeKey> = None;
        for o in &links_here {
            let l = o.link();
            let f = hex_face(body, l.edge)
                .ok_or_else(|| not_intact(EntityId::Edge(l.edge), "a merged strip's face"))?;
            let walk = loop_walk_face(body, f)
                .ok_or_else(|| not_intact(EntityId::Face(f), "a blend face's outer cycle"))?;
            let k = walk.len();
            let pos = (0..k)
                .find(|&i| walk[(i + 1) % k].1 == vertex)
                .ok_or_else(|| {
                    not_intact(
                        EntityId::Vertex(vertex),
                        "a corner is missing from the boundary of its own blend face",
                    )
                })?;
            let he1 = walk[pos].0;
            let he2 = walk[(pos + 2) % k].0;
            let (p1, p2) = (
                point_of(body, walk[pos].1).ok_or_else(|| {
                    not_intact(EntityId::Vertex(walk[pos].1), "an arc foot's vertex")
                })?,
                point_of(body, walk[(pos + 2) % k].1).ok_or_else(|| {
                    not_intact(
                        EntityId::Vertex(walk[(pos + 2) % k].1),
                        "an arc foot's vertex",
                    )
                })?,
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
            first_arc.get_or_insert(created.edge);
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
                        "corner fusion: exactly three struts on three distinct supports \
                         (checked immediately above) fuse to leave exactly one spur"
                    )
                }
                continue;
            }
            body.kef(hp).map_err(|e| op("corner-strut kef", e))?;
        }
        let Some(s) = spur else {
            unreachable!(
                "corner fusion: exactly three struts on three distinct supports (checked \
                 immediately above) fuse to leave exactly one spur"
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
        // The octant is whatever face the first arc's non-blend half
        // now bounds.
        let Some(arc) = first_arc else {
            unreachable!(
                "corner fusion: a corner's incidence list holds at least the link that \
                 discovered it, so the arc loop ran and `get_or_insert` set this on its \
                 first pass"
            )
        };
        let Some((ahp, ahm)) = halves_of(body, arc) else {
            unreachable!(
                "corner fusion: the arc was minted by the loop above and nothing \
                          between here and there kills it"
            )
        };
        let quad = hex_face(body, c.links.first().edge());
        let octant = match (face_of_half(body, ahp), face_of_half(body, ahm)) {
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
        rec.corners.push((octant, vertex));
        rec.dead.vertices.push(vertex);
        corner_faces.push(octant);
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

// ------------------------------------------------------------------
// The rim phase: one torus band per closed chain, in place.
// ------------------------------------------------------------------

/// One rim edge's stored circle carrier, plus which side its `he_plus`
/// lies on. The band inherits this frame rather than reconstructing it,
/// which is what keeps every arc's seam structure and parameter window
/// the rim's own.
fn rim_carrier<T: Decide>(
    body: &Body<T>,
    link: &Link<T>,
    host: FaceKey,
) -> Result<RimCarrier<T>, FilletError> {
    let e = link.edge;
    let ed = body
        .get_edge(e)
        .ok_or_else(|| not_intact(EntityId::Edge(e), "a rim edge"))?;
    let host_half = host_side_half(body, link, host)
        .ok_or_else(|| not_intact(EntityId::Edge(e), "a rim edge's host-side half"))?;
    let Some(c) = body.get_curve_geom(ed.curve).and_then(|g| g.certified()) else {
        return Err(unbuilt_geometry(
            EntityId::Edge(e),
            "a rim edge carries no certified carrier",
        ));
    };
    let Curve3::Circle { axis, u_ref, .. } = *c.carrier() else {
        return Err(unbuilt_geometry(
            EntityId::Edge(e),
            "a rim edge's carrier is not a circle; the band inherits the rim's \
             circular frame and no other stored shape is built",
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
/// with that side's loop — reversed by negating the axis and the
/// window, never by an endpoint `atan2` (π-arc safe).
fn scaled<T: Real>(
    rc: &RimCarrier<T>,
    center: Point3<T>,
    radius: T,
    forward: bool,
) -> (Curve3<T>, T, T) {
    if forward {
        (
            Curve3::Circle {
                center,
                axis: rc.axis,
                radius,
                u_ref: rc.u_ref,
            },
            rc.t0,
            rc.t1,
        )
    } else {
        (
            Curve3::Circle {
                center,
                axis: -rc.axis,
                radius,
                u_ref: rc.u_ref,
            },
            -rc.t1,
            -rc.t0,
        )
    }
}

/// The parameter at which a support's seam meridian meets `target`,
/// read in the SEAM CARRIER's own frame. A representation pick, not a
/// classification (the battery's junction-end pick precedent): a circle
/// carrier's angle is brought into the stored window by whole turns and
/// a line carrier's is the projection on its own direction, and either
/// way a parameter outside the stored span refuses typed rather than
/// cutting blind. `rim` names the requested edge the refusal carries.
fn seam_split_param<T: Decide + Bounds>(
    body: &Body<T>,
    seam: EdgeKey,
    rim: EdgeKey,
    target: Point3<T>,
) -> Result<T, FilletError> {
    let sd = body
        .get_edge(seam)
        .ok_or_else(|| not_intact(EntityId::Edge(seam), "a support's seam meridian"))?;
    let Some(sc) = body.get_curve_geom(sd.curve).and_then(|g| g.certified()) else {
        return Err(unbuilt_geometry(
            EntityId::Edge(seam),
            "a meridian carries no certified carrier",
        ));
    };
    let (st0, st1) = sc.params();
    match *sc.carrier() {
        Curve3::Circle {
            center,
            axis,
            u_ref,
            ..
        } => {
            let d = target - center;
            let traw = d.dot(axis.cross(u_ref)).atan2(d.dot(u_ref));
            let tau = T::from_f64(core::f64::consts::TAU);
            for k in [-2.0f64, -1.0, 0.0, 1.0, 2.0] {
                let cand = traw + tau * T::from_f64(k);
                if (cand - st0).lo() > 0.0 && (st1 - cand).lo() > 0.0 {
                    return Ok(cand);
                }
            }
        }
        Curve3::Line { origin, dir } => {
            let t = (target - origin).dot(dir);
            if (t - st0).lo() > 0.0 && (st1 - t).lo() > 0.0 {
                return Ok(t);
            }
        }
        _ => {
            return Err(unbuilt_geometry(
                EntityId::Edge(seam),
                "a meridian's carrier is neither a circle nor a line; the split reads the \
                 crossing in the meridian's own frame and no other stored shape is built",
            ));
        }
    }
    Err(unbuilt_chain(
        rim,
        "a trimline does not cross its support's seam meridian inside its span",
    ))
}

fn rim_phase<T: Decide + Bounds>(
    body: &mut Body<T>,
    rim: &RimPlan<'_, T>,
    ring: LoopKey,
    rec: &mut FilletNaming,
    tol: Tol,
) -> Result<(FaceKey, Surface<T>, Described<T>), FilletError> {
    let mut described: Described<T> = Vec::new();
    let link_of = |e: EdgeKey| -> Option<&Link<T>> { rim.chain.links().find(|l| l.edge == e) };
    // Selected by support kind, never by slot (`rim_trim_circles`
    // docs): `trim_a` is the SPHERE trim on any link whose `he_plus`
    // lies on the cap side.
    let l0 = rim.chain.first();
    let ((ca, sa), (cb, sb)) = rim_trim_circles(l0.edge, &l0.blend, l0.face_a == rim.host)?;

    // The rim edges' stored carriers, once.
    let carrier_of = |body: &Body<T>, e: EdgeKey| -> Result<RimCarrier<T>, FilletError> {
        let l = link_of(e)
            .ok_or_else(|| not_intact(EntityId::Edge(e), "a rim edge's link in the verdict"))?;
        rim_carrier(body, l, rim.host)
    };

    // ---- (1) The plane walk: the rim ring's cycle, once. Everything
    // downstream keys off its order (D9: the stored anchor's order).
    let plane_walk = loop_walk(body, ring)
        .ok_or_else(|| not_intact(EntityId::Loop(ring), "a rim's ring loop"))?;
    let n = plane_walk.len();

    // ---- (2) Meridian splits: at each rim vertex exactly one edge
    // descends into the cap (the revolve seam); split it where the
    // sphere trim circle crosses, minting the band's inner vertices
    // on EXISTING geometry rather than strutting into the cap. ----
    let chain_edges: Vec<EdgeKey> = rim.chain.links().map(|l| l.edge).collect();
    // Per plane-walk position: (rim vertex, upper remnant edge, the
    // SOURCE meridian it came from).
    let mut remnants: Vec<(VertexKey, EdgeKey, EdgeKey)> = Vec::with_capacity(n);
    for &(_, v, e) in &plane_walk {
        let incident = vertex_edges_of(body, v)
            .ok_or_else(|| not_intact(EntityId::Vertex(v), "a rim vertex's edge orbit"))?;
        let meridians: Vec<EdgeKey> = incident
            .into_iter()
            .filter(|k| !chain_edges.contains(k))
            .collect();
        let [m] = meridians[..] else {
            return Err(unbuilt_chain(
                e,
                "a rim vertex does not drop exactly one meridian into the cap; the band \
                 replacement is built for revolve-minted half-caps only",
            ));
        };
        // The split target: the sphere trim circle at this vertex's
        // azimuth — evaluated on the SCALED rim frame at the vertex's
        // own rim parameter, so the azimuth is inherited, not
        // reconstructed.
        let rc = carrier_of(body, e)?;
        let (tb_curve, tb_t0, _) = scaled(&rc, cb, sb, rc.plus_on_host);
        let target = tb_curve.eval(tb_t0);
        let t_split = seam_split_param(body, m, e, target)?;
        let created = body
            .split_edge(m, t_split, tol)
            .map_err(|e| op("meridian split", e))?;
        // The upper remnant is whichever piece still ends at the rim
        // vertex.
        let touches_v = |body: &Body<T>, e: EdgeKey| -> bool {
            halves_of(body, e).is_some_and(|(hp, hm)| {
                body.get_half_edge(hp).map(|h| h.start) == Some(v)
                    || body.get_half_edge(hm).map(|h| h.start) == Some(v)
            })
        };
        let upper = if touches_v(body, m) {
            m
        } else {
            created.new_edge
        };
        // Birth data: the split vertex and the LOWER (surviving)
        // piece are both fragments of this source meridian.
        rec.meridian_splits.push((created.vertex, m));
        let lower = if upper == m { created.new_edge } else { m };
        rec.meridian_remnants.push((lower, m));
        remnants.push((v, upper, m));
    }

    // ---- (3) The plane side: struts to the widened trim circle and
    // the trim arcs, carving the annular strips off the ring. ----
    let mut struts_p: Vec<(VertexKey, EdgeKey)> = Vec::with_capacity(n);
    let mut strut_hes: Vec<(HalfEdgeKey, Point3<T>)> = Vec::with_capacity(n);
    let mut ta_carriers = Vec::with_capacity(n);
    for &(he, v, e) in &plane_walk {
        let rc = carrier_of(body, e)?;
        let (curve, t0, t1) = scaled(&rc, ca, sa, rc.plus_on_host);
        let p = point_of(body, v).ok_or_else(|| not_intact(EntityId::Vertex(v), "a rim vertex"))?;
        // The foot inherits the rim vertex's own parameter on the
        // scaled carrier — azimuth preserved exactly, no atan2.
        let fp = curve.eval(t0);
        let created = body
            .mev(
                MevSite::Fan { he1: he, he2: he },
                fp,
                EdgeCurveSpec::line_between(p, fp),
                tol,
            )
            .map_err(|e| op("rim strut mev", e))?;
        struts_p.push((v, created.edge));
        rec.rim_feet.push((created.vertex, v));
        rec.dead.vertices.push(v);
        strut_hes.push((created.he_minus, fp));
        ta_carriers.push((curve, t0, t1));
    }
    let mut first_trim: Option<HalfEdgeKey> = None;
    for i in 0..n {
        let (he1, fp_i) = strut_hes[i];
        let fp_j = strut_hes[(i + 1) % n].1;
        let he2 = if i + 1 < n {
            strut_hes[i + 1].0
        } else {
            first_trim.ok_or_else(|| {
                unbuilt_chain(plane_walk[i].2, "a rim of a single edge is not implemented")
            })?
        };
        // Scaffold chord now (the corner-arc precedent); the exact
        // scaled arc is attached in the description pass, once the
        // band's surface exists.
        let created = body
            .mef(
                MefSite::Chords { he1, he2 },
                EdgeCurveSpec::line_between(fp_i, fp_j),
                FaceSurface::Inherit,
                tol,
            )
            .map_err(|e| op("rim trim mef", e))?;
        first_trim.get_or_insert(created.he_plus);
        rec.rim_trims
            .push((created.edge, plane_walk[i].2, RimSide::Plane));
        let (curve, t0, t1) = ta_carriers[i].clone();
        described.push((created.edge, ContactCarrier::Exact(curve, t0, t1)));
    }

    // ---- (4) The sphere side: one trim chord per half-cap, hung
    // between the two meridian split vertices around that cap piece's
    // own rim arc. ----
    let mut tb_edges: Vec<EdgeKey> = Vec::with_capacity(n);
    for &(he_p, _, e) in &plane_walk {
        let rc = carrier_of(body, e)?;
        let s_half = {
            let ed = body
                .get_edge(e)
                .ok_or_else(|| not_intact(EntityId::Edge(e), "a rim edge"))?;
            if ed.he_plus == he_p {
                ed.he_minus
            } else {
                ed.he_plus
            }
        };
        let lp = loop_of_half(body, s_half)
            .ok_or_else(|| not_intact(EntityId::HalfEdge(s_half), "a rim edge's cap-side half"))?;
        let walk = loop_walk(body, lp)
            .ok_or_else(|| not_intact(EntityId::Loop(lp), "a half-cap's loop"))?;
        let k = walk.len();
        let pos = walk
            .iter()
            .position(|(h, _, _)| *h == s_half)
            .ok_or_else(|| {
                not_intact(
                    EntityId::Loop(lp),
                    "a half-cap loop does not carry the half-edge whose parent it is",
                )
            })?;
        let he1 = walk[(pos + k - 1) % k].0;
        let he2 = walk[(pos + 2) % k].0;
        let (v1, v2) = (walk[(pos + k - 1) % k].1, walk[(pos + 2) % k].1);
        // Both run ends must be meridian split vertices (the half-cap
        // discipline): refuse rather than cut blind.
        if !remnants.iter().any(|(_, m, _)| edge_touches(body, *m, v1))
            || !remnants.iter().any(|(_, m, _)| edge_touches(body, *m, v2))
        {
            return Err(unbuilt_chain(
                e,
                "a half-cap's rim arc is not flanked by meridian split points; the band \
                 replacement is built for revolve-minted half-caps only",
            ));
        }
        let (p1, p2) = (
            point_of(body, v1)
                .ok_or_else(|| not_intact(EntityId::Vertex(v1), "a meridian split vertex"))?,
            point_of(body, v2)
                .ok_or_else(|| not_intact(EntityId::Vertex(v2), "a meridian split vertex"))?,
        );
        let created = body
            .mef(
                MefSite::Chords { he1, he2 },
                EdgeCurveSpec::line_between(p1, p2),
                FaceSurface::Inherit,
                tol,
            )
            .map_err(|e| op("rim sphere trim mef", e))?;
        let (curve, t0, t1) = scaled(&rc, cb, sb, !rc.plus_on_host);
        described.push((created.edge, ContactCarrier::Exact(curve, t0, t1)));
        rec.rim_trims.push((created.edge, e, RimSide::Sphere));
        tb_edges.push(created.edge);
    }

    // ---- (5) Excise: kill each rim edge across its two strips. ----
    for l in rim.chain.links() {
        let half = host_side_half(body, l, rim.host)
            .ok_or_else(|| not_intact(EntityId::Edge(l.edge), "a rim edge's plane-side half"))?;
        body.kef(half).map_err(|e| op("rim kef", e))?;
        rec.dead.edges.push(l.edge);
    }

    // ---- (6) Fuse the pieces around the ring: kef every plane strut
    // that still separates two faces, retiring each vertex's upper
    // meridian remnant (now a spur) with the vertex by kev. At the
    // CLOSURE vertex — where the strut's two halves share one loop —
    // the band is an annulus, and a curved face must be RING-FREE
    // (`props`' inventory; the donut's own representation): so the
    // strut dies by a fan-merging kev that re-anchors the remnant to
    // the trim foot, leaving the remnant as the band's SLIT — a
    // double-traversed torus meridian, exactly the donut's shape. Its
    // carrier is re-described as that meridian arc in the final pass
    // (the kev leaves it spanning foot → split point with a stale
    // sphere-meridian carrier; nothing validates between here and
    // there). ----
    let remnant_at = |v: VertexKey| -> Option<(EdgeKey, EdgeKey)> {
        remnants
            .iter()
            .find(|(vv, _, _)| *vv == v)
            .map(|(_, e, m)| (*e, *m))
    };
    let torus = &rim.chain.first().blend.surface;
    let Surface::Torus {
        center: tc,
        axis: taxis,
        major_radius: tmaj,
        minor_radius: tmin,
        ..
    } = *torus
    else {
        return Err(unbuilt_geometry(
            EntityId::Edge(rim.chain.first().edge),
            "a rim blend's surface is not a torus",
        ));
    };
    // The band's chart is SEAMED at the slit (certification demands a
    // Seam edge lie in the surface's own u_ref half-plane); the u_ref
    // is conventional data (D2), fixed when the closure vertex is
    // reached below.
    let mut band_surface: Option<Surface<T>> = None;
    for (idx, (v, sp)) in struts_p.iter().enumerate() {
        // Both are proven by this phase's own bookkeeping: every plane
        // strut was minted by step (3)'s `mev` and is killed at most
        // once, here; and `remnants` and `struts_p` are both filled by
        // one pass over `plane_walk`, so every strut's vertex has a
        // remnant row.
        let Some((hp, hm)) = halves_of(body, *sp) else {
            unreachable!(
                "rim fusion: a plane strut was minted by this phase's strut `mev` and has \
                 not been killed"
            )
        };
        let (fa, fb) = (face_of_half(body, hp), face_of_half(body, hm));
        let Some((mr, msrc)) = remnant_at(*v) else {
            unreachable!(
                "rim fusion: `remnants` and `struts_p` are both one row per `plane_walk` \
                 position, keyed by the same rim vertex"
            )
        };
        if fa.is_some() && fa == fb {
            // The closure vertex. Kill the strut from its FOOT side:
            // the rim vertex dies, and its remaining edge — the upper
            // meridian remnant — fan-merges onto the foot, becoming
            // the slit.
            let dying = if body.half_edge_end(hm) == Some(*v) {
                hm
            } else {
                hp
            };
            body.kev(dying).map_err(|e| op("rim closure kev", e))?;
            // The slit's true carrier: the torus minor circle at this
            // vertex's azimuth (radial read off the foot, which lies
            // on the trim circle).
            let fp = strut_hes[idx].1;
            let radial = (fp - ca) / sa;
            // The slit SURVIVES as the band's own double-traversed
            // meridian: a birth row, not a death.
            rec.slits.push((mr, msrc));
            described.push((
                mr,
                ContactCarrier::SeamArc {
                    center: tc + radial * tmaj,
                    radius: tmin,
                },
            ));
            band_surface = Some(Surface::Torus {
                center: tc,
                axis: taxis,
                major_radius: tmaj,
                minor_radius: tmin,
                u_ref: radial,
            });
        } else {
            body.kef(hp).map_err(|e| op("rim strut kef", e))?;
            // The upper meridian remnant at this vertex is now a spur
            // ending at the old rim vertex.
            let (shp, shm) = halves_of(body, mr).ok_or_else(|| {
                not_intact(EntityId::Edge(mr), "a rim vertex's upper meridian remnant")
            })?;
            let dying = if body.half_edge_end(shm) == Some(*v) {
                shm
            } else {
                shp
            };
            body.kev(dying).map_err(|e| op("rim kev", e))?;
            rec.dead.edges.push(mr);
        }
    }

    // The band: the face on the non-cap side of the first sphere trim.
    let Some(tb) = tb_edges.first() else {
        unreachable!(
            "rim phase: step (4) mints one sphere trim per `plane_walk` position, and a \
             cycle always carries at least its anchor half-edge"
        )
    };
    let Some((hp, hm)) = halves_of(body, *tb) else {
        unreachable!(
            "rim phase: the sphere trim was minted by step (4) and nothing between \
                      here and there kills it"
        )
    };
    let band_face = match (face_of_half(body, hp), face_of_half(body, hm)) {
        (Some(f1), Some(f2)) => {
            if rim.mates.contains(&f1) {
                f2
            } else {
                f1
            }
        }
        _ => unreachable!(
            "rim phase: both halves of a sphere trim this phase minted bound a face; \
             `mef` mints the trim into two loops and step (6) kills neither"
        ),
    };
    let Some(band_surface) = band_surface else {
        unreachable!(
            "rim phase: the ring step (6) walks is closed, so exactly one strut reaches \
             the closure case that sets the band's seamed chart"
        )
    };
    let mut chain_named: Vec<EdgeKey> = chain_edges.clone();
    chain_named.sort_unstable();
    rec.bands.push((band_face, chain_named));
    Ok((band_face, band_surface, described))
}

/// **The annulus band** — the one-link closed rim's surgery.
///
/// The band between two revolution walls is one more wall: two closed
/// boundary circles (the trim circles on each support) joined by a
/// double-traversed SLIT at the rim's own azimuth, because a curved face
/// must be ring-free (`props`' closed-form inventory; the donut's own
/// representation). There is no strut-and-`kef` ladder to walk — one
/// closed edge has no consecutive pair — so the whole band is minted by
/// six moves at the seam azimuth:
///
/// 1. `split_edge` the MATE wall's seam where the mate trimline
///    crosses it — the rim-side piece becomes the slit;
/// 2. `split_edge` the HOST wall's seam where the host trimline
///    crosses it — this is the annulus's substitute for the ladder's
///    strut `mev`: the foot already lies on existing geometry;
/// 3. `mef` at the host foot between the seam's two pieces — a closed
///    edge from that vertex to itself, the host trim circle, carving
///    the host's outer strip off the shrunk support;
/// 4. `mef` at the mate foot, likewise, carving the mate's strip;
/// 5. `kef` the rim edge, merging the two strips into one face;
/// 6. `kev` the host seam's rim-side piece from the FOOT side — the rim
///    vertex dies and the mate seam's rim-side piece fan-merges onto
///    the foot, becoming the slit, exactly as the ladder's closure `kev`
///    does at its closure vertex.
///
/// Both supports keep their `FaceKey`, their surface, their sense bit and
/// their rings; the surviving strip is the band. The trim circles' and
/// the slit's carriers are attached in the caller's description pass,
/// once the band's torus exists.
fn rim_phase_annulus<T: Decide + Bounds>(
    body: &mut Body<T>,
    rim: &RimPlan<'_, T>,
    ann: &AnnulusRim,
    rec: &mut FilletNaming,
    tol: Tol,
) -> Result<(FaceKey, Surface<T>, Described<T>), FilletError> {
    let mut described: Described<T> = Vec::new();
    let l0 = rim.chain.first();
    let mate = rim.mates[0];
    // Selected by support FACE, never by slot (`rim_trim_circles` docs).
    let ((ca, sa), (cb, sb)) = rim_trim_circles(l0.edge, &l0.blend, l0.face_a == rim.host)?;
    let rc = rim_carrier(body, l0, rim.host)?;
    let Surface::Torus {
        center: tc,
        axis: taxis,
        major_radius: tmaj,
        minor_radius: tmin,
        ..
    } = l0.blend.surface
    else {
        return Err(unbuilt_geometry(
            EntityId::Edge(l0.edge),
            "a rim blend's surface is not a torus",
        ));
    };

    // The two feet: each trim circle evaluated at the RIM VERTEX's own
    // parameter on the rim's own frame, so both azimuths are inherited
    // rather than reconstructed. The window spans a full turn here, so
    // the two orientations agree on this point by construction.
    let (host_curve, pt0, pt1) = scaled(&rc, ca, sa, rc.plus_on_host);
    let (mate_curve, st0, st1) = scaled(&rc, cb, sb, !rc.plus_on_host);
    let host_foot = host_curve.eval(pt0);
    let mate_foot = {
        let (curve, t0, _) = scaled(&rc, cb, sb, rc.plus_on_host);
        curve.eval(t0)
    };

    // ---- (1)+(2) The two seam splits. Each mints one foot vertex on
    // EXISTING geometry; the piece still touching the rim vertex is the
    // rim-side one. ----
    let split = |body: &mut Body<T>,
                 seam: EdgeKey,
                 target: Point3<T>,
                 site: &'static str|
     -> Result<(VertexKey, EdgeKey, EdgeKey), FilletError> {
        let t = seam_split_param(body, seam, l0.edge, target)?;
        let created = body.split_edge(seam, t, tol).map_err(|e| op(site, e))?;
        let (rim_side, far_side) = if edge_touches(body, seam, ann.vertex) {
            (seam, created.new_edge)
        } else {
            (created.new_edge, seam)
        };
        Ok((created.vertex, rim_side, far_side))
    };
    let (vs, mate_rim_side, mate_far_side) =
        split(body, ann.mate_seam, mate_foot, "annulus mate seam split")?;
    let (vp, host_rim_side, host_far_side) =
        split(body, ann.host_seam, host_foot, "annulus host seam split")?;

    // ---- (3)+(4) The trim circles: one `mef` per support, between the
    // two halves of its split seam that START at the new foot. The run
    // that moves to the NEW face is the rim side, so each support keeps
    // its own key and the strips are the new faces. ----
    let trim_circle = |body: &mut Body<T>,
                       lp: LoopKey,
                       foot: VertexKey,
                       rim_side: EdgeKey,
                       far_side: EdgeKey,
                       site: &'static str|
     -> Result<topo::MefCreated, FilletError> {
        let walk = loop_walk(body, lp)
            .ok_or_else(|| not_intact(EntityId::Loop(lp), "a rim support's boundary cycle"))?;
        let at = |e: EdgeKey| -> Option<HalfEdgeKey> {
            walk.iter()
                .find(|(_, v, k)| *v == foot && *k == e)
                .map(|(h, _, _)| *h)
        };
        let (Some(he1), Some(he2)) = (at(rim_side), at(far_side)) else {
            return Err(not_intact(
                EntityId::Loop(lp),
                "a split seam's two halves at the new foot vertex",
            ));
        };
        let p = point_of(body, foot)
            .ok_or_else(|| not_intact(EntityId::Vertex(foot), "a rim band's foot"))?;
        body.mef(
            MefSite::Chords { he1, he2 },
            EdgeCurveSpec::self_loop_circle_at(p),
            FaceSurface::Inherit,
            tol,
        )
        .map_err(|e| op(site, e))
    };
    let host_loop = loop_of_half(
        body,
        host_side_half(body, l0, rim.host)
            .ok_or_else(|| not_intact(EntityId::Edge(l0.edge), "a rim edge's host-side half"))?,
    )
    .ok_or_else(|| not_intact(EntityId::Edge(l0.edge), "a rim edge's host-side loop"))?;
    let tp = trim_circle(
        body,
        host_loop,
        vp,
        host_rim_side,
        host_far_side,
        "annulus host trim mef",
    )?;
    let mate_half = {
        let ed = body
            .get_edge(l0.edge)
            .ok_or_else(|| not_intact(EntityId::Edge(l0.edge), "a rim edge"))?;
        let ph = host_side_half(body, l0, rim.host)
            .ok_or_else(|| not_intact(EntityId::Edge(l0.edge), "a rim edge's host-side half"))?;
        if ed.he_plus == ph {
            ed.he_minus
        } else {
            ed.he_plus
        }
    };
    let mate_loop = loop_of_half(body, mate_half)
        .ok_or_else(|| not_intact(EntityId::HalfEdge(mate_half), "a rim edge's mate-side loop"))?;
    let ts = trim_circle(
        body,
        mate_loop,
        vs,
        mate_rim_side,
        mate_far_side,
        "annulus mate trim mef",
    )?;

    // ---- (5) Excise: kill the rim edge across its two strips, from
    // the HOST strip's side, so the sphere strip survives as the band
    // (the ladder's excise convention). ----
    let dying_half = host_side_half(body, l0, rim.host)
        .ok_or_else(|| not_intact(EntityId::Edge(l0.edge), "a rim edge's host-side half"))?;
    body.kef(dying_half).map_err(|e| op("annulus rim kef", e))?;

    // ---- (6) The closure: kill the host seam's rim-side piece from
    // its FOOT side. The rim vertex dies and the mate seam's rim-side
    // piece fan-merges onto the host foot, becoming the band's SLIT —
    // a double-traversed torus meridian. Its carrier is re-described
    // below (the `kev` leaves it spanning foot → split point with a
    // stale sphere-seam carrier; nothing validates in between). ----
    let Some((hp, hm)) = halves_of(body, host_rim_side) else {
        unreachable!(
            "annulus band: the host seam's rim-side piece came out of this phase's own \
             `split_edge` and nothing between there and here kills it"
        )
    };
    let dying = if body.half_edge_end(hm) == Some(ann.vertex) {
        hm
    } else {
        hp
    };
    body.kev(dying).map_err(|e| op("annulus closure kev", e))?;

    // ---- The band's chart is SEAMED at the slit (certification demands
    // a Seam edge lie in its surface's own `u_ref` half-plane; the chart
    // reference is conventional data, D2). ----
    let radial = (host_foot - ca) / sa;
    let band_surface = Surface::Torus {
        center: tc,
        axis: taxis,
        major_radius: tmaj,
        minor_radius: tmin,
        u_ref: radial,
    };
    let Some(band_face) = face_of_half(body, ts.he_minus) else {
        unreachable!(
            "annulus band: the sphere trim's minus half bounds the strip `mef` minted for \
             it, and the `kef` above killed the HOST strip"
        )
    };
    if band_face == mate || band_face == rim.host {
        return Err(not_intact(
            EntityId::Face(band_face),
            "the annulus band merged back into one of its own supports",
        ));
    }

    described.push((tp.edge, ContactCarrier::Exact(host_curve, pt0, pt1)));
    described.push((ts.edge, ContactCarrier::Exact(mate_curve, st0, st1)));
    described.push((
        mate_rim_side,
        ContactCarrier::SeamArc {
            center: tc + radial * tmaj,
            radius: tmin,
        },
    ));

    // Birth data. The plane foot is the band's foot on the planar
    // support; the mate foot is a split of that support's seam; the
    // slit SURVIVES as the band's own meridian, so it is a birth row and
    // not a death.
    rec.rim_feet.push((vp, ann.vertex));
    rec.meridian_splits.push((vs, ann.mate_seam));
    rec.meridian_remnants.push((mate_far_side, ann.mate_seam));
    rec.meridian_remnants.push((host_far_side, ann.host_seam));
    rec.rim_trims.push((tp.edge, l0.edge, RimSide::Plane));
    rec.rim_trims.push((ts.edge, l0.edge, RimSide::Sphere));
    rec.slits.push((mate_rim_side, ann.mate_seam));
    rec.dead.edges.push(l0.edge);
    // Only a SOURCE key can be retired: when the split handed the
    // rim-side piece the new edge, the source seam survives as the far
    // piece and nothing of it died.
    if host_rim_side == ann.host_seam {
        rec.dead.edges.push(ann.host_seam);
    }
    rec.dead.vertices.push(ann.vertex);
    rec.bands.push((band_face, vec![l0.edge]));
    Ok((band_face, band_surface, described))
}

/// Whether `edge` has `v` as one of its endpoints.
fn edge_touches<T: Decide>(body: &Body<T>, edge: EdgeKey, v: VertexKey) -> bool {
    halves_of(body, edge).is_some_and(|(hp, hm)| {
        body.get_half_edge(hp).map(|h| h.start) == Some(v)
            || body.get_half_edge(hm).map(|h| h.start) == Some(v)
    })
}

// ------------------------------------------------------------------
// Shared small lookups and the description pass.
// ------------------------------------------------------------------

fn point_of<T: Decide>(body: &Body<T>, v: VertexKey) -> Option<Point3<T>> {
    body.get_point(body.get_vertex(v)?.point).copied()
}

fn halves_of<T: Decide>(body: &Body<T>, e: EdgeKey) -> Option<(HalfEdgeKey, HalfEdgeKey)> {
    let ed = body.get_edge(e)?;
    Some((ed.he_plus, ed.he_minus))
}

fn loop_of_half<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Option<LoopKey> {
    Some(body.get_half_edge(he)?.parent_loop)
}

fn face_of_half<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Option<FaceKey> {
    Some(body.get_loop(loop_of_half(body, he)?)?.face)
}

/// A face's outer cycle as `(half-edge, start vertex, edge)` rows.
fn loop_walk_face<T: Decide>(
    body: &Body<T>,
    f: FaceKey,
) -> Option<Vec<(HalfEdgeKey, VertexKey, EdgeKey)>> {
    loop_walk(body, body.get_face(f)?.outer)
}

/// The prefer-intrinsic upgrade for one new edge: rebuild the exact
/// carrier and describe it as the tangential contact locus of its two
/// adjacent faces' surfaces — over the rim arcs' stored carriers as
/// well as over the straight trimlines.
///
/// **A blend trimline is BORN with its intrinsic description**, never a
/// `MappedCurve` pushforward of the construction that happened to
/// produce it: the rolling ball supplies the witness and the initial
/// caches, and nothing else of the construction survives into the
/// geometry. That is what makes an imported fillet's trimline a
/// reconstruction into a variant this kernel already stores and
/// certifies, rather than a taxonomy scramble at adoption time
/// (`CURVED-DESIGN.md` §D7, fifth leave-room obligation; the rule
/// itself is `DESIGN.md`'s prefer-intrinsic paragraph under D2).
fn attach_contact<T: Decide + Bounds>(
    body: &mut Body<T>,
    edge: EdgeKey,
    carrier: ContactCarrier<T>,
    tol: Tol,
) -> Result<(), FilletError> {
    let ed = body
        .get_edge(edge)
        .ok_or_else(|| not_intact(EntityId::Edge(edge), "an edge awaiting its description"))?;
    let (he_plus, he_minus) = (ed.he_plus, ed.he_minus);
    let (Some(s1), Some(s2)) = (
        face_of_half(body, he_plus).and_then(|f| body.get_face(f).map(|fd| fd.surface)),
        face_of_half(body, he_minus).and_then(|f| body.get_face(f).map(|fd| fd.surface)),
    ) else {
        return Err(not_intact(
            EntityId::Edge(edge),
            "the two faces a described edge separates, or their surfaces",
        ));
    };
    let (p0, p1) = {
        let start = body
            .get_half_edge(he_plus)
            .map(|h| h.start)
            .and_then(|v| point_of(body, v));
        let end = body.half_edge_end(he_plus).and_then(|v| point_of(body, v));
        match (start, end) {
            (Some(a), Some(b)) => (a, b),
            _ => {
                return Err(not_intact(
                    EntityId::HalfEdge(he_plus),
                    "a described edge's endpoints",
                ));
            }
        }
    };
    let is_seam = matches!(carrier, ContactCarrier::SeamArc { .. });
    let transverse = matches!(carrier, ContactCarrier::Chord);
    let (curve, t0, t1) = match carrier {
        ContactCarrier::TrimLine | ContactCarrier::Chord => {
            let len = p0.distance(p1);
            (
                Curve3::Line {
                    origin: p0,
                    dir: (p1 - p0) / len,
                },
                T::zero(),
                len,
            )
        }
        // ONE construction for both arc kinds, deliberately: a corner
        // arc and a band's slit are the same short arc about a stored
        // centre (sweep < π, so the `atan2` turn is unambiguous), and
        // the only thing that differs is the DESCRIPTION they take
        // below. Two byte-identical copies of the geometry would let
        // one drift from the other with nothing to say so.
        ContactCarrier::CornerArc { center, radius }
        | ContactCarrier::SeamArc { center, radius } => {
            let u = (p0 - center).normalize();
            let w = (p1 - center).normalize();
            let turn = u.cross(w);
            (
                Curve3::Circle {
                    center,
                    axis: turn.normalize(),
                    radius,
                    u_ref: u,
                },
                T::zero(),
                turn.norm().atan2(u.dot(w)),
            )
        }
        ContactCarrier::Exact(curve, t0, t1) => (curve, t0, t1),
    };
    let description = if is_seam {
        if s1 != s2 {
            return Err(not_intact(
                EntityId::Edge(edge),
                "a slit edge's two sides are not one face's surface, so the band did not \
                 close as an annulus",
            ));
        }
        EdgeGeometry::Seam { surface: s1 }
    } else if transverse {
        // The chamfer's edges: two surfaces crossing at a definite
        // angle, so the intrinsic description is the plain
        // intersection locus. Calling it a TANGENT intersection would
        // claim normal-parallelism along the locus that the geometry
        // does not have, and certification measures exactly that.
        let witness = curve.eval((t0 + t1) * T::from_f64(0.5));
        EdgeGeometry::Intersection { s1, s2, witness }
    } else {
        let witness = curve.eval((t0 + t1) * T::from_f64(0.5));
        EdgeGeometry::TangentIntersection { s1, s2, witness }
    };
    body.set_edge_curve(
        edge,
        EdgeCurveSpec {
            description,
            carrier: curve,
            param_start: t0,
            param_end: t1,
        },
        tol,
    )
    // No split by refusal kind here any more: `EulerOpError` carries
    // its own `Certification` arm, so the attachment gate's
    // certification failure reaches the caller typed, inside the
    // operator refusal that raised it.
    .map_err(|e| op("surgery contact edge", e))?;
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;
    use geom_core::{Point3, Vec3};

    use topo::EdgeKey;

    use super::super::battery::{Chain, ChainClosure, Convexity, Link};
    use super::super::build::fillet_edges;
    use super::{BlendKind, ConvexOpen, CornerLinks, FilletError, corner_plan, rim_trim_circles};
    use crate::fillet::blend::plane_sphere_blend;
    use crate::test_support::{L, R, all_links, cube};

    /// **The guard for the two cheapest row-4 proofs.**
    /// `blend_surgery`'s `unreachable!`s at the solid and shell reads
    /// both say *"checked at entry"* — and the check is ninety lines
    /// above them. Delete it and those two sentences become lies
    /// printed inside a panic, on a body that would otherwise be
    /// silently filleted as if its first solid were the only one. This
    /// row is what stands there (#720's precedent: a converted site
    /// owes a row that reddens if the check it rests on is removed).
    ///
    /// **Its reach, stated:** one grafted body trips both clauses of
    /// the gate at once, so this row does not separate them. Splitting
    /// them needs a one-solid, two-shell body — a closed void — and
    /// nothing in the tree builds one today.
    #[test]
    fn the_entry_gate_is_what_makes_the_solid_and_shell_reads_provable() {
        let mut dst = cube(L, Tol::witness());
        topo::instance::graft_disjoint_all(
            &mut dst,
            &cube(L * 0.5, Tol::witness()),
            Tol::witness(),
        )
        .expect("the public transplant door accepts a disjoint cube");
        assert_eq!(dst.solids().count(), 2, "the graft made a second solid");
        assert_eq!(dst.shells().count(), 2, "and a second shell");
        let edges: Vec<topo::EdgeKey> = dst.edges().map(|(k, _)| k).collect();
        let tol = geom_core::Tol::witness().get();
        let band = geom_core::Band::new(tol.eps, tol.k * tol.eps).expect("a band");
        let err = fillet_edges(&dst, &edges, R, band, Tol::witness())
            .expect_err("a two-solid body is outside the in-place surgery's door");
        assert!(
            matches!(
                err,
                FilletError::UnsupportedBody {
                    solids: 2,
                    shells: 2
                }
            ),
            "the gate must refuse before anything reads `solids().next()`: {err}"
        );
    }

    /// The F1 pin: trim selection is by SUPPORT KIND, never by slot.
    /// `classify_arm`'s `(Sphere, Plane)` arm swaps `trim_a`/`trim_b`
    /// so the face↔trim pairing holds — a slot-blind read of `trim_a`
    /// takes the SPHERE circle whenever `he_plus` lies on the cap.
    /// Both slot orders must read back the SAME (plane, sphere)
    /// circles bit-for-bit, and the plane circle is the strictly
    /// wider one (`s` vs `R·s/(R + r)`, the blend's own construction).
    #[test]
    fn trim_selection_is_by_support_kind() {
        // The die's own pip numbers: plane z = 0, pip ball centred
        // 0.05 below it, R = 0.09, blend r = 0.02.
        let blend = plane_sphere_blend(
            Point3::new(0.0_f64, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Point3::new(0.3, -0.2, -0.05),
            0.09,
            0.02,
            false,
        );
        let mut swapped = blend.clone();
        core::mem::swap(&mut swapped.trim_a, &mut swapped.trim_b);
        // The edge only names the site of a refusal; this pin takes
        // the Ok arm, so a null key is the honest placeholder.
        let e = EdgeKey::default();
        let a = rim_trim_circles(e, &blend, true).expect("circles");
        let b = rim_trim_circles(e, &swapped, false).expect("circles");
        for ((pa, ra), (pb, rb)) in [(a.0, b.0), (a.1, b.1)] {
            assert_eq!(pa.x, pb.x);
            assert_eq!(pa.y, pb.y);
            assert_eq!(pa.z, pb.z);
            assert_eq!(ra, rb);
        }
        assert!(
            a.0.1 > a.1.1,
            "the plane trim is the widened (outer) circle: {} vs {}",
            a.0.1,
            a.1.1
        );
        // And the blind read on the swapped blend is exactly the bug
        // the selection retires: it would hand back the sphere trim.
        let blind = rim_trim_circles(e, &swapped, true).expect("circles");
        assert_eq!(blind.0.1, a.1.1, "slot-blind trim_a IS the sphere trim");
    }

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

    /// **The surgery corner takes its links' orientation bit too**, and
    /// the concave case never reaches the derivation at all: the door
    /// refuses it.
    ///
    /// **The concave half of this probe is not a body.** The fixture
    /// FALSIFIES the battery's stored verdict on a cube whose geometry
    /// is untouched — a lie about a convex body, not a concave one.
    /// What it pins is where that lie is caught: at
    /// [`ConvexOpen::admit`], so `corner_plan` below cannot be handed
    /// one and has no convexity refusal left to make.
    #[test]
    fn a_corner_plan_takes_its_links_convexity() {
        let body = cube(L, Tol::witness());
        let links = all_links(&body, Tol::witness());
        let v = links[0].start;
        let chains: Vec<Chain<f64>> = links.iter().cloned().map(open_chain).collect();
        let admitted: Vec<ConvexOpen<'_, f64>> = chains
            .iter()
            .map(|c| ConvexOpen::admit(c).expect("a cube's links are convex plane–plane"))
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
        let convex =
            corner_plan(&body, corner_links, R, BlendKind::Fillet).expect("the corner plans");
        assert!(convex.convexity.blend_sense(), "a convex octant is outward");

        let mut concave = links[0].clone();
        concave.convexity = Convexity::Concave;
        let chain = open_chain(concave);
        let Err(err) = ConvexOpen::admit(&chain) else {
            panic!("a concave chain must be refused at the door, not planned")
        };
        assert!(
            matches!(err, FilletError::UnsupportedChain { .. }),
            "expected the open-chain door's typed refusal, got {err}"
        );
    }
}
