//! **The roster: every predicate name this crate decides is read, and
//! every site the reader cannot read is listed with what carries its
//! name.**
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
//! - any other indirect site not in [`INDIRECT`] — a funnel forwarding
//!   its caller's name, a field of a row struct, a closed type's name,
//!   and every `ladder::read_identity` call, keyed by its place in its
//!   file;
//! - a spelling it does not know, a file it does not walk, and a
//!   declared carrier that no longer matches anything.
//!
//! A funnel that hands on the name its caller gives it is spelled so
//! that no other call shares its token — `*_as`, "decided as" the row
//! it is handed — and declared in [`CARRIERS`], so its callers' names
//! are read.
//!
//! **No door table yet.** topo's roster also says which door decides
//! each name, by its prefix; geom-brep's names do not yet carry one,
//! and the table is
//! `work/onecopy/geom-brep-predicate-names-carry-no-door.md`. What this
//! row holds today is the reach: no name decided here is invisible to
//! the census, which is what the tree-wide coverage row
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
const CARRIERS: &[NameCarrier] = &[
    NameCarrier::Const("AT_THE_END"),
    NameCarrier::Const("CENTRES"),
    NameCarrier::Const("CLEAR"),
    NameCarrier::Const("CONE_RULING"),
    NameCarrier::Const("DIHEDRAL_SIGN"),
    NameCarrier::Const("EXTREME"),
    NameCarrier::Const("FACE_CLEAR"),
    NameCarrier::Const("FOOT"),
    NameCarrier::Const("FORWARD_ROW"),
    NameCarrier::Const("GAP"),
    NameCarrier::Const("HULL"),
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
    NameCarrier::Const("SITE_IN_SPAN"),
    NameCarrier::Const("SITE_PAST_SPAN"),
    NameCarrier::Const("ST_GAP"),
    NameCarrier::Const("SWING"),
    NameCarrier::Const("TILT"),
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
    NameCarrier::Call("apart_as"),
    NameCarrier::Call("band_verdict"),
    NameCarrier::Call("chart_point_normal"),
    NameCarrier::Call("check_residual"),
    NameCarrier::Call("check_residual_as"),
    NameCarrier::Call("class_gate"),
    NameCarrier::Call("classify_as"),
    NameCarrier::Call("constant_residual_roots"),
    NameCarrier::Call("decided_sense"),
    NameCarrier::Call("definite_as"),
    NameCarrier::Call("gap_as"),
    NameCarrier::Call("incidence_at"),
    NameCarrier::Call("near_as"),
    NameCarrier::Call("normal_alignment"),
    NameCarrier::Call("off_as"),
    NameCarrier::Call("pairing_of_normals"),
    NameCarrier::Call("positive_as"),
    NameCarrier::Call("radius_guard"),
    NameCarrier::Call("read_as"),
    NameCarrier::Call("section_of"),
    NameCarrier::Call("sign_as"),
    NameCarrier::Call("spread_signs"),
    NameCarrier::Call("surely"),
    NameCarrier::Call("straddle"),
    NameCarrier::Call("chart_incidence"),
    NameCarrier::Call("require_zero"),
    NameCarrier::Call("require_holes_wound_against"),
    NameCarrier::Call("mean_width_side"),
    NameCarrier::Call("HullArm"),
    NameCarrier::Call("bounded"),
    NameCarrier::Call("require_on_loop"),
    NameCarrier::Call("data_beneath_sum"),
];

const FORWARDS: &str = "a funnel handing on the name its caller gives it, read where its callers \
                        call a declared carrier; one entry stands for every such site in the \
                        file, so a new forwarder under an undeclared token here is not seen";
const ROW_FIELD: &str = "a field of a row struct, whose names are read at the struct's literal";
const IDENTITY: &str = "a `read_identity` call over data whose names are read where it is built";
const PAIRED: &str = "a row and its floor handed on as an array: its literals are read, the rest \
                      forwards its caller's name";
const CLOSED_TYPE: &str = "a closed type's `name()` or `predicate()`, whose names are spelled in \
                           its own match, which the census does not read";
const STRUCT_UPDATE: &str = "a row struct built from another's literal (`..CHART_HULL`), whose names \
                             are read there";
const SAME_NAMED: &str = "a call sharing a declared carrier's token that takes no name first: a \
                          closed row's `straddle()` accessor, quad's own \
                          `require_holes_wound_against`";
