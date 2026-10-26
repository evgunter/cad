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

use crate::common;
use geom_core::Point3;
use profile::SketchPlane;

use common::{SOLID_FIXTURES, fixture};
use geom_core::Point2;
use geom_core::Tol;
use geom_core::test_support::upper;
use profile::test_support::bulge_loop;
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
    match import_step(text, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid {
            body,
            normalizations,
            ..
        }) => {
            let p = promotions(&normalizations);
            (body.into_body(), p)
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
        common::arena_census(&base),
        common::arena_census(&body),
        "census identical across the vocabulary"
    );
    assert_eq!(
        base_promos, promos,
        "identical promotions (bit-identical patches)"
    );
    let (v1, v2) = (
        topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished("the measured body", base.clone(), Tol::witness()),
            Tol::witness(),
        )
        .volume,
        topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished("the measured body", body.clone(), Tol::witness()),
            Tol::witness(),
        )
        .volume,
    );
    assert_eq!(v1.to_bits(), v2.to_bits(), "volume bit-identical");
}

/// The straight arc-prism: three identical bulged-square sections
/// stacked along +z — its arc wall is an EXACT cylinder patch (a
/// translational extrusion of a rational quarter-circle), the
/// cylinder track's own-corpus exercise.
fn straight_arc_prism() -> topo::Body<f64> {
    let arc_section = || -> sweep::Section {
        let v = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
        vec![bulge_loop(vec![
            v(-1.0, -1.0, 0.0),
            // tan(π/8): a quarter-circle bulge on the +x side.
            v(1.0, -1.0, 0.4142135623730951),
            v(1.0, 1.0, 0.0),
            v(-1.0, 1.0, 0.0),
        ])]
    };
    let sections = vec![arc_section(), arc_section(), arc_section()];
    let places = vec![
        SketchPlane::xy(),
        SketchPlane::xy_at(Point3::new(0.0, 0.0, 1.0)),
        SketchPlane::xy_at(Point3::new(0.0, 0.0, 2.0)),
    ];
    sweep::loft_body::<f64>(&sections, &places, 2, Tol::witness())
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
        let v = |x: f64, y: f64| (Point2::new(x, y), 0.0);
        vec![bulge_loop(vec![
            v(-1.0, -1.0),
            v(1.0, -1.0),
            v(1.0, 1.0),
            v(-1.0, 1.0),
        ])]
    };
    let sections = vec![square(), square(), square()];
    let places = vec![
        SketchPlane::xy(),
        SketchPlane::xy_at(Point3::new(0.5, 0.0, 1.0)),
        SketchPlane::xy_at(Point3::new(0.0, 0.0, 2.0)),
    ];
    sweep::loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("the offset square prism builds")
        .body
}

