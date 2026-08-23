//! **The validity-predicate battery** — C8's binding order, run over
//! the INPUTS before a single surface is minted.
//!
//! Each predicate below is a named Q1 trilean through the crate's
//! [`super::decide`] funnel, with a margin in METERS at a named lever
//! arm (never a raw dimensionless quantity — a sin or a determinant
//! only becomes classifiable once it is folded against a length the
//! user can reason about). `Positive` proceeds, `Zero` and `Negative`
//! refuse typed with the margin as payload, and an in-band or
//! poisoned margin escalates through [`super::FilletError::Escalated`]
//! carrying the SAME recourse sentence as the definite arm — the
//! two-tolerance shape (D4 ¶1 addendum), on every arm, including the
//! definite ones.
//!
//! # The ordering claim, stated so it can be attacked
//!
//! The claim this module makes is: **if [`run_battery`] returns
//! `Ok`, construction cannot fail for a geometric reason.** It is
//! kept honest structurally rather than by hope:
//!
//! - The battery resolves each link's analytic ARM first
//!   ([`super::blend`]) and refuses typed on any support pair the
//!   arms do not cover. So "the constructor met a case the battery
//!   did not consider" cannot happen: the battery enumerates the
//!   cases.
//! - The setbacks predicate 2 refuses on are returned BY THE ARM —
//!   the same function the constructor calls, not a first-order
//!   estimate of it. There is no second copy of the geometry to
//!   drift.
//! - The ring-torus condition (`s > r`) that would otherwise be an
//!   assertion inside the torus constructor is predicate 3's margin,
//!   and the constructor's `None` return for it is unreachable once
//!   predicate 3 has passed — a fact the fixture
//!   `spine_regularity_refuses_before_the_torus_is_minted` pins by
//!   showing the refusal arrives with no surface allocated.

use geom::Curve3;
use geom::Surface;
use geom_core::{
    Band, Bounds, Decide, Indeterminate, Margin, MarginDiag, Point3, Real, Sign, Vec3,
};
use topo::{Body, EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};

use super::blend::{
    BlendArm, EdgeBlend, Meridian, Ruling, chamfer_strip, plane_plane_blend, plane_sphere_blend,
};
use super::{BlendKind, CornerConfig, FilletError, FilletSite, RunOutPolicy, decide};

/// The number of interior samples the chain predicates take along
/// each link. Nine, matching the certification schedule's
/// `CERT_SAMPLES` — the battery and the certificate look at the same
/// places, on purpose.
pub const CHAIN_SAMPLES: u32 = 9;

/// Which way the material wedge turns along a chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Convexity {
    /// Material wedge < π: the rolling ball is INSIDE the material,
    /// the blend removes material, and the blend face's chart normal
    /// is already the outward one (sense `true`).
    Convex,
    /// Material wedge > π: the ball rolls in the void, the blend adds
    /// material, and the blend face's sense bit is `false`.
    Concave,
}

impl core::fmt::Display for Convexity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Convex => write!(f, "convex"),
            Self::Concave => write!(f, "concave"),
        }
    }
}

impl Convexity {
    /// The sense bit a blend face on this chain mints with — read off
    /// the STORED convexity verdict, never off a sampled normal
    /// (S10/S11).
    #[must_use]
    pub fn blend_sense(self) -> bool {
        matches!(self, Self::Convex)
    }
}

/// The request the battery judges: a body, the edges to blend, and
/// the constant rolling-ball radius.
#[derive(Clone, Debug)]
pub struct FilletRequest<'a, T: Real> {
    /// The body whose edges are to be blended.
    pub body: &'a Body<T>,
    /// The edges, in any order — the battery walks them into chains.
    pub edges: Vec<EdgeKey>,
    /// The band's size, meters: the constant rolling-ball radius under
    /// [`BlendKind::Fillet`], the equal setback under
    /// [`BlendKind::Chamfer`].
    pub radius: T,
}

/// One resolved link of a chain: its edge, its two supports (with
/// their outward normals already folded through the stored sense
/// bits), the analytic arm it takes, and the blend that arm derives.
#[derive(Clone, Debug)]
pub struct Link<T: Real> {
    /// The edge being blended.
    pub edge: EdgeKey,
    /// The face on the `he_plus` side.
    pub face_a: FaceKey,
    /// The face on the `he_minus` side.
    pub face_b: FaceKey,
    /// The `he_plus` half-edge — the traversal whose direction the
    /// convexity margin is signed against.
    pub he_plus: HalfEdgeKey,
    /// The edge's start vertex (`he_plus`'s start).
    pub start: VertexKey,
    /// The edge's end vertex.
    pub end: VertexKey,
    /// Which analytic arm the support pair takes.
    pub arm: BlendArm,
    /// The blend the arm derives — surface, spine curvature, and the
    /// two trimlines with their EXACT setbacks.
    pub blend: EdgeBlend<T>,
    /// The link's convexity verdict.
    pub convexity: Convexity,
    /// The folded lever arm used by this link's angular predicates.
    pub arm_len: T,
}

/// How a chain terminates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChainClosure {
    /// The chain closes on itself: predicate 4 is the G1 closure test
    /// at every junction, wrap-around included.
    Closed,
    /// The chain has two ends: predicate 4 is the termination
    /// classification, and predicate 6 judges each end's corner.
    Open {
        /// The first end.
        head: VertexKey,
        /// The last end.
        tail: VertexKey,
    },
}

/// One resolved chain.
///
/// **A chain has a first link.** [`walk_chains`] mints one from a seed
/// link and only ever grows it, so "no links" is not a state this type
/// can hold — which is why [`Chain::first`] hands one back without an
/// `Option`, and why nothing downstream carries an empty-chain
/// refusal. The two link fields are private for exactly that reason: a
/// public `Vec` would re-admit the state the walk cannot produce.
#[derive(Clone, Debug)]
pub struct Chain<T: Real> {
    /// The chain's first link, in walk order.
    first: Link<T>,
    /// The links after [`Chain::first`], in walk order.
    rest: Vec<Link<T>>,
    /// The vertices at which consecutive links meet — the junctions
    /// predicate 4 judges. One per adjacent pair, plus the
    /// wrap-around vertex on a closed chain.
    pub junctions: Vec<VertexKey>,
    /// How it terminates.
    pub closure: ChainClosure,
}

impl<T: Real> Chain<T> {
    /// Assemble a chain from its first link and the rest.
    ///
    /// The signature is the invariant: there is no way to spell a chain
    /// with no links, here or anywhere else.
    #[must_use]
    pub(crate) fn new(
        first: Link<T>,
        rest: Vec<Link<T>>,
        junctions: Vec<VertexKey>,
        closure: ChainClosure,
    ) -> Self {
        Self {
            first,
            rest,
            junctions,
            closure,
        }
    }

    /// The chain's first link in walk order — always present.
    pub fn first(&self) -> &Link<T> {
        &self.first
    }

    /// The links after [`Chain::first`], in walk order.
    pub fn rest(&self) -> &[Link<T>] {
        &self.rest
    }

    /// Every link, in walk order. Never empty.
    pub fn links(&self) -> impl Iterator<Item = &Link<T>> + Clone {
        core::iter::once(&self.first).chain(self.rest.iter())
    }

