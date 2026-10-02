//! **The validity-predicate battery** — C8's binding order, run over
//! the INPUTS before a single surface is minted.
//!
//! Each predicate below is a named Q1 trilean through the crate's
//! [`super::decide`] funnel, with a margin in METERS at a named lever
//! arm (never a raw dimensionless quantity — a sin or a determinant
//! only becomes classifiable once it is folded against a length the
//! user can reason about). `Positive` proceeds, `Zero` and `Negative`
//! refuse typed with the margin as payload, and an in-band or
//! poisoned margin escalates through [`super::BlendError::Escalated`]
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
//!   ([`super::arms`]) and refuses typed on any support pair the
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
use topo::{Body, EdgeKey, EntityId, FaceKey, HalfEdgeKey, SurfaceKey, VertexKey};

use super::arms::{
    BlendArm, EdgeBlend, Meridian, Ruling, chamfer_strip, plane_plane_blend, plane_sphere_blend,
};
use super::build::fan_at;
use super::surgery::{CORNER_SUPPORT_NOT_PLANAR, not_intact, unbuilt_geometry};
use super::{
    BlendDecision, BlendError, BlendKind, BlendSite, ClassifiedMargin, CornerConfig, classify,
};

/// **Does this scalar hold nondegenerate brackets?** — which is the
/// same question as "which [`MarginDiag`] arm does its classifier
/// speak", asked without classifying anything.
///
/// `f64` and `Interval` present a thin reading identically (`lo ==
/// hi`) and spell it differently: `f64::sign_within` reports a reading
/// it cannot classify as [`MarginKind::Value`](geom_core::MarginKind::Value), `Interval`'s reports
/// one as [`MarginKind::Enclosure`](geom_core::MarginKind::Enclosure) even when the enclosure is a point
/// (`geom-core`'s interval suite pins the pair `Value(m)` /
/// `Enclosure { lo: m, hi: m }` for one margin at the two scalars). So
/// the shape cannot be read off the bracket, and the payload has to
/// know which kind of scalar it is on.
///
/// **Why this is arithmetic and not a trial classification.** The
/// obvious probe — classify a value the band cannot decide and read
/// the spelling off the `Indeterminate` — goes through
/// [`Decide::sign_within`], and at the probe scalar that path WRITES A
/// K-TELEMETRY SAMPLE against the ambient predicate name
/// (`k_stats::classify` sets the name, the probe scalar's
/// `sign_within` records under it). A payload constructor is not a
/// decision and must not appear in the K corpus: the first spelling of
/// this function put 1398 synthetic in-band samples into
/// `fillet3_convexity_sign`'s population, every one of them the band
/// midpoint it had just invented, and `k-lint`'s rule 1 failed the
/// run. Measuring the type instead of classifying a value keeps the
/// corpus a record of decisions the kernel actually made.
///
/// **The measurement.** One third is not a dyadic rational, so a
/// correctly-rounded enclosure of it is strictly wider than a point,
/// while an `f64` quotient is one number. Nothing else about the
/// constant matters, and no classifier, band or predicate name is
/// involved. The answer is a property of `T` alone — it is the same
/// for every value and every call — so the branch is on the TYPE, not
/// on any geometry, which is what evaluation code is forbidden to
/// branch on.
///
/// The bound is `Bounds` alone and deliberately: `Bounds: Real`, so
/// the constructor and the division come with it, and
/// `scripts/gates/no-extra-real-bounds.sh` forbids naming `Real`
/// again beside another bound.
fn holds_enclosures<T: Bounds>() -> Spelling {
    let third = T::from_f64(1.0) / T::from_f64(3.0);
    if third.lo() < third.hi() {
        Spelling::Enclosure
    } else {
        Spelling::Value
    }
}

/// Which [`MarginDiag`] arm a scalar's classifier speaks. A two-state
/// answer to a two-state question, so the question can be asked
/// without minting a `MarginDiag` that carries no reading yet.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Spelling {
    /// This scalar reports a reading as one number.
    Value,
    /// This scalar reports a reading as an enclosure, point or not.
    Enclosure,
}

/// A quantity a refusal reports, in the shape its own scalar's
/// classifier would have reported it.
///
/// Both ends are read, so an interval quantity arrives as the
/// enclosure it is rather than as one endpoint of it, and a thin
/// interval enclosure stays an enclosure — because that is what
/// `Interval::sign_within` calls it. The read is the M5 PR 12 seam's
/// own (a bracket read feeding an error payload, deciding nothing,
/// `Bounds`' delegation rule (a)); which VARIANT carries it is not
/// read off the bracket at all but asked of the scalar, by
/// [`holds_enclosures`].
pub(crate) fn measured<T: Bounds>(value: T) -> MarginDiag {
    let (lo, hi) = (value.lo(), value.hi());
    if lo.is_nan() || hi.is_nan() {
        return MarginDiag::INVALID;
    }
    match holds_enclosures::<T>() {
        Spelling::Value => MarginDiag::value(lo),
        Spelling::Enclosure => MarginDiag::enclosure(lo, hi),
    }
}

/// The reading a definite decision saw, as the payload shape that says
/// what it is — the one place this lane turns a classified `T` margin
/// into a refusal payload.
///
/// The reading is [`measured`]'s; the predicate, band and sign are the
/// decision's. Poison is reported rather than asserted away: it is
/// unreachable behind a definite `Sign` (the classifier escalates
/// poison instead of deciding it), and a payload that cannot be
/// printed is worse than one that prints an impossibility.
pub(crate) fn classified<T: Bounds>(
    decision: BlendDecision,
    margin: T,
    band: Band,
    sign: Sign,
) -> ClassifiedMargin {
    ClassifiedMargin {
        predicate: decision.predicate(),
        reading: measured(margin),
        band,
        sign,
    }
}

/// The number of samples, ends included, the chain predicates take
/// along each link: the certification schedule's, so the battery and
/// the certificate look at the same places.
pub const CHAIN_SAMPLES: u32 = geom_brep::CERT_SAMPLES;

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

    /// **The ONE sign fold, homed.** `+radius` on a convex chain,
    /// `−radius` on a concave one: the displacement of a rest ball's
    /// FOOT from the ball's centre along its support's outward normal —
    /// into the material on a convex chain, where the ball is inside the
    /// material, out of it on a concave one, where the ball rolls in the
    /// void. Every arm that displaces by `±r` spells its side through
    /// this — [`super::arms::plane_plane_blend`]'s feet,
    /// [`super::arms::plane_sphere_blend`]'s spine depth,
    /// `open::planar::corner_plan`'s `toward` — and
    /// [`super::arms::corner_ball`] alone needs the rest DEPTH, which is
    /// this value's NEGATIVE by definition of tangency and is spelled
    /// there as `-signed(..)`, the one negation the fold keeps.
    #[must_use]
    pub fn signed<T: Real>(self, radius: T) -> T {
        sided(self.blend_sense(), radius)
    }

    /// **The same fold as a SIDE.** A support's stored sense bit says
    /// which side of its chart its material is on; the ball rests on
    /// that side exactly when the chain is convex, on the far side when
    /// it is concave — `sense == blend_sense()`, the identity on a
    /// convex chain. The shared sheet reduction hands each trace this
    /// bit (`curved_arm`), and the plane–sphere arm reads it against the
    /// sphere's sense to pick the offset sphere (the ball centre is
    /// INSIDE the sphere exactly when it rests on the sphere's material
    /// side).
    #[must_use]
    pub fn ball_side(self, sense: bool) -> bool {
        sense == self.blend_sense()
    }
}

/// **The conditional negation every `R ∓ r` selector spells**: `x`
/// where `side` holds, `−x` where it does not — exact in every
/// backend. [`Convexity::signed`] is this on the chain's verdict, and
/// the sheet arms spell it on the ball side [`Convexity::ball_side`]
/// derives from that verdict, so the one home is beside the bit's
/// provenance rather than inside either consumer.
pub(super) fn sided<T: Real>(side: bool, x: T) -> T {
    if side { x } else { -x }
}

