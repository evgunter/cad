//! **The roster: every predicate name this crate decides carries a
//! door's prefix, or is listed here with the door that decides it.**
//!
//! A topo name says its door in its spelling: `bool_` is the boolean's,
//! `split_` the split's, `offset_` the replace-face offset's, and so on
//! through [`DOORS`]. **The prefix is the convention a new name
//! follows**: name it under a door's prefix, and this row reads it with
//! nothing to add. A name under no prefix, or under another door's,
//! goes in [`EXCEPTIONS`] with the door that decides it, and a prefix
//! that is a prefix of another is a red row rather than a ranking.
//!
//! The reader is `test_utils::source::predicate_census`, the tree's one
//! home for this walk, and the checks every roster makes of it are
//! `test_utils::source::roster_violations`. It reads a name at a
//! `decide*` call, or out of a carrier declared in [`CARRIERS`]: a
//! `const` the name is spelled in, or a call or row struct whose literal
//! arguments are names. **What it cannot read it reports**, and each is
//! a red row here:
//!
//! - a site whose name is a `const` this file does not declare;
//! - any other indirect site not in [`INDIRECT`] — a funnel forwarding
//!   its caller's name, a field of a row struct, keyed by the `fn` it
//!   stands in (`<file>: <fn>: <head>`), and every `read_identity` call,
//!   keyed by its place in its file and its own literals read as names;
//! - a spelling it does not know, a file it does not walk, and a
//!   declared carrier that no longer matches anything.
//!
//! A funnel that hands on the name its caller gives it is spelled so
//! that no other call shares its token — `*_as`, "decided as" the row it
//! is handed — and declared in [`CARRIERS`], so its callers' names are
//! read: a forwarder under a common word (`sign`, `zero`, `read`) would
//! hide them, since a carrier is crate-wide. **Every forwarder is an
//! entry by name**: an indirect site is keyed by its enclosing `fn`, so a
//! new forwarder that reuses a parameter name another already spells
//! (`name`, `predicate`) reds until it is declared, here or as a
//! carrier. What this row still cannot see is a name spelled in a closed
//! type's `predicate()` match or chosen by a `match` before it reaches a
//! funnel ([`CLOSED_TYPE`]), which is
//! `the-census-reads-no-closed-type-name-table`.
//!
//! **When it runs.** The walk takes seconds, so this row is in the slow
//! set: a PR runs it only when the PR names it, and the nightly runs it
//! every night. A PR that adds a name or a site without touching this
//! file meets it at the nightly, not at its own gate
//! (`work/ciw/tree-wide-source-censuses-in-the-slow-set-run-on-no-pr-that-moves-a-site.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use test_utils::source::{
    FORWARDS, NameCarrier, PredicateCensus, crate_dir, predicate_census, roster_violations,
};

/// A door: the prefixes its names carry, and the error type its
/// escalations are typed on.
struct Door {
    prefixes: &'static [&'static str],
    error: fn() -> &'static str,
}

const BOOLEAN: Door = Door {
    prefixes: &[
        "bool_",
        "carrier_",
        "contact_",
        "rim_",
        "seam_",
        "section_",
        "side_at_infinity_",
    ],
    error: std::any::type_name::<topo::BooleanError>,
};
const SPLIT: Door = Door {
    prefixes: &["split_"],
    error: std::any::type_name::<topo::SplitError>,
};
const SPLIT_JOIN: Door = Door {
    prefixes: &["chord_"],
    error: std::any::type_name::<topo::SplitJoinError>,
};
const POINT_IN_LOOP: Door = Door {
    prefixes: &["point_in_"],
    error: std::any::type_name::<topo::PointInLoopError>,
};
const REPLACE_FACE: Door = Door {
    prefixes: &["offset_"],
    error: std::any::type_name::<topo::ReplaceFaceError<f64>>,
};
/// The ray trace's reads, typed on `TraceFault`; the trace's callers are
/// the editor and the viewer, not the boolean.
const TRACE: Door = Door {
    prefixes: &["trace_"],
    error: std::any::type_name::<topo::boolean::trace::TraceFault>,
};
const LINK: Door = Door {
    prefixes: &["link_"],
    error: std::any::type_name::<topo::LinkRefusal>,
};
/// The recharts the Euler operators run on an attach, typed on
/// `EulerOpError::RechartBoundaryEscalated`.
const RECHART: Door = Door {
    prefixes: &["rechart_"],
    error: std::any::type_name::<topo::EulerOpError>,
};
const VALIDATION: Door = Door {
    prefixes: &[
        "census_",
        "pinch_corner_",
        "pm_census_",
        "ring_outer_",
        "self_intersection_",
    ],
    error: std::any::type_name::<topo::ValidationError>,
};
const CLEARANCE: Door = Door {
    prefixes: &["cell_region_", "clearance_"],
    error: std::any::type_name::<topo::clearance::MinSeparationRefusal>,
};
const CHART_REGION: Door = Door {
    prefixes: &["chart_"],
    error: std::any::type_name::<topo::ChartRegionError>,
};
const SHELL: Door = Door {
    prefixes: &["shell_"],
    error: std::any::type_name::<topo::ShellError<f64>>,
};
const PCURVES: Door = Door {
    prefixes: &["pcurve_"],
    error: std::any::type_name::<topo::PcurveMintError>,
};
/// A unit test's own fixture, decided in `#[cfg(test)]` code under
/// `src`; no door reports it.
const UNIT_TEST: Door = Door {
    prefixes: &["fuzz_", "test_"],
    error: || "no door: a unit test's fixture",
};

/// Every door, each with its prefixes declared once.
const DOORS: &[&Door] = &[
    &BOOLEAN,
    &SPLIT,
    &SPLIT_JOIN,
    &POINT_IN_LOOP,
    &REPLACE_FACE,
    &LINK,
    &TRACE,
    &RECHART,
    &VALIDATION,
    &CLEARANCE,
    &CHART_REGION,
    &SHELL,
    &PCURVES,
    &UNIT_TEST,
];