    /// How many links the chain has — at least one.
    ///
    /// Not `len`, so that no `is_empty` is owed: a constant `false`
    /// would be an accessor whose only effect is to suggest the
    /// question is open.
    pub fn link_count(&self) -> usize {
        1 + self.rest.len()
    }
}

/// The battery's verdict: every chain resolved, every predicate
/// definitely satisfied. Holding one of these is the licence to
/// construct.
#[derive(Clone, Debug)]
pub struct BatteryVerdict<T: Real> {
    /// The resolved chains.
    pub chains: Vec<Chain<T>>,
    /// The band size that was judged (radius, or chamfer setback).
    pub radius: T,
    /// Which band the request grafts — carried so the assembly reads
    /// it off the verdict instead of being told a second time.
    pub kind: BlendKind,
}

/// Escalate at a site (the shared shape, so the two-tolerance text
/// can never drift between predicates).
fn esc(site: FilletSite, source: Indeterminate) -> FilletError {
    FilletError::Escalated { site, source }
}

/// A face's outward normal at `p`: the chart normal folded through
/// the STORED sense bit (`Face::sense_sign`) — never a sampled or
/// re-derived orientation (S10 category A).
fn outward<T: Decide>(body: &Body<T>, face: FaceKey, p: Point3<T>) -> Option<Vec3<T>> {
    let f = body.get_face(face)?;
    let s = body.get_surface(f.surface)?;
    let g = geom_brep::implicit_gradient(s, p);
    Some(g.normalize() * f.sense_sign::<T>())
}

/// The face on a half-edge's side.
fn face_of<T: Real>(body: &Body<T>, he: HalfEdgeKey) -> Option<FaceKey> {
    let h = body.get_half_edge(he)?;
    Some(body.get_loop(h.parent_loop)?.face)
}

/// The sample parameters of a link, and its carrier.
fn carrier_of<T: Decide>(body: &Body<T>, edge: EdgeKey) -> Option<(Curve3<T>, T, T)> {
    let e = body.get_edge(edge)?;
    let c = body.get_curve_geom(e.curve)?.certified()?;
    let (t0, t1) = c.params();
    Some((c.carrier().clone(), t0, t1))
}

/// Sample `i` of the battery's per-link parameter schedule — the
/// [`CHAIN_SAMPLES`] places every chain predicate looks along
/// `[t0, t1]`, spelled once. The two ends are the interval bounds
/// EXACTLY (not `t0 + span·1` arithmetic, which can miss `t1` by an
/// ulp): the lever arm's reduction to the endpoint chord on straight
/// edges is bit-exact because sample 0 IS `t0` and the last sample
/// IS `t1`.
fn chain_sample_at<T: Decide>(t0: T, t1: T, i: u32) -> T {
    if i == 0 {
        t0
    } else if i == CHAIN_SAMPLES - 1 {
        t1
    } else {
        t0 + (t1 - t0) * T::from_f64(f64::from(i) / f64::from(CHAIN_SAMPLES - 1))
    }
}

/// The midpoint parameter of a link — the dihedral classifier's
/// sample, spelled once.
fn mid_param<T: Decide>(t0: T, t1: T) -> T {
    (t0 + t1) / T::from_f64(2.0)
}

/// The lever arm of a link's edge — the curvature-free straight
/// extent every angular predicate folds against: the **maximum
/// pairwise chord** over the battery's own per-link schedule
/// ([`chain_sample_at`], all [`CHAIN_SAMPLES`] samples) — the same
/// places the other chain predicates look, on purpose.
///
/// Every chord lower-bounds arc length, so the lever never
/// over-reports the edge's extent — a margin in meters folded
/// against it stays conservative. On a collinear carrier the
/// endpoint pair dominates every other pair and the schedule's ends
/// are `t0`/`t1` exactly, so a straight edge meters bit-identically
/// to its endpoint chord. On a CLOSED edge, where that endpoint
/// chord is structurally zero, the interior pairs meter the rim —
/// the schedule spans diametral pairs, so a full circular rim meters
/// its diameter — and the dihedral is judged at an honest lever
/// rather than a collapsed one.
fn extent_of<T: Decide>(carrier: &Curve3<T>, t0: T, t1: T) -> T {
    let pts: Vec<Point3<T>> = (0..CHAIN_SAMPLES)
        .map(|i| carrier.eval(chain_sample_at(t0, t1, i)))
        .collect();
    let mut best = T::zero();
    for (i, a) in pts.iter().enumerate() {
        for b in &pts[(i + 1)..] {
            best = best.max((*b - *a).norm());
        }
    }
    best
}

// ---------------------------------------------------------------
// Predicate 1 — radius vs curvature headroom.
// ---------------------------------------------------------------

/// **`fillet3_radius_headroom`** — is the rolling ball definitely
/// small enough for both supports' normal curvature along the link?
///
/// Margin: `(1 − r·κ_max)·r` in METERS, at lever arm `r`. `κ_max` is
/// the reciprocal of `geom_brep::curvature_lever_arm`, the same
/// curvature radius the dihedral classifier folds — a plane's is
/// unbounded, so a plane contributes the saturated margin `r` and
/// never limits. The quantity is C8's "r vs 1/κ_max of each support
/// along the edge": the ball must not curve harder than the surface
/// it rolls on, or the blend interferes with its own support (the
/// survey's local-interference case; the too-large-ball GLOBAL
/// interference is the same fact taken over the whole chain, which
/// is why the predicate is evaluated at every sample and not only at
/// the midpoint).
///
/// # Errors
///
/// [`FilletError::RadiusHeadroom`] on a definite `Zero`/`Negative`;
/// [`FilletError::Escalated`] in band or on poison;
/// [`FilletError::BodyNotIntact`] when the face or its stored surface
/// does not resolve.
pub fn radius_headroom<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    p: Point3<T>,
    radius: T,
    band: Band,
) -> Result<(), FilletError> {
    let Some(f) = body.get_face(face) else {
        return Err(FilletError::BodyNotIntact {
            at: EntityId::Face(face),
            detail: "a link's support face, for the curvature headroom predicate",
        });
    };
    let Some(s) = body.get_surface(f.surface) else {
        return Err(FilletError::BodyNotIntact {
            at: EntityId::Face(face),
            detail: "a support face's stored surface, for the curvature headroom predicate",
        });
    };
    let arm = geom_brep::curvature_lever_arm(s, p);
    // `(1 − r/arm)·r`, written so a plane's unbounded arm saturates
    // at `r` rather than dividing by an infinity.
    let margin = radius - radius.powi(2) / arm;
    match decide("fillet3_radius_headroom", Margin::of(margin), band)
        .map_err(|e| esc(FilletSite::Chain, e))?
    {
        Sign::Positive => Ok(()),
        _ => Err(FilletError::RadiusHeadroom {
            face,
            margin: margin.lo(),
            radius: radius.lo(),
        }),
    }
}

// ---------------------------------------------------------------
// Predicate 3 — spine regularity.
// ---------------------------------------------------------------

