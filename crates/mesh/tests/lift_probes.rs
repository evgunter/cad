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
//!
//! What is pinned, as measured:
//!
//! - every loop of a curved face a public door built has a lift, and it
//!   closes in the chart: it returns to its start, along the pole or
//!   apex row where it passes one;
//! - a loop ends a whole `u` period from its start where it is a whole
//!   turn about the axis with no seam (a rim-only face, both loops of a
//!   seamless band), where it holds a pole inside it, and where it runs
//!   through a cone's apex mid-edge; a loop through a sphere's pole
//!   mid-edge ends on the involution of its start. Of those, only the
//!   seamless cylinder and cone bands finish;
//! - a ring's lift starts on its own first image: 23 of the 24 rings
//!   measured sit inside their outer loop's polygon as stored, and one,
//!   a whole projected circle on a ball's half band, sits one `u` period
//!   below it. `topo::Body::face_lift` places every one inside, the
//!   displaced one moved that period, and answers that a seamless band's
//!   outer loop does not close;
//! - the spiric-bounded torus wall's outer loop is a simple polygon,
//!   wound with its face;
//! - at a cone's apex the lift runs along the row `v = 0` through the
//!   face's turn, as it does along a sphere's pole row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, PI, SQRT_2, TAU};
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
        (
            "the bar off the axis",
            (-1.1, 1.1),
            (0.15, 0.7),
            (-0.4, 0.1),
        ),
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

/// A whole-turn rim on the wall `surface`, described by its chart until
/// the cap it bounds exists.
fn whole_turn(surface: topo::SurfaceKey, carrier: Curve3<f64>) -> EdgeCurveSpec<f64> {
    EdgeCurveSpec {
        description: EdgeDescriptionSpec::Chart {
            surface,
            image: None,
            wrap: false,
            declared: None,
        },
        carrier,
        param_start: 0.0,
        param_end: TAU,
    }
}

/// **A seamless band**: one face of `surface` between the whole-turn
/// rims `bottom` and `top`, capped by the planes `caps`, with no seam,
/// so each of its two loops winds the axis once. Through the Euler doors,
/// by the operator sequence of `topo`'s
/// `a_seamless_pipe_cut_by_an_oblique_plane_measures`: `strut` is the
/// scaffolding edge between the rims' start points (a chord line where
/// `None`), killed by `kemr`.
fn seamless_band(
    surface: Surface<f64>,
    bottom: Curve3<f64>,
    top: Curve3<f64>,
    caps: [Surface<f64>; 2],
    strut: Option<EdgeCurveSpec<f64>>,
) -> Body<f64> {
    let tol = tol();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(bottom.eval(0.0), true).unwrap();
    let face = FaceSurface::New {
        surface,
        sense: true,
    };
    let wall = body.set_face_surface(seed.face, face).unwrap();
    let lone = MefSite::Lone {
        r#loop: seed.r#loop,
    };
    let cap_b = body
        .mef(
            lone,
            whole_turn(wall, bottom.clone()),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    let site = MevSite::Fan {
        he1: cap_b.he_plus,
        he2: cap_b.he_plus,
    };
    let strut = match strut {
        None => body.mev_line(site, top.eval(0.0), tol).unwrap(),
        Some(spec) => body.mev(site, top.eval(0.0), spec, tol).unwrap(),
    };
    let chords = MefSite::Chords {
        he1: strut.he_minus,
        he2: strut.he_minus,
    };
    let cap_t = body
        .mef(
            chords,
            whole_turn(wall, top.clone()),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    let [plane_b, plane_t] = caps;
    body.set_face_surfaces_describing(
        vec![
            Rechart::new(plane_b, cap_b.face, true),
            Rechart::new(plane_t, cap_t.face, true),
        ],
        &[],
        tol,
    )
    .unwrap();
    for (cap, carrier) in [(&cap_b, bottom), (&cap_t, top)] {
        let plane = body.get_face(cap.face).unwrap().surface;
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(wall, plane),
                selector: geom_brep::CurveSelector::Witness(carrier.eval(PI)),
            },
            carrier,
            param_start: 0.0,
            param_end: TAU,
        };
        body.set_edge_curve(cap.edge, spec, tol).unwrap();
    }
    topo::mint_pcurves(&mut body, tol).unwrap();
    body
}

/// The plane `z = z0` whose normal is `±z`.
fn level(z0: f64, up: bool) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, z0),
        normal: if up { z_axis() } else { -z_axis() },
        u_ref: x_axis(),
    }
}

/// The rim about `−z` at height `z`, radius `r`: a band's top rim.
fn rim_down(z: f64, r: f64) -> Curve3<f64> {
    Curve3::circle(Point3::new(0.0, 0.0, z), -z_axis(), r, x_axis())
}

/// The seamless pipe of `topo`'s row: radius 1, floor at `z = 0`, the
/// roof 2 above it on the axis and tilted 0.4 about `y`, an ellipse.
fn seamless_pipe() -> Body<f64> {
    let (r, h, theta) = (1.0, 2.0, 0.4f64);
    let (s, c) = theta.sin_cos();
    let n = Vec3::new(s, 0.0, c);
    let major = Vec3::new(c, 0.0, -s);
    let roof = Point3::new(0.0, 0.0, h);
    let ellipse = Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(roof, -n, major),
        r / c,
        r,
    ));
    let cylinder = Surface::Cylinder {
        origin: Point3::origin(),
        axis: z_axis(),
        radius: r,
        u_ref: x_axis(),
    };
    let tilted = Surface::Plane {
        origin: roof,
        normal: n,
        u_ref: major,
    };
    seamless_band(
        cylinder,
        rim(0.0, r),
        ellipse,
        [level(0.0, false), tilted],
        None,
    )
}

/// A seamless frustum: the cone of half-angle `π/4` between `z = 1` and
/// `z = 2`.
fn seamless_frustum() -> Body<f64> {
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: z_axis(),
        half_angle: FRAC_PI_4,
        u_ref: x_axis(),
    };
    let caps = [level(1.0, false), level(2.0, true)];
    seamless_band(cone, rim(1.0, 1.0), rim_down(2.0, 2.0), caps, None)
}

