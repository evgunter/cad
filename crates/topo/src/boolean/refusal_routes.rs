//! **What a Boolean refusal says for the decision that raised it**:
//! the two tables its predicate-routed sentences read, each held once.
//!
//! - [`contradiction`]: which fact contradicted a declared face pair
//!   (`BooleanError::DeclarationContradicted`,
//!   `MergeCoplanarError::DeclarationContradicted`), and whether a
//!   designed clearance could explain it
//!   (`contact_verify::fit_steer`).
//! - [`escalation`]: what `BooleanError::Escalated` renders for the
//!   decision that escalated. A coincidence between the two solids
//!   keeps the coincidence story and its declare lever; a decision no
//!   face-pair declaration can name renders its own words
//!   ([`super::decision_words`]) and its own lever; a kernel
//!   self-check ends as a defect.
//!
//! Both match on the predicate's NAME, which is routing and never
//! reaches the sentence. A name [`escalation`] does not carry renders
//! the gap sentence (`geom_core::MissingRecourse`) under
//! `geom_core::UNNAMED_DECISION`, which the refusal-shape guard reads
//! as no subject; `tests::every_name_the_boolean_escalates_with_is_routed`
//! enumerates the names that reach `BooleanError::Escalated` and reds
//! on one this table does not route.

use geom_core::{Indeterminate, NO_DECLARATION_RECOURSE};

/// The one lever a contradicted declaration leaves: the declaration
/// is wrong, or the geometry is.
pub(crate) const CONTRADICTION_RECOURSE: &str =
    "Recourse: fix the declaration or move the geometry";

/// The fact every contradiction states, for a name [`contradiction`]
/// does not carry: a declared pair is contradicted only when its
/// carriers are definitely not one surface.
const ANY_CONTRADICTION: &str = "the declared faces do not lie on one surface";

/// Which fact contradicted a declared pair.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Contradiction {
    /// The fact, as a clause with no colon or dash of its own.
    pub(crate) fact: &'static str,
    /// Whether the fact is a SEPARATION a designed clearance could
    /// explain (a radius difference, a centre offset, a parallel
    /// offset) rather than an angle or a kind no gap reconciles.
    pub(crate) fits_a_clearance: bool,
}

/// The fact behind a contradicted declaration, by the predicate that
/// decided it (`plane_eq`'s declared rung and `carrier_eq`'s kind and
/// data rungs). The torus separations carry no clearance steer: the
/// steer's own inventory names the plane, sphere and cylinder
/// separations.
#[must_use]
pub(crate) fn contradiction(predicate: Option<&str>) -> Option<Contradiction> {
    let (fact, fits_a_clearance) = match predicate? {
        "bool_plane_parallel" => ("the declared planes are not parallel", false),
        "bool_plane_offset" => ("the declared planes are parallel but apart", true),
        "carrier_kind" => ("the declared faces are different kinds of surface", false),
        "carrier_cyl_axis_parallel" => ("the declared cylinders' axes are not parallel", false),
        "carrier_cyl_axis_offset" => ("the declared cylinders' axes are parallel but apart", true),
        "carrier_cyl_radius" => ("the declared cylinders' radii differ", true),
        "carrier_sphere_center" => ("the declared spheres' centres differ", true),
        "carrier_sphere_radius" => ("the declared spheres' radii differ", true),
        "carrier_torus_axis_parallel" => ("the declared tori's axes are not parallel", false),
        "carrier_torus_center" => ("the declared tori's centres differ", false),
        "carrier_torus_major_radius" => ("the declared tori's major radii differ", false),
        "carrier_torus_minor_radius" => ("the declared tori's tube radii differ", false),
        _ => return None,
    };
    Some(Contradiction {
        fact,
        fits_a_clearance,
    })
}

/// [`contradiction`]'s fact for `diag`, or the fact every
/// contradiction shares.
#[must_use]
pub(crate) fn contradicted_fact(diag: &Indeterminate) -> &'static str {
    contradiction(diag.predicate).map_or(ANY_CONTRADICTION, |c| c.fact)
}

/// What `BooleanError::Escalated` renders for one decision.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Escalation {
    /// Parts of the two solids too close to call: a coincidence a
    /// face-pair declaration can settle.
    Coincidence,
    /// A decision no face-pair declaration names: its own words and
    /// its own lever (the recourse, without its label).
    Own {
        /// What was being decided.
        subject: &'static str,
        /// The lever.
        recourse: &'static str,
    },
    /// The kernel checking its own result: no lever in the model.
    KernelCheck {
        /// What the check decides.
        subject: &'static str,
    },
}

