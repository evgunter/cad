//! Profile validation: the trilean gate from raw [`Profile`] data to the
//! canonical [`ValidatedProfile`] that sweeps consume.
//!
//! # The checks, in order
//!
//! 1. **Arity** — every loop has ≥ 2 vertices (structural; a 2-vertex
//!    loop with two arc segments is the legal minimal circle).
//! 2. **Degeneracy** — no zero-length segments (chord-level vertex
//!    coincidence), via the `vertex_separation` distance predicate;
//!    in-band ⇒ escalation (the Q1 sliver semantics).
//! 3. **Simplicity** — pairwise closed-form segment/segment contact
//!    classification (line/line, line/arc, arc/arc; O(n²), fine at M2).
//!    Adjacent segments may share exactly their common vertex; any other
//!    contact is a typed error. Tangential contact (touching without
//!    crossing) is semantically indeterminate and escalates as
//!    [`ProfileError::TangentialContact`] (D4 ¶3: typed, actionable).
//! 4. **Declared tangency** (the #101 discipline) — every *joint*
//!    (adjacent-segment junction at its shared vertex) is classified by
//!    the same carrier predicates the simplicity pass uses — the
//!    carrier clearance margins (`carrier_line_circle`, the
//!    `carrier_circles_*` family) are bit-identical expressions;
//!    line/line joints reuse `chord_side` in the same expression form
//!    on the joint's far endpoint (a carrier-identity question — no
//!    new ε anywhere) — and reconciled with the loop's declared
//!    [`crate::ProfileLoop::tangent_joints`]: definite tangency between
//!    distinct carriers undeclared ⇒
//!    [`ProfileError::UndeclaredTangency`]; a declaration that is
//!    definitely not a tangency (transversal, or same-carrier
//!    continuation — collinear/cocircular joints are carrier identity,
//!    not tangency) ⇒ [`ProfileError::TangencyContradicted`] (declared
//!    tangency is verified, never trusted). In-band near-tangency
//!    escalates from the simplicity pass as it always did; the refusal
//!    text carries the declare-or-move repair menu.
//! 5. **Containment forest** — trilean point-in-loop by ray parity
//!    (rays through arc segments included); a grazing ray is refused and
//!    the next candidate ray tried deterministically (Mäntylä ch. 13's
//!    retry made principled); exhaustion is a typed escalation. Depth 0
//!    = the outer boundary, depth 1 = holes; deeper nesting or multiple
//!    outers are typed errors at M2 (one face region per profile).
//! 6. **Canonicalization** — see [`ValidatedProfile`] for the rules.
//!
//! # Predicate inventory (margins in meters; lever arms named)
//!
//! Every decision goes through the [`geom_core::k_stats::decide`] funnel
//! with one of these names — the unit of the K-experiment margin
//! statistics:
//!
//! | predicate | margin | lever arm |
//! |---|---|---|
//! | `vertex_separation` | chord length | direct displacement |
//! | `segment_straightness` | sagitta L·b/2 | half-chord (bulge → meters) |
//! | `arc_diameter_clearance` | 2r − half-span chord | ≈ L²/16r near full arcs |
//! | `chord_side` | ⟂ distance to chord line | direct |
//! | `line_span` | min(t, L−t) along carrier | direct |
//! | `arc_span` | chordal defect from apex | ×1/cos(θ/4) near full arcs |
//! | `carrier_line_circle` | r − |h| clearance | tangency: r·φ²/2 |
//! | `carrier_circles_identity` | d + |Δr| | direct |
//! | `carrier_circles_external` | d − (r₁+r₂) | direct |
//! | `carrier_circles_internal` | d − |Δr| | direct |
//! | `arc_apex_identity` | apex distance | direct |
//! | `collinear_overlap` | overlap length | direct |
//! | `contact_at_shared_vertex` | contact-to-vertex distance | direct |
//! | `ray_side` | ⟂ distance to ray line | direct |
//! | `ray_advance` | ray parameter | direct |
//! | `loop_orientation` | 2·area/perimeter (sliver width) | half-perimeter (area → meters) |
//! | `canonical_order_x` / `_y` | coordinate difference | exact-order band (see below) |
//! | `fillet_leg_fit` | leg length − tangent setback | exact-order band; fired in the fillet CONSTRUCTORS, not here |
//! | `fillet_leg_reach` | tangent setback from the corner | exact-order band; the constructor's corner-side extent test (M5 S2) |
//! | `fillet_corner_arm` | min leg lever arm | linear band; the collapsed-arm gate (M5 S2) |
//! | `fillet_corner_turn` | sin φ · arm | linear band; φ the corner's turn, arm = min(leg extents, leg carrier radii) |
//! | `fillet_offset_line_circle` | \|ρ\| − \|h\| clearance | linear band; offset-carrier intersection (M5 S2) |
//! | `fillet_offset_circles_external` | \|ρ₁\|+\|ρ₂\| − d | linear band; offset-carrier intersection (M5 S2) |
//! | `fillet_offset_circles_internal` | d − \|\|ρ₁\|−\|ρ₂\|\| | linear band; offset-carrier intersection (M5 S2) |
//! | `fillet_offset_lever` | \|ρ₂\| − C·R₂·scale²/(d·ε) | linear band; the arc×arc offset intersection's conditioning (M8) |
//!
//! Every `fillet_*` row above fires in
//! the arc-carrier fillet construction (construction sugar's one
//! documented decision site), never in validation; the two exact-order
//! rows are order/extent decisions on the same footing as
//! `canonical_order_*` and are excluded from the K lint's ratio rules
//! for the same reason.
//!
//! The band coherence worth noting for the K report: contact margins
//! and the orientation margin share K, so a loop thin enough to look
//! like a sliver (width in (ε, Kε)) already escalated at the simplicity
//! stage — the sliver band is one consistent notion across predicates.
//!
//! **The exact-order band.** Canonical-start selection is a
//! *representation* choice, not a geometric coincidence question: it
//! must be total, transitive, and bit-deterministic, and a tolerance
//! band would make near-ties non-transitive (x-coordinates 1.5ε apart
//! are legitimate and must still order). It therefore classifies
//! against the documented hairline band (`geom_core::BandError::Empty`
//! docs): (min-subnormal, 2·min-subnormal), whose open interior
//! contains no representable `f64` — comparisons are exact and total at
//! `f64`, and a Zero (tie) means the coordinates differ by at most one
//! minimum subnormal (bit-identical, or exact ±min-subnormal
//! neighbors), which validated loops cannot exhibit in both
//! coordinates at once. At the interval scalar an enclosure straddling
//! the hairline escalates honestly.

use core::fmt;

use geom_core::k_stats::decide;
use geom_core::{
    Band, BandError, COINCIDENCE_RECOURSE, Decide, Indeterminate, Margin, Point2, Real, Sign, Tol,
    Vec2,
};

