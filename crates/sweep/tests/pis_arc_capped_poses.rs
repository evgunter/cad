//! **`point_in_solid` over bodies whose PLANAR faces are bounded by
//! arcs, at rigid poses** — the class the re-posed torus barrel's false
//! `Out` belongs to (`torax_axial`'s
//! `torax_the_re_posed_barrels_cavity_reads_inside_its_outer_wall`).
//!
//! The walk's planar arm decides whether a ray's hit on a face's plane
//! lies inside the face. A face bounded by arcs is not the polygon
//! through its vertices: a revolved cap is a half-disc whose three
//! vertices are collinear, an extruded disc's cap has two, and an arc
//! bowing out of (or into) a polygon moves region across its chord. The
//! pose decides which schedule ray is the first to answer, so a misread
//! face shows at some poses and hides at others — which is why every
//! fixture here is asked at six.
//!
//! Each fixture carries its region in closed form, and every probe is a
//! grid point whose verdict is stable under a 2% nudge along each axis,
//! so no probe sits near the boundary. The assertion is the walk's
//! contract: an answer is the truth, and anything else is a typed
//! refusal.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, RawLoop, SketchPlane, test_support::bulge_loop};
use sweep::test_support::{
    ROD_FLAT, ROD_L, ROD_R, brick, dome, hemisphere_on_flat_base, prism, rod_d_profile_at,
};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, PointInSolidError, SolidContainment, point_in_solid, transform_rigid};

fn tol() -> Tol {
    Tol::witness()
}

fn pv(x: f64, y: f64, bulge: f64) -> (Point2<f64>, f64) {
    (Point2::new(x, y), bulge)
}

/// A meridian revolved fully about the sketch `y` axis.
fn revolved(verts: Vec<(Point2<f64>, f64)>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![bulge_loop(verts)])
        .validate(tol())
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol(),
    )
    .expect("the meridian revolves")
    .body
}

/// A prism whose volume is checked against the closed form, so the
/// region the probes are judged by is the body that was built.
fn prism_of(verts: Vec<(Point2<f64>, f64)>, volume: f64) -> Body<f64> {
    let body = prism(verts, 1.0, tol());
    let v = topo::mass_properties(&body, tol()).unwrap().volume;
    assert!(
        (v - volume).abs() < 1e-9,
        "the fixture is the region its truth describes: {v} vs {volume}"
    );
    body
}

/// Distance from the `y` axis and height — the revolve fixtures' frame.
fn axial(p: Point3<f64>) -> (f64, f64) {
    (p.x.hypot(p.z), p.y)
}

struct Case {
    name: &'static str,
    body: Body<f64>,
    truth: fn(Point3<f64>) -> bool,
    lo: [f64; 3],
    hi: [f64; 3],
}

#[allow(clippy::too_many_lines)] // one fixture per kind, each with its truth
fn cases() -> Vec<Case> {
    let pi = core::f64::consts::PI;
    let quarter = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    vec![
        Case {
            name: "revolved cylinder",
            body: revolved(vec![
                pv(0.0, 0.0, 0.0),
                pv(0.5, 0.0, 0.0),
                pv(0.5, 1.0, 0.0),
                pv(0.0, 1.0, 0.0),
            ]),
            truth: |p| {
                let (r, y) = axial(p);
                r < 0.5 && y > 0.0 && y < 1.0
            },
            lo: [-0.5, 0.0, -0.5],
            hi: [0.5, 1.0, 0.5],
        },
        Case {
            name: "revolved frustum",
            body: revolved(vec![
                pv(0.0, 0.0, 0.0),
                pv(0.5, 0.0, 0.0),
                pv(0.25, 1.0, 0.0),
                pv(0.0, 1.0, 0.0),
            ]),
            truth: |p| {
                let (r, y) = axial(p);
                y > 0.0 && y < 1.0 && r < 0.5 - 0.25 * y
            },
            lo: [-0.5, 0.0, -0.5],
            hi: [0.5, 1.0, 0.5],
        },
        Case {
            name: "hemisphere on a flat base",
            body: hemisphere_on_flat_base(0.5, tol()),
            truth: |p| {
                let (r, y) = axial(p);
                y > 0.0 && r * r + y * y < 0.25
            },
            lo: [-0.5, 0.0, -0.5],
            hi: [0.5, 0.5, 0.5],
        },
        Case {
            name: "torus barrel",
            body: crate::torax_axial::torus_barrel(),
            truth: |p| {
                let (r, y) = axial(p);
                y > 0.0
                    && y < 0.125
                    && r < 6.0 / 64.0 - ((5.0f64 / 64.0).powi(2) - (y - 1.0 / 16.0).powi(2)).sqrt()
            },
            lo: [-0.05, 0.0, -0.05],
            hi: [0.05, 0.125, 0.05],
        },
        Case {
            name: "torus belly",
            body: crate::torax_axial::torus_belly(),
            truth: |p| {
                let (r, y) = axial(p);
                if y <= 0.0 || y >= 0.125 {
                    return false;
                }
                if y < 1.0 / 64.0 {
                    return r < 4.0 / 64.0;
                }
                r < 7.0 / 64.0 - ((5.0f64 / 64.0).powi(2) - (y - 5.0 / 64.0).powi(2)).sqrt()
            },
            lo: [-0.0625, 0.0, -0.0625],
            hi: [0.0625, 0.125, 0.0625],
        },
        Case {
            // Two semicircles: each cap is a disc on TWO vertices.
            name: "extruded two-arc disc",
            body: prism_of(vec![pv(0.5, 0.0, 1.0), pv(-0.5, 0.0, 1.0)], pi * 0.25),
            truth: |p| p.x * p.x + p.y * p.y < 0.25 && p.z > 0.0 && p.z < 1.0,
            lo: [-0.5, -0.5, 0.0],
            hi: [0.5, 0.5, 1.0],
        },
        Case {
            // A major arc and its chord: two vertices, one of each kind.
            name: "extruded D-rod",
            body: rod_d_profile_at(tol()),
            truth: |p| {
                p.x * p.x + p.y * p.y < ROD_R * ROD_R && p.x < ROD_FLAT && p.z > 0.0 && p.z < ROD_L
            },
            lo: [-ROD_R, -ROD_R, 0.0],
            hi: [ROD_R, ROD_R, ROD_L],
        },
        Case {
            // A semicircular notch bitten into a square's top edge: the
            // arc bows INTO the polygon, so its vertices' polygon holds
            // region the face does not.
            name: "notched plate (arc bowing in)",
            body: prism_of(
                vec![
                    pv(-1.0, -1.0, 0.0),
                    pv(1.0, -1.0, 0.0),
                    pv(1.0, 1.0, 0.0),
                    pv(0.3, 1.0, -1.0),
                    pv(-0.3, 1.0, 0.0),
                    pv(-1.0, 1.0, 0.0),
                ],
                4.0 - pi * 0.09 / 2.0,
            ),
            truth: |p| {
                p.x.abs() < 1.0
                    && p.y.abs() < 1.0
                    && p.z > 0.0
                    && p.z < 1.0
                    && p.x * p.x + (p.y - 1.0).powi(2) > 0.09
            },
            lo: [-1.0, -1.0, 0.0],
            hi: [1.0, 1.0, 1.0],
        },
        Case {
            // A rectangle whose right side is a quarter-circle arc
            // bowing OUT: region beyond the polygon.
            name: "D-plate (arc bowing out)",
            body: prism_of(
                vec![
                    pv(-0.5, -0.25, 0.0),
                    pv(0.5, -0.25, quarter),
                    pv(0.5, 0.25, 0.0),
                    pv(-0.5, 0.25, 0.0),
                ],
                0.5 + 0.125 / 2.0 * (core::f64::consts::FRAC_PI_2 - 1.0),
            ),
            truth: |p| {
                let rect = p.x > -0.5 && p.x < 0.5 && p.y.abs() < 0.25;
                let seg = p.x >= 0.5 && (p.x - 0.25).powi(2) + p.y * p.y < 0.125;
                (rect || seg) && p.z > 0.0 && p.z < 1.0
            },
            lo: [-0.5, -0.25, 0.0],
            hi: [0.61, 0.25, 1.0],
        },
    ]
}

