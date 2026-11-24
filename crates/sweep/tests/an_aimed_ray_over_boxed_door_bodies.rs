//! **An aimed ray over a boxed door body reads no side a member
//! lacks** (`topo`'s side reader, `side_reader::aimed_ray`).
//!
//! Every body here is built through a sweep door at the symbolic box
//! scalar, so every member is a solid whose side at infinity is `Out`,
//! and an aimed ray starts beyond every face whatever it aims at: a
//! reading of `Side(In)` over any box, from any aim and any normal, is a
//! wrong side. These rows look for one.
//!
//! The plain prisms are the slab's kind; the others are curved bodies
//! over a box: a filleted prism and a disc of boxed depth (cylinder
//! walls under moving arc-edged caps), and a cylinder and a cone frustum
//! of boxed radius (the curved carrier itself moves). One curved body
//! is admitted over a box: the disc of boxed radius drawn by the circle
//! verb, whose arcs store the radius the verb gated and whose walls and
//! two-arc caps turn their frames onto the recorded radial
//! ([`the_circle_verbs_disc_of_boxed_radius_is_admitted_and_read`]).
//! The doors refuse the other curved bodies over every box drawn here
//! (their arcs come through the fixture table, which records no
//! radial), and the rows say so in what they print; each is read the
//! day its door admits it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom_core::{Band, Interval, ParamSymbol, Point2, Point3, Real, Sym, SymBudget, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use sweep::test_support::{prism, revolved_about_y_at};
use sweep::{ExtrudeSide, Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::Body;

type Boxed = Sym<Interval>;

fn c(x: f64) -> Boxed {
    Boxed::from_f64(x)
}

/// A boxed prism through the extrude door, or the door's refusal as
/// text: a wide box is refused by the door before any reader runs.
fn boxed_prism(
    verts: Vec<(Point2<Boxed>, Boxed)>,
    h: Boxed,
    tol: Tol,
) -> Result<Body<Boxed>, String> {
    let profile = Profile::new(SketchPlane::<Boxed>::xy(), vec![bulge_loop(verts)])
        .validate(tol)
        .map_err(|e| format!("{e:?}"))?;
    let extrusion = Extrusion::Distance {
        depth: h,
        side: ExtrudeSide::Along,
    };
    extrude(&profile, extrusion, tol)
        .map(|built| built.body.into_body())
        .map_err(|e| format!("{e:?}"))
}

/// A boxed full revolve about the sketch's `y` axis through the revolve
/// door, or the refusal as text.
fn boxed_revolve(verts: Vec<(Point2<Boxed>, Boxed)>, tol: Tol) -> Result<Body<Boxed>, String> {
    let profile = Profile::new(SketchPlane::<Boxed>::xy(), vec![bulge_loop(verts)])
        .validate(tol)
        .map_err(|e| format!("{e:?}"))?;
    let axis = RevolveAxis {
        origin: Point2::new(c(0.0), c(0.0)),
        dir: geom_core::Vec2::new(c(0.0), c(1.0)),
    };
    revolve(&profile, axis, Revolution::Full, tol)
        .map(|built| built.body.into_body())
        .map_err(|e| format!("{e:?}"))
}

fn straight(pts: &[(f64, f64)]) -> Vec<(Point2<Boxed>, Boxed)> {
    pts.iter()
        .map(|&(x, y)| (Point2::new(c(x), c(y)), c(0.0)))
        .collect()
}

/// One boxed body: its name, the middle of its boxed parameter, and its
/// build from that parameter.
struct Shape {
    name: &'static str,
    middle: f64,
    build: fn(Boxed, Tol) -> Result<Body<Boxed>, String>,
    /// A point of the body's boundary that moves with the parameter,
    /// at parameter `p` and a turn `u` in `[0, 1)`.
    moving: fn(f64, f64) -> [f64; 3],
}

fn shapes() -> Vec<Shape> {
    vec![
        Shape {
            name: "slab",
            middle: 1.0,
            build: |h, tol| {
                boxed_prism(
                    straight(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]),
                    h,
                    tol,
                )
            },
            moving: |p, u| {
                let s = 4.0 * u;
                match s as u32 {
                    0 => [s, 0.0, p],
                    1 => [1.0, s - 1.0, p],
                    2 => [3.0 - s, 1.0, p],
                    _ => [0.0, 4.0 - s, p],
                }
            },
        },
        Shape {
            name: "L prism",
            middle: 1.0,
            build: |h, tol| {
                boxed_prism(
                    straight(&[
                        (0.0, 0.0),
                        (1.0, 0.0),
                        (1.0, 0.4),
                        (0.4, 0.4),
                        (0.4, 1.0),
                        (0.0, 1.0),
                    ]),
                    h,
                    tol,
                )
            },
            moving: |p, u| [0.4 + 0.6 * u, 0.4, p],
        },
        Shape {
            name: "filleted prism",
            middle: 1.0,
            build: |h, tol| {
                let quarter = (TAU / 16.0).tan();
                let mut verts =
                    straight(&[(0.0, 0.0), (1.0, 0.0), (1.0, 0.8), (0.8, 1.0), (0.0, 1.0)]);
                verts[2].1 = c(quarter);
                boxed_prism(verts, h, tol)
            },
            moving: |p, u| {
                let a = u * TAU / 4.0;
                [0.8 + 0.2 * a.cos(), 0.8 + 0.2 * a.sin(), p]
            },
        },
        Shape {
            name: "disc",
            middle: 1.0,
            build: |h, tol| {
                boxed_prism(
                    vec![
                        (Point2::new(c(-0.5), c(0.0)), c(1.0)),
                        (Point2::new(c(0.5), c(0.0)), c(1.0)),
                    ],
                    h,
                    tol,
                )
            },
            moving: |p, u| {
                let a = u * TAU;
                [0.5 * a.cos(), 0.5 * a.sin(), p]
            },
        },
        Shape {
            name: "disc of boxed radius, the circle verb's",
            middle: 0.7,
            build: |r, tol| {
                let circle = profile::circle(Point2::new(c(0.0), c(0.0)), r, tol)
                    .map_err(|e| format!("{e:?}"))?;
                let profile = profile::ConstructedProfile::new(
                    SketchPlane::<Boxed>::xy(),
                    vec![circle.into()],
                )
                .validate(tol)
                .map_err(|e| format!("{e:?}"))?;
                let extrusion = Extrusion::Distance {
                    depth: c(1.0),
                    side: ExtrudeSide::Along,
                };
                extrude(&profile, extrusion, tol)
                    .map(|built| built.body.into_body())
                    .map_err(|e| format!("{e:?}"))
            },
            moving: |p, u| {
                let a = u * TAU;
                [p * a.cos(), p * a.sin(), (7.0 * u).fract()]
            },
        },
        Shape {
            name: "cylinder of boxed radius",
            middle: 0.7,
            build: |r, tol| {
                let z = c(0.0);
                boxed_revolve(
                    vec![
                        (Point2::new(c(0.0), c(0.0)), z),
                        (Point2::new(r, c(0.0)), z),
                        (Point2::new(r, c(1.0)), z),
                        (Point2::new(c(0.0), c(1.0)), z),
                    ],
                    tol,
                )
            },
            moving: |p, u| {
                let a = u * TAU;
                [p * a.cos(), (7.0 * u).fract(), p * a.sin()]
            },
        },
        Shape {
            name: "cylinder of boxed height",
            middle: 1.0,
            build: |h, tol| {
                let z = c(0.0);
                boxed_revolve(
                    vec![
                        (Point2::new(c(0.0), c(0.0)), z),
                        (Point2::new(c(0.7), c(0.0)), z),
                        (Point2::new(c(0.7), h), z),
                        (Point2::new(c(0.0), h), z),
                    ],
                    tol,
                )
            },
            moving: |p, u| {
                let a = u * TAU;
                [0.7 * a.cos(), p, 0.7 * a.sin()]
            },
        },
        Shape {
            name: "frustum of boxed top radius",
            middle: 0.6,
            build: |r, tol| {
                let z = c(0.0);
                boxed_revolve(
                    vec![
                        (Point2::new(c(0.0), c(0.0)), z),
                        (Point2::new(c(1.0), c(0.0)), z),
                        (Point2::new(r, c(1.0)), z),
                        (Point2::new(c(0.0), c(1.0)), z),
                    ],
                    tol,
                )
            },
            moving: |p, u| {
                let a = u * TAU;
                [p * a.cos(), 1.0, p * a.sin()]
            },
        },
    ]
}