use crate::seg::{self, CKind, PairOutcome, Seg, SegIssue, SegKind, build_seg};
use crate::{Profile, ProfileLoop, ProfileVertex};

/// Identifies a segment of the *input* profile: `segment_index` k is the
/// segment from vertex k to vertex k+1 (mod n) of loop `loop_index`, in
/// the input's own ordering (errors always reference input indices —
/// canonical reindexing exists only on success).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentRef {
    /// Index of the loop in [`Profile::loops`].
    pub loop_index: usize,
    /// Index of the segment within the loop.
    pub segment_index: usize,
}

impl fmt::Display for SegmentRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "loop {} segment {}", self.loop_index, self.segment_index)
    }
}

/// How two segments illegally meet (the definite non-simplicity kinds;
/// tangential contact is its own error variant, being an escalation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactKind {
    /// A transversal crossing interior to both segments.
    Crossing,
    /// An isolated contact at a segment endpoint that is not the
    /// segments' shared vertex.
    Touch,
    /// A shared sub-locus of positive length (collinear or cocircular
    /// overlap).
    Overlap,
}

impl fmt::Display for ContactKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Crossing => "crossing",
            Self::Touch => "endpoint contact",
            Self::Overlap => "overlap",
        })
    }
}

/// Where a classification escalated (in-band margin or poisoned value).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscalationSite {
    /// While classifying one segment.
    Segment(SegmentRef),
    /// While testing a segment pair for contact.
    SegmentPair(SegmentRef, SegmentRef),
    /// While orienting or canonicalizing a loop.
    Loop {
        /// Index of the loop in [`Profile::loops`].
        loop_index: usize,
    },
    /// While constructing a fillet corner (the `sugar` trim helpers
    /// behind the PATHS `.fillet(r)` door — the only decisions
    /// construction sugar takes: the leg-fit and corner-side extent
    /// gates against the exact-order band, and, on the arc-leg path, the
    /// lever-arm, corner-turn and offset-carrier gates against the run's
    /// linear band). The escalation's `source` names which.
    Fillet,
}

impl fmt::Display for EscalationSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Segment(s) => write!(f, "at {s}"),
            Self::SegmentPair(a, b) => write!(f, "between {a} and {b}"),
            Self::Loop { loop_index } => write!(f, "on loop {loop_index}"),
            Self::Fillet => f.write_str("in the fillet constructor's gates"),
        }
    }
}

/// Which leg of a fillet corner a diagnostic refers to (the incoming
/// leg runs chain-head → corner; the outgoing leg corner → `next`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilletLeg {
    /// The leg arriving at the corner (chain head → corner).
    Incoming,
    /// The leg leaving the corner (corner → `next`).
    Outgoing,
}

impl fmt::Display for FilletLeg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Incoming => "incoming leg",
            Self::Outgoing => "outgoing leg",
        })
    }
}

/// What kind of carrier a fillet leg runs on, with the diagnostic
/// margin metered in that carrier's own currency (M5 S2): a straight
/// leg's setback and length are linear distances; a circular leg's are
/// **arc lengths** `R·Δθ`, and the same fit margin is also reported in
/// radians so the author can read the angular room directly.
///
/// Diagnostic payload only (`f64`, the enclosure's lower bound at
/// interval scalars) — for messages, never for re-deciding.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilletLegCarrier {
    /// A straight leg: setback and length are linear distances (m).
    Line,
    /// A circular leg on a carrier of radius `radius` (m): setback and
    /// length are arc lengths, and `angular_margin` is the leg's fit
    /// margin `(length − setback)/radius` in radians (negative when the
    /// tangent point overruns the leg's swept extent).
    Arc {
        /// The leg carrier's radius, meters.
        radius: f64,
        /// The fit margin in radians (see the variant docs).
        angular_margin: f64,
    },
}

impl fmt::Display for FilletLegCarrier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Line => f.write_str("straight"),
            Self::Arc {
                radius,
                angular_margin,
            } => write!(
                f,
                "circular (carrier radius {radius} m, angular margin {angular_margin} rad)"
            ),
        }
    }
}

/// Why a fillet corner admits **no** tangent circle of the requested
/// radius (the finer split inside
/// [`crate::path::PathNoCornerReason::NoTangentCircle`], carried by
/// [`crate::PathError::NoCornerForFillet`] — the situation
/// `docs/PATHS-DESIGN.md` §2 (the Fillet section) names for the
/// algebra's `.fillet(r)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoCornerReason {
    /// The two offset carriers (each leg's carrier pushed `r` toward
    /// the corner's turn side) do not meet at all: no circle of radius
    /// `r` is tangent to both legs' carriers *anywhere*, so there is no
    /// corner of that radius to construct — the radius is too large for
    /// the corner's curvature, or the carriers are parallel /
    /// non-intersecting.
    OffsetCarriersDisjoint,
    /// Tangent circles of radius `r` exist, but every one of them
    /// touches a leg **past the corner** — the arc would round a corner
    /// the legs do not actually reach (the branch rule's corner-side
    /// extent test).
    NoCornerSideCandidate,
}

impl fmt::Display for NoCornerReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::OffsetCarriersDisjoint => {
                "no circle of that radius is tangent to both leg carriers"
            }
            Self::NoCornerSideCandidate => {
                "every tangent circle of that radius touches a leg past the corner"
            }
        })
    }
}

/// The recourse for an in-band corner **turn**, where the constructor
/// has admitted it cannot classify the margin and therefore cannot say
/// which of the two degenerate corner classes it is looking at
/// (smooth-tangent, whose recourse is declaring the tangency, or
/// reverse-tangent — a cusp, which the kernel refuses; #131 is the
/// tabled front door).
///
/// Deliberately names both doors. The two-tolerance discipline asks for
/// one sentence per user situation, and the situation here is precisely
/// "this corner is degenerate and which kind is below the tolerance" —
/// rendering either single-class sentence would assert the very thing
/// the escalation declined to decide.
const FILLET_TURN_INBAND_RECOURSE: &str = "this corner is degenerate at any precision you could care about, and which kind is below \
     the tolerance: if the legs run smoothly into each other, keep them and declare the \
     tangency (the joint's index in the loop's tangent_joints); if they double back, that is a cusp and the \
     kernel refuses it; otherwise move the geometry so a real corner exists (or lower \
     the tolerance)";

/// The recourse for a corner that admits no tangent circle of the
/// requested radius — one sentence for the definite refusal and for the
/// in-band escalation alike (the two-tolerance discipline, D4 ¶1
/// addendum: one message and one recourse per user situation).
/// Also carries `fillet_leg_reach`, whose situation is the same one:
/// whether a corner of this radius exists on the corner side at all
/// (review MINOR-1).
const FILLET_NO_CORNER_RECOURSE: &str =
    "use a smaller radius, or move the legs so a circle of that radius can sit in the corner";

