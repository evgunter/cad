//! **A plate on the support plane of a curved solid, at the exact
//! support**: the row that makes the narrow phase's soundness red-able.
//!
//! `boolean::separating::apart` clears a pair when, along some
//! direction, the gap between two items' reaches decides positive. A
//! reach that under-covers its item by a hair clears a pair that
//! touches, and only a pair that touches can show it: every clear pair
//! in `operand_gate_pose.rs` has a gap of `0.13·s` or more. Here a thin
//! plate is set on the plane of a frustum's or a torus's support along
//! an oblique direction `d` (full and 270° turns), at a signed gap `δ`
//! read in closed form: a small positive one, zero, and `−ε`. The pair
//! is moved together to a pseudo-random rigid pose, at three scales,
//! and every op runs in both operand orders.
//!
//! The oracle is the closed form, never the kernel:
//!
//! - at `δ ≤ 0` the operands touch or overlap, so an op that builds
//!   them as disjoint (`∪` an `Assembly`, `∩` empty) is wrong;
//! - every built result is sampled with `point_in_solid` against the
//!   analytic membership of the two operands under the op, away from
//!   either boundary.
//!
//! A refusal is a right answer here. Shrinking every reach in
//! `apart_along` by 0.1% of its width at each end (the review's mutant
//! M3) clears the 270° torus at `s = 10⁻³`, direction 3, at `δ = 0`
//! and `δ = −ε`, where `∪` then ships an `Assembly` and `∩` comes back
//! empty: red at ε 10⁻⁹ and 10⁻¹². At ε 10⁻⁶ the pad covers that
//! shrink, and the row stays green.
//!
//! **Excluded**: a touch on a RIM circle — the 270° torus's cut-cap rim
//! (a direction whose support falls on a cut), and every support of the
//! frustum, which always falls on one of its four rims — at `δ = 0` or
//! `δ = −ε`, the two gaps inside the band, may build the point touch it
//! records: `∪` an `Assembly` whose contacts hold the touch, `∩` empty
//! (`work/tally/a-plate-touching-a-cut-torus-at-its-cap-rim-builds-an-assembly.md`).
//! The rim's graze root is its extremum, the support itself, so `−ε`
//! reads as `0` does. An excluded `∪` that records no contact is still
//! wrong, and every excluded result is still sampled.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{brick, revolved_about_y};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

fn lcg(seed: &mut u64) -> f64 {
    *seed = seed
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    ((*seed >> 11) as f64) / ((1u64 << 53) as f64)
}

/// A pseudo-random rigid pose: a turn about a random axis through the
/// origin, then a translation of up to `2s`.
fn pose(s: f64, seed: u64) -> Affine3<f64> {
    let mut st = seed;
    let ax = Vec3::new(lcg(&mut st) - 0.5, lcg(&mut st) - 0.5, lcg(&mut st) - 0.5).normalize();
    let ang = lcg(&mut st) * 2.0 * PI;
    let t = Vec3::new(
        (lcg(&mut st) - 0.5) * 4.0 * s,
        (lcg(&mut st) - 0.5) * 4.0 * s,
        (lcg(&mut st) - 0.5) * 4.0 * s,
    );
    Affine3::translation(t) * Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), ax, ang)
}

/// `body` moved by `map` and through the at-rest gate, or `None` where
/// the move or the gate cannot certify the moved body (a pose the row
/// skips: the question is the boolean's, not the move's).
fn posed(body: &Body<f64>, map: &Affine3<f64>) -> Option<AtRestBody<f64>> {
    use topo::AtRestPolicy;
    let b = topo::transform_rigid(body, map, tol()).ok()?;
    f64::gate_at_rest_kept(b, tol()).ok()
}

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    /// About y: inner cylinder `r = 0.2`, outer cone from `r = 0.6` at
    /// `y = 0` to `r = 0.4` at `y = 0.6`, scaled by `s`.
    Frustum,
    /// About y: tube centre `(0.6, 0.3)·s`, tube radius `0.2·s`.
    Torus,
}

fn body(shape: Shape, s: f64, sweep: f64) -> Body<f64> {
    let rev = if sweep < 2.0 * PI {
        Revolution::Partial(sweep)
    } else {
        Revolution::Full
    };
    let verts = match shape {
        Shape::Frustum => [(0.2, 0.0), (0.6, 0.0), (0.4, 0.6), (0.2, 0.6)]
            .into_iter()
            .map(|(r, y)| (Point2::new(r * s, y * s), 0.0))
            .collect(),
        Shape::Torus => vec![
            (Point2::new(0.4 * s, 0.3 * s), 1.0),
            (Point2::new(0.8 * s, 0.3 * s), 1.0),
        ],
    };
    revolved_about_y(verts, rev, tol())
}

