//! JOIN-2 fix-pass delta review (`join/2-zip-reads-segments-delta-review`):
//! a battery of the reviewer's own shape. Rounded plates (bulge loops with
//! declared tangent joints) stacked flush on rounded or sharp plates, so
//! the contact is REST and the join hands what it cannot pair to the zip:
//! combs whose prong tips are pierce rings in one face, rings reached from
//! two sides, concentric and equal fillets (paired far ends, `coincide`),
//! and fillets tangent at one point with different radii (the like-far-
//! ends tie). Every op in both operand orders, one
//! `common::differential::outcome` line each, so the same file built on
//! main and on the head diffs line for line. The volume oracle is closed
//! form (shoelace plus circular segments); the interiors are disjoint, so
//! union = va + vb, a∖b = va, b∖a = vb, a∩b = ∅.
//!
//! Run: `cargo test --release -p sweep --test all join2_d_ -- --ignored
//! --nocapture`.

use super::common::differential::outcome;
use geom_core::{Point2, Tol};
use profile::RawLoop;
use sweep::test_support::{extruded, sketch_at};
use topo::Body;

fn tol() -> Tol {
    Tol::witness()
}

type Loop = Vec<((f64, f64), f64)>;

fn bulge_area(v: &[((f64, f64), f64)]) -> f64 {
    let n = v.len();
    let mut s = 0.0;
    for i in 0..n {
        let ((ax, ay), b) = v[i];
        let ((bx, by), _) = v[(i + 1) % n];
        s += (ax * by - bx * ay) / 2.0;
        if b != 0.0 {
            let c = ((bx - ax).powi(2) + (by - ay).powi(2)).sqrt();
            let th = 4.0 * b.abs().atan();
            let r = c / (2.0 * (th / 2.0).sin());
            s += b.signum() * r * r / 2.0 * (th - th.sin());
        }
    }
    s
}

fn slab(v: &[((f64, f64), f64)], z0: f64) -> Body<f64> {
    let n = v.len();
    let verts = v
        .iter()
        .map(|&((x, y), b)| (Point2::new(x, y), b))
        .collect();
    let mut joints: Vec<usize> = Vec::new();
    for (k, &(_, b)) in v.iter().enumerate() {
        // A notch centred on the corner meets its edges square: no joint.
        if b > 0.0 {
            for j in [k, (k + 1) % n] {
                if !joints.contains(&j) {
                    joints.push(j);
                }
            }
        }
    }
    joints.sort_unstable();
    let lp = profile::test_support::bulge_loop(verts).with_tangent_joints(joints);
    extruded(sketch_at(z0), vec![lp], 1.0, tol())
}

/// Quarter-turn bulge (CCW-convex).
fn q() -> f64 {
    (core::f64::consts::PI / 8.0).tan()
}

/// `[x0,x1]×[y0,y1]`, each corner rounded by its own radius (0 = sharp),
/// corners SW, SE, NE, NW. Negative radius: a concave quarter notch.
fn rrect(x0: f64, x1: f64, y0: f64, y1: f64, r: [f64; 4]) -> Loop {
    let mut v: Loop = Vec::new();
    let corner = |v: &mut Loop, p: (f64, f64), din: (f64, f64), dout: (f64, f64), r: f64| {
        if r == 0.0 {
            v.push((p, 0.0));
        } else if r > 0.0 {
            v.push(((p.0 - din.0 * r, p.1 - din.1 * r), q()));
            v.push(((p.0 + dout.0 * r, p.1 + dout.1 * r), 0.0));
        } else {
            let r = -r;
            v.push(((p.0 - din.0 * r, p.1 - din.1 * r), -q()));
            v.push(((p.0 + dout.0 * r, p.1 + dout.1 * r), 0.0));
        }
    };
    corner(&mut v, (x0, y0), (0.0, -1.0), (1.0, 0.0), r[0]);
    corner(&mut v, (x1, y0), (1.0, 0.0), (0.0, 1.0), r[1]);
    corner(&mut v, (x1, y1), (0.0, 1.0), (-1.0, 0.0), r[2]);
    corner(&mut v, (x0, y1), (-1.0, 0.0), (0.0, -1.0), r[3]);
    v
}

fn poly(p: &[(f64, f64)]) -> Loop {
    p.iter().map(|&p| (p, 0.0)).collect()
}

/// A comb: base `[x0,x1]×[y0,y0+t]`, prongs of width `pw` rising to `top`
/// at the given left edges (strictly inside the base).
fn comb(x0: f64, x1: f64, y0: f64, t: f64, prongs: &[f64], pw: f64, top: f64) -> Loop {
    let mut p = vec![(x0, y0), (x1, y0), (x1, y0 + t)];
    let mut ups: Vec<f64> = prongs.to_vec();
    ups.sort_by(|a, b| b.partial_cmp(a).unwrap());
    for &px in &ups {
        p.extend([(px + pw, y0 + t), (px + pw, top), (px, top), (px, y0 + t)]);
    }
    p.push((x0, y0 + t));
    poly(&p)
}

