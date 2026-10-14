//! **The nodes, built**: two equal cylinders whose axes meet at an
//! angle `θ`, both long enough that each passes through the other, so
//! both nodes `p ± r·n̂` lie inside a face of each wall. Their section,
//! the two bisector-plane ellipses, crosses itself at both nodes, and
//! every boolean of the pair builds:
//!
//! - `∩`, the Steinmetz solid, `V = 16r³/(3 sin θ)`, each node one vertex
//!   where four faces alternate between the walls (valence 4);
//! - `∪`, `V_A + V_B − V∩`, each node one valence-4 vertex;
//! - `A∖B` and `B∖A`, `V_A − V∩` and `V_B − V∩`: two solids touching at
//!   both nodes, each with its own valence-2 vertex there.
//!
//! Each operand is turned about its own axis first, so neither wall's
//! seam passes a node (U2's poses are not these).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::germ_pair::{cyl, repose, spin};
use geom_core::{Point3, Tol, Vec3};
use sweep::test_support::finished;
use topo::{Body, BooleanResult};

/// The seam turn about each operand's own axis.
const PHI: f64 = 0.4;
/// The walls' half-length: past every point of the intersection for the
/// angles below (`(1 + |cos θ|)/sin θ < 4`).
const H: f64 = 4.0;

/// `A` about `z`, and `B` about `(0, −sin θ, cos θ)`, each of radius 1
/// and half-length [`H`], through the origin; their nodes are `±x̂`.
fn pair(theta: f64) -> (Body<f64>, Body<f64>) {
    let a = spin(&cyl(1.0, H), Vec3::new(0.0, 0.0, 1.0), PHI);
    let b = spin(
        &spin(&cyl(1.0, H), Vec3::new(0.0, 0.0, 1.0), PHI),
        Vec3::new(1.0, 0.0, 0.0),
        theta,
    );
    (a, b)
}

/// One operation's result: its volume with its certified bracket, its
/// solids, and the valence of each vertex standing on each node, by
/// node.
struct Built {
    volume: (f64, f64),
    solids: usize,
    at_nodes: [Vec<usize>; 2],
}

fn built(
    r: Result<BooleanResult<f64>, topo::BooleanError>,
    nodes: [Point3<f64>; 2],
    what: &str,
) -> Built {
    let Ok(BooleanResult::Body(out)) = r else {
        panic!("{what}: the node builds, got {r:?}");
    };
    let body = &out.body;
    let tol = Tol::witness();
    let m = topo::mass_properties(body, tol).unwrap();
    let valence = |v| body.half_edges().filter(|(_, h)| h.start == v).count();
    let at = |n: Point3<f64>| {
        body.vertex_points()
            .filter(|(_, p)| (*p - n).norm() < 1e-9)
            .map(|(v, _)| valence(v))
            .collect::<Vec<_>>()
    };
    Built {
        volume: (m.volume, m.volume_pad),
        solids: body.solids().count(),
        at_nodes: [at(nodes[0]), at(nodes[1])],
    }
}

/// Every operation of the pair at angle `theta`, both operand orders,
/// against the closed forms and the node's shape.
fn assert_family(a: &Body<f64>, b: &Body<f64>, theta: f64, nodes: [Point3<f64>; 2], what: &str) {
    let tol = Tol::witness();
    let (a, b) = (finished("A", a.clone(), tol), finished("B", b.clone(), tol));
    let v_wall = 2.0 * PI * H;
    let cap = 16.0 / (3.0 * theta.sin());
    let close = |got: &Built, want: f64, op: &str| {
        assert!(
            (got.volume.0 - want).abs() <= got.volume.1 + 1e-9,
            "{what}, {op}: volume {} ± {}, want {want}",
            got.volume.0,
            got.volume.1
        );
    };
    for (first, second, order) in [(&a, &b, "A, B"), (&b, &a, "B, A")] {
        let cut = |op| format!("{what} ({order}), {op}");
        let i = built(topo::intersect(first, second, tol), nodes, &cut("∩"));
        close(&i, cap, &cut("∩"));
        let u = built(topo::union(first, second, tol), nodes, &cut("∪"));
        close(&u, 2.0 * v_wall - cap, &cut("∪"));
        for (got, op) in [(&i, "∩"), (&u, "∪")] {
            assert_eq!(got.solids, 1, "{}: one solid", cut(op));
            assert_eq!(
                got.at_nodes,
                [vec![4], vec![4]],
                "{}: each node is one vertex of valence 4",
                cut(op)
            );
        }
        let d = built(topo::subtract(first, second, tol), nodes, &cut("∖"));
        close(&d, v_wall - cap, &cut("∖"));
        assert_eq!(d.solids, 2, "{}: two solids", cut("∖"));
        assert_eq!(
            d.at_nodes,
            [vec![2, 2], vec![2, 2]],
            "{}: each node is a valence-2 vertex of each solid",
            cut("∖")
        );
    }
}

/// **The family at several angles**, each beside its re-posed twin.
#[test]
fn equal_cylinders_with_meeting_axes_build_every_operation_at_the_closed_forms() {
    let nodes = [Point3::new(1.0, 0.0, 0.0), Point3::new(-1.0, 0.0, 0.0)];
    for deg in [90.0_f64, 60.0, 120.0, 35.0] {
        let theta = deg.to_radians();
        let (a, b) = pair(theta);
        assert_family(&a, &b, theta, nodes, &format!("{deg}°"));
    }
}

/// **The same family re-posed**: the nodes move with the operands, and
/// nothing else changes.
#[test]
fn the_family_re_posed_builds_the_same() {
    let theta = 60.0_f64.to_radians();
    let (a, b) = pair(theta);
    let (a, b) = (repose(&a), repose(&b));
    let r = geom_core::Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 2.0, 3.0).normalize(),
        0.7,
    );
    let place =
        geom_core::Affine3::from_parts(r.linear, r.translation + Vec3::new(0.3, -0.45, 0.6));
    let nodes = [
        place.transform_point(Point3::new(1.0, 0.0, 0.0)),
        place.transform_point(Point3::new(-1.0, 0.0, 0.0)),
    ];
    assert_family(&a, &b, theta, nodes, "60°, re-posed");
}