/// **`fillet3_spine_regularity`** — does the rolling ball's own
/// centre locus stay regular at this radius?
///
/// Margin: `(1 − r·κ_spine)·r` in METERS at lever arm `r`. The spine
/// is an OFFSET locus, and an offset of radius `r` folds exactly
/// where the locus it offsets curves at `1/r`; past that the blend
/// envelope self-intersects and the "surface" is not one. For a
/// straight spine `κ_spine = 0` and the margin saturates at `r`; for
/// the pip rims' circular spine of radius `s` it is `(1 − r/s)·r`,
/// which is the ring-torus condition `s > r` in meters — so the
/// torus constructor's degenerate case is REFUSED HERE, before the
/// surface exists.
///
/// Note this is a different curvature from predicate 1's: predicate 1
/// asks about the SUPPORTS' curvature (can the ball sit on them),
/// predicate 3 about the SPINE's (does sweeping the ball along its
/// own centre locus fold). C8 lists both because they are both real.
///
/// # Errors
///
/// [`FilletError::SpineIrregular`] / [`FilletError::Escalated`].
pub fn spine_regularity<T: Decide + Bounds>(
    spine_curvature: T,
    radius: T,
    band: Band,
) -> Result<(), FilletError> {
    let margin = radius - radius.powi(2) * spine_curvature;
    match decide("fillet3_spine_regularity", Margin::of(margin), band)
        .map_err(|e| esc(FilletSite::Chain, e))?
    {
        Sign::Positive => Ok(()),
        _ => Err(FilletError::SpineIrregular {
            margin: margin.lo(),
            radius: radius.lo(),
        }),
    }
}

// ---------------------------------------------------------------
// Predicate 5 — convexity-sign consistency.
// ---------------------------------------------------------------

/// **`fillet3_convexity_sign`** — the dihedral's convexity sign at
/// one sample of one link.
///
/// Margin: `((n_a × n_b)·τ̂)·arm` in METERS, `n_a`/`n_b` the two
/// supports' OUTWARD normals (stored sense folded in), `τ̂` the
/// `he_plus` traversal direction, `arm` the folded lever arm. The
/// quantity is orientation-well-defined: swapping the two faces also
/// reverses the traversal, and the triple product is invariant under
/// doing both.
///
/// `Positive` is convex, `Negative` concave, `Zero` is a dihedral
/// with no definite wedge side at this lever — refused as
/// [`FilletError::TangentialEdge`], of which genuine tangency is one
/// cause. C8 requires the sign to
/// be CONSTANT along the chain — a dihedral flipping mid-chain has no
/// constant-radius rolling-ball blend at all — so the caller escalates
/// on a flip rather than blending each run silently.
///
/// The fold is gated by `fillet3_chain_arm` exactly as the chain-G1
/// margin is: an angle at a collapsed arm is not a question, so a
/// non-positive arm escalates `Invalid` rather than classifying —
/// the same predicate at the LINK site instead of the joint.
///
/// # Errors
///
/// [`FilletError::Escalated`] in band or on poison. A definite sign is
/// returned; the caller judges consistency.
pub fn convexity_at<T: Decide + Bounds>(
    n_a: Vec3<T>,
    n_b: Vec3<T>,
    tau: Vec3<T>,
    arm: T,
    edge: EdgeKey,
    band: Band,
) -> Result<(Convexity, T), FilletError> {
    let site = FilletSite::Link { edge };
    match decide("fillet3_chain_arm", Margin::of(arm), band).map_err(|e| esc(site, e))? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => {
            return Err(esc(
                site,
                Indeterminate {
                    margin: MarginDiag::Invalid,
                    band,
                    predicate: Some("fillet3_chain_arm"),
                },
            ));
        }
    }
    let margin = Margin::levered(n_a.cross(n_b).dot(tau.normalize()), arm);
    let sign = decide("fillet3_convexity_sign", margin, band).map_err(|e| esc(site, e))?;
    match sign {
        Sign::Positive => Ok((Convexity::Convex, margin.value())),
        Sign::Negative => Ok((Convexity::Concave, margin.value())),
        // A decided Zero establishes that the dihedral has no
        // definite wedge side at this lever — `(n_a × n_b)·τ̂` folded
        // against the arm is coincident with zero. Genuine tangency
        // (the supports sharing a tangent plane) is one cause, not
        // the established fact. Its own situation and its own error —
        // it does not DISAGREE with the chain's convexity, none was
        // decided, and reporting it as a "flip" would hand the reader
        // a chain verdict that was never taken.
        Sign::Zero => Err(FilletError::TangentialEdge {
            edge,
            margin: margin.value().lo(),
        }),
    }
}

// ---------------------------------------------------------------
// Predicate 4 — chain G1 closure / termination.
// ---------------------------------------------------------------

/// **`fillet3_chain_g1`** — do two consecutive links meet
/// tangentially at their shared vertex?
///
/// Margin: `sin θ · arm` in METERS — the same shape the dihedral
/// classifier uses one dimension up, with `θ` the angle between the
/// two carriers' unit tangents at the junction and `arm` the smaller
/// of the two links' extents. It is gated by `fillet3_chain_arm`
/// exactly as the dihedral is: an angle at a collapsed arm is not a
/// question, so a non-positive arm escalates `Invalid` rather than
/// classifying.
///
/// A closed chain must be G1 at EVERY junction (including the
/// wrap-around) for a constant-radius spine to exist through it;
/// C8's edge-chain-smoothness predicate is exactly this.
///
/// # Errors
///
/// [`FilletError::ChainNotG1`] / [`FilletError::Escalated`].
pub fn chain_g1<T: Decide + Bounds>(
    tau_in: Vec3<T>,
    tau_out: Vec3<T>,
    arm: T,
    vertex: VertexKey,
    band: Band,
) -> Result<(), FilletError> {
    let site = FilletSite::Joint { vertex };
    match decide("fillet3_chain_arm", Margin::of(arm), band).map_err(|e| esc(site, e))? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => {
            return Err(esc(
                site,
                Indeterminate {
                    margin: MarginDiag::Invalid,
                    band,
                    predicate: Some("fillet3_chain_arm"),
                },
            ));
        }
    }
    let sin_theta = tau_in.normalize().cross(tau_out.normalize()).norm();
    let margin = Margin::levered(sin_theta, arm);
    match decide("fillet3_chain_g1", margin, band).map_err(|e| esc(site, e))? {
        // A POSITIVE margin is the failure here (a corner), and a
        // ZERO one the success (tangent continuity) — the inverted
        // polarity of a coincidence predicate, stated so no reader
        // has to infer it.
        Sign::Zero => Ok(()),
        _ => Err(FilletError::ChainNotG1 {
            vertex,
            margin: margin.value().lo(),
            arm: arm.lo(),
        }),
    }
}

// ---------------------------------------------------------------
// Predicate 6 — corner configuration.
// ---------------------------------------------------------------