/// A seamless torus band: the outer quarter of the tube of the torus
/// `(2, 0.5)`, between its minor angles `∓π/4`.
fn seamless_torus_band() -> Body<f64> {
    let (major, minor) = (2.0, 0.5);
    let torus = Surface::Torus {
        center: Point3::origin(),
        axis: z_axis(),
        major_radius: major,
        minor_radius: minor,
        u_ref: x_axis(),
    };
    let (z, r) = (minor * FRAC_PI_4.sin(), major + minor * FRAC_PI_4.cos());
    let meridian = Curve3::circle(
        Point3::new(major, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        minor,
        x_axis(),
    );
    let strut = EdgeCurveSpec::arc_of_circle(meridian, -FRAC_PI_4, FRAC_PI_4).unwrap();
    let caps = [level(-z, false), level(z, true)];
    seamless_band(torus, rim(-z, r), rim_down(z, r), caps, Some(strut))
}

// ------------------------------------------------------------ the reading

/// A step of a chain from one point to the next.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Joint {
    /// The two points are one.
    Meets,
    /// Both are on one singular row of the chart (a sphere's pole row, a
    /// cone's apex row), this far apart in `u`.
    Singular(f64),
    /// They differ by whole periods `(u, v)`, `τ` each.
    Periods(i32, i32),
    /// Anything else: the step, `(Δu, Δv)`.
    Open(f64, f64),
}

/// Whether `v` is on a singular row of a `chart` chart.
fn on_singular_row(chart: SurfaceKind, v: f64) -> bool {
    match chart {
        SurfaceKind::Sphere => v.cos().abs() < MEET,
        SurfaceKind::Cone => v.abs() < MEET,
        _ => false,
    }
}

fn joint(chart: SurfaceKind, a: (f64, f64), b: (f64, f64)) -> Joint {
    let (du, dv) = (b.0 - a.0, b.1 - a.1);
    if du.abs() < MEET && dv.abs() < MEET {
        return Joint::Meets;
    }
    if dv.abs() < MEET && on_singular_row(chart, a.1) && on_singular_row(chart, b.1) {
        return Joint::Singular(du);
    }
    let (ku, kv) = ((du / TAU).round(), (dv / TAU).round());
    let v_closed = matches!(chart, SurfaceKind::Sphere | SurfaceKind::Torus);
    let u_closed = chart != SurfaceKind::Nurbs && chart != SurfaceKind::Plane;
    if u_closed
        && (du - ku * TAU).abs() < MEET
        && (dv - kv * TAU).abs() < MEET
        && (v_closed || kv == 0.0)
    {
        #[allow(clippy::cast_possible_truncation)]
        return Joint::Periods(ku as i32, kv as i32);
    }
    Joint::Open(du, dv)
}

fn form(pcurve: &Pcurve<f64>) -> &'static str {
    match pcurve {
        Pcurve::Harmonic { .. } => "Harmonic",
        Pcurve::Spiric { .. } => "Spiric",
        Pcurve::FocalSection(_) => "FocalSection",
        Pcurve::Projected(_) => "Projected",
        Pcurve::IsoLine { .. } => "IsoLine",
        Pcurve::IsoArc { .. } => "IsoArc",
        Pcurve::Fitted(_) => "Fitted",
        Pcurve::General(_) => "General",
    }
}

/// A loop's lift, sampled.
struct Chain {
    /// Each row's image form.
    forms: Vec<&'static str>,
    /// Each row's deck.
    decks: Vec<Deck>,
    /// `Lift::winding`.
    winding: Deck,
    /// `Lift::closes_in_chart`.
    closes: bool,
    /// Whether `Lift::window` answers.
    window: bool,
    /// Each row's image at `samples + 1` parameters, in walk order.
    rows: Vec<Vec<(f64, f64)>>,
}

impl Chain {
    /// The lift of `lk`, or its gap: the carrier of the half-edge the
    /// gap names, and whether that half-edge stores an image and a joint.
    fn read(body: &Body<f64>, lk: LoopKey, samples: usize) -> Result<Self, String> {
        let lift = body.loop_lift(lk).map_err(|gap| {
            let he = gap.half_edge;
            let edge = body.get_half_edge(he).expect("the gap's half-edge").edge;
            let carrier = body
                .get_curve_geom(body.get_edge(edge).expect("its edge").curve)
                .and_then(|g| g.certified())
                .map(|c| c.carrier().kind());
            format!(
                "LiftGap on a {carrier:?} edge: image stored {}, joint stored {}",
                body.pcurve(he).is_some(),
                body.joint(he).is_some()
            )
        })?;
        Ok(Self::of(&lift, samples))
    }

    /// `lift`, sampled.
    fn of(lift: &topo::pcurves::Lift<f64>, samples: usize) -> Self {
        let rows = lift
            .rows()
            .iter()
            .map(|row| {
                let (enter, exit) = row.entry_exit();
                (0..=samples)
                    .map(|k| {
                        #[allow(clippy::cast_precision_loss)]
                        let s = k as f64 / samples as f64;
                        let uv = row.pcurve.eval(enter + (exit - enter) * s);
                        (uv.x, uv.y)
                    })
                    .collect()
            })
            .collect();
        Self {
            forms: lift.rows().iter().map(|row| form(&row.pcurve)).collect(),
            decks: lift.rows().iter().map(|row| row.lift).collect(),
            winding: lift.winding().expect("a whole loop has a winding").deck,
            closes: lift.closes_in_chart(),
            window: lift.window().is_some(),
            rows,
        }
    }

    /// The joints between consecutive rows, the closure joint left out.
    fn inner_joints(&self, chart: SurfaceKind) -> Vec<Joint> {
        self.rows
            .windows(2)
            .map(|w| joint(chart, w[0][w[0].len() - 1], w[1][0]))
            .collect()
    }

    /// The closure joint: from the chain's end back to its start.
    fn closure(&self, chart: SurfaceKind) -> Joint {
        let last = &self.rows[self.rows.len() - 1];
        joint(chart, last[last.len() - 1], self.rows[0][0])
    }

    /// The hull of the samples, `((u_min, u_max), (v_min, v_max))`.
    fn hull(&self) -> ((f64, f64), (f64, f64)) {
        let fold = |pick: fn(&(f64, f64)) -> f64| {
            self.rows
                .iter()
                .flatten()
                .map(pick)
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), x| {
                    (lo.min(x), hi.max(x))
                })
        };
        (fold(|p| p.0), fold(|p| p.1))
    }

    /// The chain as one closed polygon: every row's samples but its
    /// last, and the last too where the next row does not start on it.
    fn polygon(&self) -> Vec<(f64, f64)> {
        let n = self.rows.len();
        let mut out = Vec::new();
        for (i, row) in self.rows.iter().enumerate() {
            let last = row[row.len() - 1];
            let next = self.rows[(i + 1) % n][0];
            out.extend_from_slice(&row[..row.len() - 1]);
            if (last.0 - next.0).abs() >= MEET || (last.1 - next.1).abs() >= MEET {
                out.push(last);
            }
        }
        out
    }
}

/// Twice the signed area of a closed polygon.
fn shoelace(poly: &[(f64, f64)]) -> f64 {
    let n = poly.len();
    (0..n)
        .map(|i| {
            let (a, b) = (poly[i], poly[(i + 1) % n]);
            a.0 * b.1 - b.0 * a.1
        })
        .sum()
}

