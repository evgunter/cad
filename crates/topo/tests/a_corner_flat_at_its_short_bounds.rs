//! **A long edge leaving a corner flat within the band at its short
//! bounds is classed by where it runs**, in every op, both operand
//! orders and every pose.
//!
//! The corner is the end of a roof's crease at [`MEET`]: the roof is
//! `z = 0` over `y < 0` and `z = g·y` over `y > 0`, its crease along
//! `+x`, and its end there cut back by two slanted planes through
//! `(0, ±1 mm)`, so the corner's bounds across the crease are 1 mm long
//! and lie half the zero band off the other side's plane: `g` is
//! `ε/2` over 1 mm. The other solid is a pyramid standing on MEET
//! whose edge to `P` runs 0.9 m between the two planes, half way up
//! the dent. Where `g > 0` the crease is reflex, so that edge runs
//! inside the roof, 80 ε below its top at its midpoint; where `g < 0`
//! it is convex and the edge runs outside, above it.
//!
//! Each row the naming reads at MEET is asserted against the other
//! operand's containment of the edge's midpoint (`point_in_solid`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{MEET, Pose, apex_pyramid, at, poses};
use common::{FaceGeometry, describe_as_intersections, finished, prism_ops};
use geom_core::{Band, Tol};
use topo::{
    AtRestBody, Body, BooleanError, BooleanResult, Operand, SideCode, SolidContainment, intersect,
    point_in_solid, readback, subtract, union,
};

fn t() -> Tol {
    Tol::witness()
}

/// The bounds across the crease, 1 mm.
const BOUND: f64 = 1e-3;

/// The roof, `dent` its rise per metre over `y > 0`, placed by `pose`
/// (module docs).
fn roof(dent: f64, pose: &Pose) -> AtRestBody<f64> {
    let off = |p: [f64; 3]| pose.at([0, 1, 2].map(|k| MEET[k] + p[k]));
    let mut body = Body::<f64>::new();
    prism_ops(
        &mut body,
        &[
            (-1.0, -1.0),
            (1.0, -1.0),
            (1.0, dent),
            (0.0, 0.0),
            (-1.0, 0.0),
        ],
        (0.0, 1.0),
        |u, v, w| off([w, u, v]),
        FaceGeometry::Certified,
        t(),
    );
    describe_as_intersections(&mut body, t());
    let mut roof = finished("the roof", body, t());
    for side in [1.0, -1.0] {
        // `x < 0.1·(side·y − BOUND)`, a prism along z.
        let mut profile: Vec<(f64, f64)> = [
            (0.1 * (-2.0 - BOUND), -2.0),
            (0.1 * (2.0 - BOUND), 2.0),
            (-1.0, 2.0),
            (-1.0, -2.0),
        ]
        .map(|(x, y)| (x, side * y))
        .to_vec();
        if side < 0.0 {
            profile.reverse();
        }
        let mut body = Body::<f64>::new();
        prism_ops(
            &mut body,
            &profile,
            (-2.0, 2.0),
            |x, y, z| off([x, y, z]),
            FaceGeometry::Certified,
            t(),
        );
        describe_as_intersections(&mut body, t());
        let cut = finished("a cut", body, t());
        roof = built("the roof's end cut back", subtract(&roof, &cut, t()));
    }
    roof
}

fn built(what: &str, r: Result<BooleanResult<f64>, BooleanError>) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what}: {:?}", other.map(|_| ())),
    }
}

/// The pyramid's base relative to [`MEET`]: `P`, between the roof's
/// planes over `y > 0` half way up the dent, then a corner inside the
/// roof and one above it.
fn base(dent: f64) -> [[f64; 3]; 3] {
    let s = std::f64::consts::FRAC_1_SQRT_2;
    let r = 0.9;
    [
        [r * s, r * s, r * 0.5 * dent * s],
        [0.3, 0.25, -0.3],
        [0.15, 0.3, 0.25],
    ]
}

#[test]
fn a_long_edge_beside_a_corner_flat_at_its_short_bounds_is_classed_where_it_runs() {
    let band = Band::linear(t()).unwrap();
    let half = 0.5 * t().eps() / BOUND;
    for pose in poses() {
        for dent in [half, -half] {
            let (a, b) = (roof(dent, &pose), apex_pyramid(&base(dent), &pose, t()));
            let meet = at(pose.at(MEET));
            let p = at(pose.at([0, 1, 2].map(|k| MEET[k] + base(dent)[0][k])));
            for (op, x, y, r) in [
                ("a − b", &a, &b, subtract(&a, &b, t())),
                ("b − a", &b, &a, subtract(&b, &a, t())),
                ("a ∪ b", &a, &b, union(&a, &b, t())),
                ("b ∪ a", &b, &a, union(&b, &a, t())),
                ("a ∩ b", &a, &b, intersect(&a, &b, t())),
                ("b ∩ a", &b, &a, intersect(&b, &a, t())),
            ] {
                let what = format!("{}, dent {dent:e}, {op}", pose.label);
                let r = match r {
                    Ok(BooleanResult::Body(r)) => r,
                    other => panic!("{what}: {:?}", other.map(|_| ())),
                };
                let mut to_p = 0;
                for row in &r.naming.edge_classes {
                    let (own, other) = match row.operand {
                        Operand::A => (x, y),
                        Operand::B => (y, x),
                    };
                    let Ok(v) = readback::vertex_point(own, row.vertex) else {
                        continue;
                    };
                    if at(v) != meet {
                        continue;
                    }
                    let edge = own.get_edge(row.edge).unwrap();
                    let plus = own.get_half_edge(edge.he_plus).unwrap();
                    let far = if row.starts {
                        own.get_half_edge(plus.next).unwrap().start
                    } else {
                        plus.start
                    };
                    let far = readback::vertex_point(own, far).unwrap();
                    to_p += usize::from(at(far) == p);
                    let mid = v + (far - v) * 0.5;
                    let want = match point_in_solid(other, mid, band, t()).unwrap() {
                        SolidContainment::In => SideCode::In,
                        SolidContainment::Out => SideCode::Out,
                        SolidContainment::OnBoundary => SideCode::On,
                    };
                    assert_eq!(row.class, want, "{what}: {row:?}, the edge to {far:?}");
                }
                assert_eq!(to_p, 1, "{what}: the edge to P is classed once");
            }
        }
    }
}
