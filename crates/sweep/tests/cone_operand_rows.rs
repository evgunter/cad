//! **The cone as a boolean operand**: every op on the preview cone of
//! the cone admission's spec (its §0,
//! `docs/doc-ledger/germ-verbs-cone-spec.md`) against its fixtures, through the
//! public doors.
//!
//! The preview cone is the triangle `(0,0) (1,0) (0,1)` revolved fully
//! about `y` and merged to one cone face: apex `(0, 1, 0)`, half-angle
//! `π/4`, volume `π/3`. Its partial revolves through `3π/2` and `π/2`
//! sweep from `+x` toward `−z`.
//!
//! Every body an op returns is held to [`solid_truth::assert_is`]: its
//! closed-form volume, tier 3, and `point_in_solid` at named points (one
//! per region of the pair) and over a grid, against the operands'
//! closed-form membership. Every pose that does not build refuses typed,
//! and the rows name the door. The touching configurations — a plane
//! resting along a generator, a cone sector lying on the cone face —
//! stay refusals.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::solid_truth::{self, Op, Solid, Want};
use crate::revolve_common::{axis_y, validated};

use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::{brick, finished};
use sweep::{Revolution, revolve};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

/// The preview cone's volume.
const V: f64 = PI / 3.0;

fn revolved(polygon: &[(f64, f64)], theta: Option<f64>) -> AtRestBody<f64> {
    let loop_ = ProfileLoop::polygon(polygon.iter().map(|&(x, y)| Point2::new(x, y)));
    let mut body = revolve(
        &validated(vec![loop_]),
        axis_y(),
        theta.map_or(Revolution::Full, Revolution::Partial),
        Tol::witness(),
    )
    .unwrap()
    .body;
    if theta.is_none() {
        body.merge_coplanar_faces(Tol::witness()).unwrap();
    }
    finished("the revolve", body, Tol::witness())
}

/// The preview cone, or its sector through `theta`.
fn cone(theta: Option<f64>) -> (AtRestBody<f64>, Solid) {
    (
        revolved(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)], theta),
        Solid::Revolved {
            y0: 0.0,
            y1: 1.0,
            r0: 1.0,
            k: -1.0,
            window: theta,
        },
    )
}

fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> (AtRestBody<f64>, Solid) {
    let tol = Tol::witness();
    (
        finished("the brick", brick(x, y, z, tol), tol),
        Solid::Brick([x, y, z]),
    )
}

fn moved(body: &Body<f64>, map: &Affine3<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    finished(
        "the moved body",
        topo::transform_rigid(body, map, tol).unwrap(),
        tol,
    )
}

