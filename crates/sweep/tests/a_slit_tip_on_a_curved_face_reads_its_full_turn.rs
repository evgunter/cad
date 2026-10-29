//! **A slit's tip inside a cylinder face, on the plane, reads the face's
//! full turn.** The block `[0, 4] × [−2, 2] × [0, 4]` with a half-bore
//! of radius 1 about the z axis in its `x = 0` face, less the V-trench
//! over the triangle `(1, 0)`, `(3, ±1)` above `z = 2`: its keel lies on
//! the bore's ruling through `(1, 0)`, and the material either side of
//! it is two wedges, bore | wall a and wall b | bore
//! (`crates/topo/README.md`, tier 3). So the bore's face carries the
//! keel as a slit, two coincident edges from the top to its tip at
//! `(1, 0, 2)`, inside the face, and between them at the tip the face
//! wraps a full turn.
//!
//! The body is built as the union of the block's two sides of the
//! trench, each the block cut by a prism, the lower one holding the V
//! below `z = 2`. (`block ∖ trench` builds the keel as one edge
//! between the walls with the bore's face running through it,
//! `work/splitdoor/a-keel-on-a-cylinders-ruling-subtracts-with-the-bore-through-its-wedge.md`.)
//!
//! Through the tip, the plane `z = 2` and one tilted about y, both slit
//! edges above: the split and the Boolean against a slab whose top is
//! the plane both read the vertex against the sections the plane ends
//! there (`topo::vertex_section`), where the bore's sector at the tip
//! is the full turn, and each side holds its closed form and passes
//! tiers 3 and 3′.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::test_support::{extruded, finished, sketch_from_axes};
use topo::{
    AtRestBody, BooleanResult, ContactRecords, SplitPart, intersect, split, subtract, union,
    validate_geometric, validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

fn polygon(pts: &[(f64, f64)]) -> profile::ProfileLoop<f64> {
    bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect())
}

/// A prism over `pts`, from `z0` up `h`.
fn prism(pts: &[(f64, f64)], z0: f64, h: f64) -> AtRestBody<f64> {
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    finished(
        "a prism",
        extruded(plane, vec![polygon(pts)], h, tol()),
        tol(),
    )
}

fn body_of(r: Result<BooleanResult<f64>, topo::BooleanError>, what: &str) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("{what}: {other:?}"),
    }
}

/// The grooved block less the trench, its keel a slit of the bore.
fn slit_block() -> AtRestBody<f64> {
    sides_of(finished(
        "the grooved block",
        extruded(
            SketchPlane::xy(),
            vec![bulge_loop(vec![
                (Point2::new(0.0, -2.0), 0.0),
                (Point2::new(4.0, -2.0), 0.0),
                (Point2::new(4.0, 2.0), 0.0),
                (Point2::new(0.0, 2.0), 0.0),
                (Point2::new(0.0, 1.0), -1.0),
                (Point2::new(0.0, -1.0), 0.0),
            ])],
            4.0,
            tol(),
        ),
        tol(),
    ))
}

/// `block` less the trench, built as the union of its two sides.
fn sides_of(block: AtRestBody<f64>) -> AtRestBody<f64> {
    // Wall a runs from the keel to (3, 1), wall b to (3, −1); the
    // upper side holds the block past the trench's end, `x ≥ 3`, and the
    // lower one the V below `z = 2`.
    let upper = prism(
        &[
            (1.0, 0.0),
            (3.0, 1.0),
            (3.0, -5.0),
            (5.0, -5.0),
            (5.0, 5.0),
            (-5.0, 5.0),
            (-5.0, 0.0),
        ],
        -1.0,
        6.0,
    );
    let lower = body_of(
        union(
            &prism(
                &[
                    (1.0, 0.0),
                    (-5.0, 0.0),
                    (-5.0, -5.0),
                    (3.0, -5.0),
                    (3.0, -1.0),
                ],
                -1.0,
                6.0,
            ),
            &prism(&[(1.0, 0.0), (3.0, -1.0), (3.0, 1.0)], -1.0, 3.0),
            tol(),
        ),
        "the lower tool",
    );
    body_of(
        union(
            &body_of(intersect(&block, &upper, tol()), "the upper side"),
            &body_of(intersect(&block, &lower, tol()), "the lower side"),
            tol(),
        ),
        "the two sides",
    )
}

const TIP: Point3<f64> = Point3::new(1.0, 0.0, 2.0);

/// The block's cross-section less the half-bore: its area and its
/// first moment about `x = 0`.
fn section() -> (f64, f64) {
    (16.0 - PI / 2.0, 32.0 - 2.0 / 3.0)
}

/// The trench's triangle, `(1, 0)`, `(3, ±1)`: its area and its first
/// moment about `x = 1`.
const TRENCH: (f64, f64) = (2.0, 2.0 * 4.0 / 3.0);

