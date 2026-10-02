//! Reviewer probe (reach-dual3844-r1): what the volume backstop lets out.
//! Wrong bodies planted through the public gate, with and without a
//! declaration; oracle = box arithmetic. Prints a table; asserts nothing
//! except where marked, so every row reports.


use geom_core::{Band, Point3, Tol, Vec3};
use topo::test_support::{brick, mapped_cube};
use topo::{
    AtRestPolicy, Body, BooleanCoincidence, BooleanDeclarations, BooleanOp, CarrierDesc,
    FacePairDeclaration, face_carrier,
};

fn face_facing(body: &Body<f64>, facing: f64) -> topo::FaceKey {
    body.faces()
        .map(|(k, _)| k)
        .find(|&k| {
            matches!(face_carrier(body, k),
                Some(CarrierDesc::Plane { normal, .. }) if normal.z * facing > 0.99)
        })
        .expect("face")
}

fn decl(a: topo::FaceKey, b: topo::FaceKey, n: usize) -> BooleanDeclarations {
    BooleanDeclarations {
        coincident_faces: (0..n)
            .map(|_| FacePairDeclaration::new(a, b, BooleanCoincidence::Continuation))
            .collect(),
        ..BooleanDeclarations::none()
    }
}

fn verdict(
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
    r: &Body<f64>,
    d: &BooleanDeclarations,
) -> String {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    match <f64 as AtRestPolicy>::gate_volume_backstop(op, a, b, r, d, band, tol) {
        Ok(_) => "PASSES".into(),
        Err(e) => format!("refuses {e:?}").chars().take(90).collect(),
    }
}

/// MAJ-1's wrong component (a 3 mm cube carried as extra height on a
/// 2 m plate), at scale `s`, with the plates' tops declared `n` times.
#[test]
fn probe_declared_large_face_lets_a_wrong_component_out() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    println!("eps={:e} escalate={:e}", tol.eps(), band.escalate());
    for s in [1e-3, 1.0, 1e3] {
        let plate = brick::<f64>((0.0, 2.0 * s), (0.0, 2.0 * s), (0.0, 0.1 * s), tol);
        let top = face_facing(&plate, 1.0);
        let bot = face_facing(&plate, -1.0);
        for cube in [0.003 * s, 0.01 * s, 0.03 * s] {
            let kept = cube.powi(3);
            let wrong = brick::<f64>(
                (0.0, 2.0 * s),
                (0.0, 2.0 * s),
                (0.0, 0.1 * s + kept / (4.0 * s * s)),
                tol,
            );
            // ∪ that LOST the component: shorter than A.
            let short = brick::<f64>(
                (0.0, 2.0 * s),
                (0.0, 2.0 * s),
                (0.0, 0.1 * s - kept / (4.0 * s * s)),
                tol,
            );
            for n in [0usize, 1, 100] {
                let d = decl(top, top, n);
                println!(
                    "s={s:e} cube={cube:e} dV={kept:e} allowance={:e} n={n}: ∩ {} | ∪ {} | ∪(bot-top) {}",
                    band.escalate() * 4.0 * s * s * n as f64,
                    verdict(BooleanOp::Intersect, &plate, &plate, &wrong, &d),
                    verdict(BooleanOp::Union, &plate, &plate, &short, &d),
                    verdict(BooleanOp::Union, &plate, &plate, &short, &decl(bot, top, n)),
                );
            }
        }
    }
}

/// A short ∩: a declaration on a big face lets a ∩ that dropped a small
/// separate piece through? (∩ has no lower bound, so this is the ≤ arms
/// only; recorded for completeness.) And ∖ ≥ A − B, ∪ ≤ A + B planted
/// over a declared large face.
#[test]
fn probe_joint_arms_under_declarations() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol);
    let b = brick::<f64>((0.0, 2.0), (0.0, 2.0), (1.0, 2.0), tol); // stacked: ∪ tight
    let (atop, bbot) = (face_facing(&a, 1.0), face_facing(&b, -1.0));
    for dv in [1e-9, 1e-8, 3e-8, 1e-7] {
        let over = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0 + dv / 4.0), tol);
        for n in [0, 1] {
            println!(
                "∪≤A+B dV={dv:e} n={n}: {}",
                verdict(BooleanOp::Union, &a, &b, &over, &decl(atop, bbot, n))
            );
        }
    }
    // ∖ ≥ A − B: B ⊂ A sharing A's top (declared). Correct A∖B = 2×2×1.
    let big = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), tol);
    let (btop, ttop) = (face_facing(&big, 1.0), face_facing(&b, 1.0));
    for dv in [1e-9, 1e-8, 3e-8, 1e-7] {
        let under = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0 - dv / 4.0), tol);
        for n in [0, 1] {
            println!(
                "∖≥A−B dV={dv:e} n={n}: {}",
                verdict(BooleanOp::Subtract, &big, &b, &under, &decl(btop, ttop, n))
            );
        }
    }
    // Correct bodies at the tight bounds must pass.
    let exact_u = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), tol);
    let exact_s = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol);
    println!("correct ∪ {}", verdict(BooleanOp::Union, &a, &b, &exact_u, &BooleanDeclarations::none()));
    println!("correct ∖ {}", verdict(BooleanOp::Subtract, &big, &b, &exact_s, &BooleanDeclarations::none()));
    // Flipped result: reverted body for ∩/∖ and ∪.
    let rev = exact_s.revert().expect("reverts");
    println!("flipped ∩ {}", verdict(BooleanOp::Intersect, &big, &exact_s, &rev, &BooleanDeclarations::none()));
    println!("flipped ∖ {}", verdict(BooleanOp::Subtract, &big, &b, &rev, &BooleanDeclarations::none()));
    println!("flipped ∪ {}", verdict(BooleanOp::Union, &a, &b, &rev, &BooleanDeclarations::none()));
}

