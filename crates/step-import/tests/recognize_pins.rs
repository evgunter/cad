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
fn the_seam_orphan_certifies_as_a_plane_nurbs_intersection() {
    let native = straight_arc_prism();
    let text = step_export::step_string(&native, &step_export::StepOptions::default())
        .expect("the arc prism exports");
    let eps = geom_core::Tolerance::get().eps;

    // TWO honest postures, and which one this row takes is decided by
    // ε_in against the envelope's own slack — never by a widened gate
    // (the spec's clause 3). This seam's certified between-samples sup
    // is ~6.3e-12 m: the promoted plane wall's boundary column and the
    // arc wall's own column agree to the arc endpoint's rounding, and
    // the first-order envelope over the schedule cannot say better
    // than that. So at ε_in = 1e-9 (default) and 1e-6 the pin FLIPS,
    // and at ε_in = 1e-12 the SAME geometry refuses typed WITH the
    // measured bound in the payload. Both are pinned here; a change to
    // either is a posture change to re-pin.
    let import = match import_step(&text, &ImportOptions::default()) {
        Ok(import) => import,
        Err(refusal) => {
            assert!(
                eps < 1e-9,
                "the flip is REQUIRED at ε_in ≥ 1e-9 (the default row and coarser): \
                 {refusal:?}"
            );
            let StepImportError::Adoption { id, attempts } = &refusal else {
                panic!("the too-fine row refuses through the adoption ladder: {refusal:?}");
            };
            assert_eq!(*id, 130, "the seam, named");
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
            println!("M7-8 seam #130 @ eps={eps:e}: REFUSES typed, certified sup {bound:e} m");
            return;
        }
    };
    let StepImport::Solid {
        body,
        normalizations,
        ..
    } = &import
    else {
        panic!("the arc prism is a solid");
    };

    // The wall did NOT promote: this is still the mixed
    // promoted-plane/stays-NURBS body the class is about.
    assert_eq!(
        promotions(normalizations)
            .iter()
            .map(|&(_, kind, _)| kind)
            .collect::<Vec<_>>(),
        vec![PromotedKind::Plane; 3],
        "three planar walls promote; the exact-cylinder arc wall honestly stays NURBS"
    );

    // Exactly one plane × NURBS seam, and it is edge #130's — the
    // orphan. The others are plane × plane, the wall's own iso rung,
    // or the cap rims.
    let seams = plane_nurbs_seams(body);
    assert_eq!(
        seams.len(),
        1,
        "one certified plane × NURBS seam in this body: {} found",
        seams.len()
    );
    let (curve_key, ref plane, ref wall) = seams[0];

    // The certificate, RE-DERIVED (never the stored number).
    let limbs = seam_certificate(body, curve_key, plane, wall);
    assert!(
        limbs.on_locus_max <= eps,
        "limb 1, the sampled on-locus residual over both operands: \
         {:e} m must sit inside ε_in {eps:e}",
        limbs.on_locus_max
    );
    assert!(
        limbs.hull_sup <= eps,
        "limb 2, the certified between-samples sup-norm — the number that \
         certifies: {:e} m must sit inside ε_in {eps:e}",
        limbs.hull_sup
    );
    assert!(
        limbs.min_sin_theta > 0.7,
        "the transversality margin: the y = 1 cap plane meets the quarter-cylinder \
         wall at 45°, so sin θ ≈ √2/2; measured {:e}",
        limbs.min_sin_theta
    );
    assert!(
        limbs.tube_transversality > 0.0 && limbs.tube_boxes > 0,
        "the uniqueness tube is certified over a non-empty box chain: {limbs:?}"
    );
    // Reported, not asserted — the numbers the ruling asked for, at
    // whichever ε row this binary runs (default / 1e-6 / 1e-12).
    println!(
        "M7-8 seam #130 @ eps={eps:e}: on_locus={:e} hull_sup={:e} \
         tube_radius={:e} tube_transversality={:e} boxes={} sin_theta={:e}",
        limbs.on_locus_max,
        limbs.hull_sup,
        limbs.tube_radius,
        limbs.tube_transversality,
        limbs.tube_boxes,
        limbs.min_sin_theta
    );

    // Tier-valid AT REST, in the ONLY sense the importer claims
    // (`StepImport::Solid`'s contract): to the tier its native twin
    // certifies to — nothing imports into a state its native twin
    // does not occupy. The arc prism's wall is RATIONAL, so both the
    // native and the imported body refuse tier 3 with the same banked
    // quadrature refusal (M7-3); what M7-8 has to prove is that the
    // at-rest pass finds NO certification failure — every edge,
    // including the new plane × NURBS seam, RE-DERIVES its
    // certificate under `recertify_nurbs_lane` rather than carrying
    // a stored one.
    //
    // The comparison drops the face KEY on both sides: the two bodies
    // are built by different routes (native Euler ops vs adoption), so
    // their arena numbering differs by construction and only the
    // refusal's substance is comparable.
    let refusals = |errors: &[topo::ValidationError]| -> Vec<String> {
        errors
            .iter()
            .map(|e| match e {
                topo::ValidationError::VolumeUncomputable { source } => {
                    format!("VolumeUncomputable/{source:?}")
                        .split_once("face:")
                        .map_or_else(
                            || format!("{e:?}"),
                            |(head, tail)| {
                                format!("{head}{}", tail.split_once(',').map_or(tail, |(_, t)| t))
                            },
                        )
                }
                other => format!("{other:?}"),
            })
            .collect()
    };
    let at_rest = topo::validate_geometric(body).expect_err("the banked rational wall, verbatim");
    let native_at_rest =
        topo::validate_geometric(&native).expect_err("the native twin refuses identically");
    assert_eq!(
        refusals(&at_rest),
        refusals(&native_at_rest),
        "the imported body's at-rest verdict IS its native twin's"
    );
    assert!(
        at_rest
            .iter()
            .all(|e| matches!(e, topo::ValidationError::VolumeUncomputable { .. })),
        "the only at-rest refusal is the banked rational-wall quadrature — no edge \
         failed re-certification: {at_rest:?}"
    );

    // D9: the same bytes import to the same body, twice.
    let again = import_step(&text, &ImportOptions::default()).expect("the second import");
    let StepImport::Solid { body: again, .. } = &again else {
        panic!("the arc prism is a solid");
    };
    assert_eq!(
        step_export::step_string(again, &step_export::StepOptions::default()).unwrap(),
        step_export::step_string(body, &step_export::StepOptions::default()).unwrap(),
        "D9: import twice, byte-compare"
    );

    // The near-miss: the wall's centre control point, ~5·ε_in off the
    // cylinder. The wall is no longer an exact cylinder, so the
    // seam's declared carrier is no longer on it — and the
    // declare-and-check lane says so with the number, rather than
    // certifying a carrier the file merely asserted.
    //
    // The exported file's declared uncertainty is the ambient ε the
    // export ran at, so the nudge scales with the run's ε (past the
    // budget at every matrix row). The perturbation base is the
    // FILE'S OWN token, parsed — the writer's print of the point,
    // which the bulge arithmetic left one ulp BELOW f64 √2 (not
    // `f64::consts::SQRT_2`, whose substitution would misstate the
    // intent: "the file's value, moved", not "√2, moved").
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
    let near = import_step(&near_miss, &ImportOptions::default())
        .expect("the near-miss prism still imports first-class");
    let StepImport::Solid {
        body: near,
        normalizations: near_norms,
        ..
    } = &near
    else {
        panic!("the arc prism is a solid");
    };
    assert!(
        promotions(near_norms)
            .iter()
            .all(|&(_, kind, _)| kind == PromotedKind::Plane),
        "the near-miss wall is still silent about itself: no arc-wall promotion"
    );
    // The nudge is the wall's mid-arc control point; the seam sits at
    // the patch's `u = 1` edge, where that point's basis weight
    // vanishes. So the seam's own certificate is unmoved and the
    // near-miss row pins the SEPARATION: perturbing the wall's
    // interior does not leak into the seam's declare-and-check
    // verdict. The carrier-side falsifier — the perturbation that
    // MUST be caught — is
    // `a_displaced_seam_carrier_refuses_with_the_measured_residual`.
    let near_seams = plane_nurbs_seams(near);
    assert_eq!(near_seams.len(), 1, "the same one seam");
    let near_limbs = seam_certificate(near, near_seams[0].0, &near_seams[0].1, &near_seams[0].2);
    assert!(
        near_limbs.hull_sup <= eps,
        "the seam's certified sup is unmoved by the interior nudge: {:e} m",
        near_limbs.hull_sup
    );
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

/// Re-derives a seam's plane × NURBS certificate from the body. The
/// stored `Certificate` keeps only the sample count and the max
/// residual, and this unit's whole contract is that the numbers are
/// re-MEASURED rather than trusted — so the reported limbs come from
/// a fresh run of the lane, exactly as the at-rest tier-3 pass does.
fn seam_certificate(
    body: &topo::Body<f64>,
    curve: geom_brep::keys::CurveKey,
    plane: &geom_surfaces::Surface<f64>,
    wall: &geom_surfaces::NurbsSurface<f64>,
) -> geom_brep::PlaneNurbsLimbs<f64> {
    let Some(topo::CurveGeom::Certified(edge)) =
        body.curves().find(|&(k, _)| k == curve).map(|(_, g)| g)
    else {
        panic!("a certified seam");
    };
    let geom_curves::Curve3::Nurbs(carrier) = edge.carrier() else {
        panic!("the seam's carrier is a spline");
    };
    let (t0, t1) = edge.params();
    let extent = edge.carrier().eval(t0).distance(edge.carrier().eval(t1));
    <f64 as geom_brep::EdgeNurbsLane>::plane_nurbs_limbs(
        carrier,
        plane,
        wall,
        extent,
        geom_core::Band::linear().expect("the run's linear band"),
    )
    .expect("the seam re-certifies at rest")
}
