//! Review probes for PR 5099 (a bar wholly inside a concave band's fill
//! is absorbed). Every probe runs the planar pocketed die at scale one
//! (pocket floor edge `(¼, ¼, ½) → (¾, ¼, ½)`, end walls `x = ¼, ¾`),
//! and holds the blend to `S ∪ F`: a build must be tier 3/3′ and carry
//! the closed-form volume; a crossing must refuse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Point3, Vec3};
use sweep::blend::BlendError;
use sweep::blend::build::{Blended, chamfer_edges, fillet_edges};
use sweep::test_support::{brick, finished, prism_on, sketch_from_axes};
use topo::{
    Body, ContactRecords, EdgeKey, mass_properties, query, validate_geometric,
    validate_pseudomanifold,
};

use crate::band_planar_cut_off::{D, Verb, tol};

const VERBS: [Verb; 2] = [Verb::Fillet, Verb::Chamfer];

fn boolean(union: bool, a: Body<f64>, b: Body<f64>) -> Body<f64> {
    let (a, b) = (finished("A", a, tol()), finished("B", b, tol()));
    if union {
        topo::union(&a, &b, tol())
    } else {
        topo::subtract(&a, &b, tol())
    }
    .unwrap_or_else(|e| panic!("the fixture's boolean builds, got {e}"))
    .body()
    .expect("a body")
    .body
    .clone()
    .into_body()
}

fn die() -> Body<f64> {
    let block = brick((-1.0, 2.0), (0.0, 1.0), (0.0, 1.0), tol());
    let pocket = prism_on(
        sketch_from_axes(
            Point3::new(0.0, 0.0, 0.5),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            tol(),
        ),
        [(0.25, 0.25), (0.75, 0.25), (0.75, 0.75), (0.25, 0.75)]
            .into_iter()
            .map(|(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
        1.0,
        tol(),
    );
    boolean(false, block, pocket)
}

/// A round rod along `x` from `x0` to `x1`, its axis at pocket-local
/// `(u, v)` = `(y − ¼, z − ½)`, radius `a`.
fn rod(x0: f64, x1: f64, (u, v): (f64, f64), a: f64) -> Body<f64> {
    prism_on(
        sketch_from_axes(
            Point3::new(x0, 0.25 + u, 0.5 + v),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            tol(),
        ),
        vec![(Point2::new(-a, 0.0), 1.0), (Point2::new(a, 0.0), 1.0)],
        x1 - x0,
        tol(),
    )
}

/// An axis-aligned box in pocket-local `(u, v)` over `x`.
fn lbox(x: (f64, f64), u: (f64, f64), v: (f64, f64)) -> Body<f64> {
    brick(x, (0.25 + u.0, 0.25 + u.1), (0.5 + v.0, 0.5 + v.1), tol())
}

fn floor(body: &Body<f64>) -> EdgeKey {
    let near = |p: Point3<f64>, q: [f64; 3]| {
        (p.x - q[0]).abs() + (p.y - q[1]).abs() + (p.z - q[2]).abs() < 1e-9
    };
    let (a, b) = ([0.25, 0.25, 0.5], [0.75, 0.25, 0.5]);
    query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).unwrap().he_plus;
            let s = *body
                .get_point(
                    body.get_vertex(body.get_half_edge(he).unwrap().start)
                        .unwrap()
                        .point,
                )
                .unwrap();
            let t = *body
                .get_point(
                    body.get_vertex(body.half_edge_end(he).unwrap())
                        .unwrap()
                        .point,
                )
                .unwrap();
            (near(s, a) && near(t, b)) || (near(s, b) && near(t, a))
        })
        .expect("the floor edge")
}

fn blend(verb: Verb, body: &Body<f64>) -> Result<Blended<f64>, BlendError> {
    let fb = finished("operand", body.clone(), tol());
    let e = [floor(body)];
    match verb {
        Verb::Fillet => fillet_edges(&fb, &e, D, tol()),
        Verb::Chamfer => chamfer_edges(&fb, &e, D, tol()),
    }
    .map_err(|e| e.error)
}

fn vol(body: &Body<f64>) -> f64 {
    let p = mass_properties(body, tol()).expect("props");
    p.volume
}

fn section(verb: Verb) -> f64 {
    verb.section_at(D, PI / 2.0)
}

