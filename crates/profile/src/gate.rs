//! **The question a profile gate asks**, closed (D4 ¶1 (ii)): one
//! [`Gate`] per `k_stats` name this crate decides under. Every
//! escalation the crate raises is minted by this module's funnel from a
//! `Gate` ([`Escalation`]), and each door derives the refusal's subject
//! and ending from that gate, the verdict and the door's site by an
//! exhaustive match. No ending is looked up by predicate name: the name
//! rides the payload and the K stream, never the routing.

use geom_core::{
    Band, COINCIDENCE_RECOURSE, Decide, Decided, EndingKind, Indeterminate, Margin, Sign,
};

use crate::fillet_recourse::{
    FILLET_ANCHOR_TURN_RECOURSE, FILLET_ENCLOSING_RECOURSE, FILLET_FIT_RECOURSE,
    FILLET_LEG_EXTENT_RECOURSE, FILLET_NO_CORNER_RECOURSE, FILLET_OFFSET_LEVER_RECOURSE,
    FILLET_STORED_FORM_INBAND_RECOURSE, FILLET_TURN_INBAND_RECOURSE,
};
use crate::validate::{
    ARC_READ_BACK_UNDECIDED_RECOURSE, ARC_SCENE_RESOLUTION_RECOURSE, JUNCTION_TANGENT_LEVER,
};

/// What the fillet construction's gates say they were deciding: the
/// corner the caller asked to round.
const FILLET_CORNER_SUBJECT: &str = "the fillet at this corner";

/// The lever of an undecided declared straight continuation.
const CONTINUATION_TARGET_LEVER: &str = "move the target onto the run's line";

/// The lever of an undecided authored leg extent.
const LEG_LENGTH_LEVER: &str = "author a longer leg";

/// The lever of an undecided declared seam arrival: the declaration
/// rides the target, so the geometry is what moves.
const SEAM_ARRIVAL_LEVER: &str =
    "move the geometry so the two directions agree (the declaration is the target)";

/// Declares [`Gate`] from one table, so each gate's name, ending and
/// subject are written once, side by side, and [`Gate::ALL`] holds every
/// variant by construction.
macro_rules! gates {
    ($($(#[$doc:meta])* $variant:ident => $name:expr, $kind:ident($lever:expr), $subject:expr;)*) => {
        /// **Every decision this crate makes**, with the `k_stats` name it
        /// is metered under, how its undecided refusal ends, and what it
        /// decides in words.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum Gate {
            $($(#[$doc])* $variant,)*
        }

        impl Gate {
            /// Every gate, once.
            pub const ALL: &[Self] = &[$(Self::$variant),*];

            /// The `k_stats` name the gate is metered under.
            #[must_use]
            pub const fn predicate(self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)*
                }
            }

            /// How its undecided refusal ends, with its own lever.
            #[must_use]
            pub const fn kind(self) -> EndingKind {
                match self {
                    $(Self::$variant => EndingKind::$kind($lever),)*
                }
            }

            /// What it decides, as a clause that stands alone, for the
            /// door to follow with the undecided verdict.
            #[must_use]
            pub const fn subject(self) -> &'static str {
                match self {
                    $(Self::$variant => $subject,)*
                }
            }
        }
    };
}

