//! **One wall per run, built and used.** A run of profile segments one
//! carrier holds sweeps ONE wall (crate README, "Walls: one per run").
//! These rows hold every swept body that has a run to the properties
//! the run wall is for: the topology and geometry validate, the wall
//! count is the run count, no two planar faces on one surface share an
//! edge (maximality — `merge_coplanar_faces` finds nothing to merge),
//! and a box crossing the run's stations never meets a non-maximal
//! operand: what union and subtract answer validates. Extrude by distance either way and by vector; revolve
//! full, partial, and partial the other way; a run wrapping the loop's
//! start; a run on a hole loop; runs of cocircular arcs on a cylinder,
//! a sphere and a torus, and a circle cut into arcs, which keeps its
//! canonical cut.
//!
//! The last row holds `SeamVertex`'s recourse true wherever the tag
//! fires on these bodies: one arc of a rim refused as a seam vertex
//! is one `rim_of` lists, so "request the rim whole" can be followed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane, ValidatedProfile};
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, CornerConfig};
use sweep::test_support::{bored_block_of_arcs, circle_arcs_at_z, disc_of_arcs, pocket_of_arcs};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, EdgeKey, subtract, union, validate_closed, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn cube(x0: f64, y0: f64, z0: f64, sx: f64, sy: f64, sz: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(x0, y0),
        Point2::new(x0 + sx, y0),
        Point2::new(x0 + sx, y0 + sy),
        Point2::new(x0, y0 + sy),
    ]);
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::identity(),
        Point3::new(0.0, 0.0, z0) - Point3::origin(),
    ));
    let v = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    extrude(&v, Extrusion::Distance(sz), tol()).unwrap().body
}

fn pts(p: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::polygon(p.iter().map(|&(x, y)| Point2::new(x, y)))
}

fn prof(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(tol())
        .unwrap()
}

/// Two distinct faces on one PLANAR surface key sharing an edge: a
/// planar wall that should have been one face.
fn planar_same_key_adjacency(b: &Body<f64>) -> usize {
    b.edges()
        .filter(|(_, e)| {
            let (Some(fa), Some(fb)) = (
                b.face_of_half_edge(e.he_plus),
                b.face_of_half_edge(e.he_minus),
            ) else {
                return false;
            };
            let (ka, kb) = (
                b.get_face(fa).unwrap().surface,
                b.get_face(fb).unwrap().surface,
            );
            fa != fb && ka == kb && matches!(b.get_surface(ka), Some(geom::Surface::Plane { .. }))
        })
        .count()
}

/// The row's checks; `faces` is the expected face count where the row
/// pins one. Panics naming the row.
fn holds(label: &str, b: &Body<f64>, faces: Option<usize>, tools: &[Body<f64>]) {
    let t = tol();
    if let Some(f) = faces {
        assert_eq!(b.faces().count(), f, "{label}: one wall per run");
    }
    validate_closed(b).unwrap_or_else(|e| panic!("{label}: tier 2: {e:?}"));
    validate_geometric(b, t).unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
    assert_eq!(
        planar_same_key_adjacency(b),
        0,
        "{label}: a planar wall split"
    );
    let mut m = b.clone();
    let merged = m
        .merge_coplanar_faces(t)
        .unwrap_or_else(|e| panic!("{label}: merge: {e:?}"));
    assert!(
        merged.groups.is_empty(),
        "{label}: coplanar faces left to merge"
    );
    for (i, tool) in tools.iter().enumerate() {
        for (op, r) in [
            ("union", union(b, tool, t)),
            ("subtract", subtract(b, tool, t)),
        ] {
            // A boolean may refuse for reasons of its own (a pierce it
            // has no chart for); it must never refuse the operand as
            // non-maximal, and what it answers must validate.
            let r = match r {
                Ok(r) => r,
                Err(e) => {
                    let e = format!("{e:?}");
                    assert!(
                        !e.contains("NonMaximal"),
                        "{label}: {op} with tool {i}: {e}"
                    );
                    continue;
                }
            };
            if let Some(out) = r.body() {
                validate_closed(&out.body)
                    .unwrap_or_else(|e| panic!("{label}: {op} {i} tier 2: {e:?}"));
                validate_geometric(&out.body, t)
                    .unwrap_or_else(|e| panic!("{label}: {op} {i} tier 3: {e:?}"));
            }
        }
    }
}

