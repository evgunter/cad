//! **The roster: every site where this crate decides a predicate name
//! is one the census can read.**
//!
//! The reader is `test_utils::source::predicate_census`, the tree's one
//! home for this walk, and the checks every roster makes of it are
//! `test_utils::source::roster_violations` — the same harness topo's,
//! sweep's and profile's rosters run. It reads a name at a `decide*`
//! call, or out of a carrier declared in [`CARRIERS`]: a `const` the
//! name is spelled in, or a call whose literal arguments are names.
//! **What it cannot read it reports**, and each is a red row here:
//!
//! - a site whose name is a `const` this file does not declare;
//! - a spelling it does not know, and a file it does not walk.
//!
//! A declared carrier that matches nothing reads nothing and is not a
//! red.
//!
//! A funnel that hands on the name its caller gives it is spelled so
//! that no other call shares its token — `*_as`, "decided as" the row
//! it is handed — and declared in [`CARRIERS`], so its callers' names
//! are read. A funnel no carrier declares is not read here; a test or
//! fixture naming a name behind it is red in the tree-wide reference
//! row until it is declared.
//!
//! **No door table yet.** topo's roster also says which door decides
//! each name, by its prefix; geom-brep's names do not yet carry one,
//! and the table is
//! `work/onecopy/geom-brep-predicate-names-carry-no-door.md`. What this
//! row holds today is that the census reads every site it meets, which
//! is what the tree-wide coverage row
//! (`test-utils/tests/predicate_name_coverage.rs`) resolves every
//! referenced name against.
//!
//! **When it runs.** The walk takes seconds, so this row is in the slow
//! set: a PR runs it only when the PR names it, and the nightly runs it
//! every night.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source::{NameCarrier, crate_dir, predicate_census, roster_violations};