/// A point at radius `rho`, height `y` and azimuth `u`: the solids
/// revolve about +y as `(ρ cos u, y, −ρ sin u)`, keeping `u ∈ [0, sweep]`.
fn rev_point(rho: f64, y: f64, u: f64) -> Point3<f64> {
    Point3::new(rho * u.cos(), y, -rho * u.sin())
}

/// The kept azimuth that maximises `d·(cos u, 0, −sin u)`, and whether
/// it falls on a cut (the unconstrained best lies outside the window).
fn azimuth_best(d: Vec3<f64>, sweep: f64) -> (f64, f64, bool) {
    let ustar = (-d.z).atan2(d.x).rem_euclid(2.0 * PI);
    let a = |u: f64| d.x * u.cos() - d.z * u.sin();
    if ustar <= sweep {
        (a(ustar), ustar, false)
    } else if a(0.0) >= a(sweep) {
        (a(0.0), 0.0, true)
    } else {
        (a(sweep), sweep, true)
    }
}

/// The solid's support point along the unit `d`, and whether it lies
/// on a cut.
fn support(shape: Shape, s: f64, sweep: f64, d: Vec3<f64>) -> (Point3<f64>, bool) {
    let (a, u, on_cut) = azimuth_best(d, sweep);
    let p = match shape {
        Shape::Frustum => {
            let mut best = (f64::NEG_INFINITY, Point3::new(0.0, 0.0, 0.0));
            for (y, rho_out) in [(0.0, 0.6 * s), (0.6 * s, 0.4 * s)] {
                for rho in [rho_out, 0.2 * s] {
                    let v = rho * a + d.y * y;
                    if v > best.0 {
                        best = (v, rev_point(rho, y, u));
                    }
                }
            }
            best.1
        }
        Shape::Torus => {
            let v = d.y.atan2(a);
            rev_point(0.6 * s + 0.2 * s * v.cos(), 0.3 * s + 0.2 * s * v.sin(), u)
        }
    };
    (p, on_cut)
}

/// Signed membership margin of the local point `p` (positive inside).
fn margin(shape: Shape, s: f64, sweep: f64, p: Point3<f64>) -> f64 {
    let rho = (p.x * p.x + p.z * p.z).sqrt();
    let az = if sweep >= 2.0 * PI {
        f64::INFINITY
    } else {
        let u = (-p.z).atan2(p.x).rem_euclid(2.0 * PI);
        rho * if u <= sweep {
            u.min(sweep - u)
        } else {
            -(u - sweep).min(2.0 * PI - u)
        }
    };
    let m = match shape {
        Shape::Frustum => (rho - 0.2 * s)
            .min(((0.6 * s - p.y / 3.0) - rho) * 3.0 / 10f64.sqrt())
            .min(p.y)
            .min(0.6 * s - p.y),
        Shape::Torus => 0.2 * s - (rho - 0.6 * s).hypot(p.y - 0.3 * s),
    };
    m.min(az)
}

/// The rotation taking `x̂` to the unit `d`.
fn x_to(d: Vec3<f64>) -> Affine3<f64> {
    let x = Vec3::new(1.0, 0.0, 0.0);
    let ax = x.cross(d);
    if ax.norm() < 1e-12 {
        return if x.dot(d) > 0.0 {
            Affine3::identity()
        } else {
            Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), PI)
        };
    }
    Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        ax.normalize(),
        x.dot(d).clamp(-1.0, 1.0).acos(),
    )
}

fn verdict(r: &Result<BooleanResult<f64>, BooleanError>) -> String {
    match r {
        Ok(BooleanResult::Empty) => "Empty".into(),
        Ok(BooleanResult::Body(b)) => format!("Body({:?})", b.kind),
        Err(e) => {
            let mut s = format!("{e:?}");
            s.truncate(120);
            s
        }
    }
}

