//! End-to-end probe (review of PR 3847, lane reach-dual3847-r2): the
//! near-tangent snowman widened — other radii, scales 1e-3..1e3, both
//! operand orders, every op, a result reused as an operand, and
//! point_in_solid sampled against the two balls' own distance test.
//! Prints one line per case; a wrong body or verdict prints WRONG.
#![allow(clippy::unwrap_used, clippy::panic)]
use core::f64::consts::PI;
use geom_core::{Band, Point2, Point3, Tol};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp};

fn ball(r: f64, y: f64) -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, y - r), 1.0), (Point2::new(0.0, y + r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    )
}
fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}
/// Lens volume from the radii and δ = r1 + r2 − d, cancellation-free:
/// cap heights h1 = δ(2r2 − δ)/2d, h2 = δ(2r1 − δ)/2d.
fn lens(r1: f64, r2: f64, d: f64, delta: f64) -> f64 {
    cap(r1, delta * (2.0 * r2 - delta) / (2.0 * d)) + cap(r2, delta * (2.0 * r1 - delta) / (2.0 * d))
}
fn op(o: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<Option<Body<f64>>, String> {
    let t = Tol::witness();
    let r = match o {
        BooleanOp::Union => topo::boolean::union(a, b, t),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, t),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, t),
    };
    match r {
        Ok(out) => Ok(out.body().map(|b| b.body.clone())),
        Err(e) => Err(format!("{e:?}").chars().take(60).collect()),
    }
}

#[test]
fn snowman_r2_probe() {
    let eps = Tol::witness().get().eps;
    let band = Band::linear(Tol::witness()).unwrap();
    let (mut built, mut refused, mut wrong) = (0, 0, 0);
    for (r1, r2) in [(1.0, 0.8), (1.0, 0.3), (0.5, 2.0)] {
        for sc in [1e-3, 1.0, 1e3] {
            for rel in [1e-4, 1e-5, 1e-6, 3e-7] {
                let (r1, r2) = (r1 * sc, r2 * sc);
                let d = r1 + r2 - rel * sc;
                let delta = (r1 - d) + r2;
                let (a, b) = (ball(r1, 0.0), ball(r2, d));
                let (va, vb) = (4.0 / 3.0 * PI * r1.powi(3), 4.0 / 3.0 * PI * r2.powi(3));
                let l = lens(r1, r2, d, delta);
                let h = delta * (2.0 * r2 - delta) / (2.0 * d);
                let (ys, rad) = (r1 - h, (h * (2.0 * r1 - h)).sqrt());
                for (name, o, x, y, want) in [
                    ("A∪B", BooleanOp::Union, &a, &b, va + vb - l),
                    ("B∪A", BooleanOp::Union, &b, &a, va + vb - l),
                    ("A∩B", BooleanOp::Intersect, &a, &b, l),
                    ("B∩A", BooleanOp::Intersect, &b, &a, l),
                    ("A∖B", BooleanOp::Subtract, &a, &b, va - l),
                    ("B∖A", BooleanOp::Subtract, &b, &a, vb - l),
                ] {
                    let tag = format!("eps {eps:e} r1 {r1} r2 {r2} δ {delta:e} {name}");
                    let body = match op(o, x, y) {
                        Ok(Some(b)) => b,
                        Ok(None) => { wrong += 1; println!("WRONG empty {tag}"); continue; }
                        Err(e) => { refused += 1; println!("refused {tag}: {e}"); continue; }
                    };
                    built += 1;
                    let mut notes = Vec::new();
                    if topo::validate(&body).is_err() || topo::validate_closed(&body).is_err()
                        || topo::validate_geometric(&body, Tol::witness()).is_err() {
                        notes.push("invalid".to_string());
                    }
                    match topo::mass_properties(&body, Tol::witness()) {
                        Ok(p) => if (p.volume - want).abs() > 1e-9 * want.max(sc.powi(3)) {
                            notes.push(format!("volume {} want {want}", p.volume));
                        },
                        Err(e) => notes.push(format!("mass {e:?}")),
                    }
                    let mut worst: f64 = 0.0;
                    for (key, _) in body.vertices() {
                        let p = topo::readback::vertex_point(&body, key).unwrap();
                        let off_axis = p.x.hypot(p.z);
                        if off_axis <= eps { continue; }
                        worst = worst.max((p.y - ys).hypot(off_axis - rad));
                    }
                    if worst > eps { notes.push(format!("pierce {worst:e} off the section")); }
                    // point_in_solid against the balls' own distance test,
                    // sampled on and around the section circle's rim.
                    let (mut pis_bad, mut pis_n) = (0, 0);
                    for k in 0..24 {
                        let ang = k as f64 * PI / 12.0;
                        for (dr, dy) in [(-0.5, 0.0), (0.5, 0.0), (0.0, -0.5), (0.0, 0.5), (-0.2, 0.2), (0.3, -0.3)] {
                            let (rr, yy) = (rad * (1.0 + dr), ys + dy * h.max(rad * rel));
                            let q = Point3::new(rr * ang.cos(), yy, rr * ang.sin());
                            let ia = (q.x * q.x + q.y * q.y + q.z * q.z).sqrt() - r1;
                            let ib = (q.x * q.x + (q.y - d).powi(2) + q.z * q.z).sqrt() - r2;
                            if ia.abs() < 1e3 * eps || ib.abs() < 1e3 * eps { continue; }
                            let (ina, inb) = (ia < 0.0, ib < 0.0);
                            let inside = match (name, x as *const _ == &a as *const _) {
                                ("A∪B" | "B∪A", _) => ina || inb,
                                ("A∩B" | "B∩A", _) => ina && inb,
                                (_, true) => ina && !inb,
                                (_, false) => inb && !ina,
                            };
                            pis_n += 1;
                            match topo::point_in_solid(&body, q, band, Tol::witness()) {
                                Ok(topo::SolidContainment::In) if inside => {}
                                Ok(topo::SolidContainment::Out) if !inside => {}
                                Err(_) => {}
                                other => { pis_bad += 1; if pis_bad < 3 { println!("  pis {q:?} inside {inside} got {other:?}"); } }
                            }
                        }
                    }
                    if pis_bad > 0 { notes.push(format!("point_in_solid {pis_bad}/{pis_n} wrong")); }
                    notes.push(format!("pis-n {pis_n}"));
                    // A result reused: (A∖B) ∪ (A∩B) must rebuild A.
                    if name == "A∖B" {
                        if let (Ok(Some(i)),) = (op(BooleanOp::Intersect, &a, &b),) {
                            match op(BooleanOp::Union, &body, &i) {
                                Ok(Some(u)) => match topo::mass_properties(&u, Tol::witness()) {
                                    Ok(p) if (p.volume - va).abs() > 1e-9 * va.max(sc.powi(3)) => notes.push(format!("reuse volume {} want {va}", p.volume)),
                                    Ok(_) => notes.push("reuse ok".into()),
                                    Err(e) => notes.push(format!("reuse mass {e:?}")),
                                },
                                Ok(None) => notes.push("reuse EMPTY".into()),
                                Err(e) => notes.push(format!("reuse refused {e}")),
                            }
                        }
                    }
                    let bad = notes.iter().any(|n| !n.starts_with("reuse ok") && !n.starts_with("reuse refused") && !n.starts_with("pis-n"));
                    if bad { wrong += 1; }
                    println!("{} {tag} pierce-worst {worst:.2e} {}", if bad { "WRONG" } else { "built" }, notes.join("; "));
                }
            }
        }
    }
    println!("TOTAL built {built} refused {refused} wrong {wrong}");
}
