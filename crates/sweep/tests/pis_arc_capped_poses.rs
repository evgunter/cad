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
use profile::{Profile, ProfileVertex, RawLoop, SketchPlane};
use sweep::test_support::{
    ROD_FLAT, ROD_L, ROD_R, hemisphere_on_flat_base, prism, rod_d_profile_at,
};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, PointInSolidError, SolidContainment, point_in_solid, transform_rigid};

fn tol() -> Tol {
    Tol::witness()
}

fn pv(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(Point2::new(x, y), bulge)
}

/// A meridian revolved fully about the sketch `y` axis.
fn revolved(verts: Vec<ProfileVertex<f64>>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![RawLoop::new(verts)])
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
fn prism_of(verts: Vec<ProfileVertex<f64>>, volume: f64) -> Body<f64> {
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
/// refusal must be an in-band escalation, the walk's honest posture for
/// a margin it cannot decide.
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
                    Err(PointInSolidError::Escalated { .. }) => refused += 1,
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
