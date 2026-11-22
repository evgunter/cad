//! R2 probes for PR 7432: rim junctions the glue door now routes before
//! reading anything along the rim. Each prints its verdict and, where a
//! body builds, its volume against the closed form.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use core::f64::consts::PI;

use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, BooleanResult};

fn body(what: &str, verts: Vec<(Point2<f64>, f64)>, rev: Revolution<f64>) -> AtRestBody<f64> {
    finished(what, revolved_about_y(verts, rev, Tol::witness()), Tol::witness())
}
fn p(x: f64, y: f64, b: f64) -> (Point2<f64>, f64) {
    (Point2::new(x, y), b)
}
fn q() -> f64 {
    (PI / 8.0).tan()
}
fn rod() -> AtRestBody<f64> {
    body("rod", vec![p(0.0, -2.0, 0.0), p(1.0, -2.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 0.0)], Revolution::Full)
}

/// Run a union both orders; print the verdict; return volumes of the
/// bodies that built.
fn report(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, want: f64) -> Vec<Result<f64, String>> {
    let mut out = Vec::new();
    for (order, x, y) in [("a∪b", a, b), ("b∪a", b, a)] {
        let r = match topo::union(x, y, Tol::witness()) {
            Ok(BooleanResult::Body(bb)) => {
                let t3 = topo::validate_geometric(&bb.body, Tol::witness());
                let t3p = topo::validate_pseudomanifold(&bb.body, &bb.contacts, Tol::witness());
                let v = topo::test_support::pinned_volume("u", &bb.body, Tol::witness()).volume;
                println!(
                    "R2PROBE {what} {order}: BODY solids={} vol={v} want={want} rel={:e} t3={:?} t3'={:?}",
                    bb.body.solids().count(),
                    (v - want).abs() / want,
                    t3.is_ok(),
                    t3p.is_ok()
                );
                Ok(v)
            }
            Ok(_) => {
                println!("R2PROBE {what} {order}: EMPTY");
                Err("other".into())
            }
            Err(e) => {
                println!("R2PROBE {what} {order}: REFUSED kind={:?} :: {e}", e.kind());
                Err(format!("{:?}", e.kind()))
            }
        };
        out.push(r);
    }
    out
}

#[track_caller]
fn never_wrong(what: &str, rs: &[Result<f64, String>], want: f64) {
    for r in rs {
        if let Ok(v) = r {
            assert!((v - want).abs() <= 1e-9 * want, "{what}: built a wrong volume {v} against {want}");
        }
    }
}

/// A rod whose top is a flange wider than the rod: its wall ends on the
/// rim r=1, y=0 at a concave edge. A half ball on y=0 leaves the rim G1
/// off the rod's wall (a π seam with the wall), but its sphere near the
/// rim lies inside the flange.
#[test]
fn r2_flanged_rod_and_half_ball() {
    let t = 0.3;
    let a = body(
        "flanged rod",
        vec![p(0.0, -2.0, 0.0), p(1.0, -2.0, 0.0), p(1.0, 0.0, 0.0), p(1.5, 0.0, 0.0), p(1.5, t, 0.0), p(0.0, t, 0.0)],
        Revolution::Full,
    );
    let b = body("half ball", vec![p(0.0, 0.0, 0.0), p(1.0, 0.0, q()), p(0.0, 1.0, 0.0)], Revolution::Full);
    let va = 2.0 * PI + PI * 1.5 * 1.5 * t;
    let h = 1.0 - t;
    let cap = PI * h * h * (3.0 - h) / 3.0;
    let want = va + cap;
    let rs = report("flanged rod ∪ half ball", &a, &b, want);
    never_wrong("flanged", &rs, want);
}

/// The full unit ball, its sphere split on y=0 (both hemispheres end on
/// the rod's rim): the upper one a π seam with the rod's wall, the lower
/// one leaving on the rod's side, inside the rod.
#[test]
fn r2_rod_and_full_ball_split_on_its_rim() {
    let b = body(
        "ball",
        vec![p(0.0, -1.0, q()), p(1.0, 0.0, q()), p(0.0, 1.0, 0.0)],
        Revolution::Full,
    );
    let want = 2.0 * PI + 2.0 / 3.0 * PI;
    let rs = report("rod ∪ split ball", &rod(), &b, want);
    never_wrong("split ball", &rs, want);
}

/// A half ball revolved a half turn only: its sphere ends on half the
/// rim, the rest of its boundary two half-disks. The rim is shared over
/// part of its length.
#[test]
fn r2_rod_and_half_of_a_half_ball() {
    let b = body("quarter ball", vec![p(0.0, 0.0, 0.0), p(1.0, 0.0, q()), p(0.0, 1.0, 0.0)], Revolution::Partial(PI / 2.0));
    let want = 2.0 * PI + 1.0 / 6.0 * PI;
    let rs = report("rod ∪ quarter ball", &rod(), &b, want);
    never_wrong("quarter ball", &rs, want);
}

