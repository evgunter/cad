//! **Predicate 2's reach**: a band's material — what a convex band
//! removes and a concave one adds — is metered against every face of
//! the body that is not a support of its chain, in any shell, before
//! anything is built.
//!
//! The witnesses are bodies main built wrong: a cavity whose concave
//! fillet grows into an island standing in it, on one shell and on two
//! solids, and a thin revolved wall whose convex inner fillet leaves the
//! material through the far wall. Beside each refusal stands its control:
//! the same body with the obstacle clear of the band, which builds.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, Tol};
use sweep::Revolution;
use sweep::blend::{BlendError, BlendRequest, fillet_edges};
use sweep::test_support::{band_reach, corners, revolved_about_y, rim_arcs_at};
use topo::{Body, mass_properties, validate_geometric};

use crate::common::cavity::{brick, cavity_edges, rod, vented_cavity};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("the witness band")
}

fn fuse(what: &str, a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let t = tol();
    let a = sweep::test_support::finished(&format!("{what}: the first operand"), a.clone(), t);
    let b = sweep::test_support::finished(&format!("{what}: the second operand"), b.clone(), t);
    topo::union(&a, &b, t)
        .unwrap_or_else(|e| panic!("{what}: the union succeeds: {e:?}"))
        .body()
        .unwrap_or_else(|| panic!("{what}: the union leaves material"))
        .body
        .clone()
        .into_body()
}

/// [`vented_cavity`] with an island standing `gap` off every cavity wall
/// but the ceiling, on a round stem of radius `0.1` through the floor:
/// one solid, one shell.
fn island_in_vented_cavity(gap: f64) -> Body<f64> {
    let lo = 1.0 + gap;
    let island = brick(
        Point3::new(lo, lo, lo),
        Point3::new(4.0 - lo, 4.0 - lo, 2.4),
    );
    let stem = rod(Point2::new(2.0, 2.0), 0.1, 0.9, lo + 0.05);
    fuse("island", &fuse("stem", &vented_cavity(), &stem), &island)
}

/// Whether `e` is the reach arm of predicate 2 naming a face, and the
/// face it names is not one of the cavity's supports.
fn reach_refusal(e: &BlendError) -> bool {
    matches!(e, BlendError::FaceClearance { .. })
}

/// The one-shell witness: the island `0.05` off the walls, inside the
/// band's reach (`r(1 − 1/√2) ≈ 0.073` at an edge). Main built a body
/// every tier admitted, counting the island's corners twice.
#[test]
fn a_concave_fillet_refuses_to_grow_into_an_island_on_one_shell() {
    let body = island_in_vented_cavity(0.05);
    let edges = cavity_edges(&body);
    assert_eq!(edges.len(), 12, "the cavity's twelve concave edges");
    let err = fillet_edges(&body, &edges, 0.25, tol())
        .map(|f| {
            format!(
                "built: tier 3 {:?}, V {:?}",
                validate_geometric(&f.body, tol()),
                mass_properties(&f.body, tol()).map(|m| m.volume)
            )
        })
        .expect_err("the band reaches the island");
    assert!(
        reach_refusal(&err.error),
        "refused by the reach meter: {:?}",
        err.error
    );
    assert!(
        matches!(err.error, BlendError::FaceClearance { bounded: false, .. }),
        "the island's own edge lies in the material the band adds, so the refusal is a \
         measurement and not a bound: {:?}",
        err.error
    );
}

/// The control: the island `0.15` off the walls, clear of every edge
/// band (`0.073`) and every corner patch (`r(1 − 1/√3) ≈ 0.106` per
/// axis), builds and is tier-3 valid.
#[test]
fn an_island_clear_of_the_band_builds() {
    let body = island_in_vented_cavity(0.15);
    let edges = cavity_edges(&body);
    let f = fillet_edges(&body, &edges, 0.25, tol())
        .unwrap_or_else(|e| panic!("the island is clear of the band: {:?}", e.error));
    assert_eq!(validate_geometric(&f.body, tol()), Ok(()), "tier 3");
}

