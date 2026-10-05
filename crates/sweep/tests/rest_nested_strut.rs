//! **A REST union whose seam ends at a nested strut's site.** A block
//! notched from its apex at the origin holds a wedge in the notch, the
//! two touching only along the apex line, so the apex is two vertices
//! at one point. A prism rests on the block's top with its corner on
//! the apex and its two edges from there reaching the block's filleted
//! top corners, at the arcs' tangent joints: the fillet's tangency
//! makes the join refuse, which hands the union to the REST lane. In
//! the prism's corner the wedge pair's strut hangs at the tip of the
//! notch pair's, so a seam segment's end is the notch strut's copy,
//! which the lane's strut undo kills.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::RawLoop;
use profile::test_support::bulge_loop;
use sweep::test_support::{extruded, sketch_at};
use topo::{AtRestBody, Body, BooleanBody, BooleanError, BooleanResult, CarriedVv, ContactClass};

fn tol() -> Tol {
    Tol::witness()
}

/// tan(π/8): a quarter-circle's bulge.
const Q: f64 = 0.414_213_562_373_095_03;

/// The prism over `pts` (`(x, y, bulge)`) over z ∈ [z0, z0 + 1], every
/// joint where an arc meets a line declared tangent.
fn prism(pts: &[(f64, f64, f64)], z0: f64) -> AtRestBody<f64> {
    let n = pts.len();
    let joints = (0..n)
        .filter(|&i| (pts[(i + n - 1) % n].2 == Q) != (pts[i].2 == Q))
        .collect();
    let chain = pts
        .iter()
        .map(|&(x, y, b)| (Point2::new(x, y), b))
        .collect();
    let body: Body<f64> = extruded(
        sketch_at(z0),
        vec![bulge_loop(chain).with_tangent_joints(joints)],
        1.0,
        tol(),
    );
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
    let rows = pinch
        .contacts
        .vv
        .iter()
        .map(|&pair| CarriedVv {
            pair,
            class: ContactClass::Rest,
        })
        .collect();
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

/// **The REST lane reads a nested strut's segment end as the vertex
/// its copy fuses into.** The join refuses both orders. With the pinch
/// first, the lane pairs each pinch apex vertex with the prism's corner
/// and realizes the seam on both operands, reading the nested end at
/// the corner. It then refuses `SeamOrientation` at the glue, pinned as
/// it stands: the lane has no reading of two vertices at one point
/// meeting one
/// (`work/topo/the-rest-lane-zips-no-pinch-apex.md`). Red as the
/// first row while the end stays the notch strut's copy: it pairs the
/// wedge's apex with that copy, against the v-v row pairing it with
/// the corner, and the lane gives the union back to the join's own
/// refusal. With the prism first, one corner corresponds to two pinch
/// vertices, which the lane's one-to-one reading declines: the join's
/// refusal stands.
#[test]
fn a_nested_struts_segment_end_reads_as_the_vertex_it_fuses_into() {
    let (pinch, rows) = pinch();
    let pinch = pinch.body;
    let top = resting();
    let refusal = |a: &AtRestBody<f64>, b: &AtRestBody<f64>, pinch_first: bool| {
        let mut decls = topo::test_support::flush_declarations(a, b, tol());
        if pinch_first {
            decls.carried_a.vv = rows.clone();
        } else {
            decls.carried_b.vv = rows.clone();
        }
        let join =
            topo::test_support::boolean_join_refusal(topo::BooleanOp::Union, a, b, &decls, tol());
        assert!(
            matches!(join, Ok(Some(BooleanError::Join(_)))),
            "pinch first: {pinch_first}: the join refuses, which opens the lane: {join:?}"
        );
        let got = topo::union_with(a, b, &decls, tol());
        (join.unwrap().unwrap(), got)
    };

    let (_, got) = refusal(&pinch, &top, true);
    assert!(
        matches!(got, Err(BooleanError::SeamOrientation { .. })),
        "pinch first: the lane realizes the seam and refuses at the glue: {got:?}"
    );

    let (join, got) = refusal(&top, &pinch, false);
    assert_eq!(
        format!("{:?}", got.err()),
        format!("{:?}", Some(join)),
        "prism first: the join's own refusal stands"
    );
}
