//! **An L-prism's reflex corner pierced through a plate's top builds
//! sound under every op, or refuses typed.**
//!
//! The L `[0,2]×[0,1] ∪ [0,1]×[1,2]` × `[0,1]` has its reflex corner
//! `c = (1, 1, 1)` on three edges, towards +x, +y and −z. A plane
//! through `c` with normal `k` leaning off the corner's diagonal (a
//! bearing `θ ∈ (0°, 90°)` in the L's xy, a rise `φ ∈ (0°, 90°)`) leaves
//! +x and +y on one side and −z on the other, and on the +x/+y side the
//! L falls into two pieces that touch only at `c` (`k·(p − c) ≤ 0` on
//! the square `[0,1]²×[0,1]` joining the arms, zero only at `c`). It is
//! the corner of the split's row
//! `a_reflex_corner_whose_below_edges_bound_two_pieces_hangs_a_path`.
//!
//! Here the L is turned so that plane is the top `z = 1` of a plate and
//! `c` sits at `(0, 0, 1 + δ)`: on side A the two pieces fall in the
//! plate, on side B they stand out of it. Each row runs ∪, ∩ and − in
//! both operand orders and holds it to the geometric truth: the volume
//! in closed form (a box under a half-space), tiers 3 and 3′,
//! [`corners_disjoint`], the solid count, and at `δ = 0` the number of
//! vertices at `c` (one per cone of the result's surface there). Rows
//! within 64 ε of the face may refuse instead, as an escalation.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{at, corners_disjoint};
use common::{FaceGeometry, describe_as_intersections, finished, prism_ops};
use geom_core::{Point3, Tol};
use topo::{
    AtRestBody, Body, BooleanError, BooleanResult, intersect, subtract, union, validate_geometric,
    validate_pseudomanifold,
};

fn t() -> Tol {
    Tol::witness()
}

const ELL: [(f64, f64); 6] = [
    (0.0, 0.0),
    (2.0, 0.0),
    (2.0, 1.0),
    (1.0, 1.0),
    (1.0, 2.0),
    (0.0, 2.0),
];
/// The L's halves, which the volume's closed form clips one at a time.
const HALVES: [[(f64, f64); 3]; 2] = [
    [(0.0, 2.0), (0.0, 1.0), (0.0, 1.0)],
    [(0.0, 1.0), (1.0, 2.0), (0.0, 1.0)],
];
const ELL_VOLUME: f64 = 3.0;
const CORNER: [f64; 3] = [1.0, 1.0, 1.0];
/// Where `c` lands at `δ = 0`: on the plate's top.
const TARGET: [f64; 3] = [0.0, 0.0, 1.0];
/// Reaches past every point of the L, all within 2 of `c`.
const PLATE: [(f64, f64); 3] = [(-2.5, 2.5), (-2.5, 2.5), (-2.0, 1.0)];
const PLATE_VOLUME: f64 = 75.0;

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// The cutting plane's normal `k`, by its bearing and rise (radians).
#[derive(Clone, Copy, Debug)]
struct Lean {
    theta: f64,
    phi: f64,
}

impl Lean {
    fn k(self) -> [f64; 3] {
        let (st, ct) = self.theta.sin_cos();
        let (sp, cp) = self.phi.sin_cos();
        [ct * cp, st * cp, sp]
    }
}

/// Which side of the top the L's two pieces fall on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Side {
    /// In the plate: `k` turned to −z.
    A,
    /// Out of it: `k` turned to +z.
    B,
}

/// The L turned so `k` lies along ∓z, `c` at `TARGET + δ ẑ`.
fn ell(lean: Lean, side: Side, delta: f64) -> AtRestBody<f64> {
    let k = lean.k();
    let a = cross(k, [0.0, 0.0, 1.0]);
    let e1 = a.map(|x| x / dot(a, a).sqrt());
    let e2 = cross(k, e1);
    let s = if side == Side::A { -1.0 } else { 1.0 };
    let rows = [e1, e2.map(|x| s * x), k.map(|x| s * x)];
    let mut body = Body::<f64>::new();
    prism_ops(
        &mut body,
        &ELL,
        (0.0, 1.0),
        |x, y, z| {
            let d = [x - CORNER[0], y - CORNER[1], z - CORNER[2]];
            let q = [0, 1, 2].map(|i| dot(rows[i], d) + TARGET[i]);
            Point3::new(q[0], q[1], q[2] + delta)
        },
        FaceGeometry::Certified,
        t(),
    );
    describe_as_intersections(&mut body, t());
    finished("the leaned L", body, t())
}

