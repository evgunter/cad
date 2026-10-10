//! **An edge crossing a cone face**: the crossing layer's cone column,
//! from finished bodies.
//!
//! The first row runs every op on two of the poses through the public
//! doors. The rest reach the crossing layer through
//! `topo::sweep_split`, the `sweep-testing` door that
//! runs both sweep directions and hands back the split operands.
//!
//! Each split is held to an oracle that reads no kernel code: along every
//! edge of the other operand, the sign changes of the cone's quadric form
//! `ρ² − (k·(y − y_apex))²` and of each cap's height, sampled densely and
//! bisected, kept where they land on the frustum's face (its axial window
//! for the wall, its disc for a cap). The new vertices the sweep put on
//! that operand are exactly those points, one for one, wherever the
//! crossing lane answers; the poses it must not answer refuse typed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::solid_truth::{self, Op, Solid, Want};
use crate::revolve_common::{axis_y, validated};
use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};
use topo::{AtRestBody, Body, BooleanError, BooleanOp};

/// A solid of revolution about `y` whose lateral face is one cone: the
/// radius is `r0 + k·(y − y0)` over `y ∈ [y0, y1]`, with a cap at each
/// end where the radius there is positive.
#[derive(Clone, Copy)]
struct Frustum {
    y0: f64,
    y1: f64,
    r0: f64,
    k: f64,
}

impl Frustum {
    fn r(self, y: f64) -> f64 {
        self.r0 + self.k * (y - self.y0)
    }

    fn apex_y(self) -> f64 {
        self.y0 - self.r0 / self.k
    }

    fn body(self) -> AtRestBody<f64> {
        let mut pts = vec![(0.0, self.y0), (self.r0, self.y0)];
        let r1 = self.r(self.y1);
        if r1 > 0.0 {
            pts.push((r1, self.y1));
        }
        pts.push((0.0, self.y1));
        let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
        let body = revolve(
            &validated(vec![lp]),
            axis_y(),
            Revolution::Full,
            Tol::witness(),
        )
        .unwrap()
        .body;
        finished("the frustum", body, Tol::witness())
    }

    /// The faces' crossings along `p(t)`, `t ∈ [t0, t1]`: the double
    /// cone's quadric form and each cap's height, sign changes bisected,
    /// kept where they land on the face.
    fn crossings(self, p: impl Fn(f64) -> Point3<f64>, t0: f64, t1: f64) -> Vec<Point3<f64>> {
        let rho = |q: Point3<f64>| q.x.hypot(q.z);
        let wall = |q: Point3<f64>| rho(q).powi(2) - (self.k * (q.y - self.apex_y())).powi(2);
        let caps: Vec<(f64, f64)> = [self.y0, self.y1]
            .into_iter()
            .map(|y| (y, self.r(y)))
            .filter(|&(_, r)| r > 0.0)
            .collect();
        let mut out = Vec::new();
        let on_wall = |q: Point3<f64>| q.y > self.y0 && q.y < self.y1;
        for q in sign_changes(|t| wall(p(t)), t0, t1).into_iter().map(&p) {
            if on_wall(q) {
                out.push(q);
            }
        }
        for (y, r) in caps {
            for q in sign_changes(|t| p(t).y - y, t0, t1).into_iter().map(&p) {
                if rho(q) < r {
                    out.push(q);
                }
            }
        }
        out
    }
}

/// The parameters in `[t0, t1]` where `f` changes sign, sampled at
/// 20 000 steps and bisected to the bit.
fn sign_changes(f: impl Fn(f64) -> f64, t0: f64, t1: f64) -> Vec<f64> {
    let n = 20_000;
    let at = |k: u32| t0 + (t1 - t0) * f64::from(k) / f64::from(n);
    let mut out = Vec::new();
    for k in 0..n {
        let (mut a, mut b) = (at(k), at(k + 1));
        if (f(a) < 0.0) == (f(b) < 0.0) {
            continue;
        }
        for _ in 0..80 {
            let m = (a + b) / 2.0;
            if (f(m) < 0.0) == (f(a) < 0.0) {
                a = m;
            } else {
                b = m;
            }
        }
        out.push((a + b) / 2.0);
    }
    out
}

