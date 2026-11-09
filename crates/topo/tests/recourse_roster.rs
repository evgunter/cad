//! **The roster: every predicate name this crate decides carries its
//! door's prefix, or is listed here with the door that decides it.**
//!
//! A topo name says its door in its spelling: `bool_` is the boolean's,
//! `split_` the split's, `offset_` the replace-face offset's, and so on
//! through [`DOORS`]. **The prefix is the convention a new name
//! follows**: name it under its door's prefix, and this row reads it
//! with nothing to add. A name under no prefix, or under another door's,
//! goes in [`EXCEPTIONS`] with the door that decides it, and a prefix
//! that is a prefix of another is a red row rather than a ranking.
//!
//! The reader is `test_utils::source::predicate_census`, the tree's one
//! home for this walk. It reads a name at a `decide*` call, or out of a
//! carrier declared in [`CARRIERS`]: a `const` the name is spelled in,
//! or a call or row struct whose literal arguments are names. **What it
//! cannot read it reports**, and each is a red row here:
//!
//! - a site whose name is a `const` this file does not declare;
//! - any other indirect site not in [`INDIRECT`] — a funnel forwarding
//!   its caller's name, a field of a row struct, and every
//!   `geom_brep::ladder::read_identity` call, whose data the reader
//!   cannot follow, so each lists where its data's names are spelled;
//! - a spelling it does not know, and a file it does not walk.
//!
//! A funnel that hands on the name its caller gives it is spelled so
//! that no other call shares its token — `*_as`, "decided as" the row it
//! is handed — and declared in [`CARRIERS`], so its callers' names are
//! read: a forwarder under a common word (`sign`, `zero`, `read`) would
//! hide them, since a carrier is crate-wide. What this row still cannot
//! see is a name spelled in a closed type's `predicate()` match or chosen
//! by a `match` before it reaches a funnel ([`CLOSED_TYPE`]), which is
//! `topo-name-forwarders-hide-their-callers-names`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use test_utils::source::{NameCarrier, PredicateCensus, crate_dir, predicate_census};

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
        "trace_",
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
const LINK: Door = Door {
    prefixes: &["link_"],
    error: std::any::type_name::<topo::LinkRefusal>,
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
    &VALIDATION,
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
    ("finish_straddle", &SPLIT),
    ("pc_axis_plane_parallel_straddle", &SPLIT_JOIN),
    ("germ_apex", &SPLIT_JOIN),
    ("section_pin_within", &SPLIT_JOIN),
    ("forward_window_row", &REPLACE_FACE),
    ("iso_row_runs_against", &REPLACE_FACE),
    ("keep_turn_row", &REPLACE_FACE),
    ("positive_volume", &VALIDATION),
    ("positive_volume_enclosure", &VALIDATION),
    ("rechart_boundary_residual", &VALIDATION),
    ("cone_row_apex", &CHART_REGION),
    // The cell region the clearance engine reads.
    ("cell_region_singular_gap", &CLEARANCE),
    ("cell_region_vertex_gap", &CLEARANCE),
    ("nearest_branch_row", &CHART_REGION),
    ("period_fold_row", &CHART_REGION),
    ("each_branch_row", &PCURVES),
    ("first_within_row", &PCURVES),
    ("loop_windings_joint_room", &PCURVES),
    ("planar_winding_spiric_refine", &PCURVES),
    ("undecided_mark_row", &PCURVES),
    ("clearance_support_gap", &CLEARANCE),
    ("arm", &UNIT_TEST),
    ("asymptote", &UNIT_TEST),
    ("clear", &UNIT_TEST),
    ("datum", &UNIT_TEST),
    ("fold", &UNIT_TEST),
    ("monotone", &UNIT_TEST),
    ("precision", &UNIT_TEST),
    ("probe_oracle", &UNIT_TEST),
    ("rh_straddle", &UNIT_TEST),
    ("side", &UNIT_TEST),
    ("t", &UNIT_TEST),
    ("width", &UNIT_TEST),
];

