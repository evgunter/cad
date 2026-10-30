//! Review probes for PR 6053 (`splitdoor/arm-vs-reach`): poses around a
//! long bound beside a short arm the PR's witnesses do not draw — a
//! curved bound, bounds of very different lengths, near-coplanar faces
//! and a reflex corner split at its bisector. Each run is held to its
//! closed form, or, where none is written (the tilted poses), to
//! inclusion–exclusion across the ops; a refusal must not name an
//! invariant.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_4, TAU};

use geom_core::{Affine3, Arc2, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::AtRestBody;

use crate::common::differential::{Verdict, every_op_both_orders, judged};

fn tol() -> Tol {
    Tol::witness()
}

fn poly(p: &[(f64, f64)]) -> ProfileLoop<f64> {
    RawLoop::polygon(p.iter().map(|&(x, y)| Point2::new(x, y)))
}

/// A one-segment circle of radius `r` about `c`, its vertex at `az`.
fn circle(c: (f64, f64), r: f64, az: f64) -> ProfileLoop<f64> {
    RawLoop::new([(
        Point2::new(c.0 + r * az.cos(), c.1 + r * az.sin()),
        Segment::Arc(Arc2 {
            centre: Point2::new(c.0, c.1),
            radius: r,
            sweep: TAU,
        }),
    )])
}

fn prism(
    loops: Vec<ProfileLoop<f64>>,
    z0: f64,
    h: f64,
    turn: Option<Affine3<f64>>,
) -> AtRestBody<f64> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), loops)
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: h,
        side: ExtrudeSide::Along,
    };
    let body = extrude(&profile, depth, tol()).unwrap().body;
    let mut map = Affine3::translation(Vec3::new(0.0, 0.0, z0));
    if let Some(t) = turn {
        map = t * map;
    }
    let body =
        topo::transform_rigid(&body, &geom_core::test_support::rigid(&map, tol()), tol()).unwrap();
    topo::test_support::finished("a prism", body, tol())
}

fn area(p: &[(f64, f64)]) -> f64 {
    let n = p.len();
    (0..n)
        .map(|i| {
            let (a, b) = (p[i], p[(i + 1) % n]);
            a.0 * b.1 - b.0 * a.1
        })
        .sum::<f64>()
        / 2.0
}

