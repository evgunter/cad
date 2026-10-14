//! **Predicate 2's reach**: a band's material — what a convex band
//! removes and a concave one adds — is metered against every face of
//! the body that is not a support of its chain, in any shell, before
//! anything is built.
//!
//! The witnesses are bodies main built wrong: a cavity whose concave
//! fillet grows into an island standing in it, on one shell, on two
//! solids and from one edge cut off at its end walls; a round void whose
//! floor rim grows into a washer, its bore a round or a square ring; and
//! a thin revolved wall whose convex inner fillet leaves the material
//! through the far wall. Beside each refusal stands its control:
//! the same body with the obstacle clear of the band, which builds.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, Tol};
use sweep::Revolution;
use sweep::blend::{BlendError, BlendRequest, fillet_edges};
use sweep::test_support::{band_reach, corners, finished, revolved_about_y, rim_arcs_at};
use topo::{AtRestBody, Body, mass_properties, validate_geometric};

use crate::common::cavity::{brick, cavity_edges, rod, vented_cavity};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("the witness band")
}

/// The union of a finished `a` and the fixture `b`, finished.
fn fuse(what: &str, a: &AtRestBody<f64>, b: &Body<f64>) -> AtRestBody<f64> {
    let t = tol();
    let b = sweep::test_support::finished(&format!("{what}: the second operand"), b.clone(), t);
    topo::union(a, &b, t)
        .unwrap_or_else(|e| panic!("{what}: the union succeeds: {e:?}"))
        .body()
        .unwrap_or_else(|| panic!("{what}: the union leaves material"))
        .body
        .clone()
}

