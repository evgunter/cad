//! R2 probes for PR #3844 (reach-dual3844-r2): the rounded-plate stack
//! end to end, at three scales and rotated, against the closed form.
#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout, clippy::unwrap_used)]

use geom_core::{Band, Point2, Point3, Tol};
use profile::{Open, ProfileLoop, Start};
use sweep::test_support::{extruded, sketch_at};
use topo::{AtRestPolicy, Body, BooleanCoincidence, BooleanDeclarations, BooleanError, BooleanOp, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

/// The W×H outline with corners rounded r, scaled by s, rotated th about z.
fn rounded(s: f64, th: f64) -> ProfileLoop<f64> {
    let t = tol();
    let (w, h, r) = (6.0 * s, 4.0 * s, 0.5 * s);
    let (c, sn) = (th.cos(), th.sin());
    let rot = |x: f64, y: f64| Point2::new(c * x - sn * y, sn * x + c * y);
    let dir = |x: f64, y: f64| (c * x - sn * y, sn * x + c * y);
    let (d0, d1, d2, d3) = (dir(1.0, 0.0), dir(0.0, 1.0), dir(-1.0, 0.0), dir(0.0, -1.0));
    Open.at(rot(w / 2.0, 0.0))
        .toward(d0.0, d0.1, t).unwrap()
        .fillet(r, t).unwrap()
        .at(rot(w, h / 2.0), t).unwrap()
        .toward(d1.0, d1.1, t).unwrap()
        .fillet(r, t).unwrap()
        .at(rot(w / 2.0, h), t).unwrap()
        .toward(d2.0, d2.1, t).unwrap()
        .fillet(r, t).unwrap()
        .at(rot(0.0, h / 2.0), t).unwrap()
        .toward(d3.0, d3.1, t).unwrap()
        .fillet(r, t).unwrap()
        .to(Start, t).unwrap()
        .into()
}

fn area(s: f64) -> f64 {
    (24.0 - (4.0 - core::f64::consts::PI) / 4.0) * s * s
}

/// Oracle membership in the rounded outline (unrotated frame).
fn in_outline(s: f64, th: f64, x: f64, y: f64) -> Option<bool> {
    let (c, sn) = (th.cos(), th.sin());
    let (u, v) = (c * x + sn * y, -sn * x + c * y);
    let (w, h, r) = (6.0 * s, 4.0 * s, 0.5 * s);
    let cu = u.clamp(r, w - r);
    let cv = v.clamp(r, h - r);
    let d = ((u - cu).powi(2) + (v - cv).powi(2)).sqrt();
    let inside = u > 0.0 && u < w && v > 0.0 && v < h && d < r;
    let margin = (d - r).abs().min(u.abs()).min((w - u).abs()).min(v.abs()).min((h - v).abs());
    if margin < 1e-6 * s { None } else { Some(inside) }
}

fn findings(a: &Body<f64>, b: &Body<f64>) -> BooleanDeclarations {
    let found = topo::flush::find_flush_candidates(a, b, tol()).expect("the plates decide");
    topo::flush::declare_all(&found)
}

fn vol(b: &Body<f64>) -> f64 {
    topo::mass_properties(b, tol()).unwrap().volume
}

fn gate(op: BooleanOp, a: &Body<f64>, b: &Body<f64>, r: &Body<f64>, d: &BooleanDeclarations) -> Result<(), BooleanError> {
    let band = Band::linear(tol()).unwrap();
    <f64 as AtRestPolicy>::gate_volume_backstop(op, a, b, r, d, band, tol()).map(|_| ())
}

fn tag(r: &Result<(), BooleanError>) -> &'static str {
    match r {
        Ok(()) => "PASS",
        Err(BooleanError::ResultVolumeImplausible { .. }) => "refuse",
        Err(_) => "other",
    }
}

