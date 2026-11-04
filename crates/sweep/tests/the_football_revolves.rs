//! **The football revolves.** An arc of a spindle-torus sheet's circle
//! from one of its axis points to the other, closed by its chord on the
//! axis and revolved about that chord: the lemon (`R < 0`), whose two
//! apexes are vertices, and the apple (`0 < R < r`), whose two dimple
//! apexes are. The horn (`R = r`) is reached at its one cusp from one
//! side as a needle tip; from both sides it pinches.
//!
//! Each body is measured against a closed form of revolution written
//! independently of the kernel's: Pappus on the circular segment the
//! profile bounds, and on its arc, for the football; the integral of
//! `πρ(z)²` along the axis for the needle.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::validated;
use core::f64::consts::PI;
use geom_core::{Point2, Tol, Vec2};
use profile::test_support::bulge_loop;
use sweep::{Revolution, RevolveAxis, RevolveError, revolve};

fn y_axis() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

/// The football of the sheet `(R, r)` about the sketch `y` axis: the
/// arc from the lower apex to the upper through the circle's widest
/// point, closed down the axis.
fn football(big: f64, r: f64) -> profile::ProfileLoop<f64> {
    let h = (r * r - big * big).sqrt();
    let v_star = (-big / r).acos();
    bulge_loop(vec![
        (Point2::new(0.0, -h), (v_star / 2.0).tan()),
        (Point2::new(0.0, h), 0.0),
    ])
}

/// The football's volume and area by Pappus. The axis cuts the circle
/// at `d = |R|` from its centre, parting it into a segment of half
/// angle `α = acos(d/r)` across the axis from the centre and the rest:
/// the lemon is that segment, the apple the rest. The segment's area is
/// `r²α − d√(r² − d²)`, its centroid `2(r² − d²)^{3/2}/(3A)` from the
/// centre; an arc of half angle `θ` has its centroid `r·sin θ/θ` from it.
fn pappus(big: f64, r: f64) -> (f64, f64) {
    let d = big.abs();
    let alpha = (d / r).acos();
    let segment = r * r * alpha - d * (r * r - d * d).sqrt();
    let centroid = 2.0 * (r * r - d * d).powf(1.5) / (3.0 * segment);
    if big < 0.0 {
        let arc = 2.0 * r * alpha;
        (
            2.0 * PI * segment * (centroid - d),
            2.0 * PI * arc * (r * alpha.sin() / alpha - d),
        )
    } else {
        // The disc's moment about the axis less the segment's, whose
        // centroid is across the axis.
        let moment = PI * r * r * big - segment * (big - centroid);
        let rest = PI - alpha;
        let arc = 2.0 * r * rest;
        (
            2.0 * PI * moment,
            2.0 * PI * arc * (big + r * alpha.sin() / rest),
        )
    }
}

/// The body revolved, its datum read, meshed closed, through tier 3, and
/// measured against `(volume, area)`.
fn holds(what: &str, lp: profile::ProfileLoop<f64>, sheet: f64, (volume, area): (f64, f64)) {
    let tol = Tol::witness();
    let built = revolve(&validated(vec![lp]), y_axis(), Revolution::Full, tol)
        .unwrap_or_else(|e| panic!("{what}: {e}"));
    let sheets: Vec<f64> = built
        .body
        .faces()
        .filter_map(|(_, f)| match built.body.get_surface(f.surface) {
            Some(geom::Surface::Spindle { major_radius, .. }) => Some(*major_radius),
            _ => None,
        })
        .collect();
    assert!(
        sheets.len() == 2 && sheets.iter().all(|&m| (m - sheet).abs() <= 1e-15),
        "{what}: the sheet faces' major radii {sheets:?}, not two of {sheet}"
    );
    let delta = 1e-2;
    let mesh = mesh::tessellate(&built.body, delta, tol)
        .unwrap_or_else(|e| panic!("{what}: tessellate: {e:?}"));
    mesh::validate::check_mesh(&mesh).unwrap_or_else(|e| panic!("{what}: the mesh: {e:?}"));
    crate::common::differential::assert_meshed_geometry(what, &built.body, &mesh, delta);
    let body = topo::AtRestBody::validate(built.body, tol)
        .unwrap_or_else(|e| panic!("{what}: tier 3: {e:?}"));
    let props = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("{what}: {e:?}"));
    let got = props.enclosure;
    assert!(
        (got.volume - volume).abs() <= 1e-10 * volume + got.volume_pad,
        "{what}: volume {} against {volume} (pad {})",
        got.volume,
        got.volume_pad
    );
    assert!(
        (got.surface_area - area).abs() <= 1e-10 * area + got.area_pad,
        "{what}: area {} against {area} (pad {})",
        got.surface_area,
        got.area_pad
    );
}

