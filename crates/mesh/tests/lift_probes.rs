//! **What a loop's lift reads**, measured through the public door
//! (`topo::Body::loop_lift`, the one `mesh::trimmed::trim_polygon`
//! reads) on every loop of every curved face of the reach probes'
//! bodies (`reach_probes.rs`), of this crate's common fixtures
//! (`common/`), and of seamless bands built through the Euler doors.
//!
//! A lift is sampled as a chain: each row's image (`LiftedRow::pcurve`)
//! evaluated over its carrier interval in the direction the loop walks
//! it. A JOINT of the chain is the step from one row's last point to the
//! next row's first; the CLOSURE joint is the step from the last row
//! back to the first.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, SQRT_2, TAU};
use std::fmt::Write as _;

use geom::{Curve3, Surface, SurfaceKind};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, Pcurve, SurfacePair};
use geom_core::{Affine3, Point2, Point3, Vec3};
use profile::test_support::bulge_loop;
use sweep::test_support::{ball_poled_z, brick, finished, revolved_about_y, stacked_at};
use sweep::{Revolution, Section, loft_body};
use topo::construction::{Construct, FaceSurface, MefSite, MevSite, Rechart};
use topo::{AtRestBody, Body, Deck, FaceKey, LoopKey};

use crate::common;
use crate::common::witness_bodies::{
    apex_crossing_bowtie, keyway, oblique_lens, one_circle_cut, pole_crossing_half_cap, slit,
};
use crate::reach_probes::{
    Run, kind_of, klein_elbow, lofted_blade, posed, revolved, rod, six, tol, vessel_quarter,
    widening,
};

/// Samples per row of a chain.
const SAMPLES: usize = 32;
/// Two chart points closer than this, per channel, are one.
const MEET: f64 = 1e-7;

// ------------------------------------------------------------ the sample

type Named = (String, Body<f64>);

fn six_bodies(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Vec<Named> {
    let runs: [Run; 6] = six(a, b);
    runs.into_iter()
        .map(|(op, run)| {
            let label = format!("{what}, {op}");
            let built = run.unwrap_or_else(|e| panic!("{label}: the op builds, got {e:?}"));
            let body = built
                .body()
                .unwrap_or_else(|| panic!("{label}: a body, not the empty result"));
            (label, Body::clone(&body.body))
        })
        .collect()
}

/// The hollowed partial revolves of the reach probes' spiric row.
fn spiric_hollows() -> Vec<Named> {
    [
        ("the hollowed vessel quarter", vessel_quarter(), 1.0 / 128.0),
        ("the hollowed klein elbow", klein_elbow(0.275), 0.02),
    ]
    .into_iter()
    .map(|(what, operand, t)| {
        let hollow = topo::shell(&finished("the operand", operand, tol()), t, tol())
            .unwrap_or_else(|e| panic!("{what} hollows, got {e:?}"))
            .body;
        (what.to_owned(), Body::clone(&hollow))
    })
    .collect()
}

/// The reach probes' cone rows: both frusta split by the tilted plane,
/// and the widening frustum under C1's brick.
fn cone_sections() -> Vec<Named> {
    let plane = topo::test_support::split_plane(
        Point3::new(0.0, 0.5, 0.0),
        Vec3::new(0.3f64.sin(), 0.3f64.cos(), 0.0),
        tol(),
    );
    let narrowing = revolved(&[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)]);
    let mut out = Vec::new();
    for (name, frustum) in [("narrowing", narrowing), ("widening", widening())] {
        let split = topo::splitting::split(&finished("the frustum", frustum, tol()), &plane, tol())
            .unwrap_or_else(|e| panic!("the {name} frustum splits, got {e:?}"));
        for (side, part) in [("above", &split.above), ("below", &split.below)] {
            let what = format!("the {name} frustum, {side}");
            let half = part.body().unwrap_or_else(|| panic!("{what}: a body"));
            out.push((what, Body::clone(half)));
        }
    }
    let turn = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        10f64.to_radians(),
    );
    let c1_brick = posed(
        "C1's brick",
        brick((-3.0, 3.0), (-6.0, 0.0), (-3.0, 3.0), tol()),
        Affine3::translation(Point3::new(0.0, 0.5, 0.0) - Point3::origin()) * turn,
    );
    out.extend(six_bodies(
        "C1",
        &finished("the frustum", widening(), tol()),
        &c1_brick,
    ));
    out
}

