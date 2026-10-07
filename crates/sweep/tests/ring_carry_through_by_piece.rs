//! **The ring carry-through check reads every ring piece by piece**
//! (`surgery.rs`'s `ring_pieces`), and each cycle refuses at its least
//! margin. Every fixture below builds tier-3 valid at its closed-form
//! volume where it clears, and refuses at its closed-form margin where
//! it crosses — in every subtraction order that changes the cycle walk.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Tol, Vec3};
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{cube, prism, realized};
use topo::boolean::BooleanOp;
use topo::{Body, EdgeKey, query, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn v(x: f64, y: f64, bulge: f64) -> (Point2<f64>, f64) {
    (Point2::new(x, y), bulge)
}

fn sub(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    realized(BooleanOp::Subtract, a, b, tol())
}

fn at(b: Body<f64>, x: f64, y: f64, z: f64) -> Body<f64> {
    topo::transform_rigid(&b, &Affine3::translation(Vec3::new(x, y, z)), tol()).unwrap()
}

/// A vertical bore of radius `rad` about `(x, y)`, from `z` up `h`.
fn bore(x: f64, y: f64, rad: f64, z: f64, h: f64) -> Body<f64> {
    at(
        prism(vec![v(x - rad, y, 1.0), v(x + rad, y, 1.0)], h, tol()),
        0.0,
        0.0,
        z,
    )
}

fn carrier(body: &Body<f64>, k: EdgeKey) -> geom::Curve3<f64> {
    let e = body.get_edge(k).unwrap();
    body.get_curve_geom(e.curve)
        .unwrap()
        .certified()
        .unwrap()
        .carrier()
        .clone()
}

fn vol(b: &Body<f64>) -> f64 {
    topo::mass_properties(b, tol()).unwrap().volume
}

/// The unit cube's twelve outer edges, by their line midpoints.
fn outer_box_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let on = |c: f64| c.abs() < 1e-9 || (c - 1.0).abs() < 1e-9;
    let edges: Vec<EdgeKey> = query::all_edges(body)
        .into_iter()
        .filter(|&k| {
            let e = body.get_edge(k).unwrap();
            let g = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
            let geom::Curve3::Line { .. } = g.carrier() else {
                return false;
            };
            let (t0, t1) = g.params();
            let m = g.carrier().eval((t0 + t1) / 2.0);
            on(m.x) || on(m.y)
        })
        .collect();
    assert_eq!(edges.len(), 12, "the outer box's twelve edges");
    edges
}

/// The arcs of the circle of radius `rad` at height `z`.
fn rim_at(body: &Body<f64>, z: f64, rad: f64) -> Vec<EdgeKey> {
    query::all_edges(body)
        .into_iter()
        .filter(|&k| {
            matches!(carrier(body, k), geom::Curve3::Circle { center, radius, .. }
                if (center.z - z).abs() < 1e-9 && (radius - rad).abs() < 1e-9)
        })
        .collect()
}

/// The material a unit cube keeps once its twelve edges are rounded at r.
fn rounded_cube(l: f64, r: f64) -> f64 {
    let l = l - 2.0 * r;
    l.powi(3) + 6.0 * l * l * r + 3.0 * PI * l * r * r + (4.0 / 3.0) * PI * r.powi(3)
}

/// The centroid offset of the corner region a quarter-round removes,
/// as a fraction of r (Pappus).
const CORNER_CENTROID: f64 = (10.0 - 3.0 * PI) / (3.0 * (4.0 - PI));

/// The material a fillet of radius r removes at a planar circular rim
/// of radius `big_r`, the corner region's centroid at `big_r + s·c·r`.
fn rim_removed(big_r: f64, s: f64, r: f64) -> f64 {
    2.0 * PI * (big_r + s * CORNER_CENTROID * r) * (1.0 - PI / 4.0) * r * r
}

