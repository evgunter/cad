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
//! Through the tip, the plane `z = 2` and one tilted about y: the split,
//! under either normal, and the Boolean against a slab whose top is the
//! plane, both read the vertex against the sections the plane ends there
//! (`topo::vertex_section`), where the bore's sector at the tip is the
//! full turn. A box whose bottom holds the tip and the slit's edges, on
//! the bore's block and on its planar twin (the bore a square slot),
//! cuts the full turn into strut runs of its quarters
//! (`boolean::vtxfac`). Each result holds its closed form, either way
//! round, and passes tiers 3 and 3′.
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

/// **Split through the tip**, under either normal: each side holds its
/// closed form.
#[test]
fn a_split_through_the_slits_tip_on_the_bore_builds() {
    let body = slit_block();
    for (s, below) in planes() {
        for up in [1.0, -1.0] {
            let n = Vec3::new(-s, 0.0, 1.0) * up;
            let plane = topo::test_support::split_plane(TIP, n / n.norm(), tol());
            let what = format!("the split along z = 2 + {s}·(x − 1), normal {n:?}");
            let r = split(&body, &plane, tol()).unwrap_or_else(|e| panic!("{what}: {e:?}"));
            let (above, below) = if up > 0.0 {
                (whole() - below, below)
            } else {
                (below, whole() - below)
            };
            for (side, part, want) in [("above", &r.above, above), ("below", &r.below, below)] {
                let SplitPart::Body(b) = part else {
                    panic!("{what}: nothing {side}");
                };
                holds(&format!("{what}, {side}"), b, want);
            }
        }
    }
}

/// **The Boolean against a slab whose top runs through the tip**, the
/// slit's edges above it: each op, either way round, holds its closed
/// form.
#[test]
fn a_slab_topped_through_the_slits_tip_on_the_bore_cuts_it() {
    let body = slit_block();
    for (s, below) in planes() {
        cuts(
            &format!("the slab under z = 2 + {s}·(x − 1)"),
            &body,
            &slab(Vec3::new(-s, 0.0, 1.0)),
            whole(),
            below,
        );
    }
}

/// A box `[−5, 5] × y × z`, `y` and `z` its spans.
fn box_over((y0, y1): (f64, f64), (z0, z1): (f64, f64)) -> AtRestBody<f64> {
    prism(&[(-5.0, y0), (5.0, y0), (5.0, y1), (-5.0, y1)], z0, z1 - z0)
}

/// The planar twin: the bore a square slot `x ∈ [0, 1]`, `|y| ≤ 1`, its
/// wall `x = 1` holding the keel, less the same trench.
fn slotted_block() -> AtRestBody<f64> {
    sides_of(prism(
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
    ))
}

/// The slot's cross-section area.
const SLOTTED: f64 = 14.0;

/// Holds each of `body ∩ tool`, `tool ∩ body` and `body ∖ tool` to its
/// closed form, `inside` the volume they share.
fn cuts(what: &str, body: &AtRestBody<f64>, tool: &AtRestBody<f64>, whole: f64, inside: f64) {
    for (op, r, want) in [
        ("body ∩ tool", intersect(body, tool, tol()), inside),
        ("tool ∩ body", intersect(tool, body, tol()), inside),
        ("body ∖ tool", subtract(body, tool, tol()), whole - inside),
    ] {
        let what = format!("{what}, {op}");
        holds(&what, &body_of(r, &what), want);
    }
}

/// The box over `z ∈ [2, z1]`, its bottom face holding the tip and the
/// slit's edges inside it, the slab rows' tool on the plane's other
/// side, against `body` of cross-section `area`: to `z = 3` it crosses
/// both keel edges, to `z = 5` it holds them whole. The plane cuts the
/// full turn at the tip into strut runs of its quarters
/// (`boolean::vtxfac`).
fn box_on_the_tip(what: &str, body: &AtRestBody<f64>, area: f64) {
    let whole = 4.0 * area - 2.0 * TRENCH.0;
    for z1 in [3.0_f64, 5.0] {
        cuts(
            &format!("{what}, the box z ∈ [2, {z1}]"),
            body,
            &box_over((-5.0, 5.0), (2.0, z1)),
            whole,
            (z1.min(4.0) - 2.0) * (area - TRENCH.0),
        );
    }
}

/// **A box whose bottom holds the tip cuts the bore's block.**
#[test]
fn a_box_whose_bottom_holds_the_slits_tip_cuts_the_bore() {
    box_on_the_tip("the bore's block", &slit_block(), section().0);
}