/// Every crossing the oracle finds along `other`'s edges against the
/// frustum's faces.
fn oracle(f: Frustum, other: &Body<f64>) -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    for (_, e) in other.edges() {
        let c = other
            .get_curve_geom(e.curve)
            .and_then(topo::CurveGeom::certified)
            .expect("a certified edge");
        let (t0, t1) = c.params();
        out.extend(f.crossings(|t| c.carrier().eval(t), t0.min(t1), t0.max(t1)));
    }
    out
}

/// The points of `split`'s vertices that `before` did not have.
fn new_vertices(before: &Body<f64>, split: &Body<f64>) -> Vec<Point3<f64>> {
    let old: Vec<Point3<f64>> = before.vertex_points().map(|(_, p)| p).collect();
    split
        .vertex_points()
        .map(|(_, p)| p)
        .filter(|p| old.iter().all(|q| (*p - *q).norm() > 1e-12))
        .collect()
}

/// The sweep in both orders: the other operand's new vertices are
/// exactly the oracle's crossings, one for one, each within the tolerance
/// of its oracle point (a certified root is placed within the band).
fn assert_split_matches_oracle(label: &str, f: Frustum, other: &AtRestBody<f64>) {
    let tol = Tol::witness();
    let cone = f.body();
    let truth = oracle(f, other);
    assert!(!truth.is_empty(), "{label}: the pose crosses the frustum");
    let reach = tol.eps().max(1e-10);
    for (order, swapped) in [("frustum first", false), ("frustum second", true)] {
        let label = format!("{label}, {order}");
        let (a, b) = if swapped {
            (&**other, &*cone)
        } else {
            (&*cone, &**other)
        };
        let (sa, sb, _, _) = topo::sweep_split(a, b, tol)
            .unwrap_or_else(|e| panic!("{label}: the sweep refused {e:?}"));
        let split = if swapped { sa } else { sb };
        let mut got = new_vertices(other, &split);
        assert_eq!(
            got.len(),
            truth.len(),
            "{label}: {} new vertices against the oracle's {} crossings: {got:?} vs {truth:?}",
            got.len(),
            truth.len()
        );
        for q in &truth {
            let (i, d) = got
                .iter()
                .enumerate()
                .map(|(i, p)| (i, (*p - *q).norm()))
                .min_by(|x, y| x.1.total_cmp(&y.1))
                .expect("a vertex left to match");
            assert!(
                d <= reach,
                "{label}: the oracle's crossing {q:?} is {d} from the nearest new vertex"
            );
            got.swap_remove(i);
        }
    }
}

/// The sweep refuses `want` in both orders.
fn assert_sweep_refuses(
    label: &str,
    f: Frustum,
    other: &AtRestBody<f64>,
    want: fn(&BooleanError) -> bool,
) {
    let cone = f.body();
    for (order, a, b) in [
        ("frustum first", &*cone, &**other),
        ("frustum second", &**other, &*cone),
    ] {
        match topo::sweep_split(a, b, Tol::witness()) {
            Err(e) if want(&e) => {}
            other => panic!("{label}, {order}: got {:?}", other.map(|_| "a split")),
        }
    }
}

/// Widening upward: apex at `y = −1`, the face on the `v > 0` nappe.
const WIDENING: Frustum = Frustum {
    y0: 0.0,
    y1: 1.0,
    r0: 0.5,
    k: 0.5,
};

/// Narrowing upward: apex at `y = 2`, the face on the mirror nappe.
const NARROWING: Frustum = Frustum {
    y0: 0.0,
    y1: 1.0,
    r0: 1.0,
    k: -0.5,
};

/// The full cone: apex at `y = 1`, half-angle `π/4`, its wall closed at
/// the apex.
const FULL: Frustum = Frustum {
    y0: 0.0,
    y1: 1.0,
    r0: 1.0,
    k: -1.0,
};

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished(
        "the brick",
        sweep::test_support::brick(x, y, z, Tol::witness()),
        Tol::witness(),
    )
}

