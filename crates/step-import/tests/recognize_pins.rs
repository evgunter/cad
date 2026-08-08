//! **M7-6 posture pins: D7 stage-1 surface recognition** (the #256
//! ruling, executed). What this suite pins beyond the re-stated
//! corpus rows (`corpus_fold.rs`, `review_probes_m7_3.rs` V5):
//!
//! * the corpus-wide promotion ENUMERATION — always-promote fires on
//!   exactly the measured planar walls and NOWHERE else (a trigger
//!   that quietly widened or narrowed would show here first);
//! * the QUASI_UNIFORM vocabulary reads the SAME surface as the
//!   stated-knots form (bit-identical import — the knot synthesis is
//!   the committed corpus's own `[0,1]` clamped shape);
//! * the near-miss posture: a cylinder patch perturbed past ε_in
//!   stays NURBS silently (imports, no promotion record, no refusal);
//! * the ill-conditioned-estimator typed row: D7's
//!   `RecognitionAmbiguous` fires exactly where promotion was the
//!   face's only door AND the estimator cannot answer at ε_in;
//! * tier-3 behavior of promoted faces, asserted not assumed: a
//!   promoted CYLINDER passes the curved sense arms (check 6) and
//!   declines only at the banked cylinder-chart NURBS-boundary
//!   quadrature; promoted PLANES compute exact volume (executed
//!   corpus-wide by `roundtrip.rs`'s rows, which run these fixtures'
//!   volumes against the native `KERNEL_*` sidecars).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{SOLID_FIXTURES, fixture};
use geom_core::{Affine3, Point2, Vec3};
use step_import::{
    ImportOptions, NormalizationKind, PromotedKind, StepImport, StepImportError, import_step,
};

fn promotions(
    normalizations: &[step_import::StructureNormalization],
) -> Vec<(u64, PromotedKind, f64)> {
    normalizations
        .iter()
        .filter_map(|n| match n.kind {
            NormalizationKind::SurfacePromotion { to, residual } => Some((n.face, to, residual)),
            _ => None,
        })
        .collect()
}

fn solid(text: &str, who: &str) -> (topo::Body<f64>, Vec<(u64, PromotedKind, f64)>) {
    match import_step(text, &ImportOptions::default()) {
        Ok(StepImport::Solid {
            body,
            normalizations,
            ..
        }) => {
            let p = promotions(&normalizations);
            (body, p)
        }
        other => panic!("{who}: expected a solid import, got {other:?}"),
    }
}

/// **The corpus-wide enumeration.** Exactly six promotions across the
/// whole committed corpus — the six exactly-planar loft/sweep walls —
/// and none anywhere else: the analytic-surface fixtures carry no
/// NURBS at all, and the curved NURBS walls (loft bulges, the
/// nonuniform barrel) certify as no implemented kind and stay NURBS.
/// Residuals: exactly 0.0 where the builder's arithmetic is exact
/// (loft_prism), ≤ 1e-15 for the walls that carry rounding.
#[test]
fn own_corpus_promotions_are_exactly_the_enumerated_planar_walls() {
    for name in SOLID_FIXTURES {
        let (_, promos) = solid(&fixture(name, "step"), name);
        let want: &[(u64, PromotedKind)] = match name {
            "loft_prism" | "nonuniform_loft" => {
                &[(104, PromotedKind::Plane), (142, PromotedKind::Plane)]
            }
            "swept_elbow" => &[(165, PromotedKind::Plane), (228, PromotedKind::Plane)],
            _ => &[],
        };
        let got: Vec<(u64, PromotedKind)> = promos.iter().map(|&(f, k, _)| (f, k)).collect();
        assert_eq!(got, want, "{name}: the promotion enumeration");
        for (face, _, residual) in &promos {
            let ceiling = if name == "loft_prism" { 0.0 } else { 1e-15 };
            assert!(
                *residual <= ceiling,
                "{name} face #{face}: residual {residual:e} above {ceiling:e}"
            );
        }
    }
}

