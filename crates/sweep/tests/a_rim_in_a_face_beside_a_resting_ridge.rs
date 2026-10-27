//! **A rim in a face beside a resting ridge, and an edge in a face run
//! into a wall's foot**: the poses that exercise the conic and curved
//! split sites of the sweep's worklist, which carries the planar faces
//! an edge lies in (`reduce::Pending`).
//!
//! - **A rim in the block's top beside a ridge resting on it.** The line
//!   body is the block `[-1, 1]² × [0, 1]` and a ridge, a triangular
//!   prism along y whose bottom edge rests on the block's top at
//!   `x = 0`. A cylinder (r 0.5) on the top, or buried with its top rim
//!   in it, meets it in every op, both slots and both groupings. Its rim
//!   lies in the block's top, and the ridge's faces split it there. The
//!   rim's in-plane face is not carried, so every cell answers as on
//!   the merge base: where the cylinder is met first in the block-first
//!   grouping, ∩ and ∖ build at their closed forms, and every other
//!   cell refuses typed.
//! - **A box's edges in the block's top, run into a wall's foot.** A
//!   cylinder stands on the block's top, or rises through it. A box
//!   resting on the top runs its bottom edges into the wall's foot,
//!   where the wall splits them after the top has read their ends, and
//!   every op builds in either slot, tier 3 clean, at
//!   inclusion–exclusion.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, finished, prism_at};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanError, BooleanErrorKind, BooleanResult};

/// The cylinders' radius.
const R: f64 = 0.5;

fn t() -> Tol {
    Tol::witness()
}

/// A z-axis cylinder of radius [`R`] about `c`, from `z0`, length `len`.
fn rod(c: (f64, f64), z0: f64, len: f64) -> AtRestBody<f64> {
    let lp = profile::circle(Point2::new(c.0, c.1), R, t()).unwrap();
    let plane = profile::SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let p = profile::Profile::new(plane, vec![lp.into()])
        .validate(t())
        .unwrap();
    let rod = extrude(
        &p,
        Extrusion::Distance {
            depth: len,
            side: ExtrudeSide::Along,
        },
        t(),
    )
    .unwrap()
    .body;
    finished("the rod", rod, t())
}

/// The block `[-1, 1]² × [0, 1]`.
fn block() -> AtRestBody<f64> {
    finished(
        "the block",
        brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), t()),
        t(),
    )
}

/// The ridge: `|x| ≤ (z − 1) / 2`, `z ≤ 2`, `|y| ≤ 1`, its bottom edge
/// along y at `x = 0` on the block's top.
fn ridge() -> AtRestBody<f64> {
    let v = |x: f64, y: f64| (Point2::new(x, y), 0.0);
    let p = prism_at(vec![v(0.0, 1.0), v(0.5, 2.0), v(-0.5, 2.0)], -1.0, 2.0, t());
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI / 2.0);
    let p = topo::transform_rigid(&p, &geom_core::test_support::rigid(&turn, t()), t()).unwrap();
    finished("the ridge", p, t())
}

/// `∫ f(x) dx` over `[a, b]` by the midpoint rule: the integrands here
/// are continuous with at worst a square-root end, so 2·10⁵ panels hold
/// them well inside the `1e-7` the rows assert.
fn integral(a: f64, b: f64, f: impl Fn(f64) -> f64) -> f64 {
    let n = 200_000;
    let h = (b - a) / n as f64;
    (0..n).map(|i| f(a + (i as f64 + 0.5) * h)).sum::<f64>() * h
}

/// The half-chord of the disc of radius [`R`] about `c` at `x`.
fn half_chord(c: (f64, f64), x: f64) -> f64 {
    (R * R - (x - c.0).powi(2)).max(0.0).sqrt()
}

type Op = fn(&AtRestBody<f64>, &AtRestBody<f64>, Tol) -> Result<BooleanResult<f64>, BooleanError>;

const OPS: [(&str, Op); 3] = [
    ("intersection", topo::intersect),
    ("subtraction", topo::subtract),
    ("union", topo::union),
];

/// What a cell answers: a body of this volume, the empty body, or a
/// refusal of this kind.
#[derive(Clone, Copy, Debug)]
enum Answer {
    Body(f64),
    Empty,
    Refuses(BooleanErrorKind),
}