/// The request the battery judges: a body, the edges to blend, and
/// the constant rolling-ball radius.
#[derive(Clone, Debug)]
pub struct BlendRequest<'a, T: Real> {
    /// The body whose edges are to be blended.
    pub body: &'a Body<T>,
    /// The edges, in any order — the battery walks them into chains.
    pub edges: Vec<EdgeKey>,
    /// The band's size, meters: the constant rolling-ball radius under
    /// [`BlendKind::Fillet`], the equal setback under
    /// [`BlendKind::Chamfer`].
    pub size: T,
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
    /// The dihedral margin that verdict was decided from, kept whole.
    /// The chain's sign-consistency check refuses on a link whose
    /// verdict disagrees with the chain's, and the number that refusal
    /// owes its reader is THIS one — the reading the classifier
    /// actually judged — not a fresh quantity sampled at the refusal.
    pub convexity_margin: ClassifiedMargin,
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

/// A junction of a chain: the vertex at which two consecutive links
/// meet, with the two links that meet there.
///
/// The links are carried BY POSITION in [`Chain::links`]' order, read
/// off the walk that found them incident to the vertex — so the check
/// that judges the junction is handed the two carriers that actually
/// arrive there, and no reader reconstructs the pair from where the
/// junction sits in a list. On a closed chain the wrap-around junction
/// pairs the last link with the first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Junction {
    /// The vertex the two links meet at.
    pub vertex: VertexKey,
    /// The link arriving at the vertex, as a position in walk order.
    /// Crate-private with [`Junction::arriving`] as its reader: the
    /// positions index [`Chain`]'s private links, so a consumer can
    /// read a junction's pair but not spell one the chain does not have.
    pub(crate) arriving: usize,
    /// The link leaving it: the position after `arriving` in walk
    /// order, or `0` for a closed chain's wrap-around. Read through
    /// [`Junction::leaving`], for the reason `arriving` is private.
    pub(crate) leaving: usize,
}

impl Junction {
    /// The link arriving at the vertex, as a position in
    /// [`Chain::links`]' order.
    #[must_use]
    pub fn arriving(&self) -> usize {
        self.arriving
    }

    /// The link leaving the vertex, as a position in
    /// [`Chain::links`]' order.
    #[must_use]
    pub fn leaving(&self) -> usize {
        self.leaving
    }
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
    /// predicate 4 judges — each with the two links that meet there.
    /// One per adjacent pair, plus the wrap-around on a closed chain,
    /// in walk order: `junctions[i]` sits between links `i` and
    /// `i + 1`. The check reads each junction's OWN pair and not this
    /// order; the order is kept so the record is the same chain from
    /// any seed, which is what a row that names "the first junction"
    /// of an open chain reads (`junctions[0]`) whichever link seeded
    /// the walk. The closed-chain pairing rows compare sets and read
    /// no order.
    pub junctions: Vec<Junction>,
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
        junctions: Vec<Junction>,
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
    /// The band size that was judged (the fillet's radius, or the
    /// chamfer's setback).
    pub size: T,
    /// Which band the request grafts — carried so the assembly reads
    /// it off the verdict instead of being told a second time.
    pub kind: BlendKind,
    /// The open-chain ends predicate 6 classified
    /// [`CornerConfig::TransverseCap`] — every end of every RULED link,
    /// sorted and deduplicated. The ruled band's plan
    /// (`open::ruled::RuledPlan`) reads a link's
    /// two ends off this list rather than re-deciding the cap; an end
    /// missing from it is a verdict the body disagrees with. The
    /// uniform trihedra the corner path carves are not listed: that
    /// configuration has no tag of its own ([`corner_at`]).
    pub transverse_caps: Vec<VertexKey>,
}

/// A junction arm `fillet3_chain_arm` decided non-positive: an angle at
/// so short an arm is not a question, so it refuses at `site` as that
/// decision, carrying the arm it read — the band-decided sibling of an
/// in-band arm, with the same ending (D4 ¶1 (iv)).
fn short_arm<T: Bounds>(site: BlendSite, arm: T, band: Band) -> BlendError {
    let decision = BlendDecision::ChainArm;
    BlendError::Escalated {
        site,
        decision,
        source: Indeterminate {
            margin: measured(arm),
            band,
            predicate: Some(decision.predicate()),
            terminal_sliver: false,
        },
    }
}

