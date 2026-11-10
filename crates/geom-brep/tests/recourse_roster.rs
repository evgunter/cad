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
//!   keyed by the `fn` it stands in (`<file>: <fn>: <head>`), so each
//!   forwarder is an entry by name and a new one reds until it is
//!   declared; and every `read_identity` call, keyed by its place in its
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

use test_utils::source::{FORWARDS, NameCarrier, crate_dir, predicate_census, roster_violations};

/// The carriers that hand a name to a funnel from somewhere other than
/// the call: the `const`s names are spelled in, then the calls (row
/// structs and `*_as` forwarders) whose literal arguments are names.
const CARRIERS: &[NameCarrier] = &[
    NameCarrier::Const("SQUARE_FOLD_HALF"),
    NameCarrier::Const("SQUARE_EDGE_OVER"),
    NameCarrier::Const("SPHERE_NEAR"),
    NameCarrier::Const("SPHERE_FAR"),
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
    // The forwarders a `FORWARDS` line names, declared so their callers'
    // names are read (`roster_violations` holds each line to one).
    NameCarrier::Call("apart_in_order_as"),
    NameCarrier::Call("bound_as"),
    NameCarrier::Call("classify_len_as"),
    NameCarrier::Call("cone_cylinder_section"),
    NameCarrier::Call("constant_as"),
    NameCarrier::Call("definitely_as"),
    NameCarrier::Call("extreme_sign"),
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
    NameCarrier::Call("pairing_reading_as"),
    NameCarrier::Call("reaches_edge_as"),
    NameCarrier::Call("plane_cone_section"),
    NameCarrier::Call("require_positive_as"),
    NameCarrier::Call("require_solid_angle_as"),
    NameCarrier::Call("run_spiric_checks_as"),
    NameCarrier::Call("spline_incidence_as"),
    NameCarrier::Call("spread_readings"),
    NameCarrier::Call("torus_lift_as"),
    NameCarrier::Call("within_as"),
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
    NameCarrier::Call("FirstHarmonicRows"),
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
    // Takes its rows as `(name, value)` pairs, opened by the census.
    NameCarrier::Call("signs_as"),
    NameCarrier::Call("require_on_loop"),
    NameCarrier::Call("data_beneath_sum"),
];

const ROW_FIELD: &str = "a field of a row struct, whose names are read at the struct's literal";
const ACCESSOR_LIKE_A_DOOR: &str = "a row bundle's accessor sharing a door's name (`.turn()`, \
     `.straddle()`), read as that door's call with no first argument";
const IDENTITY: &str = "a `read_identity` call over data whose names are read where it is built";
const PAIRED: &str = "a row and its floor handed on as an array: its literals are read, the rest \
                      forwards its caller's name";
const CLOSED_TYPE: &str = "a closed type's `name()`, `floor()`, `straddle()` or `predicate()`, whose \
                           names the census reads at the type's own table, a declared \
                           `NameCarrier::Table`";
const STRUCT_UPDATE: &str = "a row struct built from another's literal (`..CHART_HULL`), whose names \
                             are read there";
const SAME_NAMED: &str = "a call sharing a declared carrier's token that takes no name first: a \
                          closed row's `straddle()` accessor, quad's own \
                          `require_holes_wound_against`";
const CROSS_CRATE: &str = "a public forwarder whose callers are in other crates, whose rosters \
                           declare it a carrier and read their literals";
const CARRIER_CALL: &str = "a declared carrier's call holding no literal name: the name comes from \
                            its enclosing function's caller or a binding, named here by the call's \
                            first argument";
const NOT_A_NAME: &str = "another `decide*` or `check*` call, whose first argument is not a name";

const CLOSURE_FORWARDS: &str = "a closure bound in this `fn` hands on its callers' names, read where \
                                its callers call it (a declared carrier, or a `decide*` spelling); \
                                keyed by its `fn` until the census keys a closure by its binding \
                                (`work/onecopy/the-census-keys-no-closure-forwarder-by-its-binding.md`)";

