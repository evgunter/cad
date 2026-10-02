//! **A pierce run holding every real edge of the orbit, beyond the
//! star fixture** (review of PR 3770). `classify_vertex_on_face`
//! records such a run as the strut `mev_null` builds; before that, each
//! row below refused `JoinDesync` ("every chord arc separates a loose
//! scaffolding pair") or `SeamOrientation`. Two shapes reach it: a
//! valence-2 rim vertex of a merged face under a coplanar cap, and a
//! 315° reflex corner under a tilted cap (the fixture of
//! `review_m3_pr55::a_reflex_315_corner_tilted_cap`, which accepts
//! either outcome and so cannot see the change).
//!
//! Every row asserts the exact volume (1e-12) and tiers 2, 3 and 3′.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, flush_declarations, prism_z};
use geom_core::{Point3, Tol, Vec3};
use topo::validate::{validate_closed, validate_geometric};
use topo::{
    Body, BooleanDeclarations, BooleanError, BooleanResult, intersect_with, mass_properties, split,
    subtract_with, union_with, validate_pseudomanifold,
};

type Op = fn(
    &Body<f64>,
    &Body<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;

fn exact(label: &str, op: Op, x: &Body<f64>, y: &Body<f64>, want: f64) {
    let tol = Tol::witness();
    let decls = flush_declarations(x, y, tol);
    let r = op(x, y, &decls, tol).unwrap_or_else(|e| panic!("{label}: refused {e:?}"));
    let BooleanResult::Body(bb) = r else {
        panic!("{label}: an overlapping pair cannot be Empty");
    };
    assert_eq!(validate_closed(&bb.body), Ok(()), "{label}: tier 2");
    assert_eq!(validate_geometric(&bb.body, tol), Ok(()), "{label}: tier 3");
    assert_eq!(
        validate_pseudomanifold(&bb.body, &bb.contacts, tol),
        Ok(()),
        "{label}: tier 3′"
    );
    let v = mass_properties(&bb.body, tol).unwrap().volume;
    assert!((v - want).abs() < 1e-12, "{label}: volume {v}, want {want}");
}

/// `c ∪ d` is the bar x∈(0.5,2.2) whose y = 1 rims keep valence-2
/// vertices at x = 1.2 and 1.5. Each `g` stands across that wall with
/// a cap coplanar with the bar's over one or both of them; every op and
/// both operand orders answer exactly.
#[test]
fn a_merged_rim_vertex_under_a_coplanar_cap_answers_every_op() {
    let tol = Tol::witness();
    let c = brick::<f64>((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), tol);
    let d = brick::<f64>((1.2, 2.2), (0.0, 1.0), (0.0, 1.0), tol);
    let BooleanResult::Body(bar) =
        union_with(&c, &d, &flush_declarations(&c, &d, tol), tol).expect("the bar folds")
    else {
        panic!("the bar is not empty");
    };
    let bar = bar.body;
    let bar_v = 1.7;
    for (gx, gy, gz) in [
        ((0.5, 1.5), (0.5, 1.5), (0.0, 1.0)), // the star's f: both vertices
        ((1.1, 1.3), (0.5, 1.5), (0.0, 1.0)), // over x = 1.2 only
        ((1.3, 1.7), (0.5, 1.5), (0.0, 1.0)), // over x = 1.5 only
        ((1.1, 1.6), (0.5, 1.5), (0.0, 2.0)), // bottom cap coplanar only
        ((1.1, 1.6), (0.5, 1.5), (-1.0, 1.0)), // top cap coplanar only
        ((1.1, 1.6), (0.25, 1.5), (0.0, 1.0)), // deeper into the bar
        ((1.1, 1.6), (0.5, 1.5), (0.0, 0.5)), // half height
    ] {
        let g = brick::<f64>(gx, gy, gz, tol);
        let g_v = (gx.1 - gx.0) * (gy.1 - gy.0) * (gz.1 - gz.0);
        let meet = (gx.1 - gx.0) * (1.0 - gy.0) * (gz.1.min(1.0) - gz.0.max(0.0));
        let tag = format!("g = {gx:?} × {gy:?} × {gz:?}");
        exact(
            &format!("bar ∪ {tag}"),
            union_with,
            &bar,
            &g,
            bar_v + g_v - meet,
        );
        exact(
            &format!("{tag} ∪ bar"),
            union_with,
            &g,
            &bar,
            bar_v + g_v - meet,
        );
        exact(&format!("bar ∩ {tag}"), intersect_with, &bar, &g, meet);
        exact(&format!("{tag} ∩ bar"), intersect_with, &g, &bar, meet);
        exact(
            &format!("bar ∖ {tag}"),
            subtract_with,
            &bar,
            &g,
            bar_v - meet,
        );
        exact(&format!("{tag} ∖ bar"), subtract_with, &g, &bar, g_v - meet);
    }
}

/// The 315° reflex corner (0, 0, 1) of `a` under the tilted bottom cap
/// of `b`: the corner's three edges read Out and the reflex sector's
/// bisector In, so the run holds the whole orbit. `b` is described
/// (unlike `review_m3_pr55`'s `tprism`) so tier 3 is meaningful.
#[test]
fn the_reflex_315_corner_under_a_tilted_cap_answers_exactly() {
    let tol = Tol::witness();
    let reflex = [
        (0.0, 0.0),
        (2.0, 2.0),
        (-2.0, 2.0),
        (-2.0, -2.0),
        (2.0, -2.0),
        (2.0, 0.0),
    ];
    let sq = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let shears = [
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.25, 0.5, 1.0]],
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.5, -0.25, 1.0]],
    ];
    let meet = 13.0 / 24.0;
    for (k, m) in shears.into_iter().enumerate() {
        let a = prism_z::<f64>(&reflex, 0.0, 1.0, tol).body;
        let mut b = Body::<f64>::new();
        common::prism_ops(
            &mut b,
            &sq,
            (1.0, 3.0),
            |x, y, z| {
                Point3::new(
                    m[0][0] * x + m[0][1] * y + m[0][2] * z,
                    m[1][0] * x + m[1][1] * y + m[1][2] * z,
                    m[2][0] * x + m[2][1] * y + m[2][2] * z,
                )
            },
            common::FaceGeometry::Certified,
            tol,
        );
        common::describe_as_intersections(&mut b, tol);
        exact(&format!("shear {k}: a ∩ b"), intersect_with, &a, &b, meet);
        exact(
            &format!("shear {k}: a ∖ b"),
            subtract_with,
            &a,
            &b,
            14.0 - meet,
        );
        exact(
            &format!("shear {k}: b ∖ a"),
            subtract_with,
            &b,
            &a,
            8.0 - meet,
        );
        exact(
            &format!("shear {k}: a ∪ b"),
            union_with,
            &a,
            &b,
            22.0 - meet,
        );
    }
}

