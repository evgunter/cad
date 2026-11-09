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
//! A forwarding funnel whose callers are not read is a name this row
//! does not see; the generic forwarders (`decided`, `zero`, `sign`, …),
//! too common a token to declare crate-wide, are the follow-up row
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
    prefixes: &["clearance_"],
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
    ("tangent_sector_osculation", &SPLIT),
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
    ("asymptote", &UNIT_TEST),
    ("clear", &UNIT_TEST),
    ("datum", &UNIT_TEST),
    ("fold", &UNIT_TEST),
    ("monotone", &UNIT_TEST),
    ("precision", &UNIT_TEST),
    ("side", &UNIT_TEST),
    ("t", &UNIT_TEST),
    ("width", &UNIT_TEST),
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
    // A name spelled in a `const` and handed to a funnel by it.
    NameCarrier::Const("APEX"),
    NameCarrier::Const("CLEARANCE_SUPPORT_GAP"),
    NameCarrier::Const("COMPONENT"),
    NameCarrier::Const("CONES_SHARE_APEX"),
    NameCarrier::Const("CONE_APEX"),
    NameCarrier::Const("CONE_APEX_SPREAD"),
    NameCarrier::Const("CONE_DEFLATED_CLEAR"),
    NameCarrier::Const("CONE_DEFLATED_DEPTH"),
    NameCarrier::Const("CONE_DEFLATED_SIDE"),
    NameCarrier::Const("CONE_ON_SURFACE"),
    NameCarrier::Const("CONE_ROOT_SLACK"),
    NameCarrier::Const("CORNER_DEPARTS"),
    NameCarrier::Const("CORNER_VERTEX"),
    NameCarrier::Const("CROSS_ADVANCE"),
    NameCarrier::Const("CROSS_SIDE"),
    NameCarrier::Const("CUSP"),
    NameCarrier::Const("DEPTH_GAP"),
    NameCarrier::Const("END_VERTEX"),
    NameCarrier::Const("FACTORED_PLANE_SIDE"),
    NameCarrier::Const("FRAME_ARM"),
    NameCarrier::Const("FRAME_AXIS"),
    NameCarrier::Const("GAP_WITHIN_EPS"),
    NameCarrier::Const("LADDER_NOISE"),
    NameCarrier::Const("MONOTONE"),
    NameCarrier::Const("ON_SHARED"),
    NameCarrier::Const("PAIR_GAP"),
    NameCarrier::Const("PARTING_ORDER"),
    NameCarrier::Const("PAST_EVERY_REACH"),
    NameCarrier::Const("REGION_ARC_SIDE"),
    NameCarrier::Const("REGION_SIDE"),
    NameCarrier::Const("REGION_SLOPE"),
    NameCarrier::Const("RIDER_HULL_OFF"),
    NameCarrier::Const("RIDER_ON_CURVED"),
    NameCarrier::Const("RIDER_SPIRIC_PLANE"),
    NameCarrier::Const("RIDER_WALL_BEND_ACROSS"),
    NameCarrier::Const("ROW"),
    NameCarrier::Const("SEAM_GERM_SIDE"),
    NameCarrier::Const("SECOND_HARMONIC"),
    NameCarrier::Const("SECOND_ORDER"),
    NameCarrier::Const("SECTION_PIN_WITHIN"),
    NameCarrier::Const("SECTOR_APEX_TILT"),
    NameCarrier::Const("SECTOR_APEX_WINDOW"),
    NameCarrier::Const("SESSION_BOX_REACH"),
    NameCarrier::Const("SLIVER"),
    NameCarrier::Const("SPHERE_SEAM"),
    NameCarrier::Const("SPLINE_REGION_ARM"),
    NameCarrier::Const("SPLINE_REGION_CARRIER"),
    NameCarrier::Const("SPLINE_REGION_TRIM"),
    NameCarrier::Const("SPLIT_GATE_BOX_SIDE"),
    NameCarrier::Const("TORUS_WITNESS"),
    NameCarrier::Const("VERTEX_SECTION_CLEAR"),
    NameCarrier::Const("VERTEX_SECTION_SIDE"),
    NameCarrier::Const("WINDING_PREDICATE"),
    // The row structs whose fields are names, read at each literal.
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
    // The forwarding funnels whose callers hand them a literal.
    NameCarrier::Call("contains_step"),
    NameCarrier::Call("definitely_positive"),
    NameCarrier::Call("enclosure_reading"),
    NameCarrier::Call("gap_zero_reading"),
    NameCarrier::Call("lever_periods"),
    NameCarrier::Call("magnitude_door"),
    NameCarrier::Call("norm_gate"),
    NameCarrier::Call("relation_step"),
    NameCarrier::Call("signed_gate"),
    NameCarrier::Call("signed_zero_reading"),
    NameCarrier::Call("sine"),
    NameCarrier::Call("travel"),
    NameCarrier::Call("vertices_meet"),
    NameCarrier::Call("walk_schedule"),
];