/// **`fillet3_corner_independence`** — is a chain termination the ONE
/// corner configuration M5 ships (OQ6): a valence-three vertex whose
/// three incident edges are all convex and whose three support
/// normals are definitely independent?
///
/// Margin: `|det(n₁, n₂, n₃)|·r` in METERS at lever arm `r`. The
/// determinant is what makes the corner ball's centre a well-posed
/// solve (`c` is the unique point at distance `r` inside all three
/// planes); at a dependent trihedron the three distance conditions do
/// not determine a centre and no sphere octant exists.
///
/// Valence and convexity are COMBINATORIAL facts, so they are decided
/// before any margin and reported with their own
/// [`CornerConfig`] tag; each tag names the run-out policy that would
/// handle it ([`RunOutPolicy`]) and nothing more — zero constructor
/// surface, refusal-payload vocabulary only.
///
/// # Errors
///
/// [`FilletError::FilletCornerUnsupported`] with the tag and policy;
/// [`FilletError::Escalated`] on an in-band determinant.
pub fn corner_config<T: Decide + Bounds>(
    vertex: VertexKey,
    valence: usize,
    convex: usize,
    normals: [Vec3<T>; 3],
    radius: T,
    band: Band,
) -> Result<(), FilletError> {
    let refuse = |corner: CornerConfig| FilletError::FilletCornerUnsupported {
        vertex,
        corner,
        // Every out-of-scope corner is one a stop-at-vertex run-out
        // with a general corner patch would handle; feathering is the
        // alternative policy and is named on the mixed-convexity tag,
        // where a corner patch cannot help because the ball changes
        // sides.
        policy: match corner {
            CornerConfig::MixedConvexity { .. } => RunOutPolicy::RunOutFeather,
            _ => RunOutPolicy::RunOutStopAtVertex,
        },
    };
    if valence != 3 {
        return Err(refuse(CornerConfig::NEdgeVertex { valence }));
    }
    if convex != 3 {
        return Err(refuse(CornerConfig::MixedConvexity { convex }));
    }
    let det = normals[0].dot(normals[1].cross(normals[2]));
    let margin = Margin::levered(det.abs(), radius);
    match decide("fillet3_corner_independence", margin, band) {
        Ok(Sign::Positive) => Ok(()),
        Ok(_) => Err(refuse(CornerConfig::DependentNormals)),
        Err(source) => Err(esc(FilletSite::Joint { vertex }, source)),
    }
}

// ---------------------------------------------------------------
// Predicate 2 — face consumption.
// ---------------------------------------------------------------

/// **`fillet3_face_clearance`** — a conservative screen on whether
/// every support face survives the blend.
///
/// Margin: `gap − setback_here − setback_there` in METERS, where
/// `gap` is the Euclidean distance between two boundary features of
/// one support face and each `setback` is that feature's trimline
/// displacement — zero for a boundary the request does not blend.
/// The screen runs over every PAIR of boundary edges of every support
/// face, so a face with four blended edges (every planar face of the
/// die) is judged against all six pairs, not only against the nearest
/// non-blended neighbour. That is what stops the obvious hole: a
/// single-edge test happily passes `r < L` on a face that two opposite
/// blends at `r > L/2` erase between them.
///
/// # What this arm does NOT claim (fix pass F1)
///
/// It is a **screen**, and its name and its error say so. The two
/// setbacks are subtracted from ONE straight-line gap, which is exact
/// when the two boundary edges face each other (parallel, opposed
/// inward normals — the box, and every prism's opposite cap edges) and
/// CONSERVATIVE when they meet at an angle, because each blend then
/// eats along its own inward normal rather than along the gap. The
/// reviewer's witness is a unit hexagonal prism: this refuses from
/// `r = 0.5` although the cap survives to the apothem `0.866`.
///
/// The screen is kept in that shape deliberately. Its error is worded
/// as "cannot certify" rather than "consumes", so no false fact is
/// asserted as a definite verdict; and it is conservative in the ONE
/// direction the ordering claim depends on — it cannot pass a request
/// whose support face really is consumed. Tightening it needs the
/// inward-offset polygon's feasibility (a linear program over the
/// face's own boundary, not the same setback algebra), which is
/// recorded as a numbered deviation rather than guessed at here.
///
/// The setbacks come from [`super::blend`] — the same functions the
/// constructor calls.
///
/// # Errors
///
/// [`FilletError::FaceClearanceUncertified`] /
/// [`FilletError::Escalated`].
pub fn face_clearance<T: Decide + Bounds>(
    face: FaceKey,
    gap: T,
    setback_here: T,
    setback_there: T,
    band: Band,
) -> Result<(), FilletError> {
    let margin = gap - setback_here - setback_there;
    match decide("fillet3_face_clearance", Margin::of(margin), band)
        .map_err(|e| esc(FilletSite::Chain, e))?
    {
        Sign::Positive => Ok(()),
        _ => Err(FilletError::FaceClearanceUncertified {
            face,
            margin: margin.lo(),
            gap: gap.lo(),
        }),
    }
}

/// Resolve one link: supports, arm, blend, convexity. Refuses typed
/// on any support pair the analytic arms do not cover — naming the
/// canal-surface unit as the missing front door.
fn resolve_link<T: Decide + Bounds>(
    body: &Body<T>,
    edge: EdgeKey,
    radius: T,
    band: Band,
    kind: BlendKind,
) -> Result<Link<T>, FilletError> {
    let broken = || FilletError::ChainNotConnected { edge };
    let e = body.get_edge(edge).ok_or_else(broken)?;
    let (he_plus, he_minus) = (e.he_plus, e.he_minus);
    let face_a = face_of(body, he_plus).ok_or_else(broken)?;
    let face_b = face_of(body, he_minus).ok_or_else(broken)?;
    let start = body.get_half_edge(he_plus).ok_or_else(broken)?.start;
    let end = body.half_edge_end(he_plus).ok_or_else(broken)?;
    let (carrier, t0, t1) = carrier_of(body, edge).ok_or_else(broken)?;
    let extent = extent_of(&carrier, t0, t1);
    let mid = mid_param(t0, t1);
    let p = carrier.eval(mid);
    let tau = carrier.deriv(mid);
    let n_a = outward(body, face_a, p).ok_or_else(broken)?;
    let n_b = outward(body, face_b, p).ok_or_else(broken)?;
    // Predicate 5 first at the link level: the arm's side depends on
    // the convexity, so the sign is established before any geometry.
    let (convexity, _) = convexity_at(n_a, n_b, tau, extent, edge, band)?;
    let sa = body
        .get_surface(body.get_face(face_a).ok_or_else(broken)?.surface)
        .ok_or_else(broken)?
        .clone();
    let sb = body
        .get_surface(body.get_face(face_b).ok_or_else(broken)?.surface)
        .ok_or_else(broken)?
        .clone();
    // A face's stored sense bit read STRUCTURALLY, never re-derived
    // from a normal: for a sphere the chart normal is the outward
    // radial, so `sense` says on which side of that sphere the material
    // lies, which is which offset sphere the rolling ball's centre
    // rides (`plane_sphere_blend`).
    let sense = |f: FaceKey| body.get_face(f).map(|d| d.sense).ok_or_else(broken);
    let senses = (sense(face_a)?, sense(face_b)?);
    let (arm, blend) = classify_arm(
        &sa, n_a, &sb, n_b, senses, &carrier, p, tau, extent, radius, convexity, edge, kind, band,
    )?;
    Ok(Link {
        edge,
        face_a,
        face_b,
        he_plus,
        start,
        end,
        arm,
        blend,
        convexity,
        arm_len: extent,
    })
}