/// The recourse for a corner whose offset lever is too short to place a
/// tangent point within ε — one sentence for the definite refusal
/// ([`crate::PathError::FilletOffsetLeverTooShort`]) and for the in-band
/// escalation of `fillet_offset_lever` alike (D4 ¶1's clause (iv): a new
/// definite arm inherits its in-band sibling's one story).
///
/// It names the lever the user can actually move. ρ = R − σ·τ·r collapses
/// when the fillet radius approaches the leg's own carrier radius on the
/// side the corner turns toward, and everything else in the threshold is
/// the corner's scale, which the author usually cannot trade.
const FILLET_OFFSET_LEVER_RECOURSE: &str = "the tangent point is recovered by projecting the fillet's centre back onto that leg's \
     carrier, and the projection divides by the offset radius rho = R - sigma*tau*r, so a \
     fillet radius this close to the leg's carrier radius cannot place the tangent point \
     within tolerance: move the fillet radius away from that leg's carrier radius, or bring \
     the corner's carriers closer together (or lower the tolerance)";

/// The recourse for a radius whose tangent points fall outside their
/// legs — shared by the definite refusal and the in-band escalation.
const FILLET_FIT_RECOURSE: &str =
    "the arc would never approach the requested corner; use a smaller radius or longer legs";

/// The recourse for a fillet leg with no extent to round against.
const FILLET_LEG_EXTENT_RECOURSE: &str = "give the leg a real extent (a non-degenerate chord, or an arc carrier with a positive \
     radius and a non-zero sweep) — a leg with no extent has no direction to be tangent to";

/// Typed validation failure — the closed error enum of
/// [`Profile::validate`] (D4 ¶3: every failure is typed and actionable;
/// D9: never a panic). All indices reference the *input* profile.
#[derive(Debug, Clone, PartialEq)]
pub enum ProfileError {
    /// The run's tolerance could not form a classification band
    /// (misconfigured ε — see [`BandError`]).
    Band(BandError),
    /// The profile has no loops — there is no region to sweep.
    EmptyProfile,
    /// A loop has fewer than two vertices (a closed carrier needs ≥ 2 —
    /// the crate docs' minimal-circle rule).
    TooFewVertices {
        /// Index of the offending loop.
        loop_index: usize,
        /// Its vertex count.
        count: usize,
    },
    /// Consecutive vertices coincide at tolerance: a zero-length
    /// segment.
    DegenerateSegment(SegmentRef),
    /// An arc segment is within tolerance of a full circle (its
    /// half-span chord reaches the carrier diameter at tolerance): the
    /// complement gap is a sliver and span classification would not be
    /// honest there (the `arc_diameter_clearance` predicate — M2 PR 2
    /// review fix). Split the arc at another vertex; a full circle is
    /// two arcs by construction.
    NearFullArc(SegmentRef),
    /// Two segments meet where they may not (any contact other than
    /// adjacent segments' shared vertex).
    NonSimple {
        /// One offending segment.
        first: SegmentRef,
        /// The other.
        second: SegmentRef,
        /// How they meet.
        kind: ContactKind,
    },
    /// Two segments touch tangentially without crossing, interior to
    /// both — semantically indeterminate for simplicity (an arbitrarily
    /// small perturbation flips the region's interior status), so it is
    /// refused as its own typed escalation.
    TangentialContact {
        /// One offending segment.
        first: SegmentRef,
        /// The other.
        second: SegmentRef,
    },
    /// A declared-tangent joint index ([`crate::ProfileLoop::tangent_joints`])
    /// is not a vertex index of its loop.
    TangentJointOutOfRange {
        /// Index of the offending loop.
        loop_index: usize,
        /// The declared (out-of-range) joint index.
        joint: usize,
        /// The loop's vertex count.
        count: usize,
    },
    /// Adjacent segments meet tangentially at their shared vertex —
    /// the joint's distinct carriers are in definite first-order
    /// contact — but the joint is not declared tangent. Tangency that
    /// numerically happens-to-hold is refused (the boolean door's
    /// UndeclaredCoincidence, lifted to the profile door): declare the
    /// intent or move the geometry.
    UndeclaredTangency {
        /// The segment arriving at the joint.
        first: SegmentRef,
        /// The segment leaving the joint.
        second: SegmentRef,
        /// The joint's vertex index (input chain of `first`'s loop) —
        /// the index a declaration must name.
        joint: usize,
        /// The repair menu, rendered once at detection.
        suggestion: String,
    },
    /// A joint declared tangent is definitely not: the flag is
    /// verified, never trusted (the profile analogue of the boolean
    /// door's DeclarationContradicted).
    TangencyContradicted {
        /// The segment arriving at the joint.
        first: SegmentRef,
        /// The segment leaving the joint.
        second: SegmentRef,
        /// The joint's vertex index (input chain).
        joint: usize,
        /// `true` if the adjacent segments share one carrier
        /// (collinear/cocircular continuation — not a tangency);
        /// `false` if their distinct carriers meet transversally.
        same_carrier: bool,
    },
    /// A loop's area-per-perimeter width classified as zero: a sliver
    /// loop (unreachable for loops that passed simplicity — kept total).
    SliverLoop {
        /// Index of the offending loop.
        loop_index: usize,
    },
    /// More than one loop at containment depth 0: multiple outer
    /// boundaries (multi-region profiles are deferred past M2).
    MultipleOuterLoops {
        /// The depth-0 loop indices.
        outer_loops: Vec<usize>,
    },
    /// A loop nests deeper than outer + hole (depth ≥ 2).
    NestingTooDeep {
        /// Index of the offending loop.
        loop_index: usize,
        /// Its containment depth (number of loops strictly containing
        /// it).
        depth: usize,
    },
    /// Every candidate ray grazed while testing loop containment — a
    /// typed escalation (the deterministic retry list is exhausted;
    /// the profile sits adversarially on every candidate ray).
    RayCastingExhausted {
        /// The loop whose representative point was tested.
        loop_index: usize,
        /// The loop tested against.
        against_loop: usize,
    },
    /// A predicate could not decide: the margin landed in the ambiguity
    /// band (a genuine sliver configuration) or was poisoned (NaN
    /// coordinates). Carries the named predicate's escalation verbatim.
    Escalated {
        /// Where in the profile the decision was being made.
        site: EscalationSite,
        /// The predicate-layer escalation (name, margin diagnostic,
        /// band).
        source: Indeterminate,
    },
}