/// How a name routes: the coincidence story, its own lever (whose
/// words are [`super::decision_words`]), or a kernel check.
#[derive(Clone, Copy)]
enum Lever {
    Coincidence,
    Own(&'static str),
    Kernel(&'static str),
}

/// A corner's own shape (`sector_shape`'s rungs).
const CORNER: Lever = Lever::Own(
    "reshape that corner so its edges are clearly longer than the tolerance and clearly not \
     in line, or lower the tolerance",
);

/// Whether a pierce point lies on the curved face it pierces.
const PIERCE_POINT: Lever = Lever::Own(
    "move the parts so that point lands clearly on or clearly off the curved face, or lower \
     the tolerance",
);

/// A pierced torus face's own shape.
const TORUS_SHAPE: Lever = Lever::Own(
    "reshape the torus so its tube is clearly thicker than the tolerance and stays clearly \
     off its axis, or lower the tolerance",
);

/// Whether a point lies inside a face.
const CONTAINMENT: Lever = Lever::Own(
    "move the parts so they meet clearly inside or clearly outside that face's boundary, or \
     lower the tolerance",
);

/// Where a crossing lands along its edge, or an arc's own span: the
/// levers a split, a blend and a Boolean all have.
const EDGE: Lever = Lever::Own(NO_DECLARATION_RECOURSE);

/// How `BooleanError::Escalated` reads for the decision `predicate`
/// names; `None` for a name this table does not carry.
#[must_use]
pub(crate) fn escalation(predicate: Option<&str>) -> Option<Escalation> {
    let name = predicate?;
    Some(match lever(name)? {
        Lever::Coincidence => Escalation::Coincidence,
        Lever::Kernel(subject) => Escalation::KernelCheck { subject },
        Lever::Own(recourse) => Escalation::Own {
            subject: super::decision_words(name)?,
            recourse,
        },
    })
}

/// The one table: every name that reaches `BooleanError::Escalated`,
/// by the decision's lever. Grouped by the raise path each group
/// arrives through.
fn lever(name: &str) -> Option<Lever> {
    Some(match name {
        // The plane and carrier identity rungs (`plane_eq`,
        // `carrier_eq`), the tangent locus and the declared-tangent
        // verification: the two solids' faces coincide, or nearly.
        "bool_plane_parallel"
        | "bool_plane_orient"
        | "bool_plane_offset"
        | "carrier_sphere_center"
        | "carrier_sphere_radius"
        | "carrier_cyl_axis_parallel"
        | "carrier_cyl_axis_offset"
        | "carrier_cyl_radius"
        | "carrier_torus_axis_parallel"
        | "carrier_torus_center"
        | "carrier_torus_major_radius"
        | "carrier_torus_minor_radius"
        | "tangent_locus_axis_parallel"
        | "tangent_locus_side"
        | "tangent_locus_gap"
        | "contact_tangent_on_1"
        | "contact_tangent_on_2"
        | "contact_tangent_opposed"
        | "contact_tangent_second_order"
        | "contact_tangent_parallel"
        | "rim_circle_radius"
        | "rim_circle_center"
        | "rim_circle_axis_parallel"
        | "dihedral_arm"
        | "dihedral_wedge"
        | "tangent_second_order"
        | "material_wedge_side"
        | "material_cusp_side"
        // The section and join escalations (`join`, `rest`'s segment
        // walk, and `geom_brep`'s pair sections).
        | "bool_join_chord"
        | "bool_join_nearest"
        | "bool_join_facing"
        | "bool_join_arc_facing"
        | "bool_germ_frame_axes_parallel"
        | "bool_germ_frame_axes_coplanar"
        | "bool_ring_run_winding"
        | "pc_axis_plane_parallel"
        | "pc_parallel_gap"
        | "pc_rim_alignment"
        | "ps_center_gap"
        | "ss_carrier_identity"
        | "ss_carrier_external"
        | "ss_carrier_internal"
        | "cc_declared_radius_equality"
        | "cc_axes_parallel"
        | "cc_coaxial"
        | "cc_axes_coplanar"
        | "cc_parallel_gap"
        | "cs_cylinder_radius"
        | "cs_sphere_radius"
        | "cs_declared_coaxial"
        | "cs_wall_reach"
        // An edge of one solid against a face of the other (`reduce`'s
        // sweep, its curved-face arm and root lanes).
        | "bool_conic_face_plane_offset"
        | "bool_vertex_face_side"
        | "bool_line_cylinder_clearance"
        | "bool_circle_curved_clearance"
        | "split_conic_plane_parallel"
        | "split_conic_belly_graze"
        | "bool_point_in_solid_denom"
        | "bool_ray_cylinder_disc"
        | "bool_ray_torus_disc"
        | "bool_ray_torus_shape"
        | "bool_ray_torus_depth"
        | "bool_ray_torus_odd"
        | "bool_ray_torus_split"
        | "bool_ray_torus_split_lead"
        | "bool_circle_torus_coaxial_tilt"
        | "bool_circle_torus_coaxial_offset"
        | "bool_circle_torus_plane_height"
        | "bool_circle_torus_contour_residual"
        | "bool_circle_torus_contour_side"
        | "bool_circle_torus_root_slack"
        | "bool_circle_torus_pole"
        | "bool_circle_torus_pole_conditioning"
        | "bool_circle_torus_noise"
        | "bool_circle_torus_disc"
        | "bool_circle_torus_shape"
        | "bool_circle_torus_depth"
        | "bool_circle_torus_odd"
        | "bool_circle_torus_split"
        | "bool_circle_torus_split_lead"
        // The vertex, sector and edge-edge classification (`sectors`,
        // `vtxfac`, `insert`, `recl`, and `geom_brep`'s
        // `enters_material`).
        | "bool_chord_side"
        | "bool_pierce_sector_side_curved"
        | "enters_material"
        | "enters_material_arm"
        | "tangent_sector_order2"
        | "tangent_sector_order2_arm"
        | "bool_sector_within"
        | "bool_sector_bisector_side"
        | "bool_sector_coplanar"
        | "bool_dir_parallel"
        | "bool_dir_same"
        | "bool_faces_parallel"
        | "bool_germ_line"
        | "bool_strut_order"
        | "bool_ee_collinear"
        // The sphere extent and re-cut lanes (`ops`).
        | "bool_sphere_extent_gap"
        | "bool_sphere_sphere_gap"
        | "bool_sphere_sphere_nested"
        | "bool_sphere_escape_parallel"
        | "bool_sphere_recut_align" => Lever::Coincidence,
        // `SectorFault::Rung` (`sectors`): a corner's own shape.
        "sector_arm" | "sector_reflex" | "sector_straight" => CORNER,
        // `NormalAtError::Escalated` (`vtxfac`): the pierced face's
        // normal at the pierce point.
        "bool_pierce_normal_on_chart" => PIERCE_POINT,
        "torus_tube_positive" | "ring_torus_convention" => TORUS_SHAPE,
        // `ContainError::Escalated` (`reduce`, `ops`): whether a point
        // lies inside a face — the planar walk, the arc walk, and the
        // curved faces' trims.
        "bool_face_disc_carrier"
        | "bool_contact_vertex"
        | "bool_contact_arc_end_vertex"
        | "bool_contact_arc"
        | "bool_curved_contain_carrier"
        | "bool_curved_contain_period"
        | "bool_wall_trim"
        | "bool_wall_junction"
        | "bool_wall_outline_reach"
        | "bool_wall_piece_span"
        | "bool_wall_rim_level"
        | "bool_wall_section_tilt"
        | "bool_wall_trim_period"
        | "bool_wall_iso_meridian"
        | "bool_wall_iso_rim"
        | "bool_wall_section_seat"
        | "bool_sphere_iso_meridian"
        | "bool_sphere_iso_rim"
        | "bool_torus_trim_major_period"
        | "bool_torus_trim_minor_period"
        | "bool_sphere_trim"
        | "bool_sphere_trim_antipode"
        | "bool_sphere_trim_latitude"
        | "bool_sphere_trim_meridian_span"
        | "bool_sphere_trim_period"
        | "bool_sphere_trim_pole"
        | "bool_sphere_trim_pole_end"
        | "bool_sphere_trim_pole_interior"
        | "bool_torus_chart_affine"
        | "bool_torus_chart_box"
        | "bool_torus_chart_closure"
        | "bool_torus_frame_radius"
        | "bool_torus_trim"
        | "bool_cone_chart_box"
        | "bool_cone_group_slant"
        | "bool_cone_trim"
        | "bool_cone_trim_nappe"
        | "bool_cone_trim_period"
        | "bool_cone_trim_side"
        | "bool_ray_cone_apex"
        | "bool_ray_cone_nappe"
        | "point_in_loop_segment"
        | "point_in_loop_boundary"
        | "point_in_loop_side"
        | "point_in_loop_advance"
        | "point_in_loop_arm"
        | "point_in_arc_loop_segment"
        | "point_in_arc_loop_boundary"
        | "point_in_arc_loop_boundary_disagreement"
        | "point_in_arc_loop_side"
        | "point_in_arc_loop_advance"
        | "point_in_arc_loop_arm"
        | "point_in_arc_loop_reach"
        | "point_in_arc_loop_conic_span"
        | "point_in_arc_loop_conic_on"
        | "point_in_arc_loop_conic_end"
        | "point_in_arc_loop_conic_trim"
        | "point_in_arc_loop_conic_straddle"
        | "point_in_arc_loop_conic_window"
        | "point_in_arc_loop_conic_disc"
        | "point_in_arc_loop_conic_advance" => CONTAINMENT,
        // Where a crossing lands along its edge (`reduce`'s root
        // lanes), and an arc's own span: a face pair names neither.
        "split_conic_crossing_root"
        | "split_conic_root_order"
        | "bool_wall_root_in_span"
        | "bool_split_span_period" => EDGE,
        // The invariant lane and the quartic ladders' count
        // cross-check: reached only with a poisoned or impossible
        // reading.
        "volume_backstop" | "volume_backstop_operand" | "volume_backstop_violation" => {
            Lever::Kernel("whether the result's volume agrees with its operands'")
        }
        "bool_circle_torus_count" | "bool_ray_torus_count" => {
            Lever::Kernel("whether two counts of a quartic's roots agree")
        }
        _ => return None,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::boolean::{BooleanError, CarrierDesc, CarrierEqError, Operand, PlaneIdentity};
    use crate::entity::{EdgeKey, VertexKey};
    use crate::euler::EulerOpError;
    use crate::merge_faces::MergeCoplanarError;
    use crate::splitting::SplitReduceError;
    use geom_core::{Band, MarginDiag, Point3, Tol, Vec3};
    use std::collections::BTreeSet;
    use test_utils::refusal::{recourse_markers, stage_prefixes, subjectless_escalations};
    use test_utils::source::{NameCarrier, code_and_literals, crate_dir, predicate_census};

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness band forms")
    }

    fn diag_of(name: &'static str, margin: MarginDiag) -> Indeterminate {
        Indeterminate {
            margin,
            band: band(),
            predicate: Some(name),
            terminal_sliver: false,
        }
    }

    /// An escalation under `name`, its margin inside the run's band.
    fn in_band(name: &'static str) -> Indeterminate {
        let b = band();
        diag_of(name, MarginDiag::value((b.zero() + b.escalate()) / 2.0))
    }

    fn escalated(diag: Indeterminate) -> String {
        BooleanError::Escalated { diag }.to_string()
    }

    /// The refusal-shape guard's three checks on one rendered text:
    /// exactly one recourse, a subject for every escalation payload,
    /// and no stage prefix other than the `filed` ones.
    fn short_of_the_guard(text: &str, filed: &[&str]) -> Vec<String> {
        let mut out = Vec::new();
        match recourse_markers(text) {
            1 => {}
            n => out.push(format!("{n} recourses, not one")),
        }
        for clause in subjectless_escalations(text) {
            out.push(format!("an escalation with no subject ({clause:?})"));
        }
        for prefix in stage_prefixes(text, &[]) {
            if !filed.iter().any(|f| prefix.starts_with(f)) {
                out.push(format!("the stage prefix {prefix:?}"));
            }
        }
        out
    }

    const NO_DECLARATION_ENDING: &str = "Recourse: move the geometry, or lower the tolerance";

    /// The split door's own clause, a stage for a subject, filed with
    /// its owner: `work/reach/reach-refusals-short-of-the-shape-guard.md`.
    const SPLIT_DOOR_FILED: &str = "inserting the plane crossing on edge";

    /// **`split_edge`'s in-band interiority reads whole at every door
    /// that forwards it**, on a real raise: the split and the Boolean
    /// here (the blend's door is `sweep`'s, and its row is there). The
    /// subject is the decision, the one recourse is the lever every
    /// splitting door has, and no door is offered a declaration.
    #[test]
    fn the_split_param_escalation_reads_whole_at_every_splitting_door() {
        let raise = || {
            let cube = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness());
            let mut body = cube.body;
            let edge = cube.mevs[0].edge;
            let b = band();
            let err = body
                .split_edge(edge, (b.zero() + b.escalate()) * 0.5, Tol::witness())
                .unwrap_err();
            assert!(
                matches!(err, EulerOpError::SplitParamEscalated { .. }),
                "the band-midpoint split escalates: {err:?}"
            );
            err
        };
        let edge = EdgeKey::default();
        let rendered = [
            ("the operator", raise().to_string()),
            (
                "the split",
                SplitReduceError::CrossingInsertion {
                    edge,
                    endpoints: (VertexKey::default(), VertexKey::default()),
                    source: raise(),
                }
                .to_string(),
            ),
            (
                "the Boolean",
                BooleanError::CrossingInsertion {
                    operand: Operand::A,
                    edge,
                    source: raise(),
                }
                .to_string(),
            ),
        ];
        for (door, text) in rendered {
            let problems = short_of_the_guard(&text, &[SPLIT_DOOR_FILED]);
            assert!(problems.is_empty(), "{door}: {problems:?}: {text}");
            assert!(
                text.contains(
                    "whether a crossing lands strictly inside its edge is undecided: margin "
                ) && text.ends_with(NO_DECLARATION_ENDING)
                    && !text.contains("declare"),
                "{door} states the decision and the lever it has, and no declaration: {text}"
            );
        }
    }

    /// Each lever the escalation table routes to, by one name: the
    /// subject it states and the recourse it ends on, independent of
    /// the table.
    const LEVERS: &[(&str, &str, &str)] = &[
        (
            "bool_plane_offset",
            "parts of the two solids are too close to call at this tolerance (margin ",
            "Recourse: declare the coincidence, move the geometry, or lower the tolerance",
        ),
        (
            "sector_arm",
            "whether a corner's edges are long enough to measure its angle over is undecided: ",
            "Recourse: reshape that corner so its edges are clearly longer than the tolerance \
             and clearly not in line, or lower the tolerance",
        ),
        (
            "sector_straight",
            "whether a corner is straight or folds back on itself is undecided: ",
            "Recourse: reshape that corner so its edges are clearly longer than the tolerance \
             and clearly not in line, or lower the tolerance",
        ),
        (
            "bool_pierce_normal_on_chart",
            "whether a point lies on a curved face, so the face's normal can be read there is \
             undecided: ",
            "Recourse: move the parts so that point lands clearly on or clearly off the curved \
             face, or lower the tolerance",
        ),
        (
            "ring_torus_convention",
            "whether a torus's tube stays clear of its axis is undecided: ",
            "Recourse: reshape the torus so its tube is clearly thicker than the tolerance and \
             stays clearly off its axis, or lower the tolerance",
        ),
        (
            "point_in_loop_side",
            "whether a point lies inside a face, on its boundary, or outside it is undecided: ",
            "Recourse: move the parts so they meet clearly inside or clearly outside that \
             face's boundary, or lower the tolerance",
        ),
        (
            "bool_wall_trim",
            "whether a point lies inside a face, on its boundary, or outside it is undecided: ",
            "Recourse: move the parts so they meet clearly inside or clearly outside that \
             face's boundary, or lower the tolerance",
        ),
        (
            "split_conic_crossing_root",
            "whether a crossing lands strictly inside its edge is undecided: ",
            NO_DECLARATION_ENDING,
        ),
        (
            "volume_backstop",
            "whether the result's volume agrees with its operands' is undecided: ",
            geom_core::KERNEL_DEFECT_ENDING,
        ),
    ];

    /// **`BooleanError::Escalated` states the decision that escalated
    /// and that decision's lever**, at every lever the table routes to,
    /// for an in-band and a poisoned margin alike. Only the coincidence
    /// offers a declaration: the others' decisions are ones no face
    /// pair names.
    #[test]
    fn every_escalation_lever_renders_its_subject_and_its_recourse() {
        for &(name, subject, ending) in LEVERS {
            for diag in [in_band(name), diag_of(name, MarginDiag::INVALID)] {
                let text = escalated(diag);
                let problems = short_of_the_guard(&text, &[]);
                assert!(problems.is_empty(), "{name}: {problems:?}: {text}");
                assert!(
                    text.starts_with(subject) && text.ends_with(ending),
                    "{name} states its own subject and lever: {text}"
                );
                assert_eq!(
                    text.contains("declare"),
                    name == "bool_plane_offset",
                    "{name}: only the coincidence offers a declaration: {text}"
                );
                assert!(
                    !text.contains(name),
                    "{name}'s routing name stays out: {text}"
                );
            }
        }
    }

    /// **A name the table does not carry states the hole**, under the
    /// subject the shape guard reads as none, so a row rendering it is
    /// red; it is never the coincidence story by default.
    #[test]
    fn an_unrouted_name_states_the_hole() {
        for predicate in [Some("roster_unknown_probe"), None] {
            let text = escalated(Indeterminate {
                predicate,
                ..in_band("roster_unknown_probe")
            });
            assert!(
                text.starts_with(geom_core::UNNAMED_DECISION)
                    && text.ends_with(&geom_core::MissingRecourse(predicate).to_string())
                    && !text.contains("roster_unknown_probe")
                    && !subjectless_escalations(&text).is_empty(),
                "{text}"
            );
        }
    }

    /// The funnel calls under `src/boolean` whose name the census cannot
    /// read at the call, and where the names they carry are listed.
    const INDIRECT: &[(&str, &str)] = &[
        (
            "carrier_eq.rs: name",
            "`data_rungs`: the kind arms' names, in HAND",
        ),
        (
            "circle_torus.rs: rows.conditioning",
            "`HalfAngleRows`, a declared carrier",
        ),
        (
            "circle_torus.rs: rows.noise",
            "`HalfAngleRows`, a declared carrier",
        ),
        (
            "circle_torus.rs: rows.pole",
            "`HalfAngleRows`, a declared carrier",
        ),
        (
            "circle_torus.rs: rows.root_slack",
            "`HalfAngleRows`, a declared carrier",
        ),
        (
            "contact_verify.rs: name",
            "the tangent verification's on-surface pair, in HAND",
        ),
        ("contain.rs: END_VERTEX", "a declared const carrier"),
        (
            "rim_wedge.rs: name",
            "the shared rim's identity rungs, in HAND",
        ),
        (
            "section_cert.rs: name",
            "`sign`, which keeps a verdict and drops an escalation",
        ),
        (
            "solid_contain.rs: name",
            "the trims' `zero` and `window` gates, in HAND",
        ),
        (
            "solid_contain.rs: rows.depth",
            "`QuarticRows`, a declared carrier",
        ),
        (
            "solid_contain.rs: rows.disc",
            "`QuarticRows`, a declared carrier",
        ),
        (
            "solid_contain.rs: rows.odd",
            "`QuarticRows`, a declared carrier",
        ),
        (
            "solid_contain.rs: rows.shape",
            "`QuarticRows`, a declared carrier",
        ),
        (
            "solid_contain.rs: rows.split",
            "`QuarticRows`, a declared carrier",
        ),
        (
            "solid_contain.rs: rows.split_lead",
            "`QuarticRows`, a declared carrier",
        ),
    ];

    /// The names that reach `BooleanError::Escalated` which the census
    /// of `src/boolean` cannot see — decided in another module or crate
    /// on the escalation's path, carried where the census does not
    /// read, or built as a payload rather than decided — each with the
    /// file (from the crate root) that spells it.
    const HAND: &[(&str, &str)] = &[
        ("carrier_sphere_center", "src/boolean/carrier_eq.rs"),
        ("carrier_sphere_radius", "src/boolean/carrier_eq.rs"),
        ("carrier_cyl_axis_parallel", "src/boolean/carrier_eq.rs"),
        ("carrier_cyl_axis_offset", "src/boolean/carrier_eq.rs"),
        ("carrier_cyl_radius", "src/boolean/carrier_eq.rs"),
        ("carrier_torus_axis_parallel", "src/boolean/carrier_eq.rs"),
        ("carrier_torus_center", "src/boolean/carrier_eq.rs"),
        ("carrier_torus_major_radius", "src/boolean/carrier_eq.rs"),
        ("carrier_torus_minor_radius", "src/boolean/carrier_eq.rs"),
        ("contact_tangent_on_1", "src/boolean/contact_verify.rs"),
        ("contact_tangent_on_2", "src/boolean/contact_verify.rs"),
        ("rim_circle_radius", "src/boolean/rim_wedge.rs"),
        ("rim_circle_center", "src/boolean/rim_wedge.rs"),
        ("rim_circle_axis_parallel", "src/boolean/rim_wedge.rs"),
        ("bool_sector_bisector_side", "src/boolean/sectors.rs"),
        ("bool_wall_iso_meridian", "src/boolean/solid_contain.rs"),
        ("bool_wall_iso_rim", "src/boolean/solid_contain.rs"),
        ("bool_wall_section_seat", "src/boolean/solid_contain.rs"),
        ("bool_sphere_iso_meridian", "src/boolean/solid_contain.rs"),
        ("bool_sphere_iso_rim", "src/boolean/solid_contain.rs"),
        (
            "bool_torus_trim_major_period",
            "src/boolean/solid_contain.rs",
        ),
        (
            "bool_torus_trim_minor_period",
            "src/boolean/solid_contain.rs",
        ),
        ("sector_arm", "src/sector_shape.rs"),
        ("sector_reflex", "src/sector_shape.rs"),
        ("sector_straight", "src/sector_shape.rs"),
        ("bool_pierce_normal_on_chart", "src/face_normal.rs"),
        ("split_conic_plane_parallel", "src/splitting/classify.rs"),
        ("split_conic_belly_graze", "src/splitting/classify.rs"),
        ("split_conic_crossing_root", "src/splitting/classify.rs"),
        ("split_conic_root_order", "src/splitting/classify.rs"),
        ("point_in_loop_segment", "src/splitting/containment.rs"),
        ("point_in_loop_boundary", "src/splitting/containment.rs"),
        ("point_in_loop_side", "src/splitting/containment.rs"),
        ("point_in_loop_advance", "src/splitting/containment.rs"),
        ("point_in_loop_arm", "src/splitting/containment.rs"),
        ("point_in_arc_loop_segment", "src/splitting/containment.rs"),
        ("point_in_arc_loop_boundary", "src/splitting/containment.rs"),
        (
            "point_in_arc_loop_boundary_disagreement",
            "src/splitting/containment.rs",
        ),
        ("point_in_arc_loop_side", "src/splitting/containment.rs"),
        ("point_in_arc_loop_advance", "src/splitting/containment.rs"),
        ("point_in_arc_loop_arm", "src/splitting/containment.rs"),
        ("point_in_arc_loop_reach", "src/splitting/containment.rs"),
        (
            "point_in_arc_loop_conic_span",
            "src/splitting/containment.rs",
        ),
        ("point_in_arc_loop_conic_on", "src/splitting/containment.rs"),
        (
            "point_in_arc_loop_conic_end",
            "src/splitting/containment.rs",
        ),
        (
            "point_in_arc_loop_conic_trim",
            "src/splitting/containment.rs",
        ),
        (
            "point_in_arc_loop_conic_straddle",
            "src/splitting/containment.rs",
        ),
        (
            "point_in_arc_loop_conic_window",
            "src/splitting/containment.rs",
        ),
        (
            "point_in_arc_loop_conic_disc",
            "src/splitting/containment.rs",
        ),
        (
            "point_in_arc_loop_conic_advance",
            "src/splitting/containment.rs",
        ),
        ("torus_tube_positive", "../geom/src/surfaces.rs"),
        ("ring_torus_convention", "../geom/src/surfaces.rs"),
        ("enters_material", "../geom-brep/src/enters.rs"),
        ("enters_material_arm", "../geom-brep/src/enters.rs"),
        ("tangent_sector_order2", "../geom-brep/src/enters.rs"),
        ("tangent_sector_order2_arm", "../geom-brep/src/enters.rs"),
        ("dihedral_arm", "../geom-brep/src/dihedral.rs"),
        ("dihedral_wedge", "../geom-brep/src/dihedral.rs"),
        ("material_wedge_side", "../geom-brep/src/dihedral.rs"),
        ("pc_axis_plane_parallel", "../geom-brep/src/intersect.rs"),
        ("pc_parallel_gap", "../geom-brep/src/intersect.rs"),
        ("pc_rim_alignment", "../geom-brep/src/intersect.rs"),
        ("ps_center_gap", "../geom-brep/src/intersect.rs"),
        ("ss_carrier_identity", "../geom-brep/src/intersect.rs"),
        ("ss_carrier_external", "../geom-brep/src/intersect.rs"),
        ("ss_carrier_internal", "../geom-brep/src/intersect.rs"),
        (
            "cc_declared_radius_equality",
            "../geom-brep/src/intersect.rs",
        ),
        ("cc_axes_parallel", "../geom-brep/src/intersect.rs"),
        ("cc_coaxial", "../geom-brep/src/intersect.rs"),
        ("cc_axes_coplanar", "../geom-brep/src/intersect.rs"),
        ("cc_parallel_gap", "../geom-brep/src/intersect.rs"),
        ("cs_cylinder_radius", "../geom-brep/src/intersect.rs"),
        ("cs_sphere_radius", "../geom-brep/src/intersect.rs"),
        ("cs_declared_coaxial", "../geom-brep/src/intersect.rs"),
        ("cs_wall_reach", "../geom-brep/src/intersect.rs"),
    ];

    /// The names `src/boolean` decides that never reach
    /// `BooleanError::Escalated`, and where they go instead.
    const NOT_ESCALATED: &[(&str, &str)] = &[
        ("bool_cone_closure_period", SWALLOWED),
        ("bool_point_in_solid_advance", RAY_CAST),
        ("bool_point_in_solid_infinity", RAY_CAST),
        ("bool_point_in_solid_order", RAY_CAST),
        ("bool_point_in_solid_plane", RAY_CAST),
        ("bool_ray_cone_disc", RAY_CAST),
        ("bool_ray_cone_incidence", RAY_CAST),
        ("bool_ray_cone_lead", RAY_CAST),
        ("bool_ray_sphere_disc", RAY_CAST),
        ("bool_ray_torus_incidence", RAY_CAST),
    ];

    const SWALLOWED: &str = "`ops::apex_closure_describes` reads the verdict as a yes-or-no and \
                             refuses nothing";
    const RAY_CAST: &str = "the point-in-solid ray cast, typed on `PointInSolidError` and \
                            rendered by `BooleanError::Containment`";

    fn census() -> test_utils::source::PredicateCensus {
        predicate_census(
            &crate_dir(env!("CARGO_MANIFEST_DIR")).join("src/boolean"),
            &[
                NameCarrier::Call("HalfAngleRows"),
                NameCarrier::Call("QuarticRows"),
                NameCarrier::Const("END_VERTEX"),
            ],
        )
    }

    /// Every name the Boolean escalates with: the census of
    /// `src/boolean` less what never reaches `Escalated`, and the names
    /// the census cannot see.
    fn raised() -> BTreeSet<String> {
        let not: BTreeSet<&str> = NOT_ESCALATED.iter().map(|(n, _)| *n).collect();
        census()
            .names
            .into_iter()
            .filter(|n| !not.contains(n.as_str()))
            .chain(HAND.iter().map(|(n, _)| (*n).to_owned()))
            .collect()
    }

    /// **Every name the Boolean escalates with is routed**, and renders
    /// a refusal the shape guard passes.
    ///
    /// The names are read from the source, not from the table: the
    /// literal `decide*` sites under `src/boolean`, less
    /// [`NOT_ESCALATED`], plus [`HAND`], whose every entry is checked
    /// to be spelled in its file. A new gate in `src/boolean` is
    /// raised-and-unrouted or listed, never neither; a table entry the
    /// enumeration does not hold is stale.
    #[test]
    fn every_name_the_boolean_escalates_with_is_routed() {
        let root = crate_dir(env!("CARGO_MANIFEST_DIR"));
        let census = census();
        let declared: BTreeSet<String> = INDIRECT.iter().map(|(s, _)| (*s).to_owned()).collect();
        assert_eq!(
            census.indirect, declared,
            "the unread funnel calls are not INDIRECT's"
        );
        assert!(census.unreadable.is_empty(), "{:?}", census.unreadable);
        for site in &census.unwalked {
            let (file, line) = site.rsplit_once(':').expect("<file>:<line>");
            let text = std::fs::read_to_string(root.join("src/boolean").join(file)).unwrap();
            let line: usize = line.parse().unwrap();
            let target = text.lines().nth(line - 1).unwrap();
            let path = target
                .split('"')
                .nth(1)
                .expect("a path attribute naming a file");
            assert!(
                root.join("src/boolean").join(path).is_file() && !path.contains('/'),
                "{site} pulls in {path}, which the census does not walk"
            );
        }
        for (name, reason) in NOT_ESCALATED {
            assert!(
                census.names.contains(*name),
                "{name} is stale in NOT_ESCALATED"
            );
            assert!(
                escalation(Some(name)).is_none(),
                "{name} is routed but listed as never escalating ({reason})"
            );
        }
        for (name, file) in HAND {
            let text = std::fs::read_to_string(root.join(file)).unwrap();
            assert!(
                code_and_literals(&text).contains(&format!("\"{name}\"")),
                "{name} is not spelled in {file}"
            );
        }
        let raised = raised();
        let mut problems = Vec::new();
        for name in &raised {
            let name: &'static str = Box::leak(name.clone().into_boxed_str());
            let text = escalated(in_band(name));
            if escalation(Some(name)).is_none() {
                problems.push(format!("{name} is raised and unrouted: {text}"));
            }
            for p in short_of_the_guard(&text, &[]) {
                problems.push(format!("{name}: {p}: {text}"));
            }
        }
        // The reverse: every name the table routes is one the
        // enumeration raises.
        let own = std::fs::read_to_string(root.join("src/boolean/refusal_routes.rs")).unwrap();
        let own = code_and_literals(&own);
        let body = &own[own.find("fn lever(").unwrap()..own.find("#[cfg(test)]").unwrap()];
        for literal in body.split('"').skip(1).step_by(2) {
            if !literal.contains(' ') && !raised.contains(literal) {
                problems.push(format!("the table routes {literal}, which nothing raises"));
            }
        }
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }

    /// Each contradiction predicate with the fact its refusal states,
    /// independent of the table.
    const FACTS: &[(&str, &str)] = &[
        (
            "bool_plane_parallel",
            "the declared planes are not parallel",
        ),
        (
            "bool_plane_offset",
            "the declared planes are parallel but apart",
        ),
        (
            "carrier_kind",
            "the declared faces are different kinds of surface",
        ),
        (
            "carrier_sphere_center",
            "the declared spheres' centres differ",
        ),
        (
            "carrier_sphere_radius",
            "the declared spheres' radii differ",
        ),
        (
            "carrier_cyl_axis_parallel",
            "the declared cylinders' axes are not parallel",
        ),
        (
            "carrier_cyl_axis_offset",
            "the declared cylinders' axes are parallel but apart",
        ),
        ("carrier_cyl_radius", "the declared cylinders' radii differ"),
        (
            "carrier_torus_axis_parallel",
            "the declared tori's axes are not parallel",
        ),
        ("carrier_torus_center", "the declared tori's centres differ"),
        (
            "carrier_torus_major_radius",
            "the declared tori's major radii differ",
        ),
        (
            "carrier_torus_minor_radius",
            "the declared tori's tube radii differ",
        ),
    ];

    /// A declared pair of `c1` and `c2`, verified by the real rung
    /// (`carrier_eq`, which `recl` and `vtxfac` call and `plane_eq`
    /// serves for planes): the contradiction it raises.
    fn contradicted(c1: CarrierDesc<f64>, c2: CarrierDesc<f64>) -> Indeterminate {
        let id = PlaneIdentity {
            s1: None,
            s2: None,
            declared: true,
        };
        match crate::boolean::carrier_eq(&c1, &c2, id, 1.0, band()) {
            Err(CarrierEqError::Contradicted(diag)) => diag,
            other => panic!("a declared pair this far apart contradicts: {other:?}"),
        }
    }

    /// **A contradicted declaration names the fact that contradicted
    /// it**, one clause per predicate, on the verdict the real rung
    /// raises for a pair built to trip that predicate alone — two
    /// cylinders of different radii among them, the non-planar pair
    /// `recl` raises. No payload (the margin is `INVALID` on a definite
    /// verdict), no declare menu, one recourse.
    #[test]
    fn every_contradiction_names_the_fact_that_contradicted() {
        let p = Point3::new;
        let (x, z) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let o = p(0.0, 0.0, 0.0);
        let plane = |origin, normal| CarrierDesc::Plane { origin, normal };
        let sphere = |center, radius| CarrierDesc::Sphere {
            center,
            radius,
            outward: true,
        };
        let cylinder = |origin, axis, radius| CarrierDesc::Cylinder {
            origin,
            axis,
            radius,
            outward: true,
        };
        let torus = |center, axis, major_radius, minor_radius| CarrierDesc::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            outward: true,
        };
        let pairs = [
            (plane(o, z), plane(o, x)),
            (plane(o, z), plane(p(0.0, 0.0, 1.0), z)),
            (plane(o, z), sphere(o, 1.0)),
            (sphere(o, 1.0), sphere(p(1.0, 0.0, 0.0), 1.0)),
            (sphere(o, 1.0), sphere(o, 2.0)),
            (cylinder(o, z, 1.0), cylinder(o, x, 1.0)),
            (cylinder(o, z, 1.0), cylinder(p(1.0, 0.0, 0.0), z, 1.0)),
            (cylinder(o, z, 1.0), cylinder(o, z, 2.0)),
            (torus(o, z, 2.0, 0.5), torus(o, x, 2.0, 0.5)),
            (torus(o, z, 2.0, 0.5), torus(p(0.0, 0.0, 1.0), z, 2.0, 0.5)),
            (torus(o, z, 2.0, 0.5), torus(o, z, 3.0, 0.5)),
            (torus(o, z, 2.0, 0.5), torus(o, z, 2.0, 0.25)),
        ];
        assert_eq!(pairs.len(), FACTS.len());
        for ((c1, c2), &(name, fact)) in pairs.into_iter().zip(FACTS) {
            let diag = contradicted(c1, c2);
            assert_eq!(diag.predicate, Some(name), "the pair trips {name}");
            let planar = name.starts_with("bool_plane");
            let mut texts = vec![(
                "the Boolean",
                BooleanError::DeclarationContradicted { diag }.to_string(),
                "the Boolean",
            )];
            if planar {
                texts.push((
                    "the merge",
                    MergeCoplanarError::DeclarationContradicted { diag }.to_string(),
                    "the merge",
                ));
            }
            for (door, text, who) in texts {
                assert_eq!(
                    text,
                    format!(
                        "a declared coincidence contradicts the geometry: {fact}, and {who} \
                         never glues a lie. Recourse: fix the declaration or move the geometry"
                    ),
                    "{door}, {name}"
                );
                let problems = short_of_the_guard(&text, &[]);
                assert!(problems.is_empty(), "{door}, {name}: {problems:?}: {text}");
                assert!(
                    planar || !text.contains("planes"),
                    "{door}: a {name} pair is not a pair of planes: {text}"
                );
            }
        }
    }

    /// **The merge meets a contradicted declaration on a real raise**:
    /// two faces of a brick that meet at an edge, declared one surface.
    #[test]
    fn a_declared_pair_of_meeting_faces_is_contradicted_at_the_merge() {
        let tol = Tol::witness();
        let mut body =
            crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let normal = |b: &crate::body::Body<f64>, s| match b.get_surface(s) {
            Some(&crate::Surface::Plane { normal, .. }) => normal,
            other => panic!("a brick face is a plane: {other:?}"),
        };
        let surfaces: Vec<_> = body.faces().map(|(_, f)| f.surface).collect();
        let first = surfaces[0];
        let meeting = *surfaces
            .iter()
            .find(|&&s| normal(&body, s).dot(normal(&body, first)).abs() < 0.5)
            .expect("a brick face meets four others");
        let err = body
            .merge_coplanar_faces_declared(&[(first, meeting)], tol)
            .expect_err("two meeting faces are not one plane");
        let MergeCoplanarError::DeclarationContradicted { diag } = &err else {
            panic!("the declared rung contradicts: {err:?}");
        };
        assert_eq!(diag.predicate, Some("bool_plane_parallel"));
        let text = err.to_string();
        assert_eq!(
            text,
            "a declared coincidence contradicts the geometry: the declared planes are not \
             parallel, and the merge never glues a lie. Recourse: fix the declaration or move \
             the geometry"
        );
        let wrapped = BooleanError::Merge(err).to_string();
        let problems = short_of_the_guard(&wrapped, &[]);
        assert!(problems.is_empty(), "{problems:?}: {wrapped}");
    }
}