/// A plane's stored in-plane seam reference — the deterministic,
/// isometry-equivariant `u_ref` seed the rim blend's torus chart
/// inherits (never a coordinate-axis tie-break).
fn plane_u<T: Real>(s: &Surface<T>) -> Vec3<T> {
    match s {
        Surface::Plane { u_ref, .. } => *u_ref,
        _ => Vec3::new(T::zero(), T::zero(), T::zero()),
    }
}

/// The roster [`FilletError::SpineUnsupported`] advertises — every
/// [`BlendArm`] the fillet table carries, hand-formatted because the
/// payload is a `&'static str`. `arms::the_refusal_roster_names_every_arm`
/// checks it against [`BlendArm::name`], so an arm that grows without
/// its roster row goes red rather than shipping a stale refusal.
pub(super) const ARM_ROSTER: &str = "non-(plane–plane / plane–sphere / sphere–cone / cone–plane / \
     cone–cone / cylinder–cone / cylinder–sphere / cylinder–plane / cylinder–cylinder)";

/// The refusal a pair takes when its supports ARE an arm's kinds but do
/// not share the axis (or the ruling) that arm's spine is derived from.
pub(super) const NOT_COAXIAL: &str =
    "a curved support pair whose two supports do not share one axis of revolution (nor one \
     ruling); its spine is neither a line nor a circle";

/// **`fillet3_support_coaxiality`** — do a curved pair's two supports
/// really share the axis (or the ruling) its arm's spine is derived
/// from?
///
/// Margin: the configuration's **departure** from that hypothesis in
/// METERS, at the link's own lever arm — the rim radius for a coaxial
/// pair, the link extent for a ruled one. An angular misalignment
/// enters as `|n̂ × k̂|` times that arm; a sphere's centre enters as its
/// own distance off the axis. `Sign::Zero` is the hypothesis holding.
///
/// This is the one metric fact a curved arm needs and cannot read
/// structurally: the surface KINDS are matched on stored variants, the
/// nappe on a sign, the material side on a stored bit — but "these two
/// stored axes are the same axis" is a comparison of placed geometry,
/// and placement round-off makes exact equality the wrong question.
/// Deciding it here is what keeps a NON-coaxial pair — whose spine is
/// neither line nor circle, i.e. the canal family — from being minted
/// as an exact torus that is not one.
fn support_coaxiality<T: Decide + Bounds>(
    edge: EdgeKey,
    departure: T,
    band: Band,
    supports: &'static str,
) -> Result<(), FilletError> {
    match decide("fillet3_support_coaxiality", Margin::of(departure), band)
        .map_err(|e| esc(FilletSite::Chain, e))?
    {
        Sign::Zero => Ok(()),
        _ => Err(FilletError::SpineUnsupported { edge, supports }),
    }
}

/// The support-pair → analytic-arm table (C8's list, restricted to
/// the arms this unit implements). Anything else refuses typed.
///
/// The two plane-support rows keep their own closed forms; every curved
/// pair goes through the shared sheet reduction
/// ([`super::blend::Meridian`] / [`super::blend::Ruling`]), whose family
/// is chosen by the RIM CARRIER's own stored shape — a coaxial pair
/// meets in a circle, a ruled pair in a line.
///
/// The chamfer's table is one row wide and refuses everything else,
/// with the same shape and the same honesty: a curved support is a
/// real chamfer whose arm is not built (VERBS-ARMS' machinery), not a
/// geometry this kernel will approximate.
#[allow(clippy::too_many_arguments)]
fn classify_arm<T: Decide + Bounds>(
    sa: &Surface<T>,
    n_a: Vec3<T>,
    sb: &Surface<T>,
    n_b: Vec3<T>,
    // The two supports' stored sense bits, in `(sa, sb)` order.
    senses: (bool, bool),
    carrier: &Curve3<T>,
    p: Point3<T>,
    tau: Vec3<T>,
    extent: T,
    radius: T,
    convexity: Convexity,
    edge: EdgeKey,
    kind: BlendKind,
    band: Band,
) -> Result<(BlendArm, EdgeBlend<T>), FilletError> {
    let convex = matches!(convexity, Convexity::Convex);
    if matches!(kind, BlendKind::Chamfer) {
        return match (sa, sb) {
            (Surface::Plane { .. }, Surface::Plane { .. }) => Ok((
                BlendArm::PlanePlaneStrip,
                chamfer_strip(p, tau.normalize(), n_a, n_b, radius),
            )),
            _ => Err(FilletError::ChamferArmUnsupported {
                edge,
                supports: "non-(plane–plane)",
            }),
        };
    }
    match (sa, sb) {
        (Surface::Plane { .. }, Surface::Plane { .. }) => Ok((
            BlendArm::PlanePlaneCylinder,
            plane_plane_blend(p, tau.normalize(), n_a, n_b, radius, convex),
        )),
        (
            Surface::Plane { origin, .. },
            Surface::Sphere {
                center, radius: r, ..
            },
        ) => Ok((
            BlendArm::PlaneSphereTorus,
            plane_sphere_blend(*origin, n_a, plane_u(sa), *center, *r, radius, senses.1),
        )),
        (
            Surface::Sphere {
                center, radius: r, ..
            },
            Surface::Plane { origin, .. },
        ) => {
            let mut b =
                plane_sphere_blend(*origin, n_b, plane_u(sb), *center, *r, radius, senses.0);
            core::mem::swap(&mut b.trim_a, &mut b.trim_b);
            Ok((BlendArm::PlaneSphereTorus, b))
        }
        _ => curved_arm(sa, sb, senses, carrier, p, extent, radius, edge, band),
    }
}

/// Which curved arm a support pair takes in each family, by stored
/// surface KIND alone — the classification half of the table, keyed so
/// `trim_a` stays the FIRST support's trimline in every row (the pair
/// is reduced in the caller's own order, so nothing is swapped).
fn coaxial_arm<T: Real>(sa: &Surface<T>, sb: &Surface<T>) -> Option<BlendArm> {
    use Surface::{Cone, Cylinder, Plane, Sphere};
    match (sa, sb) {
        (Sphere { .. }, Cone { .. }) | (Cone { .. }, Sphere { .. }) => {
            Some(BlendArm::SphereConeTorus)
        }
        (Cone { .. }, Plane { .. }) | (Plane { .. }, Cone { .. }) => Some(BlendArm::ConePlaneTorus),
        (Cone { .. }, Cone { .. }) => Some(BlendArm::ConeConeTorus),
        (Cylinder { .. }, Cone { .. }) | (Cone { .. }, Cylinder { .. }) => {
            Some(BlendArm::CylinderConeTorus)
        }
        (Cylinder { .. }, Sphere { .. }) | (Sphere { .. }, Cylinder { .. }) => {
            Some(BlendArm::CylinderSphereTorus)
        }
        (Cylinder { .. }, Plane { .. }) | (Plane { .. }, Cylinder { .. }) => {
            Some(BlendArm::CylinderPlaneTorus)
        }
        _ => None,
    }
}

/// The ruled family's two rows, likewise.
fn ruling_arm<T: Real>(sa: &Surface<T>, sb: &Surface<T>) -> Option<BlendArm> {
    use Surface::{Cylinder, Plane};
    match (sa, sb) {
        (Cylinder { .. }, Cylinder { .. }) => Some(BlendArm::CylinderCylinderCylinder),
        (Cylinder { .. }, Plane { .. }) | (Plane { .. }, Cylinder { .. }) => {
            Some(BlendArm::CylinderPlaneCylinder)
        }
        _ => None,
    }
}

