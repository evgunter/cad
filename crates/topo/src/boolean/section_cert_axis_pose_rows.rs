//! **The axis pose reads the offset and the tilt as one sum** (module
//! docs, the sum): a partner whose axis stands within the band of the
//! torus axis on each term alone, but not on their sum, is not read
//! coaxial; and every row an arm serves on the parallel or coaxial
//! reading is decided across the reach beside the swing its pose admitted.
//!
//! Every pose tilts the partner in the plane of its offset, so the two
//! terms add at first order where the section lies, and checks the served
//! verdict against the closed form: the second tube's true stand-off from
//! the torus's tube, per meridian. The torus pair reads its pose over
//! the smaller torus's ball (`geom_brep::pair_section::torus`), so the
//! pose's pivot is the foot of the donut's centre on the second axis, and
//! its lever twice that foot's distance from the centre plus the donut's
//! `R + r`; a wall near-parallel to the torus axis meets the tube within
//! `r` of the pivot's height, where no tilt moves it by the swing, so the
//! pose is a second torus whose section lies a ball's radius out.
//!
//! - **split**, `0.9 + 0.9` and `0.75 + 0.75` zero bands (offset at the
//!   pivot, tilt at its lever): each row alone reads Zero; the swing is
//!   half the tilt's reading, so the offset reads `1.35` and `1.125` bands
//!   far, and the pose refuses on the offset row;
//! - **inside**, `0.4 + 0.4`: coaxial, its rows read beside a slack of
//!   `0.6`;
//! - **narrow**, `K = 1.2` and `0.95 + 0.95`: the offset's floor
//!   straddles.
//!
//! The straddles sit between the floor of the whole swing and that of
//! half of it, so an under-read swing serves them; and the turn rows read
//! a pose whose only residue past the band is the turn of a point the arm
//! reads off the pivot.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::search_kit::band;
use super::section_cert_rows::{cylinder, p, shape, tangent, torus, unit_cone};
use super::*;
use core::f64::consts::TAU;
use geom::Surface;
use geom_core::{Band, Point3, Vec3};

/// The donut's ball, `R + r`: the lever's reach from its centre.
const BALL: f64 = 2.5;

fn zero() -> f64 {
    band().zero()
}

/// The unit direction `(−s, 0, √(1 − s²))`.
fn tilted(s: f64) -> Vec3<f64> {
    Vec3::new(-s, 0.0, (1.0 - s * s).sqrt())
}

/// The donut and a second torus, `R₂ = 3 − m`, `r₂ = 0.5`, standing
/// outside its tube by the overlap `m` (the coaxial reach row's datum),
/// centred at the pivot `(e, 0, 0)` with its axis tilted in the `xz`
/// plane so that the tilt reads `tilt` at the lever over the donut's ball.
fn tori_pose(e: f64, tilt: f64, m: f64) -> Surface<f64> {
    Surface::Torus {
        center: p(e, 0.0, 0.0),
        axis: tilted(tilt / (2.0 * (e + BALL))),
        major_radius: 3.0 - m,
        minor_radius: 0.5,
        u_ref: Vec3::unit_y(),
    }
}

/// The tubes' true overlap `r + r₂ − D` per meridian of the donut, least
/// and greatest: `D` the donut's core point's distance from the second
/// torus's core circle.
fn tori_truth(t2: &Surface<f64>) -> (f64, f64) {
    let &Surface::Torus {
        center,
        axis,
        major_radius,
        minor_radius,
        ..
    } = t2
    else {
        panic!("a torus pose is a torus")
    };
    extremes(|u| {
        let q = p(2.0 * u.cos(), 2.0 * u.sin(), 0.0) - center;
        let (_, rho) = perp_to(q, axis);
        0.5 + minor_radius - (rho - major_radius).hypot(q.dot(axis))
    })
}

/// The least and greatest of `f` over the azimuth.
fn extremes(f: impl Fn(f64) -> f64) -> (f64, f64) {
    (0..3600)
        .map(|k| f(f64::from(k) * TAU / 3600.0))
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), g| {
            (lo.min(g), hi.max(g))
        })
}