/// `subject` clipped to the convex CCW polygon `clip` (Sutherland–Hodgman).
fn clip(subject: &[(f64, f64)], clip: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut out = subject.to_vec();
    for i in 0..clip.len() {
        let (a, b) = (clip[i], clip[(i + 1) % clip.len()]);
        let side = |p: (f64, f64)| (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
        let input = core::mem::take(&mut out);
        for j in 0..input.len() {
            let (p, q) = (input[j], input[(j + 1) % input.len()]);
            let (sp, sq) = (side(p), side(q));
            if sp >= 0.0 {
                out.push(p);
            }
            if (sp >= 0.0) != (sq >= 0.0) {
                let t = sp / (sp - sq);
                out.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
            }
        }
    }
    out
}

/// The boss quad: long edge `lb` at `−θ`, short edge `s` at 45°.
fn boss(s: f64, lb: f64, theta: f64) -> [(f64, f64); 4] {
    [
        (0.0, 0.0),
        (lb * theta.cos(), -lb * theta.sin()),
        (lb * 0.875, lb * 0.625),
        (s * FRAC_PI_4.cos(), s * FRAC_PI_4.sin()),
    ]
}

/// Runs every op both orders; returns the failures (BAD, wrong empty,
/// an invariant refusal, or an inclusion–exclusion break).
fn run(
    row: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    v: (f64, f64, Option<f64>),
) -> Vec<String> {
    let mut bad = Vec::new();
    let ops = every_op_both_orders(a, b, (v.0, v.1, v.2.unwrap_or(f64::NAN)), tol());
    let mut vols = std::collections::BTreeMap::new();
    for (op, r, want) in ops {
        let inv = matches!(&r, Err(e) if format!("{e:?}").contains("Invariant"));
        let j = judged(r, v.2.map(|_| want), tol());
        println!("{row}: {op}: {}", j.line);
        if let Some(vol) = j.volume {
            vols.insert(op, vol);
        }
        let fail = inv
            || matches!(
                j.verdict,
                Verdict::Bad | Verdict::EmptyWrong | Verdict::Unmeasured
            );
        if fail {
            bad.push(format!("{row}: {op}: {}", j.line));
        }
    }
    // Inclusion–exclusion where no closed form is written.
    if v.2.is_none() {
        let g = |k: &str| vols.get(k).copied();
        if let (Some(u), Some(i)) = (g("A ∪ B"), g("A ∩ B")) {
            if (u + i - v.0 - v.1).abs() > 1e-7 {
                bad.push(format!("{row}: ∪ + ∩ = {} ≠ {}", u + i, v.0 + v.1));
            }
            if let Some(d) = g("A ∖ B") {
                if (d + i - v.0).abs() > 1e-7 {
                    bad.push(format!("{row}: A∖B + A∩B = {} ≠ {}", d + i, v.0));
                }
            }
            if let Some(d) = g("B ∖ A") {
                if (d + i - v.1).abs() > 1e-7 {
                    bad.push(format!("{row}: B∖A + A∩B = {} ≠ {}", d + i, v.1));
                }
            }
        }
    }
    bad
}

/// Fails on what `known` does not name; prints both.
fn report(bad: Vec<String>, known: impl Fn(&str) -> bool) {
    let (known, bad): (Vec<_>, Vec<_>) = bad.into_iter().partition(|b| known(b));
    for b in &known {
        println!("KNOWN {b}");
    }
    for b in &bad {
        println!("FAIL {b}");
    }
    assert!(bad.is_empty(), "{} failures", bad.len());
}

/// No failure is known.
fn none(_: &str) -> bool {
    false
}

const THETAS: [f64; 6] = [1e-4, -1e-4, 1e-3, -1e-3, 1e-2, -1e-2];

/// Bounds of very different lengths: the hole's long edge `lh`, the
/// boss's `lb`, the hole's short edge `sh` (the boss's is `300ε`).
#[test]
fn review_lengths() {
    let s = 300.0 * tol().eps();
    let mut bad = Vec::new();
    for (lh, lb) in [(1.9, 0.4), (1.9, 0.01), (0.5, 0.002), (0.02, 0.4)] {
        for sh in [s, 0.2] {
            for theta in THETAS {
                let h = [(0.0, 0.0), (lh, 0.0), (lh, 0.5), (0.0, sh)];
                let bq = boss(s, lb, theta);
                let a = prism(vec![poly(&h)], -0.5, 0.5, None);
                let b = prism(vec![poly(&bq)], -0.3, 0.3, None);
                let v = (
                    area(&h) * 0.5,
                    area(&bq) * 0.3,
                    Some(area(&clip(&bq, &h)) * 0.3),
                );
                bad.extend(run(
                    &format!("lh={lh} lb={lb} sh={sh} θ={theta}"),
                    &a,
                    &b,
                    v,
                ));
            }
        }
    }
    report(bad, none);
}

/// A curved bound: a disc of radius `r` through the origin, its vertex
/// there, centred above (`+r`, bending toward the boss) or below
/// (`−r`, bending away), as a slab beside the boss's slab.
#[test]
fn review_curved() {
    let s = 300.0 * tol().eps();
    let mut bad = Vec::new();
    for (r, lb) in [(0.5, 0.4), (0.05, 0.04), (0.05, 0.4), (2.0, 0.01)] {
        for c in [r, -r] {
            for theta in THETAS {
                let bq = boss(s, lb, theta);
                let a = prism(
                    vec![circle(
                        (0.0, c),
                        r,
                        if c > 0.0 {
                            -core::f64::consts::FRAC_PI_2
                        } else {
                            core::f64::consts::FRAC_PI_2
                        },
                    )],
                    -0.5,
                    0.5,
                    None,
                );
                let b = prism(vec![poly(&bq)], -0.3, 0.3, None);
                let ngon: Vec<_> = (0..20_000)
                    .map(|k| {
                        let t = TAU * f64::from(k) / 20_000.0;
                        (r * t.cos(), c + r * t.sin())
                    })
                    .collect();
                let v = (
                    core::f64::consts::PI * r * r * 0.5,
                    area(&bq) * 0.3,
                    Some(area(&clip(&bq, &ngon)) * 0.3),
                );
                bad.extend(run(&format!("r={r} c={c} lb={lb} θ={theta}"), &a, &b, v));
            }
        }
    }
    // On main too: the two-lump union touching along a line fails tier
    // 3′ on `CensusUndecidable { MayOverlap }` (curved faces of two lumps
    // in reach), as it does at θ = −0.1; and the disc below with the
    // boss dipping 1e-4 into it refuses an invariant (main as here).
    report(bad, |b| {
        (b.contains(" c=-") && b.contains("∪") && b.contains("t3p=false"))
            || b.contains("surviving record with an On code after srecledges")
    });
}

/// A holed plate with the disc's hole and the boss standing on it.
#[test]
fn review_curved_plate() {
    let s = 300.0 * tol().eps();
    let mut bad = Vec::new();
    for (r, lb) in [(0.5, 0.4), (0.05, 0.04)] {
        for theta in THETAS {
            let bq = boss(s, lb, theta);
            let sq = [(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)];
            let plate = prism(
                vec![
                    poly(&sq),
                    circle((0.0, r), r, -core::f64::consts::FRAC_PI_2),
                ],
                -0.5,
                0.5,
                None,
            );
            let b = prism(vec![poly(&bq)], 0.0, 1.0, None);
            let v = (
                (16.0 - core::f64::consts::PI * r * r) * 0.5,
                area(&bq),
                Some(0.0),
            );
            bad.extend(run(
                &format!("plate r={r} lb={lb} θ={theta}"),
                &plate,
                &b,
                v,
            ));
        }
    }
    report(bad, none);
}

/// Near-coplanar: the boss's slab turned by `φ` about `x` or `y` through
/// the shared vertex.
#[test]
fn review_tilted() {
    let s = 300.0 * tol().eps();
    let mut bad = Vec::new();
    for axis in [Vec3::unit_x(), Vec3::unit_y()] {
        for phi in [1e-2, -1e-2, 5e-2, -5e-2, 0.2, -0.2] {
            for theta in [1e-3, -1e-3, 1e-2, -1e-2] {
                let h = [(0.0, 0.0), (0.5, 0.0), (0.5, 0.5), (0.0, s)];
                let bq = boss(s, 0.4, theta);
                let t = Affine3::rotation_about_axis(Point3::origin(), axis, phi);
                let a = prism(vec![poly(&h)], -0.5, 0.5, None);
                let b = prism(vec![poly(&bq)], -0.3, 0.3, Some(t));
                let v = (area(&h) * 0.5, area(&bq) * 0.3, None);
                bad.extend(run(&format!("axis={axis:?} φ={phi} θ={theta}"), &a, &b, v));
            }
        }
    }
    // On main too (at θ = ±1e-3; main refuses the larger θ): turned by
    // −0.05 about `y`, the boss's top holds the slab's short edge, and
    // the union and `B ∖ A` are no legal operand (`bool_plane_parallel`
    // in band, margin 1.7e-9, against a far brick though each operand
    // alone unites with it), and `A ∖ B` at θ = −1e-3 fails tier 3′.
    report(bad, |b| {
        b.contains("y: 1.0")
            && b.contains("φ=-0.05")
            && (b.contains("operand=false") || b.contains("t3p=false"))
    });
}

/// A reflex corner split at its bisector: `A`'s corner at the origin
/// runs 0°..250°, its edge in short (`sh`) at 250°.
#[test]
fn review_reflex() {
    let s = 300.0 * tol().eps();
    let mut bad = Vec::new();
    for sh in [s, 0.05] {
        for theta in THETAS {
            let a250 = 250f64.to_radians();
            let ap = [
                (0.0, 0.0),
                (0.5, 0.0),
                (0.5, 0.5),
                (-0.5, 0.5),
                (-0.5, -0.5),
                (0.1, -0.5),
                (sh * a250.cos(), sh * a250.sin()),
            ];
            // Reorder so the last point precedes the origin: already does.
            let bq = boss(s, 0.4, theta);
            let sq = [(0.0, 0.0), (0.5, 0.0), (0.5, 0.5), (0.0, 0.5)];
            let a = prism(vec![poly(&ap)], -0.5, 0.5, None);
            let b = prism(vec![poly(&bq)], -0.3, 0.3, None);
            let v = (
                area(&ap) * 0.5,
                area(&bq) * 0.3,
                Some(area(&clip(&bq, &sq)) * 0.3),
            );
            bad.extend(run(&format!("reflex sh={sh} θ={theta}"), &a, &b, v));
        }
    }
    report(bad, none);
}