/// The cut cylinder's plane: tilted 0.3 rad about `y` through the axis
/// point `(0, 0, 1.25)`.
fn tilted_normal() -> Vec3<f64> {
    Vec3::new(0.3f64.sin(), 0.0, 0.3f64.cos())
}

fn tilted_elevation(p: Point3<f64>) -> f64 {
    (p - Point3::new(0.0, 0.0, 1.25)).dot(tilted_normal())
}

/// How far a cut-cylinder volume may sit from its closed form: the
/// tilted-section wall's flux is a QUADRATURE converged to the run's ε,
/// measured 2.2e-6 m³ off at ε = 1e-6. The volumes here only confirm
/// the fixture is the half its truth describes, so the bound sits well
/// above that and far below the 0.064 m³ a missed box would move.
const TILTED_WALL_VOLUME: f64 = 1e-4;

/// A unit cylinder of height 2.5 split by the tilted plane — the
/// corpus's `cut_cylinder`, both halves. The cut face's rim is two exact
/// `Ellipse` arcs (semi-axes `1/cos 0.3` and 1).
fn cut_cylinder(above: bool) -> Body<f64> {
    use topo::splitting::{SplitPart, SplitPlane, split};
    let tall = prism(vec![pv(-1.0, 0.0, 1.0), pv(1.0, 0.0, 1.0)], 2.5, tol());
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 1.25),
        normal: tilted_normal(),
    };
    let result = split(&tall, &plane, tol()).unwrap();
    let part = if above { &result.above } else { &result.below };
    let SplitPart::Body(half) = part else {
        panic!("each half carries material");
    };
    let v = topo::mass_properties(half, tol()).unwrap().volume;
    assert!(
        (v - core::f64::consts::PI * 1.25).abs() < TILTED_WALL_VOLUME,
        "the plane halves the cylinder: {v}"
    );
    assert!(
        half.edges().any(|(_, e)| half
            .get_curve_geom(e.curve)
            .and_then(topo::CurveGeom::certified)
            .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Ellipse { .. }))),
        "the cut face is bounded by ellipse arcs"
    );
    half.clone()
}

/// The identity, the torax row's own re-pose, and four more — two of
/// which (a quarter turn about `x`, a turn about `y` that keeps the
/// revolve axis) leave the schedule meeting the fixtures as it meets
/// them unposed.
fn poses() -> Vec<(&'static str, Affine3<f64>)> {
    let about = |pivot: [f64; 3], axis: Vec3<f64>, angle: f64| {
        Affine3::rotation_about_axis(Point3::new(pivot[0], pivot[1], pivot[2]), axis, angle)
    };
    vec![
        ("identity", about([0.0; 3], Vec3::new(1.0, 0.0, 0.0), 0.0)),
        (
            "0.7 about x through (1/4, -1/2, 1/8)",
            about([0.25, -0.5, 0.125], Vec3::new(1.0, 0.0, 0.0), 0.7),
        ),
        (
            "0.7 about z",
            about([0.0; 3], Vec3::new(0.0, 0.0, 1.0), 0.7),
        ),
        (
            "0.3 about y through (0.1, 0.2, 0.3)",
            about([0.1, 0.2, 0.3], Vec3::new(0.0, 1.0, 0.0), 0.3),
        ),
        (
            "1.1 about (1,2,3) through (-0.2, 0.1, 0.4)",
            about([-0.2, 0.1, 0.4], Vec3::new(1.0, 2.0, 3.0).normalize(), 1.1),
        ),
        (
            "pi/2 about x",
            about(
                [0.0; 3],
                Vec3::new(1.0, 0.0, 0.0),
                core::f64::consts::FRAC_PI_2,
            ),
        ),
    ]
}