/// The torus pose at `(e, tilt, m)` in zero bands, classified at `b`
/// beside its closed form.
fn tori(e: f64, tilt: f64, m: f64, b: Band) -> (Section<f64>, (f64, f64)) {
    let z = zero();
    let t2 = tori_pose(e * z, tilt * z, m * z);
    let donut = torus(p(0.0, 0.0, 0.0), 2.0, 0.5);
    // The classifier takes a reach for every pair; two tori read their
    // own ball and not this one, so it is the donut's, the rows' lever.
    let reach = Reach {
        centre: p(0.0, 0.0, 0.0),
        radius: BALL,
    };
    (
        classify(
            &donut,
            &t2,
            reach,
            None,
            Some(geom_brep::SsiLane::marched()),
            b,
        ),
        tori_truth(&t2),
    )
}

/// The run's `K`, in zero bands.
fn k() -> f64 {
    band().escalate() / zero()
}

/// The offset row a pose refuses on: the far row while `1.35` bands sit
/// inside the escalation edge, else the floor's.
fn offset_row(s: &Section<f64>) -> &'static str {
    let name = tangent(s);
    assert!(
        name == "section_axes_offset" || name == "section_axes_offset_floor",
        "refused on the offset, got {name}"
    );
    name
}

/// **Split: no reading.** At `0.9 + 0.9` and `0.75 + 0.75` each term
/// alone reads Zero and their sum does not, so the pose refuses on the
/// offset whatever the tube rows would read: two essential parallels or
/// nothing past `K`, R-tan inside it. (Over the ball the pivot stands at
/// the second torus's centre, and the tilt moves the section only at
/// second order, so the closed form here stands within the offset of the
/// coaxial reading; the sum is what is read.)
#[test]
fn a_split_pose_refuses_on_the_offset() {
    for split in [0.9, 0.75] {
        for m in [0.5, -0.5, k() + 1.0, -(k() + 1.0), 20.0, -20.0] {
            let (s, _) = tori(split, split, m, band());
            offset_row(&s);
        }
    }
}

/// **Inside: coaxial, its rows across.** At `0.4 + 0.4` the pose reads
/// coaxial with a slack of `0.6`: a row `K + 1` bands past the flip is
/// served, and agrees with the closed form; `K + 0.55` straddles on the
/// floor (half the swing would serve it); `0.3` is a tangency on the far
/// row.
#[test]
fn inside_the_band_the_coaxial_reading_serves_across() {
    let row = "section_torus_coaxial_tube_reach";
    for side in [1.0, -1.0] {
        let (s, (lo, hi)) = tori(0.4, 0.4, side * (k() + 1.0), band());
        let parallels = (
            2,
            false,
            vec![true, true],
            vec![true, true],
            vec![false, false],
        );
        if side > 0.0 {
            assert!(lo > 0.0, "the closed form crosses every meridian");
            assert_eq!(shape(&s), parallels, "{row}: two parallels");
        } else {
            assert!(hi < 0.0, "the closed form crosses none");
            assert_eq!(shape(&s).0, 0, "{row}: apart");
        }
        let (s, _) = tori(0.4, 0.4, side * (k() + 0.55), band());
        assert_eq!(tangent(&s), format!("{row}_floor"), "straddles");
        let (s, _) = tori(0.4, 0.4, side * 0.3, band());
        assert_eq!(tangent(&s), row, "tangent");
    }
}

/// **Narrow: the offset's floor straddles.** At `K = 1.2` a row `1.25`
/// bands past the flip is definite; at `0.95 + 0.95` the offset's far row
/// reads `1.425` bands, definite, and its floor `0.475`, a straddle, so
/// the pose serves neither the coaxial nor the parallel reading.
#[test]
fn a_narrow_band_straddles_on_the_offset_floor() {
    let z = zero();
    let narrow = Band::new(z, 1.2 * z).expect("a well-ordered band");
    for m in [1.25, -1.25] {
        let (s, _) = tori(0.95, 0.95, m, narrow);
        assert_eq!(
            tangent(&s),
            "section_axes_offset_floor",
            "m = {m}: a straddle"
        );
    }
}