/// The fillet builds, tier-3 valid, at the volume `want`.
fn builds(body: &Body<f64>, edges: &[EdgeKey], r: f64, want: f64, what: &str) {
    let out = fillet_edges(&sweep::test_support::at_rest(body, tol()), edges, r, tol())
        .unwrap_or_else(|e| panic!("{what}: r = {r} builds, got {:?}", e.error));
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("{what}: r = {r} is tier-3 valid, got {e:?}"));
    let got = vol(&out.body);
    assert!(
        (got - want).abs() < 1e-9,
        "{what}: r = {r} keeps {want:.12}, got {got:.12}"
    );
}

/// The fillet refuses `RingClearance` at the margin `want`.
fn refuses_at(body: &Body<f64>, edges: &[EdgeKey], r: f64, want: f64, what: &str) {
    let err = fillet_edges(&sweep::test_support::at_rest(body, tol()), edges, r, tol())
        .err()
        .unwrap_or_else(|| panic!("{what}: r = {r} refuses"))
        .error;
    let BlendError::RingClearance { margin, .. } = &err else {
        panic!("{what}: r = {r} refuses RingClearance, got {err:?}")
    };
    let got = margin.reading.diagnostic_f64_for_error_text().value();
    assert!(
        got.is_some_and(|m| (m - want).abs() < 1e-12),
        "{what}: r = {r} refuses at {want}, got {margin}"
    );
}

/// The fillet escalates in band, decided by `fillet3_ring_clearance`.
fn escalates(body: &Body<f64>, edges: &[EdgeKey], r: f64, what: &str) {
    let err = fillet_edges(&sweep::test_support::at_rest(body, tol()), edges, r, tol())
        .err()
        .unwrap_or_else(|| panic!("{what}: r = {r} escalates"))
        .error;
    let BlendError::Escalated { source, .. } = &err else {
        panic!("{what}: r = {r} escalates in band, got {err:?}")
    };
    assert_eq!(
        source.predicate,
        Some("fillet3_ring_clearance"),
        "{what}: r = {r} is the ring meter's band"
    );
}

/// **A ring of arcs refuses at its least margin, whatever order its
/// bores were cut in.**
///
/// A disc top of radius 0.5 (a hostless annulus: the rim's trim, of
/// radius `0.5 − r`, becomes the top face's outer boundary) carries a
/// lens ring of two radius-0.06 bores. The outer bore reaches 0.31 from
/// the axis, so the ring clears the trim by `0.19 − r`. The inner
/// bore's arc is split at the cycle's seam into sub-arcs, and one of
/// them crosses the trim first in one order's walk. The refusal at
/// r = 0.191 reads the outer bore's −0.001 in both orders, so shrinking
/// r by the reported margin is a recourse that builds.
#[test]
fn a_ring_refuses_at_its_least_margin_in_either_subtraction_order() {
    let a = 11.25f64.to_radians();
    let cyl = bore(0.5, 0.5, 0.5, 0.0, 1.0);
    let outer = (0.5 + 0.25 * a.cos(), 0.5 + 0.25 * a.sin());
    let inner = (0.5 + 0.2 * 0.6f64.cos(), 0.5 + 0.2 * 0.6f64.sin());
    for (first, second) in [(outer, inner), (inner, outer)] {
        let body = sub(
            &sub(&cyl, &bore(first.0, first.1, 0.06, 0.8, 0.5)),
            &bore(second.0, second.1, 0.06, 0.7, 0.5),
        );
        validate_geometric(&body, tol()).expect("the twice-bored disc is valid");
        let rim = rim_at(&body, 1.0, 0.5);
        let what = format!("bored {first:?} first");
        let r = 0.189;
        builds(
            &body,
            &rim,
            r,
            vol(&body) - rim_removed(0.5, -1.0, r),
            &what,
        );
        refuses_at(&body, &rim, 0.191, 0.19 - 0.191, &what);
    }
}