/// The reach probes' donut and bar B2 (A the donut).
fn donut_and_bar() -> Vec<Named> {
    let bar = brick::<f64>((-0.05, 0.05), (0.25, 0.35), (1.0, 3.0), tol());
    six_bodies(
        "the donut and B2",
        &finished("the torus", common::donut(), tol()),
        &finished("B2", bar, tol()),
    )
}

/// The reach probes' tilted tube on the `z`-poled ball of radius `√2`
/// (A the tube).
fn tube_on_ball() -> Vec<Named> {
    let tube = posed(
        "the tube",
        rod(1.0, 1.0, 2.0),
        Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), 1.2),
    );
    let ball = finished("the ball", ball_poled_z(SQRT_2, Vec3::zero(), tol()), tol());
    six_bodies("the tube on the ball", &tube, &ball)
}

/// The reach probes' pipe and its four bars (A the pipe).
fn pipe_and_bars() -> Vec<Named> {
    let pipe = finished("the pipe", rod(1.0, -2.0, 4.0), tol());
    let mut out = Vec::new();
    for (what, x, y, z) in [
        ("the bar through", (-1.1, 1.1), (-0.3, 0.3), (-0.3, 0.3)),
        ("the bar half in", (0.5, 1.1), (-0.3, 0.3), (-0.3, 0.3)),
        ("the bar off the axis", (-1.1, 1.1), (0.15, 0.7), (-0.4, 0.1)),
    ] {
        let bar = finished("the bar", brick::<f64>(x, y, z, tol()), tol());
        out.extend(six_bodies(what, &pipe, &bar));
    }
    let turned = posed(
        "the turned bar",
        brick((-1.1, 1.1), (0.2, 0.7), (-0.15, 0.15), tol()),
        Affine3::rotation_about_axis(Point3::new(0.0, 0.45, 0.0), Vec3::new(1.0, 0.0, 0.0), 0.3),
    );
    out.extend(six_bodies("the bar turned 0.3", &pipe, &turned));
    out
}

/// The reach probes' T1 cone, apex and all, turned a quarter about `y`.
fn t1_cone() -> AtRestBody<f64> {
    let height = 1.2 / FRAC_PI_6.tan();
    posed(
        "the cone",
        revolved(&[(0.0, 0.0), (1.2, 0.0), (0.0, height)]),
        Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), FRAC_PI_2),
    )
}

/// The reach probes' T1 cone and box (A the cone).
fn cone_and_box() -> Vec<Named> {
    let height = 1.2 / FRAC_PI_6.tan();
    let edge = Point3::new(0.5 * 0.6, height - 1.5 * 0.6, 0.0);
    let turn = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        -50f64.to_radians(),
    );
    let wedge = posed(
        "the box",
        brick((0.0, 6.0), (0.0, 6.0), (-3.0, 3.0), tol()),
        Affine3::translation(edge - Point3::origin()) * turn,
    );
    six_bodies("T1", &t1_cone(), &wedge)
}

/// The reach probes' ball across a cylinder's wall: a B-spline section.
fn ball_across_a_wall() -> Vec<Named> {
    let cylinder = finished("the cylinder", rod(0.5, -1.0, 2.0), tol());
    let r = 0.3;
    let ball = posed(
        "the ball",
        revolved_about_y(
            vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
            Revolution::Full,
            tol(),
        ),
        Affine3::translation(Vec3::new(0.5, 0.0, 0.0)),
    );
    [
        ("cylinder ∩ ball", topo::intersect(&cylinder, &ball, tol())),
        ("cylinder ∪ ball", topo::union(&cylinder, &ball, tol())),
    ]
    .into_iter()
    .map(|(op, run)| {
        let built = run.unwrap_or_else(|e| panic!("{op}: the op builds, got {e:?}"));
        let body = &built.body().unwrap_or_else(|| panic!("{op}: a body")).body;
        (op.to_owned(), Body::clone(body))
    })
    .collect()
}