/// ∪, ∩, A ∖ B and B ∖ A of `a` and `b`, labelled.
fn every_op(
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
) -> [(&'static str, Result<BooleanResult<f64>, BooleanError>); 4] {
    let tol = Tol::witness();
    [
        ("∪", topo::union(a, b, tol)),
        ("∩", topo::intersect(a, b, tol)),
        ("A ∖ B", topo::subtract(a, b, tol)),
        ("B ∖ A", topo::subtract(b, a, tol)),
    ]
}

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// One fixture that builds under every op: A's and B's volumes and
/// their overlap, and a point in each region of the pair.
struct Builds {
    what: &'static str,
    a: (AtRestBody<f64>, Solid),
    b: (AtRestBody<f64>, Solid),
    /// `(V(A), V(B), V(A ∩ B))`, in closed form.
    volumes: (f64, f64, f64),
    /// Points in the regions of the pair.
    named: Vec<Point3<f64>>,
    /// The box the grid covers.
    region: (Point3<f64>, Point3<f64>),
}

impl Builds {
    /// Every op's body against its closed form and its truth.
    fn check(&self) {
        let (va, vb, vi) = self.volumes;
        let ((a, sa), (b, sb)) = (&self.a, &self.b);
        let want = |v: f64| {
            if v.abs() < 1e-12 {
                Want::Empty
            } else {
                Want::Body(v, 1e-9 * v.max(1.0))
            }
        };
        let mut points = solid_truth::grid(p(-1.05, -0.05, -1.05), p(1.05, 1.05, 1.05), 7);
        points.extend(solid_truth::grid(self.region.0, self.region.1, 5));
        let ops: [(Op, &Solid, &Solid, f64); 4] = [
            (Op::Union, sa, sb, va + vb - vi),
            (Op::Intersect, sa, sb, vi),
            (Op::Subtract, sa, sb, va - vi),
            (Op::Subtract, sb, sa, vb - vi),
        ];
        for ((label, got), (op, x, y, v)) in every_op(a, b).into_iter().zip(ops) {
            solid_truth::assert_is(
                &format!("{}, {label}", self.what),
                &got,
                want(v),
                &|q| op.depth(x, y, q),
                &self.named,
                &points,
            );
        }
    }
}

/// **No section to join: nested, inside, clear.** P9, the cone in the
/// 6³ box (∪ the box, ∩ the cone, A ∖ B empty, B ∖ A the box voided);
/// P6, a box `0.6² × 0.2` strictly inside; P4, the brick
/// `[−2, 2] × [0.2, 0.3] × [0.9, 1.2]` clear of the cone with the
/// boxes overlapping; the `3π/2` sector against a brick in its gap
/// quadrant; the quarter cone (P8) against a brick inside its box,
/// clear of it.
#[test]
fn nested_inside_and_clear_poses_build_their_closed_forms() {
    let fixtures = [
        Builds {
            what: "P9",
            a: cone(None),
            b: boxed((-3.0, 3.0), (-3.0, 3.0), (-3.0, 3.0)),
            volumes: (V, 216.0, V),
            named: vec![p(0.0, 0.3, 0.0), p(2.0, 2.0, 2.0), p(4.0, 0.0, 0.0)],
            region: (p(-3.1, -3.1, -3.1), p(3.1, 3.1, 3.1)),
        },
        Builds {
            what: "P6",
            a: cone(None),
            b: boxed((-0.3, 0.3), (0.2, 0.4), (-0.3, 0.3)),
            volumes: (V, 0.072, 0.072),
            named: vec![p(0.0, 0.3, 0.0), p(0.6, 0.1, 0.0), p(2.0, 0.5, 0.0)],
            region: (p(-0.4, 0.1, -0.4), p(0.4, 0.5, 0.4)),
        },
        Builds {
            what: "P4",
            a: cone(None),
            b: boxed((-2.0, 2.0), (0.2, 0.3), (0.9, 1.2)),
            volumes: (V, 0.12, 0.0),
            named: vec![p(0.0, 0.5, 0.0), p(0.0, 0.25, 1.0), p(0.0, 0.25, 0.85)],
            region: (p(-2.1, 0.1, 0.0), p(2.1, 0.4, 1.3)),
        },
        Builds {
            what: "the 3π/2 sector and a brick in its gap",
            a: cone(Some(1.5 * PI)),
            b: boxed((0.3, 0.6), (0.1, 0.3), (0.1, 0.4)),
            volumes: (PI / 4.0, 0.018, 0.0),
            named: vec![p(-0.3, 0.2, 0.0), p(0.45, 0.2, 0.25), p(0.2, 0.2, 0.05)],
            region: (p(0.0, 0.0, 0.0), p(0.7, 0.4, 0.5)),
        },
        Builds {
            what: "P8, the quarter cone and a brick in its box",
            a: cone(Some(0.5 * PI)),
            b: boxed((0.55, 0.8), (0.55, 0.8), (-0.3, -0.1)),
            volumes: (PI / 12.0, 0.0125, 0.0),
            named: vec![p(0.3, 0.2, -0.3), p(0.7, 0.7, -0.2), p(-0.3, 0.2, 0.3)],
            region: (p(0.4, 0.4, -0.4), p(0.9, 0.9, 0.0)),
        },
    ];
    for f in &fixtures {
        f.check();
    }
}

/// **The section crosses the base disc or runs round the axis.** P2a,
/// the brick `[−0.7, −0.5] × [−0.3, 0.1] × [−0.1, 0.1]` through the base
/// disc (its part above `y = 0` lies inside the cone: `0.004`); P10, the
/// coaxial pin `r = 0.1` over `y ∈ [−0.5, 0.5]` (`0.005π` inside). The
/// named points include `(−0.6, 0.05, 0)` and `(0, 0.25, 0)`, in both
/// operands.
#[test]
fn a_pin_through_the_base_and_a_coaxial_pin_build_their_closed_forms() {
    let pin = (
        revolved(&[(0.0, -0.5), (0.1, -0.5), (0.1, 0.5), (0.0, 0.5)], None),
        Solid::Revolved {
            y0: -0.5,
            y1: 0.5,
            r0: 0.1,
            k: 0.0,
            window: None,
        },
    );
    let fixtures = [
        Builds {
            what: "P2a",
            a: cone(None),
            b: boxed((-0.7, -0.5), (-0.3, 0.1), (-0.1, 0.1)),
            volumes: (V, 0.016, 0.004),
            named: vec![
                p(-0.6, 0.05, 0.0),
                p(0.5, 0.2, 0.0),
                p(-0.6, -0.2, 0.0),
                p(-0.6, 0.2, 0.5),
            ],
            region: (p(-0.8, -0.4, -0.2), p(-0.4, 0.2, 0.2)),
        },
        Builds {
            what: "P10",
            a: cone(None),
            b: pin,
            volumes: (V, 0.01 * PI, 0.005 * PI),
            named: vec![
                p(0.0, 0.25, 0.0),
                p(0.5, 0.2, 0.0),
                p(0.0, -0.25, 0.0),
                p(0.0, 0.9, 0.5),
            ],
            region: (p(-0.15, -0.6, -0.15), p(0.15, 0.6, 0.15)),
        },
    ];
    for f in &fixtures {
        f.check();
    }
}

/// **The axis-normal slab joins on its circles.** P5, the slab
/// `y ∈ [0.3, 0.6]` across the cone (∩ the frustum
/// `π·0.3/3·(0.49 + 0.28 + 0.16) = 0.093π`); P7, the `3π/2` sector
/// against the same slab (∩ three quarters of it), its apex-closed
/// sector placed by the closure lift.
#[test]
fn the_axis_normal_slab_builds_its_closed_forms() {
    let slab = || boxed((-2.0, 2.0), (0.3, 0.6), (-2.0, 2.0));
    let region = (p(-2.1, 0.2, -2.1), p(2.1, 0.7, 2.1));
    let fixtures = [
        Builds {
            what: "P5",
            a: cone(None),
            b: slab(),
            volumes: (V, 4.8, 0.093 * PI),
            named: vec![
                p(0.0, 0.45, 0.0),
                p(0.0, 0.1, 0.0),
                p(1.5, 0.45, 0.0),
                p(0.0, 0.8, 0.5),
            ],
            region,
        },
        Builds {
            what: "P7",
            a: cone(Some(1.5 * PI)),
            b: slab(),
            volumes: (PI / 4.0, 4.8, 0.75 * 0.093 * PI),
            named: vec![
                p(-0.2, 0.45, 0.0),
                p(-0.3, 0.1, 0.0),
                p(0.3, 0.45, 0.3),
                p(0.3, 0.1, 0.3),
            ],
            region,
        },
    ];
    for f in &fixtures {
        f.check();
    }
}

/// **P3, the apex pin, never returns a body.** The pin
/// `[−0.02, 0.02] × [0.5, 1.5] × [0.02, 0.06]` runs past the apex: its
/// four long edges go from inside one nappe to inside the other and
/// cross the lateral face near the apex, and its faces along the axis
/// cut the cone in hyperbolas. Every op refuses at the germ pair's frame,
/// naming the hyperbola.
///
/// The row is red against the mutant "the line × cone roots guard
/// reverted" (`reduce.rs`'s same-side arm taking `Torus` alone), which
/// clears those edges on the convexity a double cone lacks. The op then
/// takes the no-crossings path, the section certificate clears every
/// pair (hyperbolas W1, the parallel at `y = 0.5` W2), and the op
/// returns tier-3-valid wrong bodies: measured, ∪ an `Assembly` of
/// `π/3 + 0.0016`, ∩ `Empty`, A ∖ B the cone whole.
#[test]
fn the_apex_pin_refuses_every_op_never_a_body() {
    let (c, _) = cone(None);
    let (pin, _) = boxed((-0.02, 0.02), (0.5, 1.5), (0.02, 0.06));
    for (op, got) in every_op(&c, &pin) {
        assert!(
            matches!(
                &got,
                Err(BooleanError::GermSectionOutsideInventory {
                    conic: geom_brep::OutsideConic::Hyperbola,
                    ..
                })
            ),
            "P3, {op}: the hyperbola's refusal, never a body: {:?}",
            got.map(|r| r.body().map(|b| b.kind))
        );
    }
}

/// A solid of revolution about `y` posed by the rigid map
/// `rotation about z by tilt, then translation by at`.
fn posed(
    polygon: &[(f64, f64)],
    truth: Solid,
    tilt: f64,
    at: Vec3<f64>,
) -> (AtRestBody<f64>, Solid) {
    let map = Affine3::translation(at)
        * Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), tilt);
    (moved(&revolved(polygon, None), &map), truth.posed(map))
}