/// **A ring of three arcs is metered arc by arc in every subtraction
/// order.** Bores of radius 0.1 about (0.5, 0.5), (0.45, 0.35) and
/// (0.6, 0.38) leave one ring of three circles' arcs. The second bore
/// comes 0.25 from the `y = 0` top edge, so the outer edges clear it by
/// `0.25 − r`.
#[test]
fn a_three_arc_ring_is_metered_arc_by_arc_in_every_subtraction_order() {
    let centres = [(0.5, 0.5), (0.45, 0.35), (0.6, 0.38)];
    for order in [[0, 1, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]] {
        let mut body = cube(1.0, tol());
        for (i, &k) in order.iter().enumerate() {
            let (x, y) = centres[k];
            body = sub(&body, &bore(x, y, 0.1, 0.8 - 0.05 * i as f64, 0.5));
        }
        validate_geometric(&body, tol()).expect("the thrice-bored cube is valid");
        let edges = outer_box_edges(&body);
        let what = format!("order {order:?}");
        let r = 0.249;
        builds(
            &body,
            &edges,
            r,
            vol(&body) - (1.0 - rounded_cube(1.0, r)),
            &what,
        );
        refuses_at(&body, &edges, 0.2502, 0.25 - 0.2502, &what);
        refuses_at(&body, &edges, 0.252, 0.25 - 0.252, &what);
    }
}

/// **A hostless annulus reads a polygonal ring at its far reach.** A
/// disc top of radius 0.5 is pocketed by a square whose far vertex sits
/// 0.3 from the axis, 11.25° off `x`, between the screen's stations.
/// The trim, of radius `0.5 − r`, must enclose it: clear by `0.2 − r`.
#[test]
fn a_hostless_annulus_reads_a_polygonal_ring_at_its_far_reach() {
    let a = 11.25f64.to_radians();
    let (ux, uy) = (a.cos(), a.sin());
    let h = 0.08;
    let (px, py) = (0.5 + 0.3 * ux, 0.5 + 0.3 * uy);
    let square = prism(
        vec![
            v(px, py, 0.0),
            v(px - h * (ux - uy), py - h * (uy + ux), 0.0),
            v(px - 2.0 * h * ux, py - 2.0 * h * uy, 0.0),
            v(px - h * (ux + uy), py - h * (uy - ux), 0.0),
        ],
        0.3,
        tol(),
    );
    let body = sub(&bore(0.5, 0.5, 0.5, 0.0, 1.0), &at(square, 0.0, 0.0, 0.8));
    validate_geometric(&body, tol()).expect("the pocketed disc is valid");
    let rim = rim_at(&body, 1.0, 0.5);
    let what = "the square ring";
    let r = 0.199;
    builds(&body, &rim, r, vol(&body) - rim_removed(0.5, -1.0, r), what);
    refuses_at(&body, &rim, 0.201, 0.2 - 0.201, what);
    escalates(&body, &rim, 0.2 - 5.0 * tol().get().eps, what);
}

/// **A concave band meters a polygonal ring off the screen's
/// stations.** A side-2 cavity in a side-4 block, its ceiling vented by
/// a diamond whose vertex sits 0.2 from the `y = 1` wall at `x = 2.125`.
/// Filleting the cavity's twelve concave edges adds `8 − rounded(2, r)`
/// of material and clears the vent by `0.2 − r`.
#[test]
fn a_concave_band_meters_a_polygonal_ring_off_the_stations() {
    use crate::common::cavity::{brick, cavity_edges, cut};
    use geom_core::Point3;
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let (vx, vy, h) = (2.125, 1.2, 0.3);
    let vent = at(
        prism(
            vec![
                v(vx, vy, 0.0),
                v(vx + h, vy + h, 0.0),
                v(vx, vy + 2.0 * h, 0.0),
                v(vx - h, vy + h, 0.0),
            ],
            2.5,
            tol(),
        ),
        0.0,
        0.0,
        2.5,
    );
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let body = cut("cavity", &cut("vent", &block, &vent), &cavity);
    validate_geometric(&body, tol()).expect("the vented cavity is valid");
    let edges = cavity_edges(&body);
    let what = "the vented cavity";
    let eps = tol().get().eps;
    for r in [0.1, 0.199] {
        builds(
            &body,
            &edges,
            r,
            vol(&body) + 8.0 - rounded_cube(2.0, r),
            what,
        );
    }
    escalates(&body, &edges, 0.2 - 5.0 * eps, what);
    escalates(&body, &edges, 0.2 + 5.0 * eps, what);
    refuses_at(&body, &edges, 0.201, 0.2 - 0.201, what);
    refuses_at(&body, &edges, 0.22, 0.2 - 0.22, what);
}