/// The two-solid witness, pinned at the reach meter: the SEALED cavity
/// with the island as a second solid. The blend's per-shell door is what
/// carries this body through `fillet_edges` to the meter; the meter reads
/// every face of the body, in any shell, whichever door brings it there.
#[test]
fn the_reach_meters_an_island_that_is_a_second_solid() {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let sealed = crate::common::cavity::cut("cavity", &block, &cavity);
    let island = brick(Point3::new(1.05, 1.05, 1.05), Point3::new(2.95, 2.95, 2.95));
    let body = fuse("island", &sealed, &island);
    assert_eq!(body.solids().count(), 2, "the island is a second solid");
    let edges = cavity_edges(&body);
    assert_eq!(edges.len(), 12, "the void shell's twelve edges");
    let req = BlendRequest {
        body: &body,
        edges: edges.clone(),
        size: 0.25,
    };
    let err = band_reach(&req, band()).expect_err("the band reaches the island");
    assert!(reach_refusal(&err), "refused by the reach meter: {err:?}");
}

/// A thin bent wall (`0.05`, a 150° convex inner corner): `(a, 1) → (a,
/// 0)` down the bore, down-out along a 30° cone, back up the outer wall.
fn thin_flare(a: f64) -> Vec<(f64, f64)> {
    let w = 0.05;
    let (s, c) = (0.5_f64, 0.75_f64.sqrt());
    let dir = (s, -c);
    let n = (c, s);
    let k = (a, 0.0);
    let e = (k.0 + 2.0 * dir.0, k.1 + 2.0 * dir.1);
    let eo = (e.0 + w * n.0, e.1 + w * n.1);
    let t = (a + w - (k.0 + w * n.0)) / dir.0;
    let ko = (a + w, k.1 + w * n.1 + t * dir.1);
    vec![(a, 1.0), k, e, eo, ko, (a + w, 1.0)]
}

/// The convex witness (review of PR 4092): past `r ≈ 1.466` the band of
/// the flare's convex inner corner leaves the material through the
/// outer wall. Main refuses these radii at predicate 1 only because its
/// headroom reads the bore's curvature, which the ball does not roll
/// against; the reach meter is read directly so the row stands whatever
/// predicate 1 says. Below the crossing, the same meter certifies.
#[test]
fn a_convex_band_through_a_thin_wall_is_refused_by_the_reach_meter() {
    let a = 0.225;
    let body = revolved_about_y(corners(&thin_flare(a)), Revolution::Full, tol());
    let edges = rim_arcs_at(&body, a, 0.0);
    assert!(!edges.is_empty(), "the inner corner's rim");
    for r in [1.5, 1.6] {
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: r,
        };
        let err = band_reach(&req, band()).expect_err("the band leaves through the far wall");
        assert!(
            reach_refusal(&err),
            "r {r}: refused by the reach meter: {err:?}"
        );
    }
    for r in [0.5, 1.0] {
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: r,
        };
        if let Err(e) = band_reach(&req, band()) {
            panic!("r {r}: the band stays inside the wall: {e:?}");
        }
    }
}

/// The reach replays at the certified scalar: the same flare, revolved at
/// `Interval`, refuses past the crossing and certifies below it.
#[test]
fn the_reach_replays_at_the_interval_scalar() {
    use crate::common::interval::iv;
    use geom_core::Interval;
    let a = 0.225;
    let pts: Vec<(Point2<Interval>, Interval)> = thin_flare(a)
        .into_iter()
        .map(|(x, y)| (Point2::new(iv(x), iv(y)), iv(0.0)))
        .collect();
    let body = sweep::test_support::revolved_about_y_at::<Interval>(pts, Revolution::Full, tol());
    let edges = rim_arcs_at(&body, a, 0.0);
    assert!(!edges.is_empty(), "the inner corner's rim");
    let req = |r: f64| BlendRequest {
        body: &body,
        edges: edges.clone(),
        size: iv(r),
    };
    let err = band_reach(&req(1.6), band()).expect_err("the band leaves through the far wall");
    assert!(reach_refusal(&err), "refused by the reach meter: {err:?}");
    if let Err(e) = band_reach(&req(0.5), band()) {
        panic!("the band stays inside the wall: {e:?}");
    }
}