/// One loop of a curved face.
struct LoopFact {
    body: String,
    face: FaceKey,
    chart: SurfaceKind,
    sense: bool,
    /// `None` for the outer loop, `Some(i)` for ring `i`.
    ring: Option<usize>,
    /// Whether an edge of the face has both its half-edges on the face.
    seam: bool,
    chain: Result<Chain, String>,
    /// For a ring, what `Body::face_lift` answers for it: the move that
    /// places it and its moved lift, or the face's refusal.
    placed: Option<Result<(Deck, Chain), String>>,
}

impl LoopFact {
    fn role(&self) -> String {
        self.ring
            .map_or("outer".to_owned(), |i| format!("ring {i}"))
    }
}

/// Whether an edge of `fk` has both of its half-edges on loops of `fk`.
fn has_seam(body: &Body<f64>, fk: FaceKey) -> bool {
    let face = body.get_face(fk).expect("a live face");
    core::iter::once(face.outer)
        .chain(face.rings.iter().copied())
        .filter_map(|lk| match body.get_loop(lk).expect("a loop").boundary {
            topo::LoopBoundary::Cycle { first } => body.loop_cycle(first),
            topo::LoopBoundary::Empty { .. } => None,
        })
        .flatten()
        .any(|he| {
            let edge = body
                .get_edge(body.get_half_edge(he).expect("a half-edge").edge)
                .expect("an edge");
            [edge.he_plus, edge.he_minus].into_iter().all(|side| {
                let lk = body.get_half_edge(side).expect("a half-edge").parent_loop;
                body.get_loop(lk).expect("a loop").face == fk
            })
        })
}

/// Every loop of every non-planar face of `body`, outer loop first.
fn loops_of(name: &str, body: &Body<f64>, samples: usize) -> Vec<LoopFact> {
    let mut out = Vec::new();
    for (fk, face) in body.faces() {
        let chart = kind_of(body, fk);
        if chart == SurfaceKind::Plane {
            continue;
        }
        let seam = has_seam(body, fk);
        let band = geom_core::Band::linear(tol()).expect("the probes' band");
        let door = body.face_lift(fk, band).map_err(|e| format!("{e:?}"));
        let placed = |i: usize| {
            door.as_ref()
                .map(|f| {
                    let ring = &f.rings()[i];
                    (ring.by, Chain::of(&ring.lift, samples))
                })
                .map_err(Clone::clone)
        };
        let loops = core::iter::once((None, face.outer))
            .chain(face.rings.iter().enumerate().map(|(i, &lk)| (Some(i), lk)));
        for (ring, lk) in loops {
            out.push(LoopFact {
                body: name.to_owned(),
                face: fk,
                chart,
                sense: face.sense,
                ring,
                seam,
                chain: Chain::read(body, lk, samples),
                placed: ring.map(placed),
            });
        }
    }
    out
}

fn census(sample: &[Named], samples: usize) -> Vec<LoopFact> {
    sample
        .iter()
        .flat_map(|(name, body)| loops_of(name, body, samples))
        .collect()
}

fn pair(p: (f64, f64)) -> String {
    format!("({:+.4}, {:+.4})", p.0, p.1)
}

fn deck(d: Deck) -> String {
    format!("({},{}{})", d.u, d.v, if d.twin { ",σ" } else { "" })
}

/// One line per loop: what its lift reads.
fn table(facts: &[LoopFact]) -> String {
    let mut out = String::new();
    for f in facts {
        let _ = write!(
            out,
            "{} | {:?} {}{} | {} | ",
            f.body,
            f.chart,
            if f.sense { '+' } else { '-' },
            if f.seam { " seam" } else { "" },
            f.role()
        );
        match &f.chain {
            Err(gap) => {
                let _ = writeln!(out, "{gap}");
            }
            Ok(c) => {
                let (u, v) = c.hull();
                let decks: Vec<String> = c.decks.iter().map(|d| deck(*d)).collect();
                let _ = writeln!(
                    out,
                    "{:?} | winding {} closes {} | closure {:?} | inner {:?} | u {} v {} | \
                     2A {:+.4} | window {} | decks {}",
                    c.forms,
                    deck(c.winding),
                    c.closes,
                    c.closure(f.chart),
                    c.inner_joints(f.chart),
                    pair(u),
                    pair(v),
                    shoelace(&c.polygon()),
                    c.window,
                    decks.join(""),
                );
            }
        }
    }
    out
}

// ------------------------------------------------------------- placement

/// How many times `poly` winds about `p`.
fn winding_number(poly: &[(f64, f64)], p: (f64, f64)) -> i32 {
    let n = poly.len();
    let mut w = 0;
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        let side = (b.0 - a.0) * (p.1 - a.1) - (p.0 - a.0) * (b.1 - a.1);
        if a.1 <= p.1 {
            if b.1 > p.1 && side > 0.0 {
                w += 1;
            }
        } else if b.1 <= p.1 && side < 0.0 {
            w -= 1;
        }
    }
    w
}

/// The whole-period moves `(ku, kv)`, each within two periods, that put
/// every sample of `ring` inside the polygon `outer` draws. `kv` moves
/// only on a chart closed in `v`.
fn placements(chart: SurfaceKind, outer: &Chain, ring: &Chain) -> Vec<(i32, i32)> {
    let poly = outer.polygon();
    let kvs: &[i32] = if matches!(chart, SurfaceKind::Torus | SurfaceKind::Sphere) {
        &[-2, -1, 0, 1, 2]
    } else {
        &[0]
    };
    let mut out = Vec::new();
    for ku in -2..=2 {
        for &kv in kvs {
            let moved = |p: &(f64, f64)| (p.0 + f64::from(ku) * TAU, p.1 + f64::from(kv) * TAU);
            if ring
                .rows
                .iter()
                .flatten()
                .all(|p| winding_number(&poly, moved(p)) != 0)
            {
                out.push((ku, kv));
            }
        }
    }
    out
}

/// One ring against its face's outer loop.
struct RingFact {
    body: String,
    chart: SurfaceKind,
    sense: bool,
    ring: usize,
    outer_hull: ((f64, f64), (f64, f64)),
    ring_hull: ((f64, f64), (f64, f64)),
    /// [`placements`].
    places: Vec<(i32, i32)>,
    /// What `Body::face_lift` answers: the move that places the ring,
    /// and whether every sample of the moved ring lies inside the outer
    /// loop's polygon; or the face's refusal.
    door: Result<((i32, i32), bool), String>,
    /// Twice the ring's signed area.
    area2: f64,
    /// Whether a row of the ring's or of the outer loop's lift is moved
    /// by the sphere's involution.
    twin: bool,
    forms: Vec<&'static str>,
}

