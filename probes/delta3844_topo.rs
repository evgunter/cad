//! Delta-review probes for PR #3844 at ae4b1dbe (lane reach-delta/3844).
//! Wire: `#[path = "delta3844_topo.rs"] mod delta3844_topo;` in
//! `crates/topo/tests/all.rs`, file copied beside it. Compiles unchanged
//! against `origin/main` (the backstop's signature there is the same), so
//! every line it prints is a head/main differential. Oracle: box
//! arithmetic; never the kernel.
#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout, clippy::unwrap_used)]

use geom_core::{Band, Point3, Tol, Vec3};
use topo::test_support::{brick, cube_into, mapped_cube, prism};
use topo::{
    Body, BooleanCoincidence, BooleanDeclarations, BooleanError, BooleanOp, BooleanResult,
    CarrierDesc, FacePairDeclaration, face_carrier,
};

fn tol() -> Tol {
    Tol::witness()
}

fn tag(r: &Result<topo::AtRestOutcome, BooleanError>) -> String {
    match r {
        Ok(_) => "PASS".into(),
        Err(BooleanError::ResultVolumeImplausible { which, .. }) => format!("refuse[{which}]"),
        Err(e) => format!("other[{}]", format!("{e:?}").split([' ', '{', '(']).next().unwrap()),
    }
}

fn plate(s: f64, h: f64) -> Body<f64> {
    let pr = [(0.0, 0.0), (2.0 * s, 0.0), (2.0 * s, 2.0 * s), (0.0, 2.0 * s)];
    prism::<f64>(&pr, h, tol()).body
}

/// P1 — MAJ-1's wrong body (a plate keeping a cube of edge `c·s` as
/// extra height) at scales 1e-3, 1, 1e3, and the old allowance's edge
/// (`k × escalate × 4 s²`). Nothing can be declared any more: the
/// backstop's door takes no declarations (structural); this records the
/// verdicts the head gives, which must equal main's.
#[test]
fn d1_maj1_wrong_cube_every_scale() {
    let band = Band::linear(tol()).unwrap();
    let esc = band.escalate();
    println!("D1 eps={:e} zero={:e} escalate={:e}", tol().eps(), band.zero(), esc);
    for s in [1e-3, 1.0, 1e3] {
        let a = plate(s, 0.1 * s);
        let mut kepts: Vec<(String, f64)> = vec![("3mm·s cube".into(), (0.003 * s).powi(3))];
        for k in [0.5, 0.99, 1.01, 2.0, 100.0, 1e6] {
            kepts.push((format!("{k}×old-allowance"), k * esc * 4.0 * s * s));
        }
        for (label, kept) in kepts {
            if kept / (4.0 * s * s) > 0.05 * s {
                continue;
            }
            let thick = plate(s, 0.1 * s + kept / (4.0 * s * s));
            let thin = plate(s, 0.1 * s - kept / (4.0 * s * s));
            let i = topo::test_support::volume_backstop(BooleanOp::Intersect, &a, &a, &thick, tol());
            let d = topo::test_support::volume_backstop(BooleanOp::Subtract, &a, &a, &thick, tol());
            let u = topo::test_support::volume_backstop(BooleanOp::Union, &a, &a, &thin, tol());
            println!(
                "D1 s={s:e} {label} ΔV={kept:e}: ∩ {} | ∖ {} | ∪(thin) {}",
                tag(&i),
                tag(&d),
                tag(&u)
            );
        }
    }
}

/// P1b — duplicates through the REAL door: the plates' tops declared
/// 1, 100 and 10⁴ times; the result (a correct plate) must be the same
/// body each time. The planted path no longer exists.
#[test]
fn d2_duplicate_declarations_through_the_real_door() {
    let a = plate(1.0, 0.1);
    let pr = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
    // B: the plate's upper half, flush at the top and on all four sides.
    let b = topo::test_support::prism_z::<f64>(&pr, 0.05, 0.1, tol()).body;
    let found = topo::test_support::flush_declarations(&a, &b, tol());
    println!("D2 flush pairs found: {}", found.coincident_faces.len());
    for n in [1usize, 100, 10_000] {
        let d = BooleanDeclarations {
            coincident_faces: (0..n).flat_map(|_| found.coincident_faces.iter().cloned()).collect(),
            ..found.clone()
        };
        let out = topo::intersect_with(&a, &b, &d, tol());
        let cell = match out {
            Ok(BooleanResult::Body(bb)) => {
                format!("{:.17e}", topo::mass_properties(&bb.body, tol()).unwrap().volume)
            }
            Ok(BooleanResult::Empty) => "empty".into(),
            Err(e) => format!("{e:?}").chars().take(80).collect(),
        };
        println!("D2 copies={n}: ∩ {cell}");
    }
}

