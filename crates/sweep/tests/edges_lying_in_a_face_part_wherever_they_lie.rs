//! **An edge lying in a face along any piece of its length is parted
//! there on a plane face, and refuses typed on a curved one**
//! (`crates/topo/README.md`, tier 3: where more than two sheets meet on
//! a line, the material decides the edges). A reflex edge of one operand
//! whose two faces both leave its line into the material of a face of
//! the other lies in that face, and the result's material about the
//! line is two wedges, one between the face and each of the edge's
//! faces: two coincident edges there, never the one edge claiming the
//! face's empty side (`topo::boolean::slit`).
//!
//! - **Off the edge's midpoint.** The plate `x ∈ [−1, 3]`,
//!   `y ∈ [−½, ½]`, `z ∈ [0, 1]` less the diamond `|x − ¾| + |y| < ¼`,
//!   whose edge at `(1, 0)` runs along `z`, less a cutter on `x = 1`
//!   extruded along `+x`: its wall holds the edge over `z ∈ [.6, .9]`,
//!   past the edge's midpoint; over `[.25, .5]`, the midpoint on the
//!   wall's boundary; over `[.25, .45]` and `[.55, .75]`, the midpoint
//!   in a hole of the wall; and over `[.2, .45]` and `[.55, .8]` of two
//!   faces a slot through the cutter parts its wall into, the edge
//!   divided against both at once.
//! - **Across arcs.** A tunnel's apex edge lying in a pocket's floor and
//!   running past it, its crossings with the floor's arcs named by
//!   edge-edge records alone: a disc floor, a D-shaped floor crossed
//!   through its chord and its arc, and the D crossed twice by its arc.
//! - **Along a ruling.** The tunnel's apex resting along the bottom
//!   ruling of a rod's wall, the rod running through the block or
//!   ending inside it, where the apex crosses the wall's arcs: `A ∖ B`
//!   would leave a slit of the curved wall, which refuses `CurvedFace`.
//! - **An arc resting in a curved face**, a V revolved about `y` whose
//!   tip arc lies in a bore's wall or a spherical floor: `A ∖ B`'s two
//!   coincident arcs refuse `Curved`.
//!
//! Each op of the scenes that build, both orders, builds at its
//! closed-form volume with tiers 3 and 3′ and its material at sample
//! points, and `A ∖ B` holds two coincident edges over each piece of the
//! edge the face holds.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::PI;

use geom_core::{Affine3, Band, Point2, Point3, RigidMotion, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::test_support::{brick, finished, revolved_about_y, sketch_at};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{AtRestBody, Body, BooleanError, BooleanResult, SolidContainment};

fn tol() -> Tol {
    Tol::witness()
}

fn poly(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect())
}

fn extrude_on(
    what: &str,
    plane: SketchPlane<f64>,
    loops: Vec<ProfileLoop<f64>>,
    h: f64,
) -> AtRestBody<f64> {
    let profile = Profile::new(plane, loops).validate(tol()).unwrap();
    let body: Body<f64> = extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished(what, body, tol())
}

fn less(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> AtRestBody<f64> {
    match topo::subtract(a, b, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what}: {:?}", other.map(|_| ())),
    }
}

fn block(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished("the block", brick::<f64>(x, y, z, tol()), tol())
}

/// A margin, negative inside.
type Margin<'a> = &'a dyn Fn(Point3<f64>) -> f64;

/// One pair of operands and what each op of them is.
struct Scene<'a> {
    what: &'a str,
    a: AtRestBody<f64>,
    b: AtRestBody<f64>,
    in_a: Margin<'a>,
    in_b: Margin<'a>,
    /// `A ∪ B`, `A ∖ B`, `B ∖ A`, `A ∩ B`.
    want: [f64; 4],
    /// Corners of the box the material is sampled in.
    sampled: (Point3<f64>, Point3<f64>),
    /// The edge's line, a point and a unit direction, and the pieces of
    /// it the face holds, as spans along it.
    keel: (Point3<f64>, Vec3<f64>, &'a [(f64, f64)]),
}