/// Every ring of `facts` whose lift and whose outer loop's lift read.
fn rings_of(facts: &[LoopFact]) -> Vec<RingFact> {
    let mut out = Vec::new();
    let mut outer: Option<&LoopFact> = None;
    for f in facts {
        let Some(ring) = f.ring else {
            outer = Some(f);
            continue;
        };
        let o = outer.expect("a ring follows its face's outer loop");
        assert!(
            o.face == f.face && o.body == f.body,
            "the ring's own outer loop"
        );
        let (Ok(oc), Ok(rc)) = (&o.chain, &f.chain) else {
            continue;
        };
        out.push(RingFact {
            body: f.body.clone(),
            chart: f.chart,
            sense: f.sense,
            ring,
            outer_hull: oc.hull(),
            ring_hull: rc.hull(),
            places: placements(f.chart, oc, rc),
            door: match f.placed.as_ref().expect("a ring's door answer") {
                Ok((by, moved)) => {
                    assert!(!by.twin, "a ring is moved by a translation");
                    let poly = oc.polygon();
                    let inside = moved
                        .rows
                        .iter()
                        .flatten()
                        .all(|p| winding_number(&poly, *p) != 0);
                    Ok(((by.u, by.v), inside))
                }
                Err(e) => Err(e.clone()),
            },
            area2: shoelace(&rc.polygon()),
            twin: rc.decks.iter().chain(&oc.decks).any(|d| d.twin),
            forms: rc.forms.clone(),
        });
    }
    out
}

fn ring_table(rings: &[RingFact]) -> String {
    let mut out = String::new();
    for r in rings {
        let _ = writeln!(
            out,
            "{} | {:?} {} | ring {} {:?} | outer u {} v {} | ring u {} v {} | places {:?} | \
             door {:?} | 2A {:+.4} | twin {}",
            r.body,
            r.chart,
            if r.sense { '+' } else { '-' },
            r.ring,
            r.forms,
            pair(r.outer_hull.0),
            pair(r.outer_hull.1),
            pair(r.ring_hull.0),
            pair(r.ring_hull.1),
            r.places,
            r.door,
            r.area2,
            r.twin
        );
    }
    out
}

// ------------------------------------------------------- simple polygons

fn orient(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
    (b.0 - a.0) * (c.1 - a.1) - (c.0 - a.0) * (b.1 - a.1)
}

/// The pairs of sides of `poly`, no two sharing a vertex, that cross
/// (`.0`) or touch (`.1`).
fn crossings(poly: &[(f64, f64)]) -> (usize, usize) {
    let n = poly.len();
    let (mut cross, mut touch) = (0, 0);
    for i in 0..n {
        for j in (i + 2)..n {
            if i == 0 && j == n - 1 {
                continue;
            }
            let (a, b) = (poly[i], poly[(i + 1) % n]);
            let (c, d) = (poly[j], poly[(j + 1) % n]);
            let (o1, o2) = (orient(a, b, c), orient(a, b, d));
            let (o3, o4) = (orient(c, d, a), orient(c, d, b));
            if o1 * o2 < 0.0 && o3 * o4 < 0.0 {
                cross += 1;
            } else if o1 * o2 <= 0.0 && o3 * o4 <= 0.0 {
                // A zero product with the other pair straddling, or two
                // collinear sides: touching only where their boxes meet.
                let overlap = |p: fn(&(f64, f64)) -> f64| {
                    p(&a).min(p(&b)) <= p(&c).max(p(&d)) && p(&c).min(p(&d)) <= p(&a).max(p(&b))
                };
                if overlap(|p| p.0) && overlap(|p| p.1) {
                    touch += 1;
                }
            }
        }
    }
    (cross, touch)
}

// ------------------------------------------------------ singular joints

/// A joint of a chain on its chart's singular row: the last two samples
/// of the arriving row and the first two of the leaving one.
struct Turn {
    body: String,
    chart: SurfaceKind,
    sense: bool,
    arriving: [(f64, f64); 2],
    leaving: [(f64, f64); 2],
}

impl Turn {
    /// The step in `u` along the singular row.
    fn turn(&self) -> f64 {
        self.leaving[0].0 - self.arriving[1].0
    }
}

fn turns_of(facts: &[LoopFact]) -> Vec<Turn> {
    let mut out = Vec::new();
    for f in facts {
        let Ok(c) = &f.chain else { continue };
        let n = c.rows.len();
        for i in 0..n {
            let (a, b) = (&c.rows[i], &c.rows[(i + 1) % n]);
            let (exit, entry) = (a[a.len() - 1], b[0]);
            if matches!(joint(f.chart, exit, entry), Joint::Singular(_)) {
                out.push(Turn {
                    body: format!("{} ({})", f.body, f.role()),
                    chart: f.chart,
                    sense: f.sense,
                    arriving: [a[a.len() - 2], exit],
                    leaving: [entry, b[1]],
                });
            }
        }
    }
    out
}

fn turn_table(turns: &[Turn]) -> String {
    let mut out = String::new();
    for t in turns {
        let p = |q: (f64, f64)| format!("({:+.6}, {:+.6e})", q.0, q.1);
        let _ = writeln!(
            out,
            "{} | {:?} {} | arrives {} -> {} | leaves {} -> {} | turn {:+.6}",
            t.body,
            t.chart,
            if t.sense { '+' } else { '-' },
            p(t.arriving[0]),
            p(t.arriving[1]),
            p(t.leaving[0]),
            p(t.leaving[1]),
            t.turn(),
        );
    }
    out
}

// ---------------------------------------------------------------- probes

/// What a census of closing loops must read, as the lines that do not:
/// every loop has a lift; its winding is the identity and
/// `closes_in_chart` says so; every joint, the closure joint too, meets
/// or runs along a singular row, and only a sphere's or a cone's does
/// the latter; its polygon is simple, counter-clockwise on an outer loop
/// exactly where the face's `sense` is true and the other way on a ring;
/// and `Lift::window` answers exactly where every row is `Harmonic` or
/// `FocalSection`.
fn not_closing(facts: &[LoopFact]) -> Vec<String> {
    let mut out = Vec::new();
    for f in facts {
        let mut fail =
            |what: &str| out.push(format!("{} {:?} {}: {what}", f.body, f.chart, f.role()));
        let c = match &f.chain {
            Ok(c) => c,
            Err(gap) => {
                fail(gap);
                continue;
            }
        };
        if c.winding != Deck::IDENTITY || !c.closes {
            fail("the loop winds");
        }
        let singular_chart = matches!(f.chart, SurfaceKind::Sphere | SurfaceKind::Cone);
        let mut joints = c.inner_joints(f.chart);
        joints.push(c.closure(f.chart));
        if !joints
            .iter()
            .all(|j| *j == Joint::Meets || (singular_chart && matches!(j, Joint::Singular(_))))
        {
            fail("a joint neither meets nor runs along a singular row");
        }
        let poly = c.polygon();
        if crossings(&poly) != (0, 0) {
            fail("the polygon is not simple");
        }
        if (shoelace(&poly) > 0.0) != (f.sense == f.ring.is_none()) {
            fail("the polygon is wound against the face");
        }
        let boxed = c
            .forms
            .iter()
            .all(|form| matches!(*form, "Harmonic" | "FocalSection"));
        if c.window != boxed {
            fail("`Lift::window` answers off its forms");
        }
    }
    out
}