/// **A lens ring beside a hole's rim builds in either subtraction
/// order.** A radius-0.2 hole at (0.3, 0.5), and a lens of two radius-0.1
/// bores whose nearer arc lies 0.3 from the hole's axis: the rim's trim
/// of radius `0.2 + r` clears it by `0.1 − r`.
#[test]
fn a_lens_ring_beside_a_hole_rim_builds_in_either_order() {
    let near = (0.3 + 0.4 * 0.19635f64.cos(), 0.5 + 0.4 * 0.19635f64.sin());
    let far = (0.3 + 0.46 * 0.45f64.cos(), 0.5 + 0.46 * 0.45f64.sin());
    let holed = sub(&cube(1.0, tol()), &bore(0.3, 0.5, 0.2, -0.2, 1.4));
    for (first, second) in [(near, far), (far, near)] {
        let body = sub(
            &sub(&holed, &bore(first.0, first.1, 0.1, 0.8, 0.5)),
            &bore(second.0, second.1, 0.1, 0.7, 0.5),
        );
        validate_geometric(&body, tol()).expect("the holed, bored cube is valid");
        let rim = rim_at(&body, 1.0, 0.2);
        let what = format!("bored {first:?} first");
        for r in [0.09, 0.099] {
            builds(&body, &rim, r, vol(&body) - rim_removed(0.2, 1.0, r), &what);
        }
    }
}

/// **A curved mate's ring refuses at the ladder gate, before the ring
/// pass.** A hole's wall carries a notch ring (two rulings, two arcs);
/// filleting the hole's top rim refuses `UnsupportedChain` at
/// `resolve_rim`'s curved-support gate, so the support-boundary walk
/// reads rings of planar hosts only.
#[test]
fn a_curved_mates_ring_refuses_at_the_ladder_gate() {
    let holed = sub(&cube(1.0, tol()), &bore(0.5, 0.5, 0.3, -0.2, 1.4));
    let notch = at(
        prism(
            vec![
                v(0.47, 0.75, 0.0),
                v(0.53, 0.75, 0.0),
                v(0.53, 0.86, 0.0),
                v(0.47, 0.86, 0.0),
            ],
            0.1,
            tol(),
        ),
        0.0,
        0.0,
        0.8,
    );
    let body = sub(&holed, &notch);
    validate_geometric(&body, tol()).expect("the notched hole is valid");
    let rim = rim_at(&body, 1.0, 0.3);
    let err = fillet_edges(
        &sweep::test_support::at_rest(&body, tol()),
        &rim,
        0.09,
        tol(),
    )
    .expect_err("refuses")
    .error;
    assert!(
        matches!(&err, BlendError::UnsupportedChain { detail, .. }
            if detail.contains("curved support carries rings")),
        "the curved-support gate answers first, got {err:?}"
    );
}

/// **An elliptical ring beside a hole's rim refuses unmetered.** The
/// support-boundary walk meters an outer-boundary ellipse by its
/// certified box, but `ring_pieces` refuses a ring edge that is not a
/// line or circle (`work/band/a-ring-of-ellipse-edges-refuses-the-ring-meter.md`).
#[test]
fn an_elliptical_ring_beside_a_hole_rim_refuses_unmetered() {
    let holed = sub(&cube(1.0, tol()), &bore(0.3, 0.5, 0.15, -0.2, 1.4));
    let body = sweep::test_support::finished(
        "body",
        sub(&holed, &at(crate::common::tilted_bore(), 0.3, 0.0, 0.0)),
        tol(),
    );
    validate_geometric(&body, tol()).expect("the twice-bored cube is valid");
    let rim = rim_at(&body, 1.0, 0.15);
    for r in [0.05, 0.1] {
        let err = fillet_edges(&body, &rim, r, tol())
            .expect_err("refuses")
            .error;
        assert!(
            matches!(&err, BlendError::UnsupportedGeometry { detail, .. }
                if detail.contains("a ring edge's carrier is neither a line nor a circle")),
            "r = {r}: the ellipse ring edge refuses, got {err:?}"
        );
    }
}