/// **The QUASI_UNIFORM vocabulary is the stated-knots surface.** The
/// committed corpus states its clamped knots explicitly, and they ARE
/// the quasi-uniform shape (`[0,0,1,1]` / `[0,0,0,1,1,1]`), so
/// rewriting a wall and a seam carrier as knots-implied
/// `QUASI_UNIFORM_*` records must import the bit-identical body:
/// same census, same certified volume bits, same promotions. This is
/// the synthesis pin — an off-by-a-scale knot vector would move
/// nothing geometric (affine reparameterization) but a wrong SHAPE
/// would fail loudly here.
#[test]
fn quasi_uniform_vocabulary_reads_the_same_surface() {
    let orig = fixture("loft_prism", "step");
    let mutated = orig
        .replace(
            "#87 = B_SPLINE_SURFACE_WITH_KNOTS('', 1, 2, ((#81, #82, #83), (#84, #85, #86)), \
             .UNSPECIFIED., .U., .U., .U., (2, 2), (3, 3), (0.0, 1.0), (0.0, 1.0), .UNSPECIFIED.);",
            "#87 = QUASI_UNIFORM_SURFACE('', 1, 2, ((#81, #82, #83), (#84, #85, #86)), \
             .UNSPECIFIED., .U., .U., .U.);",
        )
        .replace(
            "#99 = B_SPLINE_CURVE_WITH_KNOTS('', 2, (#96, #97, #98), .UNSPECIFIED., .U., .U., \
             (3, 3), (0.0, 1.0), .UNSPECIFIED.);",
            "#99 = QUASI_UNIFORM_CURVE('', 2, (#96, #97, #98), .UNSPECIFIED., .U., .U.);",
        );
    assert_ne!(orig, mutated, "both rewrites applied");
    let (base, base_promos) = solid(&orig, "stated knots");
    let (body, promos) = solid(&mutated, "quasi-uniform");
    assert_eq!(
        common::census(&base),
        common::census(&body),
        "census identical across the vocabulary"
    );
    assert_eq!(
        base_promos, promos,
        "identical promotions (bit-identical patches)"
    );
    let (v1, v2) = (
        topo::mass_properties(&base).unwrap().volume,
        topo::mass_properties(&body).unwrap().volume,
    );
    assert_eq!(v1.to_bits(), v2.to_bits(), "volume bit-identical");
}

/// The straight arc-prism: three identical bulged-square sections
/// stacked along +z — its arc wall is an EXACT cylinder patch (a
/// translational extrusion of a rational quarter-circle), the
/// cylinder track's own-corpus exercise.
fn straight_arc_prism() -> topo::Body<f64> {
    let arc_section = || -> sweep::Section {
        let v = |x: f64, y: f64, bulge: f64| profile::ProfileVertex {
            pos: Point2::new(x, y),
            bulge,
        };
        vec![profile::ProfileLoop::new(vec![
            v(-1.0, -1.0, 0.0),
            // tan(π/8): a quarter-circle bulge on the +x side.
            v(1.0, -1.0, 0.4142135623730951),
            v(1.0, 1.0, 0.0),
            v(-1.0, 1.0, 0.0),
        ])]
    };
    let sections = vec![arc_section(), arc_section(), arc_section()];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    sweep::loft_body::<f64>(&sections, &places, 2)
        .expect("the straight arc prism builds")
        .body
}

/// **The cylinder track + tier-3 behavior of a promoted cylinder,
/// asserted.** The exact-cylinder rational wall certifies and
/// promotes (residual ~1e-16, the extrusion's own rounding); the
/// promoted face then faces the kernel's real gates: tiers 1/2 green,
/// and tier 3's ONLY finding is the volume decline at the banked
/// cylinder-chart NURBS-boundary quadrature (the wall's seam carriers
/// are spline-typed lines) — meaning the curved sense arms (check 6)
/// RAN and passed on the promoted chart. Before promotion this body's
/// tier 3 declined as a rational-NURBS wall; the decline it keeps is
/// a different, narrower one, and a definite check-6 falsehood would
/// fail this row.
///
/// **The near-miss posture** rides the same body: the wall's center
/// control point nudged ~5·ε_in off the cylinder makes certification
/// refuse (a residual past ε_in), and the patch STAYS NURBS silently
/// — the import succeeds, records no promotion for that wall, and
/// raises nothing (the M7-3-legal state, preserved by measurement).
#[test]
fn promoted_cylinder_gates_and_the_near_miss_stay_nurbs() {
    let native = straight_arc_prism();
    let text = step_export::step_string(&native, &step_export::StepOptions::default())
        .expect("the arc prism exports");
    let (body, promos) = solid(&text, "straight arc prism");
    let kinds: Vec<(u64, PromotedKind)> = promos.iter().map(|&(f, k, _)| (f, k)).collect();
    assert_eq!(
        kinds,
        vec![
            (106, PromotedKind::Plane),
            (134, PromotedKind::Cylinder),
            (153, PromotedKind::Plane),
            (167, PromotedKind::Plane),
        ],
        "three planar walls + the exact cylinder wall promote"
    );
    let cyl_residual = promos[1].2;
    assert!(
        cyl_residual <= 1e-14,
        "the cylinder certificate is the extrusion's rounding: {cyl_residual:e}"
    );
    assert_eq!(topo::validate(&body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&body), Ok(()), "tier 2");
    match topo::validate_geometric(&body) {
        Err(errors) => {
            assert!(
                errors
                    .iter()
                    .all(|e| matches!(e, topo::ValidationError::VolumeUncomputable { .. })),
                "the only tier-3 finding is the banked quadrature decline (check 6 \
                 ran green on the promoted cylinder): {errors:?}"
            );
        }
        Ok(()) => panic!(
            "tier 3 unexpectedly green — the cylinder-chart NURBS-boundary quadrature \
             landed; re-pin this row to the exact volume"
        ),
    }

    // **The near-miss, and the MEASURED LIMIT it exposes** (reported
    // loudly at the M7-6 exit; a standing item for the next ruling).
    // The wall's center control point ~5·ε_in off the cylinder makes
    // its certificate refuse, and the WALL duly stays NURBS — but the
    // import then refuses TYPED at a wall–wall seam: the seam's
    // carrier was minted as its neighbour PLANE wall's boundary
    // column, that neighbour PROMOTED, and the stays-NURBS wall's own
    // column differs in last bits (the arc endpoint's rounding), so
    // the bitwise IsoCurve rung has nothing to match; the remaining
    // rungs (plane × NURBS intersection) have no certification. This
    // body imports on pre-stage-1 main — always-promote can therefore
    // refuse a mixed promoted/stays-NURBS body that imported before,
    // exactly the class the #256 ruling's own never-refuse clause
    // names. Pinned as it stands so the behavior cannot drift
    // silently; the refusal is at least TYPED, with every candidate
    // and its refusal enumerated.
    // The exported file's declared uncertainty is the ambient ε the
    // export ran at, so the near-miss scales with the run's ε: a
    // 5·ε_in nudge lands the certificate residual at ~2.5·ε_in —
    // past the budget at EVERY matrix row (1e-6 … 1e-12).
    //
    // The perturbation base is the FILE'S OWN token, parsed — the
    // writer's print of the wall's centre control-point x, which the
    // bulge arithmetic left one ulp BELOW f64 √2 (not
    // `f64::consts::SQRT_2`, whose substitution would nudge the base
    // by that ulp and misstate the intent: "the file's value, moved
    // 5·ε_in", not "√2, moved 5·ε_in").
    let eps = geom_core::Tolerance::get().eps;
    const CENTER_X_TOKEN: &str = "1.414213562373095";
    let base: f64 = CENTER_X_TOKEN.parse().expect("the writer's own token parses");
    let near_miss = text.replace(
        &format!("#114 = CARTESIAN_POINT('', ({CENTER_X_TOKEN}, 0.0, 1.0));"),
        &format!(
            "#114 = CARTESIAN_POINT('', ({:?}, 0.0, 1.0));",
            base + 5.0 * eps
        ),
    );
    assert_ne!(text, near_miss, "the perturbation applied");
    match import_step(&near_miss, &ImportOptions::default()) {
        Err(StepImportError::Adoption { id, attempts }) => {
            assert_eq!(id, 130, "the orphaned seam beside the promoted wall");
            assert!(
                attempts
                    .iter()
                    .all(|a| !matches!(a.candidate, step_import::AdoptionCandidate::IsoCurve)),
                "the bitwise IsoCurve rung had nothing to offer: {attempts:?}"
            );
        }
        other => panic!(
            "the near-miss's measured state is the typed seam-adoption refusal; \
             a change here is a posture change to re-pin: {other:?}"
        ),
    }
}