/// The carriers that hand a name to a funnel from somewhere other than
/// the call: the `const`s names are spelled in, then the calls (row
/// structs and `*_as` forwarders) whose literal arguments are names.
/// An entry that matches nothing in `src` now is not a red: it reads
/// nothing and excuses nothing, and it goes stale silently.
const CARRIERS: &[NameCarrier] = &[
    NameCarrier::Const("SQUARE_FOLD_HALF"),
    NameCarrier::Const("OVAL_SIDE"),
    NameCarrier::Const("SQUARE_EDGE_OVER"),
    NameCarrier::Const("SPHERE_NEAR"),
    NameCarrier::Const("SPHERE_FAR"),
    NameCarrier::Const("PC_PARALLEL_GAP"),
    NameCarrier::Const("EXTERNAL"),
    NameCarrier::Const("INTERNAL"),
    NameCarrier::Const("TANGENCY"),
    // The closed decision types' name tables (`fn name/floor/straddle/
    // row/predicate`), read as the census reads them. A type whose
    // `fn name` spells a label (`PcurveKind`, `SsiLimb`, `FloorKind`) is
    // no name table and is not declared.
    NameCarrier::Table("CsDecision"),
    NameCarrier::Table("CcDecision"),
    NameCarrier::Table("Row"),
    NameCarrier::Table("TorusDatum"),
    NameCarrier::Table("Meter"),
    NameCarrier::Const("SQUARE_ORDER"),
    NameCarrier::Const("RADII"),
    // The cone × torus pose's apex rows (`pair_section/cone_torus.rs`).
    NameCarrier::Const("APEX_ON_TORUS"),
    NameCarrier::Const("APEX_FORM"),
    NameCarrier::Const("POSE_AXES_PARALLEL"),
    NameCarrier::Const("POSE_AXIS_ALONG_PLANE"),
    NameCarrier::Const("POSE_PLANES_FACING"),
    NameCarrier::Const("POSE_PLANES_PARALLEL"),
    NameCarrier::Const("POSE_ROUND_LARGER"),
    NameCarrier::Const("POSE_ROUND_SIDE"),
    NameCarrier::Const("TANGENT_GAP_ORIENTATION"),
    NameCarrier::Const("GRADIENT_ALIGNMENT_LEVER"),
    NameCarrier::Const("DIHEDRAL_WEDGE"),
    NameCarrier::Const("TANGENT_SECOND_ORDER"),
    NameCarrier::Const("MATERIAL_CUSP_SIDE"),
    // The carrier ladder's data names, and each kind's reach row with
    // its straddle row (`carrier_eq.rs`).
    NameCarrier::Const("CARRIER_CONE_APEX"),
    NameCarrier::Const("CARRIER_CONE_AXIS_PARALLEL"),
    NameCarrier::Const("CARRIER_CONE_HALF_ANGLE"),
    NameCarrier::Const("CARRIER_CYL_AXIS_OFFSET"),
    NameCarrier::Const("CARRIER_CYL_AXIS_PARALLEL"),
    NameCarrier::Const("CARRIER_CYL_RADIUS"),
    NameCarrier::Const("CARRIER_SPHERE_CENTER"),
    NameCarrier::Const("CARRIER_SPHERE_RADIUS"),
    NameCarrier::Const("CONE_REACH"),
    NameCarrier::Const("CYL_REACH"),
    NameCarrier::Const("PLANE_REACH"),
    NameCarrier::Const("SPHERE_REACH"),
    // The forwarders that hand on their callers' names, declared so
    // those names are read.
    NameCarrier::Call("apart_in_order_as"),
    NameCarrier::Call("stops"),
    NameCarrier::Call("probe_pick"),
    NameCarrier::Call("span_incidence_as"),
    NameCarrier::Call("bound_as"),
    NameCarrier::Call("classify_len_as"),
    NameCarrier::Call("closed_form_shown"),
    NameCarrier::Call("coincident_over_extent_as"),
    NameCarrier::Call("cone_cylinder_section"),
    NameCarrier::Call("constant_as"),
    NameCarrier::Call("definitely_as"),
    NameCarrier::Call("first_within"),
    NameCarrier::Call("gate"),
    NameCarrier::Call("incidence_over_span_as"),
    NameCarrier::Call("incident_as"),
    NameCarrier::Call("level_coincides_as"),
    NameCarrier::Call("limb2_enclosure_as"),
    NameCarrier::Call("old_apex_loop_as"),
    NameCarrier::Call("old_pole_arc_full_as"),
    NameCarrier::Call("old_rim_only_closed_as"),
    NameCarrier::Call("old_span_as"),
    NameCarrier::Call("ordered_side_as"),
    NameCarrier::Call("reaches_edge_as"),
    NameCarrier::Call("plane_cone_section"),
    NameCarrier::Call("require_positive_as"),
    NameCarrier::Call("require_solid_angle_as"),
    NameCarrier::Call("gradient_alignment"),
    NameCarrier::Call("normals_alignment"),
    NameCarrier::Call("wedge_reads_zero_at_the_offer"),
    NameCarrier::Call("shown"),
    NameCarrier::Call("unit_winding"),
    NameCarrier::Call("spline_incidence_as"),
    NameCarrier::Call("spline_point_incidence_as"),
    NameCarrier::Call("spread_readings"),
    NameCarrier::Call("harmonic_roots_as"),
    NameCarrier::Call("constant_residual_as"),
    NameCarrier::Call("extremes_residual_as"),
    NameCarrier::Call("torus_lift_as"),
    NameCarrier::Call("within_as"),
    NameCarrier::Const("AT_THE_END"),
    NameCarrier::Const("BASIN_HELD"),
    NameCarrier::Const("CENTRES"),
    NameCarrier::Const("CLEAR"),
    NameCarrier::Const("CONE_APEX"),
    NameCarrier::Const("CONE_DEFLATED_CLEAR"),
    NameCarrier::Const("CONE_DEFLATED_DEPTH"),
    NameCarrier::Const("CONE_DEFLATED_SIDE"),
    NameCarrier::Const("CONE_ROOT_SLACK"),
    NameCarrier::Const("CONE_RULING"),
    NameCarrier::Const("DIHEDRAL_SIGN"),
    NameCarrier::Const("EXTREME"),
    NameCarrier::Const("FACE_CLEAR"),
    NameCarrier::Const("FOOT"),
    NameCarrier::Const("FORWARD_ROW"),
    NameCarrier::Const("GAP"),
    NameCarrier::Const("HELD_ONE_PIECE"),
    NameCarrier::Const("HULL"),
    NameCarrier::Const("NODE_RAY"),
    NameCarrier::Const("OFFSET"),
    NameCarrier::Const("ONE_CURVE"),
    NameCarrier::Const("PARALLEL"),
    NameCarrier::Const("PARALLEL_TORI_ORDER"),
    NameCarrier::Const("PAST_THE_BAND"),
    NameCarrier::Const("PAST_THE_END"),
    NameCarrier::Const("RADIAL_MOVING"),
    NameCarrier::Const("ROOT_GAP"),
    NameCarrier::Const("ROOT_SLACK"),
    NameCarrier::Const("ROW"),
    NameCarrier::Const("SECTOR_ARM"),
    NameCarrier::Const("SECTOR_REFLEX"),
    NameCarrier::Const("SECTOR_STRAIGHT"),
    NameCarrier::Const("SENSE"),
    NameCarrier::Const("SITE_AT_NODE"),
    NameCarrier::Const("SITE_FOOT_ALONG"),
    NameCarrier::Const("SITE_FOOT_AT_END"),
    NameCarrier::Const("SITE_FOOT_BEND"),
    NameCarrier::Const("SITE_FOOT_PAST_END"),
    NameCarrier::Const("SITE_IN_SPAN"),
    NameCarrier::Const("SITE_PAST_SPAN"),
    NameCarrier::Const("SITE_PAST_SPAN_NEAR"),
    NameCarrier::Const("SITE_PAST_SPAN_STRADDLE"),
    NameCarrier::Const("ST_GAP"),
    NameCarrier::Const("SWING"),
    NameCarrier::Const("TILT"),
    NameCarrier::Const("TOUCH_SPREAD"),
    NameCarrier::Const("TORUS_ROOT_SLACK"),
    NameCarrier::Const("TT_COINCIDENT"),
    NameCarrier::Const("WALL_ROUND_AXIS"),
    NameCarrier::Const("NET"),
    NameCarrier::Const("STRADDLE"),
    NameCarrier::Const("PN_APERTURE_SIN"),
    NameCarrier::Const("PN_APERTURE_COS"),
    NameCarrier::Const("PN_SEAM_OFF_AXIS"),
    NameCarrier::Const("COC_APERTURE_SIN"),
    NameCarrier::Const("COC_APERTURE_COS"),
    NameCarrier::Const("COC_SEAM_OFF_AXIS"),
    NameCarrier::Const("PN_APEX_PAIR_REACH"),
    NameCarrier::Const("PN_APEX_TANGENT_REACH"),
    NameCarrier::Const("PN_APEX_POINT_REACH"),
    NameCarrier::Const("SPHERE_INCIDENT"),
    NameCarrier::Const("TORUS_INCIDENT"),
    NameCarrier::Const("SPINDLE_INCIDENT"),
    NameCarrier::Const("CYLINDER_INCIDENT"),
    NameCarrier::Const("PLANE_INCIDENT"),
    NameCarrier::Const("CYLINDER_RADIUS"),
    NameCarrier::Const("SPHERE_RADIUS"),
    NameCarrier::Call("HalfAngleRows"),
    NameCarrier::Call("QuarticRows"),
    NameCarrier::Call("SubdivisionRows"),
    NameCarrier::Call("GapRows"),
    NameCarrier::Call("apart_as"),
    NameCarrier::Call("band_verdict"),
    NameCarrier::Call("chart_point_normal"),
    NameCarrier::Call("check_residual"),
    NameCarrier::Call("check_residual_as"),
    NameCarrier::Call("class_gate"),
    NameCarrier::Call("classify_as"),
    NameCarrier::Call("decided_sense"),
    NameCarrier::Call("definite_as"),
    NameCarrier::Call("gap_as"),
    NameCarrier::Call("incidence_at"),
    NameCarrier::Call("near_as"),
    NameCarrier::Call("normal_alignment"),
    NameCarrier::Call("of_its_own"),
    NameCarrier::Call("off_as"),
    NameCarrier::Call("positive_as"),
    NameCarrier::Call("radius_guard"),
    NameCarrier::Call("read_as"),
    NameCarrier::Call("read_samples_as"),
    NameCarrier::Call("section_of"),
    NameCarrier::Call("sign_as"),
    NameCarrier::Call("surely"),
    NameCarrier::Call("straddle"),
    NameCarrier::Call("chart_incidence"),
    NameCarrier::Call("require_zero"),
    NameCarrier::Call("require_holes_wound_against"),
    NameCarrier::Call("mean_width_side"),
    NameCarrier::Call("HullArm"),
    NameCarrier::Call("bounded"),
    // Takes its rows as `(name, value)` pairs, opened by the census.
    NameCarrier::Call("signs_as"),
    NameCarrier::Call("require_on_loop"),
    NameCarrier::Call("data_beneath_sum"),
];

/// **The census meets the harness every roster shares**: no spelling it
/// cannot read, every const site a declared carrier —
/// and it reads names, so the row does not pass over an empty walk.
#[test]
fn every_decided_name_is_read() {
    let src = crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let census = predicate_census(&src, CARRIERS);
    assert!(
        !census.names.is_empty(),
        "the census read no name in {}",
        src.display()
    );
    let violations = roster_violations(&src, &census, CARRIERS, None);
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