/// **The curved-support rows**, all of them: reduce both supports to
/// their traces in the pair's own sheet, decide the shared-axis
/// hypothesis, and mint the torus or the cylinder the crossing implies.
///
/// The family is read off the rim's stored carrier — a circle puts the
/// pair in a meridian, a line in a cross-section — so a
/// `(Cylinder, Plane)` pair takes the torus row when it meets in a
/// latitude circle and the cylinder row when it meets along a ruling,
/// with no orientation guessed anywhere.
#[allow(clippy::too_many_arguments)]
fn curved_arm<T: Decide + Bounds>(
    sa: &Surface<T>,
    sb: &Surface<T>,
    senses: (bool, bool),
    carrier: &Curve3<T>,
    p: Point3<T>,
    extent: T,
    radius: T,
    edge: EdgeKey,
    band: Band,
) -> Result<(BlendArm, EdgeBlend<T>), FilletError> {
    let unsupported = |supports| FilletError::SpineUnsupported { edge, supports };
    match *carrier {
        Curve3::Circle { center, axis, .. } => {
            let arm = coaxial_arm(sa, sb).ok_or_else(|| unsupported(ARM_ROSTER))?;
            let sheet = Meridian {
                origin: center,
                axis,
                rim: p,
            };
            let (Some((ta, da)), Some((tb, db))) =
                (sheet.trace(sa, senses.0), sheet.trace(sb, senses.1))
            else {
                return Err(unsupported(ARM_ROSTER));
            };
            support_coaxiality(edge, da.max(db), band, NOT_COAXIAL)?;
            Ok((arm, sheet.blend(ta, tb, radius)))
        }
        Curve3::Line { dir, .. } => {
            let arm = ruling_arm(sa, sb).ok_or_else(|| unsupported(ARM_ROSTER))?;
            let sheet = Ruling {
                tau: dir.normalize(),
                rim: p,
                lever: extent,
            };
            let (Some((ta, da)), Some((tb, db))) =
                (sheet.trace(sa, senses.0), sheet.trace(sb, senses.1))
            else {
                return Err(unsupported(ARM_ROSTER));
            };
            support_coaxiality(edge, da.max(db), band, NOT_COAXIAL)?;
            Ok((arm, sheet.blend(ta, tb, radius)))
        }
        _ => Err(unsupported(ARM_ROSTER)),
    }
}

/// Walk the requested links into maximal chains.
///
/// The rule is structural and it is the one that makes predicates 4
/// and 6 disjoint: at a vertex where **exactly two** requested links
/// meet, the chain CONTINUES and the vertex is a junction, judged by
/// predicate 4 (G1). At a vertex where any other number meet — one
/// (a free end) or three or more (a corner) — the chain TERMINATES
/// and the vertex is judged by predicate 6 (corner configuration).
///
/// That is why filleting all twelve edges of a box yields twelve
/// one-link OPEN chains terminating in eight trihedral corners
/// (three links meet at every box vertex), while filleting a pip rim
/// yields one CLOSED chain (two links meet at every rim vertex) —
/// with no geometric decision taken anywhere in the walk.
fn walk_chains<T: Decide>(links: Vec<Link<T>>) -> Vec<Chain<T>> {
    let mut inc: Vec<(VertexKey, Vec<usize>)> = Vec::new();
    let bump = |v: VertexKey, i: usize, inc: &mut Vec<(VertexKey, Vec<usize>)>| match inc
        .iter_mut()
        .find(|(k, _)| *k == v)
    {
        Some((_, xs)) => xs.push(i),
        None => inc.push((v, vec![i])),
    };
    for (i, l) in links.iter().enumerate() {
        bump(l.start, i, &mut inc);
        if l.end != l.start {
            bump(l.end, i, &mut inc);
        }
    }
    let junction = |v: VertexKey, inc: &[(VertexKey, Vec<usize>)]| -> Option<Vec<usize>> {
        inc.iter()
            .find(|(k, _)| *k == v)
            .filter(|(_, xs)| xs.len() == 2)
            .map(|(_, xs)| xs.clone())
    };
    let ends = |i: usize| (links[i].start, links[i].end);
    let mut used = vec![false; links.len()];
    let mut chains: Vec<Chain<T>> = Vec::new();
    for seed in 0..links.len() {
        if used[seed] {
            continue;
        }
        used[seed] = true;
        let (mut head, mut tail) = ends(seed);
        let mut order = vec![seed];
        let mut joints_back: Vec<VertexKey> = Vec::new();
        let mut joints_fwd: Vec<VertexKey> = Vec::new();
        let mut closed = head == tail;
        // Forward, then backward, through junction vertices only.
        for forward in [true, false] {
            loop {
                let at = if forward { tail } else { head };
                let Some(pair) = junction(at, &inc) else {
                    break;
                };
                let Some(&next) = pair.iter().find(|&&j| !used[j]) else {
                    // Both links at this junction are already in the
                    // run: the chain has closed on itself.
                    if pair.iter().all(|&j| used[j]) && order.len() > 1 {
                        closed = true;
                        if forward {
                            joints_fwd.push(at);
                        } else {
                            joints_back.push(at);
                        }
                    }
                    break;
                };
                used[next] = true;
                let (a, b) = ends(next);
                let other = if a == at { b } else { a };
                if forward {
                    joints_fwd.push(at);
                    order.push(next);
                    tail = other;
                } else {
                    joints_back.push(at);
                    order.insert(0, next);
                    head = other;
                }
                if head == tail {
                    closed = true;
                    break;
                }
            }
        }
        joints_back.reverse();
        let mut junctions = joints_back;
        junctions.extend(joints_fwd);
        let closure = if closed {
            ChainClosure::Closed
        } else {
            ChainClosure::Open { head, tail }
        };
        let mut walked = order.into_iter().map(|i| links[i].clone());
        let Some(first) = walked.next() else {
            // `order` was minted `vec![seed]` at the top of this
            // iteration and only pushed to or inserted into since.
            unreachable!(
                "chain walk: `order` is seeded with this iteration's `seed` link and \
                 never shrinks"
            )
        };
        chains.push(Chain::new(first, walked.collect(), junctions, closure));
    }
    chains
}

/// The vertex's incident edges (its orbit) — the valence predicate 6
/// classifies.
fn vertex_edges<T: Decide>(body: &Body<T>, vertex: VertexKey) -> Option<Vec<EdgeKey>> {
    let v = body.get_vertex(vertex)?;
    let he = v.emanating?;
    let orbit = body.vertex_orbit(he)?;
    let mut edges: Vec<EdgeKey> = orbit
        .iter()
        .filter_map(|h| body.get_half_edge(*h).map(|x| x.edge))
        .collect();
    edges.sort_unstable();
    edges.dedup();
    Some(edges)
}

/// **Run the battery** — C8's six predicates over the request's
/// inputs, in C8's order, before any construction.
///
/// # Errors
///
/// Any of [`FilletError`]'s predicate arms, or
/// [`FilletError::Escalated`] with the offending margin as payload.
pub fn run_battery<T: Decide + Bounds>(
    req: &FilletRequest<'_, T>,
    band: Band,
) -> Result<BatteryVerdict<T>, FilletError> {
    run_battery_for(req, band, BlendKind::Fillet)
}