fn answers(what: &str, got: Result<BooleanResult<f64>, BooleanError>, want: Answer) {
    match (got, want) {
        (Ok(BooleanResult::Body(r)), Answer::Body(volume)) => {
            topo::validate_geometric(&r.body, t())
                .unwrap_or_else(|e| panic!("{what}: tier 3: {e:?}"));
            let v = topo::test_support::pinned_volume("the measured body", &r.body, t()).volume;
            assert!(
                (v - volume).abs() < 1e-7,
                "{what}: volume {v}, closed form {volume}"
            );
        }
        (Ok(BooleanResult::Empty), Answer::Empty) => {}
        (Err(e), Answer::Refuses(kind)) => assert_eq!(e.kind(), kind, "{what}: {e:?}"),
        (other, want) => panic!("{what}: {other:?}, where it answers {want:?}"),
    }
}

/// **A rim in the block's top beside a ridge resting on it answers in
/// every cell as on the merge base** (module docs). It pins main's
/// answers, the slot-asymmetric refusals among them, which
/// `work/tang/a-vertex-minted-on-an-edge-lying-on-a-curved-face-is-unread.md`
/// files as defects: fixing that re-baselines this row.
#[test]
fn a_rim_in_a_face_beside_a_resting_ridge_answers_every_op_as_before() {
    use Answer::{Body, Empty, Refuses};
    use BooleanErrorKind::{Join, JoinDesync, PartnerAlongFace, VertexReadTwice};
    let (b, rd) = (block(), ridge());
    let groupings = [(true, &b, &rd), (false, &rd, &b)];
    for (block_first, p, q) in groupings {
        let line = match topo::union(p, q, t()).unwrap() {
            BooleanResult::Body(r) => r.body,
            other => panic!("the line body: {other:?}"),
        };
        for (c, above) in [
            ((0.3, 0.0), true),
            ((0.3, 0.2), true),
            ((0.3, 0.0), false),
            ((-0.2, 0.1), false),
        ] {
            let tool = if above {
                rod(c, 1.0, 0.4)
            } else {
                rod(c, 0.6, 0.4)
            };
            let solo = PI * R * R * 0.4;
            // The ridge's part in the cylinder above the top: its height
            // over the disc runs from `2|x|` to the cylinder's top.
            let common = if above {
                integral(-0.2, 0.2, |x| {
                    (0.4 - 2.0 * x.abs()) * 2.0 * half_chord(c, x)
                })
            } else {
                solo
            };
            // [∩, ∖, ∪] with the cylinder first, then with the line body first.
            let [tool_first, line_first] = match (block_first, above) {
                (true, true) => [
                    [Body(common), Body(solo - common), Refuses(JoinDesync)],
                    [
                        Refuses(PartnerAlongFace),
                        Refuses(PartnerAlongFace),
                        Refuses(VertexReadTwice),
                    ],
                ],
                (true, false) => [
                    [Body(common), Empty, Refuses(Join)],
                    [
                        Refuses(VertexReadTwice),
                        Refuses(VertexReadTwice),
                        Refuses(PartnerAlongFace),
                    ],
                ],
                (false, true) => [
                    [
                        Refuses(PartnerAlongFace),
                        Refuses(PartnerAlongFace),
                        Refuses(VertexReadTwice),
                    ],
                    [
                        Refuses(PartnerAlongFace),
                        Refuses(PartnerAlongFace),
                        Refuses(VertexReadTwice),
                    ],
                ],
                (false, false) => [
                    [
                        Refuses(PartnerAlongFace),
                        Refuses(PartnerAlongFace),
                        Refuses(VertexReadTwice),
                    ],
                    [
                        Refuses(VertexReadTwice),
                        Refuses(VertexReadTwice),
                        Refuses(PartnerAlongFace),
                    ],
                ],
            };
            for (i, (name, op)) in OPS.into_iter().enumerate() {
                let what = format!(
                    "{name} of the cylinder about {c:?} {} the top, block first: {block_first}",
                    if above { "on" } else { "under" }
                );
                answers(
                    &format!("{what}, the cylinder first"),
                    op(&tool, &line, t()),
                    tool_first[i],
                );
                answers(
                    &format!("{what}, the line body first"),
                    op(&line, &tool, t()),
                    line_first[i],
                );
            }
        }
    }
}