impl fmt::Display for ProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band(e) => write!(f, "profile validation could not form a band: {e}"),
            Self::EmptyProfile => f.write_str("profile has no loops — nothing to sweep"),
            Self::TooFewVertices { loop_index, count } => write!(
                f,
                "loop {loop_index} has {count} vertex(es); a closed loop needs at least 2 \
                 (two arc segments make the minimal circle)"
            ),
            Self::DegenerateSegment(s) => write!(
                f,
                "{s} is degenerate: consecutive vertices coincide at tolerance"
            ),
            Self::NearFullArc(s) => write!(
                f,
                "{s} is within tolerance of a full circle — split the arc at another \
                 vertex (a full circle is two arcs by construction)"
            ),
            Self::NonSimple {
                first,
                second,
                kind,
            } => write!(
                f,
                "profile is not simple: {kind} between {first} and {second}"
            ),
            Self::TangentialContact { first, second } => write!(
                f,
                "tangential contact between {first} and {second}: touching without \
                 crossing is semantically indeterminate — {COINCIDENCE_RECOURSE} (D4)"
            ),
            Self::TangentJointOutOfRange {
                loop_index,
                joint,
                count,
            } => write!(
                f,
                "loop {loop_index} declares a tangent joint at vertex {joint}, but the \
                 loop has only {count} vertices"
            ),
            Self::UndeclaredTangency {
                first,
                second,
                joint,
                suggestion,
            } => write!(
                f,
                "{first} and {second} meet tangentially at their shared vertex \
                 (joint {joint}) without a declaration — tangency is declared intent, \
                 never a numerical accident; {suggestion}"
            ),
            Self::TangencyContradicted {
                first,
                second,
                joint,
                same_carrier,
            } => {
                if *same_carrier {
                    write!(
                        f,
                        "joint {joint} between {first} and {second} is declared tangent, \
                         but the segments continue on one shared carrier \
                         (collinear/cocircular) — continuation is not a tangency; remove \
                         the declaration (declared tangency is verified, never trusted)"
                    )
                } else {
                    write!(
                        f,
                        "joint {joint} between {first} and {second} is declared tangent, \
                         but the carriers definitely meet transversally — remove the \
                         declaration or make the tangency exact \
                         (the PATHS .fillet(r) door computes it); declared tangency is \
                         verified, never trusted"
                    )
                }
            }
            Self::SliverLoop { loop_index } => write!(
                f,
                "loop {loop_index} has zero width at tolerance (sliver loop)"
            ),
            Self::MultipleOuterLoops { outer_loops } => write!(
                f,
                "profile has {} outer boundaries {:?}; one face region per profile",
                outer_loops.len(),
                outer_loops
            ),
            Self::NestingTooDeep { loop_index, depth } => write!(
                f,
                "loop {loop_index} nests at depth {depth}; only outer boundaries \
                 (depth 0) and holes (depth 1) are supported"
            ),
            Self::RayCastingExhausted {
                loop_index,
                against_loop,
            } => write!(
                f,
                "containment of loop {loop_index} in loop {against_loop}: every \
                 candidate ray grazed — escalating rather than guessing"
            ),
            Self::Escalated { site, source } => {
                write!(f, "validation escalated {site}: {source}")?;
                // The near-tangency site note (#101 point 2, reworked by
                // the S6 two-tolerance sweep): the recourse levers ride
                // `{source}` (the shared carrier); this addendum adds
                // only the site-specific mechanics of the declare lever.
                let near_tangency = matches!(site, EscalationSite::SegmentPair(_, _))
                    && matches!(
                        source.predicate,
                        Some(
                            "carrier_line_circle"
                                | "carrier_circles_external"
                                | "carrier_circles_internal"
                        )
                    );
                if near_tangency {
                    f.write_str(
                        " — near-tangency: the PATHS .fillet(r) door computes and \
                         declares an exact tangency (declared tangency is verified)",
                    )?;
                }
                // The fillet constructor's riders (M5 S2): each in-band
                // fillet predicate renders the SAME recourse sentence as
                // its definite refusal — one message and one recourse per
                // user situation below eps_input, the margin riding the
                // payload (D4 ¶1 addendum; M5 S6's shape, composed
                // from these shared carriers).
                if matches!(site, EscalationSite::Fillet) {
                    match source.predicate {
                        Some("fillet_corner_turn") => {
                            write!(f, " — {FILLET_TURN_INBAND_RECOURSE}")?;
                        }
                        Some("fillet_corner_arm") => {
                            write!(f, " — {FILLET_LEG_EXTENT_RECOURSE}")?;
                        }
                        // `fillet_leg_reach` belongs with the offset
                        // clearances, not with the fit gate (review
                        // MINOR-1): its situation is whether a corner of
                        // this radius exists on the corner side, and its
                        // definite refusal is the path door's
                        // `PathError::NoCornerForFillet`, so the trio
                        // renders one sentence end to end.
                        Some(
                            "fillet_offset_line_circle"
                            | "fillet_offset_circles_external"
                            | "fillet_offset_circles_internal"
                            | "fillet_leg_reach",
                        ) => {
                            write!(f, " — {FILLET_NO_CORNER_RECOURSE}")?;
                        }
                        Some("fillet_leg_fit") => {
                            write!(f, " — {FILLET_FIT_RECOURSE}")?;
                        }
                        // The conditioning gate's in-band arm (M8): the
                        // same one story as its definite sibling
                        // `PathError::FilletOffsetLeverTooShort`, per
                        // D4 ¶1 (iv).
                        Some("fillet_offset_lever") => {
                            write!(f, " — {FILLET_OFFSET_LEVER_RECOURSE}")?;
                        }
                        _ => {}
                    }
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ProfileError {}

/// A validated loop's traversal role, assigned by the containment
/// forest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopRole {
    /// The outer boundary (canonicalized counterclockwise in sketch
    /// coordinates).
    Outer,
    /// A hole (canonicalized clockwise).
    Hole,
}

/// A validated segment's classified carrier, exposed read-only for
/// sweeps (PR 4 lowers `Arc` to a `geom` circle carrier and
/// `Line` to a line carrier at sweep time).
#[derive(Debug, Clone, Copy)]
pub enum SegmentKind<T: Real> {
    /// A straight segment on its chord.
    Line,
    /// A circular arc.
    Arc {
        /// The carrier circle's center (sketch coordinates).
        center: Point2<T>,
        /// The carrier circle's radius (positive).
        radius: T,
        /// The turn sense: `Positive` = counterclockwise sweep
        /// (positive bulge), `Negative` = clockwise. Never `Zero` (that
        /// classification is a `Line`).
        turn: Sign,
    },
}

/// One segment of a validated loop, in canonical traversal order.
#[derive(Debug, Clone, Copy)]
pub struct ValidatedSegment<T: Real> {
    /// Start point.
    pub start: Point2<T>,
    /// End point.
    pub end: Point2<T>,
    /// The bulge, in canonical traversal (reversal negated it if the
    /// input wound the other way). This is carried-through input data;
    /// **consumers select carriers by [`ValidatedSegment::kind`], never
    /// by re-inspecting the bulge** — a sub-tolerance bulge classifies
    /// as `Line` while retaining its stored value. One sanctioned
    /// re-inspection: for an `Arc` segment the **parameter span is
    /// θ = 4·atan|bulge|**, taken from the stored bulge (exact input
    /// data; deriving the span from endpoint `atan2` angles instead is
    /// seam-fragile for wide arcs at small ε) — PR 4's sweeps rely on
    /// this.
    pub bulge: T,
    /// The classified carrier — the decision sweeps consume (PR 4
    /// lowers `Arc` to a circle carrier, `Line` to a line carrier).
    pub kind: SegmentKind<T>,
}

/// A canonicalized loop: role, chain, and classified segments —
/// read-only.
#[derive(Debug, Clone)]
pub struct ValidatedLoop<T: Real> {
    vertices: Vec<ProfileVertex<T>>,
    segments: Vec<ValidatedSegment<T>>,
    tangent_joints: Vec<usize>,
    role: LoopRole,
}

impl<T: Real> ValidatedLoop<T> {
    /// The loop's role in the face region.
    pub fn role(&self) -> LoopRole {
        self.role
    }

    /// The canonical vertex chain (see [`ValidatedProfile`] for the
    /// canonical-form rules).
    pub fn vertices(&self) -> &[ProfileVertex<T>] {
        &self.vertices
    }

    /// The classified segments, segment k running from vertex k to
    /// vertex k+1 (mod n).
    pub fn segments(&self) -> &[ValidatedSegment<T>] {
        &self.segments
    }

    /// The declared (and verified — validation refuses otherwise)
    /// tangent joints, as **canonical** vertex indices: joint v is the
    /// junction between segment (v − 1 mod n) and segment v. Sorted
    /// ascending, deduplicated.
    pub fn tangent_joints(&self) -> &[usize] {
        &self.tangent_joints
    }

    /// **Which segment is the fillet at corner k?** — every arc
    /// segment this loop declares TANGENT at both of its junctions, in
    /// CANONICAL segment order.
    ///
    /// **Authored order is not preserved, and entry `k` is not the
    /// k-th fillet you wrote.** Validation canonicalizes: it rotates
    /// the chain to its lex-min vertex, and it REVERSES a loop wound
    /// the wrong way for its role — so a hole authored with fillets
    /// `[r1, r2]` answers `[r2, r1]`. Correlate by
    /// [`BlendArc::segment`] (a canonical index, which is the only
    /// index this layer has) or by the arc data itself, which is
    /// self-identifying; do not index by authoring position.
    ///
    /// A corner fillet leaves exactly this shape behind: an arc that
    /// meets both legs tangentially, with both junctions declared (and
    /// verified — validation refuses an unmet declaration). So the
    /// answer is STRUCTURAL — it reads [`ValidatedLoop::tangent_joints`],
    /// decides nothing, and compares no floats. That is what makes it
    /// a replacement for the alternative every caller wrote instead:
    /// scanning the segments for "the arc whose radius equals R",
    /// which finds the wrong arc as soon as two blends share a radius,
    /// and silently finds nothing when the stored radius drifts an
    /// ulp.
    ///
    /// **What it reports, exactly.** Any tangent-continuous arc
    /// answers, not only arcs an author reached through fillet sugar —
    /// a full circle lowered as two tangent semicircles presents the
    /// same shape and is listed too. This is a read of the loop's
    /// structure, not a verdict about intent, and the docs say so
    /// rather than the answer guessing.
    ///
    /// ```
    /// use geom_core::{Point2, Tolerance};
    /// use profile::{ArcSweep, Center, Open, Profile, SegmentKind, SketchPlane, Start};
    ///
    /// // The tour rocker's eye slot: two R = 1 lobes meeting tip to
    /// // tip, the TOP tip filleted at R = 1/4, the bottom left sharp.
    /// let tip = 0.75f64.sqrt();
    /// let eye = Open
    ///     .arc_fillet_arc(
    ///         Center {
    ///             c: Point2::new(-0.5, 0.0),
    ///             winding: ArcSweep::Ccw,
    ///             p: Point2::new(0.0, -tip),
    ///         },
    ///         0.25,
    ///         Center {
    ///             c: Point2::new(0.5, 0.0),
    ///             winding: ArcSweep::Ccw,
    ///             p: Start,
    ///         },
    ///     )
    ///     .expect("the near candidate resolves the tip");
    /// let slot = Profile::new(SketchPlane::xy(), vec![eye.into()])
    ///     .validate(Tolerance::get())
    ///     .expect("the eye slot validates");
    ///
    /// // One filleted corner, found by structure — no radius scan.
    /// let blends = slot.loops()[0].blend_arcs();
    /// assert_eq!(blends.len(), 1);
    /// match blends[0].kind {
    ///     SegmentKind::Arc { center, radius, .. } => {
    ///         assert!((radius - 0.25).abs() < 1e-12);
    ///         assert!(center.x.abs() < 1e-12);
    ///     }
    ///     SegmentKind::Line => panic!("a fillet is an arc"),
    /// }
    /// ```
    #[must_use]
    pub fn blend_arcs(&self) -> Vec<BlendArc<T>> {
        let n = self.segments.len();
        let tangent_at = |v: usize| self.tangent_joints.binary_search(&v).is_ok();
        (0..n)
            .filter(|&j| matches!(self.segments[j].kind, SegmentKind::Arc { .. }))
            .filter(|&j| tangent_at(j) && tangent_at((j + 1) % n))
            .map(|j| BlendArc {
                segment: j,
                kind: self.segments[j].kind,
            })
            .collect()
    }
}

/// One blend arc of a validated loop: which canonical segment it is,
/// and the arc data the classifier gave it.
#[derive(Clone, Copy, Debug)]
pub struct BlendArc<T: Real> {
    /// The canonical segment index within the loop (segment `j` runs
    /// from vertex `j` to vertex `j + 1 mod n`).
    pub segment: usize,
    /// The classified carrier — always [`SegmentKind::Arc`] here.
    pub kind: SegmentKind<T>,
}

/// A validated, canonicalized profile — the only thing sweeps accept.
///
/// # Canonical form (deterministic, D9-load-bearing)
///
/// - **Loop order**: the outer boundary first, then holes in input
///   (discovery) order.
/// - **Traversal**: the outer loop runs counterclockwise, holes run
///   clockwise, *in sketch coordinates* (reversal negates bulges — the
///   involution of the crate docs).
/// - **Starting vertex**: each loop starts at its lexicographically
///   minimal vertex (least x, then least y), compared through the
///   exact-order band (module docs) — total and rotation-invariant.
///
/// The canonical form is invariant under the input's traversal
/// direction and starting-vertex rotation of every loop (under test,
/// byte-level). Stated honestly, this is *decision* invariance: the
/// output values are exact reindexings of the input, but the
/// predicate margins deciding them are computed in
/// rotation-dependent order (e.g. `loop_orientation` anchors its
/// shoelace at the chain's first vertex), so a margin within an ulp
/// of a band edge could in principle classify differently across
/// rotations — a measure-zero configuration, stated rather than
/// hidden. It is **not** invariant under reordering the input's loop
/// *list*: holes keep discovery (input) order, and D9 recipes replay
/// that order — see the crate docs.
#[derive(Debug, Clone)]
pub struct ValidatedProfile<T: Real> {
    plane: crate::SketchPlane<T>,
    loops: Vec<ValidatedLoop<T>>,
}

impl<T: Real> ValidatedProfile<T> {
    /// The sketch plane, passed through from the input (validation is
    /// 2-D; the plane is conventional data).
    pub fn plane(&self) -> &crate::SketchPlane<T> {
        &self.plane
    }

    /// The canonicalized loops (outer first, then holes in input
    /// order).
    pub fn loops(&self) -> &[ValidatedLoop<T>] {
        &self.loops
    }
}

/// The number of candidate rays for containment parity before
/// exhaustion escalates.
const N_RAYS: usize = 16;

/// Ray-direction schedule: angle k = `RAY_BASE` + k·`RAY_STEP` radians.
/// Arbitrary fixed constants (D9: documented, deterministic), chosen so
/// no candidate aligns with axis-parallel or 45° human-scale data (the
/// golden angle is as far from every low-order rational multiple of τ
/// as an angle gets); the retry sequence is the same for every
/// containment question.
const RAY_BASE: f64 = 0.5;
/// The golden angle, 2π·(1 − 1/φ).
const RAY_STEP: f64 = 2.399_963_229_728_653;

impl<T: Decide> Profile<T> {
    /// Validates the profile and produces its canonical form — the gate
    /// everything downstream builds on (module docs: the checks, the
    /// predicate inventory; [`ValidatedProfile`]: the canonical-form
    /// rules).
    ///
    /// `tol` is the run's tolerance (normally
    /// `geom_core::Tolerance::get()`); the classification band (ε, K·ε)
    /// is built from it once at entry — the geom-core calling
    /// convention.
    ///
    /// # Errors
    ///
    /// [`ProfileError`] — closed, typed, actionable; every predicate
    /// escalation (in-band margin, poisoned coordinate) surfaces as
    /// [`ProfileError::Escalated`] with the named predicate's
    /// diagnostic, never a guess.
    pub fn validate(&self, tol: Tol) -> Result<ValidatedProfile<T>, ProfileError> {
        let band = Band::new(tol.eps(), tol.k() * tol.eps()).map_err(ProfileError::Band)?;
        // The exact-order band for canonical-start selection (module
        // docs): no representable f64 lies strictly inside it.
        let exact = Band::new(f64::from_bits(1), f64::from_bits(2)).map_err(ProfileError::Band)?;

        if self.loops.is_empty() {
            return Err(ProfileError::EmptyProfile);
        }

        // 1 + 2: arity, then per-segment degeneracy + kind
        // classification.
        let mut loop_segs: Vec<Vec<Seg<T>>> = Vec::with_capacity(self.loops.len());
        for (li, lp) in self.loops.iter().enumerate() {
            loop_segs.push(build_loop_segs(lp, li, band)?);
            // Declared-tangent joints must name vertices of their loop
            // (set semantics — duplicates are harmless, order is not
            // significant; see `ProfileLoop::tangent_joints`). Checked
            // after the loop's structural pass so arity/degeneracy
            // refusals keep their established precedence.
            if let Some(&joint) = lp.tangent_joints.iter().find(|&&j| j >= lp.vertices.len()) {
                return Err(ProfileError::TangentJointOutOfRange {
                    loop_index: li,
                    joint,
                    count: lp.vertices.len(),
                });
            }
        }

        // 3: simplicity — every unordered segment pair, adjacency
        // discounted only at the shared vertex.
        for li in 0..self.loops.len() {
            for si in 0..loop_segs[li].len() {
                // Same-loop partners after si, then every segment of
                // later loops (deterministic order, first error wins).
                for sj in (si + 1)..loop_segs[li].len() {
                    judge_pair(self, &loop_segs, (li, si), (li, sj), band)?;
                }
                for lj in (li + 1)..self.loops.len() {
                    for sj in 0..loop_segs[lj].len() {
                        judge_pair(self, &loop_segs, (li, si), (lj, sj), band)?;
                    }
                }
            }
        }

        // 3b: the declared-tangency discipline (#101) — classify every
        // joint (adjacent-segment junction) and reconcile with the
        // loop's declarations. Runs after simplicity so in-band
        // near-tangency escalates from the pair classification exactly
        // as before this arm existed. Definite-Zero tangency between
        // distinct carriers must be declared; a declaration must be
        // definite-Zero tangency (verified, never trusted).
        for (li, lp) in self.loops.iter().enumerate() {
            judge_joints(lp, &loop_segs[li], li, band)?;
        }

        // Representative point per loop: the lexicographic minimum
        // vertex (rotation/reversal invariant; needed for both
        // containment and the canonical start).
        let mut rep: Vec<Point2<T>> = Vec::with_capacity(self.loops.len());
        for (li, lp) in self.loops.iter().enumerate() {
            let idx = lex_min_index(&lp.vertices, exact, li)?;
            rep.push(lp.vertices[idx].pos);
        }

        // 4: containment forest by ray parity.
        let n = self.loops.len();
        let mut depth = vec![0usize; n];
        for i in 0..n {
            for (j, other) in loop_segs.iter().enumerate() {
                if i != j && point_in_loop(rep[i], other, band, i, j)? {
                    depth[i] += 1;
                }
            }
        }
        let outer_loops: Vec<usize> = (0..n).filter(|&i| depth[i] == 0).collect();
        if outer_loops.len() > 1 {
            return Err(ProfileError::MultipleOuterLoops { outer_loops });
        }
        if let Some(i) = (0..n).find(|&i| depth[i] >= 2) {
            return Err(ProfileError::NestingTooDeep {
                loop_index: i,
                depth: depth[i],
            });
        }
        // With ≥ 1 loop, simplicity makes containment a strict partial
        // order, so a depth-0 loop exists; defensively treat absence as
        // the multi-outer error shape (empty list = inconsistent
        // forest).
        let Some(&outer_index) = outer_loops.first() else {
            return Err(ProfileError::MultipleOuterLoops { outer_loops });
        };

        // 5: canonicalize each loop; assemble outer-first.
        let mut canonical: Vec<Option<ValidatedLoop<T>>> = (0..n).map(|_| None).collect();
        for (li, lp) in self.loops.iter().enumerate() {
            let role = if li == outer_index {
                LoopRole::Outer
            } else {
                LoopRole::Hole
            };
            canonical[li] = Some(canonicalize_loop(
                lp,
                &loop_segs[li],
                role,
                li,
                band,
                exact,
            )?);
        }
        let mut loops = Vec::with_capacity(n);
        if let Some(outer) = canonical[outer_index].take() {
            loops.push(outer);
        }
        for c in canonical.into_iter().flatten() {
            loops.push(c);
        }
        Ok(ValidatedProfile {
            plane: self.plane,
            loops,
        })
    }
}

/// Arity check + segment construction for one input loop.
fn build_loop_segs<T: Decide>(
    lp: &ProfileLoop<T>,
    loop_index: usize,
    band: Band,
) -> Result<Vec<Seg<T>>, ProfileError> {
    let n = lp.vertices.len();
    if n < 2 {
        return Err(ProfileError::TooFewVertices {
            loop_index,
            count: n,
        });
    }
    let mut segs = Vec::with_capacity(n);
    for k in 0..n {
        let a = lp.vertices[k];
        let b = lp.vertices[(k + 1) % n];
        segs.push(build_seg(a.pos, b.pos, a.bulge, band).map_err(|issue| {
            let at = SegmentRef {
                loop_index,
                segment_index: k,
            };
            match issue {
                SegIssue::Degenerate => ProfileError::DegenerateSegment(at),
                SegIssue::NearFull => ProfileError::NearFullArc(at),
                SegIssue::Escalated(source) => ProfileError::Escalated {
                    site: EscalationSite::Segment(at),
                    source,
                },
            }
        })?);
    }
    Ok(segs)
}

/// Judges one segment pair: classifies contacts and applies the
/// adjacency discount (adjacent segments may touch exactly at their
/// shared vertex/vertices; everything else is an error).
fn judge_pair<T: Decide>(
    profile: &Profile<T>,
    loop_segs: &[Vec<Seg<T>>],
    (li, si): (usize, usize),
    (lj, sj): (usize, usize),
    band: Band,
) -> Result<(), ProfileError> {
    let first = SegmentRef {
        loop_index: li,
        segment_index: si,
    };
    let second = SegmentRef {
        loop_index: lj,
        segment_index: sj,
    };
    let site = EscalationSite::SegmentPair(first, second);

    // The legal shared vertices of an adjacent pair (same loop,
    // consecutive mod n). A 2-vertex loop's two segments are adjacent
    // at both vertices.
    let mut shared: Vec<Point2<T>> = Vec::new();
    if li == lj {
        let n = loop_segs[li].len();
        if sj == si + 1 {
            shared.push(profile.loops[li].vertices[(si + 1) % n].pos);
        }
        if si == 0 && sj == n - 1 {
            shared.push(profile.loops[li].vertices[0].pos);
        }
    }

    let outcome = seg::pair_contacts(&loop_segs[li][si], &loop_segs[lj][sj], band)
        .map_err(|source| ProfileError::Escalated { site, source })?;
    let contacts = match outcome {
        PairOutcome::Overlap => {
            return Err(ProfileError::NonSimple {
                first,
                second,
                kind: ContactKind::Overlap,
            });
        }
        PairOutcome::Contacts(contacts) => contacts,
    };
    for contact in contacts {
        match contact.kind {
            CKind::Crossing => {
                return Err(ProfileError::NonSimple {
                    first,
                    second,
                    kind: ContactKind::Crossing,
                });
            }
            CKind::Tangency => {
                return Err(ProfileError::TangentialContact { first, second });
            }
            CKind::Touch => {
                let mut discounted = false;
                for &v in &shared {
                    match seg::coincident("contact_at_shared_vertex", contact.point, v, band)
                        .map_err(|source| ProfileError::Escalated { site, source })?
                    {
                        Sign::Zero => {
                            discounted = true;
                            break;
                        }
                        Sign::Positive | Sign::Negative => {}
                    }
                }
                if !discounted {
                    return Err(ProfileError::NonSimple {
                        first,
                        second,
                        kind: ContactKind::Touch,
                    });
                }
            }
        }
    }
    Ok(())
}

/// The per-loop joint pass of the declared-tangency discipline: joint
/// v is the junction between segment (v − 1 mod n) (arriving) and
/// segment v (leaving) at vertex v. Each joint is classified by
/// [`seg::joint_tangency`] (module docs list the reused carrier
/// predicates) and reconciled with the loop's declarations:
///
/// - `Tangent` undeclared ⇒ [`ProfileError::UndeclaredTangency`];
/// - `Transversal` or `SameCarrier` declared ⇒
///   [`ProfileError::TangencyContradicted`] (a declaration is verified,
///   never trusted — and same-carrier continuation is not a tangency);
/// - in-band / poisoned ⇒ [`ProfileError::Escalated`] at the pair site.
fn judge_joints<T: Decide>(
    lp: &ProfileLoop<T>,
    segs: &[Seg<T>],
    loop_index: usize,
    band: Band,
) -> Result<(), ProfileError> {
    let n = segs.len();
    for joint in 0..n {
        let prev = (joint + n - 1) % n;
        let first = SegmentRef {
            loop_index,
            segment_index: prev,
        };
        let second = SegmentRef {
            loop_index,
            segment_index: joint,
        };
        let declared = lp.tangent_joints.contains(&joint);
        let class = seg::joint_tangency(&segs[prev], &segs[joint], band).map_err(|source| {
            ProfileError::Escalated {
                site: EscalationSite::SegmentPair(first, second),
                source,
            }
        })?;
        match (class, declared) {
            (seg::JointClass::Tangent, false) => {
                return Err(ProfileError::UndeclaredTangency {
                    first,
                    second,
                    joint,
                    suggestion: format!(
                        "{COINCIDENCE_RECOURSE} (declare: add {joint} to loop \
                         {loop_index}'s tangent_joints, or author the corner with the \
                         PATHS .fillet(r) door, which declares by construction)"
                    ),
                });
            }
            (seg::JointClass::Transversal | seg::JointClass::SameCarrier, true) => {
                return Err(ProfileError::TangencyContradicted {
                    first,
                    second,
                    joint,
                    same_carrier: class == seg::JointClass::SameCarrier,
                });
            }
            (seg::JointClass::Tangent, true)
            | (seg::JointClass::Transversal | seg::JointClass::SameCarrier, false) => {}
        }
    }
    Ok(())
}

/// Ray-parity containment of point `p` in the loop with segments
/// `segs`, retrying grazing rays deterministically (module docs).
fn point_in_loop<T: Decide>(
    p: Point2<T>,
    segs: &[Seg<T>],
    band: Band,
    loop_index: usize,
    against_loop: usize,
) -> Result<bool, ProfileError> {
    'ray: for k in 0..N_RAYS {
        // k < 16: exactly representable in f64.
        let angle = RAY_BASE + (k as f64) * RAY_STEP;
        let (s, c) = T::from_f64(angle).sin_cos();
        let dir = Vec2::new(c, s);
        let mut crossings = 0usize;
        for seg in segs {
            match seg::ray_crossings(p, dir, seg, band) {
                Ok(count) => crossings += count,
                Err(seg::Graze) => continue 'ray,
            }
        }
        return Ok(crossings % 2 == 1);
    }
    Err(ProfileError::RayCastingExhausted {
        loop_index,
        against_loop,
    })
}