/// **Run the battery for one band kind** — the same predicates in the
/// same order, over the predicates that are FACTS ABOUT THE REQUEST.
///
/// Two of C8's six are rolling-ball facts and a chamfer has no ball:
/// predicate 1 asks whether the ball is small enough for the supports'
/// normal curvature, and predicate 3 whether the ball's own centre
/// locus folds. A ruled strip has neither quantity, so a chamfer run
/// does not meter them — a vacuous predicate reaching the funnel would
/// be a saturated row in the K corpus asserting a check that was never
/// a question. The four that DO transfer (clearance, chain G1,
/// convexity sign, corner configuration) are metered under their
/// existing `fillet3_*` names: they measure the same quantities over
/// the same inputs, with the ball radius replaced by the setback, and
/// a second name for the same margin would split one corpus in two.
///
/// # Errors
///
/// Any of [`FilletError`]'s predicate arms, or
/// [`FilletError::Escalated`] with the offending margin as payload.
pub fn run_battery_for<T: Decide + Bounds>(
    req: &FilletRequest<'_, T>,
    band: Band,
    kind: BlendKind,
) -> Result<BatteryVerdict<T>, FilletError> {
    let body = req.body;
    let r = req.radius;
    let rolling_ball = matches!(kind, BlendKind::Fillet);
    // Resolve first: this is where the support pairs are enumerated,
    // so an out-of-scope pair refuses before any margin is taken.
    let mut links = Vec::with_capacity(req.edges.len());
    for edge in &req.edges {
        links.push(resolve_link(body, *edge, r, band, kind)?);
    }
    let chains = walk_chains(links);

    // --- 1. radius vs curvature headroom, at every sample of every
    // link, on BOTH supports. A ball fact: not metered for a chamfer.
    if rolling_ball {
        for chain in &chains {
            for link in chain.links() {
                let Some((carrier, t0, t1)) = carrier_of(body, link.edge) else {
                    return Err(FilletError::ChainNotConnected { edge: link.edge });
                };
                for i in 0..CHAIN_SAMPLES {
                    let p = carrier.eval(chain_sample_at(t0, t1, i));
                    radius_headroom(body, link.face_a, p, r, band)?;
                    radius_headroom(body, link.face_b, p, r, band)?;
                }
            }
        }
    }

    // --- 2. face clearance (the conservative screen — see
    // `face_clearance`), over every pair of boundary edges of every
    // support face the request touches. The setbacks are the ARM's, so
    // the chamfer's screen runs on the chamfer's own setbacks.
    consumption_sweep(body, &chains, band)?;

    // --- 3. spine regularity, per link. A ball fact: not metered for
    // a chamfer.
    if rolling_ball {
        for chain in &chains {
            for link in chain.links() {
                spine_regularity(link.blend.spine_curvature, r, band)?;
            }
        }
    }

    // --- 4. chain G1 closure (closed) / termination (open). The
    // junctions the walk recorded are exactly the vertices where two
    // requested links meet; every other chain end goes to predicate 6.
    for chain in &chains {
        let ring: Vec<&Link<T>> = chain.links().collect();
        for (i, v) in chain.junctions.iter().enumerate() {
            let a = ring[i % ring.len()];
            let b = ring[(i + 1) % ring.len()];
            let (Some((ca, ta0, ta1)), Some((cb, tb0, tb1))) =
                (carrier_of(body, a.edge), carrier_of(body, b.edge))
            else {
                return Err(FilletError::ChainNotConnected { edge: a.edge });
            };
            // Tangents taken at the junction END of each carrier, so
            // "not G1" means a genuine kink and not a parameterization
            // artefact: on each side pick the parameter whose point is
            // the junction vertex.
            let tv = body
                .get_vertex(*v)
                .and_then(|x| body.get_point(x.point))
                .copied();
            let pick = |c: &Curve3<T>, t0: T, t1: T| -> Vec3<T> {
                match tv {
                    Some(pt) => {
                        let d0 = (c.eval(t0) - pt).norm();
                        let d1 = (c.eval(t1) - pt).norm();
                        // `min` is a total lattice op: no comparison
                        // operator, no branch on a scalar the interval
                        // lane cannot answer.
                        if d0.min(d1).lo() == d0.lo() {
                            -c.deriv(t0)
                        } else {
                            c.deriv(t1)
                        }
                    }
                    None => c.deriv(t1),
                }
            };
            chain_g1(
                pick(&ca, ta0, ta1),
                pick(&cb, tb0, tb1),
                a.arm_len.min(b.arm_len),
                *v,
                band,
            )?;
        }
        // A SELF-CLOSED single link registers no junction: `walk_chains`
        // counts its one vertex once, so the loop above has nothing to
        // walk and the chain's own closure would go unmetered. The
        // wrap-around is still a junction of the spine — the link's
        // carrier arrives at its start vertex and leaves it again — so
        // it is metered here, on the one link's own carrier endpoints:
        // the tangent arriving at `t1` against the tangent leaving at
        // `t0`, under the SAME predicate as every other junction. It is
        // vacuously satisfied by a `Curve3::Circle` (the closed carrier
        // this kernel mints today), and is the live check the day a
        // closed NURBS carrier arrives with a kink at its seam.
        if matches!(chain.closure, ChainClosure::Closed) && chain.junctions.is_empty() {
            let l = chain.first();
            if l.start == l.end {
                let Some((c, t0, t1)) = carrier_of(body, l.edge) else {
                    return Err(FilletError::ChainNotConnected { edge: l.edge });
                };
                chain_g1(c.deriv(t1), c.deriv(t0), l.arm_len, l.start, band)?;
            }
        }
    }

    // --- 5. convexity-sign consistency along each chain (the
    // per-link sign was decided during resolution; here it must AGREE
    // across the chain, C8's escalate-on-flip).
    for chain in &chains {
        let first = chain.first().convexity;
        for link in chain.links() {
            if link.convexity != first {
                let p = {
                    let (c, t0, t1) = carrier_of(body, link.edge)
                        .ok_or(FilletError::ChainNotConnected { edge: link.edge })?;
                    c.eval(mid_param(t0, t1))
                };
                let n_a = outward(body, link.face_a, p);
                let n_b = outward(body, link.face_b, p);
                let margin = match (n_a, n_b) {
                    (Some(a), Some(b)) => a.cross(b).norm().lo(),
                    _ => f64::NAN,
                };
                return Err(FilletError::ConvexitySignFlip {
                    edge: link.edge,
                    margin,
                    chain: first,
                });
            }
        }
    }

    // --- 6. corner configuration at every OPEN chain's two ends.
    for chain in &chains {
        if let ChainClosure::Open { head, tail } = chain.closure {
            for v in [head, tail] {
                corner_at(body, v, r, band, kind)?;
            }
        }
    }

    Ok(BatteryVerdict {
        chains,
        radius: req.radius,
        kind,
    })
}

