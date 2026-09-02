//! **The PR 12 fix pass, as pins** — the reviewer's witnesses,
//! hardened from diagnostics into rows that fail if the fix regresses.
//! The diagnostic originals are kept verbatim in
//! `review_pr12_probes.rs`; this file is what CI reads.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use profile::RawLoop;

use geom_core::Tol;
use geom_core::{Affine3, Point2, Point3, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::{Extrusion, extrude};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::query;
use topo::{Body, BooleanDeclarations};

fn prism(pts: &[(f64, f64)], h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        pts.iter()
            .map(|(x, y)| ProfileVertex::new(Point2::new(*x, *y), 0.0))
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
                assert!(margin < 0.0);
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
/// no incident edge whose chart makes the octant an iso-rectangle. The
/// body still builds and passes tiers 1 and 2; tier 3 reports
/// `VolumeUncomputable` because the closed-form mass-properties
/// inventory has no spherical-triangle form. The gap is in `props`,
/// not in the body, and this row is what says so out loud.
#[test]
fn f4_an_oblique_trihedron_builds_and_reports_volume_uncomputable() {
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
    let f = fillet_edges(&clipped, &edges, 0.08, Tol::witness())
        .expect("an oblique trihedron still builds");
    assert_eq!(topo::validate(&f.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&f.body), Ok(()), "tier 2");
    let errs = topo::validate_geometric(&f.body, Tol::witness())
        .expect_err("tier 3 cannot meter a spherical triangle at M5");
    let text = format!("{errs:?}");
    assert!(
        text.contains("VolumeUncomputable") && text.contains("NotIsoRectangle"),
        "the refusal must name the props inventory's gap: {text}"
    );
}