/// Builds right, or refuses: a build is held to tier 3/3′, one genus-0
/// shell (the bar's handle gone, no void left), and `ΔV = added(verb)`.
fn held(
    what: &str,
    body: &Body<f64>,
    added: impl Fn(Verb) -> f64,
    must_refuse: bool,
) -> Vec<String> {
    validate_geometric(body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3 {e:?}"));
    let mut log = Vec::new();
    for verb in VERBS {
        match blend(verb, body) {
            Err(e) => log.push(format!("{what} {verb:?}: refuses {e}")),
            Ok(out) => {
                let t3 = validate_geometric(&out.body, tol());
                let t3p = validate_pseudomanifold(&out.body, &ContactRecords::default(), tol());
                let c = topo::readback::euler_counts(&out.body);
                let dv = vol(&out.body) - vol(body);
                let want = added(verb);
                log.push(format!(
                    "{what} {verb:?}: BUILDS t3 {:?} t3' {:?} shells {} genus {:?} dV {dv:.12} want {want:.12}",
                    t3.is_ok(), t3p.is_ok(), c.s, c.genus()
                ));
                assert!(
                    !must_refuse,
                    "{what} {verb:?}: a crossing builds: {}",
                    log.last().unwrap()
                );
                assert!(
                    t3.is_ok() && t3p.is_ok(),
                    "{what} {verb:?}: tiers {}",
                    log.last().unwrap()
                );
                assert_eq!(
                    (c.s, c.genus()),
                    (1, Ok(0)),
                    "{what} {verb:?}: {}",
                    log.last().unwrap()
                );
                assert!(
                    (dv - want).abs() < 1e-8,
                    "{what} {verb:?}: ΔV {}",
                    log.last().unwrap()
                );
            }
        }
    }
    for l in &log {
        eprintln!("{l}");
    }
    log
}

const C: (f64, f64) = (0.012, 0.012);
const A: f64 = 0.006;

fn barred() -> Body<f64> {
    boolean(true, die(), rod(0.1, 0.9, C, A))
}

/// A tube: the bar with a sealed void inside it, wholly inside `F`.
/// `S ∪ F` fills the void: ΔV = A·L − (πa² L − πb² ℓ).
#[test]
fn p1_a_hollow_bar_with_a_sealed_void_inside_the_fill() {
    let b = 0.003;
    let body = boolean(false, barred(), rod(0.3, 0.7, C, b));
    assert_eq!(
        topo::readback::euler_counts(&body).s,
        2,
        "the void is a shell"
    );
    held(
        "p1 sealed tube",
        &body,
        |v| section(v) * 0.5 - (PI * A * A * 0.5 - PI * b * b * 0.4),
        false,
    );
}

/// The tube's bore runs on through both end walls into the block: the
/// void crosses the prism's cap planes, so the result `S ∪ F` keeps the
/// bore's two ends past the walls; unbuilt, it must refuse.
#[test]
fn p1b_a_tube_whose_bore_runs_past_the_end_walls_refuses() {
    let b = 0.003;
    let body = boolean(false, barred(), rod(0.15, 0.85, C, b));
    held("p1b tube through", &body, |_| f64::NAN, true);
}

/// A bump on the bar wholly inside `F`: absorbed (ΔV = A·L − bar − bump
/// outside bar) or refused typed.
#[test]
fn p2_a_bar_with_a_bump_inside_the_fill() {
    let body = boolean(
        true,
        barred(),
        lbox((0.45, 0.55), (0.009, 0.015), (0.012, 0.024)),
    );
    let v_src_minus_die = vol(&body) - vol(&die());
    held(
        "p2 bump inside",
        &body,
        |v| section(v) * 0.5 - v_src_minus_die,
        false,
    );
}

/// A bump on the bar that crosses the fillet's arc mid-length (corner
/// `(0.015, 0.06)` is `0.094` from the arc's centre): must refuse.
#[test]
fn p3_a_bar_with_a_bump_across_the_band_surface_refuses() {
    let body = boolean(
        true,
        barred(),
        lbox((0.45, 0.55), (0.009, 0.015), (0.012, 0.06)),
    );
    held("p3 bump across (fillet)", &body, |_| f64::NAN, true);
}

/// The same bump across, a hair from an end wall.
#[test]
fn p4_a_bump_across_the_band_surface_beside_an_end_wall_refuses() {
    let body = boolean(
        true,
        barred(),
        lbox((0.26, 0.28), (0.009, 0.015), (0.012, 0.06)),
    );
    held("p4 bump across near wall", &body, |_| f64::NAN, true);
}

/// Two bars inside `F`: both absorbed, ΔV = (A − πa² − πa'²)·L.
#[test]
fn p5_two_bars_inside_the_fill() {
    let body = boolean(true, barred(), rod(0.1, 0.9, (0.03, 0.008), 0.004));
    let log = held(
        "p5 two bars",
        &body,
        |v| (section(v) - PI * A * A - PI * 0.004 * 0.004) * 0.5,
        false,
    );
    assert!(
        log.iter().all(|l| l.contains("BUILDS")),
        "p5: both bars are absorbed under both verbs: {log:?}"
    );
}

/// An island, a free box wholly inside `F`: step 4, unbuilt; a build
/// must absorb it (ΔV = A·L − box).
#[test]
fn p6_an_island_inside_the_fill() {
    let body = boolean(
        true,
        die(),
        lbox((0.4, 0.6), (0.005, 0.015), (0.005, 0.015)),
    );
    assert_eq!(
        topo::readback::euler_counts(&body).s,
        2,
        "the island is a shell"
    );
    let island = 0.2 * 0.01 * 0.01;
    // genus check in `held` expects one shell, so a build that keeps the island fails it.
    held("p6 island", &body, |v| section(v) * 0.5 - island, false);
}

/// A bar `dist` from the fillet's arc (negative: past it, into the
/// ball), on the bisector, radius `a`, at the band's escalate widths.
/// Past the arc, a build is wrong by a sliver: it must refuse. Inside,
/// a build is held to the closed form.
#[test]
fn p7_a_bar_an_escalate_width_either_side_of_the_arc() {
    let a = 0.004;
    for dist in [-2e-8, -5e-9, -1e-9, 1e-9, 5e-9, 2e-8] {
        let rho = (D + dist + a) / 2f64.sqrt();
        let c = (D - rho, D - rho);
        let body = boolean(true, die(), rod(0.1, 0.9, c, a));
        let fb = finished("operand", body.clone(), tol());
        let got = fillet_edges(&fb, &[floor(&body)], D, tol()).map_err(|e| e.error);
        match got {
            Err(e) => eprintln!("p7 dist {dist:e}: refuses {e}"),
            Ok(out) => {
                let dv = vol(&out.body) - vol(&body);
                let want = (section(Verb::Fillet) - PI * a * a) * 0.5;
                eprintln!("p7 dist {dist:e}: BUILDS dV {dv:.12} want {want:.12}");
                assert!(
                    dist > 0.0,
                    "p7 dist {dist:e}: a bar past the arc is absorbed"
                );
                assert!(
                    validate_geometric(&out.body, tol()).is_ok(),
                    "p7 {dist:e}: tier 3"
                );
                assert!((dv - want).abs() < 1e-8, "p7 {dist:e}: ΔV");
            }
        }
    }
}

/// The union of a mitre's bands: the floor edge and the floor–end-wall
/// edge at `x = ¼` in one request, the bar's cycle on `x = ¼` now on a
/// support of the second band. Either refuses typed or builds right.
#[test]
fn p8_a_bar_under_the_union_of_two_concave_bands() {
    let body = barred();
    let near = |p: Point3<f64>, q: [f64; 3]| {
        (p.x - q[0]).abs() + (p.y - q[1]).abs() + (p.z - q[2]).abs() < 1e-9
    };
    let e2 = query::all_edges(&body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).unwrap().he_plus;
            let s = *body
                .get_point(
                    body.get_vertex(body.get_half_edge(he).unwrap().start)
                        .unwrap()
                        .point,
                )
                .unwrap();
            let t = *body
                .get_point(
                    body.get_vertex(body.half_edge_end(he).unwrap())
                        .unwrap()
                        .point,
                )
                .unwrap();
            let (a, b) = ([0.25, 0.25, 0.5], [0.25, 0.75, 0.5]);
            (near(s, a) && near(t, b)) || (near(s, b) && near(t, a))
        })
        .expect("the end wall's floor edge");
    let fb = finished("operand", body.clone(), tol());
    for verb in VERBS {
        let edges = [floor(&body), e2];
        let got = match verb {
            Verb::Fillet => fillet_edges(&fb, &edges, D, tol()),
            Verb::Chamfer => chamfer_edges(&fb, &edges, D, tol()),
        }
        .map_err(|e| e.error);
        match got {
            Err(e) => eprintln!("p8 {verb:?}: refuses {e}"),
            Ok(out) => {
                let t3 = validate_geometric(&out.body, tol());
                let c = topo::readback::euler_counts(&out.body);
                eprintln!(
                    "p8 {verb:?}: BUILDS t3 {:?} shells {} genus {:?} dV {}",
                    t3.is_ok(),
                    c.s,
                    c.genus(),
                    vol(&out.body) - vol(&body)
                );
                assert!(t3.is_ok(), "p8 {verb:?}: tier 3");
            }
        }
    }
}