/// The clearance engine's names: a door with no prefix of its own.
const CLEARANCE: Door = Door {
    prefixes: &[],
    error: std::any::type_name::<topo::clearance::MinSeparationRefusal>,
};

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
    NameCarrier::Const("SIDE"),
    NameCarrier::Const("SPHERE_REACH"),
    NameCarrier::Const("APERTURE"),
    NameCarrier::Const("APEX"),
    NameCarrier::Const("CHARGE"),
    NameCarrier::Const("CLEARANCE_SUPPORT_GAP"),
    NameCarrier::Const("COLUMN"),
    NameCarrier::Const("COMPONENT"),
    NameCarrier::Const("CONES_SHARE_APEX"),
    NameCarrier::Const("CONE_APEX"),
    NameCarrier::Const("CONE_APEX_SPREAD"),
    NameCarrier::Const("CONE_DEFLATED_CLEAR"),
    NameCarrier::Const("CONE_DEFLATED_DEPTH"),
    NameCarrier::Const("CONE_DEFLATED_SIDE"),
    NameCarrier::Const("CONE_ON_SURFACE"),
    NameCarrier::Const("CONE_ROOT_SLACK"),
    NameCarrier::Const("CORNER"),
    NameCarrier::Const("CORNER_DEPARTS"),
    NameCarrier::Const("CORNER_VERTEX"),
    NameCarrier::Const("CROSS_ADVANCE"),
    NameCarrier::Const("CROSS_SIDE"),
    NameCarrier::Const("CUSP"),
    NameCarrier::Const("DEPARTURE"),
    NameCarrier::Const("DEPTH_GAP"),
    NameCarrier::Const("EE_OVERLAP_CHARGED"),
    NameCarrier::Const("END_VERTEX"),
    NameCarrier::Const("FACTORED_PLANE_SIDE"),
    NameCarrier::Const("FRAME_ARM"),
    NameCarrier::Const("FRAME_AXIS"),
    NameCarrier::Const("GAP_WITHIN_EPS"),
    NameCarrier::Const("GRAPH"),
    NameCarrier::Const("LADDER_NOISE"),
    NameCarrier::Const("LINK_AHEAD"),
    NameCarrier::Const("LINK_BALL"),
    NameCarrier::Const("LINK_PIECE_MEET"),
    NameCarrier::Const("LINK_PIECE_REACH"),
    NameCarrier::Const("LINK_PLANE"),
    NameCarrier::Const("LINK_ROUND"),
    NameCarrier::Const("LINK_SECOND_ORDER"),
    NameCarrier::Const("LINK_TANGENT_FRAME"),
    NameCarrier::Const("LINK_TIE"),
    NameCarrier::Const("LINK_WEDGE"),
    NameCarrier::Const("MONOTONE"),
    NameCarrier::Const("NAPPE"),
    NameCarrier::Const("NEAREST_FOOT"),
    NameCarrier::Const("ON_SHARED"),
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
    NameCarrier::Const("SINGULAR"),
    NameCarrier::Const("SLIVER"),
    NameCarrier::Const("SPAN"),
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
    NameCarrier::Const("VERTEX"),
    NameCarrier::Const("VERTEX_SECTION_CLEAR"),
    NameCarrier::Const("VERTEX_SECTION_SIDE"),
    NameCarrier::Const("WINDING_PREDICATE"),
    NameCarrier::Const("WITNESS"),
    // The row structs whose fields are names, read at each literal.
    NameCarrier::Call("Parallelism"),
    NameCarrier::Call("ArcRows"),
    NameCarrier::Call("ArcTrimRows"),
    NameCarrier::Call("ChartRows"),
    NameCarrier::Call("FirstHarmonicRows"),
    NameCarrier::Call("HalfAngleRows"),
    NameCarrier::Call("ParityRows"),
    NameCarrier::Call("QuarticRows"),
    NameCarrier::Call("RayRows"),
    NameCarrier::Call("RoleNames"),
    NameCarrier::Call("SegmentRows"),
    NameCarrier::Call("SubdivisionRows"),
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
    NameCarrier::Call("zero_refused"),
];