/// A face's outward normal at `p`: the implicit gradient folded
/// through the STORED sense bit (`Face::sense`) at its one home,
/// [`geom_brep::implicit_outward_normal`] — never a sampled or
/// re-derived orientation (S10 category A). Unwrapped here because
/// both consumers read it as geometry (a dot, a mean).
fn outward<T: Decide>(body: &Body<T>, face: FaceKey, p: Point3<T>) -> Option<Vec3<T>> {
    let f = body.get_face(face)?;
    let s = body.get_surface(f.surface)?;
    Some(geom_brep::implicit_outward_normal(s, f.sense, p).vec())
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
/// `[t0, t1]`, on the kernel's one uniform schedule
/// ([`geom_brep::schedule_param`]). Its ends are the interval bounds
/// exactly, so the lever arm's reduction to the endpoint chord on
/// straight edges is bit-exact.
fn chain_sample_at<T: Decide>(t0: T, t1: T, i: u32) -> T {
    geom_brep::schedule_param(t0, t1, i, CHAIN_SAMPLES)
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
/// [`BlendError::RadiusHeadroom`] on a definite `Zero`/`Negative`;
/// [`BlendError::Escalated`] in band or on poison;
/// [`BlendError::BodyNotIntact`] when the face or its stored surface
/// does not resolve.
pub fn radius_headroom<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    p: Point3<T>,
    radius: T,
    band: Band,
) -> Result<(), BlendError> {
    let Some(f) = body.get_face(face) else {
        return Err(BlendError::BodyNotIntact {
            at: EntityId::Face(face),
            detail: "a link's support face, for the curvature headroom predicate",
        });
    };
    let Some(s) = body.get_surface(f.surface) else {
        return Err(BlendError::BodyNotIntact {
            at: EntityId::Face(face),
            detail: "a support face's stored surface, for the curvature headroom predicate",
        });
    };
    // The ball must fit inside the TIGHTEST bend, so the arm is the
    // smallest radius of curvature, not the chart's scale (they differ
    // on a fat torus, and a horn or spindle one has no bound at all).
    let arm = geom_brep::min_radius_of_curvature(s, p);
    // `(1 − r/arm)·r`, written so a plane's unbounded arm saturates
    // at `r` rather than dividing by an infinity.
    let margin = radius - radius.powi(2) / arm;
    match classify(
        BlendSite::Chain,
        BlendDecision::RadiusHeadroom,
        Margin::of(margin),
        band,
    )? {
        Sign::Positive => Ok(()),
        sign => Err(BlendError::RadiusHeadroom {
            face,
            margin: classified(BlendDecision::RadiusHeadroom, margin, band, sign),
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
/// [`BlendError::SpineIrregular`] / [`BlendError::Escalated`].
pub fn spine_regularity<T: Decide + Bounds>(
    spine_curvature: T,
    radius: T,
    band: Band,
) -> Result<(), BlendError> {
    let margin = radius - radius.powi(2) * spine_curvature;
    match classify(
        BlendSite::Chain,
        BlendDecision::SpineRegularity,
        Margin::of(margin),
        band,
    )? {
        Sign::Positive => Ok(()),
        sign => Err(BlendError::SpineIrregular {
            margin: classified(BlendDecision::SpineRegularity, margin, band, sign),
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
/// [`BlendError::TangentialEdge`], of which genuine tangency is one
/// cause. C8 requires the sign to
/// be CONSTANT along the chain — a dihedral flipping mid-chain has no
/// constant-radius rolling-ball blend at all — so the caller escalates
/// on a flip rather than blending each run silently.
///
/// The fold is gated by `fillet3_chain_arm` exactly as the chain-G1
/// margin is: an angle at an arm not definitely positive is not a
/// question, so such an arm refuses as that gate, carrying the arm it
/// read (`short_arm`), rather than classifying — the same predicate at
/// the LINK site instead of the joint.
///
/// # Errors
///
/// [`BlendError::Escalated`] in band or on poison. A definite sign is
/// returned; the caller judges consistency.
pub fn convexity_at<T: Decide + Bounds>(
    n_a: Vec3<T>,
    n_b: Vec3<T>,
    tau: Vec3<T>,
    arm: T,
    edge: EdgeKey,
    band: Band,
) -> Result<(Convexity, ClassifiedMargin), BlendError> {
    let site = BlendSite::Link { edge };
    match classify(site, BlendDecision::ChainArm, Margin::of(arm), band)? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => {
            return Err(short_arm(site, arm, band));
        }
    }
    let margin = Margin::levered(n_a.cross(n_b).dot(tau.normalize()), arm);
    let sign = classify(site, BlendDecision::ConvexitySign, margin, band)?;
    let reading = |s| classified(BlendDecision::ConvexitySign, margin.value(), band, s);
    match sign {
        Sign::Positive => Ok((Convexity::Convex, reading(Sign::Positive))),
        Sign::Negative => Ok((Convexity::Concave, reading(Sign::Negative))),
        // A decided Zero establishes that the dihedral has no
        // definite wedge side at this lever — `(n_a × n_b)·τ̂` folded
        // against the arm is coincident with zero. Genuine tangency
        // (the supports sharing a tangent plane) is one cause, not
        // the established fact. Its own situation and its own error —
        // it does not DISAGREE with the chain's convexity, none was
        // decided, and reporting it as a "flip" would hand the reader
        // a chain verdict that was never taken.
        Sign::Zero => Err(BlendError::TangentialEdge {
            edge,
            margin: reading(Sign::Zero),
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
/// exactly as the dihedral is: an angle at an arm not definitely
/// positive is not a question, so such an arm refuses as that gate,
/// carrying the arm it read (`short_arm`), rather than classifying.
///
/// A closed chain must be G1 at EVERY junction (including the
/// wrap-around) for a constant-radius spine to exist through it;
/// C8's edge-chain-smoothness predicate is exactly this.
///
/// # Errors
///
/// [`BlendError::ChainNotG1`] / [`BlendError::Escalated`].
pub fn chain_g1<T: Decide + Bounds>(
    tau_in: Vec3<T>,
    tau_out: Vec3<T>,
    arm: T,
    vertex: VertexKey,
    band: Band,
) -> Result<(), BlendError> {
    let site = BlendSite::Joint { vertex };
    match classify(site, BlendDecision::ChainArm, Margin::of(arm), band)? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => {
            return Err(short_arm(site, arm, band));
        }
    }
    let sin_theta = tau_in.normalize().cross(tau_out.normalize()).norm();
    let margin = Margin::levered(sin_theta, arm);
    match classify(site, BlendDecision::ChainG1, margin, band)? {
        // A POSITIVE margin is the failure here (a corner), and a
        // ZERO one the success (tangent continuity) — the inverted
        // polarity of a coincidence predicate, stated so no reader
        // has to infer it.
        Sign::Zero => Ok(()),
        sign => Err(BlendError::ChainNotG1 {
            vertex,
            margin: classified(BlendDecision::ChainG1, margin.value(), band, sign),
            arm: measured(arm),
        }),
    }
}

// ---------------------------------------------------------------
// Predicate 6 — corner configuration.
// ---------------------------------------------------------------

/// **`fillet3_corner_independence`** — is a chain termination a corner
/// configuration some band builds (OQ6): a valence-three vertex whose
/// three incident edges carry ONE convexity, either side, and whose
/// three support normals are definitely independent?
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
/// handle it ([`CornerConfig::policy`]) — or says that none would —
/// and nothing more: zero constructor surface, refusal-payload
/// vocabulary only.
///
/// # What this predicate does NOT decide
///
/// It reads the CORNER and nothing about the request's verb. A
/// UNIFORM trihedron — three edges of one convexity — is one
/// configuration whichever side of the material it is on, and this
/// predicate admits it on both, for both verbs: the chamfer's ruled
/// strip and flat patch take no convexity argument, and the rolling
/// ball's corner — its ball, its contact feet, its octant chart —
/// folds the side as one verdict. A MIXED corner is out of scope for
/// every band there is, because the band would change sides
/// mid-corner.
///
/// # Errors
///
/// [`BlendError::UnsupportedCorner`] with the tag and policy;
/// [`BlendError::Escalated`] on an in-band determinant.
pub fn corner_config<T: Decide + Bounds>(
    vertex: VertexKey,
    valence: usize,
    convex: usize,
    normals: [Vec3<T>; 3],
    radius: T,
    band: Band,
) -> Result<(), BlendError> {
    // Which run-out policy (if any) an out-of-scope configuration names
    // is the TAG's own fact, so it is read from the tag rather than
    // decided again here (`CornerConfig::policy`).
    let refuse = |corner: CornerConfig| super::surgery::unbuilt_corner_config(vertex, corner);
    if valence != 3 {
        return Err(refuse(CornerConfig::NEdgeVertex { valence }));
    }
    // A uniform trihedron is a configuration some band carves; a mixed
    // one is a configuration none does. Which of the two uniform sides
    // the RUNNING band carves is decided by the caller that knows it.
    if !matches!(convex, 0 | 3) {
        return Err(refuse(CornerConfig::MixedConvexity { convex }));
    }
    let det = normals[0].dot(normals[1].cross(normals[2]));
    let margin = Margin::levered(det.abs(), radius);
    match classify(
        BlendSite::Joint { vertex },
        BlendDecision::CornerIndependence,
        margin,
        band,
    )? {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => Err(refuse(CornerConfig::DependentNormals)),
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
/// reviewer's witness is a unit hexagonal prism: this builds up to
/// `r = 0.499` and refuses from `r = 0.51` against a gap of exactly
/// `1.0`, the hexagon's SIDE
/// (`m5_pr12_fix_pass::f1_the_clearance_screen_is_conservative_by_direction_on_the_hexagon`).
/// That the cap survives to the apothem `0.866` is the witness's
/// premise, derived from the hexagon and asserted by no row.
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
/// The setbacks come from [`super::arms`] — the same functions the
/// constructor calls.
///
/// # Errors
///
/// [`BlendError::FaceClearanceUncertified`] /
/// [`BlendError::Escalated`].
///
/// `cross_chain` says whether the two setbacks belong to two DIFFERENT
/// requested chains — the caller knows, this screen does not — and
/// rides into the refusal, whose recourse then names the SPLIT that
/// re-meters each chain against the face the previous carve actually
/// left (#935's boundary).
pub fn face_clearance<T: Decide + Bounds>(
    face: FaceKey,
    gap: T,
    setback_here: T,
    setback_there: T,
    cross_chain: bool,
    band: Band,
) -> Result<(), BlendError> {
    let margin = gap - setback_here - setback_there;
    match classify(
        BlendSite::Chain,
        BlendDecision::FaceClearance,
        Margin::of(margin),
        band,
    )? {
        Sign::Positive => Ok(()),
        sign => Err(BlendError::FaceClearanceUncertified {
            face,
            margin: classified(BlendDecision::FaceClearance, margin, band, sign),
            gap: measured(gap),
            cross_chain,
        }),
    }
}

/// Resolve one link: supports, arm, blend, convexity. Refuses typed
/// on any support pair the analytic arms do not cover — naming the
/// canal-surface unit as the missing front door.
pub(crate) fn resolve_link<T: Decide + Bounds>(
    body: &Body<T>,
    edge: EdgeKey,
    radius: T,
    band: Band,
    kind: BlendKind,
) -> Result<Link<T>, BlendError> {
    let broken = || BlendError::ChainNotConnected { edge };
    let sides = topo::readback::edge_sides(body, edge).map_err(|_| broken())?;
    let he_plus = sides.plus.half_edge;
    let (face_a, face_b) = sides.faces();
    let start = body.get_half_edge(he_plus).ok_or_else(broken)?.start;
    let end = body.half_edge_end(he_plus).ok_or_else(broken)?;
    let (carrier, t0, t1) = carrier_of(body, edge).ok_or_else(broken)?;
    let extent = extent_of(&carrier, t0, t1);
    let mid = geom::mid_param(t0, t1);
    let (p, tau) = carrier.ders1(mid);
    let n_a = outward(body, face_a, p).ok_or_else(broken)?;
    let n_b = outward(body, face_b, p).ok_or_else(broken)?;
    // Predicate 5 first at the link level: the arm's side depends on
    // the convexity, so the sign is established before any geometry.
    let (convexity, convexity_margin) = convexity_at(n_a, n_b, tau, extent, edge, band)?;
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
        convexity_margin,
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

/// The roster [`BlendError::SpineUnsupported`] advertises — every
/// [`BlendArm`] the fillet table carries, hand-formatted because the
/// payload is a `&'static str`. `arms::the_refusal_roster_names_every_arm`
/// checks it against [`BlendArm::name`], so an arm that grows without
/// its roster row goes red rather than shipping a stale refusal.
pub(super) const ARM_ROSTER: &str = "non-(plane–plane / plane–sphere / sphere–cone / sphere–sphere / cone–plane / \
     cone–cone / cylinder–cone / cylinder–sphere / cylinder–plane / cylinder–plane(∥) / \
     cylinder–cylinder)";

/// The roster as data, for the arm-coverage row that keeps it honest.
#[must_use]
pub fn arm_roster() -> &'static str {
    ARM_ROSTER
}

/// The refusal a pair takes when its supports ARE an arm's kinds but do
/// not share the axis (or the ruling) that arm's spine is derived from.
pub(super) const NOT_COAXIAL: &str =
    "a curved support pair whose supports do not share one axis of revolution or one ruling";

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
///
/// On a body whose edge carriers are certified the margin is Zero by
/// implication and not by luck: a circle lying wholly on a sphere has
/// its own axis through that sphere's centre, and a circle lying wholly
/// on a coaxial cylinder or cone is a latitude of it, so the carrier
/// already witnesses the shared axis. This predicate is what makes that
/// implication a CHECKED premise rather than an unstated chain through
/// somebody else's certificate.
fn support_coaxiality<T: Decide + Bounds>(
    edge: EdgeKey,
    departure: T,
    band: Band,
    supports: &'static str,
) -> Result<(), BlendError> {
    match classify(
        BlendSite::Chain,
        BlendDecision::SupportCoaxiality,
        Margin::of(departure),
        band,
    )? {
        Sign::Zero => Ok(()),
        _ => Err(BlendError::SpineUnsupported { edge, supports }),
    }
}

/// The support-pair → analytic-arm table (C8's list, restricted to
/// the arms this unit implements). Anything else refuses typed.
///
/// The two plane-support rows keep their own closed forms; every curved
/// pair goes through the shared sheet reduction
/// ([`super::arms::Meridian`] / [`super::arms::Ruling`]), whose family
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
) -> Result<(BlendArm, EdgeBlend<T>), BlendError> {
    if matches!(kind, BlendKind::Chamfer) {
        return match (sa, sb) {
            (Surface::Plane { .. }, Surface::Plane { .. }) => Ok((
                BlendArm::PlanePlaneStrip,
                chamfer_strip(p, tau.normalize(), n_a, n_b, radius),
            )),
            _ => Err(BlendError::ChamferArmUnsupported {
                edge,
                supports: "non-(plane–plane)",
            }),
        };
    }
    match (sa, sb) {
        (Surface::Plane { .. }, Surface::Plane { .. }) => Ok((
            BlendArm::PlanePlaneCylinder,
            plane_plane_blend(p, tau.normalize(), n_a, n_b, radius, convexity),
        )),
        (
            Surface::Plane { origin, .. },
            Surface::Sphere {
                center, radius: r, ..
            },
        ) => Ok((
            BlendArm::PlaneSphereTorus,
            plane_sphere_blend(
                *origin,
                n_a,
                plane_u(sa),
                *center,
                *r,
                radius,
                senses.1,
                convexity,
            ),
        )),
        (
            Surface::Sphere {
                center, radius: r, ..
            },
            Surface::Plane { origin, .. },
        ) => {
            let mut b = plane_sphere_blend(
                *origin,
                n_b,
                plane_u(sb),
                *center,
                *r,
                radius,
                senses.0,
                convexity,
            );
            core::mem::swap(&mut b.trim_a, &mut b.trim_b);
            Ok((BlendArm::PlaneSphereTorus, b))
        }
        _ => curved_arm(
            sa, sb, senses, convexity, carrier, p, extent, radius, edge, band,
        ),
    }
}

/// Which curved arm a support pair takes in each family, by stored
/// surface KIND alone — the classification half of the table, keyed so
/// `trim_a` stays the FIRST support's trimline in every row (the pair
/// is reduced in the caller's own order, so nothing is swapped).
fn coaxial_arm<T: Real>(sa: &Surface<T>, sb: &Surface<T>) -> Option<BlendArm> {
    use Surface::{Cone, Cylinder, Plane, Sphere};
    match (sa, sb) {
        // Two spheres on distinct centres ALWAYS meet in a circle whose
        // axis is the line through those centres, so this row's coaxiality
        // hypothesis is free — the only arm in the table with no
        // configuration condition on its supports.
        (Sphere { .. }, Sphere { .. }) => Some(BlendArm::SphereSphereTorus),
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
///
/// **The side each trace rests the ball on folds the chain's stored
/// convexity verdict** (S10/S11): a support's stored sense bit says
/// which side of its chart its material is on, and the ball sits on
/// that side exactly when the chain is CONVEX — on a concave chain it
/// rolls in the void, the far side of every support — so the side
/// handed to the trace is [`Convexity::ball_side`] of that bit, the
/// identity on a convex chain. ONE fold with ONE home, `Convexity`:
/// `signed` for the arms that displace by `±r` ([`plane_plane_blend`],
/// [`super::arms::plane_sphere_blend`], `open::planar::corner_plan`),
/// `ball_side` for the ones that pick a side, and
/// [`super::arms::corner_ball`]'s rest depth its one negation. The
/// `Ruling` row folds it too, on both sides: the convex side carves the
/// rod with a flat milled along it, the CONCAVE side carves a rod's
/// section standing on a block's top edge (the sunk rod, built through
/// the extrude door —
/// `review_fillet_h7_r1_probes::a_sunk_rod_has_concave_ruled_creases_that_add_material`
/// pins its material-adding band at `ΔV = +2·A·L`). The boolean
/// cannot build either concave fixture (two parallel cylinders unioned
/// refuse at the curved-pierce door; a block ∪ cylinder at the join
/// lane), which is the boolean's ground, not this fold's.
#[allow(clippy::too_many_arguments)]
fn curved_arm<T: Decide + Bounds>(
    sa: &Surface<T>,
    sb: &Surface<T>,
    senses: (bool, bool),
    convexity: Convexity,
    carrier: &Curve3<T>,
    p: Point3<T>,
    extent: T,
    radius: T,
    edge: EdgeKey,
    band: Band,
) -> Result<(BlendArm, EdgeBlend<T>), BlendError> {
    let unsupported = |supports| BlendError::SpineUnsupported { edge, supports };
    match *carrier {
        Curve3::Circle { center, axis, .. } => {
            let arm = coaxial_arm(sa, sb).ok_or_else(|| unsupported(ARM_ROSTER))?;
            let sheet = Meridian {
                origin: center,
                axis,
                rim: p,
            };
            let (Some((ta, da)), Some((tb, db))) = (
                sheet.trace(sa, convexity.ball_side(senses.0)),
                sheet.trace(sb, convexity.ball_side(senses.1)),
            ) else {
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
            let (Some((ta, da)), Some((tb, db))) = (
                sheet.trace(sa, convexity.ball_side(senses.0)),
                sheet.trace(sb, convexity.ball_side(senses.1)),
            ) else {
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
pub(crate) fn walk_chains<T: Decide>(links: Vec<Link<T>>) -> Vec<Chain<T>> {
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
        // The run is held as its SEED plus the two directions it grew
        // in, rather than as one `Vec` that happens never to be empty.
        // Non-emptiness is then the shape — there is always a seed —
        // and "the walk produced no links" is a state this loop does
        // not spell.
        let mut before: Vec<usize> = Vec::new();
        let mut after: Vec<usize> = Vec::new();
        // The run's two end links, by input index: the link whose far
        // end is `head` and the one whose far end is `tail`.
        let (mut head_link, mut tail_link) = (seed, seed);
        // Every junction met, as `(vertex, arriving, leaving)` in INPUT
        // indices — the two links the walk found incident there, the
        // arriving one earlier in walk order. Remapped to chain
        // positions once the run's order is fixed, below.
        let mut joints: Vec<(VertexKey, usize, usize)> = Vec::new();
        let mut closed = head == tail;
        // Forward, then backward, through junction vertices only.
        for forward in [true, false] {
            loop {
                let at = if forward { tail } else { head };
                let Some(pair) = junction(at, &inc) else {
                    break;
                };
                let own = if forward { tail_link } else { head_link };
                let Some(&next) = pair.iter().find(|&&j| !used[j]) else {
                    // Both links at this junction are already in the
                    // run: the chain has closed on itself, and the
                    // junction is between the run's own end link and
                    // the other link at this vertex.
                    let grew = !before.is_empty() || !after.is_empty();
                    if pair.iter().all(|&j| used[j]) && grew {
                        closed = true;
                        // The junction holds exactly two links and one
                        // of them is the run's own end link; a pair
                        // that does not contain `own` is a walk that
                        // arrived here along a link the vertex does
                        // not carry, and that is loud rather than a
                        // pairing taken on the wrong link.
                        let other = match pair[..] {
                            [a, b] if a == own => b,
                            [a, b] if b == own => a,
                            _ => unreachable!(
                                "chain walk: the run arrived at this junction along its own \
                                 end link, so that link is one of the two incident here"
                            ),
                        };
                        joints.push(if forward {
                            (at, own, other)
                        } else {
                            (at, other, own)
                        });
                    }
                    break;
                };
                used[next] = true;
                let (a, b) = ends(next);
                let other = if a == at { b } else { a };
                if forward {
                    joints.push((at, own, next));
                    after.push(next);
                    tail = other;
                    tail_link = next;
                } else {
                    joints.push((at, next, own));
                    before.push(next);
                    head = other;
                    head_link = next;
                }
                if head == tail {
                    closed = true;
                    break;
                }
            }
        }
        let closure = if closed {
            ChainClosure::Closed
        } else {
            ChainClosure::Open { head, tail }
        };
        // Head-first order: the backward run reversed, then the seed,
        // then the forward run. Both arms are ordinary — a run that
        // never extended backwards heads at its own seed.
        before.reverse();
        let (first, rest) = match before.split_first() {
            Some((&far, between)) => {
                let mut rest: Vec<usize> = between.to_vec();
                rest.push(seed);
                rest.extend(after);
                (far, rest)
            }
            None => (seed, after),
        };
        // A junction's links as POSITIONS in the chain's walk order,
        // which is what the check indexes. Every link a junction names
        // was pushed into this run by the step that recorded it.
        let position = |i: usize| -> usize {
            if i == first {
                return 0;
            }
            match rest.iter().position(|&j| j == i) {
                Some(p) => p + 1,
                None => unreachable!(
                    "chain walk: a junction names a link the run it was recorded in \
                     does not carry"
                ),
            }
        };
        let mut junctions: Vec<Junction> = joints
            .into_iter()
            .map(|(vertex, arriving, leaving)| Junction {
                vertex,
                arriving: position(arriving),
                leaving: position(leaving),
            })
            .collect();
        // Walk order along the chain, so `junctions[i]` sits between
        // links `i` and `i + 1` — the wrap-around, recorded last by the
        // backward pass, sorts to the end by its arriving link. Nothing
        // in the battery reads the order (`Chain::junctions`' doc says
        // who does).
        junctions.sort_by_key(|j| j.arriving);
        chains.push(Chain::new(
            links[first].clone(),
            rest.into_iter().map(|i| links[i].clone()).collect(),
            junctions,
            closure,
        ));
    }
    chains
}

/// **Run the battery** — C8's six predicates over the request's
/// inputs, in C8's order, before any construction.
///
/// # Errors
///
/// Any of [`BlendError`]'s predicate arms, or
/// [`BlendError::Escalated`] with the offending margin as payload.
pub fn run_battery<T: Decide + Bounds>(
    req: &BlendRequest<'_, T>,
    band: Band,
) -> Result<BatteryVerdict<T>, BlendError> {
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
/// Any of [`BlendError`]'s predicate arms, or
/// [`BlendError::Escalated`] with the offending margin as payload.
pub fn run_battery_for<T: Decide + Bounds>(
    req: &BlendRequest<'_, T>,
    band: Band,
    kind: BlendKind,
) -> Result<BatteryVerdict<T>, BlendError> {
    let body = req.body;
    let r = req.size;
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
                    return Err(BlendError::ChainNotConnected { edge: link.edge });
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
        for j in &chain.junctions {
            let v = &j.vertex;
            // The junction's two links are the ones the walk found
            // incident to it; a record that names any other link is a
            // walk defect, and this tripwire makes it loud in every
            // build that keeps debug assertions rather than a verdict
            // taken on a far-end tangent. The pin is the suites' rows,
            // which read the record and the carve, not this line.
            let (a, b) = (ring[j.arriving], ring[j.leaving]);
            debug_assert!(
                [a, b].iter().all(|l| l.start == *v || l.end == *v),
                "a junction's two links both touch it: {j:?}"
            );
            let (Some((ca, ta0, ta1)), Some((cb, tb0, tb1))) =
                (carrier_of(body, a.edge), carrier_of(body, b.edge))
            else {
                return Err(BlendError::ChainNotConnected { edge: a.edge });
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
                    return Err(BlendError::ChainNotConnected { edge: l.edge });
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
                // The number this refusal owes its reader is the
                // margin whose SIGN is the disagreement, and the link
                // already carries it: `fillet3_convexity_sign` decided
                // it, at this link's own lever, when the link
                // resolved. Re-deriving one here read the supports'
                // normals a second time and reported `‖n_a × n_b‖` —
                // unsigned, unlevered, and therefore not the quantity
                // the variant documents — with `NaN` standing in for
                // "the normals would not resolve", a structural
                // absence in a measurement's costume. Both go: the
                // decision is the record.
                return Err(BlendError::ConvexitySignFlip {
                    edge: link.edge,
                    margin: link.convexity_margin,
                    chain: first,
                });
            }
        }
    }

    // --- 6. corner configuration at every OPEN chain's two ends. Each
    // end is judged beside the link that reaches it: a ruled link's
    // ends are transverse caps, a planar link's are corners.
    let mut transverse_caps = Vec::new();
    for chain in &chains {
        if let ChainClosure::Open { head, tail } = chain.closure {
            let last = chain.rest().last().unwrap_or(chain.first());
            for (v, link) in [(head, chain.first()), (tail, last)] {
                if let Some(CornerConfig::TransverseCap) = corner_at(body, v, link, r, band, kind)?
                {
                    transverse_caps.push(v);
                }
            }
        }
    }
    transverse_caps.sort_unstable();
    transverse_caps.dedup();

    Ok(BatteryVerdict {
        chains,
        size: req.size,
        kind,
        transverse_caps,
    })
}

/// The unordered pair of SURFACES an edge's two supports carry — the
/// stored arena keys, so a co-surface seam is recognized by identity
/// rather than by comparing two placed surfaces for equality.
fn edge_surfaces<T: Decide>(body: &Body<T>, edge: EdgeKey) -> Option<(SurfaceKey, SurfaceKey)> {
    let (a, b) = topo::readback::edge_sides(body, edge).ok()?.surfaces();
    Some(if a <= b { (a, b) } else { (b, a) })
}

/// **A chart-seam vertex, recognized structurally** — the point where a
/// CLOSED rim was cut by the chart seams of its own two supports.
///
/// Its shape, and the whole of it: two incident edges carrying ONE
/// support pair between them, i.e. the same rim arriving and leaving,
/// and beside them one or two CO-SURFACE seams (one surface on both
/// sides, so the dihedral there is zero by construction and not by
/// measurement — the same structural reading S10/S11 require of every
/// sense question). Two where both supports are periodic walls cut at
/// their seams; one where a support is a whole face carrying both arcs —
/// a full revolve's plane disc or annulus, which has no seam to cut it.
/// The one-seam reading also needs `topo::query::rim_of` to list the
/// rim through this vertex: an open run of cocircular arcs swept beside
/// a whole face (one arc of a D's rim) has the same orbit at its
/// station, and there the recourse's "request the rim whole, `rim_of`
/// lists it" would be false — so the reading asks that door itself:
/// `rim_lists(seed, arcs)` is the caller's `rim_of` read, true iff the
/// rim it lists from `seed` holds every one of `arcs` (passed in, since
/// that door wants `Bounds` and this classifier is generic over
/// [`Decide`] alone).
///
/// The two families must be the SAME geometry, not merely the right
/// counts: each seam's surface has to be one of the rim's own two
/// supports. That is what makes this the rim's own charts cut — a
/// co-surface edge on some unrelated third surface is somebody else's
/// seam passing through, and the vertex it makes is not this one.
///
/// The surface is smooth through such a point: nothing about the
/// geometry changes across a seam, only which chart names it. So it is
/// not a corner, no run-out policy addresses it, and the door that does
/// is the closed-rim one — which is what [`CornerConfig::SeamVertex`]
/// says.
///
/// # One rule, three readings, and which is the weakest
///
/// The same incidence is spelled at three sites, deliberately not
/// shared, because each answers a different question and two of them
/// run against a body the third has already mutated:
///
/// - **here** — a REFUSAL classifier over a chain end's edge orbit. It
///   reads incidence and nothing else: no convexity, no arm, no
///   support-face resolution (the one-seam arm adds `rim_of`'s answer,
///   itself a read of stored incidence and carrier bits). It is the WEAKEST of the three, and that
///   is load-bearing rather than incidental — a tag that fired only
///   where the carve succeeds could not name a door in its recourse at
///   all, and the price is that the recourse must be TRUE ON BOTH
///   material sides, since this predicate is blind to convexity — which
///   `FILLET3_SEAM_VERTEX_RECOURSE` is: the closed-rim band carves a
///   concave rim through the same walk as a convex one;
/// - `surgery::resolve_seam_split_rim` — the multi-arc ADMISSION,
///   which adds everything this one omits (one support pair for the
///   whole rim, ring-free half-band supports each carrying one arc, the
///   arcs walking one cycle) and so is strictly stronger;
/// - `surgery::resolve_annulus` / `wall_seam` — the ONE-EDGE admission,
///   whose set-equality on the rim vertex's orbit is the same shape
///   read against a single self-closed edge and its doubly-traversed
///   wall seams.
///
/// The intended relation is: this one ADMITS every site the other two
/// do, and more. Anything that narrows it must narrow the recourse with
/// it; anything that widens the other two must not silently assume this
/// one already screened it.
fn is_seam_vertex<T: Decide>(
    body: &Body<T>,
    edges: &[EdgeKey],
    rim_lists: impl FnOnce(EdgeKey, &[EdgeKey]) -> bool,
) -> bool {
    let mut seams: Vec<SurfaceKey> = Vec::new();
    let mut rim: Vec<(EdgeKey, (SurfaceKey, SurfaceKey))> = Vec::new();
    for e in edges {
        let Some((a, b)) = edge_surfaces(body, *e) else {
            return false;
        };
        if a == b {
            seams.push(a);
        } else {
            rim.push((*e, (a, b)));
        }
    }
    let [(arrive, (p, q)), (_, second)] = rim[..] else {
        return false;
    };
    if (p, q) != second || !seams.iter().all(|s| *s == p || *s == q) {
        return false;
    }
    match seams.len() {
        2 => true,
        // One seam: a whole face carries the rim on one side, so
        // nothing about this vertex alone says the rim is CLOSED — one
        // arc of an open cocircular run (a D's quarter arcs, swept) has
        // the same orbit, and so do arcs that close on shared vertices
        // but sit on circles `rim_of` does not read as one (the same
        // point set stored on bits of its own per arc). The recourse
        // promises `rim_of` lists the rim, so the reading is THAT
        // door's answer: both rim arcs here are in the rim it lists.
        1 => rim_lists(arrive, &rim.iter().map(|(e, _)| *e).collect::<Vec<_>>()),
        _ => false,
    }
}

/// The refusal for a ruled link's end that is not a transverse cap —
/// an oblique cap, or a curved end face. Both are run-outs the
/// mid-curve taxonomy reserves, not corner configurations, so they
/// carry the run-out vocabulary and the corner recourse's "general
/// run-outs" clause.
pub const RULED_END_NOT_TRANSVERSE: &str =
    "a ruled band's edge ends at a face that is not a plane perpendicular to its ruling";

/// **`fillet3_cap_transverse`** — does a ruled link's end face lie
/// perpendicular to the band's ruling, so the band can be cut off in
/// the cap's own section of it?
///
/// Margin: the cap normal's **departure** from the ruling, `|n̂ × τ̂|`
/// in METERS at the link's own lever arm — [`Link::arm_len`], the
/// extent [`super::arms::Ruling::lever`] already meters the
/// shared-ruling hypothesis at, so the two decisions about one link's
/// ruling are levered alike. `Sign::Zero` is the cap being transverse;
/// a definite departure is the oblique cap, refused as the run-out it
/// is; an in-band reading escalates with the same recourse
/// (two-tolerance, D4 ¶1 addendum). Nothing here reads a sampled
/// normal: the cap plane's normal is the stored surface's, the ruling
/// the arm's own cylinder axis.
///
/// **Both directions are normalised HERE**, so the margin is the sine
/// of the angle between them whatever a caller passes: the stored
/// plane normal and cylinder axis are unit vectors already (the
/// normalisation is exact on them and moves no bit of any carve), and
/// a pin that hands in a non-unit direction still meters the angle it
/// meant.
///
/// # Errors
///
/// [`BlendError::UnsupportedRunOut`] on a definite departure;
/// [`BlendError::Escalated`] on an in-band one.
pub fn cap_transverse<T: Decide + Bounds>(
    vertex: VertexKey,
    cap_normal: Vec3<T>,
    ruling: Vec3<T>,
    lever: T,
    band: Band,
) -> Result<(), BlendError> {
    let margin = Margin::levered(
        cap_normal.normalize().cross(ruling.normalize()).norm(),
        lever,
    );
    match classify(
        BlendSite::Joint { vertex },
        BlendDecision::CapTransverse,
        margin,
        band,
    )? {
        Sign::Zero => Ok(()),
        _ => Err(super::surgery::unbuilt_run_out(
            EntityId::Vertex(vertex),
            RULED_END_NOT_TRANSVERSE,
        )),
    }
}

/// **The cap incidence at a ruled link's end**, read structurally — the
/// ONE home of the rule the battery classifies by and the surgery
/// carves by: at a trivalent vertex the two edges other than the
/// crease each join one of the link's supports (`face_a`, `face_b`) to
/// a third face, and that third face — one face, shared by both — is
/// the cap. Returned as `(rim on face_a, rim on face_b, cap)`.
///
/// `None` where the incidence does not have that shape. **On a
/// manifold body that is unreachable at valence three**: the three
/// face-corners around a trivalent vertex are its three faces, so its
/// edges separate `A|B`, `B|C`, `C|A` — one crease and two rims each
/// joining one support to the same third face, by construction. The
/// `None` arms (an edge on both supports, on neither, or two distinct
/// third faces) can be reached only by a non-manifold vertex or a
/// stale key, which is why the battery reports them as an
/// unclassifiable end and the surgery as a body that does not hold
/// together — neither is a shape a fixture can build.
pub(super) fn cap_incidence<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    crease: EdgeKey,
    face_a: FaceKey,
    face_b: FaceKey,
) -> Option<(EdgeKey, EdgeKey, FaceKey)> {
    let incident = fan_at(body.edges_of_vertex(vertex))?;
    let [_, _, _] = incident[..] else {
        return None;
    };
    let mut rim_a: Option<(EdgeKey, FaceKey)> = None;
    let mut rim_b: Option<(EdgeKey, FaceKey)> = None;
    for e in incident.into_iter().filter(|e| *e != crease) {
        let (f1, f2) = topo::readback::edge_sides(body, e).ok()?.faces();
        let on = |f: FaceKey| f == face_a || f == face_b;
        let (support, third) = match (on(f1), on(f2)) {
            (true, false) => (f1, f2),
            (false, true) => (f2, f1),
            _ => return None,
        };
        let slot = if support == face_a {
            &mut rim_a
        } else {
            &mut rim_b
        };
        if slot.replace((e, third)).is_some() {
            return None;
        }
    }
    let (Some((rim_a, cap)), Some((rim_b, cap_b))) = (rim_a, rim_b) else {
        return None;
    };
    (cap == cap_b).then_some((rim_a, rim_b, cap))
}

/// Predicate 6 at one termination vertex, beside the link that reaches
/// it, returning the CARVED configuration it classified: a RULED
/// link's end must be a transverse cap — decided by [`cap_transverse`]
/// and returned as [`CornerConfig::TransverseCap`], the tag the verdict
/// carries for `open::ruled::RuledPlan` to read — and any other link's
/// end is classified as a corner: gather valence, per-edge convexity,
/// and the three support normals, then classify. A uniform trihedron
/// that passes returns `None`: the carved trihedral configuration has
/// no tag of its own ([`CornerConfig::ThreeConvexEdges`] names the
/// convex one and no name exists for the concave one — evgunter/cad
/// issue 1355), so it is the ONE carved configuration the battery
/// admits without tagging.
fn corner_at<T: Decide + Bounds>(
    body: &Body<T>,
    vertex: VertexKey,
    link: &Link<T>,
    radius: T,
    band: Band,
    kind: BlendKind,
) -> Result<Option<CornerConfig>, BlendError> {
    let indeterminate =
        || super::surgery::unbuilt_corner_config(vertex, CornerConfig::Indeterminate);
    // In key order, so the supports below are gathered — and their
    // normals reach the independence determinant — in an order that
    // does not depend on where the vertex's orbit starts.
    // An unresolved key at the corner is a body that does not hold
    // together there, not a configuration: every such arm below
    // refuses as `BodyNotIntact`.
    let not_intact = |at: EntityId, detail: &'static str| BlendError::BodyNotIntact { at, detail };
    let mut edges = fan_at(body.edges_of_vertex(vertex)).ok_or_else(|| {
        not_intact(
            EntityId::Vertex(vertex),
            "a chain end's edge fan, for its corner configuration",
        )
    })?;
    edges.sort_unstable();
    let valence = edges.len();
    // A chart seam crossing a smooth rim is NOT a corner, so it is
    // recognized before the valence is read as a corner configuration —
    // otherwise the refusal describes a wedge that is not there and
    // names a run-out policy that could not help.
    let rim_lists = |seed: EdgeKey, arcs: &[EdgeKey]| {
        topo::query::rim_of(body, seed).is_ok_and(|listed| arcs.iter().all(|e| listed.contains(e)))
    };
    if is_seam_vertex(body, &edges, rim_lists) {
        return Err(super::surgery::unbuilt_corner_config(
            vertex,
            CornerConfig::SeamVertex,
        ));
    }
    // A ruled band terminates where its supports do, in a cap, not in
    // a corner: the classification is the cap's, and it is made before
    // any neighbour is resolved as a link — the cap's rim edges are
    // not blended and need no arm.
    if link.arm.is_ruled() && valence == 3 {
        // `None` only at a non-manifold vertex or a stale key
        // (`cap_incidence`).
        let Some((_, _, cap)) = cap_incidence(body, vertex, link.edge, link.face_a, link.face_b)
        else {
            return Err(not_intact(
                EntityId::Vertex(vertex),
                "a ruled link's end, whose three faces do not meet as a cap",
            ));
        };
        let Some(Surface::Plane { normal, .. }) =
            body.get_face(cap).and_then(|f| body.get_surface(f.surface))
        else {
            return Err(super::surgery::unbuilt_run_out(
                EntityId::Vertex(vertex),
                RULED_END_NOT_TRANSVERSE,
            ));
        };
        // The ruling is the arm's own: a ruled arm's band is the
        // cylinder about it (`Ruling::blend`), so anything else is a
        // verdict that disagrees with itself, and the end cannot be
        // classified from it.
        let Surface::Cylinder { axis, .. } = link.blend.surface else {
            return Err(indeterminate());
        };
        cap_transverse(vertex, *normal, axis, link.arm_len, band)?;
        return Ok(Some(CornerConfig::TransverseCap));
    }
    if valence != 3 {
        return corner_config(
            vertex,
            valence,
            0,
            [Vec3::new(T::zero(), T::zero(), T::zero()); 3],
            radius,
            band,
        )
        .map(|()| None);
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
                return Err(indeterminate());
            }
        }
    }
    let Some(p) = body
        .get_vertex(vertex)
        .and_then(|v| body.get_point(v.point))
    else {
        return Err(not_intact(
            EntityId::Vertex(vertex),
            "a chain end's vertex point, for its support normals",
        ));
    };
    if faces.len() != 3 {
        return corner_config(vertex, faces.len(), convex, normals, radius, band).map(|()| None);
    }
    for (i, f) in faces.iter().enumerate() {
        normals[i] = outward(body, *f, *p).ok_or_else(|| {
            not_intact(
                EntityId::Face(*f),
                "a corner's support face or its stored surface, for its outward normal",
            )
        })?;
    }
    corner_config(vertex, valence, convex, normals, radius, band).map(|()| None)
}

/// **What a junction of an open chain is**, read once for every reader:
/// the open-chain door (`admit::Joint::admit`) refuses each non-joint
/// arm by name, and predicate 2 groups exactly the [`JointVerdict::Joint`]
/// junctions into one feature — so the two cannot disagree on which
/// junctions are joints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum JointVerdict {
    /// Both links plane–plane, on the same two support faces, at a
    /// valence-2 vertex: one band the vertex only splits.
    Joint,
    /// A link's supports are not two planes.
    NotPlanar,
    /// The links lie on different support faces.
    OtherFaces,
    /// The vertex carries edges other than the two links.
    Valence(usize),
    /// The vertex's edge orbit does not walk.
    OrbitBroken,
}

/// Judge the junction at `vertex` between `arriving` and `leaving`.
pub(super) fn joint_verdict<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    arriving: &Link<T>,
    leaving: &Link<T>,
) -> JointVerdict {
    if !(arriving.arm.is_plane_plane() && leaving.arm.is_plane_plane()) {
        return JointVerdict::NotPlanar;
    }
    let mut pa = [arriving.face_a, arriving.face_b];
    let mut pb = [leaving.face_a, leaving.face_b];
    pa.sort_unstable();
    pb.sort_unstable();
    if pa != pb {
        return JointVerdict::OtherFaces;
    }
    // Two edges between the same two faces close a manifold vertex's
    // fan, so the valence is two; checked, not inherited.
    match fan_at(body.edges_of_vertex(vertex)) {
        None => JointVerdict::OrbitBroken,
        Some(es) if es.len() != 2 => JointVerdict::Valence(es.len()),
        Some(_) => JointVerdict::Joint,
    }
}

/// Predicate 2's sweep: for each support face, every pair of its
/// boundary edges, with the blended ones carrying their setbacks.
///
/// **A joined run is one feature.** Consecutive links of one chain on
/// the same two support faces (a joint, `admit::Joint`) are one band
/// whose setback on each support is ONE trimline, so two links of one
/// run are never a pair, and the run's adjacency is the run's: an edge
/// touching either end of the run is its neighbour, as a plain edge's
/// neighbours are. What a joint itself needs is metered instead: its
/// foot on each support must lie beyond the trimline of the edge that
/// meets the run at each end — the corner's own foot is on that
/// trimline, so a joint short of it would put the band's foot inside
/// the corner patch.
fn consumption_sweep<T: Decide + Bounds>(
    body: &Body<T>,
    chains: &[Chain<T>],
    band: Band,
) -> Result<(), BlendError> {
    // Setback of each blended edge on each of its two support faces,
    // and which CHAIN each blended edge belongs to — two requested
    // boundary features from two different chains make a failing pair
    // SPLITTABLE, and the refusal says so.
    let mut setback: Vec<(EdgeKey, FaceKey, T)> = Vec::new();
    let mut chain_of: Vec<(EdgeKey, usize)> = Vec::new();
    let mut faces: Vec<FaceKey> = Vec::new();
    for (ci, chain) in chains.iter().enumerate() {
        for l in chain.links() {
            setback.push((l.edge, l.face_a, l.blend.trim_a.1));
            setback.push((l.edge, l.face_b, l.blend.trim_b.1));
            chain_of.push((l.edge, ci));
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
    let chain_ix = |e: EdgeKey| -> Option<usize> {
        chain_of.iter().find(|(ee, _)| *ee == e).map(|(_, ci)| *ci)
    };
    // The joined runs: each requested edge's run id, a junction of an
    // open chain whose two links share both support faces joining the
    // leaving link's run to the arriving one's.
    let mut run_of: Vec<(EdgeKey, usize)> = Vec::new();
    // Per joint: (vertex, the arriving link) — what its feet are read
    // off.
    let mut joints: Vec<(VertexKey, &Link<T>)> = Vec::new();
    for chain in chains {
        let ring: Vec<&Link<T>> = chain.links().collect();
        let mut ids: Vec<usize> = (0..ring.len()).map(|i| run_of.len() + i).collect();
        // Joints are an OPEN chain's: a closed rim is carved by the rim
        // phases, whose arcs this screen meters edge by edge.
        let open = matches!(chain.closure, ChainClosure::Open { .. });
        for j in chain.junctions.iter().filter(|_| open) {
            let (a, b) = (ring[j.arriving], ring[j.leaving]);
            if joint_verdict(body, j.vertex, a, b) == JointVerdict::Joint {
                let (from, to) = (ids[j.leaving], ids[j.arriving]);
                for id in &mut ids {
                    if *id == from {
                        *id = to;
                    }
                }
                joints.push((j.vertex, a));
            }
        }
        for (i, l) in ring.iter().enumerate() {
            run_of.push((l.edge, ids[i]));
        }
    }
    let run =
        |e: EdgeKey| -> Option<usize> { run_of.iter().find(|(ee, _)| *ee == e).map(|(_, r)| *r) };
    let members = |e: EdgeKey| -> Vec<EdgeKey> {
        match run(e) {
            Some(r) => run_of
                .iter()
                .filter(|(_, rr)| *rr == r)
                .map(|(ee, _)| *ee)
                .collect(),
            None => vec![e],
        }
    };
    let touches = |a: EdgeKey, b: EdgeKey| -> Result<bool, BlendError> {
        touches_any(body, &members(a), &members(b))
    };
    for face in faces {
        let fa = body.get_face(face).ok_or_else(|| {
            not_intact(
                EntityId::Face(face),
                "a support face, for its boundary pairs",
            )
        })?;
        let mut boundary: Vec<ScreenedEdge<T>> = Vec::new();
        for lp in core::iter::once(fa.outer).chain(fa.rings.iter().copied()) {
            boundary.extend(screened_loop(body, lp)?);
        }
        for i in 0..boundary.len() {
            for j in (i + 1)..boundary.len() {
                let (ei, pi) = (&boundary[i].0, &boundary[i].2);
                let (ej, pj) = (&boundary[j].0, &boundary[j].2);
                // Adjacent boundary features TOUCH (gap 0 at the shared
                // vertex) — their setbacks are judged by the corner
                // and G1 predicates, not by this one, so the pair is
                // skipped exactly when the features share a vertex. Two
                // links of one joined run are one feature, and skipped.
                if run(*ei).is_some() && run(*ei) == run(*ej) {
                    continue;
                }
                if touches(*ei, *ej)? {
                    continue;
                }
                // The closest approach of the two sampled boundaries,
                // seeded from a real pair (`CHAIN_SAMPLES` ≥ 1, asserted
                // at compile time), never from an infinite sentinel: at
                // the certified scalar an infinity is the ill-formed
                // interval, which absorbs through `min` and would
                // escalate every margin downstream of it.
                let gap = pi
                    .iter()
                    .flat_map(|a| pj.iter().map(move |b| (*b - *a).norm()))
                    .fold((pj[0] - pi[0]).norm(), T::min);
                let cross_chain = match (chain_ix(*ei), chain_ix(*ej)) {
                    (Some(a), Some(b)) => a != b,
                    _ => false,
                };
                face_clearance(
                    face,
                    gap,
                    look(*ei, face),
                    look(*ej, face),
                    cross_chain,
                    band,
                )?;
            }
        }
        // Each joint on this face: its foot against the trimline of
        // every boundary edge that meets its run at an end.
        for (v, link) in &joints {
            let trim = if link.face_a == face {
                &link.blend.trim_a.0
            } else if link.face_b == face {
                &link.blend.trim_b.0
            } else {
                continue;
            };
            // A joint is plane–plane by its verdict, and
            // `arms::plane_plane_blend` mints its trimlines as lines.
            let Curve3::Line { origin, dir } = trim else {
                return Err(BlendError::SurgeryInvariant {
                    at: EntityId::Edge(link.edge),
                    detail: "a plane–plane joint's trimline is not a line",
                });
            };
            let p = body
                .get_vertex(*v)
                .and_then(|x| body.get_point(x.point))
                .ok_or_else(|| not_intact(EntityId::Vertex(*v), "a joint's stored point"))?;
            let foot = *origin + *dir * ((*p - *origin).dot(*dir) / dir.dot(*dir));
            let run_edges = members(link.edge);
            for (e, carrier, _) in &boundary {
                if run_edges.contains(e) || !touches_any(body, &run_edges, &[*e])? {
                    continue;
                }
                // The run ends at a corner on this PLANE support, so the
                // end edge is straight unless the corner's third support
                // is curved — which no corner carves, and which refuses
                // here as that rather than being sampled.
                let Curve3::Line { origin: o, dir: d } = *carrier else {
                    return Err(unbuilt_geometry(
                        EntityId::Edge(*e),
                        CORNER_SUPPORT_NOT_PLANAR,
                    ));
                };
                let gap = (foot - o).cross(d).norm() / d.norm();
                face_clearance(face, gap, T::zero(), look(*e, face), false, band)?;
            }
        }
    }
    Ok(())
}

/// One boundary edge as predicate 2 reads it: its key, its certified
/// carrier, and the [`CHAIN_SAMPLES`] points along its window.
type ScreenedEdge<T> = (EdgeKey, Curve3<T>, [Point3<T>; CHAIN_SAMPLES as usize]);

const _: () = assert!(
    CHAIN_SAMPLES >= 1,
    "predicate 2 seeds its gap from a real pair"
);

/// **One loop of a support face, read whole for predicate 2**: each
/// boundary edge, in cycle order. Whatever the screen cannot read
/// refuses here, typed — a feature left out of the pair sweep would
/// let the screen report a face clear having metered nothing for it.
fn screened_loop<T: Decide>(
    body: &Body<T>,
    lp: topo::LoopKey,
) -> Result<Vec<ScreenedEdge<T>>, BlendError> {
    let l = body
        .get_loop(lp)
        .ok_or_else(|| not_intact(EntityId::Loop(lp), "a support face's loop"))?;
    let topo::LoopBoundary::Cycle { first } = l.boundary else {
        return Err(unbuilt_geometry(
            EntityId::Loop(lp),
            "a support face carries a lone-vertex cycle, which the face-clearance screen does \
             not cover",
        ));
    };
    let cycle = body
        .loop_cycle(first)
        .ok_or_else(|| not_intact(EntityId::Loop(lp), "a support face's cycle"))?;
    cycle
        .into_iter()
        .map(|he| {
            let edge = body
                .get_half_edge(he)
                .ok_or_else(|| not_intact(EntityId::HalfEdge(he), "a support face's boundary"))?
                .edge;
            let e = body.get_edge(edge).ok_or_else(|| {
                not_intact(EntityId::Edge(edge), "a support face's boundary edge")
            })?;
            let c = body
                .get_curve_geom(e.curve)
                .ok_or_else(|| {
                    not_intact(EntityId::Edge(edge), "a support boundary edge's curve row")
                })?
                .certified()
                .ok_or_else(|| {
                    unbuilt_geometry(
                        EntityId::Edge(edge),
                        "a support face's boundary edge carries no certified carrier",
                    )
                })?;
            let (t0, t1) = c.params();
            let pts = core::array::from_fn(|i| c.carrier().eval(chain_sample_at(t0, t1, i as u32)));
            Ok((edge, c.carrier().clone(), pts))
        })
        .collect()
}

/// Whether any edge of `xs` shares an end vertex with any edge of
/// `ys`. An edge whose ends do not resolve refuses rather than reading
/// as "apart": apart is what puts a pair INTO the screen, but it is
/// also what keeps a joint's neighbour OUT of the joint's meter.
fn touches_any<T: Decide>(
    body: &Body<T>,
    xs: &[EdgeKey],
    ys: &[EdgeKey],
) -> Result<bool, BlendError> {
    let ends = |e: EdgeKey| -> Result<[VertexKey; 2], BlendError> {
        let he = body
            .get_edge(e)
            .ok_or_else(|| not_intact(EntityId::Edge(e), "a boundary edge, for its ends"))?
            .he_plus;
        body.get_half_edge(he)
            .map(|h| h.start)
            .zip(body.half_edge_end(he))
            .map(|(s, t)| [s, t])
            .ok_or_else(|| not_intact(EntityId::HalfEdge(he), "a boundary edge's ends"))
    };
    for x in xs {
        let ex = ends(*x)?;
        for y in ys {
            if ends(*y)?.iter().any(|v| ex.contains(v)) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