/// Grid points of the fixture's box whose truth is stable under a 2%
/// nudge along each axis — clear of the boundary by construction.
fn probes(case: &Case) -> Vec<(Point3<f64>, bool)> {
    const N: usize = 5;
    let scale = (0..3).map(|a| case.hi[a] - case.lo[a]).fold(0.0, f64::max);
    let delta = scale * 0.02;
    let nudges = [
        Vec3::new(delta, 0.0, 0.0),
        Vec3::new(0.0, delta, 0.0),
        Vec3::new(0.0, 0.0, delta),
    ];
    let at = |n: usize, a: usize| {
        // The per-axis shift keeps the grid off the fixtures' planes of
        // symmetry, where a probe would sit on a seam.
        case.lo[a] + (case.hi[a] - case.lo[a]) * (n as f64 + 0.5 + 0.13 * a as f64) / N as f64
    };
    let mut out = Vec::new();
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                let p = Point3::new(at(i, 0), at(j, 1), at(k, 2));
                let t = (case.truth)(p);
                if nudges
                    .iter()
                    .all(|d| (case.truth)(p + *d) == t && (case.truth)(p - *d) == t)
                {
                    out.push((p, t));
                }
            }
        }
    }
    out
}

/// **Every arc-capped kind, at every pose: an answer is the truth.**
/// Each wrong answer is reported with its fixture, pose and point; a
/// refusal must be an in-band escalation (the solid walk's own, or the
/// in-face walk's, which arrives as `Loop(Escalated)`), the honest
/// posture for a margin it cannot decide, and at most one probe in
/// twenty per cell. Measured: at ε = 1e-6 the torus barrel at the
/// (1,2,3) pose escalates 5 of its 117 probes on
/// `point_in_arc_loop_conic_disc` — an in-face ray passing within a
/// band's width of tangent to the 3/64 m cap circle.
#[test]
fn every_arc_capped_kind_reads_its_truth_at_every_pose() {
    let band = Band::linear(tol()).expect("the witness band");
    let mut wrong = Vec::new();
    for case in cases() {
        let probes = probes(&case);
        assert!(
            probes.iter().filter(|(_, t)| *t).count() >= 8,
            "{}: the grid must reach the material",
            case.name
        );
        for (pose, map) in poses() {
            let posed = transform_rigid(&case.body, &map, tol()).unwrap();
            let mut refused = 0;
            for &(p, t) in &probes {
                let want = if t {
                    SolidContainment::In
                } else {
                    SolidContainment::Out
                };
                match point_in_solid(&posed, map.transform_point(p), band, tol()) {
                    Ok(got) if got == want => {}
                    Ok(got) => wrong.push(format!(
                        "{} | {pose} | {p:?}: {got:?}, truth {want:?}",
                        case.name
                    )),
                    Err(
                        PointInSolidError::Escalated { .. }
                        | PointInSolidError::Loop(topo::splitting::PointInLoopError::Escalated {
                            ..
                        }),
                    ) => refused += 1,
                    Err(e) => wrong.push(format!("{} | {pose} | {p:?}: refused {e:?}", case.name)),
                }
            }
            assert!(
                refused * 20 <= probes.len(),
                "{} | {pose}: {refused} of {} probes escalated — the walk must answer the \
                 grid, not refuse it",
                case.name,
                probes.len()
            );
        }
    }
    assert!(
        wrong.is_empty(),
        "{} probes answered wrong or refused untyped:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
}

/// One in-face row: a label, the body, a point on the face's plane and
/// its normal (which pick the face), and each probe with its verdict.
type FaceRow<'a> = (
    &'a str,
    &'a Body<f64>,
    Point3<f64>,
    Vec3<f64>,
    Vec<(Point3<f64>, Option<bool>)>,
);

/// The one planar face of `body` whose plane passes through `at` with
/// normal `±normal`.
fn plane_face(body: &Body<f64>, at: Point3<f64>, normal: Vec3<f64>) -> topo::FaceKey {
    let hits: Vec<topo::FaceKey> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal: n, .. })
                    if n.cross(normal).norm() < 1e-12 && (at - *origin).dot(*n).abs() < 1e-12
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(hits.len(), 1, "one face on the plane through {at:?}");
    hits[0]
}

