//! **A long bound beside a short arm.** Two plane faces meet at a
//! vertex whose corners each have a short edge, `s = 300ε` long, and a
//! long one; the long edges leave the vertex `θ` apart. The sectors'
//! arm is the short edge, at which `θ = 10⁻³` reads Zero and
//! `θ = 10⁻²` in band; the long edges' own reach reads `θ` definite.
//! The Boolean's sector passes read a direction's side of a bound at
//! the bound's reach (`boolean::sectors` `within_reaching`), so each
//! pose builds at its closed form
//! (`work/splitdoor/sector-decisions-read-at-the-arm-disagree-with-codes-read-at-the-reach.md`).
//!
//! - **A boss on a holed plate** ([`a_boss_poking_out_of_a_hole`]):
//!   the boss stands over a quad hole in the plate's top, sharing the
//!   hole's corner, its long edge `θ` outside the hole's (a wedge of
//!   the boss's floor on the plate's top) or inside it.
//! - **Two slabs side by side** ([`two_slabs_beside_a_short_arm`]):
//!   the hole's quad and the boss's as solids from one floor, their
//!   tops coplanar facing one way, one standing on the other's floor
//!   or both on one.
//!
//! Every op runs in both orders and is held `SOUND` at its closed form
//! (tiers 2 and 3′, the certificate, the legal operand,
//! `differential::outcome`) with tier 3 read on the body, or empty
//! where the closed form is. One run refuses, at every ε: the slab
//! difference that leaves two wedges of material on the vertex's
//! vertical edge, which the finished-body gate reads as overlapping
//! there (`LineWedges`, the re-pairing
//! `work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`
//! owes); it refuses so with the short edges `0.2` long too.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_4;

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanError, BooleanResult};

use crate::common::differential::{every_op_both_orders, outcome};

fn tol() -> Tol {
    Tol::witness()
}

/// `outline` (each a polygon) extruded from `z = z0` up by `h`.
fn prism(outline: &[&[(f64, f64)]], z0: f64, h: f64) -> AtRestBody<f64> {
    let loops: Vec<ProfileLoop<f64>> = outline
        .iter()
        .map(|p| RawLoop::polygon(p.iter().map(|&(x, y)| Point2::new(x, y))))
        .collect();
    let profile = Profile::new(SketchPlane::<f64>::xy(), loops)
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: h,
        side: ExtrudeSide::Along,
    };
    let body = extrude(&profile, depth, tol()).unwrap().body;
    let up = Affine3::translation(Vec3::new(0.0, 0.0, z0));
    let body =
        topo::transform_rigid(&body, &geom_core::test_support::rigid(&up, tol()), tol()).unwrap();
    topo::test_support::finished("a prism", body, tol())
}

fn area(p: &[(f64, f64)]) -> f64 {
    let n = p.len();
    let twice: f64 = (0..n)
        .map(|i| {
            let (a, b) = (p[i], p[(i + 1) % n]);
            a.0 * b.1 - b.0 * a.1
        })
        .sum();
    twice.abs() / 2.0
}

/// The hole's quad: its corner at the origin, its long edge along `+x`
/// and its short edge `s` along `+y`.
fn hole(s: f64) -> [(f64, f64); 4] {
    [(0.0, 0.0), (0.5, 0.0), (0.5, 0.5), (0.0, s)]
}

/// The boss's quad: its corner at the origin, its long edge `0.4` long
/// at `−θ` and its short edge `s` at 45°. It lies inside [`hole`] but
/// for the wedge between the two long edges where `θ > 0`.
fn boss(s: f64, theta: f64) -> [(f64, f64); 4] {
    [
        (0.0, 0.0),
        (0.4 * theta.cos(), -0.4 * theta.sin()),
        (0.35, 0.25),
        (s * FRAC_PI_4.cos(), s * FRAC_PI_4.sin()),
    ]
}

/// The area of [`boss`] outside [`hole`]: the triangle between the long
/// edges, out to where the boss's next edge crosses the hole's.
fn poking(s: f64, theta: f64) -> f64 {
    if theta <= 0.0 {
        return 0.0;
    }
    let [_, p, q, _] = boss(s, theta);
    let x = p.0 + (q.0 - p.0) * (-p.1) / (q.1 - p.1);
    0.5 * x * -p.1
}

