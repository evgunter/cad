//! **The in-place edge-blend composition surgery** (M6 unit 1): blend
//! a SUBSET of a body's edges by operating on the body itself — split
//! the support faces along the stored trimlines, excise the edge
//! strips, and graft the blend walls in — instead of rebuilding a
//! whole polyhedron from scratch.
//!
//! **Both verbs run this one surgery.** The verdict's
//! [`BlendKind`] is the single discriminant, read exactly where the
//! two differ: the corner geometry (`corner_plan` grafts the
//! fillet's sphere octant or the chamfer's flat patch through the
//! three trimline feet), the band's contact carrier (a fillet's
//! trimline arc, a chamfer's straight chord), and the closed-chain
//! door (the rim phases below are the fillet's — a chamfer has no
//! closed-chain band and refuses there). Everything else in this
//! module — the splits, the excisions, the ring carry-through, the
//! naming rows — is one shared walk. Prose below that speaks of the
//! ball or the octant is describing the fillet's arm of a shared
//! move, not a fillet-only module.
//!
//! This is the unit M5 banked at PR 12 (deviation 1's second door and
//! deviation 2), sized by that review at one reviewed unit, and
//! sequenced at the head of M6 by Ev's #169 ruling. It is what makes
//! the COMPOSED DIE possible: the filleted blank, the 21 pips and the
//! filleted pip rims in ONE body.
//!
//! **Where each carve lives.** This file holds the seam every carve
//! rests on — the door ([`blend_surgery`]), the refusal constructors,
//! the plans and their admission, the ring carry-through check, the
//! description pass ([`attach_contact`]) and the one face-destroying
//! door ([`SourceFaces::kef_minted`]) — and the two CLOSED-rim walks,
//! [`rim_phase`] and [`rim_phase_annulus`]. The two OPEN bands are
//! carved under `open/`: the plane–plane band with its trihedral
//! corners in [`super::open::planar`], the ruled band with its
//! transverse cut-off in [`super::open::ruled`].
//!
//! # What the surgery does, per chain kind
//!
//! **Open chains** are the two open bands, each carved by its own
//! module and narrated there, once: the plane–plane band between
//! trivalent corners in [`super::open::planar`], the ruled band cut off
//! at transverse caps in [`super::open::ruled`]. Both are admitted here
//! ([`AdmittedOpen`]) and carve on the clone this door makes.
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
//! circle on EITHER material side — on a convex rim the band replaces
//! a strip of the flat face, on a concave one it rests on the flat face
//! beyond the rim and adds material; the curved side splits its
//! MERIDIAN seam edges at the trim circle instead of strutting into the
//! cap),
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
//! ONE closed edge — or SEVERAL arcs of one rim meeting at chart-seam
//! vertices, on a pole-touching revolve whose walls are half-bands —
//! with no ladder to walk either way. Both supports are revolution
//! WALLS — an annular full revolve mints each profile segment as one
//! face whose single cycle carries two closed latitude rims and a
//! doubly-traversed seam meridian; a pole-touching one mints two
//! half-band faces per segment, each traversing each seam once — and
//! the band is minted as one more wall of the one-face shape: per
//! crossing, the MATE's seam split plus the HOST's own foot, one
//! closed-edge `mef` per support carving its strip, one rim `kef`
//! merging the strips, and the ladder's own closure `kev` retiring the
//! rim vertex and fan-merging the MATE seam's remnant into the slit.
//! Neither support's KIND enters: the shape is what the six moves need.
//!
//! **The HOST's foot comes one of two ways, and that is the only place
//! the two annulus shapes part.** Where the host side is half-bands too,
//! a seam meets the crossing and splitting it lands the foot on existing
//! geometry — the seam split taking the strut `mev`'s place. Where ONE
//! host face carries every arc in its own outer cycle — what a
//! coplanar-face merge leaves of a pole-touching cap — the crossing is
//! TRIVALENT, there is no host seam, and the foot is minted by the
//! LADDER's strut instead. Everything after it is the same six moves.
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
//! ([`BlendError::RingClearance`]); in-band escalates with the same
//! recourse (two-tolerance, D4 ¶1 addendum). Everything else in this
//! module is structural: cycle walks, key equality, stored senses.
//!
//! # Out of scope, refused typed
//!
//! Multi-link open chains (junction carry-through),
//! partially-requested corners (run-outs), a ruled band ending at an
//! oblique or curved face (the run-out the mid-curve taxonomy
//! reserves; the battery's `fillet3_cap_transverse` refuses it),
//! closed rims that are neither a circle-carried ring of a PLANE
//! against ring-free caps nor a rim between two revolution walls (of
//! one edge, or of several arcs a chart seam split), and a LADDER rim
//! sharing a support with an ANNULUS rim in ONE call (the annulus
//! band consumes structure of the shared face beyond its own rim —
//! [`shared_support_gate`], whose recourse is sequential calls; two
//! annulus rims sharing a support face — wall or full-revolve cap —
//! carve, their later seam keys re-read live by
//! [`refresh_annulus_seams`]) — each
//! refuses through the frontier vocabulary
//! ([`BlendError::UnsupportedChain`],
//! [`BlendError::UnsupportedRunOut`],
//! [`BlendError::UnsupportedGeometry`],
//! [`BlendError::UnsupportedBody`], and
//! [`BlendError::UnsupportedCorner`] for a corner's own
//! configuration), naming itself and carrying the offending entity. The refusals are the honest boundary of the unit,
//! not gates hiding reachable geometry.
//!
//! # The three refusal classes, and what is NOT one
//!
//! A frontier is one thing; an invalid input is another; an impossible
//! state is a third (D2 addendum, rows 2, 1 and 4). This module keeps
//! them apart at every site:
//!
//! - **Row 2**, above: valid input, unbuilt door, carries the
//!   recourse that is true of it.
//! - **Row 1**, [`BlendError::BodyNotIntact`]: a stored reference
//!   that did not resolve, a cycle that did not close, or a verdict
//!   whose keys disagree with the body's own structure. **This is not a kernel
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
//!
//! # What this surgery may destroy
//!
//! A support shrinks; it does not die — enforced, not argued, at
//! [`SourceFaces::kef_minted`], the one face-destroying door of this
//! file and of the two open bands under `open/`.

use geom::Curve3;
use geom::Surface;
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Band, Bounds, Decide, Margin, Point3, Real, Sign, Vec3};
use topo::{
    Body, EdgeKey, EntityId, FaceKey, FaceSurface, HalfEdgeKey, LoopKey, MefSite, MevSite,
    ShellKey, SolidKey, SurfaceKey, VertexKey,
};

use super::admit::{AdmittedOpen, CornerFaces, CornerLinks, RequestedBoundary};
use super::arms::EdgeBlend;
use super::battery::{BatteryVerdict, Chain, ChainClosure, Convexity, Link};
use super::build::{Blended, face_cycle};
use super::naming::{BlendNaming, RimSide, second_support_is_host};
use super::open::planar::{BlankPlan, Corner, blank_phase, corner_plan};
use super::open::ruled::{RuledPlan, ruled_phase};
use super::{BlendError, BlendKind, BlendSite, CornerConfig, decide};
use geom_core::Tol;

// ------------------------------------------------------------------
// The three refusal classes this module can produce (D2 addendum).
// One named constructor each, so a site's class is greppable at the
// site and no site can inherit a class from a closure's name.
// ------------------------------------------------------------------

/// **Row 1** — the body or the verdict handed to the surgery does not
/// hold together where the plan read it: a stored reference that did
/// not resolve, a cycle that did not close, or a verdict whose keys
/// disagree with the body's own structure. Invalid input, not a
/// frontier. The third clause is the one the plan's own tokens use —
/// a link offered to a corner it does not touch, a chart offered
/// another vertex's faces — and it is why this constructor is not
/// only about arena reads.
pub(super) fn not_intact(at: EntityId, detail: &'static str) -> BlendError {
    BlendError::BodyNotIntact { at, detail }
}

/// **Row 4, announced rather than panicked** — a step of this carve
/// reached a state its own earlier steps rule out. The input was fine;
/// the surgery contradicted itself. Row 4's other sites panic because
/// they are inside a walk with nothing to return; a DOOR returns
/// `Result` already and is on the caller's path, so it announces.
fn invariant_broken(at: EntityId, detail: &'static str) -> BlendError {
    BlendError::SurgeryInvariant { at, detail }
}

/// **Row 2** — the chain's own shape is outside the built door.
pub(super) fn unbuilt_chain(edge: EdgeKey, detail: &'static str) -> BlendError {
    BlendError::UnsupportedChain { edge, detail }
}

/// **Row 2** — the corner's own CONFIGURATION is not one the running
/// band fills (the OQ6 vocabulary, shared with the battery's
/// classifier).
///
/// The one mint of this refusal, so the policy it advertises is always
/// the tag's own ([`CornerConfig::policy`]) and can never be a second,
/// drifting opinion about the same configuration.
pub(super) fn unbuilt_corner_config(vertex: VertexKey, corner: CornerConfig) -> BlendError {
    BlendError::UnsupportedCorner {
        vertex,
        corner,
        policy: corner.policy(),
    }
}

/// **Row 2** — the REQUEST does not cover a termination the corner
/// assembly needs. A run-out, not a corner configuration.
pub(super) fn unbuilt_run_out(at: EntityId, detail: &'static str) -> BlendError {
    BlendError::UnsupportedRunOut { at, detail }
}

/// The one sentence for "a support of a corner is not a plane". Four
/// branches in two modules observe exactly this fact; sharing the
/// string is what stops them wording it four ways.
pub(super) const CORNER_SUPPORT_NOT_PLANAR: &str =
    "a corner support face is not a plane; the corner patch is built over three planes only";

/// **Row 2** — a stored carrier, trimline or surface is not a shape
/// the surgery's closed forms cover.
pub(super) fn unbuilt_geometry(at: EntityId, detail: &'static str) -> BlendError {
    BlendError::UnsupportedGeometry { at, detail }
}

/// **An Euler operator refused during assembly**, kept whole and
/// tagged with the surgery step that ran it.
///
/// A plain function, not a closure factory: the step name is an
/// argument at every call rather than a value captured once per phase,
/// so `BlendError::Op` cannot be constructed here without naming its
/// site, and the operator's own typed refusal — `StaleKey`,
/// `Certification`, the whole vocabulary — travels intact.
pub(super) fn op(site: &'static str, source: topo::EulerOpError) -> BlendError {
    BlendError::Op { site, source }
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

/// One closed chain resolved onto its supports.
///
/// Both sides are per-arc. On a LADDER rim the host side repeats ONE
/// planar face while a revolve-minted cap arrives as half-cap faces
/// split by meridian seam edges through the pole, so each rim arc
/// bounds its own mate face and consecutive arcs meet at a rim vertex
/// where exactly one MERIDIAN edge descends into the cap. On an
/// ANNULUS rim the MATE side is always several FACES of one SURFACE —
/// the half-band walls a chart seam left — while the HOST side is
/// either the same, or ONE face carrying every arc in its own outer
/// cycle, which is what a coplanar-face merge leaves and which
/// [`HostFoot`] is the per-crossing consequence of. [`RimShape`]
/// carries what is true of each shape beyond that.
struct RimPlan<'a, T: Real> {
    chain: &'a Chain<T>,
    /// The HOST support of each link, per chain-link order — the side
    /// whose strip is merged away and whose seam's rim-side piece dies
    /// at the closure. It is the PLANAR support whenever the rim has
    /// one (a ladder rim always does, and so does a plane–sphere
    /// annulus); a rim between two curved walls has no planar side and
    /// takes the link's own `face_a`, which is a slot and not a kind.
    hosts: Vec<FaceKey>,
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
    /// **The annulus.** The band has two closed boundary circles and no
    /// ladder to walk. Both supports are revolution WALLS of ANY
    /// analytic kind — a full revolve mints each profile segment as a
    /// wall whose latitude rims are closed, and that shape is what the
    /// carve needs, not a plane and a sphere. The band is minted as one
    /// more wall of the same shape.
    ///
    /// A pole-touching profile's revolve splits every wall into
    /// half-bands, so the rim arrives as SEVERAL arcs meeting at
    /// chart-seam vertices. The band is still one annulus: the surface
    /// is smooth through such a vertex (the seam is a chart artifact,
    /// and the two extra edges there are co-surface meridians whose
    /// dihedral is zero by construction), so the carve walks THROUGH
    /// it. [`AnnulusRim::crossings`] is one entry per arc, and a
    /// one-edge rim is the one-entry case of it.
    ///
    /// **The host side may be ONE face carrying every arc**, which is
    /// what a coplanar-face merge leaves of a pole-touching cap: the
    /// rim is then that face's whole OUTER cycle rather than several
    /// half-bands, its crossings are TRIVALENT, and each host foot is a
    /// strut instead of a seam split ([`HostFoot`]). Nothing else about
    /// the walk changes — the mate splits, both sides' trimlines, the
    /// excise, the crossing merges and the closure slit are the same
    /// six moves, which is why this is one shape and not a third.
    Annulus(AnnulusRim),
}

/// A closed rim resolved onto the two SURFACES it separates.
struct AnnulusRim {
    /// One crossing per rim arc — the vertices the rim's arcs meet at,
    /// each with the mate's seam meridian and the host's own foot
    /// source ([`HostFoot`]). A one-edge rim has exactly one: its own
    /// vertex, where both walls' doubly-traversed seams meet it.
    crossings: Vec<SeamCrossing>,
    /// Which crossing carries the band's SLIT, and therefore the
    /// azimuth of the band chart's own seam. Every other crossing is
    /// walked through.
    closure: usize,
}

/// **Where a crossing's HOST foot comes from.** The two answers are
/// different moves, not two settings of one, and this is a VARIANT
/// rather than an `Option<EdgeKey>` so the phase matches on what it has
/// and no site can read a seam that is not there.
#[derive(Clone, Copy)]
enum HostFoot {
    /// The host is several half-band faces of one surface, so a seam
    /// meridian meets the crossing and splitting it lands the foot on
    /// EXISTING geometry. Its rim-side piece dies with the vertex.
    Seam(EdgeKey),
    /// ONE host face carries every arc in its own outer cycle, so the
    /// crossing is TRIVALENT — two rim arcs and the mate's seam — and no
    /// host seam exists to split. The foot is minted by the LADDER's own
    /// move, a strut `mev` out to the host trimline at this vertex's own
    /// parameter ([`strut_foot`]), and the strut dies at the crossing
    /// exactly as the ladder's do.
    Strut,
}