/// The names under no door's prefix, or under another door's, with the
/// door that decides each.
const EXCEPTIONS: &[(&str, &Door)] = &[
    // The face normal's reads, which the boolean's pierce point takes.
    ("cone_at_apex", &BOOLEAN),
    ("cone_off_axis", &BOOLEAN),
    ("foot_off_locus", &BOOLEAN),
    // The sphere cover the boolean's seam reads stand on.
    ("cover_side", &BOOLEAN),
    ("cover_sphere_seam", &BOOLEAN),
    ("cover_face_nearest_foot", &BOOLEAN),
    ("curve_carrier_clear", &BOOLEAN),
    ("join_regular_point", &BOOLEAN),
    ("sector_apex_tilt", &BOOLEAN),
    ("sector_apex_window", &BOOLEAN),
    ("session_box_reach", &BOOLEAN),
    ("spline_pair_shared_corner", &BOOLEAN),
    ("stands_across_meeting", &BOOLEAN),
    ("tangent_lump_senses", &BOOLEAN),
    ("vertex_section_clear", &BOOLEAN),
    ("vertex_section_side", &BOOLEAN),
    ("vertex_section_resolution", &BOOLEAN),
    // The sector side the boolean's sectors and the split's rules both
    // read; the boolean's is the first door to raise it.
    ("enters_material", &BOOLEAN),
    ("enters_material_rise", &BOOLEAN),
    ("tangent_sector_order2", &BOOLEAN),
    ("tangent_sector_witness", &BOOLEAN),
    ("tangent_sector_osculation", &SPLIT),
    // The plane-cut section's straddle between its conic lane and its
    // rulings' lane, which the split's chord join reads.
    ("pc_axis_plane_parallel_straddle", &SPLIT_JOIN),
    ("germ_apex", &SPLIT_JOIN),
    ("section_pin_within", &SPLIT_JOIN),
    ("forward_window_row", &REPLACE_FACE),
    ("iso_row_runs_against", &REPLACE_FACE),
    ("keep_turn_row", &REPLACE_FACE),
    ("positive_volume", &VALIDATION),
    ("positive_volume_enclosure", &VALIDATION),
    ("cone_row_apex", &CHART_REGION),
    ("nearest_branch_row", &CHART_REGION),
    ("period_fold_row", &CHART_REGION),
    ("each_branch_row", &PCURVES),
    ("first_within_row", &PCURVES),
    ("loop_windings_joint_room", &PCURVES),
    ("planar_winding_spiric_refine", &PCURVES),
    ("undecided_mark_row", &PCURVES),
    ("arm", &UNIT_TEST),
    ("datum", &UNIT_TEST),
    // `splitting::finish`'s `escalation_endings` rows.
    ("finish_straddle", &UNIT_TEST),
    // `on_surface_as`'s unit test's `(name, side)` row.
    ("probe_on", &UNIT_TEST),
    ("probe_oracle", &UNIT_TEST),
    // `test_support_samples`' cause named after geom-brep's arm row.
    ("dihedral_arm", &UNIT_TEST),
    ("param_span_meter", &UNIT_TEST),
    ("walk_slot_a", &UNIT_TEST),
    ("walk_slot_b", &UNIT_TEST),
    ("walk_slot_c", &UNIT_TEST),
    ("walk_slot_d", &UNIT_TEST),
    ("rh_straddle", &UNIT_TEST),
    ("t", &UNIT_TEST),
];