/// A rod of radius `r` along `y` over `[−h, h]`, posed.
fn rod(r: f64, h: f64, tilt: f64, at: Vec3<f64>) -> (AtRestBody<f64>, Solid) {
    let truth = Solid::Revolved {
        y0: -h,
        y1: h,
        r0: r,
        k: 0.0,
        window: None,
    };
    posed(&[(0.0, -h), (r, -h), (r, h), (0.0, h)], truth, tilt, at)
}

/// The cone of base radius `r` and height `h` (apex up `y`), posed.
fn small_cone(r: f64, h: f64, tilt: f64, at: Vec3<f64>) -> (AtRestBody<f64>, Solid) {
    let truth = Solid::Revolved {
        y0: 0.0,
        y1: h,
        r0: r,
        k: -r / h,
        window: None,
    };
    posed(&[(0.0, 0.0), (r, 0.0), (0.0, h)], truth, tilt, at)
}

/// **An oblique rod or a tilted cone, inside the cone or clear of it,
/// builds.** No crossing joins anything, so the no-crossings path's
/// section certificate decides every face pair, and the cone's lateral
/// face against the rod's wall or the small cone's is the general-pose
/// arm (`section_cert/ruling.rs`): the carriers' section is classified
/// on both charts and each component cleared — the far loops the
/// infinite rod's carrier cuts from the cone by their witnesses `Out`.
/// The rod `r = 0.05`, `h = 0.2`, tilted `1` rad about `z` at
/// `(0, 0.4, 0)` (volume `0.001π`), and `r = 0.05`, `h = 0.25` tilted
/// `0.8` rad at `(0.75, 0.55, 0.1)`, clear of the cone with the boxes
/// overlapping (`0.00125π`); the cone `r = 0.1`, `h = 0.2` (`π/1500`)
/// tilted `0.7` rad at `(0.05, 0.25, 0.05)`, inside, and tilted `−1.1`
/// rad at `(0.8, 0.55, 0.2)`, clear.
#[test]
fn an_oblique_rod_and_a_tilted_cone_inside_or_clear_build_their_closed_forms() {
    let v_cone = PI / 1500.0;
    let fixtures = [
        Builds {
            what: "a rod inside",
            a: cone(None),
            b: rod(0.05, 0.2, 1.0, Vec3::new(0.0, 0.4, 0.0)),
            volumes: (V, 0.001 * PI, 0.001 * PI),
            named: vec![p(0.0, 0.4, 0.0), p(0.0, 0.2, 0.5), p(2.0, 0.5, 0.0)],
            region: (p(-0.3, 0.2, -0.1), p(0.3, 0.6, 0.1)),
        },
        Builds {
            what: "a rod clear",
            a: cone(None),
            b: rod(0.05, 0.25, 0.8, Vec3::new(0.75, 0.55, 0.1)),
            volumes: (V, 0.00125 * PI, 0.0),
            named: vec![p(0.75, 0.55, 0.1), p(0.0, 0.5, 0.0), p(0.6, 0.3, 0.1)],
            region: (p(0.4, 0.3, -0.1), p(1.1, 0.8, 0.3)),
        },
        Builds {
            what: "a tilted cone inside",
            a: cone(None),
            b: small_cone(0.1, 0.2, 0.7, Vec3::new(0.05, 0.25, 0.05)),
            volumes: (V, v_cone, v_cone),
            named: vec![p(0.03, 0.3, 0.05), p(0.0, 0.7, 0.0), p(2.0, 0.5, 0.0)],
            region: (p(-0.2, 0.1, -0.1), p(0.3, 0.5, 0.2)),
        },
        Builds {
            what: "a tilted cone clear",
            a: cone(None),
            b: small_cone(0.1, 0.2, -1.1, Vec3::new(0.8, 0.55, 0.2)),
            volumes: (V, v_cone, 0.0),
            named: vec![p(0.85, 0.56, 0.2), p(0.0, 0.5, 0.0), p(0.6, 0.6, 0.2)],
            region: (p(0.6, 0.3, 0.0), p(1.2, 0.8, 0.4)),
        },
    ];
    for f in &fixtures {
        f.check();
    }
}