/// A box: its x, y and z spans.
type Box3 = ((f64, f64), (f64, f64), (f64, f64));

/// Each box of `boxes` against the block and a cylinder on its top or
/// through it, per pose of `poses` (cylinder first in the union, its
/// foot, its length), by each op of `ops` in both slots: each builds
/// at inclusion–exclusion, tier 3 clean.
fn edges_run_into_a_walls_foot(poses: &[(bool, f64, f64)], boxes: &[Box3], ops: &[(&str, Op)]) {
    let b = block();
    for &(cylinder_first, z0, len) in poses {
        let c = rod((0.0, 0.0), z0, len);
        let y = if cylinder_first {
            topo::union(&c, &b, t())
        } else {
            topo::union(&b, &c, t())
        };
        let y = match y.unwrap() {
            BooleanResult::Body(r) => r.body,
            other => panic!("the block and cylinder: {other:?}"),
        };
        let y_volume = 4.0 + PI * R * R;
        for &(bx, by, bz) in boxes {
            let x = finished("the box", brick(bx, by, bz, t()), t());
            let x_volume = (bx.1 - bx.0) * (by.1 - by.0) * (bz.1 - bz.0);
            let common = if bz.0 >= 1.0 {
                // Above the top: the box's part in the cylinder.
                let width = |x: f64| {
                    let s = half_chord((0.0, 0.0), x);
                    (by.1.min(s) - by.0.max(-s)).max(0.0)
                };
                integral(bx.0.max(-R), bx.1.min(R), width) * (bz.1 - bz.0)
            } else {
                // Under the top: the box's part in the block.
                (bx.1.min(1.0) - bx.0.max(-1.0)) * (by.1 - by.0) * (bz.1 - bz.0)
            };
            let what = format!(
                "the box {bx:?} × {by:?} × {bz:?}, the cylinder from {z0}, cylinder first: {cylinder_first}"
            );
            for x_first in [true, false] {
                let (p, q, p_volume) = if x_first {
                    (&x, &y, x_volume)
                } else {
                    (&y, &x, y_volume)
                };
                for &(name, op) in ops {
                    let volume = match name {
                        "intersection" => common,
                        "subtraction" => p_volume - common,
                        _ => x_volume + y_volume - common,
                    };
                    answers(
                        &format!("{name} of {what}, the box first: {x_first}"),
                        op(p, q, t()),
                        Answer::Body(volume),
                    );
                }
            }
        }
    }
}

/// **A box's edges in the block's top, run into a wall's foot, build in
/// every op** (module docs). Slow set; its per-PR pose is the row below.
#[test]
fn edges_in_a_face_run_into_a_walls_foot_build_every_op() {
    edges_run_into_a_walls_foot(
        &[
            (false, 1.0, 1.0),
            (true, 1.0, 1.0),
            (false, 0.5, 1.5),
            (true, 0.5, 1.5),
        ],
        &[
            ((-0.2, 2.0), (-0.1, 0.1), (1.0, 1.3)),
            ((-2.0, 2.0), (-0.1, 0.1), (1.0, 1.3)),
            ((-0.2, 2.0), (0.3, 0.6), (1.0, 1.3)),
            ((0.0, 2.0), (-0.1, 0.1), (0.5, 1.0)),
        ],
        &OPS,
    );
}

/// **The per-PR pose of the row above**: one box run into the foot of a
/// cylinder standing on the block, intersected in both slots. With the
/// box first, a wall splits its bottom edges after the block's top read
/// their ends, so the vertex the wall mints is read against the top
/// (`reduce::read_lying_in`); read at the wrong point there, this pose
/// refuses where it builds.
#[test]
fn an_edge_in_a_face_run_into_a_walls_foot_builds() {
    edges_run_into_a_walls_foot(
        &[(false, 1.0, 1.0)],
        &[((-0.2, 2.0), (-0.1, 0.1), (1.0, 1.3))],
        &OPS[..1],
    );
}