/// The carriers that hand a name to a funnel from somewhere other than
/// the call.
const CARRIERS: &[NameCarrier] = &[
    NameCarrier::Const("REACH"),
    NameCarrier::Const("NEST"),
    NameCarrier::Const("NEAR"),
    NameCarrier::Const("FAR"),
    // The tangent rows' closed table (`TangentRow::predicate`).
    NameCarrier::Table("TangentRow"),
    // The consts `ClearanceBound::predicate` returns, per arm.
    NameCarrier::Const("CLEARANCE_MARGIN"),
    NameCarrier::Const("SELF_INTERSECTION_GAP"),
    // `(name, value)` pairs, opened by the census: the pair section's
    // sign batch and the tangent witness's `(name, side)`.
    NameCarrier::Call("signs_as"),
    NameCarrier::Call("on_surface_as"),
    // `bound_side::first_order_as`'s two rows, picked by a `match` on the
    // support.
    NameCarrier::Const("BOOL_CHORD_SIDE"),
    NameCarrier::Const("BOOL_ENTERS_MATERIAL"),
    // A unit test's slot names, handed to `slots_as`.
    NameCarrier::Const("NAMES"),
    NameCarrier::Call("gate_as"),
    NameCarrier::Call("left_as"),
    NameCarrier::Call("slots_as"),
    NameCarrier::Const("VERTEX_SECTION_RESOLUTION"),
    NameCarrier::Const("SECTION_PIN_RESOLUTION"),
    NameCarrier::Const("CROSS_TURN"),
    NameCarrier::Const("ARM_INF"),
    // The forwarders a `FORWARDS` line names, declared so their callers'
    // names are read (`roster_violations` holds each line to one).
    NameCarrier::Call("advance_as"),
    NameCarrier::Call("all_as"),
    NameCarrier::Call("apart_as"),
    NameCarrier::Call("arc_leaving_as"),
    NameCarrier::Call("azimuth_turn_as"),
    NameCarrier::Call("before_as"),
    NameCarrier::Call("branches_astride_as"),
    NameCarrier::Call("clear_of_arc_as"),
    NameCarrier::Call("clear_of_boundary_as"),
    NameCarrier::Call("clears_as"),
    NameCarrier::Call("coincident_over_extent_as"),
    NameCarrier::Call("escalation_as"),
    NameCarrier::Call("first_count_as"),
    NameCarrier::Call("first_order_as"),
    NameCarrier::Call("gap_as"),
    NameCarrier::Call("gap_is_zero_as"),
    NameCarrier::Call("gate_arm_as"),
    NameCarrier::Call("held_zero_as"),
    NameCarrier::Call("lex_cmp_as"),
    NameCarrier::Call("near_as"),
    NameCarrier::Call("net_closes_as"),
    NameCarrier::Call("one_verdict_as"),
    NameCarrier::Call("pair_as"),
    NameCarrier::Call("parallel_same_as"),
    NameCarrier::Call("period_fold_as"),
    NameCarrier::Call("reach_as"),
    NameCarrier::Call("rejected_negative_as"),
    NameCarrier::Call("resolved_as"),
    NameCarrier::Call("ring_outer_contact_about_as"),
    NameCarrier::Call("side_at_closest_as"),
    NameCarrier::Call("signed_is_zero_as"),
    NameCarrier::Call("span_clear_as"),
    // The identity data's names ([`IDENTITY`], [`IDENTITY_DATA`]).
    NameCarrier::Const("BOOL_WRAP_RIM"),
    NameCarrier::Const("CARRIER_CONE_APEX"),
    NameCarrier::Const("CARRIER_CONE_AXIS_PARALLEL"),
    NameCarrier::Const("CARRIER_CONE_HALF_ANGLE"),
    NameCarrier::Const("CARRIER_CYL_AXIS_OFFSET"),
    NameCarrier::Const("CARRIER_CYL_AXIS_PARALLEL"),
    NameCarrier::Const("CARRIER_CYL_RADIUS"),
    NameCarrier::Const("CARRIER_SPHERE_CENTER"),
    NameCarrier::Const("CARRIER_SPHERE_RADIUS"),
    NameCarrier::Const("POINT_IN_LOOP_NORMAL"),
    NameCarrier::Const("POINT_IN_LOOP_PLANE"),
    NameCarrier::Const("POINT_IN_LOOP_QUERY"),
    NameCarrier::Const("RIM_CIRCLE_AXIS_PARALLEL"),
    NameCarrier::Const("RIM_CIRCLE_CENTER"),
    NameCarrier::Const("RIM_CIRCLE_RADIUS"),
    NameCarrier::Const("SEAM_LINE_COLLINEAR"),
    // A name, or a row and its straddle row, spelled in a `const` and
    // handed to a funnel by it.
    NameCarrier::Const("CONE_REACH"),
    NameCarrier::Const("CONTACT_TANGENT_ON_1"),
    NameCarrier::Const("CONTACT_TANGENT_ON_2"),
    NameCarrier::Const("CYL_REACH"),
    NameCarrier::Const("PLANE_REACH"),
    // The region doors' order and crossing rows (`Region::SIDE`).
    NameCarrier::Const("SIDE_ROWS"),
    NameCarrier::Const("SPHERE_REACH"),
    NameCarrier::Const("APERTURE"),
    NameCarrier::Const("APEX"),
    NameCarrier::Const("SELF_INTERSECTION_APEX"),
    NameCarrier::Const("TORUS_REGION_CORNER"),
    NameCarrier::Const("PIERCE_SECTOR_SIDE_CURVED"),
    NameCarrier::Const("CLEARANCE_SUPPORT_GAP"),
    NameCarrier::Const("SELF_INTERSECTION_COLUMN"),
    NameCarrier::Const("COMPONENT"),
    NameCarrier::Const("CONES_SHARE_APEX"),
    NameCarrier::Const("CONE_APEX"),
    NameCarrier::Const("CONE_APEX_SPREAD"),
    NameCarrier::Const("CONE_DEFLATED_CLEAR"),
    NameCarrier::Const("CONE_DEFLATED_DEPTH"),
    NameCarrier::Const("CONE_DEFLATED_SIDE"),
    NameCarrier::Const("CONE_ON_SURFACE"),
    NameCarrier::Const("CONE_ROOT_SLACK"),
    NameCarrier::Const("SELF_INTERSECTION_CORNER"),
    NameCarrier::Const("SELF_INTERSECTION_CORNER_DEPARTS"),
    NameCarrier::Const("SELF_INTERSECTION_CORNER_VERTEX"),
    NameCarrier::Const("CROSS_ADVANCE"),
    NameCarrier::Const("CROSS_SIDE"),
    NameCarrier::Const("SELF_INTERSECTION_CUSP"),
    NameCarrier::Const("SELF_INTERSECTION_DEPARTURE"),
    NameCarrier::Const("DEPTH_GAP"),
    NameCarrier::Const("EE_OVERLAP_CHARGED"),
    NameCarrier::Const("END_VERTEX"),
    NameCarrier::Const("FACTORED_PLANE_SIDE"),
    NameCarrier::Const("FRAME_ARM"),
    NameCarrier::Const("FRAME_AXIS"),
    NameCarrier::Const("GAP_WITHIN_EPS"),
    NameCarrier::Const("SELF_INTERSECTION_GRAPH"),
    NameCarrier::Const("LINK_AHEAD"),
    NameCarrier::Const("LINK_BALL"),
    NameCarrier::Const("LINK_PIECE_MEET"),
    NameCarrier::Const("LINK_PIECE_REACH"),
    NameCarrier::Const("LINK_ARC"),
    NameCarrier::Const("LINK_ARC_REACH"),
    NameCarrier::Const("LINK_PLANE"),
    NameCarrier::Const("LINK_ROUND"),
    NameCarrier::Const("LINK_SECOND_ORDER"),
    NameCarrier::Const("LINK_TANGENT_FRAME"),
    NameCarrier::Const("LINK_TIE"),
    NameCarrier::Const("LINK_WEDGE"),
    NameCarrier::Const("SELF_INTERSECTION_MONOTONE"),
    NameCarrier::Const("SELF_INTERSECTION_NAPPE"),
    NameCarrier::Const("NEAREST_FOOT"),
    NameCarrier::Const("SELF_INTERSECTION_ON_SHARED"),
    NameCarrier::Const("PAIR_GAP"),
    NameCarrier::Const("PARTING_ORDER"),
    NameCarrier::Const("PAST_EVERY_REACH"),
    NameCarrier::Const("REGION_ARC_SIDE"),
    NameCarrier::Const("REGION_ON_PLANE"),
    NameCarrier::Const("REGION_SIDE"),
    NameCarrier::Const("REGION_SLOPE"),
    NameCarrier::Const("RIDER_HULL_OFF"),
    NameCarrier::Const("RIDER_ON_CURVED"),
    NameCarrier::Const("RIDER_SPIRIC_PLANE"),
    NameCarrier::Const("RIDER_WALL_BEND_ACROSS"),
    NameCarrier::Const("ROW"),
    NameCarrier::Const("RULINGS"),
    NameCarrier::Const("SEAM_GERM_ON_PLANE"),
    NameCarrier::Const("SEAM_GERM_SIDE"),
    NameCarrier::Const("SECOND_HARMONIC"),
    NameCarrier::Const("SECOND_ORDER"),
    NameCarrier::Const("SECTION_PIN_WITHIN"),
    NameCarrier::Const("SECTOR_APEX_TILT"),
    NameCarrier::Const("SECTOR_APEX_WINDOW"),
    NameCarrier::Const("SESSION_BOX_REACH"),
    NameCarrier::Const("CELL_REGION_SINGULAR"),
    NameCarrier::Const("SELF_INTERSECTION_SLIVER"),
    NameCarrier::Const("SELF_INTERSECTION_SPAN"),
    NameCarrier::Const("SPAN_ORDER_CHARGED"),
    NameCarrier::Const("SPAN_ROUNDING"),
    NameCarrier::Const("SPHERE_SEAM"),
    NameCarrier::Const("SPLINE_REGION_ARM"),
    NameCarrier::Const("SPLINE_REGION_CARRIER"),
    NameCarrier::Const("SPLINE_REGION_TRIM"),
    NameCarrier::Const("SPLIT_GATE_BOX_SIDE"),
    NameCarrier::Const("TORUS_WITNESS"),
    NameCarrier::Const("TORUS_WITNESS_STRADDLE"),
    NameCarrier::Const("TRACE_CROSSING_AHEAD"),
    NameCarrier::Const("TRACE_CROSSING_APEX"),
    NameCarrier::Const("TRACE_CROSSING_ORDER"),
    NameCarrier::Const("TRACE_CROSSING_SENSE"),
    NameCarrier::Const("TRACE_ON_CARRIER"),
    NameCarrier::Const("TRACE_PLANE_DEPTH"),
    NameCarrier::Const("TRACE_PLANE_LEAD"),
    NameCarrier::Const("TRACE_PLANE_MISSES"),
    NameCarrier::Const("CELL_REGION_VERTEX"),
    NameCarrier::Const("VERTEX_SECTION_CLEAR"),
    NameCarrier::Const("VERTEX_SECTION_SIDE"),
    NameCarrier::Const("WINDING_PREDICATE"),
    NameCarrier::Const("TANGENT_SECTOR_WITNESS"),
    // The row structs whose fields are names, read at each literal.
    NameCarrier::Call("Parallelism"),
    NameCarrier::Call("ArcRows"),
    NameCarrier::Call("ChartRows"),
    NameCarrier::Call("ParityRows"),
    NameCarrier::Call("RayRows"),
    NameCarrier::Call("RoleNames"),
    NameCarrier::Call("SegmentRows"),
    NameCarrier::Call("WedgeNames"),
    // `geom_brep`'s `ConicRows::bounded`, a row and its straddle row.
    NameCarrier::Call("bounded"),
    // `geom_core::k_stats::straddle`, which reports under the name it is
    // handed as `decide` does.
    NameCarrier::Call("straddle"),
    // The funnels that hand on the name their caller gives them, each
    // spelled so that no other call shares its token: `*_as` reads
    // "decided as" the row it is handed.
    NameCarrier::Call("charged_cmp"),
    NameCarrier::Call("clear_as"),
    NameCarrier::Call("contains_step"),
    NameCarrier::Call("decided_as"),
    NameCarrier::Call("definitely_positive"),
    NameCarrier::Call("enclosure_reading"),
    NameCarrier::Call("face_placement"),
    NameCarrier::Call("gap_zero_reading"),
    NameCarrier::Call("interval_sign_as"),
    NameCarrier::Call("lever_periods"),
    NameCarrier::Call("magnitude_door"),
    NameCarrier::Call("norm_gate"),
    NameCarrier::Call("on_as"),
    NameCarrier::Call("read_as"),
    NameCarrier::Call("relation_step"),
    NameCarrier::Call("settle_as"),
    NameCarrier::Call("settle_reported"),
    NameCarrier::Call("shift_as"),
    NameCarrier::Call("sign_as"),
    NameCarrier::Call("sign_over"),
    NameCarrier::Call("signed_gate"),
    NameCarrier::Call("signed_zero_reading"),
    NameCarrier::Call("sine"),
    NameCarrier::Call("travel"),
    NameCarrier::Call("vertices_meet"),
    NameCarrier::Call("walk_schedule"),
    NameCarrier::Call("why_as"),
    NameCarrier::Call("zero_as"),
    NameCarrier::Call("turn_as"),
    NameCarrier::Call("zero_refused"),
    // The split join's chord-leave rows, picked by a `match` on the datum.
    NameCarrier::Const("CHORD_LEAVE_GERM"),
    NameCarrier::Const("CHORD_LEAVE_SECTION"),
    // The split order's two axis rows, read in a loop.
    NameCarrier::Const("SPLIT_JOIN_ORDER_U"),
    NameCarrier::Const("SPLIT_JOIN_ORDER_V"),
    // The funnels the tree-wide reference row found their callers'
    // names behind.
    NameCarrier::Call("first_within"),
];