#[test]
fn extruded_runs_build_one_wall_each() {
    let tools = |s: f64| {
        let z0 = if s > 0.0 { 0.5 } else { -1.5 };
        vec![
            cube(1.3, -0.5, z0, 1.0, 1.0, 1.0),
            cube(0.7, -0.5, if s > 0.0 { -0.5 } else { -2.5 }, 1.0, 1.0, 1.0),
            cube(0.3, -0.5, z0, 3.4, 1.0, 1.0),
        ]
    };
    // A square whose bottom side is k collinear pieces.
    for k in 1..=5usize {
        let mut p: Vec<(f64, f64)> = (0..k).map(|i| (4.0 * i as f64 / k as f64, 0.0)).collect();
        p.extend([(4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]);
        // As drawn, and started mid-run (the run wraps the loop start).
        let starts = if k >= 2 { vec![0, 1, k - 1] } else { vec![0] };
        for start in starts {
            let mut q = p.clone();
            q.rotate_left(start);
            let v = prof(vec![pts(&q)]);
            for ext in [
                Extrusion::Distance(2.0),
                Extrusion::Distance(-2.0),
                Extrusion::Vector(Vec3::new(0.0, 0.0, 2.0)),
                Extrusion::Vector(Vec3::new(0.0, 0.0, -2.0)),
            ] {
                let s = match ext {
                    Extrusion::Distance(d) => d,
                    Extrusion::Vector(w) => w.z,
                };
                let e = extrude(&v, ext, tol()).unwrap();
                assert_eq!(e.walls[0].len(), 4, "k={k} start={start}: four walls");
                assert_eq!(
                    e.walls[0].iter().map(|w| w.segments.len()).sum::<usize>(),
                    k + 3,
                    "k={k} start={start}: the walls hold every segment"
                );
                holds(
                    &format!("extrude k={k} start={start} {ext:?}"),
                    &e.body,
                    Some(6),
                    &tools(s),
                );
            }
        }
    }
    // A run on a hole loop.
    let outer = pts(&[(0.0, 0.0), (6.0, 0.0), (6.0, 6.0), (0.0, 6.0)]);
    let hole = pts(&[
        (2.0, 2.0),
        (2.0, 4.0),
        (4.0, 4.0),
        (4.0, 2.0),
        (3.3, 2.0),
        (2.7, 2.0),
    ]);
    let v = prof(vec![outer, hole]);
    for d in [2.0, -2.0] {
        let e = extrude(&v, Extrusion::Distance(d), tol()).unwrap();
        let z0 = if d > 0.0 { 0.5 } else { -1.5 };
        holds(
            &format!("extrude hole d={d}"),
            &e.body,
            Some(10),
            &[cube(2.5, 1.5, z0, 1.0, 1.0, 1.0)],
        );
    }
}

fn y_axis() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

#[test]
fn revolved_runs_build_one_wall_each() {
    let near = || cube(0.5, -0.5, 0.3, 1.0, 1.0, 1.0);
    // (label, loops, axis, full faces, quarter-turn faces, tools)
    type Row = (
        &'static str,
        Vec<ProfileLoop<f64>>,
        RevolveAxis<f64>,
        usize,
        usize,
        Vec<Body<f64>>,
    );
    let off = RevolveAxis {
        origin: Point2::new(-1.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let rows: Vec<Row> = vec![
        (
            "axis rectangle",
            vec![pts(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)])],
            y_axis(),
            4,
            5,
            vec![near()],
        ),
        (
            "axis rectangle, runs on the base, side and top",
            vec![pts(&[
                (0.0, 0.0),
                (0.7, 0.0),
                (1.4, 0.0),
                (2.0, 0.0),
                (2.0, 1.0),
                (2.0, 2.0),
                (1.0, 2.0),
                (0.0, 2.0),
            ])],
            y_axis(),
            4,
            5,
            vec![
                near(),
                cube(0.5, 1.5, 0.3, 1.0, 1.0, 1.0),
                cube(1.5, 0.5, -0.5, 1.0, 1.0, 1.0),
                cube(0.5, -0.5, -0.5, 1.0, 1.0, 1.0),
            ],
        ),
        (
            "axis rectangle started mid-run",
            vec![pts(&[
                (1.0, 0.0),
                (2.0, 0.0),
                (2.0, 2.0),
                (0.0, 2.0),
                (0.0, 0.0),
            ])],
            y_axis(),
            4,
            5,
            vec![near()],
        ),
        (
            "off-axis lamina, runs",
            vec![pts(&[
                (1.0, 0.0),
                (1.5, 0.0),
                (2.0, 0.0),
                (2.0, 1.0),
                (2.0, 2.0),
                (1.0, 2.0),
            ])],
            y_axis(),
            4,
            6,
            vec![cube(1.5, -0.5, -0.5, 1.0, 1.0, 1.0)],
        ),
        (
            "cone, slanted run",
            vec![pts(&[
                (0.0, 0.0),
                (2.0, 0.0),
                (1.5, 0.5),
                (1.0, 1.0),
                (0.0, 2.0),
            ])],
            y_axis(),
            3,
            4,
            vec![near()],
        ),
        (
            "stepped shaft, shoulder run",
            vec![pts(&[
                (0.0, 0.0),
                (2.0, 0.0),
                (2.0, 1.0),
                (1.5, 1.0),
                (1.0, 1.0),
                (1.0, 2.0),
                (0.0, 2.0),
            ])],
            y_axis(),
            7,
            7,
            vec![cube(1.2, 0.7, -0.3, 0.6, 0.6, 0.6)],
        ),
        (
            "lamina with a run on its hole",
            vec![
                pts(&[(1.0, 0.0), (5.0, 0.0), (5.0, 4.0), (1.0, 4.0)]),
                pts(&[(2.0, 1.0), (2.0, 3.0), (4.0, 3.0), (4.0, 1.0), (3.0, 1.0)]),
            ],
            y_axis(),
            8,
            10,
            vec![cube(2.5, 0.5, -0.3, 1.0, 1.0, 0.6)],
        ),
        (
            "axis rectangle with a run on its hole",
            vec![
                pts(&[(0.0, 0.0), (5.0, 0.0), (5.0, 4.0), (0.0, 4.0)]),
                pts(&[(2.0, 1.0), (2.0, 3.0), (4.0, 3.0), (4.0, 1.0), (3.0, 1.0)]),
            ],
            y_axis(),
            8,
            9,
            vec![cube(2.5, 0.5, -0.3, 1.0, 1.0, 0.6)],
        ),
        (
            "holed square about an off axis",
            vec![
                pts(&[(0.0, 0.0), (6.0, 0.0), (6.0, 6.0), (0.0, 6.0)]),
                pts(&[
                    (2.0, 2.0),
                    (2.0, 4.0),
                    (4.0, 4.0),
                    (4.0, 2.0),
                    (3.3, 2.0),
                    (2.7, 2.0),
                ]),
            ],
            off,
            8,
            10,
            vec![cube(2.0, 1.5, -0.5, 3.0, 1.0, 1.0)],
        ),
    ];
    for (label, loops, axis, full_faces, quarter_faces, tools) in rows {
        let v = prof(loops);
        for (rev, faces) in [
            (Revolution::Full, Some(full_faces)),
            (
                Revolution::Partial(core::f64::consts::FRAC_PI_2),
                Some(quarter_faces),
            ),
            (Revolution::Partial(-2.0), None),
        ] {
            let r =
                revolve(&v, axis, rev, tol()).unwrap_or_else(|e| panic!("{label} {rev:?}: {e:?}"));
            holds(&format!("revolve {label} {rev:?}"), &r.body, faces, &tools);
            let far = cube(10.0, 10.0, 10.0, 1.0, 1.0, 1.0);
            union(&r.body, &far, tol())
                .unwrap_or_else(|e| panic!("{label} {rev:?}: disjoint union: {e:?}"));
        }
    }
}

/// Edges between two distinct faces on one CURVED surface key.
fn curved_same_key_adjacency(b: &Body<f64>) -> usize {
    b.edges()
        .filter(|(_, e)| {
            let (Some(fa), Some(fb)) = (
                b.face_of_half_edge(e.he_plus),
                b.face_of_half_edge(e.he_minus),
            ) else {
                return false;
            };
            let (ka, kb) = (
                b.get_face(fa).unwrap().surface,
                b.get_face(fb).unwrap().surface,
            );
            fa != fb && ka == kb && !matches!(b.get_surface(ka), Some(geom::Surface::Plane { .. }))
        })
        .count()
}

/// A D about `x = x0`: the half circle of radius 1 centred `(x0, 0)`
/// on its `+x` side cut into `k` cocircular arcs, closed by the
/// diameter on `x = x0`; the vertex list rotated left by `start` (the
/// run wraps the loop's start when `0 < start < k + 1`).
fn d_of_arcs(x0: f64, k: usize, start: usize) -> ProfileLoop<f64> {
    use core::f64::consts::{FRAC_PI_2, PI};
    let bulge = (PI / (4.0 * k as f64)).tan();
    let mut v: Vec<(Point2<f64>, f64)> = (0..k)
        .map(|i| {
            let a = -FRAC_PI_2 + PI * i as f64 / k as f64;
            (Point2::new(x0 + a.cos(), a.sin()), bulge)
        })
        .collect();
    v.push((Point2::new(x0, 1.0), 0.0));
    v.rotate_left(start);
    bulge_loop(v)
}

/// The unit circle centred `(x0, 0)` cut into `k` arcs.
fn circle_of_arcs(x0: f64, k: usize) -> ProfileLoop<f64> {
    use core::f64::consts::PI;
    let bulge = (PI / (2.0 * k as f64)).tan();
    bulge_loop(
        (0..k)
            .map(|i| {
                let a = 2.0 * PI * i as f64 / k as f64;
                (Point2::new(x0 + a.cos(), a.sin()), bulge)
            })
            .collect(),
    )
}

/// A run of `k` cocircular arcs sweeps ONE curved wall: extruded, a
/// D's arc side is one cylinder face beside one plane, wherever the
/// loop starts; fully revolved about its diameter it is one sphere
/// wall, about an axis beside it one torus wall. A partial revolve
/// keeps each arc's wall on the run's one key
/// (`work/band/partial-revolve-arc-runs-wait-on-the-meridian-fold.md`).
/// A circle cut into `k` arcs keeps its canonical cut: `k` walls on
/// one key.
#[test]
fn arc_runs_build_one_wall_each() {
    for k in 1..=4usize {
        for start in [0, 1, k] {
            let v = prof(vec![d_of_arcs(0.0, k, start)]);
            for d in [2.0, -2.0] {
                let label = format!("extruded D of {k} arcs start={start} d={d}");
                let e = extrude(&v, Extrusion::Distance(d), tol()).unwrap();
                assert_eq!(e.walls[0].len(), 2, "{label}: the arc run and the diameter");
                let z0 = if d > 0.0 { 0.5 } else { -1.5 };
                holds(
                    &label,
                    &e.body,
                    Some(4),
                    &[cube(0.5, -0.5, z0, 1.0, 1.0, 1.0)],
                );
                assert_eq!(
                    curved_same_key_adjacency(&e.body),
                    0,
                    "{label}: a curved wall split"
                );
            }
            // (label, axis offset, full faces, quarter-turn faces): a
            // sphere's full revolve is its wall's two π-bands; the
            // torus lamina's walls wrap whole; a quarter turn has two
            // wedge caps and one wall per arc.
            for (what, x0, full, quarter) in [("sphere", 0.0, 2, k + 2), ("torus", 2.0, 2, k + 3)] {
                let v = prof(vec![d_of_arcs(x0, k, start)]);
                for (rev, faces) in [
                    (Revolution::Full, Some(full)),
                    (Revolution::Partial(core::f64::consts::FRAC_PI_2), None),
                    (Revolution::Partial(-2.0), None),
                ] {
                    let label = format!("revolved {what} D of {k} arcs start={start} {rev:?}");
                    let r = revolve(&v, y_axis(), rev, tol())
                        .unwrap_or_else(|e| panic!("{label}: {e:?}"));
                    if matches!(rev, Revolution::Full) {
                        holds(
                            &label,
                            &r.body,
                            faces,
                            &[cube(x0 + 0.5, -0.5, 0.3, 1.0, 1.0, 1.0)],
                        );
                        continue;
                    }
                    assert_eq!(r.body.faces().count(), quarter, "{label}: one wall per arc");
                    validate_geometric(&r.body, tol())
                        .unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
                    assert_eq!(
                        curved_same_key_adjacency(&r.body),
                        k - 1,
                        "{label}: one latitude between each two arcs' walls"
                    );
                }
            }
        }
    }
    for k in 2..=4usize {
        let label = format!("extruded circle of {k} arcs");
        let e = extrude(
            &prof(vec![circle_of_arcs(0.0, k)]),
            Extrusion::Distance(2.0),
            tol(),
        )
        .unwrap();
        assert_eq!(e.walls[0].len(), k, "{label}: the cut is kept");
        holds(&label, &e.body, Some(k + 2), &[]);
        assert_eq!(
            curved_same_key_adjacency(&e.body),
            k,
            "{label}: k cut struts"
        );
        for (rev, faces) in [
            (Revolution::Full, k),
            (Revolution::Partial(core::f64::consts::FRAC_PI_2), k + 2),
        ] {
            let label = format!("revolved circle of {k} arcs {rev:?}");
            let r = revolve(&prof(vec![circle_of_arcs(3.0, k)]), y_axis(), rev, tol()).unwrap();
            holds(&label, &r.body, Some(faces), &[]);
        }
    }
}

/// True iff filleting `edges` refuses as a seam vertex.
fn refuses_seam_vertex(b: &Body<f64>, edges: &[EdgeKey]) -> bool {
    matches!(
        fillet_edges(b, edges, 0.1, tol()).map_err(|r| r.error),
        Err(BlendError::UnsupportedCorner {
            corner: CornerConfig::SeamVertex,
            ..
        })
    )
}

/// `SeamVertex`'s recourse is "request the rim whole; `rim_of` lists
/// it", so wherever one arc refuses with that tag, `rim_of` answers a
/// rim holding the arc. Every circular edge of bodies whose rims are
/// several arcs: cocircular arcs swept beside whole planar faces, and
/// full revolves whose plane walls are whole.
#[test]
fn seam_vertex_fires_only_where_rim_of_lists_the_rim() {
    let t = tol();
    let mut bodies: Vec<(String, Body<f64>)> = Vec::new();
    for n in 2..=4 {
        bodies.push((format!("disc of {n} arcs"), disc_of_arcs(n, 1.0, 1.0, t)));
        bodies.push((
            format!("pocket of {n} arcs"),
            pocket_of_arcs(n, 4.0, 1.0, 1.5, t),
        ));
        bodies.push((
            format!("bore of {n} arcs"),
            bored_block_of_arcs(n, 4.0, 1.0, 1.0, t),
        ));
    }
    for (name, p) in [
        (
            "step",
            vec![
                (0.0, 0.0),
                (2.0, 0.0),
                (2.0, 1.0),
                (1.0, 1.0),
                (1.0, 3.0),
                (0.0, 3.0),
            ],
        ),
        (
            "rectangle",
            vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
        ),
        (
            "washer",
            vec![(1.0, 0.0), (2.0, 0.0), (2.0, 2.0), (1.0, 2.0)],
        ),
        (
            "groove",
            vec![
                (0.0, 0.0),
                (2.0, 0.0),
                (2.0, 1.0),
                (1.0, 1.0),
                (1.0, 2.0),
                (2.0, 2.0),
                (2.0, 3.0),
                (0.0, 3.0),
            ],
        ),
    ] {
        let r = revolve(&prof(vec![pts(&p)]), y_axis(), Revolution::Full, t).unwrap();
        bodies.push((format!("full revolve of the {name}"), r.body));
    }
    let mut fired = 0;
    for (label, b) in &bodies {
        let circles: Vec<EdgeKey> = b
            .edges()
            .filter(|(_, e)| {
                b.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Circle { .. }))
            })
            .map(|(k, _)| k)
            .collect();
        for arc in circles {
            if refuses_seam_vertex(b, &[arc]) {
                fired += 1;
                let rim = topo::query::rim_of(b, arc).unwrap_or_else(|e| {
                    panic!("{label}: SeamVertex fired but rim_of refuses: {e:?}")
                });
                assert!(rim.contains(&arc), "{label}: the rim holds the arc");
            }
        }
    }
    // The full revolves' plane-wall rims fire; the rows are not vacuous.
    assert!(fired > 0, "no row fired SeamVertex");
    // The sites the review measured flipping to `SeamVertex` when the
    // one-seam reading read incidence alone: one arc of a disc of
    // arcs. `rim_of` reads structure, so it lists the disc's arcs as
    // one rim, and the tag's recourse holds there too.
    let disc = disc_of_arcs(2, 1.0, 1.0, t);
    let arcs = circle_arcs_at_z(&disc, 1.0);
    assert_eq!(arcs.len(), 2);
    let rim = topo::query::rim_of(&disc, arcs[0]).expect("rim_of lists a disc of arcs");
    assert!(
        arcs.iter().all(|a| rim.contains(a)),
        "the rim holds both arcs"
    );
    assert!(refuses_seam_vertex(&disc, &arcs[..1]));
}