/// One run held to its closed form `want`: `SOUND` with tier 3 read on
/// the body, or empty where `want` is zero.
fn holds(row: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64) {
    if let Ok(BooleanResult::Body(b)) = &r {
        let t3 = topo::validate_geometric(&b.body, tol());
        assert!(t3.is_ok(), "{row}: tier 3 {t3:?}");
    }
    let line = outcome(r, want, tol());
    let ok = if want == 0.0 {
        line.starts_with("EMPTY ok")
    } else {
        line.starts_with("OK SOUND")
    };
    assert!(ok, "{row}: {line}");
}

const THETAS: [f64; 4] = [1e-3, -1e-3, 1e-2, -1e-2];

/// [`boss`] standing on the plate over [`hole`], and their volumes.
fn boss_on_the_plate(theta: f64) -> (AtRestBody<f64>, AtRestBody<f64>, (f64, f64, f64)) {
    let s = 300.0 * tol().eps();
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let plate = prism(&[&square, &hole(s)], -0.5, 0.5);
    let b = prism(&[&boss(s, theta)], 0.0, 1.0);
    let v = (area(&boss(s, theta)), (4.0 - area(&hole(s))) * 0.5, 0.0);
    (b, plate, v)
}

/// **A boss poking out of a hole**: [`boss`] standing on the plate
/// `[−1, 1]² × [−0.5, 0]` over the through [`hole`], their corners at
/// one vertex. Where `θ > 0` the boss's floor rests on the plate's top
/// in the wedge, which `pair_search` reads overlapping at the long
/// edges' reach; at the arm, `θ = 10⁻²` refused `bool_sector_within`.
/// Boss `area(boss)`, plate `2 − area(hole)/2`, overlap 0: the unions
/// at `θ = 10⁻²`.
#[test]
fn a_boss_poking_out_of_a_hole() {
    let (b, plate, (vb, vp, _)) = boss_on_the_plate(1e-2);
    holds("A ∪ B", topo::union(&b, &plate, tol()), vb + vp);
    holds("B ∪ A", topo::union(&plate, &b, tol()), vb + vp);
}

/// [`a_boss_poking_out_of_a_hole`], every op at every `θ`.
#[test]
fn a_boss_beside_a_holes_long_edge_at_every_angle() {
    for theta in THETAS {
        let (b, plate, v) = boss_on_the_plate(theta);
        for (op, r, want) in every_op_both_orders(&b, &plate, v, tol()) {
            holds(&format!("θ = {theta}: {op}"), r, want);
        }
    }
}

/// **Two slabs beside a short arm**: [`hole`]'s quad as a slab
/// `z ∈ [−0.5, 0]`, and [`boss`]'s from `z = z0` up to `0`, so their
/// tops lie on one plane facing one way, and their floors too where
/// `z0 = −0.5`. Where `θ > 0` the boss pokes out of the slab in the
/// wedge, and every op builds; where `θ < 0` it lies inside, and the
/// slab less the boss leaves material either side of the boss's
/// corner on the vertical edge there, which refuses `LineWedges`
/// (module docs). At the arm, `θ = 10⁻²` refused `bool_sector_within`
/// and `θ = 10⁻³` the intersection `ClassificationInvariant` ("a fate
/// cut's end lies in no corner of the dropped face").
#[test]
fn two_slabs_beside_a_short_arm() {
    let s = 300.0 * tol().eps();
    for theta in THETAS {
        for z0 in [-0.5, -0.3] {
            let a = prism(&[&hole(s)], -0.5, 0.5);
            let b = prism(&[&boss(s, theta)], z0, -z0);
            let ab = area(&boss(s, theta));
            let v = (
                area(&hole(s)) * 0.5,
                ab * -z0,
                (ab - poking(s, theta)) * -z0,
            );
            for (op, r, want) in every_op_both_orders(&a, &b, v, tol()) {
                let row = format!("θ = {theta}, z0 = {z0}: {op}");
                if theta < 0.0 && op == "A ∖ B" {
                    assert!(
                        matches!(
                            r,
                            Err(BooleanError::LineWedges {
                                fault: topo::LineWedgeFault::Overlap { .. },
                                ..
                            })
                        ),
                        "{row}: {:?}",
                        r.map(|_| "built")
                    );
                    continue;
                }
                holds(&row, r, want);
            }
        }
    }
}