/// A cube of side `s` centred at `c`, its body diagonal turned onto `+y`,
/// so that every face of it meets the cone's axis at more than the
/// half-angle and none is a ruling's plane.
fn diagonal_cube(s: f64, c: [f64; 3]) -> AtRestBody<f64> {
    let h = s / 2.0;
    let b = sweep::test_support::brick((-h, h), (-h, h), (-h, h), Tol::witness());
    let d = Vec3::new(1.0, 1.0, 1.0) / 3f64.sqrt();
    let turn = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(-1.0, 0.0, 1.0) / 2f64.sqrt(),
        d.y.acos(),
    );
    let b = topo::transform_rigid(&b, &turn, Tol::witness()).unwrap();
    let to = Affine3::translation(Vec3::new(c[0], c[1], c[2]));
    finished(
        "the cube",
        topo::transform_rigid(&b, &to, Tol::witness()).unwrap(),
        Tol::witness(),
    )
}

/// A rod of radius `r` and length `h` centred at `c`, its axis `y`
/// turned by `tilt` about `z`. Its rims are circles, its wall a cylinder,
/// and its revolve seam a line.
fn rod(r: f64, h: f64, tilt: f64, c: [f64; 3]) -> AtRestBody<f64> {
    let b = sweep::test_support::prism_at(
        vec![(Point2::new(-r, 0.0), 1.0), (Point2::new(r, 0.0), 1.0)],
        -h / 2.0,
        h,
        Tol::witness(),
    );
    let onto_y = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(1.0, 0.0, 0.0),
        -core::f64::consts::FRAC_PI_2,
    );
    let b = topo::transform_rigid(&b, &onto_y, Tol::witness()).unwrap();
    let t = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), tilt);
    let b = topo::transform_rigid(&b, &t, Tol::witness()).unwrap();
    let to = Affine3::translation(Vec3::new(c[0], c[1], c[2]));
    finished(
        "the rod",
        topo::transform_rigid(&b, &to, Tol::witness()).unwrap(),
        Tol::witness(),
    )
}

/// The overlap of [`diagonal_cube`]`(0.4, [−0.75, 0.5, 0])` with
/// [`WIDENING`], by a midpoint rule over the cube's box at 800³ cells
/// outside the kernel, stable to 4e-6 from 200³.
const CUBE_ON_THE_WIDENING_WALL: f64 = 0.030_601;

/// The overlap of the same cube with [`NARROWING`], by the same midpoint
/// rule over the cube's own box, stable to 2e-8 from 400³ to 800³.
const CUBE_ON_THE_NARROWING_WALL: f64 = 0.029_738_0;