gates! {
    /// Two arc apexes told apart by their separation.
    ArcApexIdentity => "arc_apex_identity", Sized(COINCIDENCE_RECOURSE), "whether two arc apexes are one point";
    /// The magnitude an arc's consistency difference was read at, against the one where its f64 rounding reaches the band.
    ArcCarrierResolution => "arc_carrier_resolution", Lever(ARC_SCENE_RESOLUTION_RECOURSE), "whether the scene can read an arc's difference from its vertices at its magnitude";
    /// How far an arc stops short of a full circle.
    ArcDiameterClearance => "arc_diameter_clearance", Sized(COINCIDENCE_RECOURSE), "whether an arc stops short of a full circle";
    /// How far a table arc's start, turned by its sweep, lands from its end.
    ArcLanding => "arc_landing", Sized(COINCIDENCE_RECOURSE), "whether an arc's sweep carries its start onto its end";
    /// An arc's span, as the clearance between its chord's reach and its apex.
    ArcSpan => "arc_span", Sized(COINCIDENCE_RECOURSE), "whether a point falls inside an arc's span";
    /// How far a table arc's start lies off its carrier circle.
    ArcStartOnCarrier => "arc_start_on_carrier", Sized(COINCIDENCE_RECOURSE), "whether an arc's start lies on its carrier circle";
    /// A table arc's arc length at its radius, against zero.
    ArcSweepLength => "arc_sweep_length", Lever(COINCIDENCE_RECOURSE), "whether an arc sweeps any length";
    /// A table arc's gap to a full turn, at its radius.
    ArcSweepRange => "arc_sweep_range", Lever(COINCIDENCE_RECOURSE), "whether an arc's sweep stops short of a full turn, or makes one";
    /// Two points ordered by their x difference, at the exact-order band.
    CanonicalOrderX => "canonical_order_x", Lever(COINCIDENCE_RECOURSE), "which of two points comes first along x";
    /// Two points ordered by their y difference, at the exact-order band.
    CanonicalOrderY => "canonical_order_y", Lever(COINCIDENCE_RECOURSE), "which of two points comes first along y";
    /// The depth of the chord two nearly internally tangent circles cut, at its midpoint.
    CarrierCirclesChord => "carrier_circles_chord", Sized(COINCIDENCE_RECOURSE), "whether two circles touching from inside cut a chord";
    /// Two circles' centre separation less the sum of their radii.
    CarrierCirclesExternal => "carrier_circles_external", Sized(COINCIDENCE_RECOURSE), "whether two circles touch from outside";
    /// Two circles told apart by their centre separation plus their radius difference.
    CarrierCirclesIdentity => "carrier_circles_identity", Sized(COINCIDENCE_RECOURSE), "whether two circles are one circle";
    /// Two circles' centre separation less the difference of their radii.
    CarrierCirclesInternal => "carrier_circles_internal", Sized(COINCIDENCE_RECOURSE), "whether two circles touch from inside";
    /// A line's clearance from a circle: the radius against the line's distance from the centre.
    CarrierLineCircle => "carrier_line_circle", Sized(COINCIDENCE_RECOURSE), "whether a line and a circle cross, touch or miss";
    /// A point's lateral distance from a chord's line.
    ChordSide => "chord_side", Sized(COINCIDENCE_RECOURSE), "which side of a chord a point lies on";
    /// A segment end's distance from a circle's centre less its radius, charged.
    CircleEndClear => "circle_end_clear", Sized(COINCIDENCE_RECOURSE), "whether a segment's end lies definitely off a circle";
    /// How far a computed extreme of an arc's circle lies off its span, charged.
    CircleExtremePastArc => "circle_extreme_past_arc", Sized(COINCIDENCE_RECOURSE), "whether an arc's point nearest or farthest from another circle's centre lies off it";
    /// How far a circle centre's computed foot lies off a segment's span, charged.
    CircleFootPastSegment => "circle_foot_past_segment", Sized(COINCIDENCE_RECOURSE), "whether a circle's centre's foot on a line lies off the line's segment";
    /// Which side of an arc's carrier circle a point lies on, as its distance from the centre less the radius.
    CircleSide => "circle_side", Sized(COINCIDENCE_RECOURSE), "which side of an arc's circle a point lies on";
    /// The overlap of two collinear segments along their shared carrier.
    CollinearOverlap => "collinear_overlap", Sized(COINCIDENCE_RECOURSE), "whether two segments on one line overlap";
    /// A contact point told apart from a loop vertex by their separation.
    ContactAtSharedVertex => "contact_at_shared_vertex", Sized(COINCIDENCE_RECOURSE), "whether a contact point is a loop vertex";
    /// A fillet leg's extent, metered against zero.
    FilletCornerArm => "fillet_corner_arm", Sized(FILLET_LEG_EXTENT_RECOURSE), FILLET_CORNER_SUBJECT;
    /// The turn between a fillet corner's two legs, levered by the shorter leg's extent.
    FilletCornerTurn => "fillet_corner_turn", Sized(FILLET_TURN_INBAND_RECOURSE), FILLET_CORNER_SUBJECT;
    /// The offset radius rho = R - sigma tau r of a leg's carrier, whose sign says whether the fillet swallows it.
    FilletEnclosingCarrier => "fillet_enclosing_carrier", Sized(FILLET_ENCLOSING_RECOURSE), FILLET_CORNER_SUBJECT;
    /// A leg's length less the fillet's setback on it — the run the fillet leaves on that leg — at the run's linear band.
    FilletLegFit => "fillet_leg_fit", Sized(FILLET_FIT_RECOURSE), FILLET_CORNER_SUBJECT;
    /// A tangent point's setback along its leg's carrier, at the exact-order band.
    FilletLegReach => "fillet_leg_reach", Lever(FILLET_NO_CORNER_RECOURSE), FILLET_CORNER_SUBJECT;
    /// Whether two offset circles meet from outside.
    FilletOffsetCirclesExternal => "fillet_offset_circles_external", Sized(FILLET_NO_CORNER_RECOURSE), FILLET_CORNER_SUBJECT;
    /// Whether two offset circles meet from inside.
    FilletOffsetCirclesInternal => "fillet_offset_circles_internal", Sized(FILLET_NO_CORNER_RECOURSE), FILLET_CORNER_SUBJECT;
    /// An offset lever's length against the least that places a tangent point within the
    /// tolerance. The least lever grows as ε shrinks (`∝ 1/ε`), so a smaller tolerance decides the
    /// margin against the request: the lever alone.
    FilletOffsetLever => "fillet_offset_lever", Lever(FILLET_OFFSET_LEVER_RECOURSE), FILLET_CORNER_SUBJECT;
    /// Whether an offset line meets an offset circle.
    FilletOffsetLineCircle => "fillet_offset_line_circle", Sized(FILLET_NO_CORNER_RECOURSE), FILLET_CORNER_SUBJECT;
    /// How far a full turn reaches from its vertex, levered by its diameter.
    FullTurnReach => "full_turn_reach", Sized(COINCIDENCE_RECOURSE), "whether a full turn reaches round its carrier";
    /// A full turn's signed reach from its vertex, whose sign is its turn.
    FullTurnSense => "full_turn_sense", Lever(COINCIDENCE_RECOURSE), "which way a full turn runs";
    /// How far inside a segment a point falls, from whichever end is nearer.
    LineSpan => "line_span", Sized(COINCIDENCE_RECOURSE), "whether a point falls inside a segment";
    /// A loop's signed area, levered by its perimeter.
    LoopOrientation => "loop_orientation", Lever(COINCIDENCE_RECOURSE), "which way round a loop runs";
    /// Which end of a segment a point projects nearer, as its offset along the chord from the chord's bisector.
    NearerEnd => "nearer_end", Sized(COINCIDENCE_RECOURSE), "which end of a segment a point lies nearer";
    /// How much nearer a segment end lies to the candidate contact it settles than to the carriers' other candidate.
    OwnCandidate => "own_candidate", Sized(COINCIDENCE_RECOURSE), "which of two candidate contacts a segment end lies nearer";
    /// A constructed arc's length, signed by the turn sense it was authored to: negative is the other arc between its ends, read back at a point scalar.
    PathArcBranch => "path_arc_branch", Lever(ARC_READ_BACK_UNDECIDED_RECOURSE), "whether an arc the path built turns the way it was authored to";
    /// The lesser of the authored data's headroom below the magnitude whose rounding reaches the band and the miss's clearance of the read's own rounding: a read-back miss's defect gate.
    PathArcBranchResolution => "path_arc_branch_resolution", Lever(ARC_SCENE_RESOLUTION_RECOURSE), "whether an arc's wrong branch is a defect the scene resolves";
    /// An authored bulge told apart from zero.
    PathArcBulge => "path_arc_bulge", Lever(COINCIDENCE_RECOURSE), "whether an authored bulge is zero";
    /// An authored centre's two radii told apart.
    PathArcCenterEquidistant => "path_arc_center_equidistant", Sized(COINCIDENCE_RECOURSE), "whether an authored centre is as far from one end of its arc as from the other";
    /// An authored radius metered against zero.
    PathArcCenterRadius => "path_arc_center_radius", Sized(COINCIDENCE_RECOURSE), "whether an authored radius is zero";
    /// An authored chord metered against zero.
    PathArcChord => "path_arc_chord", Sized(COINCIDENCE_RECOURSE), "whether an authored chord has any length";
    /// How far a constructed arc's start, turned by its sweep, lands from its end vertex (for an arc decided inline, from that vertex moved onto its circle), read back at a point scalar.
    PathArcLanding => "path_arc_landing", Lever(ARC_READ_BACK_UNDECIDED_RECOURSE), "whether an arc the path built lands on its end vertex";
    /// The lesser of the authored data's headroom below the magnitude whose rounding reaches the band and the miss's clearance of the read's own rounding: a read-back miss's defect gate.
    PathArcLandingResolution => "path_arc_landing_resolution", Lever(ARC_SCENE_RESOLUTION_RECOURSE), "whether an arc's miss of its end vertex is a defect the scene resolves";
    /// How far a constructed arc's start vertex lies off its stored carrier, read back at a point scalar.
    PathArcStartOnCarrier => "path_arc_start_on_carrier", Lever(ARC_READ_BACK_UNDECIDED_RECOURSE), "whether an arc the path built starts on its own carrier";
    /// The lesser of the authored data's headroom below the magnitude whose rounding reaches the band and the miss's clearance of the read's own rounding: a read-back miss's defect gate.
    PathArcStartOnCarrierResolution => "path_arc_start_on_carrier_resolution", Lever(ARC_SCENE_RESOLUTION_RECOURSE), "whether an arc's start off its carrier is a defect the scene resolves";
    /// An authored sweep's arc length at its radius, against zero.
    PathArcSweep => "path_arc_sweep", Lever(COINCIDENCE_RECOURSE), "whether an authored sweep's arc length is zero";
    /// An authored sweep's arc length short of a full turn.
    PathArcSweepFull => "path_arc_sweep_full", Lever(COINCIDENCE_RECOURSE), "whether an authored sweep is short of a full turn";
    /// How far a constructed Via arc's carrier lies off the through-point it was authored by, read back at a point scalar.
    PathArcThroughPoint => "path_arc_through_point", Lever(ARC_READ_BACK_UNDECIDED_RECOURSE), "whether an arc the path built passes through the point it was authored through";
    /// The lesser of the authored data's headroom below the magnitude whose rounding reaches the band and the miss's clearance of the read's own rounding: a read-back miss's defect gate.
    PathArcThroughPointResolution => "path_arc_through_point_resolution", Lever(ARC_SCENE_RESOLUTION_RECOURSE), "whether an arc's miss of its through-point is a defect the scene resolves";
    /// A via point's lateral offset from the chord it bulges.
    PathArcViaOffset => "path_arc_via_offset", Sized(COINCIDENCE_RECOURSE), "whether a via point lies off the chord it bulges";
    /// Whether a ray and a circle meet at all: the circle's radius against the ray's perpendicular distance from its centre.
    PathCarrierMeet => "path_carrier_meet", Sized(COINCIDENCE_RECOURSE), "whether a ray meets a circle";
    /// A circle's radius metered against zero.
    PathCircleRadius => "path_circle_radius", Sized(COINCIDENCE_RECOURSE), "whether a circle's radius is zero";
    /// A declared target's lateral miss from the tip's direction.
    PathCollinearTarget => "path_collinear_target", Sized(COINCIDENCE_RECOURSE), "whether a declared target lies on the tip's line";
    /// A declared continuation target's lateral miss from its run's line.
    PathContinuationTargetOffset => "path_continuation_target_offset", Sized(CONTINUATION_TARGET_LEVER), "whether a declared straight continuation's target lies on its run's line";
    /// Where the corner sits along a STRAIGHT side's carrier — forward of the incoming anchor, or back from the arrival's.
    PathCornerAdvance => "path_corner_advance", Sized(COINCIDENCE_RECOURSE), "whether the corner lies ahead of its anchor on a straight side";
    /// The same window on a CIRCULAR carrier: the swept extent from the anchor forward to the corner, levered to metres.
    PathCornerAdvanceArc => "path_corner_advance_arc", Sized(COINCIDENCE_RECOURSE), "whether the corner lies ahead of its anchor on an arc side";
    /// The swept extent from the corner forward to a circular arrival's anchor, levered to metres.
    PathCornerReachArc => "path_corner_reach_arc", Sized(COINCIDENCE_RECOURSE), "whether the corner lies short of an arc arrival's anchor";
    /// The turn BETWEEN the two carriers at the corner, levered by the anchor separation.
    PathCornerTurn => "path_corner_turn", Sized(FILLET_ANCHOR_TURN_RECOURSE), "whether the two carriers turn at the corner";
    /// A director's length metered against zero.
    PathDirectorNorm => "path_director_norm", Sized(COINCIDENCE_RECOURSE), geom_core::DIRECTION_LENGTH_SUBJECT;
    /// The same turn read off a fillet's STORED joints by the door's stored-form re-read (`seg::joint_tangency` under this name): its levers are the stored form's, not `.tangent()`.
    PathFilletJointTurn => "path_fillet_joint_turn", Lever(FILLET_STORED_FORM_INBAND_RECOURSE), "whether a fillet's stored joint turns";
    /// An authored fillet radius metered against zero.
    PathFilletRadius => "path_fillet_radius", Sized(COINCIDENCE_RECOURSE), "whether an authored fillet radius is zero";
    /// The cosine of a junction's turn, levered by the arriving arm; one question at both doors (`seg::junction_reverses`), asked of a junction already tangent, so neither outcome is something `.tangent()` would change, and a raw-authored loop has no path verbs.
    PathJunctionSide => "path_junction_side", Lever(COINCIDENCE_RECOURSE), "whether the path carries straight on at this junction or doubles back into a cusp";
    /// The sine of a junction's turn at the shorter leg's arm: the path door's, and validation's joint pass's (`seg::joint_tangency`).
    PathJunctionTurn => "path_junction_turn", Lever(JUNCTION_TANGENT_LEVER), "whether the path turns at this junction";
    /// An authored leg's extent metered against zero.
    PathLegLength => "path_leg_length", Sized(LEG_LENGTH_LEVER), "whether an authored leg has any length";
    /// A rectangle's width or height metered against zero.
    PathRectangleSide => "path_rectangle_side", Sized(COINCIDENCE_RECOURSE), "whether a rectangle's side is zero or negative";
    /// An emitted segment told apart from a fillet's arrival carrier: its end's lateral miss from the arrival ray and its advance along it, or its end's and arc midpoint's radial misses from the arrival circle.
    PathRunOutCarrier => "path_run_out_carrier", Sized(COINCIDENCE_RECOURSE), "whether an emitted segment lands on a fillet's arrival carrier";
    /// The lever arm the seam arrival's own turn and side gates are metered through.
    PathSeamArrivalLever => "path_seam_arrival_lever", Sized(COINCIDENCE_RECOURSE), "whether the seam arrival has a leg long enough to measure";
    /// The cosine of the seam arrival's turn, levered by its arm. Decided at the path door alone:
    /// the declaration rides the target, so this row and the next name the geometry's lever.
    PathSeamArrivalSide => "path_seam_arrival_side", Lever(SEAM_ARRIVAL_LEVER), "whether the seam arrival carries straight on or doubles back into a cusp";
    /// The sine of the seam arrival's turn, levered by its arm.
    PathSeamArrivalTurn => "path_seam_arrival_turn", Lever(SEAM_ARRIVAL_LEVER), "whether the seam arrival turns";
    /// How far along a containment ray a crossing falls.
    RayAdvance => "ray_advance", Sized(COINCIDENCE_RECOURSE), "whether a crossing lies ahead on a containment ray";
    /// Which side of a containment ray a point falls on.
    RaySide => "ray_side", Sized(COINCIDENCE_RECOURSE), "which side of a containment ray a point lies on";
    /// How far apart two straight segments' projections lie along the first one's line.
    SegmentGap => "segment_gap", Sized(COINCIDENCE_RECOURSE), "whether two straight segments lie apart along one of them";
    /// A segment's bow: its quarter-angle tangent, levered by its length.
    SegmentStraightness => "segment_straightness", Lever(COINCIDENCE_RECOURSE), "whether a segment is straight or an arc";
    /// A segment's length.
    VertexSeparation => "vertex_separation", Sized(COINCIDENCE_RECOURSE), "whether a segment has any length";
}