/// Per chart, in `SurfaceKind`'s `Debug` order of first appearance:
/// `(chart, faces, faces with a seam, rings, loops with a joint on a
/// singular row)`.
fn tally(facts: &[LoopFact]) -> Vec<(String, usize, usize, usize, usize)> {
    let mut out: Vec<(String, usize, usize, usize, usize)> = Vec::new();
    for f in facts {
        let chart = format!("{:?}", f.chart);
        let i = out
            .iter()
            .position(|row| row.0 == chart)
            .unwrap_or_else(|| {
                out.push((chart, 0, 0, 0, 0));
                out.len() - 1
            });
        let row = &mut out[i];
        if f.ring.is_none() {
            row.1 += 1;
            row.2 += usize::from(f.seam);
        } else {
            row.3 += 1;
        }
        if let Ok(c) = &f.chain {
            let mut joints = c.inner_joints(f.chart);
            joints.push(c.closure(f.chart));
            row.4 += usize::from(joints.iter().any(|j| matches!(j, Joint::Singular(_))));
        }
    }
    out.sort();
    out
}

fn rows(of: &[(&str, usize, usize, usize, usize)]) -> Vec<(String, usize, usize, usize, usize)> {
    of.iter()
        .map(|&(chart, a, b, c, d)| (chart.to_owned(), a, b, c, d))
        .collect()
}

/// Every body of the reach probes that builds in well under a second,
/// the swept fixtures of `common/`, and the reach probes' operands.
fn door_built() -> Vec<Named> {
    let mut out = spiric_hollows();
    out.extend(cone_sections());
    out.extend(donut_and_bar());
    out.extend(tube_on_ball());
    out.extend(pipe_and_bars());
    out.extend(cone_and_box());
    out.extend(lofts());
    out.extend(common_swept());
    out.extend([
        ("the T1 cone".to_owned(), Body::clone(&t1_cone())),
        ("the pipe".to_owned(), rod(1.0, -2.0, 4.0)),
        (
            "the z-poled ball".to_owned(),
            ball_poled_z(SQRT_2, Vec3::zero(), tol()),
        ),
        ("the vessel quarter".to_owned(), vessel_quarter()),
        ("the klein elbow".to_owned(), klein_elbow(0.275)),
    ]);
    out
}

/// **Every loop of every curved face a public door built has a lift,
/// and the lift closes in the chart.** Over the reach probes' bodies
/// (the spline section's two are
/// [`the_spline_section_bodies_read_lifts_that_close`]), `common/`'s
/// swept fixtures and the operands: `loop_lift` answers on all 175 loops
/// of the 151 curved faces, the 24 rings among them. Each winds nothing
/// and returns to its start, through the face's turn along the pole or
/// apex row where it passes one (22 loops), and its sampled polygon is
/// simple and wound with the face ([`not_closing`]). The 10 faces a
/// seam closes (the washer's and the donut's walls) close the same way:
/// the seam is walked twice, a period apart.
#[test]
fn every_loop_of_a_door_built_curved_face_has_a_lift_that_closes() {
    let facts = census(&door_built(), SAMPLES);
    let bad = not_closing(&facts);
    assert!(bad.is_empty(), "{}\n{}", bad.join("\n"), table(&facts));
    assert_eq!(
        tally(&facts),
        rows(&[
            ("Cone", 34, 0, 3, 11),
            ("Cylinder", 68, 2, 12, 0),
            ("Nurbs", 12, 0, 0, 0),
            ("Sphere", 14, 0, 3, 11),
            ("Torus", 23, 8, 6, 0),
        ]),
        "(chart, faces, seamed faces, rings, loops through a singular row)\n{}",
        table(&facts)
    );
}

/// **The spline section's bodies read the same.** The reach probes'
/// ball across a cylinder's wall, ∩ and ∪: the section's `Projected`
/// images chain with the rims' and meridians' harmonic ones, every loop
/// closes, and `Lift::window` answers on none of the loops the section
/// bounds.
#[test]
fn the_spline_section_bodies_read_lifts_that_close() {
    let facts = census(&ball_across_a_wall(), SAMPLES);
    let bad = not_closing(&facts);
    assert!(bad.is_empty(), "{}\n{}", bad.join("\n"), table(&facts));
    assert_eq!(
        tally(&facts),
        rows(&[("Cylinder", 3, 0, 0, 0), ("Sphere", 3, 0, 0, 2)]),
        "(chart, faces, seamed faces, rings, loops through a singular row)\n{}",
        table(&facts)
    );
}

/// The variant names of what `topo::finish` finds on `body`, sorted and
/// deduplicated; empty where it finishes.
fn findings(body: &Body<f64>) -> Vec<String> {
    let mut out: Vec<String> = match topo::finish(body.clone(), tol()) {
        Ok(_) => Vec::new(),
        Err(not) => not
            .as_slice()
            .iter()
            .map(|e| {
                let text = format!("{e:?}");
                text.split([' ', '{', '(']).next().unwrap_or("").to_owned()
            })
            .collect(),
    };
    out.sort();
    out.dedup();
    out
}

/// `(winding, closure joint)` of each loop of `facts`, in order; an
/// open closure joint by its kind alone.
fn windings(facts: &[LoopFact]) -> Vec<(String, String)> {
    facts
        .iter()
        .map(|f| {
            let c = f.chain.as_ref().expect("a lift");
            let closure = match c.closure(f.chart) {
                Joint::Open(..) => "Open".to_owned(),
                joint => format!("{joint:?}"),
            };
            (deck(c.winding), closure)
        })
        .collect()
}