/// **The integral twin, first-class end to end** (M7-8 union ruling,
/// option (c)): the mixed promoted-plane/stays-NURBS shape whose walls
/// are INTEGRAL, so the at-rest gate has a quadrature it can run and
/// the body can be first-class where the rational arc prism cannot.
///
/// Two halves, both executed here:
///
/// * **at rest** — the two exactly-planar walls promote, the two bowed
///   walls honestly stay NURBS, and `validate_geometric` answers
///   `Ok(())` on the imported body and on its native twin;
/// * **the seam** — our own writer emits every seam carrier
///   bit-identical to the wall's boundary column, so the bitwise
///   `IsoCurve` rung answers first and declare-and-check is never
///   reached for a seam (measured here: with our own bytes NO seam
///   takes the `Intersection` rung; the bowed walls' four straight cap
///   rims do, the plane × NURBS certificate taking their line). State the same seam in a foreign form —
///   this restates one control point one ULP up in `z`, the smallest
///   difference a decimal round-trip can make — and the seam certifies
///   through the declare-and-check plane × NURBS rung and CHARTS on
///   both of its faces: the plane side through the affine chart, the
///   NURBS side through the boundary-column iso image.
///
/// **The ε posture is the honest one, not a gap.** This seam's
/// certified between-samples sup is ~3.55e-14 m, so the first-class
/// import holds at ε_in = 1e-9 (default), 1e-6 and 1e-12, and below
/// ε_in = 1e-13 the SAME geometry refuses TYPED during ADOPTION,
/// carrying that number in its payload. Both cells are pinned; neither
/// is widened.
///
/// **Why the boundary sits at 1e-13**: the sup is assembled over a
/// Bézier decomposition whose every row is restricted from its own span
/// window (`CoeffWindow::restrict`), so a coefficient's width is added
/// once per step and never multiplied per insertion (a step that
/// multiplies it reads ~6.2e-12 m here, and the boundary at 1e-9). So
/// the 1e-12 matrix row takes the first-class cell; the refusal cell is
/// measured at ε_in = 1e-14 and no row the gate or the nightly runs
/// reaches it, which is filed rather than absorbed.
#[test]
fn the_integral_mixed_body_imports_first_class_with_a_charted_seam() {
    let native = offset_square_prism();
    let text = step_export::step_string(
        &crate::common::at_rest(&native),
        &step_export::StepOptions::default(),
        Tol::witness(),
    )
    .expect("the offset square prism exports");

    // ---- The half that WORKS: the body is first-class at rest. ----
    let own = import_step(&text, &ImportOptions::default(), Tol::witness())
        .expect("our own dialect imports");
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
    topo::validate_geometric(own_body.body(), Tol::witness())
        .expect("tier-valid AT REST — the integral wall quadratures");
    topo::validate_geometric(&native, Tol::witness())
        .expect("its native twin too: parity here is Ok(())");

    // With OUR bytes the bitwise rung answers, so declare-and-check is
    // never reached — the measurement behind the finding above.
    assert!(
        plane_nurbs_seams(own_body.body()).is_empty(),
        "our own writer's carriers are bit-identical to the walls' columns, \
         so every seam takes the IsoCurve rung"
    );
    // The cap rims of the two bowed walls are straight, and the plane ×
    // NURBS certificate takes their line carrier: they adopt at the
    // `Intersection` rung, which the ladder offers before the
    // conventional one.
    let rims = plane_nurbs_intersections(own_body.body());
    assert!(
        rims.len() == 4 && rims.iter().all(|(_, plane, _)| is_cap(plane)),
        "the plane × NURBS Intersections with our own bytes are the bowed walls' four \
         cap rims: {rims:?}"
    );

    // ---- The half the Intersection arm opened: a foreign
    // restatement, first-class and charted. ----
    let eps = geom_core::Tol::witness().get().eps;
    let foreign = text.replace(
        "#90 = CARTESIAN_POINT('', (0.0, -1.0, 1.0));",
        "#90 = CARTESIAN_POINT('', (0.0, -1.0, 1.0000000000000002));",
    );
    assert_ne!(text, foreign, "the foreign restatement applied");
    match import_step(&foreign, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { body, .. }) => {
            assert!(
                eps >= 1e-13,
                "the seam's certified sup does not fit inside an ε_in finer than its own \
                 rounding — a first-class import there would be a widened gate"
            );
            topo::validate_geometric(body.body(), Tol::witness())
                .expect("the foreign restatement is tier-valid at rest, exactly like our own");
            let seams = plane_nurbs_seams(body.body());
            assert_eq!(
                seams.len(),
                1,
                "exactly the restated seam takes the declare-and-check rung: {seams:?}"
            );
            assert_eq!(
                plane_nurbs_intersections(body.body()).len(),
                5,
                "beside the restated seam, the same four cap rims as our own bytes"
            );
            let (curve, _, wall) = &seams[0];
            // The seam CHARTS. The NURBS wall stores its side (the
            // boundary-column iso image); the plane keeps M2's
            // derive-on-demand status, so it stores nothing — one
            // stored cache for the edge is the whole statement.
            let stored: Vec<topo::Pcurve<f64>> = body
                .body()
                .edges()
                .filter(|(_, e)| e.curve == *curve)
                .flat_map(|(_, e)| [e.he_plus, e.he_minus])
                .filter_map(|he| body.body().pcurve(he).map(|c| c.pcurve().clone()))
                .collect();
            assert_eq!(
                stored.len(),
                1,
                "the NURBS wall charts the seam; the plane derives on demand: {stored:?}"
            );
            let topo::Pcurve::IsoLine { p0, pl } = stored[0] else {
                panic!("a seam's chart image is the wall's boundary iso line: {stored:?}");
            };
            assert_eq!(pl.x, 0.0, "a seam image holds u constant: {pl:?}");
            let (du0, du1) = wall.knots_u_f64_interim().domain();
            assert!(
                p0.x == du0 || p0.x == du1,
                "on the chart's OWN boundary column, not a [0, 1] literal: \
                 u = {} of [{du0}, {du1}]",
                p0.x
            );
            println!(
                "M8-4 integral twin @ eps={eps:e}: first-class; seam charted at u = {}",
                p0.x
            );
        }
        // The ε-fine cell, pinned as a REFUSAL with its own number:
        // this seam's certified between-samples sup is ~3.55e-14 m, so
        // below ε_in = 1e-13 the carrier refuses during ADOPTION and
        // never reaches the pcurve stage. Measured, not widened.
        Err(refusal) => {
            assert!(
                eps < 1e-13,
                "the only refusing cell is the ε-fine one: {refusal:?}"
            );
            let shown = format!("{refusal:?}");
            assert!(
                shown.contains("PlaneNurbsHull") && shown.contains("ssi_hull_sup_chart"),
                "the ε-fine refusal is the envelope's own measured bound: {shown}"
            );
            println!("M8-4 integral twin @ eps={eps:e}: adoption refuses — {shown}");
        }
        other => panic!("no other posture is pinned for this fixture: {other:?}"),
    }
}

