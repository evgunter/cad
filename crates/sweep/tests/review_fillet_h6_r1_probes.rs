//! **Review probes, FILLET-H6 lane r1** — the cap-rim classifier read
//! at the EDGE of the admitted set, not at its comfortable interior.
//!
//! These rows build the worst admitted extrusion (in-plane ε against
//! height K·ε, so the tilted wall's normal is `(K, 0, −1)/√(K²+1)` and
//! `sin θ = K/√(K²+1)`) over a rectangle with a short rim `f·K·ε` long.
//! Levered at that rim's own length the wedge would land in the band
//! or under it for small `f`; levered at the faces' extent
//! (`topo::wedge_reach`), at least the tilted wall's distance
//! `ε·√(K²+1)` to its far rim, it is at least the height `K·ε`, so
//! every rim is `Intersection` at every K and every `f`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cap_rims::cap_rims;
use geom_brep::EdgeDescription;
use geom_core::{Point2, Tol, Vec3};
use profile::{
    Profile, ProfileLoop, RawLoop, SketchPlane, ValidatedProfile, test_support::bulge_loop,
};
use sweep::ExtrudeSide;
use sweep::{Extruded, Extrusion, extrude};
use topo::validate_geometric;

fn rect(sx: f64, sy: f64) -> ValidatedProfile<f64> {
    Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(sx, 0.0),
            Point2::new(sx, sy),
            Point2::new(0.0, sy),
        ])],
    )
    .validate(Tol::witness())
    .unwrap()
}

fn cap_rim_descriptions(built: &Extruded<f64>) -> Vec<EdgeDescription<f64>> {
    cap_rims(built).into_iter().map(|r| r.description).collect()
}

/// The worst admitted obliquity on the tightest admitted rim: every
/// cap rim is the faces' corner. At K = 10 a rim levered at its own
/// length would read in band at `f = 1.002` (9.97ε); run with
/// `CAD_AMBIGUITY_K=1.1` it would read smooth. The cap rim's
/// `SliverRim` and at-rest `MaterialPairing` arms this row reached
/// then have no row now
/// (`work/lever/the-arms-the-faces-lever-left-without-a-row.md`).
#[test]
fn worst_admitted_obliquity_on_a_tight_rim_is_the_faces_corner() {
    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    let w = Vec3::new(eps, 0.0, k * eps);
    for f in [1.002, 1.1, 1.5] {
        let built = extrude(&rect(2.0, f * k * eps), Extrusion::Vector(w), tol)
            .unwrap_or_else(|e| panic!("f={f} K={k}: the rectangle extrudes, refused: {e:?}"));
        for d in cap_rim_descriptions(&built) {
            assert!(
                matches!(d, EdgeDescription::Intersection { .. }),
                "f={f} K={k}: a cap rim carries {d:?}",
            );
        }
        assert_eq!(validate_geometric(&built.body, tol), Ok(()), "f={f} K={k}");
    }
}

/// Bodies whose every cap rim is transverse validate at rest. Under a
/// planted `Transverse → Smooth` verdict at the rim this row is the
/// tier-3 witness: it says whether the mis-description is caught by
/// the at-rest gate (the cylinder wall's `TangentIntersection` is
/// refused at certification; the plane wall's chart image is the case
/// worth measuring).
#[test]
fn transverse_cap_rims_validate_at_rest() {
    let tol = Tol::witness();
    let circle = Profile::new(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            (Point2::new(-1.5, 0.0), 1.0),
            (Point2::new(1.5, 0.0), 1.0),
        ])],
    )
    .validate(tol)
    .unwrap();
    for (name, profile) in [("square", rect(2.0, 2.0)), ("cylinder", circle)] {
        let built = extrude(
            &profile,
            Extrusion::Distance {
                depth: 1.0,
                side: ExtrudeSide::Along,
            },
            tol,
        )
        .unwrap_or_else(|e| panic!("{name}: {e:?}"));
        for d in cap_rim_descriptions(&built) {
            assert!(
                matches!(d, EdgeDescription::Intersection { .. }),
                "{name}: cap rim carries {d:?}",
            );
        }
        validate_geometric(&built.body, tol).unwrap_or_else(|e| panic!("{name}: tier 3 {e:?}"));
    }
}

/// **An in-band second order refuses at the revolve door**, on the
/// body that tells the two verbs' policies apart: a bore cylinder of
/// radius `r` meeting a unit-minor-radius torus tangentially at a
/// latitude circle of radius `r`, with `r² / 2 = (1 + K)·ε / 2` — the
/// second-order sagitta squarely inside the band at every ε. The rule
/// is one rule on both verbs, so this refuses `SliverJoin` under
/// `tangent_second_order` exactly as the extrude strut does, and the
/// body tier 3 would have to refuse is never handed back.
#[test]
fn revolve_refuses_an_in_band_second_order_at_the_door() {
    use sweep::{Revolution, RevolveAxis, revolve};

    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    let r = ((1.0 + k) * eps).sqrt();
    let b = (core::f64::consts::FRAC_PI_8).tan();
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(1.5, 0.0), 0.0),
            (Point2::new(1.5, 2.0), 0.0),
            (Point2::new(1.0 + r, 2.0), b),
            (Point2::new(r, 1.0), 0.0),
        ])],
    )
    .validate(tol)
    .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    match revolve(&profile, axis, Revolution::Full, tol) {
        Err(sweep::RevolveError::SliverJoin { source, .. }) => assert_eq!(
            source.predicate,
            Some("tangent_second_order"),
            "the refusal must name the must-carry rule's metered predicate",
        ),
        Err(other) => panic!("the in-band latitude join must refuse as a sliver JOIN: {other:?}"),
        Ok(_) => panic!("an in-band second-order margin was built silently"),
    }
}