#[test]
fn e1_rounded_stack_scales_rotations() {
    let eps = tol().eps();
    let esc = Band::linear(tol()).unwrap().escalate();
    println!("eps={eps:e}");
    for s in [1e-3, 1.0, 1e3] {
        for th in [0.0, 0.3] {
            let a = extruded(sketch_at(0.0), vec![rounded(s, th)], s, tol());
            for (pose, z0) in [("sunk", 0.25 * s), ("top", 0.5 * s), ("bottom", 0.0)] {
                let b = extruded(sketch_at(z0), vec![rounded(s, th)], 0.5 * s, tol());
                let d = findings(&a, &b);
                let half = area(s) * 0.5 * s;
                let full = area(s) * s;
                let label = format!("s={s:e} th={th} {pose}");
                // the real ops
                let ops: [(&str, BooleanOp, &Body<f64>, &Body<f64>, f64); 4] = [
                    ("A∩B", BooleanOp::Intersect, &a, &b, half),
                    ("A∖B", BooleanOp::Subtract, &a, &b, full - half),
                    ("B∖A", BooleanOp::Subtract, &b, &a, 0.0),
                    ("A∪B", BooleanOp::Union, &a, &b, full),
                ];
                for (name, op, x, y, want) in ops {
                    let dd = if std::ptr::eq(x, &b) { findings(&b, &a) } else { d.clone() };
                    let out = match op {
                        BooleanOp::Intersect => topo::intersect_with(x, y, &dd, tol()),
                        BooleanOp::Subtract => topo::subtract_with(x, y, &dd, tol()),
                        BooleanOp::Union => topo::union_with(x, y, &dd, tol()),
                    };
                    match out {
                        Ok(BooleanResult::Body(bb)) => {
                            let v = vol(&bb.body);
                            let rel = (v - want).abs() / full;
                            // point_in_solid against the oracle
                            let mut bad = 0;
                            let mut n = 0;
                            let band = Band::linear(tol()).unwrap();
                            for i in 0..9 {
                                for j in 0..7 {
                                    for k in 0..5 {
                                        let (x0, y0) = (-1.0 * s + i as f64 * 0.97 * s, -1.0 * s + j as f64 * 1.03 * s);
                                        let z = -0.1 * s + k as f64 * 0.27 * s;
                                        let Some(ino) = in_outline(s, th, x0, y0) else { continue };
                                        let in_a = ino && z > 0.0 && z < s;
                                        let in_b = ino && z > z0 && z < z0 + 0.5 * s;
                                        let zc = [0.0, s, z0, z0 + 0.5 * s].iter().any(|q| (z - q).abs() < 1e-6 * s);
                                        if zc { continue; }
                                        let want_in = match name {
                                            "A∩B" => in_a && in_b,
                                            "A∖B" => in_a && !in_b,
                                            "B∖A" => in_b && !in_a,
                                            _ => in_a || in_b,
                                        };
                                        n += 1;
                                        let got = topo::point_in_solid(&bb.body, Point3::new(x0, y0, z), band, tol());
                                        let got_in = match got {
                                            Ok(topo::SolidContainment::In) => true,
                                            Ok(topo::SolidContainment::Out) => false,
                                            Ok(_) => { bad += 1; continue; }
                                            Err(_) => { bad += 1; continue; }
                                        };
                                        if got_in != want_in { bad += 1; }
                                    }
                                }
                            }
                            println!("E1 {label} {name}: builds vol rel err {rel:.2e}; pis mismatches {bad}/{n}");
                        }
                        Ok(BooleanResult::Empty) => println!("E1 {label} {name}: empty (want {want:e})"),
                        Err(e) => println!("E1 {label} {name}: refuses {}", format!("{e:?}").split(['{', '(']).next().unwrap().trim()),
                    }
                }
                // planted: the A∩B result (B itself) thickened by δ; bisect the largest δ the gate passes with the declarations
                let probe = |delta: f64, decl: &BooleanDeclarations| {
                    let wrong = extruded(sketch_at(z0), vec![rounded(s, th)], 0.5 * s + delta, tol());
                    gate(BooleanOp::Intersect, &a, &b, &wrong, decl)
                };
                let none = BooleanDeclarations::none();
                let (mut lo, mut hi) = (0.0f64, s);
                for _ in 0..40 {
                    let mid = (lo + hi) / 2.0;
                    if probe(mid, &d).is_ok() { lo = mid } else { hi = mid }
                }
                let dv = lo * area(s);
                let sum_min: f64 = d.coincident_faces.len() as f64;
                println!(
                    "E1 {label} planted ∩ thickened: largest δ passing declared = {lo:.3e} (ΔV {dv:.3e} m³, cube edge {:.3e} m = {:.0}ε; esc·A_top={:.3e}); decl pairs={sum_min}; δ=lo/2 with none: {}",
                    dv.cbrt(), dv.cbrt() / eps, esc * area(s), tag(&probe(lo / 2.0, &none))
                );
            }
        }
    }
}

/// Unions of touching rounded plates (tight at A+B) and differences of
/// a contained plate (tight at A−B): correct bodies must build.
#[test]
fn e2_tight_unions_and_differences_build() {
    for s in [1e-3, 1.0, 1e3] {
        for th in [0.0, 0.3, 1.1] {
            let p = extruded(sketch_at(0.0), vec![rounded(s, th)], s, tol());
            let q = extruded(sketch_at(s), vec![rounded(s, th)], s, tol());
            let d = findings(&p, &q);
            let r = topo::union_with(&p, &q, &d, tol());
            let want = 2.0 * area(s) * s;
            match r {
                Ok(BooleanResult::Body(bb)) => println!("E2 s={s:e} th={th} stack ∪: rel {:.2e}", (vol(&bb.body) - want).abs() / want),
                other => println!("E2 s={s:e} th={th} stack ∪: {other:?}"),
            }
            // a plate of the same outline poking out nowhere: B ⊂ A strictly inside (no contact)
            let thick = extruded(sketch_at(-s), vec![rounded(s, th)], 3.0 * s, tol());
            let r2 = topo::subtract(&thick, &p, tol());
            let w2 = 2.0 * area(s) * s;
            match r2 {
                Ok(BooleanResult::Body(bb)) => println!("E2 s={s:e} th={th} thick∖p: rel {:.2e}", (vol(&bb.body) - w2).abs() / w2),
                Err(e) => println!("E2 s={s:e} th={th} thick∖p: refuses {}", format!("{e:?}").split(['{', '(']).next().unwrap().trim()),
                other => println!("E2 s={s:e} th={th} thick∖p: {other:?}"),
            }
        }
    }
    let _ = BooleanCoincidence::REST;
}
