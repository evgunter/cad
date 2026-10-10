//! **A REST union whose seam ends at a nested strut's site, on a pinch
//! apex.** A block notched from its apex at the origin holds a wedge in
//! the notch, the two touching only along the apex line, so the apex is
//! two vertices at one point. A prism rests on the block's top with its
//! corner on the apex and its two edges from there reaching the block's
//! filleted top corners, at the arcs' tangent joints. In the prism's
//! corner the wedge pair's strut hangs at the tip of the notch pair's,
//! and the fillets' tangency puts a germ on each joint's edge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::{extruded, sketch_at};
use topo::{AtRestBody, Body, BooleanBody, BooleanResult, CarriedVv, ContactClass};

fn tol() -> Tol {
    Tol::witness()
}

/// tan(π/8): a quarter-circle's bulge.
const Q: f64 = 0.414_213_562_373_095_03;

/// The prism over `pts` (`(x, y, bulge)`) over z ∈ [z0, z0 + 1].
fn prism(pts: &[(f64, f64, f64)], z0: f64) -> AtRestBody<f64> {
    let chain = pts
        .iter()
        .map(|&(x, y, b)| (Point2::new(x, y), b))
        .collect();
    let body: Body<f64> = extruded(sketch_at(z0), vec![bulge_loop(chain)], 1.0, tol());
    sweep::test_support::finished("an operand", body, tol())
}

/// The pinch: the block, notched from 60° to 120° up to y = 1.5 with
/// its top corners filleted at radius 0.1, and the wedge from 75° to
/// 105° in the notch, folded into one body; and its v-v records as
/// `Rest` rows.
fn pinch() -> (BooleanBody<f64>, Vec<CarriedVv>) {
    let mx = 1.5 / 60f64.to_radians().tan();
    let block = prism(
        &[
            (-1.0, -1.0, 0.0),
            (1.0, -1.0, 0.0),
            (1.0, 1.4, Q),
            (0.9, 1.5, 0.0),
            (mx, 1.5, 0.0),
            (0.0, 0.0, 0.0),
            (-mx, 1.5, 0.0),
            (-0.9, 1.5, Q),
            (-1.0, 1.4, 0.0),
        ],
        0.0,
    );
    let at = |deg: f64| {
        let r = deg.to_radians();
        (1.6 * r.cos(), 1.6 * r.sin(), 0.0)
    };
    let wedge = prism(&[(0.0, 0.0, 0.0), at(75.0), at(105.0)], 0.0);
    let decls = topo::test_support::flush_declarations(&block, &wedge, tol());
    let Ok(BooleanResult::Body(pinch)) = topo::union_with(&block, &wedge, &decls, tol()) else {
        panic!("the wedge folds into the notch");
    };
    let rows = pinch.contacts.carried(ContactClass::Rest).vv;
    (pinch, rows)
}

/// The prism resting on the pinch: its corner on the apex, its edges
/// from there to the fillets' joints (1, 1.4) and (−1, 1.4), up the
/// lines x = ±1 past the arcs, and across at the height where it meets
/// each arc at 45°.
fn resting() -> AtRestBody<f64> {
    let y = 1.5 - 0.1 * (1.0 - core::f64::consts::FRAC_1_SQRT_2);
    prism(
        &[
            (0.0, 0.0, 0.0),
            (1.0, 1.4, 0.0),
            (1.0, y, 0.0),
            (-1.0, y, 0.0),
            (-1.0, 1.4, 0.0),
        ],
        1.0,
    )
}

/// **A pinch apex meeting one vertex builds, in either order.** The
/// join connects both orders and the union is the two stacked solids'
/// volumes added: the interiors are disjoint, the prism resting on the
/// pinch's top. It passes tiers 2, 3 and 3′.
#[test]
fn a_pinch_apex_meeting_one_vertex_builds_in_either_order() {
    let (pinch, rows) = pinch();
    let pinch = pinch.body;
    let top = resting();
    let vol = |b: &Body<f64>| topo::mass_properties(b, tol()).unwrap().volume;
    let want = vol(&pinch) + vol(&top);
    for pinch_first in [true, false] {
        let (a, b) = if pinch_first {
            (&pinch, &top)
        } else {
            (&top, &pinch)
        };
        let mut decls = topo::test_support::flush_declarations(a, b, tol());
        if pinch_first {
            decls.carried_a.vv = rows.clone();
        } else {
            decls.carried_b.vv = rows.clone();
        }
        let join =
            topo::test_support::boolean_join_refusal(topo::BooleanOp::Union, a, b, &decls, tol());
        assert!(
            matches!(join, Ok(None)),
            "pinch first: {pinch_first}: the join connects: {join:?}"
        );
        let Ok(BooleanResult::Body(bb)) = topo::union_with(a, b, &decls, tol()) else {
            panic!("pinch first: {pinch_first}: the union builds");
        };
        let got = vol(&bb.body);
        assert!(
            (got - want).abs() <= 1e-12 * want,
            "pinch first: {pinch_first}: volume {got} against {want}"
        );
        assert_eq!(
            topo::validate_closed(&bb.body),
            Ok(()),
            "pinch first: {pinch_first}: tier 2"
        );
        assert_eq!(
            topo::validate_geometric(&bb.body, tol()),
            Ok(()),
            "pinch first: {pinch_first}: tier 3"
        );
        assert_eq!(
            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
            Ok(()),
            "pinch first: {pinch_first}: tier 3′"
        );
    }
}