const NOT_FIRST: &str = "a declared carrier's call whose name is not its first argument, named \
                         here by its `fn` and that first argument; its literal names are read";
const ROW_FIELD: &str = "a field of a row struct, whose names are read at the struct's literal";
const ACCESSOR_LIKE_A_DOOR: &str = "a row bundle's accessor sharing a door's name \
     (`names.straddle()`, a `WedgeNames` row; `.turn()`, a `TurnRows` row), read as that door's call with no first argument";
const IDENTITY: &str = "a `read_identity` call over data named by declared consts";
const IDENTITY_DATA: &str = "a `read_identity` call over `sphere_data`, `cylinder_data` and \
                             `cone_data`, whose names are declared consts";
const CLOSED_TYPE: &str = "a closed type's `predicate()`, whose names the census reads where the \
                           type spells them: `TangentRow`'s table, a declared `NameCarrier::Table`, \
                           or the consts `ClearanceBound`'s arms return, declared carriers";
const TUPLE_PAIRS: &str = "a batch of `(name, value)` pairs, which the census does not open yet \
                           (`work/onecopy/the-census-opens-no-nested-tuple-of-names.md`)";
const CARRIER_CALL: &str = "a declared carrier's call holding no literal name: the name comes from \
                            its enclosing function's caller or a binding, named here by the call's \
                            first argument";
const NOT_A_NAME: &str = "another type's `decide*` method, whose first argument is not a name";
const SAME_NAMED: &str = "census.rs's point filter `bounded`, which shares the declared \
                          `ConicRows::bounded` carrier's token and takes no name";

const CLOSURE_FORWARDS: &str = "a closure bound in this `fn` hands on its callers' names, read where \
                                its callers call it (a declared carrier, or a `decide*` spelling); \
                                keyed by its `fn` until the census keys a closure by its binding \
                                (`work/onecopy/the-census-keys-no-closure-forwarder-by-its-binding.md`)";