fn plate() -> AtRestBody<f64> {
    finished(
        "the plate",
        common::brick::<f64>(PLATE[0], PLATE[1], PLATE[2], t()),
        t(),
    )
}

/// The volume of `{x ∈ b : n·x ≤ d}`, every `n_i ≠ 0`: the unit cube's
/// inclusion–exclusion over its corners, after an affine map making
/// every weight positive.
fn box_below(b: [(f64, f64); 3], n: [f64; 3], d: f64) -> f64 {
    let mut rhs = d;
    let mut w = [0.0; 3];
    for i in 0..3 {
        let len = b[i].1 - b[i].0;
        if n[i] > 0.0 {
            rhs -= n[i] * b[i].0;
            w[i] = n[i] * len;
        } else {
            rhs -= n[i] * b[i].1;
            w[i] = -n[i] * len;
        }
    }
    let sum: f64 = (0..8u32)
        .map(|m| {
            let mut r = rhs;
            let mut odd = false;
            for (i, wi) in w.iter().enumerate() {
                if m & (1 << i) != 0 {
                    r -= wi;
                    odd = !odd;
                }
            }
            let v = r.max(0.0).powi(3);
            if odd { -v } else { v }
        })
        .sum();
    sum / (6.0 * w[0] * w[1] * w[2]) * (0..3).map(|i| b[i].1 - b[i].0).product::<f64>()
}

/// The L's volume on the side `k·(p − c) ≤ d`.
fn ell_below(lean: Lean, d: f64) -> f64 {
    let k = lean.k();
    HALVES
        .iter()
        .map(|&h| box_below(h, k, d + dot(k, CORNER)))
        .sum()
}

#[derive(Clone, Copy, Debug)]
enum Op {
    /// `L ∪ P`, in either order.
    Union,
    /// `L ∩ P`, in either order.
    Intersect,
    /// `L − P`.
    LessPlate,
    /// `P − L`.
    PlateLess,
}

type Door = fn(&AtRestBody<f64>, &AtRestBody<f64>, Tol) -> Result<BooleanResult<f64>, BooleanError>;

/// Every op row: its name, its op, its door, and whether the plate goes
/// first.
const OPS: [(&str, Op, Door, bool); 6] = [
    ("L ∪ P", Op::Union, union, false),
    ("P ∪ L", Op::Union, union, true),
    ("L ∩ P", Op::Intersect, intersect, false),
    ("P ∩ L", Op::Intersect, intersect, true),
    ("L − P", Op::LessPlate, subtract, false),
    ("P − L", Op::PlateLess, subtract, true),
];

/// The truth: the volume, the solids, and the cones at `c` at `δ = 0`.
fn truth(lean: Lean, side: Side, delta: f64, op: Op) -> (f64, usize, usize) {
    // The L in the plate: side A `k·(p − c) ≥ δ`, side B `≤ −δ`.
    let inside = match side {
        Side::A => ELL_VOLUME - ell_below(lean, delta),
        Side::B => ell_below(lean, -delta),
    };
    let volume = match op {
        Op::Union => ELL_VOLUME + PLATE_VOLUME - inside,
        Op::Intersect => inside,
        Op::LessPlate => ELL_VOLUME - inside,
        Op::PlateLess => PLATE_VOLUME - inside,
    };
    // The two pieces are side A's L ∩ P and side B's L − P: apart while
    // c stands off their side of the top, touching at c at δ = 0, joined
    // once it passes.
    let (pieces, apart) = match side {
        Side::A => (matches!(op, Op::Intersect), delta >= 0.0),
        Side::B => (matches!(op, Op::LessPlate), delta <= 0.0),
    };
    let solids = if pieces && apart { 2 } else { 1 };
    // The result's surface at c is two cones where the two pieces, or
    // the material whose link they split, bound it.
    let cones = match (side, op) {
        (Side::A, Op::Union | Op::Intersect) | (Side::B, Op::LessPlate | Op::PlateLess) => 2,
        _ => 1,
    };
    (volume, solids, cones)
}