/// Runs `read` on `shape` over its parameter in `[lo, hi]`, in a
/// symbolic session.
fn over<R>(
    shape: &Shape,
    lo: f64,
    hi: f64,
    read: impl FnOnce(&Body<Boxed>, Band) -> R,
) -> Result<R, String> {
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    over_within(budget, shape, lo, hi, read)
}

/// [`over`] in a session of `budget`.
fn over_within<R>(
    budget: SymBudget,
    shape: &Shape,
    lo: f64,
    hi: f64,
    read: impl FnOnce(&Body<Boxed>, Band) -> R,
) -> Result<R, String> {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    geom_core::sym::with_session(budget, || {
        let p = Sym::param_over(
            ParamSymbol::new(test_utils::symbol_id("boxed_door_parameter")),
            Interval::from_bounds(lo, hi),
            lo,
            hi,
        );
        (shape.build)(p, tol).map(|body| read(&body, band))
    })
    .0
}

/// What a reading is, for the tallies.
fn kind(reading: &str) -> &'static str {
    if reading.contains("Side(In)") {
        "IN"
    } else if reading.contains("Side(Out)") {
        "out"
    } else if reading.contains("SetAside(Crossing") {
        "aside:crossing"
    } else if reading.contains("SetAside(Graze") {
        "aside:graze"
    } else if reading.contains("SetAside(Corner") || reading.contains("SetAside(Wedge") {
        "aside:corner"
    } else if reading.contains("SetAside(Limit") {
        "aside:limit"
    } else if reading.contains("Pair(") {
        "pair"
    } else if reading.contains("no aim") {
        "no aim"
    } else {
        "other"
    }
}