/// A pipe (r 0.6..1, y∈[-2,0]) and a ball of radius 0.6 whose upper half
/// stands on the inner rim: outward normals opposed across the rim
/// (wedge 0/2π), the cusp arm.
#[test]
fn r2_pipe_and_half_ball_on_its_inner_rim() {
    let pipe = body(
        "pipe",
        vec![p(0.6, -2.0, 0.0), p(1.0, -2.0, 0.0), p(1.0, 0.0, 0.0), p(0.6, 0.0, 0.0)],
        Revolution::Full,
    );
    let b = body("small half ball", vec![p(0.0, 0.0, 0.0), p(0.6, 0.0, q()), p(0.0, 0.6, 0.0)], Revolution::Full);
    let want = PI * (1.0 - 0.36) * 2.0 + 2.0 / 3.0 * PI * 0.216;
    let rs = report("pipe ∪ small half ball", &pipe, &b, want);
    never_wrong("inner rim", &rs, want);
}

/// A bowl hanging in a tube's mouth: a ball of radius 1 whose LOWER half
/// sits in a pipe of inner radius 1 (outer 1.4), ending on the inner rim
/// and leaving on the pipe's side (departures Same, the nested touch).
#[test]
fn r2_pipe_and_bowl_in_its_mouth() {
    let pipe = body(
        "pipe",
        vec![p(1.0, -2.0, 0.0), p(1.4, -2.0, 0.0), p(1.4, 0.0, 0.0), p(1.0, 0.0, 0.0)],
        Revolution::Full,
    );
    let b = body("bowl", vec![p(0.0, -1.0, q()), p(1.0, 0.0, 0.0), p(0.0, 0.0, 0.0)], Revolution::Full);
    let want = PI * (1.96 - 1.0) * 2.0 + 2.0 / 3.0 * PI;
    let rs = report("pipe ∪ bowl", &pipe, &b, want);
    never_wrong("bowl", &rs, want);
}

/// A widening cone on the rod's cap (a crease of the other sign from the
/// frustum's): radius 1 at y=0 to 1.3 at y=1.
#[test]
fn r2_rod_and_widening_frustum() {
    let b = body("wide frustum", vec![p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(1.3, 1.0, 0.0), p(0.0, 1.0, 0.0)], Revolution::Full);
    let want = 2.0 * PI + PI / 3.0 * (1.0 + 1.3 + 1.69);
    let rs = report("rod ∪ wide frustum", &rod(), &b, want);
    never_wrong("wide frustum", &rs, want);
}

/// The half ball under subtract and intersect: rod ∖ half ball is the rod,
/// rod ∩ half ball is empty (touch only on the cap).
#[test]
fn r2_rod_and_half_ball_other_ops() {
    let b = body("half ball", vec![p(0.0, 0.0, 0.0), p(1.0, 0.0, q()), p(0.0, 1.0, 0.0)], Revolution::Full);
    let a = rod();
    for (name, r) in [
        ("rod ∖ half ball", topo::subtract(&a, &b, Tol::witness())),
        ("half ball ∖ rod", topo::subtract(&b, &a, Tol::witness())),
        ("rod ∩ half ball", topo::intersect(&a, &b, Tol::witness())),
    ] {
        match r {
            Ok(BooleanResult::Body(bb)) => {
                let v = topo::test_support::pinned_volume("u", &bb.body, Tol::witness()).volume;
                println!("R2PROBE {name}: BODY vol={v}");
            }
            Ok(_) => println!("R2PROBE {name}: EMPTY"),
            Err(e) => println!("R2PROBE {name}: REFUSED kind={:?} :: {e}", e.kind()),
        }
    }
}

/// A trumpet bell on the rod's cap: its wall a torus that leaves the rim
/// G1 off the rod's cylinder (a π junction) and flares outward, so the
/// rod's wall continued past the rim lies on the bell's MATERIAL side of
/// the bell's carrier while the bell has no material below the rim.
#[test]
fn r2_rod_and_flared_bell() {
    let rho: f64 = 0.5;
    let b = body(
        "bell",
        vec![p(0.0, 0.0, 0.0), p(1.0, 0.0, -q()), p(1.0 + rho, rho, 0.0), p(0.0, rho, 0.0)],
        Revolution::Full,
    );
    // ∫_0^ρ π (1+ρ−√(ρ²−y²))² dy, by Simpson on a fine grid.
    let n = 200_000;
    let h = rho / n as f64;
    let f = |y: f64| {
        let x = 1.0 + rho - (rho * rho - y * y).max(0.0).sqrt();
        PI * x * x
    };
    let mut s = f(0.0) + f(rho);
    for i in 1..n {
        s += f(i as f64 * h) * if i % 2 == 1 { 4.0 } else { 2.0 };
    }
    let vb = s * h / 3.0;
    let want = 2.0 * PI + vb;
    let rs = report("rod ∪ bell", &rod(), &b, want);
    never_wrong("bell", &rs, want);
}