/// Each plane through the tip, `z = 2 + s·(x − 1)`, and the volume of
/// the body below it: the section under the plane's height, less the
/// trench under the plane above `z = 2`.
fn planes() -> [(f64, f64); 2] {
    let (area, moment) = section();
    [0.0, 0.25].map(|s| (s, 2.0 * area + s * (moment - area) - s * TRENCH.1))
}

fn whole() -> f64 {
    4.0 * section().0 - 2.0 * TRENCH.0
}

fn holds(what: &str, b: &AtRestBody<f64>, want: f64) {
    let mp = topo::test_support::pinned_volume(what, b, tol());
    let rounding = 64.0 * f64::EPSILON * want;
    assert!(
        (mp.volume - want).abs() <= mp.volume_pad + rounding,
        "{what}: holds {} ± {:e}, not {want}",
        mp.volume,
        mp.volume_pad
    );
    assert_eq!(validate_geometric(b, tol()), Ok(()), "{what}: tier 3");
    assert_eq!(
        validate_pseudomanifold(b, &ContactRecords::default(), tol()),
        Ok(()),
        "{what}: tier 3′"
    );
}

/// A box whose top face is the plane through `TIP` along `n`.
fn slab(n: Vec3<f64>) -> AtRestBody<f64> {
    let n = n / n.norm();
    let u = Vec3::new(0.0, 1.0, 0.0);
    let v = n.cross(u);
    let plane = sketch_from_axes(TIP - n * 20.0, u, v, tol());
    let square = polygon(&[(-10.0, -10.0), (10.0, -10.0), (10.0, 10.0), (-10.0, 10.0)]);
    finished(
        "the slab",
        extruded(plane, vec![square], 20.0, tol()),
        tol(),
    )
}

/// **The body is the slit form**: one cylinder face carries two
/// coincident line edges ending at the tip, and the body holds its
/// closed form and passes tiers 3 and 3′.
#[test]
fn the_block_carries_the_keel_as_a_slit_of_the_bore() {
    let body = slit_block();
    holds("the slit block", &body, whole());
    let at = |v: topo::VertexKey| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let bores: Vec<_> = body
        .edges()
        .filter(|(_, e)| {
            let h = e.he_plus;
            let (a, b) = (
                body.get_half_edge(h).unwrap().start,
                body.half_edge_end(h).unwrap(),
            );
            (at(a).distance(TIP) < 1e-12 || at(b).distance(TIP) < 1e-12)
                && (at(a) - at(b)).cross(Vec3::new(0.0, 0.0, 1.0)).norm() < 1e-12
        })
        .flat_map(|(_, e)| {
            [Some(e.he_plus), body.mate(e.he_plus)]
                .into_iter()
                .flatten()
                .filter_map(|h| body.face_of_half_edge(h))
                .filter(|&f| {
                    topo::query::face_surface_kind(&body, f) == Some(geom::SurfaceKind::Cylinder)
                })
        })
        .collect();
    assert_eq!(
        bores.len(),
        2,
        "two keel edges at the tip, each on the bore: {bores:?}"
    );
    assert_eq!(bores[0], bores[1], "both keel edges bound one bore face");
}

/// **Split through the tip**: each side holds its closed form.
#[test]
fn a_split_through_the_slits_tip_on_the_bore_builds() {
    let body = slit_block();
    for (s, below) in planes() {
        let n = Vec3::new(-s, 0.0, 1.0);
        let plane = topo::test_support::split_plane(TIP, n / n.norm(), tol());
        let what = format!("the split along z = 2 + {s}·(x − 1)");
        let r = split(&body, &plane, tol()).unwrap_or_else(|e| panic!("{what}: {e:?}"));
        for (side, part, want) in [
            ("above", &r.above, whole() - below),
            ("below", &r.below, below),
        ] {
            let SplitPart::Body(b) = part else {
                panic!("{what}: nothing {side}");
            };
            holds(&format!("{what}, {side}"), b, want);
        }
    }
}

/// **The Boolean against a slab whose top runs through the tip**: the
/// part below and the part above hold their closed forms.
#[test]
fn a_slab_topped_through_the_slits_tip_on_the_bore_cuts_it() {
    let body = slit_block();
    for (s, below) in planes() {
        let slab = slab(Vec3::new(-s, 0.0, 1.0));
        let what = format!("the slab under z = 2 + {s}·(x − 1)");
        holds(
            &format!("{what}, ∩"),
            &body_of(intersect(&body, &slab, tol()), &what),
            below,
        );
        holds(
            &format!("{what}, ∖"),
            &body_of(subtract(&body, &slab, tol()), &what),
            whole() - below,
        );
    }
}

/// A box `[−5, 5] × y × z`, `y` and `z` its spans.
fn box_over((y0, y1): (f64, f64), (z0, z1): (f64, f64)) -> AtRestBody<f64> {
    prism(&[(-5.0, y0), (5.0, y0), (5.0, y1), (-5.0, y1)], z0, z1 - z0)
}