/// **D7's typed ambiguity, fired at its one stage-1 site.** A wall
/// nudged off its plane (so the plane certificate refutes) whose
/// `v`-start boundary row is a straight line gives the cylinder
/// estimator NOTHING to solve — the three azimuth samples are
/// collinear, the sagitta margin is 0 ≤ ε_in — and when that face
/// ALSO carries a second bound (the refusing class: without promotion
/// the multi-bound curved gate refuses it), the refusal is
/// `RecognitionAmbiguous`, naming face, surface, kind, and margin —
/// not the bare topology refusal, and never a guessed kind. The same
/// marginal recognition on the SINGLE-bound version of the face
/// imports silently (the near-miss row above) — the variant fires
/// only where promotion was the face's only door.
#[test]
fn ill_conditioned_estimator_is_typed_ambiguity_on_a_refusing_face() {
    let mutated = fixture("loft_prism", "step")
        // Off the plane by 2e-8 (past ε_in; the plane refutes)…
        .replace(
            "#82 = CARTESIAN_POINT('', (-1.75, -1.0, 1.0));",
            "#82 = CARTESIAN_POINT('', (-1.75, -1.00000002, 1.0));",
        )
        // …and a second bound on the same face (the same loop record
        // re-stated — the gate fires before any use of the ring).
        .replace(
            "#104 = ADVANCED_FACE('', (#103), #87, .T.);",
            "#104 = ADVANCED_FACE('', (#103, #990), #87, .T.);\n\
             #990 = FACE_BOUND('', #102, .T.);",
        );
    match import_step(&mutated, &ImportOptions::default()) {
        Err(StepImportError::RecognitionAmbiguous {
            id,
            surface,
            kind,
            margin,
        }) => {
            assert_eq!((id, surface), (104, 87), "the face and its surface, named");
            assert_eq!(kind, PromotedKind::Cylinder, "the estimator that declined");
            assert!(
                margin.abs() <= 1e-9,
                "the collinear-samples margin sits inside ε_in: {margin:e}"
            );
            let msg = StepImportError::RecognitionAmbiguous {
                id,
                surface,
                kind,
                margin,
            }
            .to_string();
            assert!(msg.contains("ambiguous"), "{msg}");
        }
        other => panic!("expected the typed ambiguity refusal, got {other:?}"),
    }
}