/// The preview's bite: a `5π/3` sector of the rod of radius `0.3` along
/// `x` over `[−2, 2]`, about `(y, z) = (0.45, 0.8)`.
fn bite() -> AtRestBody<f64> {
    let sector = revolved(
        &[(0.0, -2.0), (0.3, -2.0), (0.3, 2.0), (0.0, 2.0)],
        Some(5.0 * PI / 3.0),
    );
    let along_x = moved(
        &sector,
        &Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), -PI / 2.0),
    );
    moved(&along_x, &Affine3::translation(Vec3::new(0.0, 0.45, 0.8)))
}

/// **A cone against an oblique cylinder has no frame.** P1, the bite,
/// and P2, the bite with P2a's brick beside it as a second lump: every
/// op refuses `GermFrameUnsupported` on the cone × cylinder pair
/// (`work/sect/cone-germ-pairs-against-a-curved-face-have-no-section-frame.md`),
/// in the operands' order.
#[test]
fn the_bite_refuses_at_the_germ_frame() {
    let tol = Tol::witness();
    let (c, _) = cone(None);
    let (pin, _) = boxed((-0.7, -0.5), (-0.3, 0.1), (-0.1, 0.1));
    let bite_and_pin = match topo::union(&bite(), &pin, tol) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("the bite and the pin, apart: {other:?}"),
    };
    for (what, b) in [("P1", bite()), ("P2", bite_and_pin)] {
        for (op, got) in every_op(&c, &b) {
            let kinds = if op == "B ∖ A" {
                [geom::SurfaceKind::Cylinder, geom::SurfaceKind::Cone]
            } else {
                [geom::SurfaceKind::Cone, geom::SurfaceKind::Cylinder]
            };
            assert!(
                matches!(
                    &got,
                    Err(BooleanError::GermFrameUnsupported { a_kind, b_kind, .. })
                        if [*a_kind, *b_kind] == kinds
                ),
                "{what}, {op}: {:?}",
                got.map(|r| r.body().map(|b| b.kind))
            );
        }
    }
}