/// The reach probes' lofts: one creased wall and the twisted blade.
fn lofts() -> Vec<Named> {
    let corners = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let section = |b: f64| -> Section {
        vec![bulge_loop(
            (0..4)
                .map(|i| {
                    let (x, y) = corners[i];
                    (Point2::new(x, y), if i == 1 { b } else { 0.0 })
                })
                .collect(),
        )]
    };
    let sections: Vec<Section> = [0.0, 1.0, 0.0].into_iter().map(section).collect();
    let creased = loft_body::<f64>(&sections, &stacked_at(&[0.0, 1.0, 2.0]), 1, tol())
        .unwrap_or_else(|e| {
            panic!(
                "the creased loft: {}",
                e.render(geom_brep::recourse::Reading::Build)
            )
        })
        .body
        .into_body();
    vec![
        ("the creased loft [0, 1, 0]".to_owned(), creased),
        (
            "the twisted blade".to_owned(),
            lofted_blade(&[0.0, 1.0, 2.0, 3.0], 0.4, 3),
        ),
    ]
}

fn z_axis() -> Vec3<f64> {
    Vec3::new(0.0, 0.0, 1.0)
}

fn x_axis() -> Vec3<f64> {
    Vec3::new(1.0, 0.0, 0.0)
}

/// The rim about `+z` at height `z`, radius `r`.
fn rim(z: f64, r: f64) -> Curve3<f64> {
    Curve3::circle(Point3::new(0.0, 0.0, z), z_axis(), r, x_axis())
}

/// The swept fixtures of `common/`.
fn common_swept() -> Vec<Named> {
    let named = |name: &str, body: Body<f64>| (name.to_owned(), body);
    vec![
        named("common::l_prism", common::l_prism()),
        named("common::holed_prism", common::holed_prism()),
        named("common::rounded_prism", common::rounded_prism()),
        named("common::ball", common::ball()),
        named("common::cone", common::cone()),
        named("common::cone_wedge(1, 1)", common::cone_wedge(1.0, 1.0)),
        named("common::sphere_wedge(1)", common::sphere_wedge(1.0)),
        named("common::washer", common::washer()),
        named("common::donut", common::donut()),
        named("common::wedge", common::wedge()),
        named("common::axis_wedge", common::axis_wedge()),
    ]
}

/// The witnesses `common::witness_bodies` assembles through the Euler
/// doors and leaves unminted.
fn unminted_witnesses() -> Vec<Named> {
    let named = |name: &str, body: Body<f64>| (name.to_owned(), body);
    vec![
        named("keyway", keyway().0),
        named("oblique_lens", oblique_lens().0),
        named("slit", slit().0),
        named("pole_crossing_half_cap", pole_crossing_half_cap().0),
        named("apex_crossing_bowtie", apex_crossing_bowtie().0),
    ]
}

/// `common::witness_bodies::one_circle_cut` on each chart: a surface cut
/// along one whole circle about its axis, which the door mints.
fn one_circle_cuts() -> Vec<Named> {
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: z_axis(),
        u_ref: x_axis(),
    };
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: z_axis(),
        half_angle: FRAC_PI_4,
        u_ref: x_axis(),
    };
    let cylinder = Surface::Cylinder {
        origin: Point3::origin(),
        axis: z_axis(),
        radius: 1.0,
        u_ref: x_axis(),
    };
    let torus = Surface::Torus {
        center: Point3::origin(),
        axis: z_axis(),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: x_axis(),
    };
    let up = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 1.0),
        normal: z_axis(),
        u_ref: x_axis(),
    };
    let half = (1.0f64 - 0.25).sqrt();
    let named = |name: &str, body: Body<f64>| (name.to_owned(), body);
    vec![
        named(
            "two sphere caps on one rim",
            one_circle_cut(&rim(0.5, half), sphere, None),
        ),
        named(
            "a cone apex cap and its disc",
            one_circle_cut(&rim(1.0, 1.0), cone, Some(up.clone())),
        ),
        named(
            "a one-rim cylinder face and its disc",
            one_circle_cut(&rim(1.0, 1.0), cylinder, Some(up)),
        ),
        named(
            "a torus cut on its outer equator",
            one_circle_cut(&rim(0.0, 2.5), torus, None),
        ),
    ]
}

// @@TAIL@@
