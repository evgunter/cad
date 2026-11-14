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
        .mef(lone, whole_turn(wall, bottom.clone()), FaceSurface::Inherit, tol)
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
        .mef(chords, whole_turn(wall, top.clone()), FaceSurface::Inherit, tol)
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
    seamless_band(cylinder, rim(0.0, r), ellipse, [level(0.0, false), tilted], None)
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
        Ok(Self {
            forms: lift.rows().iter().map(|row| form(&row.pcurve)).collect(),
            decks: lift.rows().iter().map(|row| row.lift).collect(),
            winding: lift.winding().expect("a whole loop has a winding").deck,
            closes: lift.closes_in_chart(),
            window: lift.window().is_some(),
            rows,
        })
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
             2A {:+.4} | twin {}",
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

/// What the validators and `mesh::tessellate` answer on `body`.
fn verdicts(name: &str, body: &Body<f64>) -> String {
    let band = geom_core::Band::linear(tol()).unwrap();
    let mut out = format!("{name}:\n");
    let _ = writeln!(
        out,
        "  validate_closed: {:?}",
        topo::validate::validate_closed(body)
    );
    let _ = writeln!(
        out,
        "  validate_geometric: {:?}",
        topo::validate::validate_geometric(body, tol())
    );
    let _ = writeln!(
        out,
        "  validate_pcurves: {:?}",
        topo::pcurves::validate_pcurves(body, band)
    );
    for delta in [1e-2, 1e-3] {
        let _ = writeln!(
            out,
            "  tessellate δ={delta:e}: {:?}",
            mesh::tessellate(body, delta, tol()).map(|m| (
                m.patches.len(),
                m.patches.iter().map(|p| p.triangles.len()).sum::<usize>(),
                mesh::validate::check_mesh(&m)
            ))
        );
    }
    out
}

#[test]
fn dump() {
    let t0 = std::time::Instant::now();
    let mut log = String::new();
    let mut sample = Vec::new();
    let mut add = |what: &str, part: Vec<Named>, log: &mut String| {
        let _ = writeln!(log, "built {what}: {} bodies at {:?}", part.len(), t0.elapsed());
        sample.extend(part);
    };
    add("spiric_hollows", spiric_hollows(), &mut log);
    add("cone_sections", cone_sections(), &mut log);
    add("donut_and_bar", donut_and_bar(), &mut log);
    add("tube_on_ball", tube_on_ball(), &mut log);
    add("pipe_and_bars", pipe_and_bars(), &mut log);
    add("cone_and_box", cone_and_box(), &mut log);
    add("ball_across_a_wall", ball_across_a_wall(), &mut log);
    add("lofts", lofts(), &mut log);
    add("common", common_swept(), &mut log);
    add("unminted", unminted_witnesses(), &mut log);
    add("one_circle_cuts", one_circle_cuts(), &mut log);
    let mut minted = Vec::new();
    for (name, mut body) in unminted_witnesses() {
        match topo::mint_pcurves(&mut body, tol()) {
            Ok(()) => minted.push((format!("minted: {name}"), body)),
            Err(e) => {
                let _ = writeln!(log, "mint_pcurves refuses the witness {name}: {e:?}");
            }
        }
    }
    add("minted witnesses", minted, &mut log);
    let mut bands = Vec::new();
    for (name, build) in [
        ("the seamless pipe", seamless_pipe as fn() -> Body<f64>),
        ("the seamless frustum", seamless_frustum),
        ("the seamless torus band", seamless_torus_band),
    ] {
        match std::panic::catch_unwind(build) {
            Ok(body) => bands.push((name.to_owned(), body)),
            Err(e) => {
                let _ = writeln!(
                    log,
                    "{name} does not build: {:?}",
                    e.downcast_ref::<String>()
                );
            }
        }
    }
    for (name, body) in &bands {
        log.push_str(&verdicts(name, body));
    }
    add("bands", bands, &mut log);
    add(
        "operands",
        vec![
            ("the T1 cone".to_owned(), Body::clone(&t1_cone())),
            ("the pipe".to_owned(), rod(1.0, -2.0, 4.0)),
            (
                "the z-poled ball".to_owned(),
                ball_poled_z(SQRT_2, Vec3::zero(), tol()),
            ),
        ],
        &mut log,
    );
    let facts = census(&sample, SAMPLES);
    let _ = writeln!(log, "census at {:?}", t0.elapsed());
    let rings = rings_of(&facts);
    let turns = turns_of(&facts);
    let mut spiric = String::new();
    for samples in [8, 32, 128] {
        for (name, body) in &sample[..2] {
            for f in loops_of(name, body, samples) {
                let Ok(c) = &f.chain else { continue };
                if !c.forms.contains(&"Spiric") {
                    continue;
                }
                let poly = c.polygon();
                let _ = writeln!(
                    spiric,
                    "{} | {:?} {} | {} at {samples}/row: {} points, crossings {:?}, 2A {:+.6}",
                    f.body,
                    f.chart,
                    if f.sense { '+' } else { '-' },
                    f.role(),
                    poly.len(),
                    crossings(&poly),
                    shoelace(&poly)
                );
            }
        }
    }
    let _ = writeln!(log, "done at {:?}", t0.elapsed());
    panic!(
        "\nLOOPS\n{}\nRINGS\n{}\nTURNS\n{}\nSPIRIC\n{spiric}\nLOG\n{log}",
        table(&facts),
        ring_table(&rings),
        turn_table(&turns)
    );
}
