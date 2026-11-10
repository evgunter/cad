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
//!   its caller's name, a field of a row struct, and every
//!   `geom_brep::ladder::read_identity` call, keyed by its place in its
//!   file and its own literals read as names;
//! - a spelling it does not know, a file it does not walk, and a
//!   declared carrier that no longer matches anything.
//!
//! A funnel that hands on the name its caller gives it is spelled so
//! that no other call shares its token — `*_as`, "decided as" the row it
//! is handed — and declared in [`CARRIERS`], so its callers' names are
//! read: a forwarder under a common word (`sign`, `zero`, `read`) would
//! hide them, since a carrier is crate-wide. What this row still cannot
//! see is a name spelled in a closed type's `predicate()` match or chosen
//! by a `match` before it reaches a funnel ([`CLOSED_TYPE`]), which is
//! `topo-name-forwarders-hide-their-callers-names`.
//!
//! **When it runs.** The walk takes seconds, so this row is in the slow
//! set: a PR runs it only when the PR names it, and the nightly runs it
//! every night. A PR that adds a name or a site without touching this
//! file meets it at the nightly, not at its own gate
//! (`work/ciw/tree-wide-source-censuses-in-the-slow-set-run-on-no-pr-that-moves-a-site.md`).

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
    ("probe_oracle", &UNIT_TEST),
    ("rh_straddle", &UNIT_TEST),
    ("t", &UNIT_TEST),
];

/// The carriers that hand a name to a funnel from somewhere other than
/// the call.
const CARRIERS: &[NameCarrier] = &[
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
    NameCarrier::Const("RIM_ON_PLANE"),
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
];

const FORWARDS: &str = "a funnel handing on the name its caller gives it, read where its callers \
                        call a declared carrier; one entry stands for every such site in the \
                        file, so a new forwarder under an undeclared token here is not seen";
const NOT_FIRST: &str = "a declared carrier's call whose name is not its first argument, named \
                         here by that first argument, so the file's sites that share it are one \
                         entry; its literal names are read";
const ROW_FIELD: &str = "a field of a row struct, whose names are read at the struct's literal";
const ACCESSOR_LIKE_A_DOOR: &str = "a row bundle's accessor sharing a door's name \
     (`names.straddle()`, a `WedgeNames` row; `.turn()`, a `TurnRows` row), read as that door's call with no first argument";
const IDENTITY: &str = "a `read_identity` call over data named by declared consts";
const IDENTITY_DATA: &str = "a `read_identity` call over `sphere_data`, `cylinder_data` and \
                             `cone_data`, whose names are declared consts";
const CLOSED_TYPE: &str = "a closed type's `predicate()`, whose names are spelled in its own \
                           match, which the census does not read";
const NOT_A_NAME: &str = "another type's `decide*` method, whose first argument is not a name";
const SAME_NAMED: &str = "census.rs's point filter `bounded`, which shares the declared \
                          `ConicRows::bounded` carrier's token and takes no name";

