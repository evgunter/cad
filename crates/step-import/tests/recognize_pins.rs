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
//! * the cylinder track under the honest between-samples envelope
//!   (R1 M-1): even an EXACT cylinder patch stays NURBS — the
//!   first-order envelope's slack is patch-scale — and the resulting
//!   mixed promoted/stays-NURBS body pins the seam-orphan refusal
//!   class loudly (a standing ruling item);
//! * the ill-conditioned-estimator typed row: D7's
//!   `RecognitionAmbiguous` fires exactly where promotion was the
//!   face's only door AND the estimator cannot answer at ε_in;
//! * promoted PLANES compute exact volume (executed corpus-wide by
//!   `roundtrip.rs`'s rows, which run these fixtures' volumes against
//!   the native `KERNEL_*` sidecars).
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

/// **The cylinder track under the honest envelope, and the amplified
/// seam-orphan limit** (R1 fix pass, M-1 consequence — reported
/// loudly; a standing item for the next ruling).
///
/// The exact-cylinder rational wall's grid residual is the
/// extrusion's own rounding (~1e-16), but the certificate is now the
/// grid PLUS the first-order between-samples envelope, whose slack is
/// patch-scale over the sample count (~1e-1 m here) — orders of
/// magnitude past any real ε_in. So the wall honestly STAYS NURBS:
/// under the derivative-hull envelope NO cylinder patch certifies at
/// fine ε, exact geometry included (the algebraic spline-product hull
/// certificate that would restore the cylinder track is banked).
///
/// The consequence is the seam-orphan class, now hit by the
/// UNPERTURBED body: the three exactly-planar walls still promote,
/// the arc wall stays NURBS, and a wall–wall seam whose carrier was
/// minted as a promoted PLANE wall's boundary column (bits differing
/// from the arc wall's own column by the arc endpoint's rounding) has
/// no bitwise IsoCurve match and no certifiable plane × NURBS
/// intersection — the import refuses TYPED where pre-stage-1 main
/// imports it. Pinned as it stands so the posture cannot drift
/// silently; the near-miss variant (the wall nudged 5·ε_in off the
/// cylinder) lands in exactly the same refusal, which also pins the
/// silent-stays-NURBS half: no promotion is recorded for the arc wall
/// in either variant.
#[test]
fn cylinder_envelope_refuses_and_the_seam_orphan_is_pinned() {
    let native = straight_arc_prism();
    let text = step_export::step_string(&native, &step_export::StepOptions::default())
        .expect("the arc prism exports");
    let orphaned_seam = |text: &str, who: &str| match import_step(text, &ImportOptions::default()) {
        Err(StepImportError::Adoption { id, attempts }) => {
            assert_eq!(id, 130, "{who}: the orphaned seam beside a promoted wall");
            assert!(
                attempts
                    .iter()
                    .all(|a| !matches!(a.candidate, step_import::AdoptionCandidate::IsoCurve)),
                "{who}: the bitwise IsoCurve rung had nothing to offer: {attempts:?}"
            );
        }
        other => panic!(
            "{who}: the measured state is the typed seam-adoption refusal; a change \
             here is a posture change to re-pin: {other:?}"
        ),
    };
    orphaned_seam(&text, "unperturbed exact-cylinder prism");

    // The near-miss: the wall's centre control point, ~5·ε_in off the
    // cylinder — the same refusal, same silence about the wall itself.
    //
    // The exported file's declared uncertainty is the ambient ε the
    // export ran at, so the nudge scales with the run's ε (past the
    // budget at every matrix row). The perturbation base is the
    // FILE'S OWN token, parsed — the writer's print of the point,
    // which the bulge arithmetic left one ulp BELOW f64 √2 (not
    // `f64::consts::SQRT_2`, whose substitution would misstate the
    // intent: "the file's value, moved", not "√2, moved").
    let eps = geom_core::Tolerance::get().eps;
    const CENTER_X_TOKEN: &str = "1.414213562373095";
    let base: f64 = CENTER_X_TOKEN
        .parse()
        .expect("the writer's own token parses");
    let near_miss = text.replace(
        &format!("#114 = CARTESIAN_POINT('', ({CENTER_X_TOKEN}, 0.0, 1.0));"),
        &format!(
            "#114 = CARTESIAN_POINT('', ({:?}, 0.0, 1.0));",
            base + 5.0 * eps
        ),
    );
    assert_ne!(text, near_miss, "the perturbation applied");
    orphaned_seam(&near_miss, "near-miss prism");
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