/// **`common/`'s Euler-door witnesses store no lift, and mint one.** As
/// built, the five store no image at all, and `loop_lift` answers
/// `LiftGap` on every curved loop, naming a half-edge with no image and
/// no joint element.
/// `topo::mint_pcurves` accepts all five, and then:
///
/// - the keyway and the slit close;
/// - the oblique lens, whose loop of two tilted circle arcs holds a pole
///   inside it, ends a period from its start on both faces;
/// - the pole-crossing half cap, whose arcs run through a pole mid-edge,
///   ends neither on its start nor a period from it: its winding is the
///   involution, the chain's start is its end moved by `(u ∓ π, π − v)`,
///   and its latitudes run to `5π/6`, past the pole row;
/// - the apex-crossing bow tie, whose generators run through the apex
///   mid-edge onto the mirror nappe, ends a period from its start.
///
/// None of the five finishes; what `topo::finish` finds is pinned.
#[test]
fn the_unminted_witnesses_store_no_lift_and_mint_one() {
    let built = unminted_witnesses();
    let stored: usize = built.iter().map(|(_, body)| body.pcurves().count()).sum();
    assert_eq!(stored, 0, "the witnesses store no image as built");
    let unminted = census(&built, SAMPLES);
    assert_eq!(
        unminted.len(),
        10,
        "two curved faces a witness\n{}",
        table(&unminted)
    );
    for f in &unminted {
        let gap = f.chain.as_ref().err().map(String::as_str);
        assert!(
            gap.is_some_and(|g| g.ends_with("image stored false, joint stored false")),
            "{} has no lift\n{}",
            f.body,
            table(&unminted)
        );
    }
    let minted: Vec<Named> = unminted_witnesses()
        .into_iter()
        .map(|(name, mut body)| {
            topo::mint_pcurves(&mut body, tol())
                .unwrap_or_else(|e| panic!("{name} mints, got {e:?}"));
            (name, body)
        })
        .collect();
    let facts = census(&minted, SAMPLES);
    let read: Vec<_> = minted
        .iter()
        .map(|(name, body)| {
            let own: Vec<LoopFact> = loops_of(name, body, SAMPLES);
            (name.as_str(), windings(&own), findings(body))
        })
        .collect();
    let s = |text: &str| text.to_owned();
    let closes = || vec![(s("(0,0)"), s("Meets")), (s("(0,0)"), s("Meets"))];
    let a_period = || {
        vec![
            (s("(1,0)"), s("Periods(-1, 0)")),
            (s("(-1,0)"), s("Periods(1, 0)")),
        ]
    };
    let want = vec![
        (
            "keyway",
            closes(),
            vec![s("CurvedSenseInverted"), s("ScaffoldAtRest")],
        ),
        (
            "oblique_lens",
            a_period(),
            vec![s("ChartSingularityNotAVertex"), s("ScaffoldAtRest")],
        ),
        ("slit", closes(), vec![s("ScaffoldingStrutVertex")]),
        (
            "pole_crossing_half_cap",
            vec![(s("(0,0,σ)"), s("Open")), (s("(-1,0,σ)"), s("Open"))],
            vec![
                s("ChartSingularityNotAVertex"),
                s("CurvedSenseInverted"),
                s("ScaffoldAtRest"),
            ],
        ),
        (
            "apex_crossing_bowtie",
            a_period(),
            vec![s("ChartSingularityNotAVertex"), s("ScaffoldAtRest")],
        ),
    ];
    assert_eq!(
        read,
        want,
        "(witness, each loop's winding and closure joint, what finish finds)\n{}",
        table(&facts)
    );
    for f in &facts {
        let c = f.chain.as_ref().expect("a minted loop has a lift");
        let (_, v) = c.hull();
        match f.body.as_str() {
            "pole_crossing_half_cap" => {
                let Joint::Open(du, dv) = c.closure(f.chart) else {
                    panic!("an open closure joint\n{}", table(&facts));
                };
                let (start, last) = (c.rows[0][0], c.rows[c.rows.len() - 1][SAMPLES]);
                assert!(
                    (du.abs() - PI).abs() < MEET
                        && (start.1 - (PI - last.1)).abs() < MEET
                        && (dv.abs() - 2.0 * PI / 3.0).abs() < MEET
                        && (v.1 - 5.0 * PI / 6.0).abs() < MEET
                        && !c.closes,
                    "the half cap's start is the involution of its end\n{}",
                    table(&facts)
                );
            }
            "apex_crossing_bowtie" => assert!(
                v.0 < -0.9 && v.1 > 0.9 && !c.closes,
                "the bow tie's lift runs on both nappes\n{}",
                table(&facts)
            ),
            "oblique_lens" => assert!(
                c.forms == ["Projected", "Projected"] && v.0 > 0.0 && !c.closes,
                "the lens's loop is two projected arcs about the north pole\n{}",
                table(&facts)
            ),
            _ => assert!(c.closes, "{} closes\n{}", f.body, table(&facts)),
        }
    }
}

/// What the validators and `mesh::tessellate` answer on `body`: the
/// variant names `topo::finish` finds, whether `validate_closed`,
/// `validate_geometric` and `validate_pcurves` are clean, and the mesh's
/// answer at two δ.
fn verdict(body: &Body<f64>) -> (Vec<String>, [bool; 3], Vec<String>) {
    let band = geom_core::Band::linear(tol()).unwrap();
    let clean = [
        topo::validate::validate_closed(body).is_ok(),
        topo::validate::validate_geometric(body, tol()).is_ok(),
        topo::pcurves::validate_pcurves(body, band).is_empty(),
    ];
    let meshed = [1e-2, 1e-3]
        .into_iter()
        .map(|delta| match mesh::tessellate(body, delta, tol()) {
            Ok(_) => "meshed".to_owned(),
            Err(mesh::TessellateError::RingOnCurvedFace { face }) => {
                format!("RingOnCurvedFace on a {:?}", kind_of(body, face))
            }
            Err(e) => format!("{e:?}"),
        })
        .collect();
    (findings(body), clean, meshed)
}