/// **The touching configurations stay refusals.** Both are held behind
/// D10 (`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`): a
/// tangency and a coincidence are not transverse verdicts.
///
/// - A brick resting on the plane `x + y = 1`, tangent to the cone
///   along the generator through `(1, 0, 0)`: the crossing layer's
///   door, `CurvedPierceUnsupported`, on a brick edge that grazes.
/// - A quarter frustum sector of the cone's own carrier
///   (`y ∈ [0.2, 0.6]`, bored `r = 0.1`), lying on the cone face: each
///   member order refuses at the crossing layer on a generator lying on
///   the other's carrier, the cone's seam meeting the apex
///   (`CrossingAtConeApex`) or the sector's generator edge
///   (`CurvedPierceUnsupported`).
#[test]
fn a_tangent_plane_and_a_sector_on_the_cone_face_refuse() {
    let (c, _) = cone(None);
    let (raw, _) = boxed((0.0, 0.5), (-0.5, 0.5), (-0.3, 0.3));
    let tangent = moved(
        &raw,
        &(Affine3::translation(Vec3::new(0.5, 0.5, 0.0))
            * Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), PI / 4.0)),
    );
    for (op, got) in every_op(&c, &tangent) {
        assert!(
            matches!(&got, Err(BooleanError::CurvedPierceUnsupported { .. })),
            "the tangent brick, {op}: {:?}",
            got.map(|r| r.body().map(|b| b.kind))
        );
    }
    let on_face = revolved(
        &[(0.1, 0.2), (0.8, 0.2), (0.4, 0.6), (0.1, 0.6)],
        Some(0.5 * PI),
    );
    for (op, got) in every_op(&c, &on_face) {
        assert!(
            matches!(
                &got,
                Err(BooleanError::CrossingAtConeApex { .. }
                    | BooleanError::CurvedPierceUnsupported { .. })
            ),
            "the sector on the cone face, {op}: {:?}",
            got.map(|r| r.body().map(|b| b.kind))
        );
    }
}

