//! **The PR 12 fix pass, as pins** — the reviewer's witnesses,
//! hardened from diagnostics into rows that fail if the fix regresses.
//! The diagnostic originals are kept verbatim in
//! `review_pr12_probes.rs`; this file is what CI reads.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Vec3};
use geom_core::{Band, ErrorTextReading, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::{Extrusion, extrude};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::query;
use topo::{Body, BooleanDeclarations};

fn prism(pts: &[(f64, f64)], h: f64) -> Body<f64> {
    let lp = bulge_loop(
        pts.iter()
            .map(|(x, y)| (Point2::new(*x, *y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
}

/// A unit-side regular hexagonal prism (circumradius 1 ⇒ side 1,
/// apothem √3/2 ≈ 0.866), tall enough that cap-to-cap pairs never bind.
fn hexagonal_prism() -> Body<f64> {
    let pts: Vec<(f64, f64)> = (0..6)
        .map(|i| {
            let th = PI / 3.0 * f64::from(i);
            (th.cos(), th.sin())
        })
        .collect();
    prism(&pts, 4.0)
}

/// **F2 (MAJOR), the row that would have caught it.** Every corner
/// face of a hexagonal prism satisfies the octant chart's own
/// iso-rectangle condition — the third support's normal is parallel to
/// one incident edge at every prism vertex — so the whole solid must
/// be tier-3 valid, mass properties included.
///
/// It was not, because the chart aimed at whichever incident edge came
/// first in link order rather than at the one the condition names. The
/// pick is now order-free (it minimises `|n_c × axis|` over the three
/// candidates), so it finds the admitting edge whenever one exists.
#[test]
fn f2_every_corner_face_of_a_hexagonal_prism_is_tier3_valid() {
    let body = hexagonal_prism();
    let edges = query::all_edges(&body);
    assert_eq!(edges.len(), 18, "a hexagonal prism has 18 edges");
    let f = fillet_edges(&body, &edges, 0.3, Tol::witness()).expect("the hexagonal prism fillets");
    assert_eq!(topo::validate(&f.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&f.body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&f.body, Tol::witness()),
        Ok(()),
        "tier 3 — every corner face admits the iso-rectangle chart"
    );
    assert_eq!(f.corner_faces.len(), 12, "one octant per prism vertex");
    assert_eq!(f.blend_faces.len(), 18, "one blend per prism edge");
    let props =
        topo::mass_properties(&f.body, Tol::witness()).expect("closed-form mass properties");
    assert!(props.volume > 0.0);
    assert_eq!(
        props.volume_pad, 0.0,
        "a closed-form body needs no enclosure pad"
    );
}

/// The same, at a second radius and on a non-regular prism, so the fix
/// is not a hexagon coincidence: an irregular pentagonal prism.
#[test]
fn f2_an_irregular_prism_is_tier3_valid_too() {
    let body = prism(
        &[(0.0, 0.0), (2.0, 0.0), (2.6, 1.1), (1.2, 2.0), (-0.3, 1.3)],
        3.0,
    );
    let edges = query::all_edges(&body);
    let f =
        fillet_edges(&body, &edges, 0.12, Tol::witness()).expect("the pentagonal prism fillets");
    assert_eq!(
        topo::validate_geometric(&f.body, Tol::witness()),
        Ok(()),
        "tier 3"
    );
    assert_eq!(f.corner_faces.len(), 10);
}

/// **F1 (MIN), the conservatism made a row.** The clearance screen is
/// EXACT when two boundary edges face each other and CONSERVATIVE when
/// they meet at an angle. The hexagon's cap is the reviewer's witness
/// on both sides of that line:
///
/// - it builds and certifies at `r = 0.499`, so the screen is not
///   simply refusing everything;
/// - it refuses from `r = 0.51`, although the cap's true limit is the
///   apothem `0.866` — the screen subtracts two setbacks from ONE
///   straight-line gap between second-neighbour cap edges (which is
///   the side, `1.0`), and those two setbacks eat along inward normals
///   `120°` apart, not along that gap.
///
/// The row pins the wording as much as the number: the refusal must
/// say it cannot CERTIFY, never that the face IS consumed, because at
/// `r ∈ (0.5, 0.866]` the stronger statement is false.
#[test]
fn f1_the_clearance_screen_is_conservative_by_direction_on_the_hexagon() {
    let body = hexagonal_prism();
    let edges = query::all_edges(&body);

    for r in [0.30, 0.45, 0.499] {
        let f = fillet_edges(&body, &edges, r, Tol::witness())
            .unwrap_or_else(|e| panic!("r = {r} is well inside the screen: {e}"));
        assert_eq!(
            topo::validate_geometric(&f.body, Tol::witness()),
            Ok(()),
            "tier 3 at r = {r}"
        );
    }

    // The conservative band: refused, but only as an uncertified
    // clearance — and the apothem says the cap really does survive here.
    let apothem = 3.0_f64.sqrt() / 2.0;
    for r in [0.51, 0.6, 0.8] {
        assert!(r < apothem, "the row is only interesting below the apothem");
        match fillet_edges(&body, &edges, r, Tol::witness()).map_err(|r| r.error) {
            Err(e @ BlendError::FaceClearanceUncertified { margin, gap, .. }) => {
                assert_eq!(margin.predicate, "fillet3_face_clearance");
                assert!(
                    margin
                        .reading
                        .diagnostic_f64_for_error_text()
                        .value()
                        .is_some_and(|m| m < 0.0)
                );
                let ErrorTextReading::Value(gap) = gap.diagnostic_f64_for_error_text() else {
                    panic!("this lane classifies at f64, so the gap is one number: {gap:?}")
                };
                assert!(
                    (gap - 1.0).abs() < 1e-9,
                    "the binding gap is the hexagon's SIDE, not its apothem: {gap}"
                );
                let text = format!("{e}");
                assert!(
                    text.contains("cannot certify") && text.contains("conservative by direction"),
                    "the screen must not assert the face IS consumed: {text}"
                );
            }
            other => panic!("expected the clearance screen at r = {r}, got {other:?}"),
        }
    }
}

/// **F4, deviation 3's missing fixture.** A genuinely OBLIQUE
/// trihedron — a cube with one corner sliced by a tilted plane — has
/// no incident edge whose chart makes the octant an iso-rectangle, so
/// each corner patch is a spherical triangle bounded by circles tilted
/// against its chart. The body builds and passes all three tiers: the
/// sphere flux arm measures those patches by Gauss–Bonnet over their
/// arcs.
///
/// **It is also the pin on tier 3's curved check-6 EXEMPTION.** Five
/// of this body's faces are exactly the input on which
/// `boundary_material_sign` refuses (no rim encodes their side), and
/// check 6 must stay silent on them: the refusal is not a sense
/// disagreement. A raise there would fail tier 3 here.
///
/// Each corner patch is held to Girard's spherical-triangle area, and
/// the volume to a bracket that reads nothing of the kernel's
/// fillet: rounding a convex edge of length `ℓ` and interior angle `θ`
/// at radius `r` removes `r²·(cot(θ/2) − (π − θ)/2)·ℓ` at most (less
/// where blends meet at corners), which is under `r²·ℓ` for every
/// `θ` above 50°, and this clip's dihedrals are all within 0.4 rad of
/// a right angle; so the rounded body lies strictly between the clip's
/// volume and that less `r²·Σℓ`.
///
/// **And the pin on the octant's pcurve rows.** The oblique corners'
/// contact circles are GENERAL circles of their sphere's chart — neither
/// polar nor meridian — so the closed-form door has no image for them;
/// the mint routes them through the fitted lane. Every half-edge of
/// every corner face carries a certified row, some of them `Fitted`,
/// each one's dense map residual — measured between its certification
/// samples — under its stored envelope and that under the band, and
/// tier 3's pcurve pass re-certifies them clean. Take the route
/// away and those faces are rowless or refused, and this half goes red.
/// The chart boundary of every corner face that stores a fitted row
/// gets past the derivation (a pole joint or a wrap may still refuse
/// it, each for its own reason).
#[test]
fn f4_an_oblique_trihedron_builds_and_passes_tier_3() {
    let c1 = prism(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 1.0);
    let c2 = prism(&[(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)], 3.0);
    let c2 = topo::transform_rigid(
        &c2,
        &Affine3::translation(Vec3::new(-1.55, -1.55, -2.45)),
        Tol::witness(),
    )
    .unwrap();
    let c2 = topo::transform_rigid(
        &c2,
        &Affine3::rotation_about_axis(
            Point3::new(1.0, 1.0, 1.0),
            Vec3::new(1.0, -1.0, 0.0).normalize(),
            0.4,
        ),
        Tol::witness(),
    )
    .unwrap();
    let clipped = boolean_op_with(
        BooleanOp::Intersect,
        &c1,
        &c2,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .expect("the oblique clip")
    .body()
    .expect("a body")
    .body
    .clone();
    let edges = query::all_edges(&clipped);
    let r = 0.08;
    let f = fillet_edges(&clipped, &edges, r, Tol::witness())
        .expect("an oblique trihedron still builds");
    assert_eq!(topo::validate(&f.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&f.body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&f.body, Tol::witness()),
        Ok(()),
        "tier 3 meters the spherical triangles, and check 6 exempts them"
    );
    let total_length: f64 = edges
        .iter()
        .map(|&k| {
            let curve = clipped
                .get_edge(k)
                .and_then(|e| clipped.get_curve_geom(e.curve))
                .and_then(|g| g.certified())
                .expect("a certified edge");
            let (t0, t1) = curve.params();
            (curve.carrier().eval(t1) - curve.carrier().eval(t0)).norm()
        })
        .sum();
    let volume = |b: &Body<f64>| topo::mass_properties(b, Tol::witness()).unwrap().volume;
    let (clip, rounded) = (volume(&clipped), volume(&f.body));
    assert!(
        rounded < clip && rounded > clip - r * r * total_length,
        "rounded {rounded} outside ({}, {clip})",
        clip - r * r * total_length
    );

    let band = Band::linear(Tol::witness()).unwrap();
    // Each corner patch is a geodesic triangle on its ball — three
    // great-circle arcs, the balls' contact circles with the three
    // fillet cylinders, whose axes pass through the ball's centre — so
    // its area is Girard's: `r²·E`, with the excess `E` of the triangle
    // on the unit vectors from the centre to its three vertices, read
    // off nothing the flux arm reads.
    for &corner in &f.corner_faces {
        let face = f.body.get_face(corner).expect("the corner face resolves");
        let Some(&geom::Surface::Sphere { center, radius, .. }) = f.body.get_surface(face.surface)
        else {
            panic!("corner face {corner:?} is a sphere patch");
        };
        let (outer, _) = topo::props::loop_edges(&f.body, face.outer).expect("its loop");
        assert_eq!(outer.len(), 3, "corner face {corner:?} is a triangle");
        let u: Vec<Vec3<f64>> = outer
            .iter()
            .map(|e| {
                let t = if e.forward { e.t0 } else { e.t1 };
                (e.carrier.eval(t) - center) / radius
            })
            .collect();
        let excess = 2.0
            * (u[0].dot(u[1].cross(u[2])).abs()
                / (1.0 + u[0].dot(u[1]) + u[1].dot(u[2]) + u[2].dot(u[0])))
            .atan();
        let surface = f.body.get_surface(face.surface).unwrap();
        let area = geom_brep::props::curved_face(surface, &outer, face.sense, band)
            .expect("the patch measures")
            .area;
        let want = radius * radius * excess;
        assert!(
            (area - want).abs() <= 1e-12 * radius * radius,
            "corner face {corner:?}: area {area} against Girard's {want}"
        );
    }
    let mut fitted = 0;
    for &corner in &f.corner_faces {
        let face = f.body.get_face(corner).expect("the corner face resolves");
        let topo::LoopBoundary::Cycle { first } = f.body.get_loop(face.outer).unwrap().boundary
        else {
            panic!("a corner face's outer loop is a cycle");
        };
        let mut face_fitted = false;
        for he in f.body.loop_cycle(first).unwrap() {
            let row = f.body.pcurve(he).unwrap_or_else(|| {
                panic!("corner face {corner:?} half-edge {he:?} carries no pcurve row")
            });
            if matches!(row.pcurve(), geom_brep::Pcurve::Fitted(_)) {
                fitted += 1;
                face_fitted = true;
                // Between the samples: the dense map residual is under
                // the stored envelope, which is under the band.
                let edge = f.body.get_half_edge(he).unwrap().edge;
                let curve = f.body.get_edge(edge).unwrap().curve;
                let Some(topo::CurveGeom::Certified(curve)) = f.body.get_curve_geom(curve) else {
                    panic!("a minted row's edge has a certified carrier");
                };
                let ((t0, t1), carrier) = (curve.params(), curve.carrier());
                let surface = f.body.get_surface(face.surface).unwrap();
                let envelope = row.certificate().envelope;
                let dense = (0..=4000)
                    .map(|k| {
                        let t = t0 + (t1 - t0) * f64::from(k) / 4000.0;
                        let p = row.pcurve().eval(t);
                        (surface.eval(p.x, p.y) - carrier.eval(t)).norm()
                    })
                    .fold(0.0, f64::max);
                assert!(
                    dense <= envelope && envelope <= band.zero(),
                    "half-edge {he:?}: dense map residual {dense:e} m, envelope {envelope:e} \
                     m, band {:e} m",
                    band.zero()
                );
            }
        }
        if face_fitted {
            let chart = f.body.get_surface(face.surface).unwrap().clone();
            if let Err(e) = topo::pcurves::chart_boundary(&f.body, corner, &chart, band) {
                assert!(
                    !matches!(e, topo::pcurves::PcurveMintError::Certify { .. }),
                    "corner face {corner:?}'s boundary refused at the derivation: {e:?}"
                );
            }
        }
    }
    assert!(
        fitted > 0,
        "the oblique corners' general circles take the fitted lane"
    );
    let findings = topo::pcurves::validate_pcurves(&f.body, band);
    assert!(
        findings.is_empty(),
        "the octant's rows re-certify: {findings:?}"
    );
}
