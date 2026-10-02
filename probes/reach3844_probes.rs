//! R2 probes for PR #3844 (reach-dual3844-r2). Oracle: box arithmetic.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Band, Bounds, Point3, Tol};

use super::ops::volume_backstop;
use crate::body::Body;
use crate::boolean::{BooleanError, BooleanOp, FacePairDeclaration};
use crate::contact::BooleanCoincidence;
use crate::props::QuadLane;
use crate::test_support_fixtures::{FaceGeometry, prism_ops};
use crate::BooleanDeclarations;

fn prism(profile: &[(f64, f64); 4], z: (f64, f64), off: (f64, f64)) -> Body<f64> {
    let pr = profile.map(|(x, y)| (x + off.0, y + off.1));
    let mut body = Body::<f64>::new();
    prism_ops(&mut body, &pr, z, Point3::new, FaceGeometry::Certified, Tol::witness());
    body
}

fn top(b: &Body<f64>) -> crate::FaceKey {
    b.faces()
        .map(|(k, _)| k)
        .find(|&k| matches!(crate::boolean::face_carrier(b, k),
            Some(crate::boolean::CarrierDesc::Plane { normal, .. }) if normal.z > 0.99))
        .unwrap()
}

fn decl(a: &Body<f64>, b: &Body<f64>, copies: usize) -> BooleanDeclarations {
    BooleanDeclarations {
        coincident_faces: (0..copies)
            .map(|_| FacePairDeclaration::new(top(a), top(b), BooleanCoincidence::Continuation))
            .collect(),
        ..BooleanDeclarations::none()
    }
}

fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>, r: &Body<f64>, d: &BooleanDeclarations) -> Result<(), BooleanError> {
    let tol = Tol::witness();
    volume_backstop(op, a, b, r, d, Band::linear(tol).unwrap(), tol, QuadLane::certified())
}

fn verdict(x: &Result<(), BooleanError>) -> String {
    match x {
        Ok(()) => "PASSES".into(),
        Err(BooleanError::ResultVolumeImplausible { which, .. }) => format!("refuses({which})"),
        Err(e) => format!("other({e:?})"),
    }
}

/// P1: MAJ-1's wrongly kept cube, with the flush tops declared (true:
/// the operands are the same plate). Main refuses every certified
/// negative at arm 1.
#[test]
fn p1_allowance_lets_a_wrong_component_out() {
    let eps = Tol::witness().eps();
    let esc = Band::linear(Tol::witness()).unwrap().escalate();
    let pr = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
    let plate = prism(&pr, (0.0, 0.1), (0.0, 0.0));
    println!("eps={eps:e} escalate={esc:e} allowance(1 pair, 4 m^2)={:e}", esc * 4.0);
    for edge_over_threshold in [0.5, 0.9, 0.99, 1.01, 2.0] {
        // a kept cube whose volume is edge_over_threshold * allowance
        let kept = edge_over_threshold * esc * 4.0;
        let edge = kept.cbrt();
        let wrong = prism(&pr, (0.0, 0.1 + kept / 4.0), (0.0, 0.0));
        for op in [BooleanOp::Intersect, BooleanOp::Subtract] {
            let none = run(op, &plate, &plate, &wrong, &BooleanDeclarations::none());
            let one = run(op, &plate, &plate, &wrong, &decl(&plate, &plate, 1));
            println!(
                "P1 {op:?} kept={kept:e} m^3 (cube edge {:.3e} m = {:.0} eps): none={} declared={}",
                edge, edge / eps, verdict(&none), verdict(&one)
            );
        }
    }
    // MAJ-1's own 3 mm cube
    let kept = 0.003_f64.powi(3);
    let wrong = prism(&pr, (0.0, 0.1 + kept / 4.0), (0.0, 0.0));
    let one = run(BooleanOp::Subtract, &plate, &plate, &wrong, &decl(&plate, &plate, 1));
    println!("P1 MAJ-1 3mm cube, tops declared: {}", verdict(&one));
}