/// Compares two points in the canonical lexicographic order (least x,
/// then least y) through the exact-order band (module docs) — the
/// **`canonical_order_x`** / **`canonical_order_y`** predicates.
fn lex_less<T: Decide>(p: Point2<T>, q: Point2<T>, exact: Band) -> Result<bool, Indeterminate> {
    match decide("canonical_order_x", Margin::of(p.x - q.x), exact)? {
        Sign::Negative => Ok(true),
        Sign::Positive => Ok(false),
        Sign::Zero => {
            Ok(decide("canonical_order_y", Margin::of(p.y - q.y), exact)? == Sign::Negative)
        }
    }
}

/// The index of a chain's lexicographically minimal vertex (the
/// canonical start; also the containment representative point).
fn lex_min_index<T: Decide>(
    vertices: &[ProfileVertex<T>],
    exact: Band,
    loop_index: usize,
) -> Result<usize, ProfileError> {
    let mut best = 0;
    for i in 1..vertices.len() {
        if lex_less(vertices[i].pos, vertices[best].pos, exact).map_err(|source| {
            ProfileError::Escalated {
                site: EscalationSite::Loop { loop_index },
                source,
            }
        })? {
            best = i;
        }
    }
    Ok(best)
}

/// Orients, rotates, and re-derives one loop into its canonical form.
///
/// Orientation is the **`loop_orientation`** predicate — margin:
/// 2·A/P, the loop's mean width (meters): A is the bulge-polygon signed
/// area (shoelace about the loop's first vertex — the ch. 13
/// translate-to-origin accuracy fix — plus per-arc circular-segment
/// corrections (r²/2)·(θ − sin θ), θ = 4·atan(b)); P is the true
/// perimeter (chords for lines, r·|θ| for arcs). Positive = the input
/// chain runs counterclockwise. The area, a length², acts through the
/// half-perimeter lever arm to become the honest sliver-width margin —
/// a thin loop of width w has 2A/P ≈ w.
fn canonicalize_loop<T: Decide>(
    lp: &ProfileLoop<T>,
    segs: &[Seg<T>],
    role: LoopRole,
    loop_index: usize,
    band: Band,
    exact: Band,
) -> Result<ValidatedLoop<T>, ProfileError> {
    let orientation = loop_orientation(segs, band).map_err(|source| ProfileError::Escalated {
        site: EscalationSite::Loop { loop_index },
        source,
    })?;
    let want_ccw = matches!(role, LoopRole::Outer);
    let chain = match orientation {
        Sign::Zero => return Err(ProfileError::SliverLoop { loop_index }),
        Sign::Positive => {
            if want_ccw {
                lp.clone()
            } else {
                lp.reversed()
            }
        }
        Sign::Negative => {
            if want_ccw {
                lp.reversed()
            } else {
                lp.clone()
            }
        }
    };
    let start = lex_min_index(&chain.vertices, exact, loop_index)?;
    let n = chain.vertices.len();
    let vertices: Vec<ProfileVertex<T>> = (0..n).map(|k| chain.vertices[(start + k) % n]).collect();
    // Declared joints follow their vertex through the rotation
    // (reversal already remapped them in `reversed()`); indices are
    // in range — validated at entry. Sorted + deduplicated: canonical.
    let mut tangent_joints: Vec<usize> = chain
        .tangent_joints
        .iter()
        .map(|&j| (j + n - start) % n)
        .collect();
    tangent_joints.sort_unstable();
    tangent_joints.dedup();

    // Re-derive classified segments on the canonical chain. The
    // classifications are reversal/rotation-symmetric (distances are
    // symmetric, negation is exact), so this cannot fail for a chain
    // whose input just passed — mapped defensively all the same.
    let mut segments = Vec::with_capacity(n);
    for k in 0..n {
        let a = vertices[k];
        let b = vertices[(k + 1) % n];
        let s = build_seg(a.pos, b.pos, a.bulge, band).map_err(|issue| {
            let at = SegmentRef {
                loop_index,
                segment_index: k,
            };
            match issue {
                SegIssue::Degenerate => ProfileError::DegenerateSegment(at),
                SegIssue::NearFull => ProfileError::NearFullArc(at),
                SegIssue::Escalated(source) => ProfileError::Escalated {
                    site: EscalationSite::Segment(at),
                    source,
                },
            }
        })?;
        segments.push(ValidatedSegment {
            start: s.a,
            end: s.b,
            bulge: s.bulge,
            kind: match &s.kind {
                SegKind::Line => SegmentKind::Line,
                SegKind::Arc(g) => SegmentKind::Arc {
                    center: g.center,
                    radius: g.radius,
                    turn: g.turn,
                },
            },
        });
    }
    Ok(ValidatedLoop {
        vertices,
        segments,
        tangent_joints,
        role,
    })
}