const NOT_A_NAME: &str = "another `decide*` or `check*` call, whose first argument is not a name";

/// The indirect sites whose expression is not a declared `const`, as
/// `<file>: <expr>`, with what carries the name there.
const INDIRECT: &[(&str, &str)] = &[
    ("certify.rs: name", FORWARDS),
    ("certify.rs: spec", NOT_A_NAME),
    ("chart_singularity.rs: t", NOT_A_NAME),
    ("curve_face/circle_roots.rs: c0", NOT_A_NAME),
    ("curve_face/circle_roots.rs: row", FORWARDS),
    ("curve_face/circle_roots.rs: rows.coaxial", ROW_FIELD),
    ("curve_face/circle_roots.rs: rows.noise", ROW_FIELD),
    ("curve_face/circle_roots.rs: rows.root_slack", ROW_FIELD),
    ("curve_face/composite.rs: row", FORWARDS),
    ("curve_face/spline_face.rs: row", FORWARDS),
    ("curve_pair/on_surface.rs: row", FORWARDS),
    ("curve_pair/pose.rs: read_identity#1(offsets)", IDENTITY),
    ("curve_pair/pose.rs: row", FORWARDS),
    ("curve_pair/walk.rs: name", FORWARDS),
    ("curve_pair/walk.rs: row", FORWARDS),
    ("dihedral.rs: name", FORWARDS),
    ("dihedral_side.rs: name", FORWARDS),
    ("edge_nurbs.rs: Interval::from_bounds(0.0, 0.0)", NOT_A_NAME),
    ("edge_nurbs.rs: end", NOT_A_NAME),
    (
        "edge_nurbs.rs: geom_core::test_support::refused_point(0.0)",
        NOT_A_NAME,
    ),
    ("edge_nurbs.rs: name", FORWARDS),
    ("edge_nurbs.rs: predicate", FORWARDS),
    (
        "enters.rs: Margin::sagitta(deriv2.dot(reference_normal.vec()) / speed_sq, arm)",
        NOT_A_NAME,
    ),
    ("enters.rs: name", FORWARDS),
    (
        "enters.rs: parting(bend, reference_normal.vec(), arm)",
        NOT_A_NAME,
    ),
    ("half_angle.rs: ROWS.side", ROW_FIELD),
    ("half_angle.rs: meter.row", ROW_FIELD),
    ("half_angle.rs: row", FORWARDS),
    ("half_angle.rs: rows.noise", ROW_FIELD),
    ("half_angle.rs: rows.side", ROW_FIELD),
    ("half_angle.rs: touch.stretch", ROW_FIELD),
    ("intersect.rs: a", NOT_A_NAME),
    ("intersect.rs: c1", NOT_A_NAME),
    ("intersect.rs: name", FORWARDS),
    (
        "locus.rs: [internal, \"tangent_locus_internal_gap_floor\"]",
        PAIRED,
    ),
    ("offset_meters.rs: meter.predicate()", CLOSED_TYPE),
    ("pair_section.rs: a", NOT_A_NAME),
    ("pair_section.rs: decision.name()", CLOSED_TYPE),
    (
        "pair_section.rs: geom::Line { origin, dir: axis }",
        NOT_A_NAME,
    ),
    ("pair_section.rs: name", FORWARDS),
    (
        "pair_section/cylinders.rs: CcDecision::Radius.name()",
        CLOSED_TYPE,
    ),
    (
        "pair_section/cylinders.rs: [decision.name(), decision.floor()]",
        PAIRED,
    ),
    ("pair_section/cylinders.rs: decision.name()", CLOSED_TYPE),
    ("pair_section/cylinders.rs: lines", NOT_A_NAME),
    ("pair_section/gap.rs: name", FORWARDS),
    ("pair_section/gap.rs: rows.floor", ROW_FIELD),
    ("pair_section/gap.rs: self.rows.clear", ROW_FIELD),
    ("pair_section/gap.rs: self.rows.gap", ROW_FIELD),
    ("pair_section/gap.rs: self.rows.monotone", ROW_FIELD),
    ("pair_section/gap.rs: self.rows.smooth", ROW_FIELD),
    ("pair_section/gap.rs: self.rows.straddle", ROW_FIELD),
    ("pair_section/gap.rs: self.rows.turn", ROW_FIELD),
    ("pair_section/ruling.rs: ", SAME_NAMED),
    ("pair_section/ruling.rs: a", NOT_A_NAME),
    ("pair_section/ruling.rs: name", FORWARDS),
    ("pair_section/ruling.rs: row.name()", CLOSED_TYPE),
    ("pair_section/ruling.rs: row.straddle()", CLOSED_TYPE),
    ("pair_section/spline_implicit/sweep.rs: jets", NOT_A_NAME),
    ("pair_section/spline_implicit/sweep.rs: name", FORWARDS),
    (
        "pair_section/torus.rs: [OFFSET, \"section_axes_offset_floor\"]",
        PAIRED,
    ),
    (
        "pair_section/torus.rs: [WALL, \"section_torus_coaxial_wall_floor\"]",
        PAIRED,
    ),
    ("pair_section/torus.rs: [row, floor_of(row)]", PAIRED),
    ("pair_section/torus.rs: name", FORWARDS),
    ("pair_section/torus_arms.rs: [far, floor]", PAIRED),
    ("pair_section/torus_arms.rs: name", FORWARDS),
    (
        "pair_section/torus_coincidence.rs: read_identity#1(d.data.map(|(datum, m)| (datum.row(), m)))",
        IDENTITY,
    ),
    ("pcurve_cache.rs: name", FORWARDS),
    ("pcurve_cache.rs: read_identity#1(gates)", IDENTITY),
    ("pcurve_cache.rs: read_identity#2(data)", IDENTITY),
    ("pcurve_cache.rs: read_identity#3(gates)", IDENTITY),
    ("pcurve_cache/projected.rs: name", FORWARDS),
    ("pcurve_cache/spline_incidence.rs: name", FORWARDS),
    ("props/closure.rs: name", FORWARDS),
    ("props/curved.rs: name", FORWARDS),
    ("props/curved.rs: whole_name", FORWARDS),
    ("props/curved/turn_rows.rs: forward", FORWARDS),
    ("props/curved/turn_rows.rs: name", FORWARDS),
    ("props/curved/turn_rows.rs: winding", FORWARDS),
    ("props/quad/rotational.rs: &entries", SAME_NAMED),
    ("props/quad/run.rs: name", FORWARDS),
    ("quartic.rs: rows.depth", ROW_FIELD),
    ("quartic.rs: rows.disc", ROW_FIELD),
    ("quartic.rs: rows.odd", ROW_FIELD),
    ("quartic.rs: rows.shape", ROW_FIELD),
    ("quartic.rs: rows.split", ROW_FIELD),
    ("quartic.rs: rows.split_lead", ROW_FIELD),
    ("roots.rs: row", FORWARDS),
    ("ssi.rs: a", NOT_A_NAME),
    ("ssi/boundary.rs: *root", NOT_A_NAME),
    ("ssi/boundary.rs: root", NOT_A_NAME),
    ("ssi/certify.rs: arm.sup", ROW_FIELD),
    ("ssi/certify.rs: arm.value", ROW_FIELD),
    (
        "ssi/certify.rs: value_limb: SsiLimb::ImageMiss",
        STRUCT_UPDATE,
    ),
    ("ssi/ends.rs: self.sys", NOT_A_NAME),
    ("ssi/handle.rs: self.sys", NOT_A_NAME),
    ("ssi/march.rs: sys", NOT_A_NAME),
    ("ssi/refine.rs: sys", NOT_A_NAME),
    ("ssi/section.rs: found", NOT_A_NAME),
    ("ssi/section.rs: predicate", FORWARDS),
    ("ssi/section.rs: root(0.0)", NOT_A_NAME),
    ("ssi/section.rs: root(f64::NAN)", NOT_A_NAME),
    ("ssi/system.rs: &sys", NOT_A_NAME),
    ("variety.rs: read_identity#1(residuals)", IDENTITY),
];

/// **The census meets the harness every roster shares**: nothing
/// invisible, no stale carrier, every const site a declared carrier,
/// the indirect sites exactly [`INDIRECT`] — and it reads names, so the
/// row does not pass over an empty walk.
#[test]
fn every_decided_name_is_read_and_every_site_is_listed() {
    let src = crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let census = predicate_census(&src, CARRIERS);
    assert!(
        !census.names.is_empty(),
        "the census read no name in {}",
        src.display()
    );
    let violations = roster_violations(&src, &census, CARRIERS, Some(INDIRECT));
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
