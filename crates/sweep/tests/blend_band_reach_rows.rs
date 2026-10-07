//! **Predicate 2's reach, beside a circular band and across scalars.**
//!
//! A drum's convex top rim against a box void and a ball void; a boss's
//! concave foot against a wall hovering over the plate; a ledge whose
//! convex and concave edges are requested together. Each refusal stands
//! beside a control that builds. The interval rows read faces that are
//! not coaxial with the band — planes parallel to its axis, a sphere off
//! it — and must give f64's verdict. The rest pin the shapes the meter
//! refuses on a body no chain can blend today: a spine with a tangent
//! end face (a typed run-out, not an unbounded window) and an open arc
//! taken over its whole turn
//! (`work/band/blend-reach-takes-an-open-arc-link-over-the-whole-turn.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Interval, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::blend::{BlendError, BlendRequest, fillet_edges};
use sweep::test_support::{band_reach, corners, finished, revolved_about_y_at, rim_arcs_at};
use topo::{AtRestBody, Body, EntityId, mass_properties, validate_geometric};

use crate::common::cavity::{brick, cut, edges_with_corners, rod};
use crate::common::interval::iv;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("the witness band")
}

/// The union of a finished `a` and the fixture `b`, finished.
fn fuse(a: &AtRestBody<f64>, b: &Body<f64>) -> AtRestBody<f64> {
    let t = tol();
    let b = sweep::test_support::finished("b", b.clone(), t);
    topo::union(a, &b, t)
        .expect("the union succeeds")
        .body()
        .expect("the union leaves material")
        .body
        .clone()
}

/// The fillet's body, tier-3 valid, with its volume; or its refusal.
fn built(
    what: &str,
    body: &AtRestBody<f64>,
    edges: &[topo::EdgeKey],
    r: f64,
) -> Result<f64, BlendError> {
    let f = fillet_edges(body, edges, r, tol()).map_err(|e| e.error)?;
    assert_eq!(validate_geometric(&f.body, tol()), Ok(()), "{what}: tier 3");
    Ok(mass_properties(&f.body, tol()).expect("a volume").volume)
}

fn is_reach(e: &BlendError) -> bool {
    matches!(e, BlendError::FaceClearance { .. })
}

/// The solid drum `ρ ≤ 1`, `y ∈ [0, 2]` about `y`, at `T`.
fn drum<T: geom_core::Decide + topo::AtRestPolicy>() -> Body<T> {
    revolved_about_y_at::<T>(
        corners(&[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0)]),
        Revolution::Full,
        tol(),
    )
}

fn subtract<T: geom_core::Decide + geom_core::Bounds + topo::AtRestPolicy>(
    a: Body<T>,
    b: Body<T>,
) -> AtRestBody<T> {
    let t = tol();
    let a = topo::test_support::finished("drum", a, t);
    let b = topo::test_support::finished("void", b, t);
    topo::subtract(&a, &b, t)
        .expect("the cut succeeds")
        .body()
        .expect("the cut leaves material")
        .body
        .clone()
}

/// The drum with a sealed `0.1` box void at `x ∈ [x0, x0 + 0.1]`,
/// `y ∈ [y0, y0 + 0.1]`, `z ∈ [−0.05, 0.05]`: four of its walls are
/// planes parallel to the rim band's axis.
fn drum_with_box_void<T: geom_core::Decide + geom_core::Bounds + topo::AtRestPolicy>(
    x0: f64,
    y0: f64,
) -> AtRestBody<T> {
    let void =
        sweep::test_support::brick::<T>((x0, x0 + 0.1), (y0, y0 + 0.1), (-0.05, 0.05), tol());
    subtract(drum::<T>(), void)
}

/// The spandrel a convex rim fillet of radius `r` removes at outer
/// radius `big` (Pappus).
fn rim_removed(big: f64, r: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let area = r * r * (1.0 - pi / 4.0);
    let off = r * (10.0 - 3.0 * pi) / (3.0 * (4.0 - pi));
    2.0 * pi * (big - off) * area
}

/// The void's far corner placed on the 45° ray from the band's ball
/// centre `(0.7, 1.7)`, `r − gap` from it: inside the ball, so `gap`
/// clear of the material the band removes.
fn just_clear(gap: f64) -> (f64, f64) {
    let d = (0.3 - gap) / 2f64.sqrt();
    let (rho, y) = (0.7 + d, 1.7 + d);
    ((rho * rho - 0.05 * 0.05).sqrt() - 0.1, y - 0.1)
}