/// End to end: does the door accept a pair declared 100 times (the
/// allowance sums the raw vector)? Wedge standing on a block, PR pose.
#[test]
fn probe_door_accepts_duplicate_declarations() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let phi = 5.0_f64.to_radians();
    let p = Point3::new(0.5, 0.2, 1.0);
    let block = brick::<f64>((0.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol);
    let theta = 1.2 * band.zero();
    let (ea, eb) = (
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(phi.cos(), phi.sin(), theta * phi.sin()),
    );
    let wedge = mapped_cube::<f64>(move |u, v, w| p + ea * u + eb * v + Vec3::new(0.0, 0.0, w), tol);
    let (top, face) = (face_facing(&block, 1.0), face_facing(&wedge, -1.0));
    for n in [1usize, 2, 100] {
        let d = BooleanDeclarations {
            coincident_faces: (0..n)
                .map(|_| FacePairDeclaration::new(top, face, BooleanCoincidence::REST))
                .collect(),
            ..BooleanDeclarations::none()
        };
        let out = topo::union_with(&block, &wedge, &d, tol);
        let v = match &out {
            Ok(topo::BooleanResult::Body(bb)) => format!("{:?}", topo::mass_properties(&bb.body, tol).map(|m| m.volume)),
            other => format!("{other:?}").chars().take(120).collect(),
        };
        println!("dup n={n}: union_with -> {v}");
    }
}

/// DESIGN tier 3's +V invariant exempts an in-band (escalated) V/A; the
/// door's `encloses_material` decides at the exact band. A body whose
/// stored geometry is inverted by less than ε: what does each say?
#[test]
fn probe_in_band_negative_sliver_tier3_vs_door() {
    let tol = Tol::witness();
    let cube = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    for t in [1e-15, 1e-13, 0.1 * tol.eps()] {
        let Ok(thin) = std::panic::catch_unwind(|| brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, t), tol)) else { println!("t={t:e}: brick refuses"); continue; };
        let sliver = thin.revert().expect("reverts");
        let v = topo::mass_properties(&sliver, tol).map(|m| m.volume);
        println!(
            "t={t:e} eps={:e}: V={v:?} tier3={:?} | door ∩ {} | door ∖ {}",
            tol.eps(),
            topo::validate_geometric(&sliver, tol).map_err(|e| format!("{e:?}").chars().take(120).collect::<String>()),
            verdict(BooleanOp::Intersect, &cube, &cube, &sliver, &BooleanDeclarations::none()),
            verdict(BooleanOp::Subtract, &cube, &cube, &sliver, &BooleanDeclarations::none()),
        );
    }
}