/// **What the HOST side of a closed rim carries**, decided once for the
/// whole rim by [`resolve_rim`]'s routing and handed to the resolution
/// so the gate there reads a decision rather than re-taking one.
///
/// It is the per-RIM mode; [`HostFoot`] is the per-CROSSING datum it
/// resolves to, and the two are separate because only one of them can
/// carry a key. Nothing derives this from the body inside the
/// resolution, because [`resolve_rim`] alone knows WHERE the rim sits in
/// its host's loop structure, which is the routing.
///
/// # Two shapes this door does not serve, both measured
///
/// - **A CURVED single face carrying every arc CAN arise, and refuses.**
///   It is reachable through `topo`'s public `kef` — kill one of a
///   sphere wall's two seam meridians and the remaining face carries
///   both rim arcs — and through no sweep or boolean door. It refuses at
///   the half-band gate on BOTH routes, and never carves:
///   `work/fillet/curved-single-host-rim-refuses-at-the-half-band-gate.md`,
///   rowed by
///   `fillet_h5_r2_probes::a_curved_single_face_carrying_both_arcs_refuses_at_the_half_band_gate_on_both_routes`.
/// - **A RINGED host refuses** even under [`Self::Struts`], on the
///   hostless host gate's first arm:
///   `work/fillet/hostless-rim-on-a-ringed-host-refuses.md`. That is the
///   condition [`super::FILLET3_ASSEMBLY_RECOURSE`]'s closed clause
///   states rather than promising past it.
#[derive(Clone, Copy)]
enum HostSide {
    /// Several half-band faces of one surface, one arc each, dropping a
    /// seam meridian at every crossing.
    Seams,
    /// ONE face carrying every arc in its own outer cycle, so no seam
    /// meets a crossing on this side.
    Struts,
}

/// One vertex a closed rim's arcs meet at, with the mate's seam meridian
/// and where the host's foot comes from.
struct SeamCrossing {
    /// The vertex itself.
    vertex: VertexKey,
    /// What supplies the band's foot on the HOST side here.
    host: HostFoot,
    /// The MATE side's seam meridian at [`SeamCrossing::vertex`]. Split
    /// at the mate trimline, its rim-side piece becomes the band's slit
    /// at the closure crossing and dies at every other.
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