const FORWARDS: &str = "a funnel handing on the name its caller gives it, whose callers are \
                        read through a declared carrier";
const NOT_FIRST: &str = "a declared carrier's call whose name is not its first argument, \
                         named by that argument here; its literal names are read";
const ROW_FIELD: &str = "a field of a row struct, whose names are read at the struct's literal";
const IDENTITY: &str = "a `read_identity` call over data named by declared consts";
const IDENTITY_DATA: &str = "a `read_identity` call over `sphere_data`, `cylinder_data` and \
                             `cone_data`, whose names are declared consts";
const NOT_A_NAME: &str = "another type's `decide*` method, whose first argument is not a name";
const SAME_NAMED: &str = "a function sharing a declared carrier's token that takes no name \
                          (census.rs's point filter `bounded`, a test's `straddle`)";
const CLOSED_TYPE: &str = "a closed type's `predicate()`, whose names are spelled in its own \
                           match, which the census does not read";

/// The indirect sites whose expression is not a declared `const`, as
/// `<file>: <expr>`, with what carries the name there.
const INDIRECT: &[(&str, &str)] = &[
    ("boolean/carrier_eq.rs: &surface(ring", NOT_A_NAME),
    (
        "boolean/carrier_eq.rs: read_identity(margins.iter().copied())",
        IDENTITY_DATA,
    ),
    ("boolean/carrier_eq.rs: row", FORWARDS),
    ("boolean/cone_frame_rows.rs: x", NOT_A_NAME),
    ("boolean/contact_verify.rs: &first", NOT_A_NAME),
    ("boolean/contact_verify.rs: on", FORWARDS),
    ("boolean/contact_verify.rs: row.predicate(", CLOSED_TYPE),
    ("boolean/contain.rs: on_plane", FORWARDS),
    ("boolean/join.rs: name", FORWARDS),
    ("boolean/ops.rs: &mut pending", NOT_A_NAME),
    ("boolean/ops.rs: name", FORWARDS),
    ("boolean/ops.rs: pending", NOT_A_NAME),
    ("boolean/ops.rs: predicate", FORWARDS),
    ("boolean/reduce.rs: row", NOT_A_NAME),
    ("boolean/region.rs: cross", FORWARDS),
    ("boolean/region.rs: end.row", ROW_FIELD),
    ("boolean/rim_wedge.rs: d", SAME_NAMED),
    ("boolean/rim_wedge.rs: name", FORWARDS),
    ("boolean/rim_wedge.rs: read_identity([)", IDENTITY),
    (
        "boolean/rim_wedge.rs: read_identity([t0, t1].map(|t| {)",
        IDENTITY,
    ),
    ("boolean/section_cert.rs: name", FORWARDS),
    ("boolean/section_cert/ruling.rs: row", FORWARDS),
    ("boolean/section_cert/ruling.rs: rows.asymptote", ROW_FIELD),
    ("boolean/section_cert/ruling.rs: rows.fold", ROW_FIELD),
    ("boolean/section_cert/ruling.rs: rows.precision", ROW_FIELD),
    ("boolean/sectors.rs: reading.predicate", FORWARDS),
    ("boolean/solid_contain.rs: SCHEDULE_ROWS.reaches", ROW_FIELD),
    ("boolean/solid_contain.rs: name", FORWARDS),
    ("boolean/solid_contain.rs: order", FORWARDS),
    ("boolean/solid_contain.rs: rows.beside", ROW_FIELD),
    ("boolean/solid_contain.rs: rows.clearance", ROW_FIELD),
    ("boolean/solid_contain.rs: rows.denom", ROW_FIELD),
    ("boolean/solid_contain.rs: rows.edge", ROW_FIELD),
    ("boolean/solid_contain.rs: rows.edge_span", ROW_FIELD),
    ("boolean/solid_contain.rs: rows.reaches", ROW_FIELD),
    ("boolean/solid_contain.rs: rows.represents", ROW_FIELD),
    ("boolean/solid_contain.rs: rows.vertex", ROW_FIELD),
    (
        "boolean/solid_contain/curved_edges.rs: rows.order",
        ROW_FIELD,
    ),
    (
        "boolean/surface_group.rs: read_identity(coaxial_margins(self.axis, rim).map(|margin| (BOOL_WRAP_RIM, margin)))",
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
    ("clearance.rs: bound.predicate(", CLOSED_TYPE),
    ("clearance.rs: self.as_bound(", CLOSED_TYPE),
    ("injective.rs: row", FORWARDS),
    ("link.rs: name", FORWARDS),
    ("offset_axial.rs: name", FORWARDS),
    ("offset_axial.rs: what", FORWARDS),
    ("offset_derive.rs: name", FORWARDS),
    ("pcurves.rs: DescribedChart::of(&s", NOT_A_NAME),
    ("pcurves.rs: DescribedChart::of(surface", NOT_A_NAME),
    ("pcurves.rs: chart", NOT_A_NAME),
    ("pcurves.rs: chart(&cone", NOT_A_NAME),
    ("pcurves.rs: chart(&s", NOT_A_NAME),
    ("pcurves.rs: chart(surface", NOT_A_NAME),
    ("pcurves.rs: name", FORWARDS),
    ("pcurves.rs: super::DescribedChart::of(s", NOT_A_NAME),
    ("props.rs: &faces", NOT_A_NAME),
    ("props.rs: &items", NOT_A_NAME),
    ("props.rs: &open", NOT_A_NAME),
    ("props.rs: &wanted", NOT_A_NAME),
    ("props.rs: e", NOT_FIRST),
    ("props.rs: faces", NOT_A_NAME),
    ("props.rs: items", NOT_A_NAME),
    ("props.rs: name", FORWARDS),
    ("props.rs: names.high", ROW_FIELD),
    ("props.rs: names.low", ROW_FIELD),
    ("props/quad_lane.rs: &faces", NOT_A_NAME),
    ("props/settle.rs: e", NOT_FIRST),
    ("props/settle.rs: name", FORWARDS),
    ("ray_walk.rs: order", FORWARDS),
    ("ray_walk.rs: row", FORWARDS),
    ("ray_walk.rs: rows.advance", ROW_FIELD),
    ("ray_walk.rs: rows.boundary", ROW_FIELD),
    ("ray_walk.rs: rows.segment", ROW_FIELD),
    ("ray_walk.rs: rows.side", ROW_FIELD),
    ("ray_walk.rs: segment: self.segment", ROW_FIELD),
    ("ring_path.rs: name", FORWARDS),
    ("splitting/containment.rs: arm_row", FORWARDS),
    (
        "splitting/containment.rs: read_identity(data.iter().map(|&(name, m, _)| (name, m)))",
        IDENTITY,
    ),
    ("splitting/containment.rs: rows.turn", ROW_FIELD),
    ("splitting/order.rs: name", FORWARDS),
    ("splitting/pieced_arc.rs: rows.clear", ROW_FIELD),
    ("splitting/pieced_arc.rs: rows.end", ROW_FIELD),
    ("splitting/pieced_arc.rs: rows.leaf", ROW_FIELD),
    ("splitting/pieced_arc.rs: rows.on", ROW_FIELD),
    ("splitting/spline_arc.rs: row", FORWARDS),
    ("test_support_samples.rs: predicate", FORWARDS),
    ("validate.rs: $name", FORWARDS),
    ("validate.rs: (name", FORWARDS),
    ("validate.rs: name", FORWARDS),
    ("validate.rs: names", ROW_FIELD),
    ("validate.rs: names.0", ROW_FIELD),
    ("validate.rs: rows.end", ROW_FIELD),
    ("validate.rs: rows.trim", ROW_FIELD),
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
/// listed, every site the reader cannot read a name at is declared or
/// listed, and nothing in `src` is invisible to it.** One census, read
/// once: it walks the whole of `src`.
#[test]
fn every_decided_name_has_its_door_and_every_site_is_read() {
    let census = predicate_census(&src(), CARRIERS);
    the_doors_are_well_formed();
    names_have_doors(&census);
    indirect_sites_are_listed(&census);
    nothing_is_invisible(&census);
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

fn names_have_doors(census: &PredicateCensus) {
    assert!(
        census.names.contains("bool_wrap_rim") && census.names.contains("split_ring_path_span"),
        "the reader found no funnel calls it should have: {:?}",
        census.names
    );
    let listed: BTreeSet<&str> = EXCEPTIONS.iter().map(|(n, _)| *n).collect();
    for name in &census.names {
        let doors = door_of(name);
        assert!(
            doors.len() == 1 || listed.contains(name.as_str()),
            "`{name}` is decided in the crate's src under {} — name it under its door's \
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

fn indirect_sites_are_listed(census: &PredicateCensus) {
    let consts: BTreeSet<&str> = CARRIERS
        .iter()
        .filter_map(|carrier| match carrier {
            NameCarrier::Const(name) => Some(*name),
            NameCarrier::Call(_) => None,
        })
        .collect();
    let mut unlisted = Vec::new();
    let mut seen = BTreeSet::new();
    for site in &census.indirect {
        let (_, expr) = site.split_once(": ").expect("`<file>: <expr>`");
        let last = expr.rsplit("::").next().unwrap_or(expr);
        let is_const = !last.is_empty()
            && last
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
        if is_const {
            if !consts.contains(last) {
                unlisted.push(format!("{site} (a const not declared in CARRIERS)"));
            }
        } else if INDIRECT.iter().any(|(s, _)| s == site) {
            seen.insert(site.as_str());
        } else {
            unlisted.push(site.clone());
        }
    }
    assert!(
        unlisted.is_empty(),
        "sites whose name the reader cannot read, neither declared nor listed in INDIRECT:\n{}",
        unlisted.join("\n")
    );
    let stale: Vec<&str> = INDIRECT
        .iter()
        .map(|(s, _)| *s)
        .filter(|s| !seen.contains(s))
        .collect();
    assert!(
        stale.is_empty(),
        "INDIRECT lists sites the census no longer finds: {stale:?}"
    );
}

/// No spelling the reader does not know, and every file a `#[path]`
/// mounts lies under `src`, which the reader walks whole.
fn nothing_is_invisible(census: &PredicateCensus) {
    assert!(
        census.unreadable.is_empty(),
        "spellings the reader cannot read: {:?}",
        census.unreadable
    );
    assert!(
        !census.unwalked.is_empty(),
        "the reader found no `#[path]` mount it should have"
    );
    let src = src();
    for site in &census.unwalked {
        let (file, line) = site.rsplit_once(':').expect("`<file>:<line>`");
        let line: usize = line.parse().expect("a line number");
        let file = src.join(file);
        let text = std::fs::read_to_string(&file).expect("a readable source file");
        let code = test_utils::source::code_and_literals(&text);
        let attr = code
            .lines()
            .nth(line - 1)
            .expect("the line the census names");
        let target = attr
            .trim()
            .strip_prefix("#[path =")
            .and_then(|rest| rest.trim().strip_suffix(']'))
            .and_then(test_utils::source::plain_string_literal)
            .unwrap_or_else(|| panic!("{site}: not a `#[path = …]` mount: {attr}"));
        let mounted = file.parent().expect("a file's directory").join(target);
        assert!(
            mounted.is_file() && mounted.starts_with(&src) && !target.contains(".."),
            "{site} mounts `{target}`, which is not a file under src"
        );
    }
}