fn tally(readings: &[String]) -> String {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for r in readings {
        *counts.entry(kind(r)).or_default() += 1;
    }
    format!("{counts:?}")
}

/// **Every face's own aim, over boxes from a millionth to three tenths
/// of the parameter**: no aim reads `In`. The table of what each width
/// reads is printed (how wide a box each body's rays still read at),
/// asserted only for the wrong side.
///
/// **What this can go red on:** a reading of the side no member has, and
/// nothing else. A reading lost is not a wrong one, so a change that
/// only sets more rays aside leaves it green; and it stayed green with
/// an undecided edge gap read as an edge met in `edge_meets`, alone and
/// with the edge read off its two vertices, where
/// `topo`'s `a_ray_beside_a_moving_silhouette_edge_is_set_aside_at_it`
/// is red and eighteen of its rays read the wrong side. Its draws do not put a
/// ray beside a moving silhouette edge closely enough, so it is evidence
/// that these draws find no wrong side, not that the edge reading is
/// sound.
#[test]
fn no_aim_of_a_boxed_door_body_reads_in() {
    let eps = Tol::witness().eps();
    for shape in shapes() {
        for half in [1e-6, 1e-4, 1e-3, 1e-2, 0.025, 0.05, 0.1, 0.2, 0.3] {
            let (lo, hi) = (shape.middle - half, shape.middle + half);
            let readings = match over(&shape, lo, hi, |body, band| {
                topo::test_support::each_aim(body, band)
            }) {
                Ok(readings) => readings,
                Err(refusal) => {
                    let short: String = refusal.chars().take(160).collect();
                    println!(
                        "ε {eps:e} {} ±{half:e}: the door refuses: {short}",
                        shape.name
                    );
                    continue;
                }
            };
            println!("ε {eps:e} {} ±{half:e}: {}", shape.name, tally(&readings));
            for r in &readings {
                assert!(
                    kind(r) != "IN",
                    "{} over [{lo}, {hi}] at ε {eps:e}: a wrong side: {r}",
                    shape.name
                );
            }
        }
    }
}