/// **A box whose bottom holds the tip cuts the planar twin**, the slot's
/// wall `x = 1` holding the keel.
#[test]
fn a_box_whose_bottom_holds_the_slits_tip_cuts_the_slot() {
    box_on_the_tip("the slotted block", &slotted_block(), SLOTTED);
}

/// **A tool whose face holds the slit's edges.** The box `y ≤ 0, z ≥ 2`:
/// its face `y = 0` holds the keel, its bottom the tip.
#[test]
fn a_box_whose_face_holds_the_slit_cuts_it() {
    cuts(
        "the half box",
        &slit_block(),
        &box_over((-5.0, 0.0), (2.0, 5.0)),
        whole(),
        section().0 - TRENCH.0,
    );
}

/// **A box from below the tip to above it** (`z ∈ [1, 3]`): the tip
/// inside the tool, no face through it.
#[test]
fn a_box_round_the_slits_tip_cuts_it() {
    cuts(
        "the box z ∈ [1, 3]",
        &slit_block(),
        &box_over((-5.0, 5.0), (1.0, 3.0)),
        whole(),
        2.0 * section().0 - TRENCH.0,
    );
}

/// **The box less the block refuses typed**: it keeps the trench's V
/// and the bore's (or slot's) space, which meet along the keel, four
/// sheets on one line, a pairing the Boolean does not build
/// (`work/cleave/lumps-meeting-four-sheets-on-a-line-refuse-line-wedges.md`).
/// It refuses by name, on the keel.
#[test]
fn the_box_less_the_block_refuses_the_four_sheets_at_the_keel() {
    for (what, body) in [
        ("the bore's block", slit_block()),
        ("the slotted block", slotted_block()),
    ] {
        for (z0, z1) in [(2.0, 3.0), (1.0, 3.0)] {
            let what = format!("the box z ∈ [{z0}, {z1}] less {what}");
            match subtract(&box_over((-5.0, 5.0), (z0, z1)), &body, tol()) {
                Err(topo::BooleanError::LineWedges { from, to, .. }) => {
                    for end in [from, to] {
                        assert!(
                            end.starts_with("(1.0, ")
                                && (end.ends_with(", 2.0)") || end.ends_with(", 3.0)")),
                            "{what}: refuses on the keel, not at {end}"
                        );
                    }
                }
                other => panic!("{what}: {other:?}"),
            }
        }
    }
}

/// The plane `z = 2 + s·y` through the tip, and the tool on either side
/// of it (`up`: above, the box rows' side). In the bore's tangent plane
/// `x = 1` the plane's line turns with `s`, so the side below holds two
/// of the full turn's quarters, a bisector-only strut run of two where
/// the tool is above. The section and trench are symmetric in `y`, so
/// the body below holds `2·area − |s|/3` (the trench's half on the
/// plane's raised side has first moment ⅓).
fn slab_about_x(s: f64, up: bool) -> AtRestBody<f64> {
    let n = Vec3::new(0.0, -s, 1.0);
    let n = n / n.norm();
    let u = Vec3::new(1.0, 0.0, 0.0);
    let v = n.cross(u);
    let origin = if up { TIP } else { TIP - n * 20.0 };
    let plane = sketch_from_axes(origin, u, v, tol());
    let square = polygon(&[(-10.0, -10.0), (10.0, -10.0), (10.0, 10.0), (-10.0, 10.0)]);
    finished(
        "the slab about x",
        extruded(plane, vec![square], 20.0, tol()),
        tol(),
    )
}

/// **A slab tilted about x through the tip cuts both blocks**, the tool
/// under the plane and over it.
#[test]
fn a_slab_tilted_about_x_through_the_tip_cuts_both_blocks() {
    for (what, body, area) in [
        ("the bore's block", slit_block(), section().0),
        ("the slotted block", slotted_block(), SLOTTED),
    ] {
        let whole = 4.0 * area - 2.0 * TRENCH.0;
        for up in [true, false] {
            for s in [0.0_f64, 0.25, -0.25, 0.75, -0.75] {
                let below = 2.0 * area - s.abs() / 3.0;
                cuts(
                    &format!(
                        "{what}, the slab {} z = 2 + {s}·y",
                        if up { "over" } else { "under" }
                    ),
                    &body,
                    &slab_about_x(s, up),
                    whole,
                    if up { whole - below } else { below },
                );
            }
        }
    }
}