/// **The splitter's twin**: a plane through the 315° prism's reflex top
/// corner `(0, 0, 1)`, tilted so the corner's three edges read Above
/// and the top cap's reflex bisector Below. The Above run holds the
/// whole orbit and the splitter's null edge is the strut inside the
/// reflex sector, its tip the Below copy. Each split hands back two
/// parts that pass tiers 2 and 3 and sum to the prism's 14. The
/// `(1, 0, −1)` row's below part is checked by hand: ∫₀¹ 4(z + 1) dz.
#[test]
fn a_split_through_the_reflex_corner_whose_run_holds_the_whole_orbit() {
    let tol = Tol::witness();
    let reflex = [
        (0.0, 0.0),
        (2.0, 2.0),
        (-2.0, 2.0),
        (-2.0, -2.0),
        (2.0, -2.0),
        (2.0, 0.0),
    ];
    let a = prism_z::<f64>(&reflex, 0.0, 1.0, tol).body;
    for (n, above_v, below_v) in [
        ((1.0, 0.2, -1.0), 8.0, 6.0),
        ((1.0, 0.5, -0.5), 7.0, 7.0),
        ((1.0, 0.0, -1.0), 8.0, 6.0),
    ] {
        let plane = topo::test_support::split_plane(
            Point3::new(0.0, 0.0, 1.0),
            Vec3::new(n.0, n.1, n.2).normalize(),
            geom_core::Tol::witness(),
        );
        let r = split(&a, &plane, tol).unwrap_or_else(|e| panic!("n = {n:?}: refused {e:?}"));
        for (part, want, side) in [(&r.above, above_v, "above"), (&r.below, below_v, "below")] {
            let b = part
                .body()
                .unwrap_or_else(|| panic!("n = {n:?}: no {side} part"));
            assert_eq!(validate_closed(b), Ok(()), "n = {n:?} {side}: tier 2");
            assert_eq!(
                validate_geometric(b, tol),
                Ok(()),
                "n = {n:?} {side}: tier 3"
            );
            let v = mass_properties(b, tol).unwrap().volume;
            assert!(
                (v - want).abs() < 1e-12,
                "n = {n:?} {side}: volume {v}, want {want}"
            );
        }
    }
}