/// The drum's convex top rim (`r = 0.3`) refuses a void whose corner
/// lies in the material it removes, and builds — at the exact volume —
/// with the void deep inside or `1e-2` and `1e-4` clear of it.
#[test]
fn a_convex_rim_refuses_a_void_in_its_material_and_builds_beside_one_just_clear() {
    let r = 0.3;
    let body = drum_with_box_void::<f64>(0.85, 1.85);
    let edges = rim_arcs_at(&body, 1.0, 2.0);
    let e = built("interfering", &body, &edges, r).expect_err("the band cuts the void");
    assert!(is_reach(&e), "interfering: refused by the reach: {e:?}");
    let want = 2.0 * std::f64::consts::PI - 1e-3 - rim_removed(1.0, r);
    for (what, (x0, y0)) in [
        ("deep", (0.4, 1.0)),
        ("1e-2 clear", just_clear(1e-2)),
        ("1e-4 clear", just_clear(1e-4)),
    ] {
        let body = drum_with_box_void::<f64>(x0, y0);
        let edges = rim_arcs_at(&body, 1.0, 2.0);
        let v = built(what, &body, &edges, r).unwrap_or_else(|e| panic!("{what}: clear: {e:?}"));
        assert!((v - want).abs() < 1e-9, "{what}: V {v}, want {want}");
    }
}

/// The meter's verdict on the rim at `f64` and at `Interval`, for one
/// body built at both scalars.
fn verdicts(
    f: &Body<f64>,
    i: &Body<Interval>,
    r: f64,
) -> (Result<(), BlendError>, Result<(), BlendError>) {
    let fe = rim_arcs_at(f, 1.0, 2.0);
    let ie = rim_arcs_at(i, 1.0, 2.0);
    assert!(
        !fe.is_empty() && fe.len() == ie.len(),
        "the rim at both scalars"
    );
    let at_f64 = band_reach(
        &BlendRequest {
            body: f,
            edges: fe,
            size: r,
        },
        band(),
    );
    let at_interval = band_reach(
        &BlendRequest {
            body: i,
            edges: ie,
            size: iv(r),
        },
        band(),
    );
    (at_f64, at_interval)
}

/// **Planes parallel to the band's axis, at both scalars.** The box
/// void's walls have `n·a = 0` in the rim band's sheet, which once made
/// the plane's sheet form `0/|0|` — NaN at f64, which `max` dropped, and
/// the poison interval at `Interval`, which refused every clear body
/// `Escalated(Invalid)`. Both scalars refuse the interfering void and
/// pass the three clear ones.
#[test]
fn planes_parallel_to_a_circular_band_meter_alike_at_both_scalars() {
    let (xb, yb) = just_clear(1e-2);
    for (what, x0, y0, refuses) in [
        ("interfering", 0.85, 1.85, true),
        ("deep", 0.4, 1.0, false),
        ("1e-2 clear", xb, yb, false),
        ("clear, in the reach's box", 0.55, 1.75, false),
    ] {
        let (f, i) = verdicts(
            &drum_with_box_void::<f64>(x0, y0),
            &drum_with_box_void::<Interval>(x0, y0),
            0.3,
        );
        for (scalar, v) in [("f64", &f), ("Interval", &i)] {
            match (refuses, v) {
                (true, Err(e)) => assert!(is_reach(e), "{what} at {scalar}: {e:?}"),
                (false, Ok(())) => {}
                _ => panic!("{what} at {scalar}: refuses {refuses}, got {v:?}"),
            }
        }
    }
}

/// **A sphere off the band's axis, at both scalars**: a ball void of
/// radius `0.05` beside the rim, its sheet slack its centre's distance
/// from the axis. On the 45° ray from the band's ball centre at `0.283`
/// it reaches the material by `0.033` and refuses; at `0.24` it is
/// `0.01` clear and passes.
#[test]
fn a_sphere_off_a_circular_band_s_axis_meters_alike_at_both_scalars() {
    for (what, d, refuses) in [("reaching", 0.2828, true), ("0.01 clear", 0.24, false)] {
        let k = d / 2f64.sqrt();
        let c = (0.7 + k, 1.7 + k, 0.0);
        let f = subtract(
            drum::<f64>(),
            sweep::test_support::ball_poled_z_at(0.05, Vec3::new(c.0, c.1, c.2), tol()),
        );
        let i = subtract(
            drum::<Interval>(),
            sweep::test_support::ball_poled_z_at(
                iv(0.05),
                Vec3::new(iv(c.0), iv(c.1), iv(c.2)),
                tol(),
            ),
        );
        let (f, i) = verdicts(&f, &i, 0.3);
        for (scalar, v) in [("f64", &f), ("Interval", &i)] {
            match (refuses, v) {
                (true, Err(e)) => assert!(is_reach(e), "{what} at {scalar}: {e:?}"),
                (false, Ok(())) => {}
                _ => panic!("{what} at {scalar}: refuses {refuses}, got {v:?}"),
            }
        }
    }
}