/// The `loop_orientation` margin and classification (see
/// [`canonicalize_loop`] for the formula and lever-arm story).
fn loop_orientation<T: Decide>(segs: &[Seg<T>], band: Band) -> Result<Sign, Indeterminate> {
    let origin = segs[0].a;
    let half = T::from_f64(0.5);
    let four = T::from_f64(4.0);
    let mut twice_area = T::zero();
    let mut perimeter = T::zero();
    for s in segs {
        twice_area = twice_area + (s.a - origin).perp_dot(s.b - origin);
        match &s.kind {
            SegKind::Line => {
                perimeter = perimeter + s.len;
            }
            SegKind::Arc(g) => {
                let theta = four * s.bulge.atan();
                // Circular-segment correction: (r²/2)(θ − sin θ),
                // doubled here since we accumulate 2A. `r²` is the tight
                // square, and here it is the tight square of something
                // already known nonnegative: `Seg::radius` is
                // `signed_radius.abs()`, so at `Interval` the enclosure
                // has `lo >= 0` and the plain product's four-corner
                // minimum IS the tight square. The conversion changes no
                // bound at any instantiation — it is here so the file
                // spells a square the one way the rule names, not
                // because this site was wide.
                twice_area = twice_area + g.radius.powi(2) * (theta - theta.sin());
                perimeter = perimeter + g.radius * theta.abs();
            }
        }
    }
    let area = twice_area * half;
    decide(
        "loop_orientation",
        Margin::over_lever(area + area, perimeter),
        band,
    )
}