/// P1b: the same pair declared N times (validate_declarations admits a
/// repeat under the same class) multiplies the allowance.
#[test]
fn p1b_duplicate_declarations_multiply_the_allowance() {
    let esc = Band::linear(Tol::witness()).unwrap().escalate();
    let pr = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
    let plate = prism(&pr, (0.0, 0.1), (0.0, 0.0));
    // a result 10% too thick: 0.04 m^3 wrong
    let wrong = prism(&pr, (0.0, 0.11), (0.0, 0.0));
    let need = (0.04 / (esc * 4.0)).ceil() as usize + 1;
    for n in [1usize, 1000, need.min(5_000_000)] {
        let d = decl(&plate, &plate, n);
        println!("P1b copies={n}: {}", verdict(&run(BooleanOp::Intersect, &plate, &plate, &wrong, &d)));
    }
}

/// P3: interval width away from the origin; MAJ-1 with nothing
/// declared, far from the origin.
#[test]
fn p3_far_origin_interval_width() {
    let pr0 = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
    for off in [0.0, 1e2, 1e3, 1e4, 1e5, 1e6] {
        let plate = prism(&pr0, (off, off + 0.1), (off, off));
        let kept = 0.003_f64.powi(3);
        let wrong = prism(&pr0, (off, off + 0.1 + kept / 4.0), (off, off));
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let faces = crate::query::all_faces(&wrong);
        let (_, cert) = crate::props::sign_walk(&wrong, &faces, band, tol, Some(QuadLane::<f64>::certified()), |_| None, |r| r).unwrap();
        let t = cert.past_target();
        let (v, _a) = t.interval_volume().unwrap().unwrap();
        // exact oracle of the STORED geometry: rational arithmetic on the f64 corners
        println!("P3x offset={off:e} lo={:.17e} hi={:.17e} f64={:.17e} corners x1={:?} z0={:?} z1={:?}", v.lo(), v.hi(), t.props().volume, 2.0+off, off, off + 0.1 + kept / 4.0);
        println!(
            "P3 offset={off:e}: f64 vol={:e} interval width={:e}; MAJ-1 none: {} ; correct plate: {}",
            t.props().volume, v.hi() - v.lo(),
            verdict(&run(BooleanOp::Subtract, &plate, &plate, &wrong, &BooleanDeclarations::none())),
            verdict(&run(BooleanOp::Subtract, &plate, &plate, &plate, &BooleanDeclarations::none())),
        );
    }
}

/// P4: new arms against exact tight cases at non-dyadic sizes.
#[test]
fn p4_tight_bounds_on_correct_planted_results() {
    let mut bad = 0;
    for i in 1..200 {
        let s = 0.1 + i as f64 * 0.0137;
        let w = s / 3.0;
        let a = prism(&[(0.0, 0.0), (w, 0.0), (w, s), (0.0, s)], (0.0, s * 0.7), (0.0, 0.0));
        let b = prism(&[(w, 0.0), (2.0 * w + 0.1, 0.0), (2.0 * w + 0.1, s), (w, s)], (0.0, s * 0.7), (0.0, 0.0));
        let u = prism(&[(0.0, 0.0), (2.0 * w + 0.1, 0.0), (2.0 * w + 0.1, s), (0.0, s)], (0.0, s * 0.7), (0.0, 0.0));
        // ∪ of touching boxes = the joint box (tight at A+B)
        let r1 = run(BooleanOp::Union, &a, &b, &u, &BooleanDeclarations::none());
        // ∖: u ∖ b = a (tight at A − B since b ⊂ u)
        let r2 = run(BooleanOp::Subtract, &u, &b, &a, &BooleanDeclarations::none());
        // ∩: u ∩ a = a (tight at ≤ B)
        let r3 = run(BooleanOp::Intersect, &u, &a, &a, &BooleanDeclarations::none());
        for r in [&r1, &r2, &r3] {
            if r.is_err() {
                bad += 1;
                println!("P4 s={s}: {}", verdict(r));
            }
        }
    }
    println!("P4 refusals of correct tight results: {bad}");
    // planted violations just past each tight bound, no declarations
    let a = prism(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], (0.0, 1.0), (0.0, 0.0));
    for k in [1e-15, 1e-12, 1e-9, 1e-6] {
        let big = prism(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], (0.0, 2.0 + k), (0.0, 0.0));
        let small = prism(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], (0.0, 1.0 - k), (0.0, 0.0));
        println!(
            "P4 k={k:e}: ∪ 2+k: {} | ∖ (1-k) of A∖A: {} | ∩ 1+k... small∖: {}",
            verdict(&run(BooleanOp::Union, &a, &a, &big, &BooleanDeclarations::none())),
            verdict(&run(BooleanOp::Subtract, &big, &a, &small, &BooleanDeclarations::none())),
            verdict(&run(BooleanOp::Intersect, &small, &a, &a, &BooleanDeclarations::none())),
        );
    }
}