/// The die with end walls leaned `x = x₀ + s·(y − ¼)`.
fn leaned_die(s: f64) -> Body<f64> {
    let block = brick((-1.0, 2.0), (0.0, 1.0), (0.0, 1.0), tol());
    let pocket = prism_on(
        sketch_from_axes(
            Point3::new(0.0, 0.0, 0.5),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            tol(),
        ),
        [
            (0.25, 0.25),
            (0.75, 0.25),
            (0.75 + s / 2.0, 0.75),
            (0.25 + s / 2.0, 0.75),
        ]
        .into_iter()
        .map(|(x, y)| (Point2::new(x, y), 0.0))
        .collect(),
        1.0,
        tol(),
    );
    boolean(false, block, pocket)
}

/// A round rod of radius `a` on the segment `p → q` (world), run on
/// `over` past each end along its axis; its seams along `u` turned
/// square to the axis.
fn rod_on(p: Point3<f64>, q: Point3<f64>, over: f64, a: f64, u: Vec3<f64>) -> Body<f64> {
    let t = q - p;
    let th = t.normalize();
    let o = p - th * over;
    let u = (u - th * u.dot(th)).normalize();
    let v = th.cross(u);
    prism_on(
        sketch_from_axes(o, u, v, tol()),
        vec![(Point2::new(-a, 0.0), 1.0), (Point2::new(a, 0.0), 1.0)],
        t.norm() + 2.0 * over,
        tol(),
    )
}