/// Predicate 6 at one termination vertex: gather valence, per-edge
/// convexity, and the three support normals, then classify.
fn corner_at<T: Decide + Bounds>(
    body: &Body<T>,
    vertex: VertexKey,
    radius: T,
    band: Band,
    kind: BlendKind,
) -> Result<(), FilletError> {
    let edges = vertex_edges(body, vertex).ok_or(FilletError::FilletCornerUnsupported {
        vertex,
        corner: CornerConfig::Indeterminate,
        policy: RunOutPolicy::RunOutStopAtVertex,
    })?;
    let valence = edges.len();
    if valence != 3 {
        return corner_config(
            vertex,
            valence,
            0,
            [Vec3::new(T::zero(), T::zero(), T::zero()); 3],
            radius,
            band,
        );
    }
    let mut convex = 0usize;
    let mut normals = [Vec3::new(T::zero(), T::zero(), T::zero()); 3];
    let mut faces: Vec<FaceKey> = Vec::new();
    for (i, e) in edges.iter().enumerate() {
        let link = resolve_link(body, *e, radius, band, kind);
        match link {
            Ok(l) => {
                if matches!(l.convexity, Convexity::Convex) {
                    convex += 1;
                }
                for f in [l.face_a, l.face_b] {
                    if !faces.contains(&f) {
                        faces.push(f);
                    }
                }
                let _ = i;
            }
            // An edge at the corner whose own supports are out of the
            // arms' scope makes the CORNER unclassifiable — reported
            // as the corner situation, not as that edge's.
            //
            // **The fold is deliberate and it is lossy** (fix pass
            // F6): a neighbour that REFUSED definitely and one that
            // ESCALATED in band both land on
            // `CornerConfig::Indeterminate`, so this arm does not
            // carry the two-tolerance shape the six predicates do.
            // The grounds: the user situation is the same either way
            // — "this corner's configuration could not be read" — and
            // the actionable recourse is the same sentence, so
            // splitting it would give two errors for one situation
            // (the inverse of the D4 ¶1 addendum's rule). What is
            // genuinely lost is the neighbour's own margin, which a
            // future corner taxonomy should carry as payload; it is
            // not lost SILENTLY, because the tag says the
            // configuration did not classify.
            Err(_) => {
                return Err(FilletError::FilletCornerUnsupported {
                    vertex,
                    corner: CornerConfig::Indeterminate,
                    policy: RunOutPolicy::RunOutStopAtVertex,
                });
            }
        }
    }
    let Some(p) = body
        .get_vertex(vertex)
        .and_then(|v| body.get_point(v.point))
    else {
        return Err(FilletError::FilletCornerUnsupported {
            vertex,
            corner: CornerConfig::Indeterminate,
            policy: RunOutPolicy::RunOutStopAtVertex,
        });
    };
    if faces.len() != 3 {
        return corner_config(vertex, faces.len(), convex, normals, radius, band);
    }
    for (i, f) in faces.iter().enumerate() {
        // A support whose outward normal does not resolve leaves a
        // ZERO normal, which drives the independence determinant to
        // zero and lands on `DependentNormals` — a refusal, never a
        // pass. Documented rather than silent (fix pass F6).
        normals[i] = outward(body, *f, *p).unwrap_or(Vec3::new(T::zero(), T::zero(), T::zero()));
    }
    corner_config(vertex, valence, convex, normals, radius, band)
}

/// Predicate 2's sweep: for each support face, every pair of its
/// boundary edges, with the blended ones carrying their setbacks.
fn consumption_sweep<T: Decide + Bounds>(
    body: &Body<T>,
    chains: &[Chain<T>],
    band: Band,
) -> Result<(), FilletError> {
    // Setback of each blended edge on each of its two support faces.
    let mut setback: Vec<(EdgeKey, FaceKey, T)> = Vec::new();
    let mut faces: Vec<FaceKey> = Vec::new();
    for chain in chains {
        for l in chain.links() {
            setback.push((l.edge, l.face_a, l.blend.trim_a.1));
            setback.push((l.edge, l.face_b, l.blend.trim_b.1));
            for f in [l.face_a, l.face_b] {
                if !faces.contains(&f) {
                    faces.push(f);
                }
            }
        }
    }
    let look = |e: EdgeKey, f: FaceKey| -> T {
        setback
            .iter()
            .find(|(ee, ff, _)| *ee == e && *ff == f)
            .map_or(T::zero(), |(_, _, s)| *s)
    };
    for face in faces {
        let Some(fa) = body.get_face(face) else {
            continue;
        };
        let mut loops = vec![fa.outer];
        loops.extend(fa.rings.iter().copied());
        let mut boundary: Vec<(EdgeKey, Vec<Point3<T>>)> = Vec::new();
        for lp in loops {
            let Some(topo::LoopBoundary::Cycle { first }) = body.get_loop(lp).map(|l| l.boundary)
            else {
                continue;
            };
            let Some(cycle) = body.loop_cycle(first) else {
                continue;
            };
            for he in cycle {
                let Some(h) = body.get_half_edge(he) else {
                    continue;
                };
                let Some((c, t0, t1)) = carrier_of(body, h.edge) else {
                    continue;
                };
                let pts = (0..CHAIN_SAMPLES)
                    .map(|i| c.eval(chain_sample_at(t0, t1, i)))
                    .collect();
                boundary.push((h.edge, pts));
            }
        }
        for i in 0..boundary.len() {
            for j in (i + 1)..boundary.len() {
                let (ei, pi) = (&boundary[i].0, &boundary[i].1);
                let (ej, pj) = (&boundary[j].0, &boundary[j].1);
                // Adjacent boundary edges TOUCH (gap 0 at the shared
                // vertex) — their setbacks are judged by the corner
                // and G1 predicates, not by this one, so the pair is
                // skipped exactly when the edges share a vertex.
                if shares_vertex(body, *ei, *ej) {
                    continue;
                }
                // The closest approach of the two sampled boundaries.
                // Seeded from the FIRST real pair, never from an
                // infinite sentinel: at the certified scalar an
                // infinity is the ill-formed interval (NaI), NaI
                // absorbs through `min`, and every clearance margin
                // downstream of it escalates `Invalid` — which is what
                // kept the whole fillet op out of the Interval lane
                // (and its `fillet3_*` family out of the K corpus)
                // until the gate caught it. There is always a first
                // pair here: both sample vectors are non-empty by
                // construction (`CHAIN_SAMPLES` ≥ 1).
                let mut pairs = pi
                    .iter()
                    .flat_map(|a| pj.iter().map(move |b| (*b - *a).norm()));
                let Some(first) = pairs.next() else {
                    continue;
                };
                let gap = pairs.fold(first, T::min);
                face_clearance(face, gap, look(*ei, face), look(*ej, face), band)?;
            }
        }
    }
    Ok(())
}

fn shares_vertex<T: Decide>(body: &Body<T>, a: EdgeKey, b: EdgeKey) -> bool {
    let ends = |e: EdgeKey| -> Option<(VertexKey, VertexKey)> {
        let edge = body.get_edge(e)?;
        let s = body.get_half_edge(edge.he_plus)?.start;
        let t = body.half_edge_end(edge.he_plus)?;
        Some((s, t))
    };
    match (ends(a), ends(b)) {
        (Some((a0, a1)), Some((b0, b1))) => a0 == b0 || a0 == b1 || a1 == b0 || a1 == b1,
        _ => false,
    }
}