/// **The football and the apple build through the revolve door** with
/// their apexes as vertices: two sheet faces, each a π-band between the
/// two meridians that end at both apexes (the pole form), meshed closed
/// with a fan at each apex, through tier 3, and measuring their exact
/// volume and area.
#[test]
fn the_football_and_the_apple_revolve() {
    for (what, big, r) in [
        ("the lemon", -0.5, 1.0),
        ("a slim lemon", -0.9, 1.0),
        ("the apple", 0.4, 1.0),
        ("a small apple", 0.3, 0.7),
    ] {
        holds(what, football(big, r), big, pappus(big, r));
    }
}

/// **A horn's cusp reached from one side is a needle tip**: the horn of
/// radius 1 (`R = r`, its centre `(1, 0)` and its cusp at the origin),
/// its arc from the cusp to `(1, 1)`, closed by a line to the axis and
/// down it. The face is the sheet's band `π/2 ≤ v ≤ π` between the cusp
/// and a disc. The arc's centre is read a hair from `r`, the revolve
/// decides it on the axis, and the sheet is stored a horn exactly, its
/// one cusp at the arc's end.
///
/// `ρ(z) = 1 − √(1 − z²)` on `0 ≤ z ≤ 1`, so `π∫ρ² dz = π(5/3 − π/2)`;
/// the area is the band's `2π(π/2 − 1)` and the disc's `π`.
#[test]
fn a_horn_reached_at_its_cusp_from_one_side_is_a_needle_tip() {
    let needle = bulge_loop(vec![
        (Point2::new(0.0, 0.0), -(PI / 8.0).tan()),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    holds(
        "the needle",
        needle,
        1.0,
        (
            PI * (5.0 / 3.0 - PI / 2.0),
            2.0 * PI * (PI / 2.0 - 1.0) + PI,
        ),
    );
}

/// **A horn's cusp reached from both sides pinches**: the horn's whole
/// circle, its vertex at the cusp, would revolve to a solid that meets
/// itself at one point, which the full revolve refuses at the vertex.
#[test]
fn a_horn_reached_at_its_cusp_from_both_sides_pinches() {
    let quarter = -(PI / 8.0).tan();
    let pinch = bulge_loop(vec![
        (Point2::new(0.0, 0.0), quarter),
        (Point2::new(1.0, 1.0), quarter),
        (Point2::new(2.0, 0.0), quarter),
        (Point2::new(1.0, -1.0), quarter),
    ]);
    let got = revolve(
        &validated(vec![pinch]),
        y_axis(),
        Revolution::Full,
        Tol::witness(),
    );
    assert!(
        matches!(
            got,
            Err(RevolveError::NonManifoldAxisContact { loop_index: 0, .. })
        ),
        "{:?}",
        got.map(|_| ())
    );
}

/// **The other sheet's apex is no apex of this one.** Both sheets of a
/// spindle torus share their apexes, so a body whose sheet faces are
/// re-stored on the other sheet (`R → −R`, the same axis, apexes and
/// tube) has every vertex where it was, every apex at a vertex, and
/// every edge on the whole torus. Tier 3 must refuse it: the edges are
/// off the sheet the faces now name.
#[test]
fn a_football_restored_on_the_other_sheet_is_refused() {
    let tol = Tol::witness();
    for (what, big, r) in [("the lemon", -0.5, 1.0), ("the apple", 0.4, 1.0)] {
        let built = revolve(
            &validated(vec![football(big, r)]),
            y_axis(),
            Revolution::Full,
            tol,
        )
        .unwrap();
        let mut body = built.body;
        let keys: Vec<_> = body
            .surfaces()
            .filter(|(_, s)| matches!(s, geom::Surface::Spindle { .. }))
            .map(|(k, _)| k)
            .collect();
        assert!(!keys.is_empty(), "{what}: no sheet");
        for key in keys {
            let Some(geom::Surface::Spindle {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            }) = body.get_surface(key).cloned()
            else {
                unreachable!("the key was read off a sheet")
            };
            body = body
                .with_surface_moved_for_tests(
                    key,
                    geom::Surface::Spindle {
                        center,
                        axis,
                        major_radius: -major_radius,
                        minor_radius,
                        u_ref,
                    },
                )
                .unwrap();
        }
        assert!(
            topo::AtRestBody::validate(body, tol).is_err(),
            "{what}: the other sheet passes tier 3"
        );
    }
}

/// **A football revolved part way refuses typed** at the meridian a cap
/// meets the sheet on: that edge ends at both apexes, where the sheet's
/// distance bound has no positive lever
/// (`work/spiricarm/a-football-revolved-part-way-refuses-at-its-cap-meridians-certificate.md`).
#[test]
fn a_football_revolved_part_way_refuses_typed() {
    for (what, big) in [("the lemon", -0.5), ("the apple", 0.4)] {
        let got = revolve(
            &validated(vec![football(big, 1.0)]),
            y_axis(),
            Revolution::Partial(2.0),
            Tol::witness(),
        );
        assert!(
            matches!(got, Err(RevolveError::Op { .. })),
            "{what}: {:?}",
            got.map(|_| ())
        );
    }
}