/// The parallel arms' rows, each `K + past` bands from the flip beside
/// a swing of `0.45` (the tilt reading `0.9` at its lever, which is
/// twice the swing's), its far row named.
fn parallel_rows(past: f64) -> [(&'static str, Section<f64>); 2] {
    let z = zero();
    let gap = (k() + past) * z;
    let swing = 0.45 * z;
    let one = |at: Point3<f64>| Reach {
        centre: at,
        radius: 1.0,
    };
    // A pin of radius 0.2 clear of the donut's outer equator by `gap`,
    // its tilt levered over the donut's ball from the pivot `(e, 0, 0)`.
    let e = 2.7 + gap;
    let pin = cylinder(p(e, 0.0, 0.0), tilted(swing / (e + BALL)), 0.2);
    let torus_pin = super::section_cert_rows::classify(&torus(p(0.0, 0.0, 0.0), 2.0, 0.5), &pin);
    // A wall of radius 0.2 standing `gap` off the cone's axis.
    let e = 0.2 + gap;
    let wall = cylinder(p(e, 0.0, 0.0), tilted(swing), 0.2);
    let cone_wall = classify(
        &unit_cone(),
        &wall,
        one(p(e, 0.0, 0.0)),
        None,
        Some(geom_brep::SsiLane::marched()),
        band(),
    );
    [
        ("section_torus_offset_wall_near_outer", torus_pin),
        ("section_cone_cylinder_apex", cone_wall),
    ]
}

/// **The parallel rows read on the floor.** `K + 0.35` bands past the
/// flip beside a swing of `0.45` straddles, where main served the side
/// (and half the swing would); `K + 0.9` serves it.
#[test]
fn parallel_rows_serve_a_side_on_the_floor() {
    for (row, s) in parallel_rows(0.35) {
        assert_eq!(tangent(&s), format!("{row}_floor"), "{row}: straddles");
    }
    for (row, s) in parallel_rows(0.9) {
        match (row, &s) {
            ("section_torus_offset_wall_near_outer", Section::Components { parts, .. }) => {
                assert!(parts.is_empty(), "{row}: the pin is clear of the tube")
            }
            ("section_cone_cylinder_apex", Section::Components { parts, .. }) => assert!(
                parts.len() == 2 && parts.iter().all(|c| !c.essential_f && c.essential_g),
                "{row}: one loop per nappe, the wall clear of the axis: {parts:?}"
            ),
            _ => panic!("{row}: served, got {s:?}"),
        }
    }
}

/// **Wide walls tilted in the band read crossing over a region on the
/// wall.** Two walls of radius `r` on one axis, `gap` bands apart in
/// radius, the second tilted `0.45` bands per metre, read over a unit
/// region on the wall `r` from the axis. The pair's pose levers the tilt
/// from the axis's foot of the region's centre out across the region,
/// `r + 1`, so it reads the axes crossing. The two walls meet tens to
/// hundreds of metres along the axis, where the gap closes against the
/// tilt, so the region holds no component. At `r = 1000` the ruling sweep
/// decides that: no ruling near the region meets the second wall. At
/// `r = 100` the rulings near the region do meet it, far off, and their
/// meetings, levered by `1/sin` of the tilt, straddle the region's sphere
/// at every cell, so the sweep escalates (R-tan, on `ruling_cut`). The
/// turn slides the walls
/// along themselves, which that lever does not credit
/// (`sectarms/the-cylinder-pose-levers-a-tilt-from-the-axis-foot-of-the-region`).
#[test]
fn wide_walls_tilted_in_the_band_read_crossing_over_a_region_on_the_wall() {
    let z = zero();
    for (r, gap, decided) in [
        (100.0, 20.0, false),
        (100.0, 40.0, false),
        (1000.0, 200.0, true),
    ] {
        let s = classify(
            &cylinder(p(0.0, 0.0, 0.0), Vec3::unit_z(), r),
            &cylinder(p(0.0, 0.0, 0.0), tilted(0.45 * z), r + gap * z),
            Reach {
                centre: p(r, 0.0, 0.0),
                radius: 1.0,
            },
            Some(geom_brep::ExtentBall::new(p(r, 0.0, 0.0), 1.0)),
            Some(geom_brep::SsiLane::marched()),
            band(),
        );
        let pinned = if decided {
            matches!(&s, Section::Components { parts, .. } if parts.is_empty())
        } else {
            matches!(&s, Section::Tangent(name) if name.starts_with("ruling_cut"))
        };
        assert!(pinned, "r = {r}, gap = {gap}: {s:?}");
    }
}

/// The reach about the pivot whose radius `L` makes the turn of a point
/// half a metre along the axis from it, `0.5·sin²`, read `TURN` bands with
/// the tilt at `TILT` of the band at its lever `2L`; the swing is `TILT/2`.
const TILT: f64 = 0.85;
const TURN: f64 = 0.9;
fn turn_reach(at: Point3<f64>) -> (Reach<f64>, f64) {
    let z = zero();
    let radius = (0.5 * (TILT / 2.0).powi(2) * z / TURN).sqrt();
    (Reach { centre: at, radius }, TILT * z / (2.0 * radius))
}

/// **The turn of a point read off the pivot counts.** A cone whose apex
/// is the pivot, on the torus axis, against a torus whose centre stands
/// `0.5` m up the axis from it. (A torus pair reads its turn over a
/// torus's ball, where `sin²` of a tilt in the band times a station
/// inside the ball is far below it.) It has no offset, and
/// the swing is `0.425` bands; the arm's coaxial reading reads the height
/// at the centre or the apex, which the turn moves by `0.9` bands more,
/// so each refuses on the offset; so does a second cone whose apex stands
/// `0.5` m off the pivot along its axis. The same pose with the read point
/// at the pivot is served: two parallels apart from the cone, the coaxial
/// cones' two parallels.
#[test]
fn the_turn_of_a_point_read_off_the_pivot_counts() {
    let at = p(0.0, 0.0, 0.0);
    let (reach, tilt) = turn_reach(at);
    let d = tilted(tilt);
    let cone = Surface::Cone {
        apex: at,
        axis: d,
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref: Vec3::unit_y(),
    };
    for (rise, refuses) in [(0.5, true), (0.0, false)] {
        let s = classify(
            &torus(p(0.0, 0.0, rise), 2.0, 0.5),
            &cone,
            reach,
            None,
            Some(geom_brep::SsiLane::marched()),
            band(),
        );
        if refuses {
            offset_row(&s);
        } else {
            assert_eq!(
                shape(&s).0,
                0,
                "cone at the pivot's height: apart, got {s:?}"
            );
        }
        // A second cone whose apex stands `0.5` m along its own axis from
        // the pivot, against a cone whose apex is a metre down the first
        // axis: refused coaxial, the pair goes to the ruling charts.
        let apexes = Surface::Cone {
            apex: at + d * rise,
            axis: d,
            half_angle: core::f64::consts::FRAC_PI_4,
            u_ref: Vec3::unit_y(),
        };
        let first = Surface::Cone {
            apex: p(0.0, 0.0, -1.0),
            axis: Vec3::unit_z(),
            half_angle: core::f64::consts::FRAC_PI_4,
            u_ref: Vec3::unit_x(),
        };
        let s = classify(
            &first,
            &apexes,
            reach,
            None,
            Some(geom_brep::SsiLane::marched()),
            band(),
        );
        let coaxial = matches!(
            &s,
            Section::Components { parts, .. } if parts.len() == 2
                && parts.iter().all(|c| c.essential_f && c.essential_g && c.witness.is_none())
        );
        assert_eq!(coaxial, !refuses, "cones, apex {rise} off the pivot: {s:?}");
    }
}