/// [`vented_cavity`] with an island standing `gap` off every cavity wall
/// but the ceiling, on a round stem of radius `0.1` through the floor:
/// one solid, one shell.
fn island_in_vented_cavity(gap: f64) -> AtRestBody<f64> {
    let lo = 1.0 + gap;
    let island = brick(
        Point3::new(lo, lo, lo),
        Point3::new(4.0 - lo, 4.0 - lo, 2.4),
    );
    let stem = rod(Point2::new(2.0, 2.0), 0.1, 0.9, lo + 0.05);
    let cavity = finished("the cavity", vented_cavity(), tol());
    fuse("island", &fuse("stem", &cavity, &stem), &island)
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

/// The two-solid witness: the SEALED cavity with the island as a second
/// solid, the void's twelve edges filleted. The per-shell door carves
/// the void's shell alone, and the reach reads every face of the body,
/// in any shell, so the island refuses it — through `fillet_edges` and
/// at the meter itself alike.
#[test]
fn the_reach_meters_an_island_that_is_a_second_solid() {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let sealed = crate::common::cavity::cut("cavity", &block, &cavity);
    let island = brick(Point3::new(1.05, 1.05, 1.05), Point3::new(2.95, 2.95, 2.95));
    let body = fuse(
        "island",
        &finished("the sealed cavity", sealed, tol()),
        &island,
    );
    assert_eq!(body.solids().count(), 2, "the island is a second solid");
    let edges = cavity_edges(&body);
    assert_eq!(edges.len(), 12, "the void shell's twelve edges");
    let err = fillet_edges(&body, &edges, 0.25, tol())
        .map(|f| {
            format!(
                "built: V {:?}",
                mass_properties(&f.body, tol()).map(|m| m.volume)
            )
        })
        .expect_err("the band reaches the island");
    assert!(
        matches!(err.error, BlendError::FaceClearance { bounded: false, .. }),
        "the island's own edge lies in the material the void's band adds: {:?}",
        err.error
    );
    let req = BlendRequest {
        body: &body,
        edges: edges.clone(),
        size: 0.25,
    };
    let err = band_reach(&req, band()).expect_err("the band reaches the island");
    assert!(reach_refusal(&err), "refused by the reach meter: {err:?}");
}

/// Two bands of one request: the sealed cavity's twelve edges and one
/// TOP edge of the island, `0.05` off the walls. The island's front face
/// is a support of that edge's band, which replaces its upper strip; its
/// lower part and the island's sharp bottom edges stay in the material
/// the void's band adds, so the request refuses. (With all
/// twelve island edges requested, the island's round recedes from the
/// void's and the request builds: `blend_per_shell_carry`.)
#[test]
fn a_co_requested_band_replaces_only_its_own_strip() {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let sealed = crate::common::cavity::cut("cavity", &block, &cavity);
    let island = brick(Point3::new(1.05, 1.05, 1.05), Point3::new(2.95, 2.95, 2.95));
    let body = fuse(
        "island",
        &finished("the sealed cavity", sealed, tol()),
        &island,
    );
    let mut req = cavity_edges(&body);
    req.extend(crate::common::cavity::edges_with_corners(&body, |p| {
        (p.z - 2.95).abs() < 1e-9
            && [p.x, p.y]
                .iter()
                .all(|c| (c - 1.05).abs() < 1e-9 || (c - 2.95).abs() < 1e-9)
    }));
    assert_eq!(
        req.len(),
        16,
        "the void's twelve edges and the island's top four"
    );
    // Read at the meter: this request's walk ends a cavity chain at a
    // corner predicate 4 refuses first, which is not this row's question.
    let req = BlendRequest {
        body: &body,
        edges: req,
        size: 0.25,
    };
    let err =
        band_reach(&req, band()).expect_err("the island's lower faces stay in the void's band");
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

/// The tour teapot's lid, its three rims in one call (the flange's cone
/// × plane, the dome's sphere × cone, the knob's cylinder × plane). The
/// dome is a sphere ZONE beside the flange's band: its sphere passes
/// through that band's reach below the dome's foot, and the zone does
/// not. The meter reads a coaxial face's extent off its own boundary, so
/// the lid builds.
#[test]
fn a_sphere_zone_beside_a_band_is_metered_by_its_own_extent() {
    let u = 1.0 / 256.0;
    let base = 0.3;
    // The dome: the circle about `(0, base + u)` through `(12u, base +
    // 6u)` and `(5u, base + 13u)`, the 5-12-13 points.
    let bulge = ((120.0_f64 / 169.0).acos() / 4.0).tan();
    let lid = vec![
        (Point2::new(0.0, base), 0.0),
        (Point2::new(14.0 * u, base), 0.0),
        (Point2::new(12.0 * u, base + 6.0 * u), bulge),
        (Point2::new(5.0 * u, base + 13.0 * u), 0.0),
        (Point2::new(5.0 * u, base + 18.0 * u), 0.0),
        (Point2::new(0.0, base + 18.0 * u), 0.0),
    ];
    let body = revolved_about_y(lid, Revolution::Full, tol());
    let rims = [
        (14.0 * u, base),
        (12.0 * u, base + 6.0 * u),
        (5.0 * u, base + 18.0 * u),
    ];
    let edges: Vec<_> = rims
        .iter()
        .flat_map(|&(r, y)| rim_arcs_at(&body, r, y))
        .collect();
    assert_eq!(edges.len(), 6, "three rims, two half-arcs each");
    let f = fillet_edges(
        &sweep::test_support::at_rest(&body, tol()),
        &edges,
        2.0 * u,
        tol(),
    )
    .unwrap_or_else(|e| panic!("the dome is clear of the flange's band: {:?}", e.error));
    assert_eq!(validate_geometric(&f.body, tol()), Ok(()), "tier 3");
}

/// One cavity floor edge, `y = z = 1`, alone: the band is cut off at
/// both end walls in their planes (the planar band's local carve), and
/// the island `gap` off the walls stands beside it, clear of both chain
/// vertices.
fn one_floor_edge(body: &Body<f64>) -> Vec<topo::EdgeKey> {
    crate::common::cavity::edges_with_corners(body, |p| {
        crate::common::cavity::cavity_corner(p)
            && (p.y - 1.0).abs() < 1e-12
            && (p.z - 1.0).abs() < 1e-12
    })
}

/// A band cut off at its end faces is metered over the window those
/// faces' planes close: the island's front-bottom edge, `0.05` off the
/// floor and the wall, lies in the material the one-edge band adds; at
/// `0.15` it is clear and the cut-off builds.
#[test]
fn a_band_cut_off_at_its_end_faces_refuses_an_island_in_its_reach() {
    let body = island_in_vented_cavity(0.05);
    let edges = one_floor_edge(&body);
    assert_eq!(edges.len(), 1, "one floor edge");
    let err = fillet_edges(&body, &edges, 0.25, tol())
        .map(|f| format!("built: tier 3 {:?}", validate_geometric(&f.body, tol())))
        .expect_err("the cut-off band reaches the island");
    assert!(
        matches!(err.error, BlendError::FaceClearance { bounded: false, .. }),
        "the island's edge lies in the material the cut-off band adds: {:?}",
        err.error
    );
    let clear = island_in_vented_cavity(0.15);
    let edges = one_floor_edge(&clear);
    let f = fillet_edges(&clear, &edges, 0.25, tol())
        .unwrap_or_else(|e| panic!("the island is clear of the cut-off band: {:?}", e.error));
    assert_eq!(validate_geometric(&f.body, tol()), Ok(()), "tier 3");
}

/// A round void, `ρ ≤ 1.5` over `y ∈ [1, 3]` about the `y` axis, sealed
/// in a block, with a washer standing `gap` off its floor and its wall
/// as a second solid. `bore` cuts the washer's hole: `None` keeps the
/// revolve's round bore, so each flat face is one plane annulus whose
/// bore is a ring; `Some(h)` cuts a square hole of half-side `h` along
/// the axis instead, so the ring is a polygon.
fn washer_in_round_void(gap: f64, bore: Option<f64>) -> AtRestBody<f64> {
    let block = brick(Point3::new(-3.0, 0.0, -3.0), Point3::new(3.0, 4.0, 3.0));
    let void = revolved_about_y(
        corners(&[(0.0, 1.0), (1.5, 1.0), (1.5, 3.0), (0.0, 3.0)]),
        Revolution::Full,
        tol(),
    );
    let sealed = crate::common::cavity::cut("void", &block, &void);
    let (y0, rho) = (1.0 + gap, 1.5 - gap);
    let washer = match bore {
        None => revolved_about_y(
            corners(&[(0.5, y0), (rho, y0), (rho, 1.5), (0.5, 1.5)]),
            Revolution::Full,
            tol(),
        ),
        Some(h) => {
            let disk = revolved_about_y(
                corners(&[(0.0, y0), (rho, y0), (rho, 1.5), (0.0, 1.5)]),
                Revolution::Full,
                tol(),
            );
            let hole = brick(Point3::new(-h, 0.5, -h), Point3::new(h, 2.0, h));
            crate::common::cavity::cut("bore", &disk, &hole)
        }
    };
    let body = fuse(
        "washer",
        &finished("the sealed void", sealed, tol()),
        &washer,
    );
    assert_eq!(body.solids().count(), 2, "the washer is a second solid");
    body
}

/// The void's floor rim, a circular spine, reaches the washer's flat
/// underside — a coaxial plane metered in the band's meridian sheet,
/// its extent read off its outer circle and its bore ring together —
/// and its outer wall. Both bores refuse at `0.05` and build at `0.15`.
#[test]
fn a_circular_band_meters_a_washer_through_its_ring_round_or_square() {
    for bore in [None, Some(0.35)] {
        let body = washer_in_round_void(0.05, bore);
        let edges = rim_arcs_at(&body, 1.5, 1.0);
        assert!(!edges.is_empty(), "{bore:?}: the void's floor rim");
        let err = fillet_edges(&body, &edges, 0.25, tol())
            .map(|f| format!("built: tier 3 {:?}", validate_geometric(&f.body, tol())))
            .expect_err("the void's band reaches the washer");
        assert!(
            reach_refusal(&err.error),
            "{bore:?}: refused by the reach meter: {:?}",
            err.error
        );
        let clear = washer_in_round_void(0.15, bore);
        let edges = rim_arcs_at(&clear, 1.5, 1.0);
        let f = fillet_edges(&clear, &edges, 0.25, tol())
            .unwrap_or_else(|e| panic!("{bore:?}: the washer is clear of the band: {:?}", e.error));
        assert_eq!(
            validate_geometric(&f.body, tol()),
            Ok(()),
            "{bore:?}: tier 3"
        );
    }
}