#[test]
fn plates_at_the_exact_support_of_a_frustum_and_a_torus_are_right_at_every_op() {
    let eps = tol().eps();
    let band = geom_core::Band::linear(tol()).unwrap();
    let mut wrong = Vec::new();
    let (mut runs, mut built, mut sampled, mut skipped) = (0usize, 0usize, 0usize, 0usize);
    for (shape, sweep, name) in [
        (Shape::Frustum, 2.0 * PI, "frustum full"),
        (Shape::Frustum, 1.5 * PI, "frustum 270°"),
        (Shape::Torus, 2.0 * PI, "torus full"),
        (Shape::Torus, 1.5 * PI, "torus 270°"),
    ] {
        for s in [1e-3, 1.0, 1e3] {
            let curved = body(shape, s, sweep);
            let mut st = 0xC0FFEE ^ s.to_bits() ^ (sweep.to_bits() >> 3);
            for k in 0..6 {
                let d = Vec3::new(lcg(&mut st) - 0.5, lcg(&mut st) - 0.5, lcg(&mut st) - 0.5)
                    .normalize();
                let (p, on_cut) = support(shape, s, sweep, d);
                let pose = pose(s, st);
                let m = Affine3::translation(Vec3::new(p.x, p.y, p.z)) * x_to(d);
                let (inv_a, inv_b) = (pose.inverse(), (pose * m).inverse());
                let world_p = pose.transform_point(p);
                let Some(a) = posed(&curved, &pose) else {
                    skipped += 1;
                    continue;
                };
                for delta in [1e-6 * s, 0.0, -eps] {
                    let plate = brick::<f64>(
                        (delta, delta + 0.1 * s),
                        (-0.15 * s, 0.15 * s),
                        (-0.15 * s, 0.15 * s),
                        tol(),
                    );
                    let Some(b) = topo::transform_rigid(&plate, &m, tol())
                        .ok()
                        .and_then(|plate| posed(&plate, &pose))
                    else {
                        skipped += 1;
                        continue;
                    };
                    let in_a = |q| margin(shape, s, sweep, inv_a.transform_point(q));
                    let in_b = |q| {
                        let l = inv_b.transform_point(q);
                        (l.x - delta)
                            .min(delta + 0.1 * s - l.x)
                            .min(0.15 * s - l.y.abs())
                            .min(0.15 * s - l.z.abs())
                    };
                    let excluded = shape == Shape::Frustum || (sweep < 2.0 * PI && on_cut);
                    for (op, out) in [
                        ("a∪b", topo::union(&a, &b, tol())),
                        ("b∪a", topo::union(&b, &a, tol())),
                        ("a∩b", topo::intersect(&a, &b, tol())),
                        ("a∖b", topo::subtract(&a, &b, tol())),
                        ("b∖a", topo::subtract(&b, &a, tol())),
                    ] {
                        runs += 1;
                        let label = format!(
                            "[ε {eps:e}] {name} s {s:e} dir{k} {d:?} δ {delta:e} {op}: {}",
                            verdict(&out)
                        );
                        let Ok(res) = out else { continue };
                        built += 1;
                        if delta <= 0.0 {
                            let (disjoint, touch_recorded) = match &res {
                                BooleanResult::Empty => (true, true),
                                BooleanResult::Body(bb) => (
                                    bb.kind == topo::BooleanResultKind::Assembly,
                                    bb.contacts != topo::ContactRecords::default(),
                                ),
                            };
                            if disjoint && !(excluded && touch_recorded) {
                                wrong.push(format!("{label}: touching operands built as disjoint"));
                            }
                        }
                        // Sample about the support point, near and wide,
                        // skipping points within `10⁻⁶·s` of either
                        // operand's boundary.
                        let mut ps = st ^ 0x5eed ^ delta.to_bits();
                        let mut bad = 0;
                        for i in 0..32 {
                            let r = if i < 20 { 0.05 * s } else { 0.5 * s };
                            let q = Point3::new(
                                world_p.x + (lcg(&mut ps) - 0.5) * 2.0 * r,
                                world_p.y + (lcg(&mut ps) - 0.5) * 2.0 * r,
                                world_p.z + (lcg(&mut ps) - 0.5) * 2.0 * r,
                            );
                            let (x, y) = (in_a(q), in_b(q));
                            if x.abs() < 1e-6 * s || y.abs() < 1e-6 * s {
                                continue;
                            }
                            let (ia, ib) = (x > 0.0, y > 0.0);
                            let want = match op {
                                "a∪b" | "b∪a" => ia || ib,
                                "a∖b" => ia && !ib,
                                "b∖a" => ib && !ia,
                                _ => ia && ib,
                            };
                            let got = match &res {
                                BooleanResult::Empty => false,
                                BooleanResult::Body(bb) => {
                                    match topo::point_in_solid(&bb.body, q, band, tol()) {
                                        Ok(topo::SolidContainment::In) => true,
                                        Ok(topo::SolidContainment::Out) => false,
                                        // A point the classifier cannot
                                        // read is no reading.
                                        _ => continue,
                                    }
                                }
                            };
                            sampled += 1;
                            if got != want {
                                bad += 1;
                            }
                        }
                        if bad > 0 {
                            wrong.push(format!("{label}: {bad} sampled points misclassified"));
                        }
                    }
                }
            }
        }
    }
    println!("ε {eps:e}: runs {runs}, built {built}, sampled {sampled}, poses skipped {skipped}");
    assert!(
        wrong.is_empty(),
        "runs {runs}, built {built}, sampled {sampled}, poses skipped {skipped}:\n{}",
        wrong.join("\n")
    );
    assert!(
        built > 0 && sampled > 0,
        "runs {runs}, poses skipped {skipped}: nothing built, nothing checked"
    );
}