const FORWARDS: &str = "a funnel handing on the name its caller gives it: read where the funnel \
                        is a declared carrier, else the follow-up row's";
const ROW_FIELD: &str = "a field of a row struct, whose names are read at the struct's literal";
const IDENTITY: &str = "a `read_identity` call over data named by declared consts";
const IDENTITY_DATA: &str = "a `read_identity` call over `sphere_data`, `cylinder_data` and \
                             `cone_data`, whose names are declared consts";
const CLOSED_TYPE: &str = "a closed type's `predicate()`, whose names are spelled in its own \
                           match, which the census does not read";
const NOT_A_NAME: &str = "another type's `decide*` method, whose first argument is not a name";
const SAME_NAMED: &str = "census.rs's own `bounded`, a point filter sharing the declared \
                          `ConicRows::bounded` carrier's token";

/// The indirect sites whose expression is not a declared `const`, as
/// `<file>: <expr>`, with what carries the name there.
const INDIRECT: &[(&str, &str)] = &[
    ("boolean/carrier_eq.rs: &surface(ring", NOT_A_NAME),
    (
        "boolean/carrier_eq.rs: read_identity#1(margins.iter().copied())",
        IDENTITY_DATA,
    ),
    ("boolean/carrier_eq.rs: row", FORWARDS),
    ("boolean/cone_frame_rows.rs: x", NOT_A_NAME),
    ("boolean/contact_verify.rs: &first", NOT_A_NAME),
    ("boolean/contact_verify.rs: row.predicate(", CLOSED_TYPE),
    ("boolean/contact_verify.rs: on", FORWARDS),
    ("boolean/contain.rs: on_plane", FORWARDS),
    ("boolean/join.rs: name", FORWARDS),
    ("boolean/ops.rs: &mut pending", NOT_A_NAME),
    ("boolean/ops.rs: name", FORWARDS),
    ("boolean/ops.rs: pending", NOT_A_NAME),
    ("boolean/ops.rs: predicate", FORWARDS),
    ("boolean/reduce.rs: row", FORWARDS),
    ("boolean/region.rs: cross", FORWARDS),
    ("boolean/region.rs: end.row", ROW_FIELD),
    ("boolean/rim_wedge.rs: name", FORWARDS),
    ("boolean/rim_wedge.rs: read_identity#1([)", IDENTITY),
    (
        "boolean/rim_wedge.rs: read_identity#2([t0, t1].map(|t| {)",
        IDENTITY,
    ),
    ("boolean/section_cert.rs: name", FORWARDS),
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
        "boolean/surface_group.rs: read_identity#1(coaxial_margins(self.axis, rim).map(|margin| \
         (BOOL_WRAP_RIM, margin)))",
        IDENTITY,
    ),
    ("boolean/trace.rs: name", FORWARDS),
    ("cell_region.rs: row", FORWARDS),
    (
        "census.rs: ([T::zero(), reach], straddle, within, door)",
        FORWARDS,
    ),
    ("census.rs: (a.p0 + a.dir * s)", SAME_NAMED),
    (
        "census.rs: (e.p0 + e.dir * ((f.origin - e.p0).dot(f.normal) / along))",
        SAME_NAMED,
    ),
    ("census.rs: (name, margin, band, errors)", FORWARDS),
    ("census.rs: name", FORWARDS),
    ("census/curved.rs: name", FORWARDS),
    ("chart_region.rs: name", FORWARDS),
    ("chord_join.rs: name", FORWARDS),
    ("clearance.rs: bound.predicate(", FORWARDS),
    ("clearance.rs: self.as_bound(", FORWARDS),
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
    ("props.rs: (e, RelOp::Ge, bound, name, band)", FORWARDS),
    ("props.rs: (e, RelOp::Le, bound, name, band)", FORWARDS),
    ("props.rs: (e, lo, hi, name, band)", FORWARDS),
    ("props.rs: faces", NOT_A_NAME),
    ("props.rs: items", NOT_A_NAME),
    ("props.rs: name", FORWARDS),
    ("props/quad_lane.rs: &faces", NOT_A_NAME),
    ("props/settle.rs: (e, RelOp::Ge, lo, name, band)", FORWARDS),
    ("props/settle.rs: (e, RelOp::Le, hi, name, band)", FORWARDS),
    ("props/settle.rs: name", FORWARDS),
    ("ray_walk.rs: order", FORWARDS),
    ("ray_walk.rs: row", FORWARDS),
    ("ray_walk.rs: rows.advance", ROW_FIELD),
    ("ray_walk.rs: rows.boundary", ROW_FIELD),
    ("ray_walk.rs: rows.segment", ROW_FIELD),
    ("ray_walk.rs: rows.side", ROW_FIELD),
    (
        "ray_walk.rs: { segment: self.segment, boundary: self.boundary, }",
        ROW_FIELD,
    ),
    ("ring_path.rs: name", FORWARDS),
    ("splitting/containment.rs: arm_row", FORWARDS),
    (
        "splitting/containment.rs: read_identity#1(data.iter().map(|&(name, m, _)| (name, m)))",
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
