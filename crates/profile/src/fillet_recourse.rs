//! **What the fillet construction's gates tell their caller.**
//!
//! Each sentence is a gate's own lever (D4 ¶1 (i)): [`crate::Gate`]'s
//! table names it beside the gate, and the ending derived from the gate
//! and its verdict ([`crate::Gate::kind`]) adds the conditional
//! tolerance arm where a smaller ε decides the margin for the request.
//! So no sentence here names the tolerance: where it is a lever, its
//! value comes from the margin. At the two exact-order gates
//! (`fillet_leg_fit`, `fillet_leg_reach`, classified against
//! `Band::least()`) it is none, and so it is at `fillet_offset_lever`,
//! whose least lever grows as ε shrinks.
//!
//! The definite sibling refusals spell their own recourse
//! ([`crate::PathError::NoCornerOfPair`],
//! [`crate::PathError::FilletOffsetLeverTooShort`]), except where a
//! sentence below says it is read by one.

/// The lever of an undecided corner **turn** (`fillet_corner_turn`).
///
/// The margin is levered, `sin φ · arm` with the shorter leg's extent as
/// the arm (`sugar::arc_fillet_corner`'s gate (2)), so the band admits
/// two situations and the sentence names both: a degenerate angle (the
/// legs run smoothly into each other, or double back into a cusp), and a
/// real angle on a leg a few ε long, whose lever is the leg's extent.
/// Pinned at both by `profile/tests/fillet_recourse_followability.rs`'s
/// `the_turn_in_band_recourse_names_both_of_its_situations`. The gate's
/// definite arm, `PathNoCornerReason::TurnWithinTolerance` on the shorter
/// leg, ends in it too.
/// Gates: `fillet_corner_turn`.
#[cfg_attr(
    not(any(test, feature = "test-support")),
    allow(
        unreachable_pub,
        reason = "re-exported by the crate root only under \
     `test-support`; interior in every other build"
    )
)]
pub const FILLET_TURN_INBAND_RECOURSE: &str = "if the angle is real, give the shorter leg a longer extent; if the legs \
     continue smoothly, construct the tangency (.tangent()); if they exactly double back, the \
     cusp is refused; otherwise move the geometry";

/// The lever of a straight pair's corner turn (`path_corner_turn`),
/// undecided or zero at tolerance
/// ([`crate::PathNoCornerReason::TurnWithinTolerance`] on the anchors'
/// separation). The margin is levered by the separation of the incoming
/// ray's start from the arrival anchor, and at a real angle it is short
/// only when the corner sits close to BOTH anchors: the corner has to lie
/// definitely ahead of the ray's start and behind the arrival anchor, so
/// moving one anchor alone leaves the other window refusing.
/// Gates: `path_corner_turn`.
#[cfg_attr(
    not(any(test, feature = "test-support")),
    allow(
        unreachable_pub,
        reason = "re-exported by the crate root only under \
     `test-support`; interior in every other build"
    )
)]
pub const FILLET_ANCHOR_TURN_RECOURSE: &str = "if the angle is real, move both anchors away \
     from the corner; if the rays continue smoothly, construct the tangency (.tangent()); if \
     they exactly double back, the cusp is refused; otherwise move the geometry";

/// The lever where no corner of the requested radius may exist: the
/// three offset-carrier clearances and the corner-side reach gate
/// (`fillet_offset_line_circle`, `fillet_offset_circles_external`,
/// `fillet_offset_circles_internal`, `fillet_leg_reach`). Followed to a
/// build by `the_no_corner_recourse_reduces_to_a_radius_that_builds` and
/// `the_offset_clearance_recourse_reaches_the_caller_and_reduces`.
/// Gates: `fillet_offset_line_circle`, `fillet_offset_circles_external`, `fillet_offset_circles_internal`, `fillet_leg_reach`.
#[cfg_attr(
    not(any(test, feature = "test-support")),
    allow(
        unreachable_pub,
        reason = "re-exported by the crate root only under \
     `test-support`; interior in every other build"
    )
)]
pub const FILLET_NO_CORNER_RECOURSE: &str =
    "use a smaller radius, or move the legs so a circle of that radius can sit in the corner";

/// The lever of an undecided offset lever (`fillet_offset_lever`), with
/// its direction and its bound.
///
/// ρ = R − σ·τ·r collapses as the fillet radius nears the leg's own
/// carrier radius on the side the corner turns toward, so the direction
/// is away from it. The threshold `C·u·R₂·scale²/(d·ε)`
/// (`sugar::ArcCarrier::offset_circles`) carries ρ² in its `scale²`, so
/// where ρ dominates the corner's magnitude a larger move refuses again,
/// definitely, and the sentence says so. Reached in band through the
/// public door by
/// `the_offset_lever_recourse_reaches_the_caller_at_its_own_site` and
/// the two reviewer rows it cites.
/// Gates: `fillet_offset_lever`.
#[cfg_attr(
    not(any(test, feature = "test-support")),
    allow(
        unreachable_pub,
        reason = "re-exported by the crate root only under \
     `test-support`; interior in every other build"
    )
)]
pub const FILLET_OFFSET_LEVER_RECOURSE: &str = "move the fillet radius away from that leg's carrier radius (a large move can \
     refuse again), or bring the corner's carriers closer together";