/// **Review: a tool whose bottom face holds the tip, the slit's edges
/// inside it** — the slab rows' tool on the plane's other side. The box
/// `z ∈ [2, 3]` crosses both keel edges at `z = 3`; `z ∈ [2, 5]` holds
/// them whole, so the crossing is not what this row reads. On main each
/// op refuses `Escalated { Coincidence(SectorSide, Moot) }`; on the
/// reviewed head each panics at `boolean::vtxfac`'s "the bisector run's
/// corner … left the vertex or gained a half".
#[test]
fn review_a_box_whose_bottom_holds_the_tip_cuts_it() {
    let body = slit_block();
    for (z1, inside) in [
        (3.0, section().0 - TRENCH.0),
        (5.0, 2.0 * section().0 - 2.0 * TRENCH.0),
    ] {
        let tool = box_over((-5.0, 5.0), (2.0, z1));
        for (what, r, want) in [
            ("body ∩ box", intersect(&body, &tool, tol()), inside),
            ("box ∩ body", intersect(&tool, &body, tol()), inside),
            (
                "body ∖ box",
                subtract(&body, &tool, tol()),
                whole() - inside,
            ),
        ] {
            holds(&format!("z ∈ [2, {z1}], {what}"), &body_of(r, what), want);
        }
    }
}

/// **Review: a tool whose face holds the slit's edges.** The box
/// `y ≤ 0, z ≥ 2`: its face `y = 0` holds the keel, its bottom the tip.
#[test]
fn review_a_box_whose_face_holds_the_slit_cuts_it() {
    let body = slit_block();
    let tool = box_over((-5.0, 0.0), (2.0, 5.0));
    let inside = 2.0 * (section().0 / 2.0 - TRENCH.0 / 2.0);
    for (what, r, want) in [
        ("body ∩ half box", intersect(&body, &tool, tol()), inside),
        ("half box ∩ body", intersect(&tool, &body, tol()), inside),
        (
            "body ∖ half box",
            subtract(&body, &tool, tol()),
            whole() - inside,
        ),
    ] {
        holds(what, &body_of(r, what), want);
    }
}

/// **Review: a box from below the tip to above it** (`z ∈ [1, 3]`): the
/// tip inside the tool, no face through it.
#[test]
fn review_a_box_round_the_tip_cuts_it() {
    let body = slit_block();
    let tool = box_over((-5.0, 5.0), (1.0, 3.0));
    let inside = 2.0 * section().0 - TRENCH.0;
    for (what, r, want) in [
        ("body ∩ box", intersect(&body, &tool, tol()), inside),
        (
            "body ∖ box",
            subtract(&body, &tool, tol()),
            whole() - inside,
        ),
    ] {
        holds(what, &body_of(r, what), want);
    }
}

/// **Review: the split through the tip, its normal reversed**, the slit's
/// edges below the plane.
#[test]
fn review_a_split_through_the_tip_reversed_builds() {
    let body = slit_block();
    for (s, below) in planes() {
        let n = -Vec3::new(-s, 0.0, 1.0);
        let plane = topo::test_support::split_plane(TIP, n / n.norm(), tol());
        let what = format!("the split along z = 2 + {s}·(x − 1), reversed");
        let r = split(&body, &plane, tol()).unwrap_or_else(|e| panic!("{what}: {e:?}"));
        for (side, part, want) in [
            ("above", &r.above, below),
            ("below", &r.below, whole() - below),
        ] {
            let SplitPart::Body(b) = part else {
                panic!("{what}: nothing {side}");
            };
            holds(&format!("{what}, {side}"), b, want);
        }
    }
}

/// Review probe: the bottom-at-the-tip box against the planar twin
/// (the bore a square slot `x ∈ [0, 1]`, `|y| ≤ 1`, its wall `x = 1`
/// holding the keel), each op's outcome printed.
#[test]
#[ignore = "review probe"]
fn review_probe_the_planar_twin() {
    let body = sides_of(prism(
        &[
            (0.0, -2.0),
            (4.0, -2.0),
            (4.0, 2.0),
            (0.0, 2.0),
            (0.0, 1.0),
            (1.0, 1.0),
            (1.0, -1.0),
            (0.0, -1.0),
        ],
        0.0,
        4.0,
    ));
    println!(
        "PROBE twin: {}",
        topo::test_support::pinned_volume("twin", &body, tol()).volume
    );
    for (z0, z1) in [(2.0, 3.0), (1.0, 2.0)] {
        let tool = box_over((-5.0, 5.0), (z0, z1));
        for (what, swap, sub) in [
            ("∩", false, false),
            ("swapped ∩", true, false),
            ("∖", false, true),
        ] {
            let (a, b) = if swap { (&tool, &body) } else { (&body, &tool) };
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if sub {
                    subtract(a, b, tol())
                } else {
                    intersect(a, b, tol())
                }
            }));
            let out = match r {
                Err(_) => "PANIC".to_string(),
                Ok(Ok(BooleanResult::Body(b))) => format!(
                    "body {}",
                    topo::test_support::pinned_volume(what, &b.body, tol()).volume
                ),
                Ok(other) => format!("{other:?}"),
            };
            println!("PROBE twin z∈[{z0},{z1}] {what}: {out:.200}");
        }
    }
}
