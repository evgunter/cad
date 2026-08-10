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
//!   mixed promoted/stays-NURBS body now imports FIRST-CLASS, its
//!   wall–wall seam carrying a certified plane × NURBS intersection
//!   (M7-8 retired the seam-orphan refusal class; the flip and its
//!   planted falsifier are pinned here);
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

/// The **INTEGRAL** mixed prism (M7-8 union ruling, option (c)): the
/// same promoted-plane/stays-NURBS shape the seam-orphan class is
/// about, built so that every weight is 1.
///
/// A square section lofted (degree 2 in `v`) through three places
/// whose MIDDLE one is offset in `+x`. The `y = ±1` walls keep every
/// vertex at their own `y`, so they stay exactly planar and PROMOTE;
/// the `x = ±1` walls bow into parabolic cylinders — genuinely curved,
/// no analytic kind, so they honestly STAY NURBS. No bulge is used
/// anywhere, so no rational arc enters and the walls are INTEGRAL:
/// the patch-flux quadrature class #207 fixed, hence tier-valid at
/// rest, which is what the arc prism's rational wall cannot be.
fn offset_square_prism() -> topo::Body<f64> {
    let square = || -> sweep::Section {
        let v = |x: f64, y: f64| profile::ProfileVertex {
            pos: Point2::new(x, y),
            bulge: 0.0,
        };
        vec![profile::ProfileLoop::new(vec![
            v(-1.0, -1.0),
            v(1.0, -1.0),
            v(1.0, 1.0),
            v(-1.0, 1.0),
        ])]
    };
    let sections = vec![square(), square(), square()];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.5, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    sweep::loft_body::<f64>(&sections, &places, 2)
        .expect("the offset square prism builds")
        .body
}