/// **A loop of whole-turn rims ends a period from its start.** On every
/// analytic chart, a loop that is one circle about the axis (`common/`'s
/// one-circle cuts: a sphere cap, a cone's apex cap, a one-rim cylinder
/// face, a torus cut on its equator) winds `(±1, 0)`: its chain ends one
/// `u` period from its start, `closes_in_chart` is false, and no edge of
/// the face is a seam. None of the four finishes (`topo::finish`; what it
/// finds is pinned), and `meridian_free_face.rs` has what the mesh
/// answers on them.
///
/// **A seamless band has no loop that closes.** A cylinder, a cone
/// frustum and a torus band between two whole-turn rims, through the
/// Euler doors: the outer loop winds `(1, 0)` and the ring `(−1, 0)`, no
/// whole-period move puts the ring inside a polygon of the outer loop,
/// and the face has no seam. The pipe and the frustum finish
/// (`topo::finish`); the torus band mints, and `validate_geometric`
/// refuses its volume. `mesh::tessellate` refuses all three
/// `RingOnCurvedFace`: the outer loop is a circle, so
/// `tessellate::lane_of` sends the face to the curved lane, whose ring
/// guard is ahead of the walk.
#[test]
fn a_loop_of_whole_turn_rims_ends_a_period_from_its_start() {
    let cut = one_circle_cuts();
    let cuts = census(&cut, SAMPLES);
    assert_eq!(cuts.len(), 6, "{}", table(&cuts));
    let unfinished: Vec<(&str, Vec<String>)> = cut
        .iter()
        .map(|(name, body)| (name.as_str(), findings(body)))
        .collect();
    let found = |names: &[&str]| names.iter().map(|n| (*n).to_owned()).collect::<Vec<_>>();
    let want = vec![
        (
            "two sphere caps on one rim",
            found(&["ChartSingularityNotAVertex", "JoinableVertexAtRest"]),
        ),
        (
            "a cone apex cap and its disc",
            found(&[
                "CurvedSenseInverted",
                "JoinableVertexAtRest",
                "LoopRoleInverted",
            ]),
        ),
        (
            "a one-rim cylinder face and its disc",
            found(&["JoinableVertexAtRest", "LoopRoleInverted"]),
        ),
        (
            "a torus cut on its outer equator",
            found(&["EssentialCyclesUnpaired", "JoinableVertexAtRest"]),
        ),
    ];
    assert_eq!(unfinished, want, "what finish finds on each one-circle cut");
    let bands: Vec<Named> = vec![
        ("the seamless pipe".to_owned(), seamless_pipe()),
        ("the seamless frustum".to_owned(), seamless_frustum()),
        ("the seamless torus band".to_owned(), seamless_torus_band()),
    ];
    let banded = census(&bands, SAMPLES);
    assert_eq!(banded.len(), 6, "{}", table(&banded));
    for f in cuts.iter().chain(&banded) {
        let c = f.chain.as_ref().expect("a minted loop has a lift");
        let turns = c.winding.u;
        assert!(
            turns.abs() == 1
                && c.winding
                    == (Deck {
                        u: turns,
                        v: 0,
                        twin: false
                    })
                && !c.closes
                && c.closure(f.chart) == Joint::Periods(-turns, 0)
                && c.inner_joints(f.chart).iter().all(|j| *j == Joint::Meets)
                && !f.seam,
            "{} {}: a seamless loop one period round\n{}{}",
            f.body,
            f.role(),
            table(&cuts),
            table(&banded)
        );
    }
    let rings = rings_of(&banded);
    assert!(
        rings.len() == 3
            && rings.iter().all(|r| r.places.is_empty()
                && r.door.as_ref().is_err_and(|e| e.starts_with("OuterOpen"))),
        "no move places a band's ring in its outer loop, and the door says the outer loop \
         does not close\n{}",
        ring_table(&rings)
    );
    for pair in banded.chunks(2) {
        let wind = |f: &LoopFact| f.chain.as_ref().map(|c| c.winding.u).ok();
        assert_eq!(
            (wind(&pair[0]), wind(&pair[1])),
            (Some(1), Some(-1)),
            "{}: the outer loop and the ring wind opposite ways",
            pair[0].body
        );
    }
    let read: Vec<_> = bands
        .iter()
        .map(|(name, body)| (name.as_str(), verdict(body)))
        .collect();
    let ring = |chart: &str| vec![format!("RingOnCurvedFace on a {chart}"); 2];
    let want = vec![
        (
            "the seamless pipe",
            (Vec::new(), [true, true, true], ring("Cylinder")),
        ),
        (
            "the seamless frustum",
            (Vec::new(), [true, true, true], ring("Cone")),
        ),
        (
            "the seamless torus band",
            (
                vec!["VolumeUncomputable".to_owned()],
                [true, false, true],
                ring("Torus"),
            ),
        ),
    ];
    assert_eq!(
        read, want,
        "(findings of finish, [closed, geometric, pcurves] clean, the mesh at two δ)"
    );
}

/// Whether the hull `inner` lies in the hull `outer`, to [`MEET`].
fn hull_within(inner: ((f64, f64), (f64, f64)), outer: ((f64, f64), (f64, f64))) -> bool {
    let within = |i: (f64, f64), o: (f64, f64)| i.0 >= o.0 - MEET && i.1 <= o.1 + MEET;
    within(inner.0, outer.0) && within(inner.1, outer.1)
}

/// **Where a ring's lift sits against its outer loop's.** The 24 rings
/// of the reach probes' ringed faces: a spiric oval on each half of the
/// donut's tube, ruling-and-arc and ellipse-arc rings on the pipe's
/// half walls, an ellipse ring on the cone's half, and a tilted circle
/// on the ball's half band.
///
/// 23 sit inside the polygon their outer loop's lift draws as stored,
/// and no other whole-period move puts them there. One does not: on
/// ball ∖ tube the ring is one whole `Projected` circle, its lift is on
/// the image's own branch (`u ∈ (−2.44, −0.70)`), and the outer loop's
/// is on `u ∈ [π, 2π]`: the ring is one `u` period below, and `(1, 0)`
/// is the one move that places it. On ball ∪ tube the same circle is
/// two arcs and sits inside as stored.
///
/// `Body::face_lift` answers the same move for every ring, and every
/// sample of the ring it moves lies inside the outer loop's polygon.
///
/// Every ring runs clockwise where its face's `sense` is true, no row of
/// a ringed face is moved by the sphere's involution, and the ball's
/// ring and outer loop both lie between the pole rows `v = ±π/2`.
#[test]
fn a_ring_sits_in_its_outer_loops_polygon_but_on_the_ball_less_the_tube() {
    let mut sample = donut_and_bar();
    sample.extend(tube_on_ball());
    sample.extend(pipe_and_bars());
    sample.extend(cone_and_box());
    let facts = census(&sample, SAMPLES);
    let rings = rings_of(&facts);
    let by_chart = |chart: SurfaceKind| rings.iter().filter(|r| r.chart == chart).count();
    assert_eq!(
        (
            rings.len(),
            by_chart(SurfaceKind::Torus),
            by_chart(SurfaceKind::Cylinder),
            by_chart(SurfaceKind::Cone),
            by_chart(SurfaceKind::Sphere)
        ),
        (24, 6, 12, 3, 3),
        "the ringed faces\n{}",
        ring_table(&rings)
    );
    for r in &rings {
        let displaced = r.body == "the tube on the ball, B ∖ A";
        let want = if displaced { (1, 0) } else { (0, 0) };
        assert_eq!(
            (
                r.places.as_slice(),
                hull_within(r.ring_hull, r.outer_hull),
                &r.door
            ),
            (&[want][..], !displaced, &Ok((want, true))),
            "{} ring {}: the one move that places it, whether it is inside as stored, and \
             the door's move and whether the ring it moved is inside\n{}",
            r.body,
            r.ring,
            ring_table(&rings)
        );
        assert!(
            (r.area2 < 0.0) == r.sense && !r.twin,
            "{} ring {}: clockwise on a face of true sense, and no twin row\n{}",
            r.body,
            r.ring,
            ring_table(&rings)
        );
        if r.chart == SurfaceKind::Sphere {
            let between = |v: (f64, f64)| v.0 >= -FRAC_PI_2 - MEET && v.1 <= FRAC_PI_2 + MEET;
            assert!(
                between(r.ring_hull.1) && between(r.outer_hull.1),
                "{}: both loops between the pole rows\n{}",
                r.body,
                ring_table(&rings)
            );
        }
    }
    let displaced = rings
        .iter()
        .find(|r| r.body == "the tube on the ball, B ∖ A")
        .expect("the ball less the tube holds a ring");
    let (u, o) = (displaced.ring_hull.0, displaced.outer_hull.0);
    assert!(
        displaced.forms == ["Projected"]
            && u.0 < -2.4
            && u.1 < -0.7
            && (o.0 - PI).abs() < MEET
            && (o.1 - TAU).abs() < MEET,
        "the displaced ring is one whole projected circle, a period below its outer loop\n{}",
        ring_table(&rings)
    );
}