/// **Every op on a cone wall answers its truth or refuses typed.** The
/// turned cube across the widening wall and across the narrowing one,
/// placed across the frustum's seam and inside one wall face (where ∪
/// and A ∖ B keep its footprint as a ring on the cone face), builds
/// under all four ops: each
/// body's volume is its closed form from the frustum's, the cube's and
/// their overlap, it passes tier 3, and `point_in_solid` agrees with
/// both operands' closed-form membership over a grid. The tilted rod
/// across the narrowing wall is a cone × oblique cylinder pair, which
/// has no germ frame: every op refuses there, naming the pair.
#[test]
fn every_op_on_a_cone_wall_answers_its_truth_or_refuses_typed() {
    let tol = Tol::witness();
    let d = Vec3::new(1.0, 1.0, 1.0) / 3f64.sqrt();
    for (place, c, (lo, hi)) in [
        (
            "across the seam",
            [-0.75, 0.5, 0.0],
            (Point3::new(-1.1, 0.0, -0.4), Point3::new(-0.4, 1.0, 0.4)),
        ),
        (
            "inside one wall face",
            [0.0, 0.5, -0.75],
            (Point3::new(-0.4, 0.0, -1.1), Point3::new(0.4, 1.0, -0.4)),
        ),
    ] {
        let cube = diagonal_cube(0.4, c);
        let turned = Solid::Brick([(-0.2, 0.2); 3]).posed(
            Affine3::translation(Vec3::new(c[0], c[1], c[2]))
                * Affine3::rotation_about_axis(
                    Point3::origin(),
                    Vec3::new(-1.0, 0.0, 1.0) / 2f64.sqrt(),
                    d.y.acos(),
                ),
        );
        let points = solid_truth::grid(lo, hi, 7);
        for (wall, f, overlap) in [
            ("widening", WIDENING, CUBE_ON_THE_WIDENING_WALL),
            ("narrowing", NARROWING, CUBE_ON_THE_NARROWING_WALL),
        ] {
            let cone = f.body();
            let frustum = Solid::Revolved {
                y0: f.y0,
                y1: f.y1,
                r0: f.r0,
                k: f.k,
                window: None,
            };
            let (vb, vi) = (0.064, overlap);
            // The frustum's volume, `π·h·(r₀² + r₀·r₁ + r₁²)/3`.
            let va =
                PI * (f.y1 - f.y0) / 3.0 * (f.r0.powi(2) + f.r0 * f.r(f.y1) + f.r(f.y1).powi(2));
            for (op_label, got, op, x, y, v) in [
                (
                    "∪",
                    topo::union(&cone, &cube, tol),
                    Op::Union,
                    &frustum,
                    &turned,
                    va + vb - vi,
                ),
                (
                    "∩",
                    topo::intersect(&cone, &cube, tol),
                    Op::Intersect,
                    &frustum,
                    &turned,
                    vi,
                ),
                (
                    "A ∖ B",
                    topo::subtract(&cone, &cube, tol),
                    Op::Subtract,
                    &frustum,
                    &turned,
                    va - vi,
                ),
                (
                    "B ∖ A",
                    topo::subtract(&cube, &cone, tol),
                    Op::Subtract,
                    &turned,
                    &frustum,
                    vb - vi,
                ),
            ] {
                // ∪ and A ∖ B keep a cone face bounded by the cube's tilted
                // sections, whose trim `point_in_solid` cannot always read
                // (`PartialConeFace`); inside one wall face, that face
                // carries the cube's footprint as a ring. On the widening wall
                // one grid point refuses there; on the narrowing wall 153 do,
                // and 152 in ∩ and B ∖ A. Measured in both places at every ε
                // row.
                let keeps_the_wall = matches!(op_label, "∪" | "A ∖ B");
                let partial = match (wall, keeps_the_wall) {
                    ("widening", true) => 1,
                    ("widening", false) => 0,
                    (_, true) => 153,
                    (_, false) => 152,
                };
                let rings = usize::from(keeps_the_wall && place == "inside one wall face");
                assert_eq!(
                    ringed_cone_faces(&got),
                    rings,
                    "the turned cube {place} on the {wall} wall, {op_label}: ringed cone faces"
                );
                solid_truth::assert_is_but(
                    &format!("the turned cube {place} on the {wall} wall, {op_label}"),
                    &got,
                    Want::Body(v, 1e-5),
                    &|q| op.depth(x, y, q),
                    &[],
                    &points,
                    partial,
                );
            }
        }
    }
    let narrowing = NARROWING.body();
    let rod = rod(0.12, 0.5, 0.3, [-0.75, 0.5, 0.0]);
    for (op_label, op, a, b) in [
        ("∪", BooleanOp::Union, &narrowing, &rod),
        ("∩", BooleanOp::Intersect, &narrowing, &rod),
        ("A ∖ B", BooleanOp::Subtract, &narrowing, &rod),
        ("B ∖ A", BooleanOp::Subtract, &rod, &narrowing),
    ] {
        let got = match op {
            BooleanOp::Union => topo::boolean::union(a, b, tol),
            BooleanOp::Intersect => topo::boolean::intersect(a, b, tol),
            BooleanOp::Subtract => topo::boolean::subtract(a, b, tol),
        };
        assert!(
            matches!(
                got,
                Err(BooleanError::GermFrameUnsupported { a_kind, b_kind, .. })
                    if matches!(
                        [a_kind, b_kind],
                        [geom::SurfaceKind::Cone, geom::SurfaceKind::Cylinder]
                            | [geom::SurfaceKind::Cylinder, geom::SurfaceKind::Cone]
                    )
            ),
            "the tilted rod, {op_label}: got {:?}",
            got.map(|r| r.body().map(|b| b.kind))
        );
    }
}