/// **A concave boss foot beside a wall hovering `0.05` over the
/// plate**, a second solid, so no boundary feature on the plate shows
/// it and only the reach can. At that height the band reaches
/// `ρ = 0.5677`: the wall `0.05` and `0.06` off the boss refuses, and
/// `0.075` and `0.1` off it builds.
#[test]
fn a_concave_boss_foot_refuses_a_hovering_wall_in_its_reach() {
    let r = 0.2;
    for (gap, refuses) in [(0.05, true), (0.06, true), (0.075, false), (0.1, false)] {
        let plate = brick(Point3::new(-3.0, -3.0, 0.0), Point3::new(3.0, 3.0, 1.0));
        let boss = rod(Point2::new(0.0, 0.0), 0.5, 0.5, 2.0);
        let wall = brick(
            Point3::new(0.5 + gap, -1.0, 1.05),
            Point3::new(0.9 + gap, 1.0, 2.5),
        );
        let plate = finished("the plate", plate, tol());
        let body = fuse(&fuse(&plate, &boss), &wall);
        let foot = edges_with_corners(&body, |p| {
            (p.z - 1.0).abs() < 1e-9 && (p.x.hypot(p.y) - 0.5).abs() < 1e-9
        });
        match (refuses, built(&format!("gap {gap}"), &body, &foot, r)) {
            (true, Err(e)) => assert!(is_reach(&e), "gap {gap}: {e:?}"),
            (false, Ok(_)) => {}
            (_, out) => panic!("gap {gap}: refuses {refuses}, got {out:?}"),
        }
    }
}

/// **A ledge's convex and concave edges, requested together**: two
/// chains apart, sharing the ledge as a support. The meter clears them
/// while the two trimlines stay apart (`r < 0.5` on a ledge `1` wide)
/// and refuses from `r = 0.5`, naming the other band by its edge, since
/// that band has no face yet.
#[test]
fn a_ledge_s_two_bands_meter_each_other_until_their_trimlines_meet() {
    let p = |x: f64, y: f64| Point2::new(x, y);
    let body = crate::common::cavity::prism(
        &[
            p(0.0, 0.0),
            p(3.0, 0.0),
            p(3.0, 2.0),
            p(1.0, 2.0),
            p(1.0, 1.0),
            p(0.0, 1.0),
        ],
        0.0,
        4.0,
    );
    let mut both = edges_with_corners(&body, |q| (q.y - 1.0).abs() < 1e-9 && q.x.abs() < 1e-9);
    both.extend(edges_with_corners(&body, |q| {
        (q.y - 1.0).abs() < 1e-9 && (q.x - 1.0).abs() < 1e-9
    }));
    assert_eq!(both.len(), 2, "the ledge's two edges");
    for (r, refuses) in [(0.3, false), (0.49, false), (0.5, true), (0.6, true)] {
        let req = BlendRequest {
            body: &body,
            edges: both.clone(),
            size: r,
        };
        match (refuses, band_reach(&req, band())) {
            (true, Err(BlendError::FaceClearance { at, .. })) => {
                assert!(
                    matches!(at, EntityId::Edge(_)),
                    "r {r}: names the band: {at:?}"
                )
            }
            (false, Ok(())) => {}
            (_, out) => panic!("r {r}: refuses {refuses}, got {out:?}"),
        }
    }
}