/// **The PR's tilted fillet bar at leaned end walls**: the axis touches
/// the arc at mid-length (the bar runs `a` into the ball there), its
/// ends `h` either side along the arc's tangent on each leaned wall.
/// It crosses the band surface, so it must refuse.
#[test]
fn p9_a_tilted_bar_through_the_band_surface_at_leaned_walls_refuses() {
    let (r, h, a) = (D, 0.0255, 0.002);
    let s2 = 2f64.sqrt();
    let touch = (r - r / s2, r - r / s2);
    let p = (touch.0 - h / s2, touch.1 + h / s2);
    let q = (touch.0 + h / s2, touch.1 - h / s2);
    for s in [0.0, 0.3, 1.0, -1.0] {
        let w = |x0: f64, (u, v): (f64, f64)| Point3::new(x0 + s * u, 0.25 + u, 0.5 + v);
        let body = boolean(
            true,
            leaned_die(s),
            rod_on(w(0.25, p), w(0.75, q), 0.15, a, Vec3::new(0.0, 1.0, -1.0)),
        );
        validate_geometric(&body, tol()).expect("source tier 3");
        assert_eq!(
            topo::readback::euler_counts(&body).genus(),
            Ok(1),
            "s {s}: a handle"
        );
        for verb in VERBS {
            let fb = finished("operand", body.clone(), tol());
            let got = match verb {
                Verb::Fillet => fillet_edges(&fb, &[floor(&body)], D, tol()),
                Verb::Chamfer => chamfer_edges(&fb, &[floor(&body)], D, tol()),
            }
            .map_err(|e| e.error);
            match got {
                Err(e) => {
                    eprintln!("p9 s {s} {verb:?}: refuses {e}");
                    assert!(
                        matches!(verb, Verb::Fillet),
                        "p9 s {s}: the chamfer's tilted bar is inside its triangle and is absorbed, got {e}"
                    );
                }
                Ok(out) => {
                    let dv = vol(&out.body) - vol(&body);
                    eprintln!("p9 s {s} {verb:?}: BUILDS dV {dv:.12}");
                    assert!(
                        matches!(verb, Verb::Chamfer),
                        "p9 s {s}: the fillet absorbs a bar through its band surface"
                    );
                    // Chamfer: the bar is inside the chord's triangle (the arc's
                    // tangent point is `r(1 − 1/√2) ≈ 0.029` from each support,
                    // the chord's nearest `r/2 = 0.05`). ΔV = A·L − πa²·ℓ.
                    let ell = (w(0.75, q) - w(0.25, p)).norm();
                    let want = section(verb) * 0.5 - PI * a * a * ell;
                    assert!(
                        (dv - want).abs() < 1e-8,
                        "p9 s {s}: chamfer ΔV {dv} vs {want}"
                    );
                    assert!(
                        validate_geometric(&out.body, tol()).is_ok(),
                        "p9 s {s}: tier 3"
                    );
                }
            }
        }
    }
}