/// End to end beyond the PR's poses: the wedge standing on / sunk in the
/// block at tilts within ±3ε, scaled ×1e-3/×1/×1e3, all four ops, the
/// union reused as an operand, and point_in_solid sampled against the
/// box/parallelepiped oracle.
#[test]
fn probe_wedge_door_widened() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let phi = 5.0_f64.to_radians();
    let mut seed = 0x9e3779b97f4a7c15_u64;
    let mut rnd = move || {
        seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    for s in [1e-3, 1.0, 1e3] {
        let block = brick::<f64>((0.0, 3.0 * s), (-2.0 * s, 2.5 * s), (0.0, s), tol);
        let p = Point3::new(0.5 * s, 0.2 * s, s);
        for (pose, facing, class, height, depth) in [
            ("standing", -1.0, BooleanCoincidence::REST, 1.0, 0.0),
            ("sunk", 1.0, BooleanCoincidence::Continuation, 0.5, 0.5),
        ] {
            for over in [-2.9, -2.0, -1.2, -0.5, 0.0, 0.5, 1.2, 2.0, 2.9] {
                // tilt as a displacement over the wedge face's extent (~s)
                let theta = over * band.zero() / s;
                let (ea, eb) = (
                    Vec3::new(s, 0.0, 0.0),
                    Vec3::new(s * phi.cos(), s * phi.sin(), s * theta * phi.sin()),
                );
                let wedge = mapped_cube::<f64>(
                    move |u, v, w| p + ea * u + eb * v + Vec3::new(0.0, 0.0, s * (height * w - depth)),
                    tol,
                );
                let (top, face) = (face_facing(&block, 1.0), face_facing(&wedge, facing));
                let ab = decl1(top, face, class);
                let ba = decl1(face, top, class);
                let wv = phi.sin() * height * s.powi(3);
                let bv = 13.5 * s.powi(3);
                let (u, i, d, e) = if pose == "standing" { (bv + wv, 0.0, bv, wv) } else { (bv, wv, bv - wv, 0.0) };
                let mut line = format!("s={s:e} {pose} {over:+}ε:");
                let tolv = band.escalate() * phi.sin() * s * s;
                for (name, out, want) in [
                    ("∪", topo::union_with(&block, &wedge, &ab, tol), u),
                    ("∩", topo::intersect_with(&block, &wedge, &ab, tol), i),
                    ("A∖B", topo::subtract_with(&block, &wedge, &ab, tol), d),
                    ("B∖A", topo::subtract_with(&wedge, &block, &ba, tol), e),
                    ("B∪A", topo::union_with(&wedge, &block, &ba, tol), u),
                ] {
                    let r = match out {
                        Ok(topo::BooleanResult::Empty) => if want == 0.0 { "ok(empty)".to_string() } else { format!("EMPTY≠{want:e}") },
                        Ok(topo::BooleanResult::Body(bb)) => {
                            let v = topo::mass_properties(&bb.body, tol).map(|m| m.volume).unwrap_or(f64::NAN);
                            let mut bad = 0;
                            if name == "∪" {
                                for _ in 0..200 {
                                    let q = Point3::new(rnd() * 3.0 * s, (rnd() * 4.5 - 2.0) * s, rnd() * 2.2 * s);
                                    // oracle: in block, or in wedge (param solve)
                                    let in_block = q.z < s && q.z > 0.0;
                                    let rel = q - Point3::new(p.x, p.y, p.z - depth * s);
                                    let v_ = rel.y / (s * phi.sin());
                                    let u_ = (rel.x - v_ * s * phi.cos()) / s;
                                    let w_ = (rel.z - v_ * s * theta * phi.sin()) / (s * height);
                                    let in_w = (0.0..1.0).contains(&u_) && (0.0..1.0).contains(&v_) && (0.0..1.0).contains(&w_);
                                    let want_in = in_block || in_w;
                                    if let Ok(c) = topo::point_in_solid(&bb.body, q, band, tol) {
                                        let got_in = matches!(c, topo::SolidContainment::In);
                                        let on = matches!(c, topo::SolidContainment::OnBoundary);
                                        if !on && got_in != want_in { bad += 1; }
                                    }
                                }
                                // reuse: (A ∪ B) ∩ A must be A
                                if let Ok(topo::BooleanResult::Body(rb)) = topo::intersect(&bb.body, &block, tol) {
                                    let rv = topo::mass_properties(&rb.body, tol).map(|m| m.volume).unwrap_or(f64::NAN);
                                    line += &format!(" [(∪)∩A {:.1e}]", rv - bv);
                                } else { let e = topo::intersect(&bb.body, &block, tol); line += &format!(" [(∪)∩A {}]", format!("{e:?}").chars().take(60).collect::<String>()); }
                            }
                            let flag = if (v - want).abs() <= tolv { "" } else { "!!" };
                            format!("{flag}{:+.1e}{}", v - want, if bad > 0 { format!(" PIS-mismatch {bad}") } else { String::new() })
                        }
                        Err(err) => format!("{err:?}").split([' ', '{']).next().unwrap_or("").to_string(),
                    };
                    line += &format!(" {name} {r} |");
                }
                println!("{line}");
            }
        }
    }
}

fn decl1(a: topo::FaceKey, b: topo::FaceKey, c: BooleanCoincidence) -> BooleanDeclarations {
    BooleanDeclarations {
        coincident_faces: vec![FacePairDeclaration::new(a, b, c)],
        ..BooleanDeclarations::none()
    }
}