/// The widening frustum's material below the plane through
/// `(0, 0.5, 0)` whose normal is `+y` turned `t` about `z`, without the
/// kernel: each slice `y` is the disc of radius `0.5 + 0.5·y` cut by the
/// line `x > (y − 0.5)·cot t`, whose segment's area is closed-form, and
/// Simpson's rule takes the slices.
fn frustum_below_a_tilted_plane(t: f64) -> f64 {
    let segment = |y: f64| {
        let r = 0.5 + 0.5 * y;
        let c = (y - 0.5) / t.tan();
        if c <= -r {
            PI * r * r
        } else if c >= r {
            0.0
        } else {
            r * r * (c / r).acos() - c * (r * r - c * c).sqrt()
        }
    };
    let n = 20_000;
    let h = 1.0 / n as f64;
    let mut sum = segment(0.0) + segment(1.0);
    for i in 1..n {
        sum += segment(i as f64 * h) * if i % 2 == 1 { 4.0 } else { 2.0 };
    }
    sum * h / 3.0
}

/// **The tilted plane's ellipse joins.** C1 of the cone sector spec: a
/// 6 m brick whose top face, through `(0, 0.5, 0)`, is tilted 10° off
/// axis-normal, against the widening frustum (radius `0.5 → 1` over
/// `y ∈ [0, 1]`). The face cuts the frustum's wall in an ellipse, and
/// every op builds the closed form.
#[test]
fn the_tilted_planes_ellipse_builds_its_closed_forms() {
    let tol = Tol::witness();
    let t = 10f64.to_radians();
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), t);
    let place = Affine3::translation(Vec3::new(0.0, 0.5, 0.0)) * turn;
    let half = moved(&brick((-3.0, 3.0), (-6.0, 0.0), (-3.0, 3.0), tol), &place);
    let frustum = (
        revolved(&[(0.0, 0.0), (0.5, 0.0), (1.0, 1.0), (0.0, 1.0)], None),
        Solid::Revolved {
            y0: 0.0,
            y1: 1.0,
            r0: 0.5,
            k: 0.5,
            window: None,
        },
    );
    Builds {
        what: "C1",
        a: frustum,
        b: (
            half,
            Solid::Brick([(-3.0, 3.0), (-6.0, 0.0), (-3.0, 3.0)]).posed(place),
        ),
        volumes: (
            PI / 3.0 * (0.25 + 0.5 + 1.0),
            216.0,
            frustum_below_a_tilted_plane(t),
        ),
        named: vec![
            p(0.0, 0.2, 0.0),
            p(0.0, 0.8, 0.0),
            p(1.5, 0.2, 0.0),
            p(0.0, 1.5, 0.0),
        ],
        region: (p(-1.1, -0.1, -1.1), p(1.1, 1.1, 1.1)),
    }
    .check();
}
