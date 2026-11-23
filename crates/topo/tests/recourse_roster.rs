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
//! - a spelling it does not know, and a file it does not walk.
//!
//! A declared carrier that matches nothing reads nothing and is not a
//! red.
//!
//! A name that reaches a `decide*` through a funnel no carrier declares
//! is not read here. It is not lost: a test or fixture that names it is
//! red in the tree-wide reference row
//! (`test-utils/tests/predicate_name_coverage.rs`) until the funnel is
//! declared a carrier.
//!
//! A funnel that hands on the name its caller gives it is spelled so
//! that no other call shares its token — `*_as`, "decided as" the row it
//! is handed — and declared in [`CARRIERS`], so its callers' names are
//! read: a forwarder under a common word (`sign`, `zero`, `read`) would
//! hide them, since a carrier is crate-wide. A closed type's
//! `predicate()` table is read where the type spells it, as a declared
//! `NameCarrier::Table` (`TangentRow`) or as the `Const` carriers its arms
//! return (`ClearanceBound`). What this row still cannot see is a name
//! chosen by a `match` on something else before it reaches a funnel.
//!
//! **When it runs.** The walk takes seconds, so this row is in the slow
//! set: a PR runs it only when the PR names it, and the nightly runs it
//! every night.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use test_utils::source::{
    NameCarrier, PredicateCensus, crate_dir, predicate_census, roster_violations,
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
    error: std::any::type_name::<topo::JoinError>,
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
/// The ring door's reading of a point against an analytic face's outer
/// loop (`pcurves`' ring rule, `Body::face_lift`).
const RING_PLACE: Door = Door {
    prefixes: &["ring_place_"],
    error: std::any::type_name::<topo::pcurves::RingPlacementError>,
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
    &RING_PLACE,
    &UNIT_TEST,
];