/// **A ring pinched to a hole's rim at a vertex builds, and filleting
/// the rim refuses.** A diamond pocket whose vertex meets the rim's
/// vertex runs one edge down the hole's seam ruling, a ruling lying on
/// the wall, which the subtract places as an ON event: in either order
/// it builds at its closed form (the cube less the bore and the pocket's
/// `0.02 × 0.2`), valid at tiers 3 and 3′, the pocket's floor vertex
/// recorded touching the ruling. Its top face's boundary then passes
/// through the rim vertex twice, so the rim's ring carries the pocket's
/// edges, and a fillet of the rim alone refuses on that chain.
#[test]
fn a_ring_pinched_to_a_rim_vertex_builds_and_its_rim_fillet_refuses() {
    let hole = bore(0.3, 0.5, 0.2, -0.2, 1.4);
    let pocket = at(
        prism(
            vec![
                v(0.5, 0.5, 0.0),
                v(0.6, 0.4, 0.0),
                v(0.7, 0.5, 0.0),
                v(0.6, 0.6, 0.0),
            ],
            0.3,
            tol(),
        ),
        0.0,
        0.0,
        0.8,
    );
    let want = 1.0 - PI * 0.04 - 0.02 * 0.2;
    let holed = sub(&cube(1.0, tol()), &hole);
    let pocketed = sub(&cube(1.0, tol()), &pocket);
    for (what, a, b) in [
        ("hole first", &holed, &pocket),
        ("pocket first", &pocketed, &hole),
    ] {
        use sweep::test_support::finished;
        use topo::boolean::{SweepStrategy, boolean_op_with};
        use topo::{BooleanDeclarations, BooleanResult};
        let r = boolean_op_with(
            BooleanOp::Subtract,
            &finished("a", a.clone(), tol()),
            &finished("b", b.clone(), tol()),
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            tol(),
        );
        let Ok(BooleanResult::Body(bb)) = r else {
            panic!("{what}: the pinch builds: {r:?}");
        };
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
            .unwrap_or_else(|e| panic!("{what}: tier 3′: {e:?}"));
        let body = sweep::test_support::finished("body", bb.body.into_body(), tol());
        validate_geometric(&body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3: {e:?}"));
        let got = topo::mass_properties(&body, tol()).unwrap().volume;
        assert!(
            (got - want).abs() <= 1e-12,
            "{what}: the closed form: {got} vs {want}"
        );
        assert_eq!(
            (
                body.faces().count(),
                body.edges().count(),
                body.vertices().count(),
                body.shells().count(),
            ),
            (13, 30, 19, 1),
            "{what}: F, E, V, shells"
        );
        assert_eq!(
            (
                bb.contacts.vv.len(),
                bb.contacts.a_on_b.len() + bb.contacts.b_on_a.len(),
                bb.contacts.ve.len(),
                bb.contacts.ee.len(),
            ),
            (0, 0, 1, 0),
            "{what}: [v-v, v-f, v-e, e-e] records"
        );
        let rim = rim_at(&body, 1.0, 0.2);
        for r in [0.02, 0.05] {
            let err = fillet_edges(&body, &rim, r, tol())
                .expect_err("refuses")
                .error;
            assert!(
                matches!(&err, BlendError::UnsupportedChain { detail, .. }
                    if detail.contains("a rim ring carries edges outside the requested chain")),
                "{what}, r = {r}: the rim's ring carries the pocket, got {err:?}"
            );
        }
    }
}

/// **The polygonal ring decides at `Interval`.** The diamond pocket of
/// `review_fillet_e2_probes`' twin, its vertex 0.2 from the `y = 0`
/// edge: r = 0.15 builds tier-3 valid, 0.2 ∓ 5ε escalates as a terminal
/// sliver, and r = 0.201 refuses with an enclosure about −0.001.
mod interval_lane {
    use crate::common::interval::{iv, p2, v3};
    use geom_core::{Affine3, Bounds, Interval, Tol};
    use sweep::blend::BlendError;
    use sweep::blend::build::fillet_edges;
    use sweep::test_support::{cube, finished, prism};
    use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
    use topo::{Body, BooleanDeclarations, EdgeKey};

    fn t() -> Tol {
        Tol::witness()
    }

    #[test]
    fn a_polygonal_ring_decides_at_interval() {
        let diamond = prism(
            vec![
                (p2(0.45, 0.20), iv(0.0)),
                (p2(0.55, 0.30), iv(0.0)),
                (p2(0.45, 0.40), iv(0.0)),
                (p2(0.35, 0.30), iv(0.0)),
            ],
            iv(0.3),
            t(),
        );
        let diamond =
            topo::transform_rigid(&diamond, &Affine3::translation(v3(0.0, 0.0, 0.8)), t()).unwrap();
        let body: Body<Interval> = boolean_op_with(
            BooleanOp::Subtract,
            &finished("the cube", cube(1.0, t()), t()),
            &finished("the diamond", diamond, t()),
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            t(),
        )
        .expect("the cut at Interval")
        .body()
        .expect("a body")
        .body
        .clone()
        .into_body();
        let on = |c: Interval| c.lo().abs() < 1e-9 || (c.lo() - 1.0).abs() < 1e-9;
        let outer: Vec<EdgeKey> = body
            .edges()
            .filter_map(|(k, e)| {
                let g = body.get_curve_geom(e.curve)?.certified()?;
                let geom::Curve3::Line { .. } = g.carrier() else {
                    return None;
                };
                let (t0, t1) = g.params();
                let m = g.carrier().eval((t0 + t1) / iv(2.0));
                (on(m.x) || on(m.y)).then_some(k)
            })
            .collect();
        assert_eq!(outer.len(), 12, "the outer box's twelve edges");

        let out = fillet_edges(
            &sweep::test_support::at_rest(&body, t()),
            &outer,
            iv(0.15),
            t(),
        )
        .expect("r = 0.15 builds");
        topo::validate_geometric(&out.body, t()).expect("r = 0.15 is tier-3 valid");

        let eps = t().get().eps;
        for r in [0.2 - 5.0 * eps, 0.2 + 5.0 * eps] {
            let err = fillet_edges(
                &sweep::test_support::at_rest(&body, t()),
                &outer,
                iv(r),
                t(),
            )
            .expect_err("refuses")
            .error;
            assert!(
                matches!(&err, BlendError::Escalated { source, .. }
                    if source.predicate == Some("fillet3_ring_clearance")
                        && source.terminal_sliver),
                "r = {r}: a terminal sliver of the ring meter, got {err:?}"
            );
        }

        let err = fillet_edges(
            &sweep::test_support::at_rest(&body, t()),
            &outer,
            iv(0.201),
            t(),
        )
        .expect_err("refuses")
        .error;
        let BlendError::RingClearance { margin, .. } = &err else {
            panic!("r = 0.201 refuses RingClearance, got {err:?}")
        };
        let geom_core::ErrorTextReading::Enclosure { lo, hi } =
            margin.reading.diagnostic_f64_for_error_text()
        else {
            panic!("r = 0.201: the margin is an enclosure, got {margin}")
        };
        assert!(
            lo <= -0.001 && -0.001 <= hi && hi - lo < 1e-12,
            "r = 0.201: an enclosure about −0.001, got [{lo}, {hi}]"
        );
    }
}