/// The indirect sites whose expression is not a declared `const`, as
/// `<file>: <expr>`, with what carries the name there.
const INDIRECT: &[(&str, &str)] = &[
    (
        "boolean/section_cert.rs: cone_sphere: [(name, b)]",
        FORWARDS,
    ),
    (
        "boolean/contact_verify.rs: tangent_pass: (on, side)",
        FORWARDS,
    ),
    (
        "boolean/carrier_eq.rs: coincident_over_extent_as: row",
        FORWARDS,
    ),
    ("boolean/carrier_eq.rs: pair_door_reading: c1", CARRIER_CALL),
    (
        "boolean/carrier_eq.rs: read_identity#1(margins.iter().copied())",
        IDENTITY_DATA,
    ),
    (
        "boolean/carrier_eq.rs: the_ladder_reads_two_tori_as_their_pose_does: &surface(ring)",
        NOT_A_NAME,
    ),
    (
        "boolean/cone_frame_rows.rs: declared_equal_walls_meeting_at_a_sliver_are_the_bisector_loops: x",
        NOT_A_NAME,
    ),
    (
        "boolean/conic_quadric/in_band_sweep.rs: a_circle_of_radius_1500_reading_its_noise_in_the_gap_is_answered_truly: LADDER_ROWS.noise()",
        ROW_FIELD,
    ),
    (
        "boolean/contact_verify.rs: a_rim_the_pose_reads_tangent_can_still_escalate_along_the_rim: &first",
        NOT_A_NAME,
    ),
    ("boolean/contact_verify.rs: on_surface_as: on", FORWARDS),
    (
        "boolean/contact_verify.rs: on_surface_as: row.predicate()",
        CLOSED_TYPE,
    ),
    (
        "boolean/contact_verify.rs: on_surface_pads_each_verdict_in_its_own_direction: (\"probe_on\", PairSide::Second)",
        TUPLE_PAIRS,
    ),
    (
        "boolean/contact_verify.rs: tangent_pass: (on, side)",
        TUPLE_PAIRS,
    ),
    ("boolean/contain.rs: face_placement: on_plane", FORWARDS),
    ("boolean/join.rs: line_of: name", CLOSURE_FORWARDS),
    ("boolean/join.rs: travel: name", FORWARDS),
    (
        "boolean/offer_rows.rs: -: Vec3::new(1.0, 0.0, 0.0)",
        NOT_A_NAME,
    ),
    ("boolean/ops.rs: apply_cut_ins: name", CLOSURE_FORWARDS),
    ("boolean/ops.rs: carried: pending", NOT_A_NAME),
    ("boolean/ops.rs: ledger: &mut pending", NOT_A_NAME),
    ("boolean/ops.rs: vertices_meet: predicate", FORWARDS),
    (
        "boolean/recl.rs: a_flanker_reads_its_side_of_the_other_faces_at_its_reach: band",
        CARRIER_CALL,
    ),
    ("boolean/recl.rs: recl_edges: m.dir", CARRIER_CALL),
    ("boolean/recl.rs: recl_edges: mentions[i].dir", CARRIER_CALL),
    ("boolean/recl.rs: resolve_edge_edge: band", CARRIER_CALL),
    ("boolean/reduce.rs: vf: row", NOT_A_NAME),
    ("boolean/reduce.rs: vv: row", NOT_A_NAME),
    ("boolean/region.rs: place: at", CARRIER_CALL),
    ("boolean/region.rs: short_of_end: end.row", ROW_FIELD),
    ("boolean/region.rs: side_at_closest_as: cross", FORWARDS),
    ("boolean/rim_wedge.rs: departures: a", CARRIER_CALL),
    ("boolean/rim_wedge.rs: departures: b", CARRIER_CALL),
    ("boolean/rim_wedge.rs: reach_as: name", FORWARDS),
    ("boolean/rim_wedge.rs: read_identity#1([)", IDENTITY),
    (
        "boolean/rim_wedge.rs: read_identity#2([t0, t1].map(|t| {)",
        IDENTITY,
    ),
    ("boolean/section_cert.rs: sign_as: name", FORWARDS),
    ("boolean/section_cert/ruling.rs: assemble: lo", CARRIER_CALL),
    (
        "boolean/section_cert/ruling.rs: assemble: theta",
        CARRIER_CALL,
    ),
    ("boolean/section_cert/ruling.rs: before_as: row", FORWARDS),
    (
        "boolean/section_cert/ruling.rs: read: rows.precision()",
        ROW_FIELD,
    ),
    (
        "boolean/section_cert/ruling.rs: reading: rows.asymptote()",
        ROW_FIELD,
    ),
    (
        "boolean/section_cert/ruling.rs: reading: rows.fold()",
        ROW_FIELD,
    ),
    ("boolean/section_cert/ruling.rs: roots: o", CARRIER_CALL),
    (
        "boolean/sectors.rs: a_zero_direction_refuses_rather_than_reading_parallel_or_apart: -tiny",
        CARRIER_CALL,
    ),
    (
        "boolean/sectors.rs: a_zero_direction_refuses_rather_than_reading_parallel_or_apart: p",
        CARRIER_CALL,
    ),
    (
        "boolean/sectors.rs: a_zero_direction_refuses_rather_than_reading_parallel_or_apart: tiny",
        CARRIER_CALL,
    ),
    ("boolean/sectors.rs: coincident_run: a.dir", CARRIER_CALL),
    ("boolean/sectors.rs: crescent: da", CARRIER_CALL),
    ("boolean/sectors.rs: first_order_overlap: u", CARRIER_CALL),
    (
        "boolean/sectors.rs: parallel_same_as: reading.predicate",
        FORWARDS,
    ),
    (
        "boolean/sectors.rs: parallelism_reads_the_directions_and_not_their_norms: u * 1e3",
        CARRIER_CALL,
    ),
    (
        "boolean/sectors.rs: parallelism_reads_the_directions_and_not_their_norms: u * norm",
        CARRIER_CALL,
    ),
    ("boolean/sectors.rs: runs_into: dir", CARRIER_CALL),
    (
        "boolean/solid_contain.rs: a_root_whose_reach_margin_lies_in_band_is_set_aside: SCHEDULE_ROWS.reaches()",
        ROW_FIELD,
    ),
    (
        "boolean/solid_contain.rs: cast_ray: SCHEDULE_ROWS.order()",
        CARRIER_CALL,
    ),
    ("boolean/solid_contain.rs: cast_ray: body", CARRIER_CALL),
    (
        "boolean/solid_contain.rs: clear_of_carrier: rows.clearance()",
        ROW_FIELD,
    ),
    (
        "boolean/solid_contain.rs: edge_meets: rows.edge()",
        ROW_FIELD,
    ),
    (
        "boolean/solid_contain.rs: edge_meets: rows.edge_span()",
        ROW_FIELD,
    ),
    ("boolean/solid_contain.rs: pair_as: order", FORWARDS),
    (
        "boolean/solid_contain.rs: ray_crossings: rows.denom()",
        ROW_FIELD,
    ),
    (
        "boolean/solid_contain.rs: ray_crossings: rows.represents()",
        ROW_FIELD,
    ),
    (
        "boolean/solid_contain.rs: ray_passes_beside: rows.beside()",
        ROW_FIELD,
    ),
    (
        "boolean/solid_contain.rs: reaches_root_in_band: rows.reaches()",
        ROW_FIELD,
    ),
    (
        "boolean/solid_contain.rs: sphere_chart_trim: name",
        CLOSURE_FORWARDS,
    ),
    ("boolean/solid_contain.rs: vertex: rows.vertex()", ROW_FIELD),
    (
        "boolean/solid_contain/curved_edges.rs: nearest: rows.order()",
        ROW_FIELD,
    ),
    (
        "boolean/solid_contain/curved_edges.rs: read: rows.order()",
        ROW_FIELD,
    ),
    (
        "boolean/surface_group.rs: read_identity#1(coaxial_margins(self.axis, rim).map(|margin| (BOOL_WRAP_RIM, margin)))",
        IDENTITY,
    ),
    ("boolean/trace.rs: sign_as: name", FORWARDS),
    ("bound_side.rs: first_order_as: predicate", FORWARDS),
    ("bound_side.rs: read: dir", CARRIER_CALL),
    ("bound_side.rs: read_plane_side: dir", CARRIER_CALL),
    ("cell_region.rs: cell_region_lifted: boundary", CARRIER_CALL),
    ("cell_region.rs: cell_region_lifted: row", CLOSURE_FORWARDS),
    ("cell_region.rs: clear_as: row", FORWARDS),
    ("cell_region.rs: clear_of_arc_as: row", FORWARDS),
    ("cell_region.rs: clear_of_boundary_as: row", FORWARDS),
    ("census.rs: charged_cmp: name", FORWARDS),
    ("census.rs: enclosure_reading: straddle", FORWARDS),
    ("census.rs: face_holds: body", NOT_FIRST),
    ("census.rs: gap_is_zero_as: name", FORWARDS),
    ("census.rs: gap_zero_reading: name", FORWARDS),
    ("census.rs: magnitude_door: name", FORWARDS),
    ("census.rs: meeting_point: a.p0 + a.dir * s", SAME_NAMED),
    (
        "census.rs: meeting_point: e.p0 + e.dir * ((f.origin - e.p0).dot(f.normal) / along)",
        SAME_NAMED,
    ),
    ("census.rs: pierce_inside: body", CARRIER_CALL),
    ("census.rs: signed_is_zero_as: name", FORWARDS),
    ("census.rs: signed_zero_reading: name", FORWARDS),
    ("census.rs: span_clear_as: [T::zero(), reach]", NOT_FIRST),
    ("census.rs: span_clear_as: name", FORWARDS),
    ("census/curved.rs: decided_as: name", FORWARDS),
    (
        "census/pierce_rows.rs: a_side_with_no_closed_form_clearance_is_not_read_clear: pair.body",
        CARRIER_CALL,
    ),
    (
        "census/pierce_rows.rs: a_spans_reach_counts_the_band_and_the_slope: pair.body",
        CARRIER_CALL,
    ),
    ("census/pierce_rows.rs: clear: pair.body", CARRIER_CALL),
    (
        "chart_region.rs: cylinder_pair_overlap: name",
        CLOSURE_FORWARDS,
    ),
    (
        "chart_region.rs: cylinder_pair_overlap: window_mid(&uv_a) - window_mid(&uv_b)",
        CARRIER_CALL,
    ),
    ("chart_region.rs: gate_arm_as: name", FORWARDS),
    ("chart_region.rs: period_fold_as: name", FORWARDS),
    ("chart_region.rs: spline_chart_arms: arm_u", CARRIER_CALL),
    ("chart_region.rs: spline_chart_arms: arm_v", CARRIER_CALL),
    (
        "chart_region.rs: spline_chart_arms: inf.inf_u",
        CARRIER_CALL,
    ),
    (
        "chart_region.rs: spline_chart_arms: inf.inf_v",
        CARRIER_CALL,
    ),
    ("chart_region.rs: spline_chart_arms: name", CLOSURE_FORWARDS),
    (
        "chart_region.rs: the_fold_is_its_whole_periods_as_a_theorem_over_a_box: a - b",
        CARRIER_CALL,
    ),
    ("chord_join.rs: arc_leaving_as: name", FORWARDS),
    ("chord_join.rs: bool_planar_chord_spec: face", CARRIER_CALL),
    ("chord_join.rs: chord_spec: face", CARRIER_CALL),
    (
        "chord_join/torus_chord_rows.rs: a_chord_on_an_oval_is_an_arc_on_its_own_parameter: FaceKey::default()",
        CARRIER_CALL,
    ),
    ("clearance.rs: pair: bound.predicate()", CLOSED_TYPE),
    (
        "clearance.rs: reading: self.as_bound().predicate()",
        CLOSED_TYPE,
    ),
    ("injective.rs: interval_sign_as: row", FORWARDS),
    ("injective.rs: of: carrier", CARRIER_CALL),
    (
        "link.rs: a_corner_off_the_point_by_its_tie_reads_nothing_in_band: band",
        CARRIER_CALL,
    ),
    (
        "link.rs: a_segment_of_no_length_clears_by_its_points_distance: iv(p)",
        CARRIER_CALL,
    ),
    (
        "link.rs: a_segment_of_no_length_clears_by_its_points_distance: p",
        CARRIER_CALL,
    ),
    ("link.rs: clears_as: name", FORWARDS),
    ("link.rs: decided_as: name", FORWARDS),
    ("link.rs: escalation_as: name", FORWARDS),
    (
        "link.rs: nearly_opposite_ends_read_the_segments_own_clearance: p",
        CARRIER_CALL,
    ),
    ("link.rs: read_as: name", FORWARDS),
    ("link.rs: reads_given: p", CARRIER_CALL),
    ("link.rs: settle_as: name", FORWARDS),
    ("link.rs: settle_reported: name", FORWARDS),
    ("link.rs: sign_as: name", FORWARDS),
    ("link.rs: sign_over: name", FORWARDS),
    ("link.rs: touches: name", CLOSURE_FORWARDS),
    ("link.rs: why_as: name", FORWARDS),
    ("link.rs: winds: p", CARRIER_CALL),
    ("link.rs: zero_refused: name", FORWARDS),
    ("offset_axial.rs: azimuth_turn_as: name", FORWARDS),
    ("offset_axial.rs: classify: what", CLOSURE_FORWARDS),
    ("offset_axial.rs: reauthor: name", CLOSURE_FORWARDS),
    ("offset_derive.rs: held_zero_as: name", FORWARDS),
    ("offset_derive.rs: holds_the_move: name", CLOSURE_FORWARDS),
    (
        "offset_derive.rs: normal_in_plane_along: name",
        CLOSURE_FORWARDS,
    ),
    ("offset_derive.rs: translates_along: name", CLOSURE_FORWARDS),
    ("offset_derive.rs: zero_as: name", FORWARDS),
    (
        "pcurves.rs: a_half_turn_gap_answers_by_the_band_at_every_turn_count: chart(&cone)",
        NOT_A_NAME,
    ),
    (
        "pcurves.rs: a_long_u_domain_crosses_its_seam_by_one_period: DescribedChart::of(&s).unwrap()",
        NOT_A_NAME,
    ),
    (
        "pcurves.rs: a_narrow_cone_loop_joint_clear_of_the_apex_takes_a_lift_of_its_point: chart(surface)",
        NOT_A_NAME,
    ),
    (
        "pcurves.rs: a_spline_gate_that_is_not_off_writes_a_reset: chart(&s)",
        NOT_A_NAME,
    ),
    (
        "pcurves.rs: a_whole_turn_decides_one_period_on_each_channel: chart",
        NOT_A_NAME,
    ),
    ("pcurves.rs: branches_astride_as: name", FORWARDS),
    ("pcurves.rs: chart_u_period: payload", CARRIER_CALL),
    ("pcurves.rs: chart_v_period: payload", CARRIER_CALL),
    (
        "pcurves.rs: joint: DescribedChart::of(&s).unwrap()",
        NOT_A_NAME,
    ),
    ("pcurves.rs: lever_periods: name", FORWARDS),
    (
        "pcurves.rs: lift: DescribedChart::of(surface).unwrap()",
        NOT_A_NAME,
    ),
    ("pcurves.rs: net_closes_as: name", FORWARDS),
    (
        "pcurves.rs: only_a_net_closed_in_v_decides_a_v_element: super::DescribedChart::of(s).unwrap()",
        NOT_A_NAME,
    ),
    ("pcurves.rs: pcurve_pass: chart", NOT_A_NAME),
    ("pcurves.rs: split_cache: chart", NOT_A_NAME),
    ("pcurves.rs: turn_element: chart", NOT_A_NAME),
    (
        "pcurves.rs: u_crossing: DescribedChart::of(&s).unwrap()",
        NOT_A_NAME,
    ),
    ("pcurves.rs: walk_cycle: chart", NOT_A_NAME),
    (
        "props.rs: a_closed_form_face_refusing_about_its_reference_is_carried_from_the_origin: &faces",
        NOT_A_NAME,
    ),
    (
        "props.rs: a_shell_of_no_quadrature_face_reads_about_its_first_faces_start: &faces",
        NOT_A_NAME,
    ),
    ("props.rs: at_round_zero: faces", NOT_A_NAME),
    ("props.rs: decide_faces: items", NOT_A_NAME),
    ("props.rs: mass_properties_one_point: &faces", NOT_A_NAME),
    ("props.rs: one_verdict_as: name", FORWARDS),
    ("props.rs: read_sign: name", CLOSURE_FORWARDS),
    ("props.rs: read_sign: names.high()", ROW_FIELD),
    ("props.rs: read_sign: names.low()", ROW_FIELD),
    ("props.rs: resolution_runs: faces", NOT_A_NAME),
    ("props.rs: resume_to: &wanted", NOT_A_NAME),
    ("props.rs: settle_on: e", NOT_FIRST),
    ("props.rs: slots_as: name", FORWARDS),
    (
        "props.rs: the_enclosure_holds_the_vertex_polyhedrons_exact_volume: &faces",
        NOT_A_NAME,
    ),
    (
        "props.rs: the_parallel_arm_decides_every_face_and_then_drops_the_rest: &items",
        NOT_A_NAME,
    ),
    (
        "props.rs: the_serial_arm_decides_no_face_past_the_first_refusal: &items",
        NOT_A_NAME,
    ),
    ("props.rs: volume_about: &faces", NOT_A_NAME),
    ("props.rs: walk_set: &open", NOT_A_NAME),
    ("props/quad_lane.rs: runs: &faces", NOT_A_NAME),
    ("props/settle.rs: contains_step: e", NOT_FIRST),
    ("props/settle.rs: contains_step: name", FORWARDS),
    ("props/settle.rs: relation_step: name", FORWARDS),
    ("ray_walk.rs: advance_as: row", FORWARDS),
    ("ray_walk.rs: apart_as: row", FORWARDS),
    ("ray_walk.rs: closest: k", CARRIER_CALL),
    ("ray_walk.rs: closest: order", CLOSURE_FORWARDS),
    ("ray_walk.rs: first_count_as: order", FORWARDS),
    ("ray_walk.rs: on_segment_margin: rows.boundary()", ROW_FIELD),
    ("ray_walk.rs: on_segment_margin: rows.segment()", ROW_FIELD),
    ("ray_walk.rs: ray_crossings: rows.advance()", ROW_FIELD),
    ("ray_walk.rs: ray_crossings: rows.side()", ROW_FIELD),
    ("ring_path.rs: path_parity: name", CLOSURE_FORWARDS),
    ("rows.rs: line: self.line", ROW_FIELD),
    ("rows.rs: names: ", ACCESSOR_LIKE_A_DOOR),
    ("rows.rs: segment_rows: segment: self.segment", ROW_FIELD),
    (
        "side_reader.rs: aimed_ray: AIMED_ROWS.order()",
        CARRIER_CALL,
    ),
    ("side_reader.rs: aimed_ray: body", CARRIER_CALL),
    (
        "splitting/containment.rs: old_span_rule: rows.rows().turn()",
        ROW_FIELD,
    ),
    (
        "splitting/containment.rs: read_identity#1(data.iter().map(|&(name, m, _)| (name, m)))",
        IDENTITY,
    ),
    (
        "splitting/containment.rs: the_span_rule_tags_an_over_wound_margin_and_a_straddle: ",
        ACCESSOR_LIKE_A_DOOR,
    ),
    ("splitting/containment.rs: walk_schedule: arm_row", FORWARDS),
    ("splitting/order.rs: lex_cmp_as: name", FORWARDS),
    (
        "splitting/order.rs: one_ulp_apart_orders_strictly: &p",
        CARRIER_CALL,
    ),
    (
        "splitting/order.rs: sort_indices_by_point: &points[b]",
        CARRIER_CALL,
    ),
    ("splitting/pieced_arc.rs: contact: rows.end()", ROW_FIELD),
    (
        "splitting/pieced_arc.rs: pieces_contact: rows.clear()",
        ROW_FIELD,
    ),
    (
        "splitting/pieced_arc.rs: pieces_contact: rows.leaf()",
        ROW_FIELD,
    ),
    (
        "splitting/pieced_arc.rs: pieces_contact: rows.on()",
        ROW_FIELD,
    ),
    (
        "splitting/section_loops.rs: outlines_disjoint: body",
        CARRIER_CALL,
    ),
    ("splitting/spline_arc.rs: all_as: row", FORWARDS),
    (
        "test_support_samples.rs: rejected_negative_as: predicate",
        FORWARDS,
    ),
    ("validate.rs: decide: name", FORWARDS),
    ("validate.rs: decide_agreed: name", FORWARDS),
    ("validate.rs: decide_agreed: straddle", FORWARDS),
    ("validate.rs: decide_at_lever: names", ROW_FIELD),
    ("validate.rs: decide_at_lever: names.0", ROW_FIELD),
    ("validate.rs: decide_levered: names", ROW_FIELD),
    ("validate.rs: decide_nonzero: name", FORWARDS),
    ("validate.rs: decide_positive: name", FORWARDS),
    ("validate.rs: decide_reported: name", FORWARDS),
    ("validate.rs: definitely_positive: name", FORWARDS),
    ("validate.rs: resolved_as: name", FORWARDS),
    ("validate.rs: ring_outer_contact: body", CARRIER_CALL),
    ("validate.rs: ring_outer_contact_about_as: $name", FORWARDS),
    ("validate.rs: tier3_local_checks_marked: body", CARRIER_CALL),
    ("validate.rs: wedge_holds_named: ", ACCESSOR_LIKE_A_DOOR),
    (
        "validate.rs: wedge_holds_named: (name, names.straddle())",
        CLOSURE_FORWARDS,
    ),
    ("validate.rs: wedge_holds_named: name", CLOSURE_FORWARDS),
    (
        "validate.rs: wedge_holds_named: names.after_leaving()",
        CARRIER_CALL,
    ),
    (
        "validate.rs: wedge_holds_named: names.before_arriving()",
        CARRIER_CALL,
    ),
    (
        "validate.rs: wedge_holds_named: names.straight()",
        CARRIER_CALL,
    ),
    ("validate.rs: wedge_holds_named: names.turn()", CARRIER_CALL),
    ("vertex_section.rs: decide_vertex_on_plane: name", FORWARDS),
];