/// The names under no door's prefix, or under another door's, with the
/// door that decides each.
/// A name nothing decides now is not a red: it excuses nothing, and
/// it goes stale silently.
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
    // The star's cuts and their order (`topo::star`), which both verbs
    // read; the Boolean's is the first door to raise them.
    ("germ_line", &BOOLEAN),
    ("germ_order", &BOOLEAN),
    ("germ_sense", &BOOLEAN),
    ("germ_side", &BOOLEAN),
    ("join_regular_point", &BOOLEAN),
    ("sector_apex_tilt", &BOOLEAN),
    ("sector_apex_window", &BOOLEAN),
    ("session_box_reach", &BOOLEAN),
    ("spline_pair_shared_corner", &BOOLEAN),
    // A point against a spline face's trim (`spline_trim`). Two doors
    // raise them: the boolean's spline arm (`BooleanError`, through
    // `ContainError::Escalated`) and the ring door
    // (`RingPlacementError::Undecided`). The entry holds one door, so it
    // names the boolean's, the first to raise them.
    ("spline_trim_arm", &BOOLEAN),
    ("spline_trim_clear", &BOOLEAN),
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
    ("germ_apex", &SPLIT_JOIN),
    ("section_pin_within", &SPLIT_JOIN),
    ("forward_window_row", &REPLACE_FACE),
    ("iso_row_runs_against", &REPLACE_FACE),
    ("keep_turn_row", &REPLACE_FACE),
    ("positive_volume", &VALIDATION),
    ("positive_volume_enclosure", &VALIDATION),
    ("period_fold_row", &CHART_REGION),
    ("each_branch_row", &PCURVES),
    ("first_within_row", &PCURVES),
    ("loop_windings_joint_room", &PCURVES),
    ("planar_winding_enclosure_refine", &PCURVES),
    ("undecided_mark_row", &PCURVES),
    ("arm", &UNIT_TEST),
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
/// An entry that matches nothing in `src` now is not a red: it reads
/// nothing and excuses nothing, and it goes stale silently.
const CARRIERS: &[NameCarrier] = &[
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
    NameCarrier::Call("side_as"),
    NameCarrier::Call("left_as"),
    NameCarrier::Call("slots_as"),
    NameCarrier::Const("VERTEX_SECTION_RESOLUTION"),
    NameCarrier::Const("SECTION_PIN_RESOLUTION"),
    NameCarrier::Const("CROSS_TURN"),
    NameCarrier::Const("ARM_INF"),
    // The forwarders that hand on their callers' names, declared so
    // those names are read.
    NameCarrier::Call("advance_as"),
    NameCarrier::Call("spline_chart_arms"),
    NameCarrier::Call("closest"),
    NameCarrier::Call("place"),
    NameCarrier::Const("AT"),
    NameCarrier::Call("all_as"),
    NameCarrier::Call("apart_as"),
    NameCarrier::Call("arc_leaving_as"),
    NameCarrier::Call("azimuth_turn_as"),
    NameCarrier::Call("before_as"),
    NameCarrier::Call("branches_astride_as"),
    NameCarrier::Call("clear_of_arc_as"),
    NameCarrier::Call("clear_of_boundary_as"),
    NameCarrier::Call("clears_as"),
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
    // The identity data's names ([`IDENTITY`]).
    NameCarrier::Const("BOOL_WRAP_RIM"),
    NameCarrier::Const("POINT_IN_LOOP_NORMAL"),
    NameCarrier::Const("POINT_IN_LOOP_PLANE"),
    NameCarrier::Const("POINT_IN_LOOP_QUERY"),
    NameCarrier::Const("RIM_CIRCLE_AXIS_PARALLEL"),
    NameCarrier::Const("RIM_CIRCLE_CENTER"),
    NameCarrier::Const("RIM_CIRCLE_RADIUS"),
    NameCarrier::Const("SEAM_LINE_COLLINEAR"),
    // A name, or a row and its straddle row, spelled in a `const` and
    // handed to a funnel by it.
    NameCarrier::Const("CONTACT_TANGENT_ON_1"),
    NameCarrier::Const("CONTACT_TANGENT_ON_2"),
    // The region doors' order and crossing rows (`Region::SIDE`).
    NameCarrier::Const("SIDE_ROWS"),
    NameCarrier::Const("APERTURE"),
    NameCarrier::Const("APEX"),
    NameCarrier::Const("SELF_INTERSECTION_APEX"),
    NameCarrier::Const("TORUS_REGION_CORNER"),
    NameCarrier::Const("PIERCE_SECTOR_SIDE_CURVED"),
    NameCarrier::Const("CLEARANCE_SUPPORT_GAP"),
    NameCarrier::Const("SELF_INTERSECTION_COLUMN"),
    NameCarrier::Const("COMPONENT"),
    NameCarrier::Const("CONES_SHARE_APEX"),
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
    NameCarrier::Const("LINK_NORMAL"),
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
    NameCarrier::Const("PARTING_ONE_POINT"),
    NameCarrier::Const("PARTING_STRADDLE"),
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
    NameCarrier::Const("RING_PLACE_ARM"),
    NameCarrier::Const("RING_PLACE_CLEAR"),
    NameCarrier::Const("SPLINE_REGION_CARRIER"),
    NameCarrier::Const("SPLINE_TRIM_ARM"),
    NameCarrier::Const("SPLINE_TRIM_CLEAR"),
    NameCarrier::Const("SPLIT_GATE_BOX_SIDE"),
    NameCarrier::Const("TORUS_WITNESS"),
    NameCarrier::Const("TORUS_WITNESS_STRADDLE"),
    NameCarrier::Const("TRACE_CROSSING_SENSE"),
    NameCarrier::Const("TRACE_PLANE_LEAD"),
    NameCarrier::Const("TRACE_PLANE_MISSES"),
    NameCarrier::Const("CELL_REGION_VERTEX"),
    NameCarrier::Const("VERTEX_SECTION_CLEAR"),
    NameCarrier::Const("VERTEX_SECTION_SIDE"),
    // The star's germ line and order (`star/cut.rs`, `star/order.rs`).
    NameCarrier::Const("GERM_LINE"),
    NameCarrier::Const("GERM_ORDER"),
    NameCarrier::Const("GERM_SENSE"),
    NameCarrier::Const("GERM_SIDE"),
    NameCarrier::Const("WINDING_PREDICATE"),
    NameCarrier::Const("TANGENT_SECTOR_ORDER2"),
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
    NameCarrier::Call("ray_parity_as"),
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
/// (`test_utils::source::roster_violations`: no spelling it cannot read,
/// every const site a declared carrier). One census,
/// read once: it walks the whole of `src`.
#[test]
fn every_decided_name_has_its_door_and_every_site_is_read() {
    let census = predicate_census(&src(), CARRIERS);
    the_doors_are_well_formed();
    names_have_doors(&census);
    let violations = roster_violations(&src(), &census, CARRIERS, None);
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

/// Each name carries exactly one door's prefix or is an exception. An
/// exception nothing decides now, or a prefix no name carries, excuses
/// nothing and is not a red.
fn names_have_doors(census: &PredicateCensus) {
    assert!(
        !census.names.is_empty(),
        "the census read no name in the crate's src"
    );
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
}