/// **The RATIONAL mixed body, first-class end to end** — the cylinder
/// track under the honest envelope, and the seam it produces.
///
/// The exact-cylinder rational wall's grid residual is the
/// extrusion's own rounding (~1e-16), but the certificate is the
/// grid PLUS the first-order between-samples envelope, whose slack is
/// patch-scale over the sample count (~1e-1 m here) — orders of
/// magnitude past any real ε_in. So the wall honestly STAYS NURBS:
/// under the derivative-hull envelope NO cylinder patch certifies at
/// fine ε, exact geometry included (the algebraic spline-product hull
/// certificate that would restore the cylinder track is banked).
///
/// That is what puts the UNPERTURBED body on the intrinsic rung: the
/// three exactly-planar walls promote and the arc wall stays NURBS.
/// Both arc-wall seams carry the wall's boundary columns in their
/// bits: the skin's `segment_curve` places the first column at the
/// vertex the section stores and the last at `eval(1)`, the start
/// turned through the stored sweep, never at a point re-derived from a
/// radius. `eval(1)` is the stored end vertex bit for bit in THIS
/// fixture, not on every chord, so here each seam's carrier IS the
/// wall's boundary column and the bitwise `IsoCurve` rung answers. Declare-and-check is not reached on this body; the planted
/// falsifier below forces it by moving the file's carrier off the
/// column.
///
/// The RATIONAL wall completes the picture the integral twin starts:
/// its arc rims chart through the rational-quadratic iso image and its
/// seams through the boundary-column one, so the whole mixed body
/// imports first-class — rational patch flux and all.
///
/// **First-class at every ε row, ε-fine included.** No envelope sits
/// between the seam and its chart, so nothing here has a bound for ε
/// to undercut. Two postures remain and both are pinned: at a coarse
/// ε_in the straight corner carriers also promote to LINE (#388), and
/// the fixed schedule's quadrature-budget frontier.
#[test]
fn the_mixed_arc_prism_imports_first_class_over_the_intersection_pcurve_arm() {
    let native = straight_arc_prism();
    let text = step_export::step_string(
        &crate::common::at_rest(&native),
        &step_export::StepOptions::default(),
        Tol::witness(),
    )
    .expect("the arc prism exports");
    let eps = geom_core::Tol::witness().get().eps;

    match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        // **First-class, end to end, at every ε.** The three
        // exactly-planar walls promote, the arc wall stays NURBS under
        // the honest envelope, every face charts, and both arc-wall
        // seams take the bitwise `IsoCurve` rung: no seam is left for
        // declare-and-check. At a coarser ε_in the straight corner
        // carriers additionally promote to LINE (#388 — their Greville
        // map fold is ~1.3e-7 at unit scale, between the two bands);
        // the seams chart the same way either side of it.
        Ok(StepImport::Solid {
            body,
            curve_promotions,
            ..
        }) => {
            topo::validate_geometric(body.body(), Tol::witness())
                .expect("first-class at rest: the rational wall's flux reaches its target");
            let taken = plane_nurbs_intersections(body.body());
            assert!(
                taken.is_empty(),
                "both seams are the arc wall's own columns, bit for bit; the \
                 declare-and-check rung has no takers, seam or cap rim: {taken:?}"
            );
            let line_promotions: Vec<_> = curve_promotions
                .iter()
                .filter(|p| p.kind == step_import::PromotedCurveKind::Line)
                .collect();
            if !line_promotions.is_empty() {
                assert_eq!(
                    line_promotions.len(),
                    4,
                    "the four straight corner carriers promote: {line_promotions:?}"
                );
                for p in &line_promotions {
                    assert!(
                        p.residual <= eps && p.residual > 1e-9,
                        "the corner promotion is the coarse band's own fold: {:e}",
                        p.residual
                    );
                }
            }
            let wall = body
                .body()
                .surfaces()
                .find_map(|(_, s)| match s {
                    geom::Surface::Nurbs(n) if !n.is_placeholder() => Some((**n).clone()),
                    _ => None,
                })
                .expect("the arc wall stays NURBS at every band");
            let (du0, du1) = wall.knots_u_f64_interim().domain();
            let images: Vec<_> = body
                .body()
                .edges()
                .flat_map(|(_, e)| [e.he_plus, e.he_minus])
                .filter_map(|he| body.body().pcurve(he).map(|c| c.pcurve().clone()))
                .filter_map(|p| match p {
                    topo::Pcurve::IsoLine { p0, pl } if pl.x == 0.0 => Some(p0),
                    _ => None,
                })
                .collect();
            assert_eq!(
                images.len(),
                2,
                "both seams chart as u-constant iso lines: {images:?}"
            );
            for p0 in &images {
                assert!(
                    p0.x == du0 || p0.x == du1,
                    "on the chart's OWN boundary columns: u = {} of [{du0}, {du1}]",
                    p0.x
                );
            }
            println!(
                "M8-4 seams @ eps={eps:e}: {} LINE corners; both seams charted on their \
                 boundary columns",
                line_promotions.len()
            );
        }
        // The honest frontier: where the seam certifies but the
        // rational wall's sign is still open at its work wall, check 7
        // refuses TYPED. That is a DIFFERENT refusal from the retired
        // bank — the lane ran, and said how far it got.
        Err(refusal @ StepImportError::TierInvalid { .. }) => {
            let shown = format!("{refusal:?}");
            assert!(
                shown.contains("VolumeSignUnresolved"),
                "the only surviving at-rest refusal is a sign open at the wall: {shown}"
            );
            assert!(
                !shown.contains("RATIONAL patch flux"),
                "the RATIONAL patch flux bank is RETIRED — no refusal may name it: {shown}"
            );
            assert!(
                !shown.contains("Adoption") && !shown.contains("PlaneNurbs"),
                "the seam-orphan class is RETIRED: no adoption refusal survives here: {shown}"
            );
            println!("M7-8 arc prism @ eps={eps:e}: seams charted; volume at the budget");
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
    let text = step_export::step_string(
        &crate::common::at_rest(&native),
        &step_export::StepOptions::default(),
        Tol::witness(),
    )
    .expect("the arc prism exports");
    // #121 is the middle control point of #123, the B-spline carrier
    // of EDGE_CURVE #124 — the seam this unit certifies. (The ids are
    // emission order: they moved from #127 / #129 / #130 when C6's
    // sub-arc η made the prism's quarter arc one span, three control
    // points a row where there were five.)
    let doctored = text.replace(
        "#121 = CARTESIAN_POINT('', (1.0, 1.0, 1.0));",
        "#121 = CARTESIAN_POINT('', (1.001, 1.0, 1.0));",
    );
    assert_ne!(text, doctored, "the falsifier applied");

    let Err(StepImportError::Adoption { id, attempts, file }) =
        import_step(&doctored, &ImportOptions::default(), Tol::witness())
    else {
        panic!("a carrier displaced 1e-3 m off the locus is NEVER trusted");
    };
    assert_eq!(id, 124, "the doctored seam, named");
    let msg = format!("{:?}", attempts);
    let measured = attempts.iter().find_map(|a| match a.refusal {
        topo::EulerOpError::Certification {
            error:
                geom_brep::CertifyError::PlaneNurbs(geom_brep::PlaneNurbsRefusal::Limb {
                    margin, ..
                }),
        } => Some(upper(margin)),
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
    let text = StepImportError::Adoption { id, attempts, file }.to_string();
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
    geom::Surface<f64>,
    geom::NurbsSurface<f64>,
)> {
    plane_nurbs_intersections(body)
        .into_iter()
        .filter(|(_, plane, _)| !is_cap(plane))
        .collect()
}

/// Whether a plane is one of the prisms' two caps, `z` constant: a
/// plane × NURBS `Intersection` on a cap is a cap RIM, which the plane ×
/// NURBS certificate takes for the line it is, not a wall–wall seam.
fn is_cap(plane: &geom::Surface<f64>) -> bool {
    matches!(plane, geom::Surface::Plane { normal, .. } if normal.z.abs() > 0.5)
}

/// Every plane × described-NURBS `Intersection` edge: the seams of
/// [`plane_nurbs_seams`] and the cap rims beside them.
fn plane_nurbs_intersections(
    body: &topo::Body<f64>,
) -> Vec<(
    geom_brep::keys::CurveKey,
    geom::Surface<f64>,
    geom::NurbsSurface<f64>,
)> {
    let surfaces: std::collections::BTreeMap<_, _> = body.surfaces().collect();
    let described = |k| match surfaces.get(&k) {
        Some(geom::Surface::Nurbs(n)) if !n.is_placeholder() => Some((**n).clone()),
        _ => None,
    };
    let plane = |k| match surfaces.get(&k) {
        Some(p @ geom::Surface::Plane { .. }) => Some((*p).clone()),
        _ => None,
    };
    body.curves()
        .filter_map(|(key, geom)| {
            let topo::CurveGeom::Certified(curve) = geom else {
                return None;
            };
            let geom_brep::EdgeDescription::Intersection { pair, .. } = curve.description() else {
                return None;
            };
            let (plane, wall) = pair
                .keys()
                .into_iter()
                .find_map(|k| plane(k).zip(described(pair.other(k)?)))?;
            Some((key, plane, wall))
        })
        .collect()
}

// The seam's own certified NUMBERS are pinned where they are measured
// rather than re-derived here. What the ε-fine branches above assert
// about the payload is only that its number EXPLAINS the refusal
// (`sup > eps`) — not the number itself, which would be a second copy
// of a measured quantity. Its value at the finest row measured is
// 3.5528237131349995e-14 m, and
// geom-brep's `m7_8_plane_nurbs_edge` rows measure the same
// quarter-cylinder-meets-plane geometry at the lane and at the door.
// What the rows above add is the CONSEQUENCE — the certified seam
// charts on both of its faces and the body is first-class.