/// **The in-face walk, asked directly** — the rows the ray sweep cannot
/// be relied on to reach: a point ON an arc (the boundary pre-pass), a
/// point on an arc's chord (interior, not boundary), a point on an
/// arc's circle but off the arc, the lune an arc bows out over and the
/// notch it bows into, points a micron either side of an arc, a vertex
/// joining two arcs, full-circle edges and a ring. `Some(true)` is
/// inside, `Some(false)` outside, `None` on the boundary.
#[test]
fn the_in_face_walk_reads_each_edge_on_its_carrier() {
    let band = Band::linear(tol()).expect("the witness band");
    let z = Vec3::new(0.0, 0.0, 1.0);
    let (s45, c200, s200) = (
        core::f64::consts::FRAC_1_SQRT_2,
        200f64.to_radians().cos(),
        200f64.to_radians().sin(),
    );
    // Near, but clear of the widest band a run draws (ε = 1e-6 escalates
    // to 1e-5).
    let near = 1e-4;
    let disc = prism_of(
        vec![pv(0.5, 0.0, 1.0), pv(-0.5, 0.0, 1.0)],
        core::f64::consts::PI * 0.25,
    );
    let rod = rod_d_profile_at(tol());
    let quarter = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let d_plate = prism_of(
        vec![
            pv(-0.5, -0.25, 0.0),
            pv(0.5, -0.25, quarter),
            pv(0.5, 0.25, 0.0),
            pv(-0.5, 0.25, 0.0),
        ],
        0.5 + 0.125 / 2.0 * (core::f64::consts::FRAC_PI_2 - 1.0),
    );
    let notched = prism_of(
        vec![
            pv(-1.0, -1.0, 0.0),
            pv(1.0, -1.0, 0.0),
            pv(1.0, 1.0, 0.0),
            pv(0.3, 1.0, -1.0),
            pv(-0.3, 1.0, 0.0),
            pv(-1.0, 1.0, 0.0),
        ],
        4.0 - core::f64::consts::PI * 0.09 / 2.0,
    );
    let dome = dome(1.0, tol());
    let holed = holed_plate();
    let cut = cut_cylinder(true);
    let axis_point = Point3::new(0.0, 0.0, 1.25);
    let on_cut = |x: f64, y: f64| {
        // The in-plane point over `(x, y)`.
        Point3::new(x, y, 1.25 - x * 0.3f64.tan())
    };
    let p = Point3::new;
    let rows: Vec<FaceRow<'_>> = vec![
        (
            "two-arc disc cap",
            &disc,
            p(0.0, 0.0, 1.0),
            z,
            vec![
                (p(0.0, 0.0, 1.0), Some(true)), // on the two arcs' shared chord
                (p(0.3, 0.2, 1.0), Some(true)),
                (p((0.5 - near) * s45, (0.5 - near) * s45, 1.0), Some(true)),
                (p((0.5 + near) * s45, (0.5 + near) * s45, 1.0), Some(false)),
                (p((0.5 - near) * c200, (0.5 - near) * s200, 1.0), Some(true)),
                (
                    p((0.5 + near) * c200, (0.5 + near) * s200, 1.0),
                    Some(false),
                ),
                (p(0.0, 0.5, 1.0), None), // on an arc
                (p(0.5, 0.0, 1.0), None), // the vertex joining the arcs
                (p(0.7, 0.0, 1.0), Some(false)),
            ],
        ),
        (
            "D-rod cap (arc and chord)",
            &rod,
            p(0.0, 0.0, ROD_L),
            z,
            vec![
                (p(0.2, 0.0, ROD_L), Some(true)),
                (p(0.4, 0.0, ROD_L), Some(false)), // inside the circle, past the flat
                (p(0.4, 0.3, ROD_L), Some(false)), // on the circle, off the arc
                (p(0.3, 0.1, ROD_L), None),        // on the flat
                (p(-0.5, 0.0, ROD_L), None),       // on the arc
            ],
        ),
        (
            "D-plate cap (arc bowing out)",
            &d_plate,
            p(0.0, 0.0, 1.0),
            z,
            vec![
                (p(0.55, 0.0, 1.0), Some(true)), // the lune beyond the chord
                (p(0.6, 0.0, 1.0), Some(true)),
                (p(0.61, 0.0, 1.0), Some(false)),
                (p(0.55, 0.2, 1.0), Some(false)),
                (p(0.5, 0.0, 1.0), Some(true)), // on the chord
            ],
        ),
        (
            "notched plate cap (arc bowing in)",
            &notched,
            p(0.0, 0.0, 1.0),
            z,
            vec![
                (p(0.0, 0.85, 1.0), Some(false)), // in the notch
                (p(0.0, 1.0, 1.0), Some(false)),  // on the chord, in the notch
                (p(0.0, 0.65, 1.0), Some(true)),
                (p(0.5, 0.9, 1.0), Some(true)),
                (p(0.0, 0.7, 1.0), None), // the notch's deepest point
            ],
        ),
        (
            "dome base annulus (two full-circle edges joined by a seam)",
            &dome,
            p(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            vec![
                (p(0.75 * s45, 0.0, 0.75 * s45), Some(true)),
                (p(0.75, 0.0, 0.0), None), // on the seam
                (p(0.0, 0.0, -0.75), Some(true)),
                (p(0.0, 0.0, 0.25), Some(false)), // in the bore
                (p(1.2, 0.0, 0.0), Some(false)),
                (p(0.0, 0.0, -1.0), None), // on the outer circle
                (p(0.0, 0.0, 0.5), None),  // on the bore
            ],
        ),
        (
            "holed plate top (a two-arc ring)",
            &holed,
            p(0.0, 0.0, 1.0),
            z,
            vec![
                (p(0.0, 0.0, 1.0), Some(false)), // in the hole
                (p(0.3, 0.3, 1.0), Some(false)),
                (p(0.4, 0.4, 1.0), Some(true)),
                (p(1.5, -1.5, 1.0), Some(true)),
                (p(0.0, 0.5, 1.0), None), // on the ring
                (p(0.0, 0.5 + near, 1.0), Some(true)),
                (p(2.5, 0.0, 1.0), Some(false)),
            ],
        ),
        (
            "cut cylinder's cut face (two ellipse arcs)",
            &cut,
            axis_point,
            tilted_normal(),
            vec![
                (axis_point, Some(true)),
                (on_cut(0.0, 1.0 - near), Some(true)),
                (on_cut(0.0, 1.0 + near), Some(false)),
                (on_cut(0.999, 0.0), Some(true)),
                (on_cut(0.6, 0.79), Some(true)),
                (on_cut(0.6, 0.81), Some(false)),
                (on_cut(0.0, 1.0), None),
            ],
        ),
    ];
    let mut wrong = Vec::new();
    for (name, body, at, normal, points) in rows {
        let face = plane_face(body, at, normal);
        for (q, want) in points {
            match topo::test_support::point_in_face(body, face, q, band) {
                Ok(got) if got == want => {}
                got => wrong.push(format!("{name} at {q:?}: want {want:?}, got {got:?}")),
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// **An edge with no crossing row refuses only where it could matter.**
/// The sectioned vessel's cavity carries planar faces bounded by
/// SPIRICS (a plane's section of the offset torus). A point on such a
/// face's plane but outside a ball holding the whole loop is outside
/// the face; a point inside that ball — here the loop's own vertex —
/// is refused typed rather than read off a chord.
#[test]
fn a_spiric_bounded_face_refuses_only_within_its_reach() {
    let band = Band::linear(tol()).expect("the witness band");
    let (_, cavity) = crate::spiric_rim::vessel_cavity(1.0 / 128.0);
    let spiric_face = cavity
        .faces()
        .find(|(_, f)| {
            matches!(
                body_surface(&cavity, f.surface),
                Some(geom::Surface::Plane { .. })
            ) && loop_carriers(&cavity, f.outer)
                .iter()
                .any(|c| matches!(c, geom::Curve3::Spiric { .. }))
        })
        .map(|(k, _)| k)
        .expect("a planar face bounded by a spiric");
    let data = cavity.get_face(spiric_face).expect("face");
    let Some(geom::Surface::Plane { normal, .. }) = body_surface(&cavity, data.surface) else {
        unreachable!("selected as a plane");
    };
    let vertex = loop_vertex(&cavity, data.outer);
    // An in-plane direction, and a point far along it.
    let across = normal.cross(Vec3::new(0.3, 0.5, 0.7)).normalize();
    let far = vertex + across * 10.0;
    assert_eq!(
        topo::test_support::point_in_face(&cavity, spiric_face, far, band).ok(),
        Some(Some(false)),
        "far outside the loop's reach, the face is missed"
    );
    let got = topo::test_support::point_in_face(&cavity, spiric_face, vertex, band);
    assert!(
        matches!(got, Err(PointInSolidError::EdgeCarrierUnsupported { face }) if face == spiric_face),
        "within the loop's reach the face refuses typed, got {got:?}"
    );
}

fn body_surface(body: &Body<f64>, s: topo::SurfaceKey) -> Option<geom::Surface<f64>> {
    body.get_surface(s).cloned()
}

fn loop_half_edges(body: &Body<f64>, lk: topo::LoopKey) -> Vec<topo::HalfEdgeKey> {
    let topo::LoopBoundary::Cycle { first } = body.get_loop(lk).expect("loop").boundary else {
        panic!("a cycle");
    };
    body.loop_cycle(first).expect("cycle")
}

fn loop_carriers(body: &Body<f64>, lk: topo::LoopKey) -> Vec<geom::Curve3<f64>> {
    loop_half_edges(body, lk)
        .into_iter()
        .filter_map(|he| {
            let e = body.get_edge(body.get_half_edge(he)?.edge)?;
            Some(body.get_curve_geom(e.curve)?.certified()?.carrier().clone())
        })
        .collect()
}

fn loop_vertex(body: &Body<f64>, lk: topo::LoopKey) -> Point3<f64> {
    let he = loop_half_edges(body, lk)[0];
    let v = body.get_half_edge(he).expect("half edge").start;
    *body
        .get_point(body.get_vertex(v).expect("vertex").point)
        .expect("point")
}

/// **One ellipse-bounded face no longer takes `point_in_solid` away
/// from the whole body.** A box sunk into the upper half of the cut
/// cylinder is disjoint from its boundary, so the subtract answers
/// through the containment fallback — which crosses the cut face's
/// ellipse arcs rather than refusing the face.
#[test]
fn a_box_inside_the_cut_cylinder_subtracts_through_the_containment_fallback() {
    let half = cut_cylinder(true);
    let cavity = brick((-0.2, 0.2), (-0.2, 0.2), (1.8, 2.2), tol());
    let out = match topo::subtract(&half, &cavity, tol()) {
        Ok(topo::BooleanResult::Body(out)) => out.body,
        other => panic!("the sunk box subtracts, got {other:?}"),
    };
    assert_eq!(topo::validate_geometric(&out, tol()), Ok(()), "tier 3");
    let v = topo::mass_properties(&out, tol()).unwrap().volume;
    let truth = core::f64::consts::PI * 1.25 - 0.4 * 0.4 * 0.4;
    assert!(
        (v - truth).abs() < TILTED_WALL_VOLUME,
        "volume {v}, truth {truth}"
    );
}

/// A 4 × 4 × 1 plate with a radius-½ circular hole on its axis: the top
/// face carries the hole as a RING of two arcs.
fn holed_plate() -> Body<f64> {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![
            RawLoop::polygon([
                Point2::new(-2.0, -2.0),
                Point2::new(2.0, -2.0),
                Point2::new(2.0, 2.0),
                Point2::new(-2.0, 2.0),
            ]),
            profile::circle(Point2::new(0.0, 0.0), 0.5, tol())
                .expect("the hole")
                .into(),
        ],
    )
    .validate(tol())
    .expect("the holed plate validates");
    let body = sweep::extrude(&profile, sweep::Extrusion::Distance(1.0), tol())
        .expect("the plate extrudes")
        .body;
    assert!(
        body.faces().any(|(_, f)| !f.rings.is_empty()),
        "the hole is a ring of the top and bottom faces"
    );
    body
}

/// **The cut cylinder's ellipse-bounded face refuses nothing.** Before
/// the ellipse arm, one ellipse-bounded flat face took `point_in_solid`
/// away from the whole body: every probe hit that face's plane and
/// refused. No probe refuses on it now, at any pose. (Whether each
/// answer is RIGHT is the wall arm's question, not this face's — see
/// the ignored row below.)
#[test]
fn the_cut_cylinders_ellipse_face_takes_nothing_away() {
    let band = Band::linear(tol()).expect("the witness band");
    for above in [true, false] {
        let half = cut_cylinder(above);
        for (pose, map) in poses() {
            let posed = transform_rigid(&half, &map, tol()).unwrap();
            for i in 0..5 {
                for j in 0..5 {
                    for k in 0..5 {
                        let f =
                            |n: usize, lo: f64, hi: f64| lo + (hi - lo) * (n as f64 + 0.5) / 5.0;
                        let q = map.transform_point(Point3::new(
                            f(i, -1.0, 1.0),
                            f(j, -1.0, 1.0),
                            f(k, 0.0, 2.5),
                        ));
                        let got = point_in_solid(&posed, q, band, tol());
                        assert!(
                            !matches!(got, Err(PointInSolidError::EdgeCarrierUnsupported { .. })),
                            "above = {above} | {pose} | {q:?}: {got:?}"
                        );
                    }
                }
            }
        }
    }
}

/// **The cut cylinder read against its closed form.** Its WALL faces
/// are bounded by the tilted section, so their region is not the
/// rectangle their boundary VERTICES span: that rectangle reaches past
/// the section where it stands high, and a ray from just across the cut
/// plane would count a hit there as a crossing and read `In`. The wall
/// arm reads each wall as the azimuth window cut by its two planes
/// (`wall_outline`), which is the face exactly.
#[test]
fn the_cut_cylinder_reads_its_truth() {
    let band = Band::linear(tol()).expect("the witness band");
    let mut wrong = Vec::new();
    let (mut answered, mut at_infinity) = (0usize, 0usize);
    for above in [true, false] {
        let half = cut_cylinder(above);
        for (pose, map) in poses() {
            let posed = transform_rigid(&half, &map, tol()).unwrap();
            for i in 0..5 {
                for j in 0..5 {
                    for k in 0..5 {
                        let f =
                            |n: usize, lo: f64, hi: f64| lo + (hi - lo) * (n as f64 + 0.5) / 5.0;
                        let p = Point3::new(f(i, -1.0, 1.0), f(j, -1.0, 1.0), f(k, 0.0, 2.5));
                        let e = tilted_elevation(p);
                        let r2 = p.x * p.x + p.y * p.y;
                        if (r2 - 1.0).abs() < 0.05 || e.abs() < 0.05 {
                            continue;
                        }
                        let t = r2 < 1.0 && if above { e > 0.0 } else { e < 0.0 };
                        let want = if t {
                            SolidContainment::In
                        } else {
                            SolidContainment::Out
                        };
                        match point_in_solid(&posed, map.transform_point(p), band, tol()) {
                            Ok(got) => {
                                answered += 1;
                                if got != want {
                                    wrong
                                        .push(format!("above = {above} | {pose} | {p:?}: {got:?}"));
                                }
                            }
                            Err(PointInSolidError::VolumeUncertified) => at_infinity += 1,
                            Err(e) => {
                                wrong.push(format!("above = {above} | {pose} | {p:?}: {e:?}"))
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    // The only refusal is the at-infinity side (a ray that crosses
    // nothing, whose side the props lane cannot give for this wall), and
    // it cannot be most of the grid: a wall read would have to be
    // refused for that.
    assert!(
        answered > at_infinity,
        "answered {answered}, at-infinity refusals {at_infinity}"
    );
}

/// **A point ON the wall but below the section is not on the upper
/// half's boundary.** At azimuth 2.8 the section stands at ≈ 1.541, so
/// the wall point at height 1.2 lies on the lower half's wall; the
/// vertex rectangle (heights 0.941 to 2.5) held it and the boundary
/// pre-pass read it `OnBoundary`. A point above the section at the same
/// azimuth is on the upper half's wall and still reads so.
#[test]
fn a_wall_point_across_the_section_is_not_on_the_upper_half() {
    let band = Band::linear(tol()).expect("the witness band");
    let half = cut_cylinder(true);
    let at = |h: f64| Point3::new(2.8f64.cos(), 2.8f64.sin(), h);
    for (pose, map) in poses() {
        let posed = transform_rigid(&half, &map, tol()).unwrap();
        let below = point_in_solid(&posed, map.transform_point(at(1.2)), band, tol());
        assert!(
            !matches!(
                below,
                Ok(SolidContainment::In | SolidContainment::OnBoundary)
            ),
            "{pose}: {below:?}"
        );
        let above = point_in_solid(&posed, map.transform_point(at(2.0)), band, tol());
        assert!(
            matches!(above, Ok(SolidContainment::OnBoundary)),
            "{pose}: {above:?}"
        );
    }
}

/// **Iso-bounded walls still read through their rectangle.** A plain
/// cylinder's walls are bounded by rims and seam meridians, and a
/// quarter sector's wall by rims and the two meridians its radial cuts
/// leave: both are the class whose rectangle IS the face, so every
/// probe answers its truth or refuses for a reason that is not the
/// wall's outline.
#[test]
fn iso_bounded_walls_answer_through_their_rectangle() {
    let band = Band::linear(tol()).expect("the witness band");
    let quarter = (core::f64::consts::PI / 8.0).tan();
    let cases = [
        Case {
            name: "plain cylinder",
            body: prism_of(
                vec![pv(-1.0, 0.0, 1.0), pv(1.0, 0.0, 1.0)],
                core::f64::consts::PI,
            ),
            truth: |p| p.x * p.x + p.y * p.y < 1.0 && p.z > 0.0 && p.z < 1.0,
            lo: [-1.0, -1.0, 0.0],
            hi: [1.0, 1.0, 1.0],
        },
        Case {
            name: "quarter sector",
            body: prism_of(
                vec![pv(0.0, 0.0, 0.0), pv(1.0, 0.0, quarter), pv(0.0, 1.0, 0.0)],
                core::f64::consts::FRAC_PI_4,
            ),
            truth: |p| {
                p.x * p.x + p.y * p.y < 1.0 && p.x > 0.0 && p.y > 0.0 && p.z > 0.0 && p.z < 1.0
            },
            lo: [-1.0, -1.0, 0.0],
            hi: [1.0, 1.0, 1.0],
        },
    ];
    for Case {
        name,
        body,
        truth,
        lo,
        hi,
    } in cases
    {
        let mut answered = 0;
        for (pose, map) in poses() {
            let posed = transform_rigid(&body, &map, tol()).unwrap();
            for i in 0..5 {
                for j in 0..5 {
                    for k in 0..5 {
                        let f =
                            |n: usize, a: usize| lo[a] + (hi[a] - lo[a]) * (n as f64 + 0.5) / 5.0;
                        let p = Point3::new(f(i, 0), f(j, 1), f(k, 2));
                        let r2 = p.x * p.x + p.y * p.y;
                        if (r2 - 1.0).abs() < 0.05 || p.x.abs() < 0.05 || p.y.abs() < 0.05 {
                            continue;
                        }
                        let want = if truth(p) {
                            SolidContainment::In
                        } else {
                            SolidContainment::Out
                        };
                        match point_in_solid(&posed, map.transform_point(p), band, tol()) {
                            Ok(got) => {
                                assert_eq!(got, want, "{name} | {pose} | {p:?}");
                                answered += 1;
                            }
                            Err(e) => assert!(
                                !matches!(e, PointInSolidError::WallOutlineUnsupported { .. }),
                                "{name} | {pose} | {p:?}: {e:?}"
                            ),
                        }
                    }
                }
            }
        }
        assert!(answered > 0, "{name}: no probe answered");
    }
}

/// One cutting plane of a tilted-cut fixture: a point on it and its
/// normal; the kept side is where `(p − point)·normal` is negative.
#[derive(Clone, Copy)]
struct Cut {
    point: Point3<f64>,
    normal: Vec3<f64>,
}

impl Cut {
    fn tilted(z: f64, about_y: f64) -> Self {
        Cut {
            point: Point3::new(0.0, 0.0, z),
            normal: Vec3::new(about_y.sin(), 0.0, about_y.cos()),
        }
    }

    fn at(point: [f64; 3], normal: [f64; 3]) -> Self {
        Cut {
            point: Point3::new(point[0], point[1], point[2]),
            normal: Vec3::new(normal[0], normal[1], normal[2]).normalize(),
        }
    }

    fn flip(self) -> Self {
        Cut {
            point: self.point,
            normal: self.normal * -1.0,
        }
    }

    fn elevation(&self, p: Point3<f64>) -> f64 {
        (p - self.point).dot(self.normal)
    }
}

/// The unit cylinder of height 2.5 cut down to the side of every plane
/// in `cuts` below it, through the split door.
fn cut_by(cuts: &[Cut]) -> Body<f64> {
    use topo::splitting::{SplitPart, SplitPlane, split};
    let mut body = prism(vec![pv(-1.0, 0.0, 1.0), pv(1.0, 0.0, 1.0)], 2.5, tol());
    for cut in cuts {
        let result = split(
            &body,
            &SplitPlane {
                origin: cut.point,
                normal: cut.normal,
            },
            tol(),
        )
        .expect("the cut splits");
        let SplitPart::Body(kept) = result.below else {
            panic!("material below every cut");
        };
        body = kept;
    }
    body
}

/// A tilted-cut fixture and its closed form.
struct CutCase {
    name: &'static str,
    body: Body<f64>,
    /// `Some(inside)` for a probe clear of the boundary, `None` near it.
    truth: Box<dyn Fn(Point3<f64>) -> Option<bool>>,
}

/// Inside the unit cylinder of height 2.5, clear of its wall and caps
/// by the probe margin.
fn in_cylinder(p: Point3<f64>) -> Option<bool> {
    let r2 = p.x * p.x + p.y * p.y;
    if (r2 - 1.0).abs() < 0.05 || p.z.abs() < 0.05 || (p.z - 2.5).abs() < 0.05 {
        return None;
    }
    Some(r2 < 1.0 && p.z > 0.0 && p.z < 2.5)
}

/// Below every cut, clear of each plane.
fn below_all(cuts: &[Cut], p: Point3<f64>) -> Option<bool> {
    let mut inside = true;
    for cut in cuts {
        let e = cut.elevation(p);
        if e.abs() < 0.05 {
            return None;
        }
        inside &= e < 0.0;
    }
    Some(inside)
}

fn tilted_cut_cases() -> Vec<CutCase> {
    let region = |cuts: Vec<Cut>| -> Box<dyn Fn(Point3<f64>) -> Option<bool>> {
        Box::new(move |p| Some(in_cylinder(p)? && below_all(&cuts, p)?))
    };
    let mut cases = Vec::new();
    let mut add = |name: &'static str, cuts: Vec<Cut>| {
        cases.push(CutCase {
            name,
            body: cut_by(&cuts),
            truth: region(cuts),
        });
    };
    // One cut through the axis, each side: the pinned witness's shape.
    add("cut 0.3 (below)", vec![Cut::tilted(1.25, 0.3)]);
    add("cut 0.3 (above)", vec![Cut::tilted(1.25, 0.3).flip()]);
    // A cut running out through the top cap: the wall's top chain is
    // rim, section, rim.
    add(
        "corner clip",
        vec![Cut::at([0.6, 0.0, 2.5], [0.6f64.sin(), 0.0, 0.6f64.cos()])],
    );
    add(
        "corner clip (the chip)",
        vec![Cut::at([0.6, 0.0, 2.5], [0.6f64.sin(), 0.0, 0.6f64.cos()]).flip()],
    );
    // Steep through the centre: the section runs out through both caps.
    add("tilt 0.9 (below)", vec![Cut::tilted(1.25, 0.9)]);
    add("tilt 0.9 (above)", vec![Cut::tilted(1.25, 0.9).flip()]);
    // Between two parallel steep cuts.
    add(
        "slab at tilt 1.0",
        vec![Cut::tilted(1.65, 1.0), Cut::tilted(0.85, 1.0).flip()],
    );
    // Below two cuts meeting in a ridge over the axis: a convex roof.
    add(
        "wedge",
        vec![
            Cut::at([0.0, 0.0, 2.0], [0.4f64.sin(), 0.0, 0.4f64.cos()]),
            Cut::at([0.0, 0.0, 2.0], [-(0.4f64.sin()), 0.0, 0.4f64.cos()]),
        ],
    );
    // Between two cuts tilted opposite ways about `x`, crossing on the
    // seam line: each wall face is a lens of two section arcs.
    add(
        "lens",
        vec![
            Cut::at([0.0, 0.0, 1.25], [0.0, 0.3f64.sin(), 0.3f64.cos()]).flip(),
            Cut::at([0.0, 0.0, 1.25], [0.0, -(0.3f64.sin()), 0.3f64.cos()]),
        ],
    );
    // Through the subtract door: the cut cylinder (below the cut) minus a
    // box inside it. Its walls keep the tilted section.
    let cut = Cut::tilted(1.25, 0.3);
    let pocket: Body<f64> = brick((-0.3, 0.1), (-0.3, 0.1), (0.3, 0.7), tol());
    match topo::subtract(&cut_by(&[cut]), &pocket, tol()) {
        Ok(topo::BooleanResult::Body(b)) => cases.push(CutCase {
            name: "cut 0.3 minus a box (subtract)",
            body: b.body,
            truth: Box::new(move |p| {
                let clear =
                    |v: f64, lo: f64, hi: f64| (v - lo).abs() >= 0.05 && (v - hi).abs() >= 0.05;
                if !(clear(p.x, -0.3, 0.1) && clear(p.y, -0.3, 0.1) && clear(p.z, 0.3, 0.7)) {
                    return None;
                }
                let in_box =
                    p.x > -0.3 && p.x < 0.1 && p.y > -0.3 && p.y < 0.1 && p.z > 0.3 && p.z < 0.7;
                Some(in_cylinder(p)? && below_all(&[cut], p)? && !in_box)
            }),
        }),
        other => panic!("the pocket subtracts: {:?}", other.err()),
    }
    cases
}

/// **Every tilted-cut wall reads its truth, and its outline is read,
/// not refused.** Each fixture's wall faces are bounded by rims, seam
/// meridians and planar sections in the combinations the split and
/// subtract doors mint: a cut through the axis, one running out through
/// a cap, a steep one through both caps, a slab, a convex roof and a
/// lens. Every answer is the truth, no probe refuses on a
/// wall's outline, and the only refusal allowed is the at-infinity side
/// the props lane owns.
#[test]
fn every_tilted_cut_wall_reads_its_truth() {
    let band = Band::linear(tol()).expect("the witness band");
    let mut problems = Vec::new();
    for case in tilted_cut_cases() {
        let (mut answered, mut wrong) = (0usize, 0usize);
        let mut refused: std::collections::BTreeMap<String, usize> = Default::default();
        for (pose, map) in poses() {
            let posed = transform_rigid(&case.body, &map, tol()).unwrap();
            for i in 0..5 {
                for j in 0..5 {
                    for k in 0..5 {
                        let f =
                            |n: usize, lo: f64, hi: f64| lo + (hi - lo) * (n as f64 + 0.5) / 5.0;
                        let p = Point3::new(f(i, -1.0, 1.0), f(j, -1.0, 1.0), f(k, 0.0, 2.5));
                        let Some(inside) = (case.truth)(p) else {
                            continue;
                        };
                        let want = if inside {
                            SolidContainment::In
                        } else {
                            SolidContainment::Out
                        };
                        match point_in_solid(&posed, map.transform_point(p), band, tol()) {
                            Ok(got) => {
                                answered += 1;
                                if got != want {
                                    wrong += 1;
                                    problems
                                        .push(format!("{} | {pose} | {p:?}: {got:?}", case.name));
                                }
                            }
                            Err(e) => {
                                let kind = format!("{e:?}");
                                let kind = kind.split([' ', '{', '(']).next().unwrap_or("");
                                *refused.entry(kind.to_string()).or_default() += 1;
                                if kind != "VolumeUncertified" {
                                    problems.push(format!("{} | {pose} | {p:?}: {e:?}", case.name));
                                }
                            }
                        }
                    }
                }
            }
        }
        eprintln!(
            "MEASURE {}: answered {answered}, wrong {wrong}, refused {refused:?}",
            case.name
        );
        if answered == 0 {
            problems.push(format!("{}: no probe answered", case.name));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}
