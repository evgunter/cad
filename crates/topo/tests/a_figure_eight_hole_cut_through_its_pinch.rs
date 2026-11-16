//! **A figure-eight hole cut through its pinch leaves each lobe on its
//! own side.** A plate drilled through by a figure-eight, one loop through
//! the origin twice, its lobes the squares `[-1, 0]²` and `[0, 1]²`, cut
//! through the pinch: by the split along the plane `x + y = 0`, and by a
//! subtract and an intersect with a prism whose face lies on `x + y = 0`
//! or on `x = 0`, where the lobes' walls lie on the tool's face. Each side
//! keeps one lobe, merged into its face's outer loop: every one of these
//! divisions runs through the pinch vertex and identifies it, so no ring
//! is left a bystander touching the division. It pins the identified
//! case `chord_join::paths_rung`'s premise rests on. A ring crossed at
//! its vertices, the division divided there or refusing in the band, is
//! `a_ring_crossed_at_its_vertices_is_divided_or_refused`.
//!
//! The plate is built raw (`drill_hole`, described, finished): the public
//! doors refuse its subtract (`LineWedges` overlap at the pinch line).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{
    FaceGeometry, describe_as_intersections, drill_hole, finished, identity_map, plane_every_face,
    prism_ops, prism_z, split_plane,
};
use geom_core::{Point3, Tol, Vec3};
use topo::{Body, ContactRecords, SplitPart, intersect, split, subtract, validate_pseudomanifold};

fn figure_eight_plate(tol: Tol) -> topo::AtRestBody<f64> {
    let pt = identity_map::<f64>;
    let mut body = Body::<f64>::new();
    let ops = prism_ops(
        &mut body,
        &[(-3.0, -3.0), (3.0, -3.0), (3.0, 3.0), (-3.0, 3.0)],
        (0.0, 1.0),
        pt,
        FaceGeometry::Declined,
        tol,
    );
    let entry = ops.sides[0].he_plus;
    let lobes = [
        (0.0, 0.0),
        (-1.0, 0.0),
        (-1.0, -1.0),
        (0.0, -1.0),
        (0.0, 0.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
    ];
    let rim: Vec<_> = lobes.iter().map(|&(x, y)| pt(x, y, 1.0)).collect();
    let drops: Vec<_> = lobes.iter().map(|&(x, y)| pt(x, y, 0.0)).collect();
    drill_hole(&mut body, entry, ops.bottom.face, &rim, &drops, tol);
    plane_every_face(&mut body, tol);
    describe_as_intersections(&mut body, tol);
    finished("the figure-eight plate", body, tol)
}

/// `body` keeps the lobe at `kept` (a far corner of one lobe) and not
/// the one at `gone`, and no face of it has a ring.
fn holds_one_lobe(what: &str, body: &Body<f64>, kept: (f64, f64), gone: (f64, f64)) {
    let at = |c: (f64, f64)| body.vertex_points().any(|(_, p)| p.x == c.0 && p.y == c.1);
    assert!(
        at(kept) && !at(gone),
        "{what}: the lobe at {kept:?} is kept and the one at {gone:?} is not"
    );
    assert!(
        body.faces().all(|(_, f)| f.rings.is_empty()),
        "{what}: the lobe is merged into its face's outer loop, no ring is left"
    );
}

#[test]
fn a_figure_eight_hole_cut_through_its_pinch_keeps_each_lobe_on_its_side() {
    let tol = Tol::witness();
    let plate = figure_eight_plate(tol);
    let (up, down) = ((1.0, 1.0), (-1.0, -1.0));

    let plane = split_plane(Point3::new(0.0, 0.0, 0.5), Vec3::new(1.0, 1.0, 0.0), tol);
    let r = split(&plate, &plane, tol).unwrap();
    for (what, side, kept, gone) in [("above", r.above, up, down), ("below", r.below, down, up)] {
        let SplitPart::Body(body) = side else {
            panic!("split {what}: a side is empty");
        };
        holds_one_lobe(&format!("split {what}"), &body, kept, gone);
    }

    let tool = |what, profile: &[(f64, f64)]| {
        finished(what, prism_z::<f64>(profile, -1.0, 2.0, tol).body, tol)
    };
    let diagonal = tool(
        "the diagonal prism",
        &[(-4.0, 4.0), (4.0, -4.0), (4.0, 4.0)],
    );
    let upright = tool(
        "the x = 0 prism",
        &[(0.0, -4.0), (4.0, -4.0), (4.0, 4.0), (0.0, 4.0)],
    );
    for (what, r, kept, gone) in [
        (
            "subtract x + y = 0",
            subtract(&plate, &diagonal, tol),
            down,
            up,
        ),
        (
            "intersect x + y = 0",
            intersect(&plate, &diagonal, tol),
            up,
            down,
        ),
        ("subtract x = 0", subtract(&plate, &upright, tol), down, up),
        (
            "intersect x = 0",
            intersect(&plate, &upright, tol),
            up,
            down,
        ),
    ] {
        let r = r.unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let body = &r.body().unwrap_or_else(|| panic!("{what}: empty")).body;
        assert_eq!(
            validate_pseudomanifold(body, &ContactRecords::default(), tol),
            Ok(()),
            "{what}: the census passes"
        );
        holds_one_lobe(what, body, kept, gone);
    }
}