/// `rays` aims about `shape` over `[lo, hi]`, each with a normal: half
/// in its bounding box grown by a fifth, half on the boundary that moves
/// with the parameter at a member of the box; the normals are the axes
/// and random unit vectors.
fn draws_about(
    rng: &mut test_utils::fuzz::Rng,
    shape: &Shape,
    lo: f64,
    hi: f64,
    rays: usize,
) -> Vec<([f64; 3], [f64; 3])> {
    (0..rays)
        .map(|k| {
            let aim = if k % 2 == 0 {
                [
                    rng.range(-1.3, 1.3),
                    rng.range(-0.3, 1.4),
                    rng.range(-1.3, 1.4),
                ]
            } else {
                let member = match rng.below(4) {
                    0 => lo,
                    1 => hi,
                    2 => shape.middle,
                    _ => rng.range(lo, hi),
                };
                (shape.moving)(member, rng.unit())
            };
            let normal = match rng.below(8) {
                0 => [1.0, 0.0, 0.0],
                1 => [0.0, 1.0, 0.0],
                2 => [0.0, 0.0, 1.0],
                3 => [0.0, 0.0, -1.0],
                _ => {
                    let v = [
                        rng.range(-1.0, 1.0),
                        rng.range(-1.0, 1.0),
                        rng.range(-1.0, 1.0),
                    ];
                    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-3);
                    [v[0] / n, v[1] / n, v[2] / n]
                }
            };
            (aim, normal)
        })
        .collect()
}

/// **Rays aimed anywhere about a boxed body, from any normal, read no
/// `In`** (counterexample search, shape 1). Half the aims are drawn in
/// the body's bounding box grown by a fifth; the other half are points
/// of the boundary that moves with the parameter, taken at a member of
/// the box, so the ray passes through a moving edge, rim or wall for
/// that member and beside it for the others. Normals are the axes and
/// random unit vectors.
///
/// **What this can go red on:** a reading of the side no member has, and
/// nothing else. A reading lost is not a wrong one, so a change that
/// only sets more rays aside leaves it green; and it stayed green with
/// an undecided edge gap read as an edge met in `edge_meets`, alone and
/// with the edge read off its two vertices, where
/// `topo`'s `a_ray_beside_a_moving_silhouette_edge_is_set_aside_at_it`
/// is red and eighteen of its rays read the wrong side. Its draws do not put a
/// ray beside a moving silhouette edge closely enough, so it is evidence
/// that these draws find no wrong side, not that the edge reading is
/// sound.
#[test]
fn no_ray_about_a_boxed_door_body_reads_in() {
    let mut rng = test_utils::fuzz::start("boxed_door_boxed_rays");
    let replay = test_utils::fuzz::replay();
    let eps = Tol::witness().eps();
    let boxes = test_utils::fuzz::scaled(2);
    let rays = test_utils::fuzz::scaled(40);
    for shape in shapes() {
        let (mut all, mut refused) = (Vec::new(), 0);
        for _ in 0..boxes {
            let half = rng.log_uniform(1e-5, 0.08) * shape.middle;
            let (lo, hi) = (shape.middle - half, shape.middle + half);
            let draws = draws_about(&mut rng, &shape, lo, hi, rays);
            let read = over(&shape, lo, hi, |body, band| {
                let face = body.faces().next().expect("a body has faces").0;
                draws
                    .iter()
                    .map(|&(a, n)| {
                        topo::test_support::ray_at(
                            body,
                            face,
                            Point3::new(c(a[0]), c(a[1]), c(a[2])),
                            Vec3::new(c(n[0]), c(n[1]), c(n[2])),
                            band,
                        )
                    })
                    .collect::<Vec<_>>()
            });
            let Ok(readings) = read else {
                refused += 1;
                continue;
            };
            for (r, (aim, normal)) in readings.iter().zip(&draws) {
                assert!(
                    kind(r) != "IN",
                    "{} over [{lo}, {hi}] at ε {eps:e}, aim {aim:?} normal {normal:?}: a wrong \
                     side: {r} ({replay})",
                    shape.name
                );
            }
            all.extend(readings);
        }
        println!(
            "ε {eps:e} {} rays: {} ({refused} of {boxes} boxes refused by the door)",
            shape.name,
            tally(&all)
        );
        if all.is_empty() {
            println!(
                "ε {eps:e} {}: NOT EXERCISED, the door refuses every box",
                shape.name
            );
        }
    }
}