/// P5: the settled-residue wedge (door_backstop_settled_residue's
/// fixture) across tilts within ±3ε, every op; outcome and volume
/// against box arithmetic.
#[test]
fn p5_wedge_tilt_sweep() {
    use crate::test_support_fixtures::{brick, mapped_cube};
    use geom_core::Vec3;
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let phi = 5.0_f64.to_radians();
    let p = Point3::new(0.5, 0.2, 1.0);
    let block = brick::<f64>((0.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol);
    let face_facing = |body: &Body<f64>, facing: f64| {
        body.faces().map(|(k, _)| k).find(|&k| matches!(crate::boolean::face_carrier(body, k),
            Some(crate::boolean::CarrierDesc::Plane { normal, .. }) if normal.z * facing > 0.99)).unwrap()
    };
    for (pose, (height, depth), facing, class) in [
        ("standing", (1.0, 0.0), -1.0, BooleanCoincidence::REST),
        ("sunk", (0.5, 0.5), 1.0, BooleanCoincidence::Continuation),
    ] {
        for over in [-3.0, -2.5, -2.0, -1.5, -1.2, -1.0, -0.5, 0.0, 0.5, 1.0, 1.2, 1.5, 2.0, 2.5, 3.0] {
            let theta = over * band.zero();
            let (ea, eb) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(phi.cos(), phi.sin(), theta * phi.sin()));
            let wedge = mapped_cube::<f64>(move |u, v, w| p + ea * u + eb * v + Vec3::new(0.0, 0.0, height * w - depth), tol);
            let (top, face) = (face_facing(&block, 1.0), face_facing(&wedge, facing));
            let ab = BooleanDeclarations { coincident_faces: vec![FacePairDeclaration::new(top, face, class)], ..BooleanDeclarations::none() };
            let ba = BooleanDeclarations { coincident_faces: vec![FacePairDeclaration::new(face, top, class)], ..BooleanDeclarations::none() };
            let wv = phi.sin() * height;
            let bv = 13.5;
            let want: [f64; 4] = if pose == "standing" { [bv + wv, 0.0, bv, wv] } else { [bv, wv, bv - wv, 0.0] };
            let outs = [
                crate::union_with(&block, &wedge, &ab, tol),
                crate::intersect_with(&block, &wedge, &ab, tol),
                crate::subtract_with(&block, &wedge, &ab, tol),
                crate::subtract_with(&wedge, &block, &ba, tol),
            ];
            let mut line = format!("P5 {pose} tilt {over:+}ε:");
            for (out, w) in outs.into_iter().zip(want) {
                let cell = match out {
                    Ok(crate::BooleanResult::Empty) => format!(" empty(Δ{:.1e})", -w),
                    Ok(crate::BooleanResult::Body(bb)) => {
                        let v = crate::mass_properties(&bb.body, tol).unwrap().volume;
                        format!(" {:+.2e}", v - w)
                    }
                    Err(e) => format!(" {}", format!("{e:?}").split(['{', '(', ' ']).next().unwrap()),
                };
                line.push_str(&cell);
            }
            println!("{line}  (residue bound {:.1e})", band.escalate() * phi.sin());
        }
    }
}