fn battery(label: &str, a: &Loop, b: &Loop) {
    let (va, vb) = (bulge_area(a), bulge_area(b));
    let (pa, pb) = (slab(a, 0.0), slab(b, 1.0));
    for (order, x, y, vx, vy) in [("ab", &pa, &pb, va, vb), ("ba", &pb, &pa, vb, va)] {
        let d = topo::test_support::flush_declarations(x, y, tol());
        let d2 = topo::test_support::flush_declarations(y, x, tol());
        for (op, r, want) in [
            ("U", topo::union_with(x, y, &d, tol()), vx + vy),
            ("S", topo::subtract_with(x, y, &d, tol()), vx),
            ("R", topo::subtract_with(y, x, &d2, tol()), vy),
            ("I", topo::intersect_with(x, y, &d, tol()), 0.0),
        ] {
            println!("DLT {label} {order} {op} {}", outcome(r, want, tol()));
        }
    }
}

/// Claim 1: several rings in one face, rings reached from two sides.
#[test]
#[ignore = "delta review battery; prints lines"]
fn join2_d_rings() {
    let (w, h) = (6.0, 4.0);
    for r in [0.5, 1.0] {
        let m = r * (1.0 - core::f64::consts::FRAC_1_SQRT_2);
        let lower = rrect(0.0, w, 0.0, h, [r; 4]);
        // Combs whose base lies on the south edge and runs onto the SW
        // fillet (tangent point, arc point, past it), prong tips rings.
        for x0 in [0.0, m, r, 1.5] {
            for (prongs, pw, top) in [
                (vec![2.0, 4.0], 0.5, 3.0),
                (vec![1.7, 2.5, 3.5, 4.4], 0.4, 2.0),
                (vec![2.0], 1.0, h),
                (vec![2.0, 4.0], 0.5, h),
            ] {
                let base_y = if x0 < r {
                    r - (r * r - (r - x0).powi(2)).sqrt()
                } else {
                    0.0
                };
                for x1 in [5.0, w - r, w] {
                    let up = comb(x0, x1, base_y.max(0.0), 0.6, &prongs, pw, top);
                    battery(
                        &format!("comb r={r} x0={x0:.4} x1={x1} n={} top={top}", prongs.len()),
                        &lower,
                        &up,
                    );
                }
            }
        }
        // A plus sign: four arms, two ends on the boundary, the hub's
        // corners rings reached from two sides.
        for (cx, cy) in [(3.0, 2.0), (r, 2.0), (3.0, r)] {
            let t = 0.4;
            let plus = poly(&[
                (cx - t, 0.0),
                (cx + t, 0.0),
                (cx + t, cy - t),
                (w - 0.5, cy - t),
                (w - 0.5, cy + t),
                (cx + t, cy + t),
                (cx + t, h - 0.5),
                (cx - t, h - 0.5),
                (cx - t, cy + t),
                (0.5f64.max(cx - 2.0), cy + t),
                (0.5f64.max(cx - 2.0), cy - t),
                (cx - t, cy - t),
            ]);
            battery(&format!("plus r={r} c=({cx},{cy})"), &lower, &plus);
        }
    }
}

/// Claim 2/3: two fillets tangent at one point (like far ends, concentric
/// and equal arcs), sharp and rounded partners, plus concave notches.
#[test]
#[ignore = "delta review battery; prints lines"]
fn join2_d_fillets() {
    let (w, h) = (6.0, 4.0);
    for ra in [0.5, 1.0, 2.0] {
        let lower = rrect(0.0, w, 0.0, h, [0.0, ra, 0.0, 0.0]);
        // Upper SE fillet sharing lower's tangent point (w - ra, 0),
        // radius rb: the arcs leave the site together and part.
        for rb in [0.25, 0.5, 1.0, 1.5, 2.0, 3.0] {
            for (y1, x0) in [(h, 0.0), (h - 1.0, 1.0), (2.5, w - ra - 0.5)] {
                let x1 = w - ra + rb;
                let up = rrect(x0, x1, 0.0, y1.max(rb + 0.1), [0.0, rb, 0.0, 0.0]);
                battery(&format!("tan ra={ra} rb={rb} x0={x0} y1={y1}"), &lower, &up);
            }
            // Sharing the other tangent point (w, ra).
            let y0 = ra - rb;
            let up = rrect(2.0, w, y0, h, [0.0, rb, 0.0, 0.0]);
            battery(&format!("tanN ra={ra} rb={rb}"), &lower, &up);
        }
        // Concentric: centre (w - ra, ra), inset d, radius ra - d.
        for d in [0.0, 0.1, 0.3] {
            let rb = ra - d;
            if rb <= 0.0 {
                continue;
            }
            for y0 in [d, 0.0] {
                let up = rrect(1.0, w - d, d, h - 1.0, [0.0, rb, 0.0, 0.0]);
                let _ = y0;
                battery(&format!("conc ra={ra} d={d}"), &lower, &up);
            }
        }
        // A concave notch in the upper at its SE corner, its arc tangent
        // to lower's straight south edge or east edge.
        for rb in [0.5, 1.0] {
            let up = rrect(1.0, w - ra, 0.0, h - 1.0, [0.0, -rb, 0.0, 0.0]);
            battery(&format!("notch ra={ra} rb={rb}"), &lower, &up);
        }
    }
    // Two rounded plates, all corners, offset along one axis.
    for (ra, rb) in [(1.0, 1.0), (1.0, 0.5), (0.5, 1.0)] {
        for dx in [0.0, 0.5, ra, 3.0] {
            let lower = rrect(0.0, 6.0, 0.0, 4.0, [ra; 4]);
            let up = rrect(dx, dx + 4.0, 0.0, 3.0, [rb; 4]);
            battery(&format!("rr ra={ra} rb={rb} dx={dx}"), &lower, &up);
        }
    }
}