/// The curved bodies of the translation probe, at `f64`.
fn curved_bodies(tol: Tol) -> Vec<(&'static str, Body<f64>)> {
    let p = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
    let quarter = (TAU / 16.0).tan();
    vec![
        (
            "filleted prism",
            prism(
                vec![
                    p(0.0, 0.0, 0.0),
                    p(1.0, 0.0, 0.0),
                    p(1.0, 0.8, quarter),
                    p(0.8, 1.0, 0.0),
                    p(0.0, 1.0, 0.0),
                ],
                1.0,
                tol,
            ),
        ),
        (
            "disc",
            prism(vec![p(-0.5, 0.0, 1.0), p(0.5, 0.0, 1.0)], 1.0, tol),
        ),
        (
            "cone frustum",
            revolved_about_y_at(
                vec![
                    p(0.0, 0.0, 0.0),
                    p(1.0, 0.0, 0.0),
                    p(0.6, 1.0, 0.0),
                    p(0.0, 1.0, 0.0),
                ],
                Revolution::Full,
                tol,
            ),
        ),
        (
            "half torus",
            revolved_about_y_at(
                vec![p(2.5, 0.0, 1.0), p(1.5, 0.0, 1.0)],
                Revolution::Partial(TAU / 2.0),
                tol,
            ),
        ),
        (
            "bead",
            revolved_about_y_at(
                vec![p(2.5, 0.0, 1.0), p(1.5, 0.0, 1.0)],
                Revolution::Full,
                tol,
            ),
        ),
    ]
}

/// **A ray given through a point further along its line reads the same
/// crossings of a curved body**, each advance that much less: `topo`'s
/// own row says so of a brick, and this says it of walls, cones, tori
/// and the edges between them, where the forward tests and the arm are
/// the curved arms' own. The origin is put before the body, inside it
/// and past it. A line whose reading faults is compared by whether it
/// faults (the arm differs, so a decision on the band's edge may not
/// agree; such a disagreement is counted, and bounded, not refused).
#[test]
fn a_ray_given_further_along_its_line_reads_a_curved_body_the_same() {
    let mut rng = test_utils::fuzz::start("boxed_door_translated_rays");
    let replay = test_utils::fuzz::replay();
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let lines = test_utils::fuzz::scaled(300);
    for (name, body) in curved_bodies(tol) {
        let face = body.faces().next().expect("a body has faces").0;
        let counts = |ray: &geom::Ray<f64>| {
            topo::test_support::ray_counts(&body, face, ray, band, (0, false)).map(|mut hits| {
                hits.sort_by(|a, b| a.0.total_cmp(&b.0));
                hits
            })
        };
        let (mut read, mut faulted, mut split) = (0, 0, 0);
        for _ in 0..lines {
            let through = Point3::new(
                rng.range(-2.6, 2.6),
                rng.range(-0.6, 1.1),
                rng.range(-2.6, 2.6),
            );
            let v = Vec3::new(
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            );
            let d = v.normalize();
            let start = through - d * 8.0;
            let from_start = counts(&geom::Ray::new(start, geom::stated_direction(d)));
            for k in [3.0, 8.0, 8.0 + rng.range(-1.0, 1.0), 16.0] {
                let given = counts(&geom::Ray::starting(
                    start + d * k,
                    geom::stated_direction(d),
                    -k,
                ));
                match (&from_start, &given) {
                    (Ok(a), Ok(b)) => {
                        read += 1;
                        assert_eq!(
                            a.len(),
                            b.len(),
                            "{name}, through {through:?} along {d:?}, origin {k} along: {b:?} \
                             against {a:?} ({replay})"
                        );
                        for (x, y) in a.iter().zip(b) {
                            assert!(
                                (x.0 - k - y.0).abs() < 1e-7
                                    && (x.1 - k - y.1).abs() < 1e-7
                                    && (x.2, x.3) == (y.2, y.3),
                                "{name}, through {through:?} along {d:?}, origin {k} along: \
                                 {y:?} is not {x:?} less {k} ({replay})"
                            );
                        }
                    }
                    (Err(_), Err(_)) => faulted += 1,
                    (a, b) => {
                        split += 1;
                        println!(
                            "{name}, through {through:?} along {d:?}, origin {k} \
                             along: {a:?} against {b:?}"
                        );
                    }
                }
            }
        }
        println!(
            "ε {:e} {name}: {read} read alike, {faulted} faulted alike, {split} \
             read one way and faulted the other",
            tol.eps()
        );
        assert!(
            50 * split <= read + faulted,
            "{name}: {split} lines read one way and faulted the other ({replay})"
        );
    }
}