/// **The integral twin, and the SECOND gap it measures** (M7-8 union
/// ruling, option (c) — executed, and reporting that the option is not
/// reachable today).
///
/// The intent was to remove the arc prism's confound: same mixed
/// promoted-plane/stays-NURBS shape, same export→import path, same
/// declare-and-check seam, but INTEGRAL walls, so the at-rest gate has
/// a quadrature to run and the body can be first-class. The body half
/// works — it imports and `validate_geometric` returns `Ok(())`,
/// which the rational arc prism can never do.
///
/// The SEAM half does not, and the reason is structural rather than
/// incidental. Our own writer emits every seam carrier bit-identical
/// to the wall's boundary column, so the bitwise `IsoCurve` rung
/// answers first and declare-and-check is never reached — measured
/// here: with our own bytes all four wall seams certify as
/// `IsoCurve`. State the same seam in any foreign form (this restates
/// one control point one ULP up in `z`, the smallest difference a
/// decimal round-trip can make) and the `Intersection` rung does
/// certify it — and then pcurve minting refuses:
/// `IsoUnsupported`, because `nurbs_iso_derive` derives chart images
/// for `IsoCurve` and `PlacedSegment` descriptions ONLY, and an
/// `Intersection` on a described NURBS chart has no arm.
///
/// So the two wall kinds fail in opposite directions, and between them
/// they leave no fixture that is first-class end to end today:
///
/// * **rational** wall (the arc prism): pcurve minting is WAIVED
///   (`pcurves.rs`: "the placeholder and rational walls mint
///   nothing"), so the `Intersection` seam sails through import — and
///   the body is tier-INVALID at rest, because rational patch flux is
///   banked (M7-3 Arm B).
/// * **integral** wall (here): the body is tier-VALID at rest, and
///   pcurve minting is demanded and has no derivation for the
///   `Intersection` description.
///
/// Both are pinned, neither is widened. **Flip when fixed**: this row
/// becomes the first-class end-to-end pin as soon as
/// `nurbs_iso_derive` grows an `Intersection` arm on described NURBS
/// charts — the trimmed-NURBS pcurve lane its own refusal names as
/// "the cut-loft unit's". Nothing in M7-8 needs to change for it.
#[test]
fn the_integral_mixed_body_is_tier_valid_and_pins_the_pcurve_gap() {
    let native = offset_square_prism();
    let text = step_export::step_string(&native, &step_export::StepOptions::default())
        .expect("the offset square prism exports");

    // ---- The half that WORKS: the body is first-class at rest. ----
    let own = import_step(&text, &ImportOptions::default()).expect("our own dialect imports");
    let StepImport::Solid {
        body: own_body,
        normalizations,
        ..
    } = &own
    else {
        panic!("the offset square prism is a solid");
    };
    // Still the MIXED shape the class is about: the two exactly-planar
    // walls promote, the two bowed walls honestly stay NURBS.
    let promoted = promotions(normalizations);
    assert_eq!(
        promoted.iter().map(|&(_, k, _)| k).collect::<Vec<_>>(),
        vec![PromotedKind::Plane; 2],
        "two planar walls promote; the bowed walls stay NURBS: {promoted:?}"
    );
    // THE POINT the arc prism cannot make: tier-valid at rest as
    // `Ok(())`, not as parity with a refusing twin. The integral
    // wall's patch flux is computable, so nothing here is banked.
    topo::validate_geometric(own_body).expect("tier-valid AT REST — the integral wall quadratures");
    topo::validate_geometric(&native).expect("its native twin too: parity here is Ok(())");

    // With OUR bytes the bitwise rung answers, so declare-and-check is
    // never reached — the measurement behind the finding above.
    assert!(
        plane_nurbs_seams(own_body).is_empty(),
        "our own writer's carriers are bit-identical to the walls' columns, \
         so every seam takes the IsoCurve rung"
    );

    // ---- The half that is BLOCKED: a foreign restatement. ----
    let foreign = text.replace(
        "#90 = CARTESIAN_POINT('', (0.0, -1.0, 1.0));",
        "#90 = CARTESIAN_POINT('', (0.0, -1.0, 1.0000000000000002));",
    );
    assert_ne!(text, foreign, "the foreign restatement applied");
    let Err(refusal) = import_step(&foreign, &ImportOptions::default()) else {
        panic!(
            "FLIP: a foreign-stated seam on an INTEGRAL wall now imports — \
             nurbs_iso_derive grew its Intersection arm; retire this pin and \
             assert the first-class body plus its certified seam instead"
        );
    };
    let shown = format!("{refusal:?}");
    if geom_core::Tolerance::get().eps < 1e-9 {
        // The ε-fine posture, the same clause-3 shape the arc prism
        // takes: this seam's own certified between-samples sup is
        // ~6.22e-12 m, so at ε_in = 1e-12 the carrier refuses during
        // ADOPTION carrying that number, and never reaches the pcurve
        // stage. Measured, not widened.
        assert!(
            shown.contains("PlaneNurbsCertificate") && shown.contains("ssi_hull_sup_chart"),
            "the ε-fine refusal is the envelope's own measured bound: {shown}"
        );
    } else {
        assert!(
            shown.contains("IsoUnsupported"),
            "the block is the pcurve lane's missing Intersection arm, not this \
             unit's certification: {shown}"
        );
    }
    println!("M7-8 integral twin: tier-valid at rest; foreign seam refused at {shown}");
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
/// The consequence WAS the seam-orphan class, hit by the UNPERTURBED
/// body: the three exactly-planar walls promote, the arc wall stays
/// NURBS, and a wall–wall seam whose carrier was minted as a promoted
/// PLANE wall's boundary column (bits differing from the arc wall's
/// own column by the arc endpoint's rounding) has no bitwise IsoCurve
/// match. **M7-8 gave that edge its honest path** and this row now
/// pins the FLIP: the file's carrier is adopted as EVIDENCE and
/// certified against both operands (declare-and-check, Evan's #264
/// ruling), so the mixed body imports first-class. What survives from
/// the old pin is the half that did not move: the arc wall STILL
/// stays NURBS under the honest envelope, and the IsoCurve rung still
/// has nothing to offer at this seam — the certificate comes from the
/// new `Intersection` rung, not from a widened bitwise match.
///
/// `seam_certificate` re-derives the certificate's limbs from the
/// imported body rather than reading the stored `max_residual`, so
/// what is reported here is a fresh measurement (the same discipline
/// the at-rest tier-3 pass applies below).
///
/// **The flip is ε-dependent, and that is the honest answer, not a
/// gap.** This seam's certified between-samples sup is ~6.3e-12 m —
/// the two columns agree only to the arc endpoint's rounding and the
/// first-order envelope cannot say better. So the pin flips at ε_in =
/// 1e-9 (default) and 1e-6, and at the 1e-12 matrix row the SAME
/// geometry refuses TYPED, carrying that 6.3e-12 in the payload. The
/// spec's clause 3 is explicit that this is the outcome to ship: a
/// bound too loose at ε refuses with its number, never through a
/// widened gate. Both postures are pinned below.
#[test]
fn the_seam_orphan_certifies_and_is_pinned_at_the_rational_quadrature_gate() {
    let native = straight_arc_prism();
    let text = step_export::step_string(&native, &step_export::StepOptions::default())
        .expect("the arc prism exports");
    let eps = geom_core::Tolerance::get().eps;

    match import_step(&text, &ImportOptions::default()) {
        // FLIP WHEN FIXED. The moment rational patch flux lands, this
        // body becomes first-class and this row must be retired for the
        // full assertion: the certified seam PLUS `Ok(())` at rest.
        Ok(_) => panic!(
            "FLIP: the arc prism now imports first-class — rational patch flux \
             quadrature has landed. Retire this waypoint and assert the certified \
             plane x NURBS seam on a first-class body."
        ),
        // The ε-fine posture, UNCHANGED by the gate: at 1e-12 the
        // envelope's own slack refuses during adoption, so the body
        // never reaches the at-rest pass at all.
        Err(StepImportError::Adoption { id, attempts }) => {
            assert!(
                eps < 1e-9,
                "adoption itself only refuses at the ε-fine row: {attempts:?}"
            );
            assert_eq!(id, 130, "the seam, named");
            let bound = attempts.iter().find_map(|a| match a.refusal {
                topo::EulerOpError::Certification {
                    error:
                        geom_brep::CertifyError::Escalated {
                            check: geom_brep::CertCheck::PlaneNurbsCertificate,
                            cause,
                            ..
                        },
                } => match cause.margin {
                    geom_core::MarginDiag::Value(v) => Some(v),
                    _ => None,
                },
                _ => None,
            });
            let Some(bound) = bound else {
                panic!("the refusal must carry the lane's measured bound: {attempts:?}");
            };
            assert!(
                bound > eps,
                "the refusal's own number explains it: the certified sup {bound:e} m \
                 does not fit inside ε_in {eps:e}"
            );
            println!("M7-8 seam #130 @ eps={eps:e}: adoption refuses, certified sup {bound:e} m");
        }
        // THE WAYPOINT. At default and 1e-6 the seam CERTIFIES — the
        // adoption ladder no longer refuses this edge, which is the
        // whole of what M7-8 owns and retires. The body is then
        // refused by the shared at-rest gate, on a fact about the
        // wall's VOLUME rather than the seam's certification.
        Err(refusal @ StepImportError::TierInvalid { .. }) => {
            assert!(
                eps >= 1e-9,
                "the at-rest gate is only reached where adoption succeeds: {refusal:?}"
            );
            let shown = format!("{refusal:?}");
            // THE REFUSAL ADVANCED. It used to say the seam had no
            // certification path; it now says the rational wall's
            // volume is not computable. Both halves are asserted so
            // neither can regress silently.
            assert!(
                shown.contains("QuadratureUnsupported") && shown.contains("RATIONAL patch flux"),
                "the surviving refusal is the BANKED rational patch flux (M7-3 Arm B), \
                 not anything about the seam: {shown}"
            );
            assert!(
                !shown.contains("Adoption") && !shown.contains("PlaneNurbs"),
                "the seam-orphan class is RETIRED: no adoption refusal survives here: {shown}"
            );
            println!("M7-8 seam #130 @ eps={eps:e}: seam certifies; body pinned at the gate");
        }
        other => panic!("no other posture is pinned for this fixture: {other:?}"),
    }
}

/// **The planted falsifier — declare-and-check, executed at the
/// importer.** The file's carrier is EVIDENCE, never truth: a seam
/// carrier doctored off the true intersection must refuse, and the
/// refusal must carry the number that caught it.
///
/// The displacement is the seam spline's middle control point pushed
/// `+x` by 1e-3 m — off the cylinder wall while staying exactly on
/// the `y = 1` cap plane, so it is the NURBS-side residual (the
/// certified foot distance and its between-samples sup) that has to
/// do the catching, not the closed-form plane distance. A degree-2
/// Bézier moves half its middle control point's displacement at
/// mid-parameter, so the honest measured bound is ~5e-4 m: past ε_in
/// at every matrix row by six orders of magnitude and more.
#[test]
fn a_displaced_seam_carrier_refuses_with_the_measured_residual() {
    let native = straight_arc_prism();
    let text = step_export::step_string(&native, &step_export::StepOptions::default())
        .expect("the arc prism exports");
    // #127 is the middle control point of #129, the B-spline carrier
    // of EDGE_CURVE #130 — the seam this unit certifies.
    let doctored = text.replace(
        "#127 = CARTESIAN_POINT('', (1.0, 1.0, 1.0));",
        "#127 = CARTESIAN_POINT('', (1.001, 1.0, 1.0));",
    );
    assert_ne!(text, doctored, "the falsifier applied");

    let Err(StepImportError::Adoption { id, attempts }) =
        import_step(&doctored, &ImportOptions::default())
    else {
        panic!("a carrier displaced 1e-3 m off the locus is NEVER trusted");
    };
    assert_eq!(id, 130, "the doctored seam, named");
    let msg = format!("{:?}", attempts);
    let measured = attempts.iter().find_map(|a| match a.refusal {
        topo::EulerOpError::Certification {
            error:
                geom_brep::CertifyError::PlaneNurbs(geom_brep::PlaneNurbsRefusal::Limb {
                    value, ..
                }),
        } => Some(value),
        _ => None,
    });
    let Some(measured) = measured else {
        panic!("the refusal must carry the plane × NURBS lane's measured bound: {msg}");
    };
    assert!(
        (1e-4..1e-2).contains(&measured),
        "the measured bound is the displacement the falsifier planted \
         (~5e-4 m at mid-parameter): {measured:e}"
    );
    let text = StepImportError::Adoption { id, attempts }.to_string();
    assert!(
        text.contains("not on both surfaces"),
        "the refusal text states the declare-and-check verdict: {text}"
    );
}

/// Every `Intersection` edge in `body` whose operands are one PLANE
/// and one described NURBS wall, as `(curve, plane, wall)` — the
/// M7-8 class, located by DESCRIPTION rather than by key so the pin
/// survives arena renumbering.
fn plane_nurbs_seams(
    body: &topo::Body<f64>,
) -> Vec<(
    geom_brep::keys::CurveKey,
    geom_surfaces::Surface<f64>,
    geom_surfaces::NurbsSurface<f64>,
)> {
    let surfaces: std::collections::BTreeMap<_, _> = body.surfaces().collect();
    let described = |k| match surfaces.get(&k) {
        Some(geom_surfaces::Surface::Nurbs(n)) if !n.is_placeholder() => Some((**n).clone()),
        _ => None,
    };
    let plane = |k| match surfaces.get(&k) {
        Some(p @ geom_surfaces::Surface::Plane { .. }) => Some((*p).clone()),
        _ => None,
    };
    body.curves()
        .filter_map(|(key, geom)| {
            let topo::CurveGeom::Certified(curve) = geom else {
                return None;
            };
            let (&s1, &s2) = match curve.description() {
                geom_brep::EdgeGeometry::Intersection { s1, s2, .. } => (s1, s2),
                _ => return None,
            };
            let pair = plane(s1)
                .zip(described(s2))
                .or_else(|| plane(s2).zip(described(s1)))?;
            Some((key, pair.0, pair.1))
        })
        .collect()
}

// `seam_certificate` — which re-derived this seam's limbs from the
// IMPORTED body — is retired with the first-class row it served: the
// arc prism no longer produces an imported body to re-derive from
// (the at-rest gate refuses it), and the integral twin's seam never
// reaches the rung (the pcurve lane blocks it). The seam's own
// certified numbers stay pinned in two places that still execute: the
// ε-fine branch above, which asserts this seam's certified sup
// (6.3156e-12 m) from the refusal payload, and geom-brep's
// `m7_8_plane_nurbs_edge` rows, which measure the same
// quarter-cylinder-meets-plane geometry at the lane and at the door.