    /// The host support of the chain's FIRST link — the one every
    /// per-rim quantity that is a property of the rim rather than of an
    /// arc is read against.
    fn host0(&self) -> FaceKey {
        let Some(&h) = self.hosts.first() else {
            unreachable!(
                "rim plan: `hosts` carries one face per chain link and a chain always \
                 carries its first link"
            )
        };
        h
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
pub(super) fn blend_surgery<T: Decide + Bounds + geom_brep::PcurveFittedLane>(
    source: &Body<T>,
    verdict: &BatteryVerdict<T>,
    band: Band,
    tol: Tol,
) -> Result<Blended<T>, BlendError> {
    // **The gate and the reads are one step.** The check that says
    // "exactly one solid, exactly one shell" BINDS what it counted, so
    // the mutation phase below adopts the keys the door already saw
    // instead of re-deriving them ninety lines later and finding a
    // state the door had ruled out. There is no "the entry check
    // passed and the read came back empty" left to spell.
    let solids: Vec<SolidKey> = source.solids().map(|(k, _)| k).collect();
    let shells: Vec<ShellKey> = source.shells().map(|(k, _)| k).collect();
    let ([solid], [shell]) = (&solids[..], &shells[..]) else {
        return Err(BlendError::UnsupportedBody {
            solids: solids.len(),
            shells: shells.len(),
        });
    };
    let (solid, shell) = (*solid, *shell);
    let radius = verdict.size;
    let kind = verdict.kind;

    // ---- Classify the verdict's chains (structural only). The open
    // chains go through the door as [`AdmittedOpen`], which IS the
    // three-clause admission below. ----
    let mut opens: Vec<AdmittedOpen<'_, T>> = Vec::new();
    let mut rims: Vec<RimPlan<'_, T>> = Vec::new();
    for chain in &verdict.chains {
        match chain.closure {
            ChainClosure::Open { .. } => opens.push(AdmittedOpen::admit(chain)?),
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
    opens.sort_by_key(AdmittedOpen::edge);
    rims.sort_by_key(|r| r.chain.first().edge);
    shared_support_gate(&rims)?;
    // The two open bands part here: a PLANAR link terminates in corners
    // and carves its supports whole (below); a RULED link terminates in
    // transverse caps and carves in `ruled`. Both are admitted opens.
    let (planar, ruled): (Vec<AdmittedOpen<'_, T>>, Vec<AdmittedOpen<'_, T>>) = opens
        .iter()
        .copied()
        .partition(|o| !o.link().arm.is_ruled());

    // ---- Corners: every planar open-link end must be a
    // fully-requested trivalent vertex. Each end's incidence list is
    // seeded by the link that discovered it, so it is non-empty by
    // shape rather than by a check three functions deep. ----
    let mut ends: Vec<CornerLinks<'_, T>> = Vec::new();
    for o in &planar {
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
        let (seed, others) = links.sorted();
        let mut here: Vec<EdgeKey> = core::iter::once(seed.edge())
            .chain(others.iter().map(AdmittedOpen::edge))
            .collect();
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
    for o in &planar {
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
        supports.push(RequestedBoundary::admit(source, f, &planar, &corner_rows)?);
    }

    // ---- The ruled bands' caps, read off the source before anything
    // is carved: each end's cap face, the two rim edges it shares with
    // the supports, and the feet the trimlines put on them. ----
    let mut ruled_plans: Vec<RuledPlan<'_, T>> = Vec::with_capacity(ruled.len());
    for o in &ruled {
        ruled_plans.push(RuledPlan::plan(
            source,
            *o,
            &opens,
            &verdict.transverse_caps,
        )?);
    }

    // ---- The ring carry-through honesty check (the one decision this
    // module adds — module docs). ----
    ring_clearance_pass(source, &opens, &rims, band)?;

    // ---- Mutation, on a clone. From here on every step is an Euler
    // operator or a certified setter; refusals map to Op/Certify. ----
    // **The one snapshot.** Every face the surgery may NOT kill is a
    // face of the body it was handed, so the set is read once, here,
    // before the clone — not re-assembled per phase from whatever that
    // phase happens to hold. It is the door's own argument, which is
    // what makes a narrower set unspellable.
    let sources = SourceFaces::of(source)?;
    let mut body = source.clone();
    let mut rec = BlendNaming::default();
    let blank = BlankPlan {
        opens: &planar,
        corners: &corners,
        supports: &supports,
    };
    let (planar_faces, corner_faces, mut described) =
        blank_phase(&mut body, &blank, &sources, &mut rec, tol, kind)?;
    // One blend face per open link, paired with its link and put back
    // in the opens' own edge order once both bands have carved.
    let mut blend_rows: Vec<(AdmittedOpen<'_, T>, FaceKey)> =
        planar.iter().copied().zip(planar_faces).collect();
    for plan in &ruled_plans {
        let (face, mut arcs) = ruled_phase(&mut body, plan, &sources, &mut rec, tol)?;
        described.append(&mut arcs);
        blend_rows.push((plan.link(), face));
    }
    blend_rows.sort_by_key(|(o, _)| o.edge());
    let blend_faces: Vec<FaceKey> = blend_rows.iter().map(|(_, f)| *f).collect();
    let mut band_faces = Vec::with_capacity(rims.len());
    let mut band_surfaces = Vec::with_capacity(rims.len());
    for (i, rim) in rims.iter().enumerate() {
        let (band_face, band_surface, mut arcs) = match &rim.shape {
            RimShape::Ladder { ring } => rim_phase(&mut body, rim, *ring, &sources, &mut rec, tol)?,
            RimShape::Annulus(ann) => {
                // The #935 refresh: an earlier band's carve on a shared
                // wall split this rim's seam meridians, so their LIVE
                // identity is re-read against the carved body. Rims
                // sharing nothing keep the plan's keys on the plan's
                // own path.
                let live = if rims[..i].iter().any(|p| rims_share_support(p, rim)) {
                    refresh_annulus_seams(&body, rim, ann)?
                } else {
                    ann.crossings
                        .iter()
                        .map(|c| LiveSeams {
                            host: c.host,
                            mate: c.mate_seam,
                        })
                        .collect()
                };
                rim_phase_annulus(&mut body, rim, ann, &live, &sources, &mut rec, tol)?
            }
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
    for (o, fk) in &blend_rows {
        let fk = *fk;
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
    topo::mint_pcurves(&mut body, tol).map_err(|source| BlendError::Certify {
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
    Ok(Blended {
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

/// Resolve one closed chain onto its two supports, with every
/// structural precondition of the band replacement checked.
///
/// **Which gates are per-KIND and which are per-SHAPE.** The band a
/// closed rim replaces is a torus, and that is the only geometric claim
/// this resolution rests on — so the arm gate asks for a torus arm
/// ([`super::arms::BlendArm::is_coaxial_torus`]) and not for one pair of kinds. Below
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
/// **What routes a MULTI-LINK chain between them is the host side, not
/// the link count**, and the host side answers in three ways rather than
/// two. When ONE planar face hosts every link, WHERE the rim sits in
/// that face's loop structure decides:
///
/// - the rim is a RING of it → **LADDER**. The face keeps a boundary of
///   its own outside the rim, the mate side is ring-free half-caps, and
///   the ladder's struts and trim arcs walk the ring.
/// - the rim is that face's own OUTER cycle → **ANNULUS with hostless
///   crossings**. This is what a coplanar-face merge leaves of a
///   pole-touching cap: the host has no seam at a crossing, so the
///   crossing is trivalent and the host foot is a strut
///   ([`HostFoot::Strut`]) rather than a seam split. The mate side is
///   what the annulus already serves.
///
/// When NO single planar face hosts every link — a rim a chart seam has
/// split, whose supports are half-band walls, several FACES of one
/// SURFACE per side — it is the annulus with SEAM crossings, as before.
/// A one-link chain is an annulus by shape (a ring of one face has a
/// link count greater than one whenever it is a ring at all).
///
/// **Neither shape asks which material side the rim is on.** The band
/// is a torus about the rim's own spine whichever side the ball rests
/// on; the walk below is struts or seam splits to the feet, a trimline
/// `mef` per support, a `kef` per arc and the crossing merges, none of
/// which is a material-side fact. A convex rim's carve REMOVES the two
/// support strips between the rim and the trimlines and a concave rim's
/// identical carve ADDS them; the one sense the band carries is its
/// face bit, folded from the chain's stored verdict at the door
/// ([`Convexity::blend_sense`]).
fn resolve_rim<'a, T: Decide + Bounds>(
    body: &Body<T>,
    chain: &'a Chain<T>,
) -> Result<RimPlan<'a, T>, BlendError> {
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
        let host = if second_support_is_host(a_planar, b_planar) {
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
            hosts: vec![host],
            mates: vec![mate],
            shape,
        });
    }

    // ---- The LADDER's discriminant: ONE planar face hosting every
    // link. Anything else is a rim whose supports are half-band walls,
    // which is the annulus. ----
    // One link's (host, mate) split, or `None` where neither support
    // is a plane — the annulus's discriminant, not a refusal.
    let split = |link: &Link<T>| -> Result<Option<(FaceKey, FaceKey)>, BlendError> {
        let a_planar = is_plane(link.face_a)
            .ok_or_else(|| not_intact(EntityId::Face(link.face_a), "a rim link's first support"))?;
        // A support that does not RESOLVE is a broken body, not a
        // frontier: the two answers are different rows and the absent
        // one never borrows the refusal of the unbuilt one.
        let b_planar = is_plane(link.face_b).ok_or_else(|| {
            not_intact(EntityId::Face(link.face_b), "a rim link's second support")
        })?;
        // WHICH support is the host is the shared rule's answer, not a
        // third spelling of it. What stays here is the question the rule
        // cannot answer — whether the rim has a planar support AT ALL —
        // because `None` is this function's routing decision and not a
        // host pick.
        Ok((a_planar || b_planar).then(|| {
            if second_support_is_host(a_planar, b_planar) {
                (link.face_b, link.face_a)
            } else {
                (link.face_a, link.face_b)
            }
        }))
    };
    // The FIRST link fixes the shared plane, and a chain carries its
    // first link in a field rather than in a `Vec` — so the host
    // support is a VALUE here, and "the loop that must have run did
    // not" is a state this function no longer spells.
    let Some((plane, first_mate)) = split(link0)? else {
        return resolve_seam_split_rim(body, chain, HostSide::Seams);
    };
    let mut mates = Vec::with_capacity(chain.link_count());
    mates.push(first_mate);
    for link in chain.rest() {
        let Some((p, s)) = split(link)? else {
            return resolve_seam_split_rim(body, chain, HostSide::Seams);
        };
        if p != plane {
            return resolve_seam_split_rim(body, chain, HostSide::Seams);
        }
        mates.push(s);
    }
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

    // WHERE the rim sits in the one planar host's loop structure is the
    // routing (function docs). A ring is the LADDER; the face's own
    // outer cycle is the annulus with HOSTLESS crossings, whose plan is
    // built at the seam-split resolution's own host gate — the one site
    // that asks what the host side carries.
    let ring = plane_loop;
    let pd = body
        .get_face(plane)
        .ok_or_else(|| not_intact(EntityId::Face(plane), "a rim's plane support"))?;
    if !pd.rings.contains(&ring) {
        if pd.outer == plane_loop {
            return resolve_seam_split_rim(body, chain, HostSide::Struts);
        }
        // A half-edge's parent loop is a loop of the face it bounds, so
        // a loop that is neither the outer cycle nor a ring of that face
        // is a body whose loop set disagrees with its half-edges — Row
        // 1, not a frontier.
        return Err(not_intact(
            EntityId::Loop(plane_loop),
            "a rim's plane-side loop is neither that face's outer cycle nor one of its rings",
        ));
    }

    // The LADDER's remaining gates: each arc's mate face is a ring-free
    // cap piece carrying exactly that one chain arc on its boundary
    // (revolve-minted half-caps).
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
        hosts: vec![plane; chain.link_count()],
        mates,
        shape: RimShape::Ladder { ring },
    })
}

/// Resolve a closed rim of SEVERAL arcs onto the two surfaces it
/// separates — the arcs a chart seam left behind, whose supports are
/// half-band faces, and equally the same rim after a coplanar-face
/// merge has made ONE of those sides a single face.
///
/// The band is still ONE annulus. What makes that true is the geometry
/// of a seam vertex: the rim arrives and leaves on the same two
/// surfaces (the surface is smooth through it), and the extra incident
/// edges are co-surface meridians whose dihedral is zero by
/// construction. So the walk carries THROUGH such a vertex; it never
/// stops at one, and nothing here adds a termination.
///
/// Every precondition is read off the SOURCE body, before any mutation:
///
/// - one support PAIR for the whole rim, and the two surfaces distinct
///   (a rim between two faces of one surface has no two sides to rest
///   on, and is a tangency the battery has already refused);
/// - each MATE face ring-free and carrying exactly ONE of the chain's
///   arcs — the half-band discipline the band replacement needs, the
///   annulus twin of the ladder's half-cap one;
/// - the HOST side per `host_side` ([`HostSide`]): the same half-band
///   discipline under [`HostSide::Seams`], or, under
///   [`HostSide::Struts`], ONE ring-free face whose OUTER CYCLE is
///   exactly the chain's arcs and nothing else — the analogue of the
///   ladder's "a rim ring carries edges outside the requested chain",
///   and what makes the strip carve well-defined when the trim chords
///   run in that cycle;
/// - each arc's two ends met by exactly one other arc, so the arcs walk
///   one cycle in the host side's own traversal;
/// - at every such vertex, incidence is exactly the two arcs plus ONE
///   co-surface seam meridian per side that HAS one — both sides under
///   `Seams`, the mate alone under `Struts`, where the crossing is
///   trivalent because the merge consumed the host's seam.
///
/// **One rule, four readings.** This incidence is also spelled by
/// [`super::battery`]'s `is_seam_vertex` (the refusal classifier, which
/// reads incidence and NOTHING else — no convexity, no support
/// resolution, so it is strictly weaker than this), by
/// [`resolve_annulus`]/[`wall_seam`] (the same shape on a rim of ONE
/// self-closed edge and its doubly-traversed wall seams), and by
/// [`refresh_annulus_seams`] (the CARVE-time re-read of these keys on
/// a body carrying earlier bands — a reader adding a fifth reading
/// starts from this list). They are not
/// shared because each answers a different question at a different
/// phase; the intended relation is that the battery's ADMITS every site
/// these two do and more, which is why the seam tag's recourse has to
/// be true at every site the tag fires — and is, on either material
/// side, since the closed-rim band carves on both.
///
/// # Why this is not merged with the other annulus resolver
///
/// [`resolve_annulus`] (one self-closed edge) and this one (several
/// arcs) resolve the SAME band onto the same two surfaces, and a
/// unified resolver is structurally available: the one-edge case is
/// this one with a single crossing whose two seams are its wall's
/// doubly-traversed ones. It is deliberately NOT taken here, and the
/// cost is stated so it is a decision rather than an oversight:
///
/// - the one-edge path's gates are load-bearing in a way this one's are
///   not — `wall_seam`'s "traversed exactly twice by ONE loop" is the
///   revolution-wall shape itself, and it has no counterpart here,
///   where a seam is traversed once by each of two half-band faces;
/// - its refusal strings are the ones several existing suites read, and
///   merging would either change them or freeze this resolver's
///   wording to theirs.
///
/// **The hazard the decline accepts is DRIFT**: two admissions of one
/// carve can diverge silently, since the phase below serves both from
/// one representation. What bounds it is that the representation is
/// shared — both produce [`AnnulusRim`] crossings, and
/// [`rim_phase_annulus`] has no branch on which resolver made them — so
/// a divergence has to be in what is ADMITTED, not in what is built.
fn resolve_seam_split_rim<'a, T: Decide + Bounds>(
    body: &Body<T>,
    chain: &'a Chain<T>,
    host_side: HostSide,
) -> Result<RimPlan<'a, T>, BlendError> {
    let link0 = chain.first();
    let surface_of = |f: FaceKey| -> Option<SurfaceKey> { Some(body.get_face(f)?.surface) };
    let pair_of = |l: &Link<T>| -> Result<(SurfaceKey, SurfaceKey), BlendError> {
        let a = surface_of(l.face_a)
            .ok_or_else(|| not_intact(EntityId::Face(l.face_a), "a rim link's first support"))?;
        let b = surface_of(l.face_b)
            .ok_or_else(|| not_intact(EntityId::Face(l.face_b), "a rim link's second support"))?;
        Ok((a, b))
    };
    let (ka, kb) = pair_of(link0)?;
    if ka == kb {
        return Err(unbuilt_chain(
            link0.edge,
            "a closed rim's two supports are ONE surface, so the band has no two sides \
             to rest on",
        ));
    }
    for link in chain.links() {
        let (a, b) = pair_of(link)?;
        if (a, b) != (ka, kb) && (b, a) != (ka, kb) {
            return Err(unbuilt_chain(
                link.edge,
                "a closed chain's arcs do not carry ONE support pair; a rim a chart seam \
                 split arrives and leaves on the same two surfaces",
            ));
        }
    }
    // The HOST surface is the planar one when the rim has exactly one,
    // so a plane-and-curved rim keeps the roles a one-edge rim gives
    // it; between two curved walls the roles are the first link's own
    // slots, which is a slot and not a kind.
    let is_plane_surface = |k: SurfaceKey| -> Result<bool, BlendError> {
        let s = body
            .get_surface(k)
            .ok_or_else(|| not_intact(EntityId::Face(link0.face_a), "a rim support's surface"))?;
        Ok(matches!(s, Surface::Plane { .. }))
    };
    let host_surface = if second_support_is_host(is_plane_surface(ka)?, is_plane_surface(kb)?) {
        kb
    } else {
        ka
    };
    let mut hosts = Vec::with_capacity(chain.link_count());
    let mut mates = Vec::with_capacity(chain.link_count());
    for link in chain.links() {
        // NOT the host rule again: that was decided once, above, for
        // the whole chain. This asks which SLOT of this particular
        // link carries the surface already chosen — the links of one
        // seam-split rim do not agree on slot order.
        let a_is_host = surface_of(link.face_a)
            .ok_or_else(|| not_intact(EntityId::Face(link.face_a), "a rim link's first support"))?
            == host_surface;
        let (h, m) = if a_is_host {
            (link.face_a, link.face_b)
        } else {
            (link.face_b, link.face_a)
        };
        hosts.push(h);
        mates.push(m);
    }

    // The MATE face of every arc is a HALF-BAND of its wall: ring-free,
    // and carrying exactly the one arc it hosts. Under `Seams` the host
    // faces answer the same gate; under `Struts` the host is ONE face
    // whose whole outer cycle is the rim, checked below instead.
    let chain_edges: Vec<EdgeKey> = chain.links().map(|l| l.edge).collect();
    let half_band = |f: FaceKey, link: &Link<T>| -> Result<(), BlendError> {
        let fd = body
            .get_face(f)
            .ok_or_else(|| not_intact(EntityId::Face(f), "a rim arc's support"))?;
        if !fd.rings.is_empty() {
            return Err(unbuilt_chain(
                link.edge,
                "a seam-split rim's support carries rings of its own",
            ));
        }
        let carried: Vec<EdgeKey> = face_cycle(body, f)
            .ok_or_else(|| {
                not_intact(
                    EntityId::Face(f),
                    "a rim arc's support has no boundary cycle that walks",
                )
            })?
            .iter()
            .filter_map(|he| body.get_half_edge(*he).map(|h| h.edge))
            .filter(|e| chain_edges.contains(e))
            .collect();
        if carried != [link.edge] {
            return Err(unbuilt_chain(
                link.edge,
                "a seam-split rim's support does not carry exactly its own rim arc \
                 (the half-band discipline the band replacement needs)",
            ));
        }
        Ok(())
    };
    for (i, link) in chain.links().enumerate() {
        half_band(mates[i], link)?;
        match host_side {
            HostSide::Seams => half_band(hosts[i], link)?,
            HostSide::Struts => {}
        }
    }
    // **The HOSTLESS host gate: ONE ring-free face whose outer cycle is
    // exactly the chain's arcs**, and the FRONTIER of this door. Both
    // arms below refuse a body that satisfies "one face carries every
    // arc" and is still not carvable, so
    // [`super::FILLET3_ASSEMBLY_RECOURSE`]'s closed clause states both
    // conditions — a ring-free host, the rim as its whole outer cycle —
    // rather than promising the carve at a body that meets neither.
    // Each arm's sentence is audited against that clause at its own
    // site.
    if let HostSide::Struts = host_side {
        let Some(&host) = hosts.first() else {
            unreachable!(
                "rim plan: `hosts` carries one face per chain link and a chain always \
                 carries its first link"
            )
        };
        // NOT a gate: [`HostSide::Struts`] is only reached from
        // [`resolve_rim`]'s routing, which fixed ONE planar face as the
        // host of every link before choosing it. The invariant is
        // asserted rather than re-decided, because a refusal here would
        // hand out a recourse for a shape no caller can present.
        debug_assert!(
            hosts.iter().all(|&h| h == host),
            "hostless routing admits one host face for the whole rim"
        );
        let fd = body
            .get_face(host)
            .ok_or_else(|| not_intact(EntityId::Face(host), "a rim's host support"))?;
        // **Frontier arm 1 — a RINGED host.** The band's host trim is
        // the face's new outer boundary, and nothing here says where a
        // ring of that face then sits relative to it; the ring-clearance
        // pass answers that for a LADDER rim and is scoped away from
        // this one. Refused rather than carved:
        // `work/fillet/hostless-rim-on-a-ringed-host-refuses.md`, whose
        // instance is the boss's top outer rim. The recourse this hands
        // out is true at the site because its clause asks for a
        // RING-FREE host.
        if !fd.rings.is_empty() {
            return Err(unbuilt_chain(
                link0.edge,
                "a hostless-crossing rim's host face carries rings of its own",
            ));
        }
        let mut cycle: Vec<EdgeKey> = face_cycle(body, host)
            .ok_or_else(|| {
                not_intact(
                    EntityId::Face(host),
                    "a rim's host support has no outer cycle that walks",
                )
            })?
            .iter()
            .filter_map(|he| body.get_half_edge(*he).map(|h| h.edge))
            .collect();
        cycle.sort_unstable();
        cycle.dedup();
        let mut want = chain_edges.clone();
        want.sort_unstable();
        want.dedup();
        // **Frontier arm 2 — an outer cycle wider than the request.**
        // "Exactly" is what the ladder's ring gate asks of its ring and
        // for the same reason: a trim chord runs between consecutive
        // feet IN this cycle, so an edge of it the request did not name
        // would end up inside a strip the carve excises. The recourse is
        // true at the site because its clause asks for the rim to be the
        // host's WHOLE outer cycle. Rowed by
        // `fillet_h5_r2_probes::a_hostless_host_with_an_unrequested_outer_cycle_edge_refuses_at_the_host_gate`.
        if cycle != want {
            return Err(unbuilt_chain(
                link0.edge,
                "a hostless-crossing rim's host face carries edges outside the requested \
                 chain in its outer cycle",
            ));
        }
    }

    // The arcs walk ONE cycle in the host side's traversal: every arc's
    // host-side start is some other arc's host-side end.
    let mut ends = Vec::with_capacity(chain.link_count());
    for (i, link) in chain.links().enumerate() {
        let half = host_side_half(body, link, hosts[i])
            .ok_or_else(|| not_intact(EntityId::Edge(link.edge), "a rim arc's host-side half"))?;
        let (Some(s), Some(e)) = (
            body.get_half_edge(half).map(|h| h.start),
            body.half_edge_end(half),
        ) else {
            return Err(not_intact(
                EntityId::HalfEdge(half),
                "a rim arc's host-side traversal",
            ));
        };
        ends.push((s, e));
    }
    for (i, link) in chain.links().enumerate() {
        let starts = ends.iter().filter(|(s, _)| *s == ends[i].1).count();
        let finishes = ends.iter().filter(|(_, e)| *e == ends[i].1).count();
        if starts != 1 || finishes != 1 {
            return Err(unbuilt_chain(
                link.edge,
                "a seam-split rim's arcs do not walk one cycle on the host side",
            ));
        }
    }

    // One crossing per arc, in chain-link order, keyed by the vertex
    // each arc's host-side traversal ENDS at.
    let mut crossings = Vec::with_capacity(chain.link_count());
    for (i, link) in chain.links().enumerate() {
        let vertex = ends[i].1;
        let mut incident = vertex_edges_of(body, vertex)
            .ok_or_else(|| not_intact(EntityId::Vertex(vertex), "a rim vertex's edge orbit"))?;
        incident.sort_unstable();
        incident.dedup();
        let (arcs, seams): (Vec<EdgeKey>, Vec<EdgeKey>) =
            incident.iter().partition(|e| chain_edges.contains(e));
        // One seam per side that HAS one: both under `Seams`, the mate
        // alone under `Struts`, where the coplanar merge consumed the
        // host's and left the crossing TRIVALENT.
        //
        // **The recourse audit at this arm.** Under `Seams` this is the
        // pre-existing site. Under `Struts` a crossing carrying a HOST
        // seam cannot reach it — the outer-cycle arm above admitted a
        // host whose cycle is exactly the two arcs, so that face meets
        // the crossing exactly twice and has no third half-edge to give
        // — and what does reach it is a crossing with two MATE seams,
        // i.e. three mate faces meeting there. That is neither
        // "each support face carries one arc" nor "one ring-free face
        // carries every arc", so the sentence does not promise it and is
        // true here.
        let want_seams = match host_side {
            HostSide::Seams => 2,
            HostSide::Struts => 1,
        };
        if arcs.len() != 2 || seams.len() != want_seams {
            return Err(unbuilt_chain(
                link.edge,
                "a seam-split rim's vertex carries more than the rim's two arcs and one \
                 seam meridian per side; the annulus band is built for revolution walls \
                 only",
            ));
        }
        let (mut host_seam, mut mate_seam) = (None, None);
        for seam in seams {
            let (fp, fm) = edge_faces(body, seam)
                .ok_or_else(|| not_intact(EntityId::Edge(seam), "a seam meridian's two faces"))?;
            let (sp, sm) = (
                surface_of(fp).ok_or_else(|| {
                    not_intact(EntityId::Face(fp), "a seam meridian's first support")
                })?,
                surface_of(fm).ok_or_else(|| {
                    not_intact(EntityId::Face(fm), "a seam meridian's second support")
                })?,
            );
            // A CO-SURFACE edge is what a chart seam is, and it is what
            // makes the dihedral zero by construction rather than by a
            // sampled normal.
            if sp != sm {
                return Err(unbuilt_chain(
                    link.edge,
                    "an extra edge at a rim vertex is not a co-surface seam meridian, so \
                     the rim is not smooth through it",
                ));
            }
            let slot = if sp == host_surface {
                &mut host_seam
            } else {
                &mut mate_seam
            };
            if slot.replace(seam).is_some() {
                return Err(unbuilt_chain(
                    link.edge,
                    "a rim vertex drops two seam meridians into ONE of its supports",
                ));
            }
        }
        let Some(mate_seam) = mate_seam else {
            return Err(unbuilt_chain(
                link.edge,
                "a rim vertex does not drop one seam meridian into each of its supports",
            ));
        };
        // Under `Struts` the surviving seam is the MATE's by structure,
        // not by a further check: the count gate admitted exactly one
        // extra edge, the co-surface gate makes it a chart seam, and the
        // host gate above put every arc on ONE host face — so the three
        // faces at a trivalent crossing are that host and the arcs' two
        // mates, and the only pair a co-surface edge can separate is the
        // mates. A host-surface seam therefore leaves the mate slot
        // empty and refuses on the line above.
        let host = match (host_side, host_seam) {
            (HostSide::Seams, Some(seam)) => HostFoot::Seam(seam),
            (HostSide::Struts, None) => HostFoot::Strut,
            (HostSide::Seams, None) | (HostSide::Struts, Some(_)) => {
                return Err(unbuilt_chain(
                    link.edge,
                    "a rim vertex does not drop one seam meridian into each of its supports",
                ));
            }
        };
        crossings.push(SeamCrossing {
            vertex,
            host,
            mate_seam,
        });
    }
    Ok(RimPlan {
        chain,
        hosts,
        mates,
        shape: RimShape::Annulus(AnnulusRim {
            crossings,
            closure: 0,
        }),
    })
}

/// Whether two rims of one plan rest on any common support face.
fn rims_share_support<T: Real>(a: &RimPlan<'_, T>, b: &RimPlan<'_, T>) -> bool {
    a.hosts
        .iter()
        .chain(a.mates.iter())
        .any(|s| b.hosts.contains(s) || b.mates.contains(s))
}

/// **What one call may carve onto a shared support face.** Every plan
/// in this module is resolved against the SOURCE body before anything
/// is carved (the decide-first discipline), and the bands are then
/// carved one after another into one clone — so a later band's plan can
/// name an ENTITY an earlier band's carve has since split, even though
/// every decision in it is still right. Per sharing pair:
///
/// - **Two LADDER rims** carve freely: each carve is confined to its
///   own ring and the caps that ring bounds, so two of them never
///   meet (the composed die's 21 pip rims on six planes).
/// - **Two ANNULUS rims** carve too — sharing a revolution WALL or a
///   full-revolve PLANE CAP; a cap is one more wall of the same shape,
///   its radial seam the meridian (the annulus resolution never asked
///   a support's kind, so neither does this). What an annulus carve
///   reaches on a shared face is that face's SEAM MERIDIAN, and the
///   only plan data that names it is the later rim's crossing seam
///   KEYS, whose live identity [`refresh_annulus_seams`] re-reads
///   immediately before that rim's own phase. Identity is all that
///   moves: every decision stays the plan's (#935; the one-call result
///   is pinned equal to the sequential composition and its closed
///   form — wall pairs and cap pairs both, `blend_tworims.rs` and
///   `blend2_r2_probes.rs`).
/// - **A LADDER and an ANNULUS rim** sharing a support are refused:
///   an annulus carve consumes structure of the shared face beyond
///   its own rim (the seam split, the trimline carve of the face's
///   outer cycle), and nothing here proves a ladder plan's ring walk
///   survives that — nor the converse. Refused in BOTH orders, before
///   any mutation, with the sequential recourse (each call plans
///   against its own source, so sequence is always honest).
///
///   **The shape IS authorable now, and this arm is still not what
///   refuses it.** The old premise here — that a plane face carrying
///   both a RING and a revolution-wall cycle has no public
///   construction but a boolean of a ball against a revolve — is
///   false: a revolve whose flat top runs in to a dome, followed by
///   `merge_coplanar_faces`, mints exactly that face, and THIS unit
///   made its outer cycle a rim the annulus routes. So a request for
///   that face's ring rim (a LADDER) and its outer rim together does
///   reach a mixed pair. It does not reach THIS arm: the outer rim is
///   refused earlier, by the hostless host gate's ring arm, because
///   the host carries a ring. The canary is therefore
///   `review_fillet_h5_r1_probes::r1_the_mixed_shared_support_arm_is_not_what_refuses_the_bosss_two_rims`,
///   which measures that the boss's two rims are refused by the host
///   gate and NOT here — and reds the day the ringed host carves
///   (`work/fillet/hostless-rim-on-a-ringed-host-refuses.md`), which is
///   the moment this arm first needs a row of its own.
fn shared_support_gate<T: Real>(rims: &[RimPlan<'_, T>]) -> Result<(), BlendError> {
    for (i, a) in rims.iter().enumerate() {
        for b in rims.iter().skip(i + 1) {
            if !rims_share_support(a, b) {
                continue;
            }
            let annulus = |r: &RimPlan<'_, T>| matches!(r.shape, RimShape::Annulus(_));
            if annulus(a) != annulus(b) {
                return Err(unbuilt_chain(
                    b.chain.first().edge,
                    "a ladder rim and an annulus rim of one request share a support \
                     face, and the annulus band consumes structure of that face beyond \
                     its own rim — blend them in SEQUENTIAL calls (the second on the \
                     first's result); one call is not implemented",
                ));
            }
        }
    }
    Ok(())
}

/// The LIVE identity of one crossing's seam meridians at CARVE time —
/// the mate's always, the host's when it HAS one — as the plan's own
/// keys, unless an earlier band's carve on a shared wall split them, in
/// which case the piece that still meets this rim's crossing vertex
/// carries a different key.
///
/// Identity only, never decision: the feet, the trimlines, the torus
/// and the closure choice all stay the plan's, resolved against the
/// source (the decide-first discipline). What a carve consumes of a
/// shared wall is exactly its seam meridian, so this pair is exactly
/// what can go stale between bands — everything else the phase reads
/// is either plan geometry or read live at its own use site.
struct LiveSeams {
    /// The host's foot source under its CARVE-time key. A
    /// [`HostFoot::Strut`] crossing has no seam to re-read and carries
    /// through unchanged — the strut is minted by the phase, so nothing
    /// an earlier band did can stale it.
    host: HostFoot,
    mate: EdgeKey,
}

/// Re-read a rim's crossing seam keys against the CURRENT body — the
/// #935 refresh, run immediately before the rim's own phase and only
/// when an earlier band's carve shared one of this rim's supports.
///
/// **The fourth reading of the seam-incidence rule** (see
/// [`resolve_seam_split_rim`] for the other three and the intended
/// relation): at a crossing vertex, the incident edges are the rim's
/// own arcs plus ONE co-surface seam meridian per support side THAT HAS
/// ONE. An earlier annulus carve preserves that incidence at every
/// OTHER rim's crossing — it splits the shared seam at its own feet and
/// kills its own vertices only — so the re-read finds the surviving
/// piece per side and nothing else. A body where it finds anything else
/// is a composition this refresh cannot repair, and it refuses BEFORE
/// this rim mutates anything (the clone already carries the earlier
/// bands; the caller's body is untouched either way), naming the
/// sequential recourse, which is always honest: each sequential call
/// resolves its plan against its own source.
///
/// **A HOSTLESS crossing re-reads its mate alone.** Its host foot is a
/// strut this phase mints, which no earlier carve can have staled, and
/// its host face carries the whole rim in its own outer cycle — so no
/// second rim of the same call rests on that face and nothing there
/// moves. What such a rim can share is its MATE wall, which is exactly
/// the side still re-read.
fn refresh_annulus_seams<T: Decide + Bounds>(
    body: &Body<T>,
    rim: &RimPlan<'_, T>,
    ann: &AnnulusRim,
) -> Result<Vec<LiveSeams>, BlendError> {
    let surface_of = |f: FaceKey| -> Option<SurfaceKey> { Some(body.get_face(f)?.surface) };
    let host_surface = surface_of(rim.host0())
        .ok_or_else(|| not_intact(EntityId::Face(rim.host0()), "a rim's host support"))?;
    let Some(&mate0) = rim.mates.first() else {
        unreachable!(
            "rim plan: `mates` carries one face per chain link and a chain always \
             carries its first link"
        )
    };
    // `mates[0]` answers for every mate (and `host0` for every host):
    // the multi-arc resolver admits exactly ONE support pair for the
    // whole rim, and a one-edge rim has a single face per side — so
    // per-side surface homogeneity is the resolvers' own gate, read
    // here rather than re-proven.
    let mate_surface = surface_of(mate0)
        .ok_or_else(|| not_intact(EntityId::Face(mate0), "a rim's mate support"))?;
    // NOT KNOWN REACHABLE (disclosed narrowing, #935 fix pass): the
    // multi-arc resolver refuses a one-surface support pair at plan
    // time (its `ka == kb` gate), and a one-edge rim between two faces
    // of one surface is a tangency the battery refuses before any rim
    // resolves. No fixture reaches this arm; it stays a refusal rather
    // than an `unreachable!` because neither of those gates PROVES the
    // fact for every body this function can legally see, and an
    // ambiguous classification must not carve.
    if host_surface == mate_surface {
        return Err(unbuilt_chain(
            rim.chain.first().edge,
            "a rim's two supports carry ONE surface, so a re-read cannot tell its \
             host seam from its mate seam — blend the rims in SEQUENTIAL calls",
        ));
    }
    let chain_edges: Vec<EdgeKey> = rim.chain.links().map(|l| l.edge).collect();
    // The three refusal arms in this loop are NOT KNOWN REACHABLE for
    // the compositions the gate serves (disclosed narrowings, #935 fix
    // pass): an earlier ANNULUS carve preserves this incidence at
    // every other rim's crossing — it splits seams at its own feet and
    // kills its own vertices only — and both review lanes' mutation
    // passes found no fixture reddening any of them. They stay
    // refusals rather than `unreachable!`s because their premise is
    // what earlier CARVES did, not a fact this call established (the
    // Row-4 convention's own line), and each names the sequential
    // recourse, which is honest wherever they could fire.
    let mut live = Vec::with_capacity(ann.crossings.len());
    for c in &ann.crossings {
        let mut incident = vertex_edges_of(body, c.vertex)
            .ok_or_else(|| not_intact(EntityId::Vertex(c.vertex), "a rim vertex's edge orbit"))?;
        incident.sort_unstable();
        incident.dedup();
        let extras: Vec<EdgeKey> = incident
            .into_iter()
            .filter(|e| !chain_edges.contains(e))
            .collect();
        let (mut host_seam, mut mate_seam) = (None, None);
        for e in extras {
            let (fp, fm) = edge_faces(body, e)
                .ok_or_else(|| not_intact(EntityId::Edge(e), "a seam meridian's two faces"))?;
            let (sp, sm) = (
                surface_of(fp).ok_or_else(|| {
                    not_intact(EntityId::Face(fp), "a seam meridian's first support")
                })?,
                surface_of(fm).ok_or_else(|| {
                    not_intact(EntityId::Face(fm), "a seam meridian's second support")
                })?,
            );
            let slot = if sp == sm && sp == host_surface {
                &mut host_seam
            } else if sp == sm && sp == mate_surface {
                &mut mate_seam
            } else {
                return Err(unbuilt_chain(
                    rim.chain.first().edge,
                    "an earlier band's carve left an edge at a rim crossing that is not \
                     a co-surface seam meridian of either support — this composition is \
                     not repaired by a seam re-read; blend the rims in SEQUENTIAL calls",
                ));
            };
            if slot.replace(e).is_some() {
                return Err(unbuilt_chain(
                    rim.chain.first().edge,
                    "an earlier band's carve left two seam meridians in ONE support at a \
                     rim crossing — this composition is not repaired by a seam re-read; \
                     blend the rims in SEQUENTIAL calls",
                ));
            }
        }
        let Some(mate) = mate_seam else {
            return Err(unbuilt_chain(
                rim.chain.first().edge,
                "an earlier band's carve consumed a seam meridian at a rim crossing \
                 outright — this composition is not repaired by a seam re-read; blend \
                 the rims in SEQUENTIAL calls",
            ));
        };
        // A hostless crossing has no host seam to find, and finding one
        // would mean an earlier carve put a co-surface edge into a face
        // whose whole outer cycle is this rim — the same unrepairable
        // composition the arms above name.
        let host = match (&c.host, host_seam) {
            (HostFoot::Seam(_), Some(seam)) => HostFoot::Seam(seam),
            (HostFoot::Strut, None) => HostFoot::Strut,
            (HostFoot::Seam(_), None) | (HostFoot::Strut, Some(_)) => {
                return Err(unbuilt_chain(
                    rim.chain.first().edge,
                    "an earlier band's carve consumed a seam meridian at a rim crossing \
                     outright — this composition is not repaired by a seam re-read; blend \
                     the rims in SEQUENTIAL calls",
                ));
            }
        };
        live.push(LiveSeams { host, mate });
    }
    Ok(live)
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
) -> Result<RimShape, BlendError> {
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
    // the band's slit is minted from the MATE seam's rim-side piece
    // and the HOST seam's rim-side piece dies with this vertex, so a
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
        crossings: vec![SeamCrossing {
            vertex,
            host: HostFoot::Seam(host_seam),
            mate_seam,
        }],
        closure: 0,
    }))
}

/// The seam meridian of a revolution wall's boundary cycle: the one
/// edge the cycle traverses TWICE and which meets the rim at `vertex`.
///
/// **One rule, four readings** — this is the ONE-EDGE reading of the
/// seam incidence that [`resolve_seam_split_rim`] spells for several
/// arcs, that [`super::battery`]'s `is_seam_vertex` spells weakest
/// (a refusal classifier over incidence alone), and that
/// [`refresh_annulus_seams`] spells at carve time on a body carrying
/// earlier bands. See [`resolve_seam_split_rim`] for the intended
/// relation between them.
///
/// **Not merged with the multi-arc resolver, deliberately**, and the
/// drift hazard that decline accepts is stated at
/// [`resolve_seam_split_rim`]. The gate this function is here for —
/// a seam traversed exactly TWICE by one loop — is the
/// revolution-wall shape and has no counterpart on a seam-split body,
/// where each seam is traversed once by each of two half-band faces.
/// The rim itself must be carried once, so the wall really is the
/// one-rim-per-side shape the band replacement assumes.
fn wall_seam<T: Decide>(
    body: &Body<T>,
    lp: LoopKey,
    rim: EdgeKey,
    vertex: VertexKey,
) -> Result<EdgeKey, BlendError> {
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
fn ring_circle<T: Decide>(body: &Body<T>, ring: LoopKey) -> Result<(Point3<T>, T), BlendError> {
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
    // Row 0 (`D96`): NO — the non-emptiness is `topo::loop_cycle`'s,
    // and carrying it locally means `loop_walk` returning a split
    // head/tail through six call sites that index and length it. The
    // cost is written up in `docs/SMELL-T-LOG.md`'s `T-c` record.
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
/// this reason. Callers pass `host_is_a = (link.face_a == the link's own
/// host)`;
/// reading `trim_a` blind would take the mate's trim for the host's on
/// exactly the links where the two disagree. Pinned by
/// `tests::trim_selection_is_by_support_kind`.
#[allow(clippy::type_complexity)]
fn rim_trim_circles<T: Real>(
    edge: EdgeKey,
    blend: &EdgeBlend<T>,
    host_is_a: bool,
) -> Result<((Point3<T>, T), (Point3<T>, T)), BlendError> {
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
/// [`BlendError::RingClearance`]; an in-band margin escalates with
/// the SAME recourse (two-tolerance, D4 ¶1 addendum — this arm is
/// trio-pinned like every `fillet3_*` predicate). Crate-visible, and
/// reached from outside the crate only through the test-support door
/// ([`ring_clearance_for_tests`], re-exported as
/// `test_support::ring_clearance`) for exactly that trio: the margins
/// themselves are derived inside the surgery from stored trimlines and
/// ring carriers, never sampled, so no production caller exists.
///
/// Predicate 2's sampled screen meters the same gap first, and this
/// check is the EXACT form of it — the one the ring carry-through
/// soundness argument rests on, because sampling can overestimate a
/// gap and the closed form cannot.
///
/// **What that ordering does and does not give.** The screen samples
/// each boundary edge at `CHAIN_SAMPLES` places, so it reads a gap
/// that is never SMALLER than the true one. The invariant is therefore
/// one-sided: no request this check would pass is refused by the
/// screen, and any request the screen refuses would be refused here
/// too. It is NOT that the screen always answers first — when the
/// ring's closest approach to a requested edge lands between samples,
/// the sampled gap is strictly larger, and a setback in between passes
/// the screen and is refused HERE, at the front door. A 30°-turned
/// dimpled prism does exactly that
/// (`sweep/tests/review_fillet_e2_probes.rs`); the axis-aligned
/// fixtures this doc was written against do not, which is why it read
/// as "front-door screened".
///
/// # Errors
///
/// [`BlendError::RingClearance`] / [`BlendError::Escalated`].
pub(crate) fn ring_clearance<T: Decide + Bounds>(
    face: FaceKey,
    margin: T,
    band: Band,
) -> Result<(), BlendError> {
    match decide(RING_CLEARANCE, Margin::of(margin), band).map_err(|e| BlendError::Escalated {
        site: BlendSite::Chain,
        source: e,
    })? {
        Sign::Positive => Ok(()),
        sign => Err(BlendError::RingClearance {
            face,
            margin: super::battery::classified(RING_CLEARANCE, margin, band, sign),
        }),
    }
}

/// The test-support door to [`ring_clearance`]: the same function, made
/// nameable from this crate's `tests/` binaries for its two-tolerance
/// trio pin (`tests/m6_surgery.rs`) and compiled into no shipped build.
/// It lives here rather than in `test_support` because its signature
/// carries the surgery's own `Decide + Bounds` compound, which the
/// `Bounds` scope rule ratifies for the edge-blend seam alone —
/// `battery.rs`, `build.rs`, this file and the two open bands under
/// `open/` — and for no other file in the crate.
#[cfg(any(test, feature = "test-support"))]
pub fn ring_clearance_for_tests<T: Decide + Bounds>(
    face: FaceKey,
    margin: T,
    band: Band,
) -> Result<(), BlendError> {
    ring_clearance(face, margin, band)
}

/// The pre-mutation honesty pass (module docs): every ring of every
/// touched support face must clear every blend trimline by a definite
/// margin, in closed form.
fn ring_clearance_pass<T: Decide + Bounds>(
    body: &Body<T>,
    opens: &[AdmittedOpen<'_, T>],
    rims: &[RimPlan<'_, T>],
    band: Band,
) -> Result<(), BlendError> {
    // A ring's EFFECTIVE radius: its own circle, widened to the trim
    // circle when the ring is itself a requested rim (a single call
    // may blend the box edges and the rims together).
    let effective = |ring: LoopKey| -> Result<Option<(Point3<T>, T)>, BlendError> {
        for rim in rims {
            if rim.ladder_ring() == Some(ring) {
                let l0 = rim.chain.first();
                let (plane_trim, _) =
                    rim_trim_circles(l0.edge, &l0.blend, l0.face_a == rim.host0())?;
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
        // A rim's host side may be several faces (of one surface), and
        // each face's rings are its own question — so the walk is over
        // the DISTINCT host faces, once each. A ladder rim repeats one
        // plane in `hosts`, and metering its rings once per link would
        // be the same decision taken N times.
        let mut seen: Vec<FaceKey> = Vec::with_capacity(rim.hosts.len());
        for (l, &host) in rim.chain.links().zip(rim.hosts.iter()) {
            if seen.contains(&host) {
                continue;
            }
            seen.push(host);
            let ((ci, si), _) = rim_trim_circles(l.edge, &l.blend, l.face_a == host)?;
            let fd = body
                .get_face(host)
                .ok_or_else(|| not_intact(EntityId::Face(host), "a rim's host support"))?;
            for ring in fd.rings.clone() {
                if rim.ladder_ring() == Some(ring) {
                    continue;
                }
                let (cj, aj) = match effective(ring)? {
                    Some(widened) => widened,
                    None => ring_circle(body, ring)?,
                };
                let margin = (cj - ci).norm() - si - aj;
                ring_clearance(host, margin, band)?;
            }
        }
        let l0 = rim.chain.first();
        let ((ci, si), _) = rim_trim_circles(l0.edge, &l0.blend, l0.face_a == rim.host0())?;
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
        // passes near the trim circle. **Class (1) DOES occur** — a
        // revolve whose flat top runs in to a dome, repaired with
        // `merge_coplanar_faces`, is a ladder rim whose widened trim
        // circle sits CONCENTRIC inside the host's circular outer
        // boundary, and the external form reads minus the sum of the
        // two radii where the containment margin is comfortably
        // positive
        // (`work/fillet/ring-clearance-refuses-a-nested-trim-circle.md`,
        // which carries the fixture and the reading). Class (2) stays
        // unmeasured. A body that hits either refuses `RingClearance`
        // loudly rather than passing silently, which is why this is a
        // false refusal and not a soundness hole.
        // Anything else is already screened by predicate 2's sampled
        // sweep and adds nothing exact here.
        let outer = face_cycle(body, rim.host0()).ok_or_else(|| {
            not_intact(
                EntityId::Face(rim.host0()),
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
                    ring_clearance(rim.host0(), margin, band)?;
                }
                Curve3::Circle { center, radius, .. } => {
                    let margin = (center - ci).norm() - si - radius;
                    ring_clearance(rim.host0(), margin, band)?;
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
// The description pass's vocabulary: what every phase — the two open
// bands in `open/` and the two rim phases below — records for
// `attach_contact`.
// ------------------------------------------------------------------

/// A recorded new edge awaiting its intrinsic description.
pub(super) enum ContactCarrier<T: Real> {
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
    /// A ruled band's cut-off at a transverse cap: the arc of the cap
    /// plane's section of the band (a circle of the band's radius about
    /// the spine's crossing, sweep < π) — where the band meets the cap
    /// TRANSVERSALLY, so it is described as the plain intersection
    /// locus, never a tangent one.
    TransverseArc { center: Point3<T>, radius: T },
    /// An exact stored arc (the rim trim circles — π-safe).
    Exact(Curve3<T>, T, T),
    /// A torus band's SLIT: a double-traversed minor-circle arc
    /// described as the SEAM image of the band's own
    /// surface (sweep < π; the donut's representation).
    SeamArc { center: Point3<T>, radius: T },
}

pub(super) type Described<T> = Vec<(EdgeKey, ContactCarrier<T>)>;

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
) -> Result<RimCarrier<T>, BlendError> {
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
/// classification (the battery's junction-end pick precedent): the
/// parameter is [`geom::Curve3::param_near`] anchored at the CARRIER'S
/// SEAM — a circle carrier's angle on its principal branch, a line
/// carrier's the projection on its own direction — and either way a
/// parameter outside the stored span refuses typed rather than cutting
/// blind. `rim` names the requested edge the refusal carries.
///
/// # Why the anchor is the seam and NOT the stored window's midpoint
///
/// **Because the answer has to be a function of the POINT and the
/// CARRIER, and a window-derived anchor makes it a function of the
/// stored window too.** `param_near` is `near + atan2(…)` read in the
/// frame AT `near`, so the anchor enters the arithmetic continuously:
/// two calls asking about the same crossing on the same carrier return
/// parameters a few ulps apart if their windows differ. The stored
/// window is NOT stable across a surgery — carving one rim splits the
/// shared meridian and hands the next rim a narrower one — so a
/// window-derived anchor makes the split parameter depend on the ORDER
/// the rims were filleted in, and one-call composition stops agreeing
/// bitwise with sequential composition.
///
/// **That is measured, not feared.** With the midpoint anchor the zone
/// pair's shared meridian is split at `3.32886824659950897e-2` in one
/// order and `3.32886824659949787e-2` in the other — the same crossing,
/// ~6 ulps apart, because one window was `[0, 7.7627…e-1]` and the
/// other `[0, 7.3144…e-1]`. Three rows fail on it:
/// `blend2_r2_probes::r2_p1_zone_pair_equality_off_the_fixture_radius`,
/// `…::r2_p2_lantern_triple_equality_off_the_fixture_radius`, and
/// `verbs_arms1_r1_probes::both_zone_rims_in_one_call_match_the_sequential_composition`.
/// The seam is the only anchor available here that is a property of the
/// CARRIER alone, so it is the only one that restores the invariant;
/// with it all three are green.
///
/// This is the per-site anchor fact [`geom::Curve3::param_near`]'s docs
/// say each consumer owns, and it buys back nothing the consolidation
/// removed: no `k·2π` selection here, no `floor`, no five-candidate
/// scan — one anchored read and the window test.
///
/// # What makes the principal branch the RIGHT branch
///
/// The seam anchor returns the branch in `(−π, π]`, which need not be
/// the one inside the stored window. The period guard below is what
/// makes that sound rather than lucky: branches of a point are exactly
/// one period apart, so a window narrower than a period holds AT MOST
/// ONE of them. Hence a returned parameter that lands inside the window
/// IS the in-window branch; and if the in-window branch is some other
/// turn, the returned one falls outside and the window test refuses
/// typed. Both arms are sound and neither cuts blind.
///
/// Without the guard the failure is silent, and it is the one `geom`'s
/// `past_one_period_the_answer_aliases_by_a_turn_inside_the_span` row
/// exhibits: past a period the aliased answer is STILL strictly
/// interior, so the window test cannot see it. Measured on this tree no
/// live call comes near — 335 circle windows in the `sweep` suite,
/// widest `1.5825331081812426` rad against a period of
/// `6.2831853071795862`, every one of them inside `(−π, π]`. The
/// guard's refuse arm is therefore not reached by any assembly
/// fixture. It is here because the distance between "measured to be
/// fine" and "checked" is exactly the comment it replaces.
pub(super) fn seam_split_param<T: Decide + Bounds>(
    body: &Body<T>,
    seam: EdgeKey,
    rim: EdgeKey,
    target: Point3<T>,
) -> Result<T, BlendError> {
    let sd = body
        .get_edge(seam)
        .ok_or_else(|| not_intact(EntityId::Edge(seam), "a support edge the band splits"))?;
    let Some(sc) = body.get_curve_geom(sd.curve).and_then(|g| g.certified()) else {
        return Err(unbuilt_geometry(
            EntityId::Edge(seam),
            "a support edge the band splits carries no certified carrier",
        ));
    };
    let (st0, st1) = sc.params();
    // THE PERIOD GUARD, and it is CHECKED rather than assumed — the
    // function docs derive why it is what makes the principal branch
    // the in-window one. It stays a BOUNDS-lane read, like the window
    // test below and for the same reason (the battery's junction-end
    // pick precedent): this is a question about the STORED window, a
    // representation datum, not about where a point lies. A window
    // whose whole enclosure is not strictly under a period refuses
    // typed rather than cutting blind.
    //
    // A period is a CIRCLE's: a line carrier (a transverse cap's chord
    // rim) has no branch to alias by, and its window is a length that
    // may exceed 2π without meaning anything. The skip is by
    // construction, not by measurement: no fixture in the tree splits a
    // line rim longer than 2π, so an unconditional guard survives every
    // row today — the carrier-kind test is here because the sentence
    // above is true, not because a row demands it.
    //
    // `topo`'s edge split spells the same guard as the
    // `bool_split_span_period` DECIDE row, which is the right posture
    // there and not here: that site is mid-classification with a band
    // in hand, this one is picking a representation and has neither.
    if matches!(sc.carrier(), Curve3::Circle { .. }) && (T::tau() - (st1 - st0)).lo() <= 0.0 {
        return Err(unbuilt_geometry(
            EntityId::Edge(seam),
            "a split edge's stored window is not under one period; the split parameter would \
             alias by a turn and still land inside the window",
        ));
    }
    // Anchored at the CARRIER'S SEAM, not at the stored window — the
    // function docs carry the measured reason (a window-derived anchor
    // makes the split parameter order-dependent). `param_near` carries
    // the derivation of the anchored read itself.
    let t = sc.carrier().param_near(target, T::zero()).ok_or_else(|| {
        unbuilt_geometry(
            EntityId::Edge(seam),
            "a split edge's carrier is neither a circle nor a line; the split reads the \
                 crossing in the carrier's own frame and no other stored shape is built",
        )
    })?;
    // The window test is the representation pick's other half, and it
    // stays a BOUNDS-lane read rather than a `decide` row (the
    // battery's junction-end pick precedent): a parameter whose whole
    // enclosure is not strictly inside the stored span refuses typed
    // rather than cutting blind.
    let inside = |t: T| (t - st0).lo() > 0.0 && (st1 - t).lo() > 0.0;
    if inside(t) {
        return Ok(t);
    }
    // The in-window branch may be the principal one's neighbour by a
    // turn (a cap arc sweeping past π puts its far foot there), and
    // the period guard above is what makes that neighbour UNIQUE: a
    // window under one period holds at most one branch, so a shifted
    // parameter that lands inside is the branch, and the value is still
    // a function of the point and the carrier alone — the window only
    // says which turn.
    if matches!(sc.carrier(), Curve3::Circle { .. }) {
        for shifted in [t + T::tau(), t - T::tau()] {
            if inside(shifted) {
                return Ok(shifted);
            }
        }
    }
    Err(unbuilt_chain(
        rim,
        "a trimline does not cross the support edge it splits inside that edge's span",
    ))
}

/// **The band's STRUT foot on a host support** — the `mev` from a rim
/// vertex out to that support's trimline, and the ONE home of the move
/// for both closed-rim phases: the ladder struts at every vertex of its
/// ring, and the annulus struts at a crossing whose host side has no
/// seam to split ([`HostFoot::Strut`]).
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
/// [`blend_surgery`]'s debug postcondition takes.
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
fn strut_foot<T: Decide + Bounds>(
    body: &mut Body<T>,
    he: HalfEdgeKey,
    v: VertexKey,
    fp: Point3<T>,
    site: &'static str,
    tol: Tol,
) -> Result<topo::MevCreated, BlendError> {
    let p = point_of(body, v).ok_or_else(|| not_intact(EntityId::Vertex(v), "a rim vertex"))?;
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

fn rim_phase<T: Decide + Bounds>(
    body: &mut Body<T>,
    rim: &RimPlan<'_, T>,
    ring: LoopKey,
    sources: &SourceFaces,
    rec: &mut BlendNaming,
    tol: Tol,
) -> Result<(FaceKey, Surface<T>, Described<T>), BlendError> {
    let mut described: Described<T> = Vec::new();
    let link_of = |e: EdgeKey| -> Option<&Link<T>> { rim.chain.links().find(|l| l.edge == e) };
    // Selected by the carve's ROLES, never by slot
    // (`rim_trim_circles` docs): `trim_a` is the MATE-side trim on any
    // link whose `he_plus` lies on the cap side. The ladder's gates
    // require the mate to be a ring-free half-cap, not a sphere.
    let l0 = rim.chain.first();
    let ((ca, sa), (cb, sb)) = rim_trim_circles(l0.edge, &l0.blend, l0.face_a == rim.host0())?;

    // The rim edges' stored carriers, once.
    let carrier_of = |body: &Body<T>, e: EdgeKey| -> Result<RimCarrier<T>, BlendError> {
        let l = link_of(e)
            .ok_or_else(|| not_intact(EntityId::Edge(e), "a rim edge's link in the verdict"))?;
        rim_carrier(body, l, rim.host0())
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
        // The foot inherits the rim vertex's own parameter on the
        // scaled carrier — azimuth preserved exactly, no atan2. Same
        // reported finding as the support strut above: a radial chord of
        // the ring is a chord, not an image of the ring's chart, so it
        // reaches rest through the scaffolding door and the fence names
        // it.
        let fp = curve.eval(t0);
        let created = strut_foot(body, he, v, fp, "rim strut mev", tol)?;
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
            .push((created.edge, plane_walk[i].2, RimSide::Host));
        let (curve, t0, t1) = ta_carriers[i].clone();
        described.push((created.edge, ContactCarrier::Exact(curve, t0, t1)));
    }

    // ---- (4) The MATE side: one trim chord per half-cap, hung
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
        let ((he1, v1), (he2, v2)) =
            flank(body, lp, |row| row.0 == s_half, 1, 2).ok_or_else(|| {
                not_intact(
                    EntityId::Loop(lp),
                    "a half-cap loop does not carry the half-edge whose parent it is",
                )
            })?;
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
            .map_err(|e| op("rim mate trim mef", e))?;
        let (curve, t0, t1) = scaled(&rc, cb, sb, !rc.plus_on_host);
        described.push((created.edge, ContactCarrier::Exact(curve, t0, t1)));
        rec.rim_trims.push((created.edge, e, RimSide::Mate));
        tb_edges.push(created.edge);
    }

    // ---- (5) Excise: kill each rim edge across its two strips. ----
    for l in rim.chain.links() {
        let half = host_side_half(body, l, rim.host0())
            .ok_or_else(|| not_intact(EntityId::Edge(l.edge), "a rim edge's plane-side half"))?;
        sources.kef_minted(body, half, "rim kef")?;
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
            sources.kef_minted(body, hp, "rim strut kef")?;
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
    // Row 0 (`D96`): NO — one row per `plane_walk` position, so the
    // non-emptiness is the cycle's, the same cause as `ring_circle`'s
    // (`docs/SMELL-T-LOG.md`, `T-c`).
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
    // Row 0 (`D96`): NO — "exactly one strut reaches the closure case"
    // is the outcome of a walk over the ring, not a shape a type can
    // carry to this line (`docs/SMELL-T-LOG.md`, `T-c`).
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

/// The two half-edges a support's trim `mef` runs between: the one
/// starting the RIM-side seam piece at one of this support's feet, and
/// the one starting the FAR-side piece at a foot. The run between them
/// is the rim side, so it is what moves to the new strip face.
///
/// A one-edge rim's two halves start at the SAME foot, which is what
/// makes its trim a closed edge from that vertex to itself; a rim a
/// chart seam split has one foot per end of the arc, so its trim is an
/// arc between them.
///
/// This is the SEAM side's pick, keyed on the two pieces a split leaves
/// at a foot. A hostless host has no pieces and is picked by position
/// instead ([`flank`]); the two are not one because what they
/// key on is what differs.
fn trim_chords<T: Decide>(
    body: &Body<T>,
    lp: LoopKey,
    feet: &[(VertexKey, EdgeKey, EdgeKey)],
) -> Option<(HalfEdgeKey, HalfEdgeKey)> {
    let walk = loop_walk(body, lp)?;
    let pick = |rim_side: bool| {
        walk.iter()
            .find(|(_, v, k)| {
                feet.iter()
                    .any(|(f, r, x)| f == v && *k == if rim_side { *r } else { *x })
            })
            .map(|(h, _, _)| *h)
    };
    Some((pick(true)?, pick(false)?))
}

/// One `mef` for one support's trim: the arc between the two feet, or
/// the closed circle at the one foot a rim the charts did not split
/// has. The carrier is the rim's own frame scaled to the trimline
/// ([`scaled`]), so the arc's azimuths are inherited rather than
/// reconstructed; the description pass restates it as the tangential
/// contact locus once the band's torus exists.
#[allow(clippy::too_many_arguments)]
fn mef_trim<T: Decide + Bounds>(
    body: &mut Body<T>,
    he1: HalfEdgeKey,
    he2: HalfEdgeKey,
    curve: &Curve3<T>,
    window: (T, T),
    rim_edge: EdgeKey,
    site: &'static str,
    tol: Tol,
) -> Result<topo::MefCreated, BlendError> {
    let (Some(v1), Some(v2)) = (
        body.get_half_edge(he1).map(|h| h.start),
        body.get_half_edge(he2).map(|h| h.start),
    ) else {
        return Err(not_intact(EntityId::HalfEdge(he1), "a trim chord's foot"));
    };
    let spec = if v1 == v2 {
        let p = point_of(body, v1)
            .ok_or_else(|| not_intact(EntityId::Vertex(v1), "a rim band's foot"))?;
        EdgeCurveSpec::self_loop_circle_at(p)
    } else {
        EdgeCurveSpec::arc_of_circle(curve.clone(), window.0, window.1).ok_or_else(|| {
            unbuilt_geometry(
                EntityId::Edge(rim_edge),
                "a rim band's trimline is not a circle, so it has no arc between its feet",
            )
        })?
    };
    body.mef(
        MefSite::Chords { he1, he2 },
        spec,
        FaceSurface::Inherit,
        tol,
    )
    .map_err(|e| op(site, e))
}

/// **One crossing's HOST foot as the carve holds it** — the mid-walk
/// twin of [`HostFoot`], which says where the foot COMES from; this says
/// what minting it left behind.
enum HostAnchor {
    /// The seam was split at the foot: its rim-side piece runs foot →
    /// crossing and dies there, its far-side piece survives as the
    /// support's remnant.
    Seam {
        foot: VertexKey,
        rim_side: EdgeKey,
        far_side: EdgeKey,
    },
    /// A strut was minted from the crossing out to the foot. There is no
    /// far side — that is the whole difference at a hostless crossing —
    /// and the strut is a mid-call entity that never reaches rest, so it
    /// owes no birth or death row of its own.
    Strut { foot: VertexKey, strut: EdgeKey },
}

impl HostAnchor {
    /// The foot vertex, minted either way.
    fn foot(&self) -> VertexKey {
        match *self {
            Self::Seam { foot, .. } | Self::Strut { foot, .. } => foot,
        }
    }

    /// The edge running FOOT → crossing, which the crossing's `kev`
    /// kills from the foot side. A seam's rim-side piece and a strut
    /// play the same role here, which is why the closure step below has
    /// no branch.
    fn dying(&self) -> EdgeKey {
        match *self {
            Self::Seam { rim_side, .. } => rim_side,
            Self::Strut { strut, .. } => strut,
        }
    }
}

/// **The two half-edges flanking a position in a loop's cycle** — the
/// run a `mef` moves onto a new face, keyed by the half-edge `at`
/// picks: the one `back` positions before it (inclusive) and the one
/// `fwd` positions after it (exclusive), each with its start vertex.
/// ONE spelling for every chord the carve hangs between two existing
/// vertices — here, and in the two open bands under `open/` through
/// [`chord_site`]: the corner arc (`0, 2`, keyed on the half-edge ENDING at
/// the corner), the hostless host's rim arc (`1, 2`, keyed on the arc's
/// own half), and the ruled band's cap arc (`0, 2`) and trimlines
/// (`1, 2`).
///
/// **What is NOT shared is what a foot IS**, and that is deliberately
/// the caller's: the ladder's mate side asks for a meridian split
/// point, the hostless host for one of this phase's strut feet, the
/// ruled band for the rim split it just made — and each refuses in its
/// own words. Returning the vertices rather than taking a predicate
/// keeps each caller's check at its own site, where its refusal
/// sentence is.
///
/// Read LIVE, once per chord: each `mef` splits the face under it, so
/// by the last chord the half after it can be an earlier trim rather
/// than a foot edge — still starting at a foot, which is the property
/// that has to hold. `None` where the loop does not walk or does not
/// carry the keyed half-edge.
pub(super) fn flank<T: Decide>(
    body: &Body<T>,
    lp: LoopKey,
    at: impl Fn(&(HalfEdgeKey, VertexKey, EdgeKey)) -> bool,
    back: usize,
    fwd: usize,
) -> Option<((HalfEdgeKey, VertexKey), (HalfEdgeKey, VertexKey))> {
    let walk = loop_walk(body, lp)?;
    let k = walk.len();
    let pos = walk.iter().position(at)?;
    let (h1, v1, _) = walk[(pos + k - back) % k];
    let (h2, v2, _) = walk[(pos + fwd) % k];
    Some(((h1, v1), (h2, v2)))
}

/// [`flank`] on a face's OUTER cycle, refusing typed where the cycle
/// does not walk or does not carry the keyed half-edge — the ruled
/// band's and the corner arc's spelling, whose chords always hang in an
/// outer cycle (the cut-off `mef` leaves a cap's rings on the cap; a
/// support with a ring is refused at the plan).
pub(super) fn chord_site<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    at: impl Fn(&(HalfEdgeKey, VertexKey, EdgeKey)) -> bool,
    back: usize,
    fwd: usize,
) -> Result<(HalfEdgeKey, HalfEdgeKey, VertexKey, VertexKey), BlendError> {
    let outer = body
        .get_face(face)
        .ok_or_else(|| not_intact(EntityId::Face(face), "a face whose cycle a chord spans"))?
        .outer;
    let ((he1, v1), (he2, v2)) = flank(body, outer, at, back, fwd).ok_or_else(|| {
        not_intact(
            EntityId::Face(face),
            "a face's outer cycle does not walk, or does not carry the half-edge the carve \
             keys on",
        )
    })?;
    Ok((he1, he2, v1, v2))
}

/// Where one crossing's two feet go, and the arc whose stored frame put
/// them there — which is the arc a refusal at that crossing names.
struct FootTarget<T: Real> {
    host: Point3<T>,
    mate: Point3<T>,
    named: EdgeKey,
}

/// One rim arc's two trimlines, in each support's own traversal, and
/// the crossings its host-side traversal runs between.
struct ArcPlan<T: Real> {
    /// The HOST trimline's carrier, oriented with the host loop, and
    /// its parameter window.
    host_curve: Curve3<T>,
    host_window: (T, T),
    /// The host trimline's circle, for the band chart's radial.
    host_circle: (Point3<T>, T),
    /// The MATE trimline's carrier, oriented with the mate loop, and
    /// its parameter window.
    mate_curve: Curve3<T>,
    mate_window: (T, T),
    /// The crossing indices this arc's HOST-side traversal starts and
    /// ends at. Equal on a rim of one closed edge.
    at: (usize, usize),
}

/// **The annulus band** — the closed rim's surgery, arc by arc.
///
/// The band between two revolution walls is one more wall: two closed
/// boundary circles (the trim circles on each support) joined by a
/// double-traversed SLIT at one azimuth, because a curved face must be
/// ring-free (`props`' closed-form inventory; the donut's own
/// representation). There is no strut-and-`kef` ladder to walk — every
/// arc of the rim carries the SAME support pair — so the band is minted
/// by moves at the rim's crossings:
///
/// 1. `split_edge` each crossing's MATE seam where the mate trimline
///    crosses it — the rim-side piece becomes the slit or dies;
/// 2. land each crossing's HOST foot, by whichever move its plan names
///    ([`HostFoot`]): `split_edge` its HOST seam where the host trimline
///    crosses it, the foot then lying on existing geometry; or, at a
///    HOSTLESS crossing, mint it with the ladder's strut `mev`
///    ([`strut_foot`]) — the two moves differ only in whether a seam
///    was there to split, and both leave one foot vertex joined to the
///    crossing by one edge that dies at step 6;
/// 3. `mef` on each host support between the halves at its feet that
///    start the rim-side and the far-side seam pieces — or, hostless,
///    the strut halves flanking the arc ([`flank`]) — the host
///    trim, carving that support's outer strip off the shrunk face;
/// 4. `mef` likewise on each mate support;
/// 5. `kef` each rim arc, merging its two strips into one sector;
/// 6. `kev` each crossing's host seam rim-side piece from the FOOT
///    side — the crossing vertex dies and the mate seam's rim-side
///    piece fan-merges onto the host foot;
/// 7. at every crossing but the CLOSURE one, `kef` that re-based mate
///    piece as well, merging the two sectors it separated.
///
/// The closure crossing's mate piece survives as the band's SLIT,
/// exactly as the ladder's closure `kev` leaves one at its closure
/// vertex. A rim of ONE closed edge has one crossing, so steps 5–6 are
/// its whole closure and step 7 does not run; a rim a chart seam split
/// walks THROUGH its other crossings, which is sound because the
/// surface is smooth there — the seam is a chart artifact and the
/// meridians crossing it are co-surface, dihedral zero by construction.
/// A HOSTLESS crossing is walked through on the same ground: its host
/// side is one smooth face either way, and the strut is scaffolding
/// this call both mints and consumes.
///
/// **The struts leave the body tier-2 invalid between steps 2 and 3**,
/// because a foot is valence-1 until its trim `mef` lands — measured,
/// and the LADDER's identical `mev`-then-`mef` gap, not something the
/// hostless arm introduces. The seam arm has no such window (a split
/// leaves both pieces attached). Validity is a claim about the body at
/// REST, which [`blend_surgery`]'s debug postcondition takes.
///
/// Both supports keep their `FaceKey`, their surface, their sense bit and
/// their rings; the surviving strip is the band. The trim arcs' and
/// the slit's carriers are attached in the caller's description pass,
/// once the band's torus exists.
/// `live` carries the crossings' seam meridians under their CARVE-time
/// keys, one entry per crossing in `ann.crossings` order — the plan's
/// own keys unless an earlier band's carve on a shared wall split them
/// ([`refresh_annulus_seams`]). The SPLITS below run on the live keys;
/// every naming row keeps the PLAN's key, because a birth record names
/// the SOURCE entity an output was minted for and the live piece is a
/// mid-call fragment of it.
fn rim_phase_annulus<T: Decide + Bounds>(
    body: &mut Body<T>,
    rim: &RimPlan<'_, T>,
    ann: &AnnulusRim,
    live: &[LiveSeams],
    sources: &SourceFaces,
    rec: &mut BlendNaming,
    tol: Tol,
) -> Result<(FaceKey, Surface<T>, Described<T>), BlendError> {
    // `live` pairs with `ann.crossings` BY INDEX. Both producers mint
    // exactly one entry per crossing (the identity map and the
    // refresh's per-crossing loop), so a length divergence is a fact
    // no input can produce — checked here so the pairing convention
    // cannot rot silently under a third producer.
    if live.len() != ann.crossings.len() {
        unreachable!(
            "annulus band: `live` carries one seam pair per crossing — both producers \
             iterate the crossings themselves"
        )
    }
    let mut described: Described<T> = Vec::new();
    let l0 = rim.chain.first();
    let n = rim.chain.link_count();
    // ONE torus over the whole rim: the first arc's blend states it, and
    // every other arc's trimline is described against it, so an arc
    // whose own blend disagreed is reported at rest rather than blended
    // away here.
    //
    // **What that leaves unchecked AT THIS DOOR, said plainly.** The
    // arcs' tori are not compared to each other here, and nothing in
    // this phase re-derives geometry: the door checks STRUCTURE (the
    // plan's gates, all against the source body) and mints, while every
    // geometric claim it makes — each trimline lying on the tangential
    // contact locus of the band and its support, the slit lying in the
    // band chart's own `u_ref` half-plane, the band being a torus at
    // all — is re-derived by tier 3 at rest, on the caller's side of
    // `fillet_edges`. So a disagreement between two arcs' blends is
    // DETECTED, but by `validate_geometric` and not here, which is why
    // every fixture row for this door validates rather than trusting
    // the mint.
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

    // ---- Per ARC: its two trimlines, selected by support FACE and
    // never by slot (`rim_trim_circles` docs), in the rim's own frame
    // (`scaled`) so every azimuth is inherited rather than
    // reconstructed. ----
    let ix_of = |v: VertexKey| -> Result<usize, BlendError> {
        ann.crossings
            .iter()
            .position(|c| c.vertex == v)
            .ok_or_else(|| {
                not_intact(
                    EntityId::Vertex(v),
                    "a rim arc's end among the rim's own crossings",
                )
            })
    };
    let mut arcs: Vec<ArcPlan<T>> = Vec::with_capacity(n);
    for (i, l) in rim.chain.links().enumerate() {
        let host = rim.hosts[i];
        let ((ca, sa), (cb, sb)) = rim_trim_circles(l.edge, &l.blend, l.face_a == host)?;
        let rc = rim_carrier(body, l, host)?;
        let (host_curve, hp0, hp1) = scaled(&rc, ca, sa, rc.plus_on_host);
        let (mate_curve, mp0, mp1) = scaled(&rc, cb, sb, !rc.plus_on_host);
        let half = host_side_half(body, l, host)
            .ok_or_else(|| not_intact(EntityId::Edge(l.edge), "a rim arc's host-side half"))?;
        let (Some(s), Some(e)) = (
            body.get_half_edge(half).map(|h| h.start),
            body.half_edge_end(half),
        ) else {
            return Err(not_intact(
                EntityId::HalfEdge(half),
                "a rim arc's host-side traversal",
            ));
        };
        arcs.push(ArcPlan {
            host_curve,
            host_window: (hp0, hp1),
            host_circle: (ca, sa),
            mate_curve,
            mate_window: (mp0, mp1),
            at: (ix_of(s)?, ix_of(e)?),
        });
    }

    // Each crossing's two feet, and the arc that names it in a refusal:
    // read off the arc whose host-side traversal STARTS there, so both
    // azimuths come from ONE arc's stored frame. The host trimline runs
    // with the host loop and the mate trimline against it, which is why
    // the mate foot is the mate window's FAR end.
    let mut targets: Vec<Option<FootTarget<T>>> =
        core::iter::repeat_with(|| None).take(n).collect();
    // Which arc's host traversal starts at each crossing — the same
    // correspondence, kept because a STRUT foot is minted into that
    // arc's own host half-edge and needs it by index.
    let mut starter: Vec<Option<usize>> = core::iter::repeat_with(|| None).take(n).collect();
    for (i, (a, l)) in arcs.iter().zip(rim.chain.links()).enumerate() {
        targets[a.at.0] = Some(FootTarget {
            host: a.host_curve.eval(a.host_window.0),
            mate: a.mate_curve.eval(a.mate_window.1),
            named: l.edge,
        });
        starter[a.at.0] = Some(i);
    }
    let mut feet_targets = Vec::with_capacity(n);
    let mut starters = Vec::with_capacity(n);
    for ((slot, start), c) in targets
        .drain(..)
        .zip(starter.drain(..))
        .zip(ann.crossings.iter())
    {
        let (Some(t), Some(start)) = (slot, start) else {
            return Err(not_intact(
                EntityId::Vertex(c.vertex),
                "a rim crossing no arc's host-side traversal starts at",
            ));
        };
        feet_targets.push(t);
        starters.push(start);
    }

    // ---- (1)+(2) The seam splits. Each mints one foot vertex on
    // EXISTING geometry; the piece still touching the crossing vertex is
    // the rim-side one. ----
    let split = |body: &mut Body<T>,
                 seam: EdgeKey,
                 at: VertexKey,
                 target: Point3<T>,
                 named: EdgeKey,
                 site: &'static str|
     -> Result<(VertexKey, EdgeKey, EdgeKey), BlendError> {
        let t = seam_split_param(body, seam, named, target)?;
        let created = body.split_edge(seam, t, tol).map_err(|e| op(site, e))?;
        let (rim_side, far_side) = if edge_touches(body, seam, at) {
            (seam, created.new_edge)
        } else {
            (created.new_edge, seam)
        };
        Ok((created.vertex, rim_side, far_side))
    };
    // A split of an edge an EARLIER band recorded as a meridian
    // remnant supersedes that record: the piece it named is subdivided
    // here, and this band's own rows re-cover both children (the far
    // piece as its remnant, the rim-side piece as its slit or as a
    // death). Retiring the row before the split is what keeps the
    // records a partition — one row per output entity, which the
    // emitter refuses to violate.
    let mut mate_feet = Vec::with_capacity(n);
    for (ix, c) in ann.crossings.iter().enumerate() {
        rec.meridian_remnants.retain(|(e, _)| *e != live[ix].mate);
        mate_feet.push(split(
            body,
            live[ix].mate,
            c.vertex,
            feet_targets[ix].mate,
            feet_targets[ix].named,
            "annulus mate seam split",
        )?);
    }
    // The HOST foot, by whichever move this crossing's plan says. A seam
    // split lands it on existing geometry; a hostless crossing mints it
    // with the LADDER's strut, from the arc that starts here so the
    // parameter is that arc's own window start — the same point the
    // seam split targets, reached without a seam.
    let mut host_feet = Vec::with_capacity(n);
    for (ix, c) in ann.crossings.iter().enumerate() {
        let anchor = match live[ix].host {
            HostFoot::Seam(seam) => {
                rec.meridian_remnants.retain(|(e, _)| *e != seam);
                let (foot, rim_side, far_side) = split(
                    body,
                    seam,
                    c.vertex,
                    feet_targets[ix].host,
                    feet_targets[ix].named,
                    "annulus host seam split",
                )?;
                HostAnchor::Seam {
                    foot,
                    rim_side,
                    far_side,
                }
            }
            HostFoot::Strut => {
                let i = starters[ix];
                let l = rim.chain.links().nth(i).ok_or_else(|| {
                    not_intact(
                        EntityId::Vertex(c.vertex),
                        "a rim crossing's own starting arc among the chain's links",
                    )
                })?;
                let he = host_side_half(body, l, rim.hosts[i]).ok_or_else(|| {
                    not_intact(EntityId::Edge(l.edge), "a rim arc's host-side half")
                })?;
                // The foot target this crossing already holds — the
                // arc that STARTS here, evaluated at its own window
                // start, which is exactly what the seam arm splits
                // toward.
                let created = strut_foot(
                    body,
                    he,
                    c.vertex,
                    feet_targets[ix].host,
                    "annulus host strut mev",
                    tol,
                )?;
                HostAnchor::Strut {
                    foot: created.vertex,
                    strut: created.edge,
                }
            }
        };
        host_feet.push(anchor);
    }
    let host_foot_vertices: Vec<VertexKey> = host_feet.iter().map(HostAnchor::foot).collect();
    // `trim_chords` keys on the two seam PIECES at a foot, which only a
    // seam anchor has; the rows it reads are built once here so the
    // pick below stays the seam path's own, untouched.
    let seam_chord_feet: Vec<(VertexKey, EdgeKey, EdgeKey)> = host_feet
        .iter()
        .filter_map(|a| match *a {
            HostAnchor::Seam {
                foot,
                rim_side,
                far_side,
            } => Some((foot, rim_side, far_side)),
            HostAnchor::Strut { .. } => None,
        })
        .collect();

    // ---- (3)+(4) The trimlines, one `mef` per support face. The run
    // that moves to the NEW face is the rim side, so each support keeps
    // its own key and the strips are the new faces. ----
    let mut host_trims = Vec::with_capacity(n);
    for (i, l) in rim.chain.links().enumerate() {
        let half = host_side_half(body, l, rim.hosts[i])
            .ok_or_else(|| not_intact(EntityId::Edge(l.edge), "a rim arc's host-side half"))?;
        let lp = loop_of_half(body, half)
            .ok_or_else(|| not_intact(EntityId::Edge(l.edge), "a rim arc's host-side loop"))?;
        // A seam host is picked by the two pieces at its feet; a
        // hostless one has no pieces, so its two halves are the ones
        // FLANKING this arc in the live loop.
        let (he1, he2) = if seam_chord_feet.is_empty() {
            let ((h1, v1), (h2, v2)) =
                flank(body, lp, |row| row.0 == half, 1, 2).ok_or_else(|| {
                    not_intact(
                        EntityId::Loop(lp),
                        "this arc's two strut feet around it in the host's loop",
                    )
                })?;
            // DEFENSIVE, not a gate: by construction this loop carries
            // only the rim's arcs (the outer-cycle gate admitted exactly
            // those), this phase's own struts, and the trims it has
            // already minted between feet — so the halves flanking an
            // arc start at feet. Kept because the alternative to
            // checking is cutting the face blind, and a wrong `mef` site
            // is a wrong solid rather than a refusal.
            if !(host_foot_vertices.contains(&v1) && host_foot_vertices.contains(&v2)) {
                return Err(not_intact(
                    EntityId::Loop(lp),
                    "this arc's two strut feet around it in the host's loop",
                ));
            }
            (h1, h2)
        } else {
            trim_chords(body, lp, &seam_chord_feet).ok_or_else(|| {
                not_intact(
                    EntityId::Loop(lp),
                    "a split seam's rim-side and far-side halves at this support's feet",
                )
            })?
        };
        host_trims.push(mef_trim(
            body,
            he1,
            he2,
            &arcs[i].host_curve,
            arcs[i].host_window,
            l.edge,
            "annulus host trim mef",
            tol,
        )?);
    }
    let mut mate_trims = Vec::with_capacity(n);
    for (i, l) in rim.chain.links().enumerate() {
        let hhalf = host_side_half(body, l, rim.hosts[i])
            .ok_or_else(|| not_intact(EntityId::Edge(l.edge), "a rim arc's host-side half"))?;
        let ed = body
            .get_edge(l.edge)
            .ok_or_else(|| not_intact(EntityId::Edge(l.edge), "a rim arc"))?;
        let mhalf = if ed.he_plus == hhalf {
            ed.he_minus
        } else {
            ed.he_plus
        };
        let lp = loop_of_half(body, mhalf)
            .ok_or_else(|| not_intact(EntityId::HalfEdge(mhalf), "a rim arc's mate-side loop"))?;
        let (he1, he2) = trim_chords(body, lp, &mate_feet).ok_or_else(|| {
            not_intact(
                EntityId::Loop(lp),
                "a split seam's rim-side and far-side halves at this support's feet",
            )
        })?;
        mate_trims.push(mef_trim(
            body,
            he1,
            he2,
            &arcs[i].mate_curve,
            arcs[i].mate_window,
            l.edge,
            "annulus mate trim mef",
            tol,
        )?);
    }

    // ---- (5) Excise: kill each rim arc across its two strips, from the
    // HOST strip's side, so the mate strips survive as the band's
    // sectors (the ladder's excise convention). ----
    for (i, l) in rim.chain.links().enumerate() {
        let dying = host_side_half(body, l, rim.hosts[i])
            .ok_or_else(|| not_intact(EntityId::Edge(l.edge), "a rim arc's host-side half"))?;
        sources.kef_minted(body, dying, "annulus rim kef")?;
    }

    // ---- (6)+(7) The crossings. Carry-through ones first, so every
    // sector merge still has two sectors to merge; the closure crossing
    // runs last and keeps its mate piece as the slit. ----
    let order = (0..n)
        .filter(|ix| *ix != ann.closure)
        .chain(core::iter::once(ann.closure));
    for ix in order {
        let c = &ann.crossings[ix];
        let Some((hp, hm)) = halves_of(body, host_feet[ix].dying()) else {
            unreachable!(
                "annulus band: the host's foot→crossing edge — a seam's rim-side piece or \
                 a strut — came out of this phase's own mint and nothing between there \
                 and here kills it"
            )
        };
        let dying = if body.half_edge_end(hm) == Some(c.vertex) {
            hm
        } else {
            hp
        };
        body.kev(dying).map_err(|e| op("annulus closure kev", e))?;
        if ix == ann.closure {
            continue;
        }
        // The mate piece now spans the two feet with a sector on each
        // side; killing it merges them, which is what carrying the walk
        // through this crossing means.
        let Some((mp, _)) = halves_of(body, mate_feet[ix].1) else {
            unreachable!(
                "annulus band: the mate seam's rim-side piece came out of this phase's \
                 own `split_edge` and only the closure crossing keeps one"
            )
        };
        sources.kef_minted(body, mp, "annulus seam-crossing kef")?;
    }

    // ---- The band's chart is SEAMED at the slit (certification demands
    // a Seam edge lie in its surface's own `u_ref` half-plane; the chart
    // reference is conventional data, D2). ----
    let closure = &ann.crossings[ann.closure];
    let Some(closure_arc) = arcs.iter().position(|a| a.at.0 == ann.closure) else {
        unreachable!(
            "annulus band: every crossing is the host-side START of exactly one arc — \
             the plan checked it (`resolve_seam_split_rim`'s one-cycle gate, and a \
             one-edge rim's single crossing is its arc's own two ends), and the feet \
             above were derived through that same correspondence"
        )
    };
    let (cc, cr) = arcs[closure_arc].host_circle;
    let radial = (feet_targets[ann.closure].host - cc) / cr;
    let band_surface = Surface::Torus {
        center: tc,
        axis: taxis,
        major_radius: tmaj,
        minor_radius: tmin,
        u_ref: radial,
    };
    let Some(first_mate) = mate_trims.first() else {
        unreachable!(
            "annulus band: one mate trim is minted per chain link and a chain always \
             carries its first link"
        )
    };
    let Some(band_face) = face_of_half(body, first_mate.he_minus) else {
        unreachable!(
            "annulus band: a mate trim's minus half bounds the strip `mef` minted for \
             it, and the `kef`s above kill the HOST strips"
        )
    };
    if rim.mates.contains(&band_face) || rim.hosts.contains(&band_face) {
        return Err(not_intact(
            EntityId::Face(band_face),
            "the annulus band merged back into one of its own supports",
        ));
    }

    for (i, _) in rim.chain.links().enumerate() {
        described.push((
            host_trims[i].edge,
            ContactCarrier::Exact(
                arcs[i].host_curve.clone(),
                arcs[i].host_window.0,
                arcs[i].host_window.1,
            ),
        ));
    }
    for (i, _) in rim.chain.links().enumerate() {
        described.push((
            mate_trims[i].edge,
            ContactCarrier::Exact(
                arcs[i].mate_curve.clone(),
                arcs[i].mate_window.0,
                arcs[i].mate_window.1,
            ),
        ));
    }
    described.push((
        mate_feet[ann.closure].1,
        ContactCarrier::SeamArc {
            center: tc + radial * tmaj,
            radius: tmin,
        },
    ));

    // Birth data. A host foot is the band's foot on the host support; a
    // mate foot is a split of that support's seam; the closure
    // crossing's mate piece SURVIVES as the band's own meridian, so it
    // is a birth row and not a death.
    for (ix, c) in ann.crossings.iter().enumerate() {
        rec.rim_feet.push((host_feet[ix].foot(), c.vertex));
    }
    for (ix, c) in ann.crossings.iter().enumerate() {
        rec.meridian_splits.push((mate_feet[ix].0, c.mate_seam));
    }
    for (ix, c) in ann.crossings.iter().enumerate() {
        rec.meridian_remnants.push((mate_feet[ix].2, c.mate_seam));
        // A hostless crossing leaves no host remnant: nothing of the
        // host was subdivided, so there is no fragment of a source
        // meridian to name. The STRUT is not one either — this call
        // minted it and the closure `kev` consumes it, so it reaches
        // neither the output nor the source and owes no row in either
        // direction.
        if let (HostAnchor::Seam { far_side, .. }, HostFoot::Seam(seam)) = (&host_feet[ix], &c.host)
        {
            rec.meridian_remnants.push((*far_side, *seam));
        }
    }
    for (i, l) in rim.chain.links().enumerate() {
        rec.rim_trims
            .push((host_trims[i].edge, l.edge, RimSide::Host));
        rec.rim_trims
            .push((mate_trims[i].edge, l.edge, RimSide::Mate));
    }
    rec.slits
        .push((mate_feet[ann.closure].1, closure.mate_seam));
    for l in rim.chain.links() {
        rec.dead.edges.push(l.edge);
    }
    for (ix, c) in ann.crossings.iter().enumerate() {
        // Only a SOURCE key can be retired: when the split handed the
        // rim-side piece the new edge, the source seam survives as the
        // far piece and nothing of it died. Under a seam refresh this
        // plan-key comparison and the live-key one COINCIDE today, and
        // structurally: `split_edge` keeps the parent key for the
        // `[t0, t]` child, a seam meridian's two ends are its wall's
        // two latitude rims, so a refreshed live key differs from the
        // plan's exactly when the earlier band sat at the seam's t0
        // end — which puts THIS band at the t1 end, where the dying
        // rim-side piece is always the FRESH key and neither spelling
        // fires. The plan-key spelling is kept because it states the
        // invariant directly (retire source keys only) instead of
        // deriving it from the split's retention direction, which
        // could change under it without a fixture noticing.
        //
        // A STRUT never appears here for the same reason it owes no
        // birth row: it is not a source key, so its death is not a
        // retirement of anything the caller handed in.
        if let (HostAnchor::Seam { rim_side, .. }, HostFoot::Seam(seam)) = (&host_feet[ix], &c.host)
            && *rim_side == *seam
        {
            rec.dead.edges.push(*seam);
        }
        if ix != ann.closure && mate_feet[ix].1 == c.mate_seam {
            rec.dead.edges.push(c.mate_seam);
        }
        rec.dead.vertices.push(c.vertex);
    }
    rec.bands
        .push((band_face, rim.chain.links().map(|l| l.edge).collect()));
    Ok((band_face, band_surface, described))
}

/// Whether `edge` has `v` as one of its endpoints.
pub(super) fn edge_touches<T: Decide>(body: &Body<T>, edge: EdgeKey, v: VertexKey) -> bool {
    halves_of(body, edge).is_some_and(|(hp, hm)| {
        body.get_half_edge(hp).map(|h| h.start) == Some(v)
            || body.get_half_edge(hm).map(|h| h.start) == Some(v)
    })
}

// ------------------------------------------------------------------
// Shared small lookups and the description pass.
// ------------------------------------------------------------------

pub(super) fn point_of<T: Decide>(body: &Body<T>, v: VertexKey) -> Option<Point3<T>> {
    body.get_point(body.get_vertex(v)?.point).copied()
}

pub(super) fn halves_of<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
) -> Option<(HalfEdgeKey, HalfEdgeKey)> {
    let ed = body.get_edge(e)?;
    Some((ed.he_plus, ed.he_minus))
}

pub(super) fn loop_of_half<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Option<LoopKey> {
    Some(body.get_half_edge(he)?.parent_loop)
}

pub(super) fn face_of_half<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Option<FaceKey> {
    Some(body.get_loop(loop_of_half(body, he)?)?.face)
}

/// The two faces an edge separates — `he_plus`'s, then `he_minus`'s.
/// Equal on a co-surface seam of a wall the charts did not split.
fn edge_faces<T: Decide>(body: &Body<T>, e: EdgeKey) -> Option<(FaceKey, FaceKey)> {
    let ed = body.get_edge(e)?;
    Some((
        face_of_half(body, ed.he_plus)?,
        face_of_half(body, ed.he_minus)?,
    ))
}

// ------------------------------------------------------------------
// The surgery's ONE face-destroying door. Every `kef` in this file and
// in `open/` is `kef_minted`; nothing else calls `Body::kef`
// (`tests/review_fillet_t_r1_probes.rs` is the mechanical pin).
// ------------------------------------------------------------------

/// **Every face of the body the surgery was handed**, read once before
/// the clone is touched. It is the door's argument, so there is no
/// second set to build and no wrong one to spell.
pub(super) struct SourceFaces(Vec<FaceKey>);

impl SourceFaces {
    /// The one constructor. Refuses a face-less body: an empty snapshot
    /// would make [`SourceFaces::kef_minted`] vacuous, which is the one
    /// way past that door that shows nowhere.
    ///
    /// # Errors
    ///
    /// [`BlendError::SurgeryInvariant`] on a body with no faces —
    /// unreachable once the entry gate has admitted a solid and a
    /// shell, which is why the row is the surgery's own and not the
    /// input's.
    fn of<T: Decide>(source: &Body<T>) -> Result<Self, BlendError> {
        let mut faces: Vec<FaceKey> = source.faces().map(|(k, _)| k).collect();
        if faces.is_empty() {
            return Err(invariant_broken(
                EntityId::Face(FaceKey::default()),
                "the blend surgery snapshotted a source body with no faces",
            ));
        }
        faces.sort_unstable();
        Ok(Self(faces))
    }

    /// **Kill the face of `dying`, unless it is a face of the source.**
    ///
    /// `kef` kills the face of the half it is given, and every face this
    /// surgery kills is one it MINTED. A support shrinks; it does not
    /// die (`super::naming::Retired` carries no face channel for exactly
    /// this reason). WHICH half dies stays the carve step's own pick —
    /// at most sites both sides are minted — so what this door adds is
    /// the refusal.
    ///
    /// # Errors
    ///
    /// [`BlendError::SurgeryInvariant`] when the half's face is one the
    /// surgery did not mint, or does not resolve;
    /// [`BlendError::Op`] tagged `site` when the operator refuses.
    pub(super) fn kef_minted<T: Decide>(
        &self,
        body: &mut Body<T>,
        dying: HalfEdgeKey,
        site: &'static str,
    ) -> Result<(), BlendError> {
        let f = face_of_half(body, dying).ok_or_else(|| {
            invariant_broken(
                EntityId::HalfEdge(dying),
                "a carve step's dying half does not resolve to a face",
            )
        })?;
        if self.0.binary_search(&f).is_ok() {
            return Err(invariant_broken(
                EntityId::Face(f),
                "a carve step was about to kill a face of the SOURCE body; this surgery \
                 kills only faces its own `mef`s minted",
            ));
        }
        body.kef(dying).map_err(|e| op(site, e))?;
        Ok(())
    }
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
/// (the rule is `DESIGN.md`'s prefer-intrinsic paragraph under D2).
fn attach_contact<T: Decide + Bounds>(
    body: &mut Body<T>,
    edge: EdgeKey,
    carrier: ContactCarrier<T>,
    tol: Tol,
) -> Result<(), BlendError> {
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
    let transverse = matches!(
        carrier,
        ContactCarrier::Chord | ContactCarrier::TransverseArc { .. }
    );
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
        // ONE construction for all three arc kinds, deliberately: a
        // corner arc, a cap's cut-off arc and a band's slit are the
        // same short arc about a stored centre (sweep < π, so the
        // `atan2` turn is unambiguous), and the only thing that differs
        // is the DESCRIPTION they take below. Byte-identical copies of
        // the geometry would let one drift from another with nothing
        // to say so.
        ContactCarrier::CornerArc { center, radius }
        | ContactCarrier::TransverseArc { center, radius }
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
        EdgeDescriptionSpec::seam(s1)
    } else if transverse {
        // The chamfer's edges and the ruled band's cut-off arcs: two
        // surfaces crossing at a definite angle, so the intrinsic
        // description is the plain intersection locus. Calling it a
        // TANGENT intersection would claim normal-parallelism along
        // the locus that the geometry does not have. The description
        // is chosen for what the geometry IS, not for what the
        // certifier would catch: a cut-off arc mis-described as a
        // tangent intersection of band and cap certifies and passes
        // tier 3 today (the `TangentParallel` margin `sin θ / |κ_rel|`
        // admits a 90° crossing —
        // `work/fillet/tangent-parallel-certifier-passes-a-transverse-arc.md`).
        let witness = curve.eval((t0 + t1) * T::from_f64(0.5));
        EdgeDescriptionSpec::Intersection { s1, s2, witness }
    } else {
        let witness = curve.eval((t0 + t1) * T::from_f64(0.5));
        EdgeDescriptionSpec::TangentIntersection { s1, s2, witness }
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

    use super::super::battery::Convexity;
    use super::super::build::fillet_edges;
    use super::{BlendError, rim_trim_circles};
    use crate::blend::arms::plane_sphere_blend;
    use crate::test_support::{L, R, cube};

    /// **The door's own one-solid, one-shell clause.**
    /// `blend_surgery` binds the solid and the shell out of the same
    /// step that counts them, so there is no second read to prove — but
    /// the refusal is still the only thing standing between a
    /// multi-solid body and a surgery that would carve it as if its
    /// first solid were the only one. Delete the clause and this row
    /// reds.
    ///
    /// **Its reach, stated:** one grafted body trips both halves of
    /// the gate at once, so this row does not separate them. Splitting
    /// them needs a one-solid, two-shell body — a closed void — and
    /// nothing in the tree builds one today.
    #[test]
    fn a_body_that_is_not_one_solid_and_one_shell_is_refused_at_the_door() {
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
        let err = fillet_edges(&dst, &edges, R, Tol::witness())
            .expect_err("a two-solid body is outside the in-place surgery's door");
        assert!(
            matches!(
                err.error,
                BlendError::UnsupportedBody {
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
            Convexity::Convex,
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
}