/// **Lines crossing a cone wall**: a cube turned so no face of it is a
/// ruling's plane, straddling the wall of each nappe's frustum. Its
/// twelve edges cross the wall where the oracle says, and nowhere else.
/// On the base every such pair refused `CurvedPierceUnsupported`.
#[test]
fn lines_crossing_a_cone_wall_split_where_the_oracle_crosses() {
    for (label, f) in [("widening", WIDENING), ("narrowing", NARROWING)] {
        for c in [[-0.75, 0.5, 0.0], [0.1, 0.45, -0.72]] {
            let label = format!("a cube at {c:?} across the {label} wall");
            assert_split_matches_oracle(&label, f, &diagonal_cube(0.4, c));
        }
    }
}

/// **Circles and a ruling crossing a cone wall**: rods tilted across it,
/// their rims arcs of circles and their seams lines; and a rod coaxial
/// with the cone through its top cap, whose rims' residual against the
/// wall is constant (the conic door's first-harmonic arm) and whose
/// wall and seam cross only the cap.
#[test]
fn arcs_crossing_a_cone_wall_split_where_the_oracle_crosses() {
    for (label, f, other) in [
        (
            "a tilted rod across the widening wall",
            WIDENING,
            rod(0.12, 0.5, 0.3, [-0.75, 0.5, 0.0]),
        ),
        (
            "a tilted rod across the narrowing wall",
            NARROWING,
            rod(0.12, 0.5, -0.4, [-0.75, 0.5, 0.0]),
        ),
        (
            "a steep rod across the widening wall",
            WIDENING,
            rod(0.1, 0.6, 1.2, [0.0, 0.55, 0.75]),
        ),
        (
            "a coaxial rod through the top cap",
            WIDENING,
            rod(0.3, 0.6, 0.0, [0.0, 1.0, 0.0]),
        ),
    ] {
        assert_split_matches_oracle(label, f, &other);
    }
}

/// **A segment from inside one nappe to inside the other.** A thin
/// brick beside the full cone's axis runs from inside the cone, past its
/// apex, into the mirror nappe. Both ends of each long edge read inside
/// the double cone, and between them the edge crosses the cone's wall
/// just below the apex and the mirror nappe just above it. The convexity
/// argument that clears a line with both ends inside a wall or a sphere
/// does not hold for a cone, whose residual carries `−|h|`: read so, the
/// wall crossing was skipped. The mirror nappe's root is told off.
#[test]
fn a_segment_between_the_nappes_crosses_the_wall_below_the_apex() {
    let other = brick((-0.03, -0.01), (0.5, 1.5), (-0.01, 0.01));
    assert_split_matches_oracle("a thin brick past the full cone's apex", FULL, &other);
}

/// **Roots on the far nappe** are not the face's: a thin brick below
/// the widening frustum's floor, reaching down past the cone's apex,
/// crosses the double cone twice per long edge outside the face's
/// window, once on each nappe, and the floor once. Only the floor is
/// crossed.
#[test]
fn roots_off_the_faces_window_and_nappe_are_not_its_crossings() {
    let other = brick((0.04, 0.06), (-2.5, 0.5), (-0.01, 0.01));
    assert_split_matches_oracle("a thin brick through the apex region", WIDENING, &other);
}

/// **An edge through the apex** refuses typed: the brick's corner sits
/// at the full cone's apex, so three of its edges leave the apex.
/// Neither a crossing nor a miss can be read there.
#[test]
fn an_edge_through_the_apex_refuses_at_the_apex() {
    let other = brick((0.0, 0.4), (1.0, 1.4), (0.0, 0.4));
    assert_sweep_refuses("a brick cornered at the apex", FULL, &other, |e| {
        matches!(e, BooleanError::CrossingAtConeApex { .. })
    });
}

/// **A graze keeps the frontier**: a brick whose bottom edge runs
/// tangent to the full cone's wall at `y = 0.8`, where its radius is
/// `0.2`.
#[test]
fn an_edge_grazing_the_wall_keeps_the_frontier() {
    let other = brick((-0.2, 0.2), (0.8, 1.2), (0.2, 0.6));
    assert_sweep_refuses("a brick grazing the wall", FULL, &other, |e| {
        matches!(e, BooleanError::CurvedPierceUnsupported { .. })
    });
}