/// P3 — the +V arm, both sides of the band: a two-shell result (a
/// reverted unit cube and a cube of side `s`), `V/A` at multiples of
/// `zero` and `escalate`, under ∩ and ∖.
#[test]
fn d3_plus_v_against_the_band() {
    let band = Band::linear(tol()).unwrap();
    let unit = |x0: f64, side: f64| {
        move |u: f64, v: f64, w: f64| Point3::new(x0 + side * u, side * v, side * w)
    };
    let cube = mapped_cube::<f64>(unit(0.0, 1.0), tol());
    // A big operand so ∩ ≤ A is slack; ∖ is cube ∖ big, so ∖ ≤ A and
    // ∖ ≥ A − B are slack.
    let big = brick::<f64>((-1.0, 10.0), (-1.0, 3.0), (-1.0, 3.0), tol());
    for (label, m) in [
        ("-0.5 zero", -0.5 * band.zero()),
        ("-0.99 zero", -0.99 * band.zero()),
        ("-1.01 zero", -1.01 * band.zero()),
        ("-0.99 esc", -0.99 * band.escalate()),
        ("-1.01 esc", -1.01 * band.escalate()),
        ("-1.1 esc", -1.1 * band.escalate()),
        ("-2 esc", -2.0 * band.escalate()),
        ("-1e3 esc", -1e3 * band.escalate()),
    ] {
        let s = (1.0 + 12.0 * m).cbrt();
        let mut result = mapped_cube::<f64>(unit(0.0, 1.0), tol()).revert().unwrap();
        cube_into(&mut result, unit(5.0, s), tol());
        let v = s.powi(3) - 1.0;
        let a = 6.0 + 6.0 * s * s;
        let i = topo::test_support::volume_backstop(BooleanOp::Intersect, &big, &cube, &result, tol());
        let d = topo::test_support::volume_backstop(BooleanOp::Subtract, &cube, &big, &result, tol());
        println!(
            "D3 V/A {label} (target {m:e}, stored {:e}): ∩ {} | ∖ {}",
            v / a,
            tag(&i),
            tag(&d)
        );
    }
}

/// P4 — the settled residue at its rows' poses: the actual verdict per
/// op, and the volume's offset from box arithmetic against the gap.
#[test]
fn d4_residue_verdicts() {
    let band = Band::linear(tol()).unwrap();
    let phi = 5.0_f64.to_radians();
    let p = Point3::new(0.5, 0.2, 1.0);
    let block = brick::<f64>((0.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol());
    let face_facing = |body: &Body<f64>, facing: f64| {
        body.faces()
            .map(|(k, _)| k)
            .find(|&k| {
                matches!(face_carrier(body, k),
                    Some(CarrierDesc::Plane { normal, .. }) if normal.z * facing > 0.99)
            })
            .unwrap()
    };
    for (pose, (height, depth, facing), class) in [
        ("standing", (1.0, 0.0, -1.0), BooleanCoincidence::REST),
        ("sunk", (0.5, 0.5, 1.0), BooleanCoincidence::Continuation),
    ] {
        for over in [-2.0, -1.5, -1.2, -0.5, 0.5, 1.2, 1.5, 2.0] {
            let theta = over * band.zero();
            let gap = 0.5 * theta.abs() * phi.sin().powi(2);
            let (ea, eb) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(phi.cos(), phi.sin(), theta * phi.sin()));
            let tool = mapped_cube::<f64>(
                move |u, v, w| p + ea * u + eb * v + Vec3::new(0.0, 0.0, height * w - depth),
                tol(),
            );
            let (top, face) = (face_facing(&block, 1.0), face_facing(&tool, facing));
            let dd = |x, y| BooleanDeclarations {
                coincident_faces: vec![FacePairDeclaration::new(x, y, class)],
                ..BooleanDeclarations::none()
            };
            let wv = phi.sin() * height;
            let want = if pose == "standing" { [13.5 + wv, 0.0, 13.5, wv] } else { [13.5, wv, 13.5 - wv, 0.0] };
            let outs = [
                topo::union_with(&block, &tool, &dd(top, face), tol()),
                topo::intersect_with(&block, &tool, &dd(top, face), tol()),
                topo::subtract_with(&block, &tool, &dd(top, face), tol()),
                topo::subtract_with(&tool, &block, &dd(face, top), tol()),
            ];
            let mut line = format!("D4 {pose} θ={over:+}ε gap={gap:.2e}:");
            for (out, w) in outs.into_iter().zip(want) {
                line.push_str(&match out {
                    Ok(BooleanResult::Empty) => " empty".into(),
                    Ok(BooleanResult::Body(bb)) => {
                        let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
                        format!(" {:+.2e}", v - w)
                    }
                    Err(BooleanError::ResultVolumeImplausible { which, .. }) => format!(" REFUSE[{which}]"),
                    Err(e) => format!(" {}", format!("{e:?}").split([' ', '{', '(']).next().unwrap()),
                });
            }
            println!("{line}");
        }
    }
}