/// **The spiric-bounded torus wall's outer loop is a simple polygon.**
/// The cavity walls of the reach probes' two hollowed partial revolves
/// (one on the vessel, two on the klein elbow): spiric, meridian arc,
/// spiric, meridian arc. Sampled uniformly in each edge's own parameter
/// at 8, 32 and 128 chords an edge, the lift's four rows meet end to
/// end, no two sides of the polygon cross or touch, and it runs
/// clockwise in `(u, v)` on these faces of `sense` false, which is the
/// orientation `tessellate_trimmed` checks an analytic chart's polygon
/// for. `Lift::window` answers `None` on it.
#[test]
fn the_spiric_bounded_torus_walls_outer_loop_is_a_simple_polygon() {
    let hollows = spiric_hollows();
    let mut seen = Vec::new();
    for samples in [8, 32, 128] {
        let facts = census(&hollows, samples);
        for f in &facts {
            let c = f.chain.as_ref().expect("a lift");
            if !c.forms.contains(&"Spiric") {
                continue;
            }
            let poly = c.polygon();
            assert!(
                f.chart == SurfaceKind::Torus
                    && f.ring.is_none()
                    && !f.sense
                    && c.forms == ["Spiric", "Harmonic", "Spiric", "Harmonic"]
                    && c.closes
                    && c.closure(f.chart) == Joint::Meets
                    && c.inner_joints(f.chart) == [Joint::Meets; 3]
                    && poly.len() == 4 * samples
                    && crossings(&poly) == (0, 0)
                    && shoelace(&poly) < 0.0
                    && !c.window,
                "{} at {samples} an edge: crossings {:?}, 2A {}\n{}",
                f.body,
                crossings(&poly),
                shoelace(&poly),
                table(&facts)
            );
            seen.push((f.body.clone(), samples));
        }
    }
    assert_eq!(
        seen.len(),
        9,
        "one wall on the vessel, two on the elbow: {seen:?}"
    );
}

/// **At a cone's apex the lift runs along the apex row through the
/// face's turn**, as it does along a sphere's pole row. A full cone
/// revolves into two half faces and a wedge into one; each face's loop
/// is rim, generator, generator, and the two generators' images meet
/// the apex row `v = 0` at different azimuths: the arriving one's last
/// point and the leaving one's first are both at `v == 0.0` exactly,
/// and the step between them in `u` is the face's turn there, `π` for a
/// half and the sweep for a wedge, signed so the polygon closes wound
/// with the face.
///
/// The sphere's rows beside it: a ball's half band and a wedge of one
/// have two such joints each, at `v = ±π/2` to within the `64ε` that
/// `trimmed::at_pole` admits.
#[test]
fn a_lift_runs_along_the_apex_row_through_the_faces_turn() {
    let funnel = revolved(&[(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
    let sample: Vec<Named> = vec![
        ("common::cone".to_owned(), common::cone()),
        ("the T1 cone".to_owned(), Body::clone(&t1_cone())),
        ("the funnel".to_owned(), funnel),
        (
            "common::cone_wedge(1, 1)".to_owned(),
            common::cone_wedge(1.0, 1.0),
        ),
        ("common::ball".to_owned(), common::ball()),
        (
            "common::sphere_wedge(1)".to_owned(),
            common::sphere_wedge(1.0),
        ),
    ];
    let facts = census(&sample, SAMPLES);
    let bad = not_closing(&facts);
    assert!(bad.is_empty(), "{}\n{}", bad.join("\n"), table(&facts));
    let turns = turns_of(&facts);
    let read: Vec<(&str, f64, f64)> = turns
        .iter()
        .map(|t| (t.body.as_str(), t.arriving[1].1, t.turn()))
        .collect();
    for t in &turns {
        let (exit, entry) = (t.arriving[1], t.leaving[0]);
        #[allow(clippy::float_cmp)]
        let on_row = match t.chart {
            SurfaceKind::Cone => exit.1 == 0.0 && entry.1 == 0.0,
            _ => [exit.1, entry.1]
                .iter()
                .all(|v| FRAC_PI_2 - v.abs() <= 64.0 * f64::EPSILON),
        };
        // The rows either side are meridians: constant in `u`.
        #[allow(clippy::float_cmp)]
        let meridians = t.arriving[0].0 == exit.0 && t.leaving[1].0 == entry.0;
        assert!(
            on_row && meridians && t.sense,
            "{}: both points on the singular row, between two meridians\n{}",
            t.body,
            turn_table(&turns)
        );
    }
    let want = [
        ("common::cone (outer)", 0.0, -PI),
        ("common::cone (outer)", 0.0, -PI),
        ("the T1 cone (outer)", 0.0, -PI),
        ("the T1 cone (outer)", 0.0, -PI),
        ("the funnel (outer)", 0.0, PI),
        ("the funnel (outer)", 0.0, PI),
        ("common::cone_wedge(1, 1) (outer)", 0.0, -1.0),
        ("common::ball (outer)", -FRAC_PI_2, PI),
        ("common::ball (outer)", FRAC_PI_2, -PI),
        ("common::ball (outer)", FRAC_PI_2, -PI),
        ("common::ball (outer)", -FRAC_PI_2, PI),
        ("common::sphere_wedge(1) (outer)", FRAC_PI_2, -1.0),
        ("common::sphere_wedge(1) (outer)", -FRAC_PI_2, 1.0),
    ];
    assert!(
        read.len() == want.len()
            && read.iter().zip(&want).all(|(r, w)| {
                r.0 == w.0 && (r.1 - w.1).abs() < 1e-12 && (r.2 - w.2).abs() < 1e-12
            }),
        "(loop, the row's v, the step in u): {read:?}\n{}",
        turn_table(&turns)
    );
}