/// How many of `got`'s body's cone faces carry a ring.
fn ringed_cone_faces(got: &Result<topo::BooleanResult<f64>, BooleanError>) -> usize {
    let Ok(topo::BooleanResult::Body(bb)) = got else {
        return 0;
    };
    bb.body
        .faces()
        .filter(|&(k, f)| {
            !f.rings.is_empty()
                && topo::query::face_surface_kind(&bb.body, k) == Some(geom::SurfaceKind::Cone)
        })
        .count()
}

/// The frustum the cone-ring fuzz drew as seed 15 at ε = 1e-12.
const SEED_15: Frustum = Frustum {
    y0: 0.0,
    y1: 19.689,
    r0: 3.6339,
    k: 0.5513,
};

/// The cube's side, centre and turn for [`SEED_15`]: side 4.748, its
/// centre 0.08 outside the wall at `y = 15.2`, azimuth 3.952 from `+x`
/// towards `+z`, turned 1.2567 about `(−0.342, 0.22, −0.0995)`.
fn seed_15_cube() -> (f64, Point3<f64>, Affine3<f64>) {
    let (y, az, out) = (15.2f64, 3.952f64, 0.08);
    let r = SEED_15.r(y) + out;
    let axis = Vec3::new(-0.342, 0.22, -0.0995);
    let turn = Affine3::rotation_about_axis(Point3::origin(), axis / axis.norm(), 1.2567);
    (4.748, Point3::new(r * az.cos(), y, r * az.sin()), turn)
}

/// The overlap of the cube [`seed_15_cube`] with [`SEED_15`], by a
/// midpoint rule over the cube's own coordinates at 1600³ cells outside
/// the kernel, stable to 4e-6 from 800³.
const SEED_15_OVERLAP: f64 = 50.088_746;

/// **A ring on a cone wall whose winding sums to rounding answers at
/// every ε row.** The cone-ring fuzz's seed 15: a turned cube through
/// the wall of a frustum, inside one wall face, so ∪ and A ∖ B keep
/// its footprint as a ring on the cone face. Re-derived in interval
/// arithmetic for check 7, the ring's winding about the axis encloses
/// zero to ±1.7e-12, and a zero test of it escalated at ε = 1e-12;
/// decided against half a turn, it does not. Every op builds, its
/// volume the closed form from the frustum's, the cube's and their
/// overlap, through tier 3.
#[test]
fn a_ring_on_a_cone_wall_whose_winding_sums_to_rounding_answers() {
    let tol = Tol::witness();
    let cone = SEED_15.body();
    let (side, c, turn) = seed_15_cube();
    let h = side / 2.0;
    let raw = sweep::test_support::brick((-h, h), (-h, h), (-h, h), tol);
    let to = Affine3::translation(c - Point3::origin());
    let cube = finished(
        "the cube",
        topo::transform_rigid(&raw, &(to * turn), tol).unwrap(),
        tol,
    );
    let f = SEED_15;
    let va = PI * (f.y1 - f.y0) / 3.0 * (f.r0.powi(2) + f.r0 * f.r(f.y1) + f.r(f.y1).powi(2));
    let (vb, vi) = (side.powi(3), SEED_15_OVERLAP);
    for (op, got, want, rings) in [
        ("∪", topo::union(&cone, &cube, tol), va + vb - vi, 1),
        ("∩", topo::intersect(&cone, &cube, tol), vi, 0),
        ("A ∖ B", topo::subtract(&cone, &cube, tol), va - vi, 1),
        ("B ∖ A", topo::subtract(&cube, &cone, tol), vb - vi, 0),
    ] {
        let bb = match got.as_ref().map(topo::BooleanResult::body) {
            Ok(Some(bb)) => bb,
            _ => panic!("{op}: wanted a body, got {got:?}"),
        };
        assert_eq!(ringed_cone_faces(&got), rings, "{op}: ringed cone faces");
        topo::validate_geometric(&bb.body, tol).unwrap_or_else(|e| panic!("{op}: tier 3: {e:?}"));
        let v = topo::mass_properties(&bb.body, tol)
            .unwrap_or_else(|e| panic!("{op}: the volume refused {e:?}"))
            .volume;
        assert!((v - want).abs() <= 1e-5, "{op}: volume {v} against {want}");
    }
}