/// Six ops, each with which closed form and which membership rule.
const OPS: [(&str, usize); 6] = [
    ("A ∪ B", 0),
    ("B ∪ A", 0),
    ("A ∖ B", 1),
    ("B ∖ A", 2),
    ("A ∩ B", 3),
    ("B ∩ A", 3),
];

/// **Every op of `scene` builds sound** (module docs); what does not,
/// as a line each.
fn every_op(scene: &Scene<'_>) -> Vec<String> {
    every_op_but(scene, None)
}

/// [`every_op`], less the op `skip`.
fn every_op_but(scene: &Scene<'_>, skip: Option<&str>) -> Vec<String> {
    let eps = tol().eps();
    let (a, b) = (&scene.a, &scene.b);
    let mut wrong = Vec::new();
    for (op, k) in OPS.into_iter().filter(|&(op, _)| Some(op) != skip) {
        let what = format!("{}, {op}, ε = {eps:e}", scene.what);
        let r = match run(op, a, b) {
            Ok(BooleanResult::Body(r)) => r,
            Ok(BooleanResult::Empty) => {
                wrong.push(format!("{what}: empty"));
                continue;
            }
            Err(e) => {
                wrong.push(format!("{what}: refuses {:?}: {e}", e.kind()));
                continue;
            }
        };
        wrong.extend(unsound(&what, scene, &r, k));
    }
    wrong
}

/// Op `op` of [`OPS`] of `a` and `b`.
fn run(
    op: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
) -> Result<BooleanResult<f64>, BooleanError> {
    match op {
        "A ∪ B" => topo::union(a, b, tol()),
        "B ∪ A" => topo::union(b, a, tol()),
        "A ∖ B" => topo::subtract(a, b, tol()),
        "B ∖ A" => topo::subtract(b, a, tol()),
        "A ∩ B" => topo::intersect(a, b, tol()),
        _ => topo::intersect(b, a, tol()),
    }
}

/// What is wrong with `r`, op `k` of `scene`'s closed forms.
fn unsound(what: &str, scene: &Scene<'_>, r: &topo::BooleanBody<f64>, k: usize) -> Vec<String> {
    let mut wrong = Vec::new();
    let v = topo::mass_properties(&r.body, tol(), topo::test_support::pinned_resolution(tol()))
        .unwrap()
        .enclosure
        .volume;
    if (v - scene.want[k]).abs() > 1e-6 {
        wrong.push(format!("{what}: volume {v}, closed form {}", scene.want[k]));
    }
    if let Err(e) = topo::validate_geometric(&r.body, tol()) {
        wrong.push(format!("{what}: tier 3: {e:?}"));
    }
    if let Err(e) = topo::validate_pseudomanifold(&r.body, &r.contacts, tol()) {
        wrong.push(format!("{what}: tier 3′: {e:?}"));
    }
    let band = Band::linear(tol()).unwrap();
    let (lo, hi) = scene.sampled;
    for i in 0..7u32 {
        for j in 0..7u32 {
            for l in 0..7u32 {
                let at =
                    |a: f64, b: f64, s: u32, phase: f64| a + (b - a) * (f64::from(s) + phase) / 7.0;
                let q = Point3::new(
                    at(lo.x, hi.x, i, 0.4137),
                    at(lo.y, hi.y, j, 0.5719),
                    at(lo.z, hi.z, l, 0.3371),
                );
                let (ma, mb) = ((scene.in_a)(q), (scene.in_b)(q));
                if ma.abs() < 1e-3 || mb.abs() < 1e-3 {
                    continue;
                }
                let (ia, ib) = (ma < 0.0, mb < 0.0);
                let inside = [ia || ib, ia && !ib, ib && !ia, ia && ib][k];
                let want = if inside {
                    SolidContainment::In
                } else {
                    SolidContainment::Out
                };
                match topo::point_in_solid(&r.body, q, band, tol()) {
                    Ok(got) if got == want => {}
                    got => wrong.push(format!("{what}: at {q:?} {got:?}, where {want:?}")),
                }
            }
        }
    }
    if k == 1 {
        let (origin, dir, pieces) = scene.keel;
        let along = |p: Point3<f64>| (p - origin).dot(dir);
        let on_line = |p: Point3<f64>| (p - origin - dir * along(p)).norm() < 1e-7;
        for &(s0, s1) in pieces {
            let mid = 0.5 * (s0 + s1);
            let holding = r
                .body
                .edges()
                .filter(|(_, edge)| {
                    let start = r.body.get_half_edge(edge.he_plus).unwrap().start;
                    let end = r.body.half_edge_end(edge.he_plus).unwrap();
                    let [p, q] =
                        [start, end].map(|v| topo::readback::vertex_point(&r.body, v).unwrap());
                    let halfway = r
                        .body
                        .get_curve_geom(edge.curve)
                        .and_then(|c| c.certified())
                        .unwrap()
                        .mid_point();
                    [p, q, halfway].into_iter().all(on_line)
                        && along(p).min(along(q)) < mid
                        && mid < along(p).max(along(q))
                })
                .count();
            if holding != 2 {
                wrong.push(format!(
                    "{what}: {holding} edges over the piece [{s0}, {s1}] the face holds, where two"
                ));
            }
        }
    }
    wrong
}