/// The leans: the split row's, and four more over the open quadrant.
fn leans() -> [(&'static str, Lean); 5] {
    let deg = |theta: f64, phi: f64| Lean {
        theta: theta.to_radians(),
        phi,
    };
    [
        (
            "the split row's",
            Lean {
                theta: std::f64::consts::TAU * 0.37 / 12.0,
                phi: 0.05,
            },
        ),
        ("45°, 0.3", deg(45.0, 0.3)),
        ("75°, 0.15", deg(75.0, 0.15)),
        ("20°, 0.8", deg(20.0, 0.8)),
        ("88°, 0.02", deg(88.0, 0.02)),
    ]
}

/// Runs every lean, δ and op on `side` and asserts each row (module
/// docs). `δ = 0` and `δ = ±10⁻³` must build; within 64 ε of the face a
/// row may refuse as an escalation instead.
fn rows(side: Side) {
    let eps = t().eps();
    let plate = plate();
    let c = at(Point3::new(TARGET[0], TARGET[1], TARGET[2]));
    let mut refused = Vec::new();
    for (lean_name, lean) in leans() {
        for (delta, must_build) in [
            (0.0, true),
            (4.0 * eps, false),
            (-4.0 * eps, false),
            (64.0 * eps, false),
            (-64.0 * eps, false),
            (1e-3, true),
            (-1e-3, true),
        ] {
            let l = ell(lean, side, delta);
            for (name, op, door, plate_first) in OPS {
                let what = format!("lean {lean_name}, side {side:?}, δ = {delta:e}: {name}");
                let (a, b) = if plate_first {
                    (&plate, &l)
                } else {
                    (&l, &plate)
                };
                let r = match door(a, b, t()) {
                    Ok(BooleanResult::Body(r)) => r,
                    Ok(BooleanResult::Empty) => panic!("{what}: empty"),
                    Err(BooleanError::Escalated { decision, .. }) if !must_build => {
                        refused.push(format!("{what}: {decision:?}"));
                        continue;
                    }
                    Err(e) => panic!("{what}: refused {e:?}"),
                };
                let body = &r.body;
                let (volume, solids, cones) = truth(lean, side, delta, op);
                assert_eq!(validate_geometric(body, t()), Ok(()), "{what}: tier 3");
                assert!(
                    validate_pseudomanifold(body, &r.contacts, t()).is_ok(),
                    "{what}: tier 3′"
                );
                assert_eq!(corners_disjoint(body, t()), Ok(()), "{what}: corners");
                assert_eq!(body.solids().count(), solids, "{what}: solids");
                assert_eq!(body.shells().count(), solids, "{what}: shells");
                let v = topo::test_support::pinned_volume("the result", body, t()).volume;
                assert!(
                    (v - volume).abs() < 1e-9,
                    "{what}: volume {v}, closed form {volume}"
                );
                if delta == 0.0 {
                    let at_c = body
                        .vertices()
                        .filter(|&(k, _)| at(topo::readback::vertex_point(body, k).unwrap()) == c)
                        .count();
                    assert_eq!(at_c, cones, "{what}: vertices at c");
                }
            }
        }
    }
    println!("{} rows refused as escalations:", refused.len());
    for r in &refused {
        println!("  {r}");
    }
}

/// **Side A: the L's two pieces fall in the plate**, so `L ∩ P` is two
/// solids touching at `c` and `L ∪ P` keeps two cones of surface there.
#[test]
fn a_reflex_corner_whose_two_pieces_fall_in_a_plate_builds_or_refuses_typed() {
    rows(Side::A);
}

/// **Side B: the L's two pieces stand out of the plate**, so `L − P` is
/// two solids touching at `c` and `P − L` is pinched there.
#[test]
fn a_reflex_corner_whose_two_pieces_stand_out_of_a_plate_builds_or_refuses_typed() {
    rows(Side::B);
}