/// A prism over a bulge loop, `z ∈ [z0, z1]`.
fn bulged(pts: Vec<(Point2<f64>, f64)>, z0: f64, z1: f64) -> Body<f64> {
    use profile::{Profile, RawLoop, test_support::bulge_loop};
    use sweep::{ExtrudeSide, Extrusion, extrude};
    let n = pts.len();
    let lp = bulge_loop(pts).with_tangent_joints((0..n).collect());
    let profile = Profile::new(crate::common::cavity::sketch_at(z0), vec![lp])
        .validate(tol())
        .expect("the profile");
    extrude(
        &profile,
        Extrusion::Distance {
            depth: z1 - z0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .expect("the extrusion")
    .body
}

/// **A straight link whose end face is tangent to its spine** — an
/// obround boss's top rim, at a line-to-arc junction, and a rounded
/// block's — has no plane that ends its window there. The meter refuses
/// that typed (`UnsupportedRunOut` at the junction) where the window's
/// pad once divided by `n·τ = 0` and ran to infinity. `fillet_edges`
/// refuses these closed mixed chains upstream today.
#[test]
fn a_spine_with_a_tangent_end_face_refuses_typed_not_with_an_unbounded_window() {
    let p = |x: f64, y: f64| Point2::new(x, y);
    let plate = brick(Point3::new(-6.0, -3.0, 0.0), Point3::new(6.0, 3.0, 1.0));
    let obround = bulged(
        vec![
            (p(-1.0, -1.0), 0.0),
            (p(1.0, -1.0), 1.0),
            (p(1.0, 1.0), 0.0),
            (p(-1.0, 1.0), 1.0),
        ],
        0.5,
        2.0,
    );
    let post = brick(Point3::new(3.5, 0.8, 0.5), Point3::new(4.0, 1.2, 2.5));
    let plate = finished("the plate", plate, tol());
    let boss = fuse(&fuse(&plate, &obround), &post);
    let rim = edges_with_corners(&boss, |q| (q.z - 2.0).abs() < 1e-9 && q.x.abs() < 2.5);
    let b = (std::f64::consts::PI / 8.0).tan();
    let block = bulged(
        vec![
            (p(1.0, 0.0), 0.0),
            (p(5.0, 0.0), b),
            (p(6.0, 1.0), 0.0),
            (p(6.0, 5.0), b),
            (p(5.0, 6.0), 0.0),
            (p(1.0, 6.0), b),
            (p(0.0, 5.0), 0.0),
            (p(0.0, 1.0), b),
        ],
        0.0,
        2.0,
    );
    let block_rim = edges_with_corners(&block, |q| (q.z - 2.0).abs() < 1e-9);
    for (what, body, edges) in [
        ("obround", &*boss, rim),
        ("rounded block", &block, block_rim),
    ] {
        assert!(!edges.is_empty(), "{what}: its rim");
        let req = BlendRequest {
            body,
            edges,
            size: 0.2,
        };
        let e = band_reach(&req, band()).expect_err("a tangent end face ends no window");
        assert!(
            matches!(
                e,
                BlendError::UnsupportedRunOut {
                    at: EntityId::Vertex(_),
                    ..
                }
            ),
            "{what}: {e:?}"
        );
    }
}

/// **An open arc link is read over its whole turn** (filed): a D-boss's
/// arc, cut off at its flat `x = 0.5`, refuses a post past the flat on
/// the arc's circle, and a 120° segment boss's arc refuses a post across
/// its circle — both clear of the band itself. A post off the circle
/// passes. Each refusal flips to a pass when an open arc's reach is
/// bounded by its ends.
#[test]
fn an_open_arc_link_s_reach_is_its_whole_turn() {
    let plate = || brick(Point3::new(-3.0, -3.0, 0.0), Point3::new(3.0, 3.0, 1.0));
    let arc_of = |b: &Body<f64>| -> Vec<topo::EdgeKey> {
        edges_with_corners(b, |q| {
            (q.z - 2.0).abs() < 1e-9 && (q.x.hypot(q.y) - 1.0).abs() < 1e-9
        })
        .into_iter()
        .filter(|k| {
            let e = b.get_edge(*k).expect("the edge");
            let c = b.get_curve_geom(e.curve).expect("its curve");
            matches!(
                c.certified().expect("certified").carrier(),
                geom::Curve3::Circle { .. }
            )
        })
        .collect()
    };
    let d_boss = cut(
        "flat",
        &rod(Point2::new(0.0, 0.0), 1.0, 0.5, 2.0),
        &brick(Point3::new(0.5, -2.0, 0.0), Point3::new(2.0, 2.0, 3.0)),
    );
    let segment = cut(
        "flat",
        &rod(Point2::new(0.0, 0.0), 1.0, 0.5, 2.0),
        &brick(Point3::new(-2.0, -0.5, 0.0), Point3::new(2.0, 2.0, 3.0)),
    );
    for (what, boss, post, refuses) in [
        (
            "D boss, post on the circle",
            &d_boss,
            ((0.85, -0.1), (1.15, 0.1)),
            true,
        ),
        (
            "D boss, post off the circle",
            &d_boss,
            ((1.3, -0.1), (1.6, 0.1)),
            false,
        ),
        (
            "segment, post across the circle",
            &segment,
            ((-0.1, 0.85), (0.1, 1.15)),
            true,
        ),
        (
            "segment, post off the circle",
            &segment,
            ((-0.1, 1.5), (0.1, 1.8)),
            false,
        ),
    ] {
        let ((x0, y0), (x1, y1)) = post;
        let body = fuse(
            &fuse(&finished("the plate", plate(), tol()), boss),
            &brick(Point3::new(x0, y0, 0.5), Point3::new(x1, y1, 2.5)),
        );
        let req = BlendRequest {
            body: &body,
            edges: arc_of(&body),
            size: 0.2,
        };
        assert!(!req.edges.is_empty(), "{what}: the arc");
        match (refuses, band_reach(&req, band())) {
            (true, Err(e)) => assert!(is_reach(&e), "{what}: {e:?}"),
            (false, Ok(())) => {}
            (_, out) => panic!("{what}: refuses {refuses}, got {out:?}"),
        }
    }
}