/// The plate less the diamond.
fn plate() -> AtRestBody<f64> {
    extrude_on(
        "the plate",
        sketch_at(0.0),
        vec![
            poly(&[(-1.0, -0.5), (3.0, -0.5), (3.0, 0.5), (-1.0, 0.5)]),
            poly(&[(1.0, 0.0), (0.75, 0.25), (0.5, 0.0), (0.75, -0.25)]),
        ],
        1.0,
    )
}

fn in_plate(q: Point3<f64>) -> f64 {
    let bx = (-1.0 - q.x)
        .max(q.x - 3.0)
        .max(-0.5 - q.y)
        .max(q.y - 0.5)
        .max(-q.z)
        .max(q.z - 1.0);
    let diamond = ((q.x - 0.75).abs() + q.y.abs() - 0.25) / std::f64::consts::SQRT_2;
    bx.max(-diamond)
}

/// The cutter: the `(y, z)` rectangle `|y| ≤ .6`, `z ∈ [z0, z1]` on
/// `x = 1`, extruded along `+x` by 3.
fn cutter(z0: f64, z1: f64) -> AtRestBody<f64> {
    let plane = SketchPlane::from_frame(RigidMotion::axes_yz(Point3::new(1.0, 0.0, 0.0)));
    extrude_on(
        "the cutter",
        plane,
        vec![poly(&[(-0.6, z0), (0.6, z0), (0.6, z1), (-0.6, z1)])],
        3.0,
    )
}

fn in_cutter(q: Point3<f64>, z0: f64, z1: f64) -> f64 {
    (1.0 - q.x)
        .max(q.x - 4.0)
        .max(q.y.abs() - 0.6)
        .max(z0 - q.z)
        .max(q.z - z1)
}

/// The notch `x ∈ [1, 1.2]`, `|y| ≤ .1`, `z ∈ [.45, .55]`, a hole of the
/// cutter's wall around the edge's midpoint.
fn in_notch(q: Point3<f64>) -> f64 {
    (1.0 - q.x)
        .max(q.x - 1.2)
        .max(q.y.abs() - 0.1)
        .max(0.45 - q.z)
        .max(q.z - 0.55)
}