/// How far a ray aimed along `+z` drifts along `+x` per unit it drops:
/// the aimed incidence and turn's, read off `topo`'s own probe of the
/// same ray (`side_reader::boxed_brick_rows`, where it is computed).
const DRIFT_X: f64 = 0.130_018_027_939_938;

/// **The fixture brick's wrong side, asked of the extrude door's slab**:
/// one fixed ray comes down beside the top edge it drifts toward, enters
/// every member's top cap clear of that edge and leaves through its
/// bottom cap short of the wall, so every member's ray reads `Out`. On
/// `topo`'s boxed brick with exact normals it reads `In` over depths in
/// `[0.05, 0.3]`. Here the body is the door's own, whose stored normals
/// carry the box and which the door refuses over a wide one: this
/// prints, for each box, the door's refusal or the reading, and fails on
/// a wrong side.
///
/// **What this can go red on:** a reading of the side no member has, and
/// nothing else. A reading lost is not a wrong one, so a change that
/// only sets more rays aside leaves it green; and it stayed green with
/// an undecided edge gap read as an edge met in `edge_meets`, alone and
/// with the edge read off its two vertices, where
/// `topo`'s `a_ray_beside_a_moving_silhouette_edge_is_set_aside_at_it`
/// is red and eighteen of its rays read the wrong side. Its draws do not put a
/// ray beside a moving silhouette edge closely enough, so it is evidence
/// that these draws find no wrong side, not that the edge reading is
/// sound.
#[test]
fn a_ray_beside_a_door_slabs_moving_silhouette_edge_reads_no_wrong_side() {
    let slab = shapes().remove(0);
    let eps = Tol::witness().eps();
    let mut wrong = Vec::new();
    for (lo, hi) in [
        (0.28, 0.3),
        (0.2, 0.3),
        (0.1, 0.3),
        (0.05, 0.3),
        (0.05, 0.2),
        (0.02, 0.1),
    ] {
        for margin in [0.002, 0.004, 0.01] {
            let aim = [1.0 - (DRIFT_X * hi + margin), 0.5, hi];
            let read = over(&slab, lo, hi, |body, band| {
                let face = body.faces().next().expect("a slab has faces").0;
                topo::test_support::ray_at(
                    body,
                    face,
                    Point3::new(c(aim[0]), c(aim[1]), c(aim[2])),
                    Vec3::new(c(0.0), c(0.0), c(1.0)),
                    band,
                )
            });
            let short: String = match &read {
                Ok(reading) => reading.chars().take(200).collect(),
                Err(refusal) => format!(
                    "the door refuses: {}",
                    refusal.chars().take(160).collect::<String>()
                ),
            };
            println!("ε {eps:e} door slab, depth in [{lo}, {hi}], margin {margin}: {short}");
            if read.is_ok_and(|r| r.contains("Side(In)")) {
                wrong.push(format!("depth in [{lo}, {hi}], margin {margin}"));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a ray every member reads Out reads In over the box: {wrong:#?}"
    );
}

/// **The circle verb's disc of boxed radius is admitted, and read.** A
/// disc whose radius is a box up to a tenth wide either way, drawn by
/// `profile::circle` and extruded: the door builds it at every width
/// here (cylinder walls under two-arc caps, all moving with the
/// radius). Of each face's own aim none reads `In`; up to a box of
/// `1e-4` some read `Out`, and from `1e-2` each is set aside as crossing
/// the moving wall. Of the rays drawn about it as
/// [`no_ray_about_a_boxed_door_body_reads_in`] draws them, half through
/// the wall that moves with the radius, none reads `In`.
///
/// **What this can go red on:** the door refusing the disc (a circle
/// verb that stores `radius.abs()`, or an arc frame turned onto the
/// start vertex's normalized offset, refuses it from `1e-6` up), and a
/// reading of the side no member has. It is evidence that these aims
/// find no wrong side on the one curved body a sweep door admits over a
/// box, not that the curved edge reading is sound: it stays green with
/// `topo`'s `reaches_root_in_band` answering reached without deciding,
/// and with `curved_edges::passes` reading an edge it could not place
/// as cleared (three of the rays drawn at `1e-4` then read a side or a
/// graze where they were set aside as crossing, none `In`).
#[test]
fn the_circle_verbs_disc_of_boxed_radius_is_admitted_and_read() {
    let shape = shapes()
        .into_iter()
        .find(|s| s.name.starts_with("disc of boxed radius"))
        .expect("the disc");
    let mut rng = test_utils::fuzz::start("boxed_disc_rays");
    let replay = test_utils::fuzz::replay();
    for half in [1e-6, 1e-4, 1e-2, 0.05, 0.1] {
        let (lo, hi) = (shape.middle - half, shape.middle + half);
        // The budget the sweeps' frame rows run in: the disc's
        // mapped-source forms outgrow the 64 terms the prisms need.
        let budget = SymBudget {
            max_terms: 4096,
            max_degree: 128,
        };
        let readings = over_within(budget, &shape, lo, hi, |body, band| {
            topo::test_support::each_aim(body, band)
        })
        .unwrap_or_else(|refusal| panic!("±{half:e}: the door refuses the disc: {refusal}"));
        let kinds: Vec<&str> = readings.iter().map(|r| kind(r)).collect();
        assert!(
            !kinds.contains(&"IN"),
            "±{half:e}: a wrong side: {readings:?}"
        );
        println!("±{half:e}: {}", tally(&readings));
        assert!(
            half > 1e-3 || kinds.contains(&"out"),
            "±{half:e}: no aim reads a side: {}",
            tally(&readings)
        );
        let draws = draws_about(&mut rng, &shape, lo, hi, test_utils::fuzz::scaled(40));
        let drawn = over_within(budget, &shape, lo, hi, |body, band| {
            let face = body.faces().next().expect("a body has faces").0;
            draws
                .iter()
                .map(|&(a, n)| {
                    topo::test_support::ray_at(
                        body,
                        face,
                        Point3::new(c(a[0]), c(a[1]), c(a[2])),
                        Vec3::new(c(n[0]), c(n[1]), c(n[2])),
                        band,
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|refusal| panic!("±{half:e}: the door refuses the disc: {refusal}"));
        println!("±{half:e} drawn: {}", tally(&drawn));
        for (r, (aim, normal)) in drawn.iter().zip(&draws) {
            assert!(
                kind(r) != "IN",
                "±{half:e}, aim {aim:?} normal {normal:?}: a wrong side: {r} ({replay})"
            );
        }
    }
}