fn src() -> std::path::PathBuf {
    crate_dir(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// The door whose prefix `name` carries, if one does.
fn door_of(name: &str) -> Vec<&'static Door> {
    DOORS
        .iter()
        .copied()
        .filter(|door| door.prefixes.iter().any(|p| name.starts_with(p)))
        .collect()
}

/// **Every name the crate decides carries one door's prefix or is
/// listed, and the census meets the harness every roster shares**
/// (`test_utils::source::roster_violations`: nothing invisible, no
/// stale carrier, every const site a declared carrier, the indirect
/// sites exactly [`INDIRECT`]). One census, read once: it walks the
/// whole of `src`.
#[test]
fn every_decided_name_has_its_door_and_every_site_is_read() {
    let census = predicate_census(&src(), CARRIERS);
    the_doors_are_well_formed();
    names_have_doors(&census);
    let violations = roster_violations(&src(), &census, CARRIERS, Some(INDIRECT));
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

/// No prefix is another's prefix (a name would carry two doors), and
/// no name is listed twice.
fn the_doors_are_well_formed() {
    let prefixes: Vec<&str> = DOORS
        .iter()
        .flat_map(|door| door.prefixes.iter().copied())
        .collect();
    for a in &prefixes {
        for b in &prefixes {
            assert!(
                a == b || !b.starts_with(a),
                "the prefix `{a}` is a prefix of `{b}`: a name under `{b}` carries two doors"
            );
        }
    }
    let unique: BTreeSet<&str> = prefixes.iter().copied().collect();
    assert_eq!(unique.len(), prefixes.len(), "a prefix is declared twice");
    let listed: BTreeSet<&str> = EXCEPTIONS.iter().map(|(n, _)| *n).collect();
    assert_eq!(listed.len(), EXCEPTIONS.len(), "a name is listed twice");
}

/// Each name carries exactly one door's prefix or is an exception, each
/// exception is decided, and each prefix has a name decided under it —
/// which is also what keeps this row from passing over an empty census.
fn names_have_doors(census: &PredicateCensus) {
    let listed: BTreeSet<&str> = EXCEPTIONS.iter().map(|(n, _)| *n).collect();
    for name in &census.names {
        let doors = door_of(name);
        assert!(
            doors.len() == 1 || listed.contains(name.as_str()),
            "`{name}` is decided in the crate's src under {} — name it under a door's \
             prefix, or list it in EXCEPTIONS with the door that decides it",
            match doors.as_slice() {
                [] => "no door's prefix".to_string(),
                more => more
                    .iter()
                    .map(|door| (door.error)())
                    .collect::<Vec<_>>()
                    .join(" and "),
            }
        );
    }
    for (name, door) in EXCEPTIONS {
        assert!(
            census.names.contains(*name),
            "`{name}` is listed as {}'s, but nothing in the crate's src decides it — a stale \
             entry",
            (door.error)()
        );
    }
    for door in DOORS {
        for prefix in door.prefixes {
            assert!(
                census.names.iter().any(|n| n.starts_with(prefix)),
                "no name is decided under `{prefix}` ({}) — a stale prefix",
                (door.error)()
            );
        }
    }
}