/// **The wall holds the diamond's edge away from its midpoint, and
/// every op builds sound** (module docs).
#[test]
fn an_edge_lying_in_a_wall_away_from_its_midpoint_parts_in_every_op() {
    let line = (Point3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let sampled = (Point3::new(-1.1, -0.7, -0.1), Point3::new(4.1, 0.7, 1.1));
    let in_wall_hole = |q: Point3<f64>| in_cutter(q, 0.25, 0.75).max(-in_notch(q));
    // The slot `x ∈ [.9, 1.2]`, `|y| ≤ 1`, `z ∈ [.45, .55]` cuts the
    // wall `x = 1` into two faces, each holding a piece of the edge.
    let slot = finished(
        "the slot",
        brick::<f64>((0.9, 1.2), (-1.0, 1.0), (0.45, 0.55), tol()),
        tol(),
    );
    let in_slotted = |q: Point3<f64>| {
        let slot = (0.9 - q.x)
            .max(q.x - 1.2)
            .max(q.y.abs() - 1.0)
            .max(0.45 - q.z)
            .max(q.z - 0.55);
        in_cutter(q, 0.2, 0.8).max(-slot)
    };
    let notch = finished(
        "the notch",
        brick::<f64>((1.0, 1.2), (-0.1, 0.1), (0.45, 0.55), tol()),
        tol(),
    );
    let scenes = [
        Scene {
            what: "the wall over z ∈ [.6, .9], past the midpoint",
            a: plate(),
            b: cutter(0.6, 0.9),
            in_a: &in_plate,
            in_b: &|q| in_cutter(q, 0.6, 0.9),
            want: [4.355, 3.275, 0.48, 0.6],
            sampled,
            keel: (line.0, line.1, &[(0.6, 0.9)]),
        },
        Scene {
            what: "the wall over z ∈ [.25, .5], the midpoint on its boundary",
            a: plate(),
            b: cutter(0.25, 0.5),
            in_a: &in_plate,
            in_b: &|q| in_cutter(q, 0.25, 0.5),
            want: [4.275, 3.375, 0.4, 0.5],
            sampled,
            keel: (line.0, line.1, &[(0.25, 0.5)]),
        },
        Scene {
            what: "the wall over z ∈ [.25, .75], the midpoint in its hole",
            a: plate(),
            b: less("the notched cutter", &cutter(0.25, 0.75), &notch),
            in_a: &in_plate,
            in_b: &in_wall_hole,
            want: [4.675, 2.879, 0.8, 0.996],
            sampled,
            keel: (line.0, line.1, &[(0.25, 0.45), (0.55, 0.75)]),
        },
        Scene {
            what: "the wall over z ∈ [.2, .8] slotted through, two faces",
            a: plate(),
            b: less("the slotted cutter", &cutter(0.2, 0.8), &slot),
            in_a: &in_plate,
            in_b: &in_slotted,
            want: [4.831, 2.695, 0.956, 1.18],
            sampled,
            keel: (line.0, line.1, &[(0.2, 0.45), (0.55, 0.8)]),
        },
    ];
    let wrong: Vec<String> = scenes.iter().flat_map(every_op).collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// The tunnel along `x`: the `(y, z)` triangle with its apex at
/// `(y0, 1)` and its base `.4` wide at `z = .6`, from `x = −2.5` by 5.
fn tunnel_along_x(y0: f64) -> AtRestBody<f64> {
    let plane = SketchPlane::from_frame(RigidMotion::axes_yz(Point3::new(-2.5, 0.0, 0.0)));
    extrude_on(
        "the tunnel",
        plane,
        vec![poly(&[(y0, 1.0), (y0 - 0.2, 0.6), (y0 + 0.2, 0.6)])],
        5.0,
    )
}

/// The tunnel along `y`: the `(z, x)` triangle with its apex at
/// `(1, x0)` and its base `.4` wide at `z = .6`, from `y = −2.5` by 5.
fn tunnel_along_y(x0: f64) -> AtRestBody<f64> {
    let plane = SketchPlane::from_frame(RigidMotion::axes_zx(Point3::new(0.0, -2.5, 0.0)));
    extrude_on(
        "the tunnel",
        plane,
        vec![poly(&[(1.0, x0), (0.6, x0 - 0.2), (0.6, x0 + 0.2)])],
        5.0,
    )
}

/// The tunnel's margin, `across` its section's width coordinate, `up`
/// its height and `along` its run over `[−2.5, 2.5]`.
fn in_tunnel(across: f64, up: f64, along: f64) -> f64 {
    (0.6 - up)
        .max((up - 1.0 + 2.0 * across.abs()) / 5f64.sqrt())
        .max(along.abs() - 2.5)
}

fn in_box(q: Point3<f64>, x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> f64 {
    (x.0 - q.x)
        .max(q.x - x.1)
        .max(y.0 - q.y)
        .max(q.y - y.1)
        .max(z.0 - q.z)
        .max(q.z - z.1)
}

/// **A tunnel's apex crossing a pocket floor's arcs parts in every op**
/// (module docs): the floor holds it between the crossings its
/// edge-edge records name.
#[test]
fn a_tunnel_apex_crossing_a_pocket_floors_arcs_parts_in_every_op() {
    let disc_block = ((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.5));
    let d_block = ((-2.0, 2.0), (-1.5, 2.5), (0.0, 1.5));
    let disc = extrude_on(
        "the disc",
        sketch_at(1.0),
        vec![bulge_loop(vec![
            (Point2::new(0.0, -1.0), 1.0),
            (Point2::new(0.0, 1.0), 1.0),
        ])],
        0.8,
    );
    let d = extrude_on(
        "the D",
        sketch_at(1.0),
        vec![bulge_loop(vec![
            (Point2::new(-1.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 1.0),
        ])],
        0.8,
    );
    let in_disc_pocket = |q: Point3<f64>| {
        let (x, y, z) = disc_block;
        let pocket = ((q.x * q.x + q.y * q.y).sqrt() - 1.0).max(1.0 - q.z);
        in_box(q, x, y, z).max(-pocket)
    };
    let in_d_pocket = |q: Point3<f64>| {
        let (x, y, z) = d_block;
        let pocket = ((q.x * q.x + q.y * q.y).sqrt() - 1.0)
            .max(-q.y)
            .max(1.0 - q.z);
        in_box(q, x, y, z).max(-pocket)
    };
    // The pocket takes `π r² / 2` or `π r² / 4` of the block, and the
    // tunnel `.08` per unit run, of which 4 lies in the block.
    let disc_less = 24.0 - PI / 2.0;
    let d_less = 24.0 - PI / 4.0;
    let forms = |a: f64| [a + 0.4 - 0.32, a - 0.32, 0.08, 0.32];
    let crossing = (1.0f64 - 0.25).sqrt();
    let scenes = [
        Scene {
            what: "the disc floor crossed along x",
            a: less(
                "the disc pocket",
                &block(disc_block.0, disc_block.1, disc_block.2),
                &disc,
            ),
            b: tunnel_along_x(0.0),
            in_a: &in_disc_pocket,
            in_b: &|q| in_tunnel(q.y, q.z, q.x),
            want: forms(disc_less),
            sampled: (Point3::new(-2.4, -2.1, -0.1), Point3::new(2.4, 2.1, 1.6)),
            keel: (
                Point3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
                &[(-1.0, 1.0)],
            ),
        },
        Scene {
            what: "the D floor crossed through its chord and its arc",
            a: less("the D pocket", &block(d_block.0, d_block.1, d_block.2), &d),
            b: tunnel_along_y(0.3),
            in_a: &in_d_pocket,
            in_b: &|q| in_tunnel(q.x - 0.3, q.z, q.y),
            want: forms(d_less),
            sampled: (Point3::new(-2.1, -2.4, -0.1), Point3::new(2.1, 2.6, 1.6)),
            keel: (
                Point3::new(0.3, 0.0, 1.0),
                Vec3::new(0.0, 1.0, 0.0),
                &[(0.0, (1.0f64 - 0.09).sqrt())],
            ),
        },
        Scene {
            what: "the D floor crossed twice by its arc",
            a: less("the D pocket", &block(d_block.0, d_block.1, d_block.2), &d),
            b: tunnel_along_x(0.5),
            in_a: &in_d_pocket,
            in_b: &|q| in_tunnel(q.y - 0.5, q.z, q.x),
            want: forms(d_less),
            sampled: (Point3::new(-2.4, -1.6, -0.1), Point3::new(2.4, 2.6, 1.6)),
            keel: (
                Point3::new(0.0, 0.5, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
                &[(-crossing, crossing)],
            ),
        },
    ];
    let wrong: Vec<String> = scenes.iter().flat_map(every_op).collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// The rod along `x`: the circle of radius .3 about `(y, z) = (0, 1.3)`,
/// from `x = x0` by `len`, its wall's two arcs meeting on `y = ±.3`, so
/// its bottom ruling lies inside the lower one.
fn rod_along_x(x0: f64, len: f64) -> AtRestBody<f64> {
    let plane = SketchPlane::from_frame(RigidMotion::axes_yz(Point3::new(x0, 0.0, 0.0)));
    extrude_on(
        "the rod",
        plane,
        vec![bulge_loop(vec![
            (Point2::new(0.3, 1.3), 1.0),
            (Point2::new(-0.3, 1.3), 1.0),
        ])],
        len,
    )
}

/// **The tunnel's apex resting along the bottom ruling of a rod's wall
/// builds in every op but `A ∖ B`** (module docs), the rod through the
/// block and ending inside it. `A ∖ B` leaves the apex lying in the
/// rod's wall with both its faces leaving into the wall's material: two
/// coincident edges, a slit of a curved face, which the parting refuses
/// `EdgeInFace` (`CurvedFace`) rather than ship one edge
/// (`work/splitdoor/the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured.md`).
#[test]
fn a_keel_along_a_cylinder_floors_ruling_refuses_only_its_slit() {
    let the_block = ((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.5));
    let a = less(
        "the tunnelled block",
        &block(the_block.0, the_block.1, the_block.2),
        &tunnel_along_x(0.0),
    );
    let in_a = |q: Point3<f64>| {
        let (x, y, z) = the_block;
        in_box(q, x, y, z).max(-in_tunnel(q.y, q.z, q.x))
    };
    // The rod's section inside the block, `z ≤ 1.5`: its disc less the
    // cap `.1` deep above the block.
    let section = {
        let (r, d) = (0.3f64, 0.2f64);
        PI * r * r - (r * r * (d / r).acos() - d * (r * r - d * d).sqrt())
    };
    let tunnelled = 24.0 - 0.32;
    let forms = |len: f64, inside: f64| {
        let rod = PI * 0.09 * len;
        let both = section * inside;
        [tunnelled + rod - both, tunnelled - both, rod - both, both]
    };
    let in_rod =
        |q: Point3<f64>, x0: f64, x1: f64| (q.y.hypot(q.z - 1.3) - 0.3).max(x0 - q.x).max(q.x - x1);
    let line = (Point3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
    let sampled = (Point3::new(-2.6, -2.1, -0.1), Point3::new(2.6, 2.1, 1.7));
    let scenes = [
        Scene {
            what: "the rod through the block",
            a: a.clone(),
            b: rod_along_x(-2.5, 5.0),
            in_a: &in_a,
            in_b: &|q| in_rod(q, -2.5, 2.5),
            want: forms(5.0, 4.0),
            sampled,
            keel: (line.0, line.1, &[(-2.0, 2.0)]),
        },
        Scene {
            what: "the rod ending inside the block",
            a,
            b: rod_along_x(-1.0, 2.0),
            in_a: &in_a,
            in_b: &|q| in_rod(q, -1.0, 1.0),
            want: forms(2.0, 2.0),
            sampled,
            keel: (line.0, line.1, &[(-1.0, 1.0)]),
        },
    ];
    let mut wrong: Vec<String> = Vec::new();
    for scene in &scenes {
        wrong.extend(every_op_but(scene, Some("A ∖ B")));
        match run("A ∖ B", &scene.a, &scene.b) {
            Err(BooleanError::EdgeInFace {
                gap: topo::EdgeInFaceGap::CurvedFace,
                ..
            }) => {}
            other => wrong.push(format!(
                "{}, A ∖ B: {:?}, where it refuses CurvedFace",
                scene.what,
                other.map(|_| ()).map_err(|e: BooleanError| e.kind())
            )),
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// A V revolved 150° about `y`, turned `turn` degrees about it.
fn turned_v(pts: &[(f64, f64)], turn: f64) -> AtRestBody<f64> {
    let v = revolved_about_y(
        pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect(),
        Revolution::Partial(150f64.to_radians()),
        tol(),
    );
    let turned = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), turn.to_radians());
    let v =
        topo::transform_rigid(&v, &geom_core::test_support::rigid(&turned, tol()), tol()).unwrap();
    finished("the V", v, tol())
}

/// **An arc resting in a bore's wall, or in a spherical floor, refuses
/// typed in every op**, `CurvedPierceUnsupported` at the reduction, which
/// places no arc lying in a curved face's interior
/// (`work/cleave/an-arc-lying-in-a-curved-faces-interior-refuses-at-the-reduction.md`).
/// This pins a refusal main gives too: the parting is never reached
/// here, and past the reduction it refuses `Curved`. The slab is a 270°
/// revolve about `y` whose inner wall is the cylinder `r = .5` or the
/// sphere `|p| = 2`, less a V whose tip arc lies on it, the V inside the
/// slab's turn (60°) or leaving through its end face (−20°).
#[test]
fn an_arc_resting_in_a_cylinder_or_sphere_face_refuses_only_its_slit() {
    let profile = |pts: Vec<(Point2<f64>, f64)>| {
        finished(
            "the slab",
            revolved_about_y(pts, Revolution::Partial(1.5 * PI), tol()),
            tol(),
        )
    };
    let corner = |x: f64, y: f64| (Point2::new(x, y), 0.0);
    let bore = profile(vec![
        corner(0.5, 1.0),
        corner(3.0, 1.0),
        corner(3.0, 2.0),
        corner(0.5, 2.0),
    ]);
    let top = (4.0f64 - 1.8 * 1.8).sqrt();
    let sphere = profile(vec![
        corner(2.0, 0.0),
        corner(3.0, 0.0),
        corner(3.0, 1.8),
        (Point2::new(top, 1.8), -((1.8f64 / 2.0).asin() / 4.0).tan()),
    ]);
    let tip = (3f64.sqrt(), 1.0);
    let off_tip = |deg: f64| {
        let t = f64::to_radians(deg);
        (tip.0 + t.cos(), tip.1 + t.sin())
    };
    let mut wrong = Vec::new();
    for (what, slab, v) in [
        (
            "the bore's wall",
            &bore,
            [(0.5, 1.5), (2.0, 1.2), (2.0, 1.8)],
        ),
        (
            "the spherical floor",
            &sphere,
            [tip, off_tip(15.0), off_tip(45.0)],
        ),
    ] {
        for turn in [60.0, -20.0] {
            let v = turned_v(&v, turn);
            for (op, _) in OPS {
                let cut = format!("{what}, the V turned {turn}°, {op}");
                match (op, run(op, slab, &v)) {
                    (
                        "A ∖ B",
                        Err(BooleanError::EdgeInFace {
                            gap: topo::EdgeInFaceGap::Curved,
                            ..
                        }),
                    ) => {}
                    (op, Ok(BooleanResult::Body(r))) if op != "A ∖ B" => {
                        if let Err(e) = topo::validate_geometric(&r.body, tol()) {
                            wrong.push(format!("{cut}: tier 3: {e:?}"));
                        }
                        if let Err(e) = topo::validate_pseudomanifold(&r.body, &r.contacts, tol()) {
                            wrong.push(format!("{cut}: tier 3′: {e:?}"));
                        }
                    }
                    // The V turned 60° lies inside the slab.
                    ("B ∖ A", Ok(BooleanResult::Empty)) if turn > 0.0 => {}
                    (_, other) => wrong.push(format!(
                        "{cut}: {:?}",
                        other.map(|_| ()).map_err(|e: BooleanError| e.kind())
                    )),
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