/// The indirect sites whose expression is not a declared `const`, as
/// `<file>: <expr>`, with what carries the name there.
const INDIRECT: &[(&str, &str)] = &[
    (
        "pair_section/torus_arms.rs: reaches_edge_as: [(name, m)]",
        FORWARDS,
    ),
    (
        "carrier_resolution.rs: the_resolution_bounds_each_section_tables_carrier: &cut",
        CARRIER_CALL,
    ),
    (
        "carrier_resolution.rs: the_resolution_bounds_each_section_tables_carrier: &through",
        CARRIER_CALL,
    ),
    ("certify.rs: check_residual: name", FORWARDS),
    ("certify.rs: mint_checks: spec", NOT_A_NAME),
    ("certify.rs: run_checks: spec", NOT_A_NAME),
    ("chart_singularity.rs: nearest_on_edge: t", NOT_A_NAME),
    ("chart_singularity.rs: sampled_near: t", NOT_A_NAME),
    (
        "curve_face/circle_roots.rs: constant_residual_roots: row",
        FORWARDS,
    ),
    (
        "curve_face/circle_roots.rs: harmonic_crossing: c0",
        NOT_A_NAME,
    ),
    (
        "curve_face/circle_roots.rs: harmonic_crossing: rows.coaxial()",
        ROW_FIELD,
    ),
    (
        "curve_face/circle_roots.rs: harmonic_crossing: rows.extreme()",
        CARRIER_CALL,
    ),
    (
        "curve_face/circle_roots.rs: harmonic_crossing: rows.noise()",
        ROW_FIELD,
    ),
    (
        "curve_face/circle_roots.rs: harmonic_crossing: rows.root_slack()",
        ROW_FIELD,
    ),
    ("curve_face/composite.rs: settle: row", CLOSURE_FORWARDS),
    ("curve_face/spline_face.rs: settle: row", CLOSURE_FORWARDS),
    ("curve_pair/on_surface.rs: decided: row", FORWARDS),
    ("curve_pair/on_surface.rs: section_of: row", FORWARDS),
    ("curve_pair/pose.rs: constant_as: row", FORWARDS),
    ("curve_pair/pose.rs: definite_as: row", FORWARDS),
    ("curve_pair/pose.rs: read_identity#1(offsets)", IDENTITY),
    ("curve_pair/walk.rs: stops: reach", CARRIER_CALL),
    ("curve_pair/walk.rs: stops: reach - f", CARRIER_CALL),
    ("curve_pair/walk.rs: stops: row", CLOSURE_FORWARDS),
    ("curve_pair/walk.rs: surely: name", FORWARDS),
    ("dihedral.rs: classify_edge_wedge: name", CROSS_CRATE),
    (
        "dihedral.rs: classify_material_pairing_as: name",
        CROSS_CRATE,
    ),
    ("dihedral.rs: decide: name", FORWARDS),
    ("dihedral.rs: decide_magnitude: name", FORWARDS),
    ("dihedral.rs: decide_magnitude_reported: name", FORWARDS),
    ("dihedral.rs: decide_nonzero: name", FORWARDS),
    ("dihedral.rs: decide_nonzero_reported: name", FORWARDS),
    ("dihedral.rs: decide_positive: name", FORWARDS),
    ("dihedral.rs: decide_reported: name", FORWARDS),
    ("dihedral.rs: gate: name", FORWARDS),
    ("dihedral.rs: pairing_of_normals: name", FORWARDS),
    ("dihedral.rs: pairing_reading_as: name", FORWARDS),
    ("dihedral_side.rs: chart_point_normal: name", FORWARDS),
    ("dihedral_side.rs: near_as: name", FORWARDS),
    (
        "edge_nurbs.rs: a_refused_offset_bound_stalls_limb_two: Interval::from_bounds(0.0, 0.0)",
        NOT_A_NAME,
    ),
    (
        "edge_nurbs.rs: a_refused_offset_bound_stalls_limb_two: geom::SurfaceKind::Plane",
        CARRIER_CALL,
    ),
    (
        "edge_nurbs.rs: a_refused_offset_bound_stalls_limb_two: geom_core::test_support::refused_point(0.0)",
        NOT_A_NAME,
    ),
    ("edge_nurbs.rs: first_within: name", FORWARDS),
    ("edge_nurbs.rs: limb2: kind", CARRIER_CALL),
    ("edge_nurbs.rs: limb2_enclosure_as: predicate", FORWARDS),
    ("edge_nurbs.rs: measured_ends: end", NOT_A_NAME),
    ("edge_nurbs.rs: probe_pick: what", CARRIER_CALL),
    (
        "enters.rs: enters_material_order2: Margin::sagitta(deriv2.dot(reference_normal.vec()) / speed_sq, arm)",
        NOT_A_NAME,
    ),
    ("enters.rs: normal_alignment: name", FORWARDS),
    (
        "enters.rs: only_a_reading_the_gates_verdict_pairs_with_is_re_quoted: 1e-9",
        CARRIER_CALL,
    ),
    (
        "enters.rs: only_a_reading_the_gates_verdict_pairs_with_is_re_quoted: 5e-10",
        CARRIER_CALL,
    ),
    (
        "enters.rs: only_a_reading_the_gates_verdict_pairs_with_is_re_quoted: 5e-9",
        CARRIER_CALL,
    ),
    (
        "enters.rs: parting_side: parting(bend, reference_normal.vec(), arm)",
        NOT_A_NAME,
    ),
    ("half_angle.rs: dropped: ROWS.side()", ROW_FIELD),
    ("half_angle.rs: ladder_roots: rows.noise()", ROW_FIELD),
    ("half_angle.rs: subdivision: meter.row", ROW_FIELD),
    ("half_angle.rs: subdivision: row", CLOSURE_FORWARDS),
    ("half_angle.rs: subdivision: rows.clear()", CARRIER_CALL),
    ("half_angle.rs: subdivision: rows.monotone()", CARRIER_CALL),
    ("half_angle.rs: subdivision: rows.side()", ROW_FIELD),
    ("half_angle.rs: subdivision: rows.width()", CARRIER_CALL),
    ("half_angle.rs: subdivision: touch.stretch()", ROW_FIELD),
    ("intersect.rs: cone_cylinder_section: name", FORWARDS),
    ("intersect.rs: cylinder_cylinder_section: c1", NOT_A_NAME),
    ("intersect.rs: plane_cone_section: name", FORWARDS),
    ("intersect.rs: route_pose: a", NOT_A_NAME),
    ("intersect.rs: route_pose: b", CARRIER_CALL),
    (
        "locus.rs: parallel_cylinder_gaps: [internal, \"tangent_locus_internal_gap_floor\"]",
        PAIRED,
    ),
    (
        "offset_meters.rs: offset_curvature_headroom: meter.predicate()",
        CLOSED_TYPE,
    ),
    (
        "offset_meters.rs: offset_normal_floor: meter.predicate()",
        CLOSED_TYPE,
    ),
    (
        "pair_section.rs: decide: geom::Line { origin, dir: axis }",
        NOT_A_NAME,
    ),
    (
        "pair_section.rs: decide_stance: decision.name()",
        CLOSED_TYPE,
    ),
    ("pair_section.rs: decide_stance: name", FORWARDS),
    ("pair_section.rs: pair_section: a", NOT_A_NAME),
    ("pair_section/cylinders.rs: decide: lines", NOT_A_NAME),
    (
        "pair_section/cylinders.rs: decide_stance: CcDecision::Radius.name()",
        CLOSED_TYPE,
    ),
    (
        "pair_section/cylinders.rs: decide_stance: [decision.name(), decision.floor()]",
        PAIRED,
    ),
    (
        "pair_section/cylinders.rs: read: decision.name()",
        CLOSED_TYPE,
    ),
    ("pair_section/gap.rs: cell: self.rows.clear()", ROW_FIELD),
    ("pair_section/gap.rs: cell: self.rows.gap()", ROW_FIELD),
    ("pair_section/gap.rs: cell: self.rows.monotone()", ROW_FIELD),
    ("pair_section/gap.rs: critical_gap: rows.floor()", ROW_FIELD),
    ("pair_section/gap.rs: floor_refusal: ", ACCESSOR_LIKE_A_DOOR),
    (
        "pair_section/gap.rs: floor_refusal: self.rows.gap()",
        ROW_FIELD,
    ),
    (
        "pair_section/gap.rs: floor_refusal: self.rows.straddle()",
        ROW_FIELD,
    ),
    ("pair_section/gap.rs: positive_as: name", FORWARDS),
    ("pair_section/gap.rs: third: self.rows.smooth()", ROW_FIELD),
    ("pair_section/gap.rs: turn: self.rows.turn()", ROW_FIELD),
    (
        "pair_section/gap.rs: turn_window: self.rows.gap()",
        ROW_FIELD,
    ),
    ("pair_section/ruling.rs: escalate: ", SAME_NAMED),
    ("pair_section/ruling.rs: escalate: name", CLOSURE_FORWARDS),
    ("pair_section/ruling.rs: escalate: reading.hi", CARRIER_CALL),
    ("pair_section/ruling.rs: escalate: reading.lo", CARRIER_CALL),
    (
        "pair_section/ruling.rs: escalate: row.straddle()",
        CLOSED_TYPE,
    ),
    ("pair_section/ruling.rs: positive: row.name()", CLOSED_TYPE),
    ("pair_section/ruling.rs: ruled: a", NOT_A_NAME),
    (
        "pair_section/spline_implicit/sweep.rs: sweep: jets",
        NOT_A_NAME,
    ),
    (
        "pair_section/spline_implicit/sweep.rs: sweep: name",
        CLOSURE_FORWARDS,
    ),
    (
        "pair_section/torus.rs: axis_pose: [OFFSET, \"section_axes_offset_floor\"]",
        PAIRED,
    ),
    (
        "pair_section/torus.rs: decide: [WALL, \"section_torus_coaxial_wall_floor\"]",
        PAIRED,
    ),
    (
        "pair_section/torus.rs: decide: [row, floor_of(row)]",
        PAIRED,
    ),
    ("pair_section/torus.rs: radius_guard: name", FORWARDS),
    (
        "pair_section/torus_arms.rs: apart_in_order_as: name",
        FORWARDS,
    ),
    ("pair_section/torus_arms.rs: sign_as: name", FORWARDS),
    ("pair_section/torus_arms.rs: signs_as: name", FORWARDS),
    (
        "pair_section/torus_arms.rs: signs_across: [far, floor]",
        PAIRED,
    ),
    (
        "pair_section/torus_arms.rs: sphere_touch: name",
        CLOSURE_FORWARDS,
    ),
    (
        "pair_section/torus_coincidence.rs: read_identity#1(d.data.map(|(datum, m)| (datum.row(), m)))",
        IDENTITY,
    ),
    ("patch_bound.rs: patch_cells: n", CARRIER_CALL),
    (
        "patch_bound.rs: patch_cells_across_creases: n",
        CARRIER_CALL,
    ),
    ("pcurve_cache.rs: certify: spiric", CARRIER_CALL),
    ("pcurve_cache.rs: chart_incidence: name", FORWARDS),
    ("pcurve_cache.rs: check_residual_as: name", FORWARDS),
    ("pcurve_cache.rs: class_gate: name", FORWARDS),
    ("pcurve_cache.rs: derive_class_zero: carrier", CARRIER_CALL),
    ("pcurve_cache.rs: incidence_at: name", FORWARDS),
    ("pcurve_cache.rs: incidence_over_span_as: name", FORWARDS),
    ("pcurve_cache.rs: incident_as: name", FORWARDS),
    (
        "pcurve_cache.rs: read_edge_incidence: carrier",
        CARRIER_CALL,
    ),
    ("pcurve_cache.rs: read_identity#1(gates)", IDENTITY),
    ("pcurve_cache.rs: read_identity#2(data)", IDENTITY),
    ("pcurve_cache.rs: read_identity#3(gates)", IDENTITY),
    ("pcurve_cache.rs: read_shown: carrier", CARRIER_CALL),
    ("pcurve_cache.rs: recertify: spiric", CARRIER_CALL),
    ("pcurve_cache.rs: run_spiric_checks_as: name", FORWARDS),
    (
        "pcurve_cache.rs: spiric_chart_pcurve: carrier",
        CARRIER_CALL,
    ),
    (
        "pcurve_cache.rs: torus_oblique_circle: name",
        CLOSURE_FORWARDS,
    ),
    (
        "pcurve_cache/envelope_lemma_fuzz.rs: cone_section: &plane",
        CARRIER_CALL,
    ),
    (
        "pcurve_cache/harmonic_sup_rows.rs: focal_cone_a_drifted_slant_pair_reads_once: &plane",
        CARRIER_CALL,
    ),
    (
        "pcurve_cache/placement_rows.rs: cone_ellipse: &plane",
        CARRIER_CALL,
    ),
    ("pcurve_cache/projected.rs: positive_as: name", FORWARDS),
    (
        "pcurve_cache/projected.rs: require_positive_as: name",
        FORWARDS,
    ),
    (
        "pcurve_cache/spline_incidence.rs: spline_incidence_as: name",
        FORWARDS,
    ),
    ("props/closure.rs: loop_turns: name", CLOSURE_FORWARDS),
    (
        "props/curved.rs: a_pair_inside_the_band_by_its_chord_is_one_level: RimLevel::Unit(0.0, 0.0)",
        CARRIER_CALL,
    ),
    (
        "props/curved.rs: a_rims_lift_is_its_whole_turns_as_a_theorem_over_a_box: a",
        CARRIER_CALL,
    ),
    ("props/curved.rs: classify_as: name", FORWARDS),
    ("props/curved.rs: cone_loop_winding: name", CLOSURE_FORWARDS),
    ("props/curved.rs: level_coincides_as: name", FORWARDS),
    ("props/curved.rs: mean_width_side: name", FORWARDS),
    (
        "props/curved.rs: mixed_kind_levels_refuse_typed: RimLevel::Length(1.0_f64)",
        CARRIER_CALL,
    ),
    (
        "props/curved.rs: require_holes_wound_against: whole_name",
        FORWARDS,
    ),
    ("props/curved.rs: require_on_loop: name", FORWARDS),
    (
        "props/curved.rs: require_rims_at_extremes: rim.level",
        CARRIER_CALL,
    ),
    ("props/curved.rs: require_solid_angle_as: name", FORWARDS),
    ("props/curved.rs: require_zero: name", FORWARDS),
    ("props/curved.rs: torus_chart: at", CARRIER_CALL),
    ("props/curved.rs: torus_lift_as: name", FORWARDS),
    (
        "props/curved/turn_rows.rs: old_apex_loop_as: name",
        FORWARDS,
    ),
    (
        "props/curved/turn_rows.rs: old_pole_arc_full_as: name",
        FORWARDS,
    ),
    (
        "props/curved/turn_rows.rs: old_rim_only_closed_as: name",
        FORWARDS,
    ),
    ("props/curved/turn_rows.rs: old_span_as: forward", FORWARDS),
    ("props/curved/turn_rows.rs: old_span_as: winding", FORWARDS),
    (
        "props/curved/turn_rows.rs: the_apex_loop_sweep_agrees_with_the_old_test: in_band",
        CARRIER_CALL,
    ),
    (
        "props/curved/turn_rows.rs: the_apex_loop_sweep_agrees_with_the_old_test: total",
        CARRIER_CALL,
    ),
    (
        "props/curved/turn_rows.rs: the_circle_span_doors_agree_with_the_old_pair: span",
        CARRIER_CALL,
    ),
    (
        "props/curved/turn_rows.rs: the_cone_ellipse_door_agrees_with_the_old_pair: span",
        CARRIER_CALL,
    ),
    (
        "props/curved/turn_rows.rs: the_pole_arc_full_read_agrees_with_the_old_test: span",
        CARRIER_CALL,
    ),
    (
        "props/curved/turn_rows.rs: the_rim_only_closure_agrees_with_the_old_excess: du",
        CARRIER_CALL,
    ),
    (
        "props/quad/rotational.rs: rotational_cut_face_rounds: &entries",
        SAME_NAMED,
    ),
    ("props/quad/run.rs: ask: ", CARRIER_CALL),
    ("props/quad/run.rs: classify_len_as: name", FORWARDS),
    (
        "quartic.rs: depressed_quartic_roots: rows.depth()",
        ROW_FIELD,
    ),
    (
        "quartic.rs: depressed_quartic_roots: rows.disc()",
        ROW_FIELD,
    ),
    ("quartic.rs: depressed_quartic_roots: rows.odd()", ROW_FIELD),
    (
        "quartic.rs: depressed_quartic_roots: rows.shape()",
        ROW_FIELD,
    ),
    (
        "quartic.rs: depressed_quartic_roots: rows.split()",
        ROW_FIELD,
    ),
    (
        "quartic.rs: depressed_quartic_roots: rows.split_lead()",
        ROW_FIELD,
    ),
    ("roots.rs: extreme_sign: row", FORWARDS),
    ("roots.rs: spread_readings: row", FORWARDS),
    ("roots.rs: spread_signs: row", FORWARDS),
    ("rows.rs: names: ", ACCESSOR_LIKE_A_DOOR),
    ("rows.rs: quartic: &self.quartic", ROW_FIELD),
    ("rows.rs: verify: &self.verify", ROW_FIELD),
    ("ssi.rs: cylinder_sphere_ssi: a", NOT_A_NAME),
    ("ssi.rs: cylinder_torus_ssi: a", NOT_A_NAME),
    ("ssi.rs: sphere_torus_ssi: a", NOT_A_NAME),
    ("ssi.rs: torus_torus_ssi: a", NOT_A_NAME),
    ("ssi/boundary.rs: graze: root", NOT_A_NAME),
    ("ssi/boundary.rs: run: *root", NOT_A_NAME),
    (
        "ssi/certify.rs: -: value_limb: SsiLimb::ImageMiss",
        STRUCT_UPDATE,
    ),
    ("ssi/certify.rs: hull_limb: arm.sup", ROW_FIELD),
    ("ssi/certify.rs: hull_limb: arm.value", ROW_FIELD),
    ("ssi/ends.rs: hermite: self.sys", NOT_A_NAME),
    ("ssi/handle.rs: settle: self.sys", NOT_A_NAME),
    ("ssi/march.rs: march: sys", NOT_A_NAME),
    ("ssi/refine.rs: halve: sys", NOT_A_NAME),
    (
        "ssi/section.rs: a_refused_slope_refuses_where_the_crossing_is_decided: root(0.0)",
        NOT_A_NAME,
    ),
    (
        "ssi/section.rs: a_refused_slope_refuses_where_the_crossing_is_decided: root(f64::NAN)",
        NOT_A_NAME,
    ),
    ("ssi/section.rs: band_verdict: predicate", FORWARDS),
    ("ssi/section.rs: roots: found", NOT_A_NAME),
    (
        "ssi/system.rs: the_marchs_transversality_refusal_names_its_lever: &sys",
        NOT_A_NAME,
    ),
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