/// The lever where the fillet radius sits within the band of a leg's
/// own carrier radius (`fillet_enclosing_carrier`): the sign of ρ, and
/// with it whether the fillet would swallow that carrier, is below the
/// tolerance. Followed to a build by
/// `the_enclosing_recourse_endorses_a_bound_that_builds`.
/// Gates: `fillet_enclosing_carrier`.
#[cfg_attr(
    not(any(test, feature = "test-support")),
    allow(
        unreachable_pub,
        reason = "re-exported by the crate root only under \
     `test-support`; interior in every other build"
    )
)]
pub const FILLET_ENCLOSING_RECOURSE: &str = "the fillet radius is too close to the leg's own carrier radius to tell whether the \
     fillet would swallow that carrier: move the radius well below the carrier radius";

/// The lever of the run a fillet leaves on a leg (`fillet_leg_fit`),
/// read on the run's linear band: in band the run is neither a segment
/// nor none, on either side of zero, and lengthening it decides both.
/// Lengthening the run is followed by
/// `the_fit_recourse_is_followed_by_a_smaller_radius_and_by_longer_legs`.
/// Gates: `fillet_leg_fit`.
#[cfg_attr(
    not(any(test, feature = "test-support")),
    allow(
        unreachable_pub,
        reason = "re-exported by the crate root only under \
     `test-support`; interior in every other build"
    )
)]
pub const FILLET_FIT_RECOURSE: &str = "the run the fillet leaves on a leg is too short to tell from none: use a smaller radius or \
     longer legs";

/// The lever of a fillet leg with no extent to round against
/// (`fillet_corner_arm`). Followed to a build by
/// `the_leg_extent_recourse_is_followed_by_giving_the_leg_an_extent`.
/// Gates: `fillet_corner_arm`.
#[cfg_attr(
    not(any(test, feature = "test-support")),
    allow(
        unreachable_pub,
        reason = "re-exported by the crate root only under \
     `test-support`; interior in every other build"
    )
)]
pub const FILLET_LEG_EXTENT_RECOURSE: &str = "give the leg a real extent (a non-degenerate chord, or an arc carrier with a positive \
     radius and a non-zero sweep) — a leg with no extent has no direction to be tangent to";

/// **The recourse for a fillet arc too shallow to be STORED as an arc**
/// ([`crate::PathError::FilletArcFlattenedInStorage`]).
///
/// Validation reads a stored arc through `segment_straightness`, whose
/// margin is the sagitta `r(1 − cos(θ/2)) ≈ r·θ²/8`; below the run's ε
/// the arc reads as a line. A larger turn or a larger radius moves the
/// sagitta; the radius has the ceiling
/// [`FILLET_SCENE_RESOLUTION_RECOURSE`] names, and at a tight ε and a
/// tiny turn the two bounds cross, so the radius is offered
/// conditionally and dropping the fillet always. Followed by
/// `the_flattened_recourse_is_followed_by_a_larger_turn_and_a_larger_radius`.
#[cfg_attr(
    not(any(test, feature = "test-support")),
    allow(
        unreachable_pub,
        reason = "re-exported by the crate root only under \
     `test-support`; interior in every other build"
    )
)]
pub const FILLET_FLATTENED_RECOURSE: &str = "turn the corner further, or use a LARGER radius while the scene still resolves \
     one; if the corner is too shallow for both, drop the fillet and leave the corner sharp";

/// **The recourse for a fillet whose carrier the scene cannot resolve**
/// ([`crate::PathError::FilletCarrierBelowSceneResolution`]).
///
/// A joint's turn is read off its legs' headings, each a difference of
/// coordinates at the scene's magnitude `M` over the length it is taken
/// across, so levered by the joint's arm it resolves only to about
/// `M·2⁻⁵²`; when that is coarser than ε the joint cannot be classified
/// whatever the turn is. The lever is the magnitude. Followed by
/// `the_scene_resolution_recourse_is_followed_by_a_smaller_radius_and_a_nearer_scene`.
#[cfg_attr(
    not(any(test, feature = "test-support")),
    allow(
        unreachable_pub,
        reason = "re-exported by the crate root only under \
     `test-support`; interior in every other build"
    )
)]
pub const FILLET_SCENE_RESOLUTION_RECOURSE: &str = "move the geometry nearer the origin, keep the radius no larger than its distance from the \
     origin, or leave the corner sharp";

/// **The lever when the stored form's own read is undecided** — the
/// undecided twin of the two above, read at the door's stored-form
/// site ([`crate::EscalatedAt::StoredForm`]) whatever gate escalated
/// there. The run has not said which way the stored form reads, so the
/// sentence names both things that can be true of it and the lever that
/// works for either. Followed by
/// `the_stored_form_inband_recourse_is_followed_by_dropping_the_fillet`.
#[cfg_attr(
    not(any(test, feature = "test-support")),
    allow(
        unreachable_pub,
        reason = "re-exported by the crate root only under \
     `test-support`; interior in every other build"
    )
)]
pub const FILLET_STORED_FORM_INBAND_RECOURSE: &str = "drop the fillet; or, if the arc is too shallow to store, turn the corner further or \
     enlarge the radius; or, if it is too fine for the scene, move nearer the origin";