/// The indirect sites whose expression is not a declared `const`, as
/// `<file>: <expr>`, with what carries the name there.
const INDIRECT: &[(&str, &str)] = &[
    ("boolean/carrier_eq.rs: &surface(ring)", NOT_A_NAME),
    (
        "boolean/carrier_eq.rs: read_identity#1(margins.iter().copied())",
        IDENTITY_DATA,
    ),
    ("boolean/carrier_eq.rs: row", FORWARDS),
    ("boolean/cone_frame_rows.rs: x", NOT_A_NAME),
    (
        "boolean/conic_quadric/in_band_sweep.rs: LADDER_ROWS.noise()",
        ROW_FIELD,
    ),
    ("boolean/contact_verify.rs: &first", NOT_A_NAME),
    ("boolean/contact_verify.rs: on", FORWARDS),
    ("boolean/contact_verify.rs: row.predicate()", CLOSED_TYPE),
    ("boolean/contain.rs: on_plane", FORWARDS),
    ("boolean/join.rs: name", FORWARDS),
    ("boolean/ops.rs: &mut pending", NOT_A_NAME),
    ("boolean/ops.rs: name", FORWARDS),
    ("boolean/ops.rs: pending", NOT_A_NAME),
    ("boolean/ops.rs: predicate", FORWARDS),
    ("boolean/reduce.rs: row", NOT_A_NAME),
    ("boolean/region.rs: cross", FORWARDS),
    ("boolean/region.rs: end.row", ROW_FIELD),
    ("boolean/rim_wedge.rs: name", FORWARDS),
    ("boolean/rim_wedge.rs: read_identity#1([)", IDENTITY),
    (
        "boolean/rim_wedge.rs: read_identity#2([t0, t1].map(|t| {)",
        IDENTITY,
    ),
    ("boolean/section_cert.rs: name", FORWARDS),
    ("boolean/section_cert/ruling.rs: row", FORWARDS),
    (
        "boolean/section_cert/ruling.rs: rows.asymptote()",
        ROW_FIELD,
    ),
    ("boolean/section_cert/ruling.rs: rows.fold()", ROW_FIELD),
    (
        "boolean/section_cert/ruling.rs: rows.precision()",
        ROW_FIELD,
    ),
    ("boolean/sectors.rs: reading.predicate", FORWARDS),
    (
        "boolean/solid_contain.rs: SCHEDULE_ROWS.reaches()",
        ROW_FIELD,
    ),
    ("boolean/solid_contain.rs: name", FORWARDS),
    ("boolean/solid_contain.rs: order", FORWARDS),
    ("boolean/solid_contain.rs: rows.beside()", ROW_FIELD),
    ("boolean/solid_contain.rs: rows.clearance()", ROW_FIELD),
    ("boolean/solid_contain.rs: rows.denom()", ROW_FIELD),
    ("boolean/solid_contain.rs: rows.edge()", ROW_FIELD),
    ("boolean/solid_contain.rs: rows.edge_span()", ROW_FIELD),
    ("boolean/solid_contain.rs: rows.reaches()", ROW_FIELD),
    ("boolean/solid_contain.rs: rows.represents()", ROW_FIELD),
    ("boolean/solid_contain.rs: rows.vertex()", ROW_FIELD),
    (
        "boolean/solid_contain/curved_edges.rs: rows.order()",
        ROW_FIELD,
    ),
    (
        "boolean/surface_group.rs: read_identity#1(coaxial_margins(self.axis, rim).map(|margin| \
         (BOOL_WRAP_RIM, margin)))",
        IDENTITY,
    ),
    ("boolean/trace.rs: name", FORWARDS),
    ("bound_side.rs: predicate", FORWARDS),
    ("cell_region.rs: row", FORWARDS),
    ("census.rs: [T::zero(), reach]", NOT_FIRST),
    ("census.rs: a.p0 + a.dir * s", SAME_NAMED),
    ("census.rs: body", NOT_FIRST),
    (
        "census.rs: e.p0 + e.dir * ((f.origin - e.p0).dot(f.normal) / along)",
        SAME_NAMED,
    ),
    ("census.rs: name", FORWARDS),
    ("census.rs: straddle", FORWARDS),
    ("census/curved.rs: name", FORWARDS),
    ("chart_region.rs: name", FORWARDS),
    ("chord_join.rs: name", FORWARDS),
    ("clearance.rs: bound.predicate()", CLOSED_TYPE),
    ("clearance.rs: self.as_bound().predicate()", CLOSED_TYPE),
    ("injective.rs: row", FORWARDS),
    ("link.rs: name", FORWARDS),
    ("offset_axial.rs: name", FORWARDS),
    ("offset_axial.rs: what", FORWARDS),
    ("offset_derive.rs: name", FORWARDS),
    ("pcurves.rs: DescribedChart::of(&s).unwrap()", NOT_A_NAME),
    (
        "pcurves.rs: DescribedChart::of(surface).unwrap()",
        NOT_A_NAME,
    ),
    ("pcurves.rs: chart", NOT_A_NAME),
    ("pcurves.rs: chart(&cone)", NOT_A_NAME),
    ("pcurves.rs: chart(&s)", NOT_A_NAME),
    ("pcurves.rs: chart(surface)", NOT_A_NAME),
    ("pcurves.rs: name", FORWARDS),
    (
        "pcurves.rs: super::DescribedChart::of(s).unwrap()",
        NOT_A_NAME,
    ),
    ("props.rs: &faces", NOT_A_NAME),
    ("props.rs: &items", NOT_A_NAME),
    ("props.rs: &open", NOT_A_NAME),
    ("props.rs: &wanted", NOT_A_NAME),
    ("props.rs: e", NOT_FIRST),
    ("props.rs: faces", NOT_A_NAME),
    ("props.rs: items", NOT_A_NAME),
    ("props.rs: name", FORWARDS),
    ("props.rs: names.high()", ROW_FIELD),
    ("props.rs: names.low()", ROW_FIELD),
    ("props/quad_lane.rs: &faces", NOT_A_NAME),
    ("props/settle.rs: e", NOT_FIRST),
    ("props/settle.rs: name", FORWARDS),
    ("ray_walk.rs: order", FORWARDS),
    ("ray_walk.rs: row", FORWARDS),
    ("ray_walk.rs: rows.advance()", ROW_FIELD),
    ("ray_walk.rs: rows.boundary()", ROW_FIELD),
    ("ray_walk.rs: rows.segment()", ROW_FIELD),
    ("ray_walk.rs: rows.side()", ROW_FIELD),
    ("ring_path.rs: name", FORWARDS),
    ("rows.rs: ", ACCESSOR_LIKE_A_DOOR),
    ("rows.rs: segment: self.segment", ROW_FIELD),
    ("rows.rs: self.line", ROW_FIELD),
    ("splitting/containment.rs: ", ACCESSOR_LIKE_A_DOOR),
    ("splitting/containment.rs: arm_row", FORWARDS),
    (
        "splitting/containment.rs: read_identity#1(data.iter().map(|&(name, m, _)| (name, m)))",
        IDENTITY,
    ),
    ("splitting/containment.rs: rows.rows().turn()", ROW_FIELD),
    ("splitting/order.rs: name", FORWARDS),
    ("splitting/pieced_arc.rs: rows.clear()", ROW_FIELD),
    ("splitting/pieced_arc.rs: rows.end()", ROW_FIELD),
    ("splitting/pieced_arc.rs: rows.leaf()", ROW_FIELD),
    ("splitting/pieced_arc.rs: rows.on()", ROW_FIELD),
    ("splitting/spline_arc.rs: row", FORWARDS),
    ("test_support_samples.rs: predicate", FORWARDS),
    ("validate.rs: ", ACCESSOR_LIKE_A_DOOR),
    ("validate.rs: $name", FORWARDS),
    ("validate.rs: (name, names.straddle())", FORWARDS),
    ("validate.rs: name", FORWARDS),
    ("validate.rs: names", ROW_FIELD),
    ("validate.rs: names.0", ROW_FIELD),
    ("validate.rs: straddle", FORWARDS),
    ("vertex_section.rs: name", FORWARDS),
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