/// What an escalated decision of this crate was deciding, by its
/// `k_stats` name: the words a report that holds only the name (a flip
/// report, `editor-core`'s one lookup over every owner's words) states.
/// No door of this crate reads it; each derives its subject from the
/// [`Gate`] its escalation carries.
#[must_use]
pub fn decision_subject(predicate: &str) -> Option<&'static str> {
    Gate::ALL
        .iter()
        .find(|gate| gate.predicate() == predicate)
        .map(|gate| gate.subject())
}

/// **An escalation, with the gate that raised it.** Minted only by this
/// module's funnel, so the gate is the one whose name the escalation's
/// payload carries.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Escalation {
    gate: Gate,
    source: Indeterminate,
}

impl Escalation {
    /// The gate that escalated.
    pub(crate) const fn gate(&self) -> Gate {
        self.gate
    }

    /// The predicate-layer escalation.
    pub(crate) const fn source(&self) -> &Indeterminate {
        &self.source
    }

    /// The pair, for a public error's fields.
    pub(crate) const fn parts(self) -> (Gate, Indeterminate) {
        (self.gate, self.source)
    }
}

impl Gate {
    /// **The funnel for every other `geom-core` door that decides under a
    /// name**: `decide` is handed this gate's name, and an escalation it
    /// returns is this gate's. One whose payload names another predicate
    /// is a kernel bug, and panics.
    pub(crate) fn read<R>(
        self,
        decide: impl FnOnce(&'static str) -> Result<R, Indeterminate>,
    ) -> Result<R, Escalation> {
        decide(self.predicate()).map_err(|source| {
            assert_eq!(
                source.predicate,
                Some(self.predicate()),
                "an escalation read under {self:?} names another predicate"
            );
            Escalation { gate: self, source }
        })
    }
}

/// [`geom_core::k_stats::decide`] under `gate`'s name.
pub(crate) fn decide<T: Decide>(
    gate: Gate,
    margin: Margin<T>,
    band: Band,
) -> Result<Sign, Escalation> {
    gate.read(|name| geom_core::k_stats::decide(name, margin, band))
}

/// [`geom_core::k_stats::decide_passing`] under `gate`'s name.
pub(crate) fn decide_passing<T: Decide>(
    gate: Gate,
    margin: Margin<T>,
    band: Band,
    passes: geom_core::SizedPass,
) -> Result<Sign, Escalation> {
    gate.read(|name| geom_core::k_stats::decide_passing(name, margin, band, passes))
}

/// [`geom_core::k_stats::prove`] under `gate`'s name: a proof's
/// inequality read exactly (the exact-order band, validate module docs),
/// with no `Band` parameter to pass a wider one by mistake.
pub(crate) fn prove<T: Decide>(gate: Gate, margin: Margin<T>) -> Result<Sign, Escalation> {
    gate.read(|name| geom_core::k_stats::prove(name, margin))
}

/// [`geom_core::k_stats::decide_reported`] under `gate`'s name.
pub(crate) fn decide_reported<T: Decide>(
    gate: Gate,
    margin: Margin<T>,
    band: Band,
) -> Result<Decided, Escalation> {
    gate.read(|name| geom_core::k_stats::decide_reported(name, margin, band))
}

/// [`geom_core::k_stats::check_unlogged`] under `gate`'s name.
pub(crate) fn check_unlogged<T: Decide>(
    gate: Gate,
    margin: T,
    band: Band,
    ledger_row: &'static str,
) -> Result<Sign, Escalation> {
    gate.read(|name| geom_core::k_stats::check_unlogged(name, margin, band, ledger_row))
}

/// Two readings of `gate` that disagree, escalated through
/// [`geom_core::k_stats::straddle`] under its name.
pub(crate) fn straddle(gate: Gate, lower: Decided, upper: Decided, band: Band) -> Escalation {
    Escalation {
        gate,
        source: geom_core::k_stats::straddle(gate.predicate(), lower, upper, band),
    }
}
